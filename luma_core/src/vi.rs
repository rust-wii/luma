//! ``vi`` module of ``luma_core``.
//!
//! Contains functions for basic video access.

use crate::allocate::alloc_aligned;
use crate::io::{read16, write16, write32};
use alloc::boxed::Box;
use core::pin::Pin;

/// A struct representing the eXternal FrameBuffer, or XFB.  It represents the image that will be
/// sent to the screen, in YUYV format.  It must be allocated as contiguous physical memory.
pub struct Xfb {
    data: Pin<Box<[u8]>>,
    width: usize,
    height: usize,
}

impl Xfb {
    /// Allocate an XFB with the given width and height.
    pub fn allocate(width: usize, height: usize) -> Xfb {
        let stride = width * 2;
        let data = alloc_aligned(stride * height);
        Xfb {
            data,
            width,
            height,
        }
    }

    /// Get the width with which this XFB got allocated.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height with which this XFB got allocated.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the stride of this XFB, given the YUYV format this is always width × 2.
    pub fn stride(&self) -> usize {
        // YUYV always takes two bytes per pixel.
        self.width * 2
    }

    /// Return the raw pointer to this XFB.
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    /// Return the raw mutable pointer to this XFB.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

const BASE: u32 = 0xcc00_2000;

bitflags::bitflags! {
    pub struct ConfigureFlags: u16 {
        const NTSC = 0 << 8;
        const PAL = 1 << 8;
        const MPAL = 2 << 8;
        const DEBUG = 3 << 8;
        const STEREO3D = 1 << 3;
        const INTERLACED = 0 << 2;
        const PROGRESSIVE = 1 << 2;
        const RESET = 1 << 1;
        const ENABLE = 1 << 0;
    }
}

unsafe fn set_vertical_timing(height: u16, equ: u16) {
    assert!(height <= 0xfff);
    assert!(equ <= 0x0f);
    let half_height = height / 2;
    // TODO: figure out what that is.  Tenth of framerate?
    write16(BASE + 0x00, (half_height << 4) | (equ as u16));
}

unsafe fn configure(flags: ConfigureFlags) {
    write16(BASE + 0x02, flags.bits());
}

unsafe fn set_horizontal_timing(s1: u32, e1: u32, w1: u32, s2: u32, e2: u32, w2: u32) {
    assert!(s1 <= 0x7f);
    assert!(e1 <= 0x7f);
    assert!(w1 <= 0x1ff);
    assert!(s2 <= 0x3ff);
    assert!(e2 <= 0x3ff);
    assert!(w2 <= 0x7f);
    write32(BASE + 0x04, (s1 << 24) | (e1 << 16) | w1);
    write32(BASE + 0x08, (s2 << 17) | (e2 << 7) | w2);
}

unsafe fn set_field_vertical_timing(psb1: u32, prb1: u32, psb2: u32, prb2: u32) {
    assert!(psb1 <= 0x3ff);
    assert!(prb1 <= 0x3ff);
    assert!(psb2 <= 0x3ff);
    assert!(prb2 <= 0x3ff);
    write32(BASE + 0x0c, (psb1 << 16) | prb1);
    write32(BASE + 0x10, (psb2 << 16) | prb2);
}

unsafe fn set_burst_blanking_interval_1(be1: u32, bs1: u32, be3: u32, bs3: u32) {
    assert!(be1 <= 0x7ff);
    assert!(bs1 <= 0x1f);
    assert!(be3 <= 0x7ff);
    assert!(bs3 <= 0x1f);
    write32(BASE + 0x14, (be3 << 21) | (bs3 << 16) | (be1 << 5) | bs1);
}

unsafe fn set_burst_blanking_interval_2(be2: u32, bs2: u32, be4: u32, bs4: u32) {
    assert!(be2 <= 0x7ff);
    assert!(bs2 <= 0x1f);
    assert!(be4 <= 0x7ff);
    assert!(bs4 <= 0x1f);
    write32(BASE + 0x18, (be4 << 21) | (bs4 << 16) | (be2 << 5) | bs2);
}

unsafe fn set_xfb(addr: u32, xfb: &Xfb, bottom: bool) {
    let stride = xfb.stride() as u32;
    let xfb = xfb.as_ptr();
    let mut xfb = xfb as u32;
    let shift;
    if bottom {
        xfb += stride;
    }
    if xfb < 0x0100_0000 {
        shift = false;
    } else {
        xfb >>= 5;
        shift = true;
    }
    write32(addr, ((shift as u32) << 28) | xfb);
}

unsafe fn set_top_xfb(xfb: &Xfb) {
    set_xfb(BASE + 0x1c, xfb, false);
}

unsafe fn set_bottom_xfb(xfb: &Xfb) {
    set_xfb(BASE + 0x24, xfb, true);
}

/*
/// Used for stereoscopy.
unsafe fn set_top_right_xfb(xfb: &Xfb) {
    set_xfb(BASE + 0x20, xfb);
}

/// Used for stereoscopy.
unsafe fn set_bottom_right_xfb(xfb: &Xfb) {
    set_xfb(BASE + 0x28, xfb);
}
*/

unsafe fn set_display_interrupts() {
    write32(BASE + 0x30, 0x110701ae);
    write32(BASE + 0x34, 0x10010001);
    write32(BASE + 0x38, 0x00010001);
    write32(BASE + 0x3c, 0x00010001);
}

unsafe fn set_scaled_width(width: u16) {
    // TODO: add actual support for scaled width…
    write16(BASE + 0x48, 0x2850);
    write16(BASE + 0x4a, 0x0100);
}

unsafe fn set_aa_filters() {
    write32(BASE + 0x4c, 0x1ae771f0);
    write32(BASE + 0x50, 0x0db4a574);
    write32(BASE + 0x54, 0x00c1188e);
    write32(BASE + 0x58, 0xc4c0cbe2);
    write32(BASE + 0x5c, 0xfcecdecf);
    write32(BASE + 0x60, 0x13130f08);
    write32(BASE + 0x64, 0x00080c0f);

    // Maybe?
    write32(BASE + 0x68, 0x00ff0000);
}

unsafe fn set_clock(clock: u16) {
    let clock = match clock {
        27 => 0,
        54 => 1,
        _ => panic!("Wrong clock for VI"),
    };
    write16(BASE + 0x6c, clock);
}

unsafe fn get_visel() -> u16 {
    read16(BASE + 0x6e)
}

unsafe fn set_border() {
    // TODO: add actual support for borders, and hwtest it.
    write16(BASE + 0x72, 0x0000);
    write16(BASE + 0x74, 0x0000);
}

unsafe fn setup_interlaced(width: usize, height: usize, xfb: &Xfb) {
    set_vertical_timing(height as u16, 6);
    configure(ConfigureFlags::PAL | ConfigureFlags::INTERLACED | ConfigureFlags::ENABLE);
    // TODO: figure out why 0x40 becomes 0x42 once read here…
    set_horizontal_timing(71, 105, 429, 373, 162, 64);
    set_field_vertical_timing(3, 24, 2, 25);
    set_burst_blanking_interval_1(520, 12, 520, 12);
    set_burst_blanking_interval_2(519, 13, 519, 13);
    set_top_xfb(xfb);
    set_bottom_xfb(xfb);
    set_display_interrupts();
    // 0x40 and 0x44 are display latch registers, unused?
    set_scaled_width(width as u16);
    set_aa_filters();
    set_clock(27 /* MHz */);
    set_border();
}

/// A struct representing the Video Interface, or VI.  This is the piece of hardware which scans
/// out the XFB to the screen.
pub struct Vi {
    xfb: Xfb,
}

impl Vi {
    /// Setup the VI with the given XFB.
    pub fn setup(xfb: Xfb) -> Vi {
        unsafe { setup_interlaced(xfb.width(), xfb.height(), &xfb) };
        Vi { xfb }
    }

    /// Get back a mutable reference to the XFB.
    pub fn xfb(&mut self) -> &mut Xfb {
        &mut self.xfb
    }

    // TODO: document!
    pub fn visel(&self) -> u16 {
        unsafe { get_visel() }
    }
}
