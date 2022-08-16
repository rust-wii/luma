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
        write!(DolphinHle, $($arg)*).unwrap();
    }};
}
