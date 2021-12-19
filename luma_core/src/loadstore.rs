//! ``loadstore`` module of ``luma_core``.
//!
//! Contains functions for load and store instructions.

use core::arch::asm;

/// (`lhbrx`) PowerPC Load Instruction
#[inline(always)]
pub fn lhbrx(base: u32, index: u32) -> u16 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("lhbrx {0}, {1}, {2}",
            lateout(reg) register,
            in(reg_nonzero) index, in(reg) base, 
            options(nostack));
    }

    // Return the register value.
    register
}

/// (`lwbrx`) PowerPC Load Instruction
#[inline(always)]
pub fn lwbrx(base: u32, index: u32) -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("lwbrx {0}, {1}, {2}",
            lateout(reg) register,
            in(reg_nonzero) index, in(reg) base,
            options(nostack));
    }

    // Return the register value.
    register
}

/// (`sthbrx`) PowerPC Store Instruction
#[inline(always)]
pub fn sthbrx(base: u32, index: u32, value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("sthbrx {0}, {1}, {2}",
            in(reg) value, in(reg_nonzero) index, in(reg) base, 
            options(nostack));
    }
}

/// (`stwbrx`) PowerPC Store Instruction
#[inline(always)]
pub fn stwbrx(base: u32, index: u32, value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("stwbrx {0}, {1}, {2}",
            in(reg) value, in(reg_nonzero) index, in(reg) base, 
            options(nostack));
    }
}
