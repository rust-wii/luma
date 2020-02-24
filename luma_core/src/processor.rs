//! ``processor`` module of ``luma_core``.
//!
//! Contains basic macros and functions for instructions.

/// PowerPC NOP Instruction
pub fn nop() {
    unsafe { asm!("nop" :::: "volatile") }
}

// PowerPC Execution Synchronization
pub fn sync() {
    unsafe { asm!("sync" :::: "volatile") }
}

/// PowerPC System Halt
pub fn ppc_halt() {
    // Sync execution.
    sync();

    // Loop execution.
    loop {
        unsafe {
            // NOP Instruction.
            nop();

            // Load Immediate.
            asm!("li 3,0" :::: "volatile");

            // NOP Instruction.
            nop();
        }
    }
}

/// PowerPC System Context Synchronization
/// 
/// NOTE: This sync is different from the ``sync``
/// instruction! This sync is a system contextual sync.
pub fn ppc_sync() {
    // Context Synchronization.
    unsafe {
        asm!("sc" :::: "volatile");
    }
}