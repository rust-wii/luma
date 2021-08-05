//! ``integer`` module of ``luma_core``.
//!
//! Contains functions for integer instructions.

/// (`cntlzw`) PowerPC Integer Instruction
pub fn cntlzw(value: u32) -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        llvm_asm!("cntlzw $0, $1" : "=r"(register) : "r"(value) :: "volatile");
    }

    // Return the register value.
    register
}
