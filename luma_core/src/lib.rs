//! ``luma_core`` is the core module of ``luma``.
//!
//! This module contains core processor features.
//!
//! **NOTE**: This is currently in a very experimental state and is subject to change.
#![no_std]
#![allow(unused_attributes)]
#![feature(asm_experimental_arch, box_into_boxed_slice, allocator_api)]

extern crate alloc;

use alloc::string::ToString;
use core::arch::asm;
use core::fmt;

// Broadway Processor Utilities
pub mod processor;

// Broadway Register Utilities
pub mod register;

// Broadway Integer Utilities
pub mod integer;

// Broadway Load and Store Utilities
pub mod loadstore;

// Broadway I/O Utilities
pub mod io;

// Broadway Cache Subsystem
pub mod cache;

// Helper functions to allocate aligned memory on the heap
pub mod allocate;

// VI Subsystem
pub mod vi;

//IPC Subsystem
pub mod ipc;

/// Do nothing, this is for Dolphin’s use until we get actual USB Gecko support.
///
/// This function must exist and its symbol must be kept in order to get HLE debugging in Dolphin.
///
/// Unlike puts(), it doesn’t require a null-terminated CStr, so in the optimal case we can pass a
/// &str’s pointer as is, without doing any extra allocation.
#[no_mangle]
#[inline(never)]
unsafe extern "C" fn __write_console(_unused: u32, message: *const u8, size: *const u32) {
    asm!("/* {0} {1} */", in(reg) message, in(reg) size);
}

/// Implements Write using Dolphin’s HLE.
pub struct DolphinHle;

impl fmt::Write for DolphinHle {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let len = s.len() as u32;
        unsafe { __write_console(0, s.as_ptr(), &len as *const u32) };
        Ok(())
    }

    #[inline(always)]
    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        if let Some(s) = args.as_str() {
            self.write_str(s)
        } else {
            self.write_str(&args.to_string())
        }
    }
}

/// Reimplementation of Rust’s println!(), using Dolphin’s HLE.
///
/// This macro requires luma_core and core::fmt::Write to be present in the callee’s environment.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use luma_core::DolphinHle;
        use core::fmt::Write;
        write!(DolphinHle, $($arg)*).unwrap();
    }};
}

pub mod framebuffer {
    use crate::vi::Xfb;
    extern crate alloc;

    #[derive(Copy, Clone)]
    pub struct Rgba {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    #[derive(Copy, Clone, Default)]
    #[repr(align(4))]
    pub struct Yuv444 {
        y: u8,
        u: i8,
        v: i8,
    }

    #[derive(Copy, Clone, Default)]
    pub struct Yuv422 {
        pub y: u8,
        pub uv: u8,
    }

    impl Yuv422 {
        pub fn as_u16(&self) -> u16 {
            u16::from(self.y) << 8 | u16::from(self.uv)
        }
        pub fn into_u16(self) -> u16 {
            u16::from(self.y) << 8 | u16::from(self.uv)
        }
    }

    pub fn rgba_to_yuv444(rgba: Rgba) -> Yuv444 {
        let r = f64::from(rgba.r);
        let g = f64::from(rgba.g);
        let b = f64::from(rgba.b);

        let y_f64 = (0.299 * r) + (0.587 * g) + (0.114 * b);
        let u_f64 = -(0.168736 * r) - (0.331264 * g) + (0.5 * b);
        let v_f64 = (0.5 * r) - (0.418688 * g) - (0.081312 * b);

        Yuv444 {
            y: y_f64 as u8,
            u: u_f64 as i8,
            v: v_f64 as i8,
        }
    }

    pub fn rgba_to_xfb(xfb: &mut Xfb, rgba_framebuf: &mut [Rgba]) {
        const XFB_MAX_WIDTH: usize = 640;
        let width = xfb.width();
        let height = xfb.height();

        let mut scanline = [Yuv444::default(); XFB_MAX_WIDTH];
        let mut packed_scanline = [Yuv422::default(); XFB_MAX_WIDTH];
        for y in 0..height {
            let rgba_scanline = &rgba_framebuf[y * width..(y + 1) * width];

            for (scl, rgba) in scanline.iter_mut().zip(rgba_scanline) {
                *scl = rgba_to_yuv444(*rgba);
            }

            for (pcked_scl_pair, scl_triple) in packed_scanline
                .chunks_exact_mut(2)
                .zip(scanline.windows(3).step_by(2))
            {
                pcked_scl_pair[0].y = 16u8.saturating_add(scl_triple[1].y);
                pcked_scl_pair[0].uv = (128f64
                    + (0.25 * f64::from(scl_triple[0].u))
                    + (0.5 * f64::from(scl_triple[1].u))
                    + (0.25 * f64::from(scl_triple[2].u)))
                    as u8;

                pcked_scl_pair[1].y = 16u8.saturating_add(scanline[1].y);
                pcked_scl_pair[1].uv = (128f64
                    + (0.25 * f64::from(scl_triple[0].v))
                    + (0.5 * f64::from(scl_triple[1].v))
                    + (0.25 * f64::from(scl_triple[2].v)))
                    as u8;
            }
            if let Some(yuv) = xfb.iter_mut().nth(y) {
                let len = yuv.len();
                yuv[..].copy_from_slice(&packed_scanline.map(Yuv422::into_u16)[..len]);
            }
        }
    }
}
