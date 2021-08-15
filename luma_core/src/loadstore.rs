//! ``loadstore`` module of ``luma_core``.
//!
//! Contains functions for load and store instructions.

/// (`lhbrx`) PowerPC Load Instruction
#[inline(always)]
pub fn lhbrx(base: u32, index: u32) -> u16 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        llvm_asm!("lhbrx $0, $1, $2" 
            : "=r"(register)
            : "b%"(index), "r"(base) 
            : "memory" : "volatile");
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
        llvm_asm!("lwbrx $0, $1, $2" 
            : "=r"(register)
            : "b%"(index), "r"(base) 
            : "memory" : "volatile");
    }

    // Return the register value.
    register
}

/// (`sthbrx`) PowerPC Store Instruction
#[inline(always)]
pub fn sthbrx(base: u32, index: u32, value: u32) {
    // Run the assembly instruction.
    unsafe {
        llvm_asm!("sthbrx $0, $1, $2" :
            : "r"(value), "b%"(index), "r"(base) 
            : "memory" : "volatile");
    }
}

/// (`stwbrx`) PowerPC Store Instruction
#[inline(always)]
pub fn stwbrx(base: u32, index: u32, value: u32) {
    // Run the assembly instruction.
    unsafe {
        llvm_asm!("stwbrx $0, $1, $2" :
            : "r"(value), "b%"(index), "r"(base) 
            : "memory" : "volatile");
    }
}
