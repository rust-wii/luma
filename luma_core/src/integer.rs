//! ``integer`` module of ``luma_core``.
//!
//! Contains functions for integer instructions.

use core::arch::asm;

/// (`cntlzw`) PowerPC Integer Instruction
#[inline(always)]
pub fn cntlzw(value: u32) -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("cntlzw {0}, {1}",
            lateout(reg) register,
            in(reg) value,
            options(nostack));
    }

    // Return the register value.
    register
}
