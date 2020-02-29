//! ``processor`` module of ``luma_core``.
//!
//! Contains functions for system instructions.

/// PowerPC NOP Instruction
#[inline(always)]
pub fn ppc_nop() {
    unsafe { asm!("nop" :::: "volatile") }
}

/// PowerPC Execution Synchronization
#[inline(always)]
pub fn ppc_exec_sync() {
    unsafe { asm!("sync" :::: "volatile") }
}

/// PowerPC System Halt
pub fn ppc_halt() {
    // Sync execution.
    ppc_exec_sync();

    // Loop execution.
    loop {
        unsafe {
            // NOP Instruction.
            ppc_nop();

            // Load Immediate.
            asm!("li 3,0" :::: "volatile");

            // NOP Instruction.
            ppc_nop();
        }
    }
}

/// PowerPC System Context Synchronization
///
/// NOTE: This sync is different from the ``sync``
/// instruction! This sync is a system contextual sync.
#[inline(always)]
pub fn ppc_ctx_sync() {
    // Context Synchronization.
    unsafe {
        asm!("sc" :::: "volatile");
    }
}
