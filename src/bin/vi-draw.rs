//! This is an example of how to draw things to the screen using Luma.
//!
//! The drawing has been ported from Weston’s clients/simple-shm.c

#![no_std]

extern crate luma_core;
extern crate luma_runtime;

use luma_core::vi::{Vi, Xfb};

// Constants used for the YUV conversion.
const YR: i32 = (0.299 * (1 << 16) as f64) as i32;
const YG: i32 = (0.587 * (1 << 16) as f64) as i32;
const YB: i32 = (0.114 * (1 << 16) as f64) as i32;

const UR: i32 = (-0.169 * (1 << 16) as f64) as i32;
const UG: i32 = (-0.331 * (1 << 16) as f64) as i32;
const UB: i32 = (0.500 * (1 << 16) as f64) as i32;

const VR: i32 = (0.500 * (1 << 16) as f64) as i32;
const VG: i32 = (-0.419 * (1 << 16) as f64) as i32;
const VB: i32 = (-0.081 * (1 << 16) as f64) as i32;

/// Very bad conversion, it should take two pixels and output an u32, a proper implementation is
/// left as an exercise to the reader.
///
/// An even better implementation would use the GPU to do the conversion.
fn rgba2yuyv(pixel: u32, odd: bool) -> u16 {
    let r: i32 = ((pixel >> 16) & 0xff) as i32;
    let g: i32 = ((pixel >> 8) & 0xff) as i32;
    let b: i32 = ((pixel >> 0) & 0xff) as i32;

    let y: i32 = (YR * r + YG * g + YB * b) >> 16;
    let u: i32 = (UR * r + UG * g + UB * b) >> 16;
    let v: i32 = (VR * r + VG * g + VB * b) >> 16;

    let chroma = if odd { u } else { v } + 128;
    (y as u16) << 8 | (chroma as u16)
}

/// Ported from Weston’s clients/simple-shm.c
fn paint_pixels(mut image: *mut u16, padding: u32, width: u32, height: u32, time: u32) {
    let halfh = padding + (height - padding * 2) / 2;
    let halfw = padding + (width - padding * 2) / 2;

    // Squared radii thresholds
    let mut or = (if halfw < halfh { halfw } else { halfh }) - 8;
    let mut ir = or - 32;
    or *= or;
    ir *= ir;

    image = unsafe { image.offset((padding * width) as isize) };
    for y in padding..(height - padding) {
        let y2 = (y - halfh) * (y - halfh);

        image = unsafe { image.offset(padding as isize) };
        for x in padding..(width - padding) {
            let mut v: u32;

            /* squared distance from center */
            let r2 = (x - halfw) * (x - halfw) + y2;

            if r2 < ir {
                v = (r2 / 32 + time / 64) * 0x0080401;
            } else if r2 < or {
                v = (y + time / 32) * 0x0080401;
            } else {
                v = (x + time / 16) * 0x0080401;
            }
            v &= 0x00ffffff;

            unsafe { *image = rgba2yuyv(v, (x & 1) != 0) };
            image = unsafe { image.offset(1) };
        }

        image = unsafe { image.offset(padding as isize) };
    }
}

fn main() {
    // Setup the video interface.
    let xfb = Xfb::allocate(640, 480);
    let mut vi = Vi::setup(xfb);

    // First fill the XFB with white.
    let xfb = vi.xfb().as_mut_ptr() as *mut u16;
    for i in 0..(640 * 480) {
        unsafe { xfb.offset(i).write(0xff80) };
    }

    // Then draw to it as fast as we can (that is, super slowly).
    let mut i = 0;
    loop {
        paint_pixels(xfb, 20, 640, 480, i);
        i += 1;
    }
}
