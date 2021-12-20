//! ``luma_core`` is the core module of ``luma``.
//!
//! This module contains core processor features.
//!
//! **NOTE**: This is currently in a very experimental state and is subject to change.
#![no_std]
#![allow(unused_attributes)]
#![feature(asm_experimental_arch, box_into_boxed_slice, allocator_api)]

use core::arch::asm;

extern crate alloc;

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

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn puts(unused: u32, message: *const u8) {
    // Do nothing, this is for Dolphin’s use until we get actual USB Gecko support.
    asm!("/* {0} {1} */", in(reg) unused, in(reg) message);
}

#[macro_export]
macro_rules! println {
    ($fmt: expr, $($args: expr),*) => {{
        // TODO: figure out a way to not have to import those crates with these names in user code.
        let string = alloc::format!($fmt, $($args),*);
        unsafe { luma_core::puts(0, string.as_ptr()) };
    }}
}
