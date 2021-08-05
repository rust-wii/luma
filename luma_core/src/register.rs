//! ``register`` module of ``luma_core``.
//!
//! Contains functions for register instructions.

/// (`mfspr`) PowerPC Register Instruction
#[macro_export]
macro_rules! mfspr {
    ($R:tt) => {
        unsafe {
            let mut output: u32;
            llvm_asm!( concat!("mfspr", " $0,", stringify!($R))
                : "=r"(output) ::: "volatile"
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
            llvm_asm!( concat!("mtspr", ' ', stringify!($R), ",$0")
                :: "r"($val) :: "volatile"
            )
        }
    }
}

/// (`mfpvr`) PowerPC Register Instruction
pub fn mfpvr() -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        llvm_asm!("mfpvr $0" : "=r"(register) ::: "volatile");
    }

    // Return the register value.
    register
}

/// (`mfmsr`) PowerPC Register Instruction
pub fn mfmsr() -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        llvm_asm!("mfmsr $0" : "=r"(register) ::: "volatile");
    }

    // Return the register value.
    register
}

/// (`mtmsr`) PowerPC Register Instruction
#[inline(always)]
pub fn mtmsr(value: u32) {
    // Run the assembly instruction.
    unsafe {
        llvm_asm!("mtmsr $0" :: "r"(value) :: "volatile");
    }
}

/// (`mtdec`) PowerPC Register Instruction
#[inline(always)]
pub fn mtdec(value: u32) {
    // Run the assembly instruction.
    unsafe {
        llvm_asm!("mtdec $0" :: "r"(value) :: "volatile");
    }
}
