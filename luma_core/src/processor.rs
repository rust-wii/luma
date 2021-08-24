//! ``processor`` module of ``luma_core``.
//!
//! Contains functions for system instructions.

/// PowerPC NOP Instruction
#[inline(always)]
pub fn ppc_nop() {
    unsafe { asm!("nop", options(nostack)) }
}

/// PowerPC Execution Synchronization
#[inline(always)]
pub fn ppc_exec_sync() {
    unsafe { asm!("sync", options(nostack)) }
}

/// PowerPC System Halt
#[inline(always)]
pub fn ppc_halt() {
    // Sync execution.
    ppc_exec_sync();

    // Loop execution.
    loop {
        // NOP Instruction.
        ppc_nop();

        unsafe {
            // Load Immediate.
            asm!("li 3,0", options(nostack));
        }

        // NOP Instruction.
        ppc_nop();
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
        asm!("sc", options(nostack));
    }
}

/// PowerPC CPU ISR Enable
#[inline(always)]
pub fn cpu_isr_enable() {
    // Define a register variable.
    let mut _val = 0u32;

    // Run the assembly instruction.
    unsafe {
        asm!("mfmsr {0}",
             "ori {0},{0},0x8000",
             "mtmsr {0}",
            inout(reg) _val, options(nostack));
    }
}

/// PowerPC CPU ISR Disable
#[inline(always)]
pub fn cpu_isr_disable(mut _isr_cookie: u32) {
    // Define variables.
    let mut _disable_mask = 0u32;
    _isr_cookie = 0;

    // Run the assembly instruction.
    unsafe {
        asm!("mfmsr {0}",
             "rlwinm {1},{0},0,17,15",
             "mtmsr {1}",
             "extrwi {0},{0},1,16",
            inout(reg) _isr_cookie, inout(reg) _disable_mask, 
            options(nostack));
    }
}

/// PowerPC CPU ISR Restore
#[inline(always)]
pub fn cpu_isr_restore(mut _isr_cookie: u32) {
    // Define mask variable.
    let mut _enable_mask = 0u32;

    // Run the assembly instruction.
    unsafe {
        asm!("cmpwi {0},0",
             "beq 1f",
             "mfmsr {1}",
             "ori {1},{1},0x8000",
             "mtmsr {1}",
             "1:",
            inout(reg) _isr_cookie, inout(reg) _enable_mask,
            options(nostack));
    }
}
