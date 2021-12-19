//! ``register`` module of ``luma_core``.
//!
//! Contains functions for register instructions.

use core::arch::asm;

/// (`mfspr`) PowerPC Register Instruction
#[macro_export]
macro_rules! mfspr {
    ($R:tt) => {
        unsafe {
            let mut output: u32;
            asm!( concat!("mfspr", " {0},", stringify!($R)),
                out(reg) output, options(nostack)
            );
            output
        }
    }
}

/// (`mtspr`) PowerPC Register Instruction
#[macro_export]
macro_rules! mtspr {
    ($val:expr, $R:tt) => {
        unsafe {
            asm!( concat!("mtspr", ' ', stringify!($R), ",{0}"),
                in(reg) $val, options(nostack)
            )
        }
    }
}

/// (`mfpvr`) PowerPC Register Instruction
#[inline(always)]
pub fn mfpvr() -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("mfpvr {0}",
            out(reg) register,
            options(nostack));
    }

    // Return the register value.
    register
}

/// (`mfmsr`) PowerPC Register Instruction
#[inline(always)]
pub fn mfmsr() -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("mfmsr {0}",
            out(reg) register,
            options(nostack));
    }

    // Return the register value.
    register
}

/// (`mtmsr`) PowerPC Register Instruction
#[inline(always)]
pub fn mtmsr(value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("mtmsr {0}",
            in(reg) value,
            options(nostack));
    }
}

/// (`mtdec`) PowerPC Register Instruction
#[inline(always)]
pub fn mtdec(value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("mtdec {0}",
            in(reg) value,
            options(nostack));
    }
}
