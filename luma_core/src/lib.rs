//! ``luma_core`` is the core module of ``luma``.
//!
//! This module contains core processor features.
//!
//! **NOTE**: This is currently in a very experimental state and is subject to change.
#![no_std]
#![allow(unused_attributes)]

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

// IPC Subsystem
pub mod ipc;

// Serial Interface (SI) Subsystem Utilities
pub mod serial_interface;

pub mod pad;

/// Do nothing, this is for Dolphin’s use until we get actual USB Gecko support.
///
/// This function must exist and its symbol must be kept in order to get HLE debugging in Dolphin.
///
/// Unlike puts(), it doesn’t require a null-terminated CStr, so in the optimal case we can pass a
/// &str’s pointer as is, without doing any extra allocation.
#[unsafe(no_mangle)]
#[inline(never)]
extern "C" fn __write_console(_unused: u32, message: *const u8, size: *const u32) {
    unsafe {
        core::arch::asm!(
            "mr 4, {0}",
            "mr 5, {1}",
            in(reg) message,
            in(reg) size,
            options(nostack, preserves_flags)
        );
    }
}

/// Implements Write using Dolphin’s HLE.
pub struct DolphinHle;

impl fmt::Write for DolphinHle {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let len = s.len() as u32;
        __write_console(0, s.as_ptr(), &len);
        Ok(())
    }
}

/// Reimplementation of Rust’s println!(), using Dolphin’s HLE.
///
/// This macro requires luma_core and core::fmt::Write to be present in the callee’s environment.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use $crate::DolphinHle;
        use core::fmt::Write;

        write!(DolphinHle, $($arg)*).unwrap();
    }};
}

#[inline(always)]
pub fn breakpoint() {
    unsafe {
        // 'twge r0, r0' is Trap Word Greater Than or Equal (if r0 >= r0, which is always true)
        // This is the standard PowerPC hardware breakpoint instruction.
        core::arch::asm!("twge r0, r0");
    }
}
