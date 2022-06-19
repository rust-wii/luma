//! ``irq`` module of ``luma_core``.
//!
//! Contains an interrupt handler and interrupt controller.

use self::consts::*;
use crate::integer::cntlzw;
use core::mem;

// IRQ constants.
pub mod consts;

/// Spurious Interrupt Vector
static mut SPURIOUS_IRQ: u64 = 0;

/// Previous and Current Interrupt ,asks
static mut PREV_IRQ_MASK: u32 = 0;
static mut CUR_IRQ_MASK: u32 = 0;

/// Global IRQ Handler
static mut GLOBAL_IRQ_HANDLER: [IrqHandler; 32] = [IrqHandler {
    handle: None,
    context: 0 as *mut libc::c_void,
}; 32];

/// Macro for getting the related mask from a condition and a mask.
macro_rules! irqcond {
    ($var: ident, $cond: expr, $mask: tt) => {
        if $cond {
            $var = $var | 0x80000000 >> $mask;
        }
    };
}

/// Raw IRQ Handler Type
pub type RawIrqHandler = Option<unsafe extern "C" fn(a: u32, b: *mut libc::c_void) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct IrqHandler {
    pub handle: RawIrqHandler,
    pub context: *mut libc::c_void,
}

/// Interrupt Dispatcher
#[no_mangle]
pub unsafe extern "C" fn interrupt_dispatch() {
    // Get the cause and mask from the PI register.
    let cause = *PI_REG.offset(0) & !(0x10000);
    let mut mask = *PI_REG.offset(1);

    // Helper variables for interrupt handling.
    let mut icause = 0;
    let mut intmask = 0;

    // Check if the incoming interrupt isn't spurious.
    if cause == 0 || cause & mask == 0 {
        SPURIOUS_IRQ += 1;
        return;
    }

    // Memory Interface Interrupt
    if cause & 0x80 != 0 {
        icause = *MEM_REG.offset(15) as u32;

        // IRQ_MEM0 Mask
        irqcond!(intmask, icause & 0x1 != 0, IM_MEM0);

        // IRQ_MEM1 Mask
        irqcond!(intmask, icause & 0x2 != 0, IM_MEM1);

        // IRQ_MEM2 Mask
        irqcond!(intmask, icause & 0x4 != 0, IM_MEM2);

        // IRQ_MEM3 Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_MEM3);

        // IRQ_MEMADDRESS Mask
        irqcond!(intmask, icause & 0x10 != 0, IM_MEMADDRESS);
    }

    // DSP Interrupt
    if cause & 0x40 != 0 {
        icause = *DSP_REG.offset(5) as u32;

        // IRQ_DSP_AI Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_DSP_AI);

        // IRQ_DSP_ARAM Mask
        irqcond!(intmask, icause & 0x20 != 0, IM_DSP_ARAM);

        // IRQ_DSP_DSP Mask
        irqcond!(intmask, icause & 0x80 != 0, IM_DSP_DSP);
    }

    // Streaming Interrupt
    if cause & 0x20 != 0 {
        icause = *AI_REG.offset(0);

        // IRQ_AI Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_AI);
    }

    // EXI Interrupt
    if cause & 0x10 != 0 {
        // EXI 0
        icause = *EXI_REG.offset(0);

        // IRQ_EXI0_EXI Mask
        irqcond!(intmask, icause & 0x2 != 0, IM_EXI0_EXI);

        // IRQ_EXI0_TC Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_EXI0_TC);

        // IRQ_EXI0_EXT Mask
        irqcond!(intmask, icause & 0x800 != 0, IM_EXI0_EXT);

        // EXI 1
        icause = *EXI_REG.offset(5);

        // IRQ_EXI1_EXI Mask
        irqcond!(intmask, icause & 0x2 != 0, IM_EXI1_EXI);

        // IRQ_EXI1_TC Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_EXI1_TC);

        // IRQ_EXI1_EXT Mask
        irqcond!(intmask, icause & 0x800 != 0, IM_EXI1_EXT);

        // EXI 2
        icause = *EXI_REG.offset(10);

        // IRQ_EXI2_EXI Mask
        irqcond!(intmask, icause & 0x2 != 0, IM_EXI2_EXI);

        // IRQ_EXI2_TC Mask
        irqcond!(intmask, icause & 0x8 != 0, IM_EXI2_TC);
    }

    // Get the current interrupt mask.
    mask = intmask & !(PREV_IRQ_MASK | CUR_IRQ_MASK);

    // Helper variables for the interrupt handler.
    let mut i = 0;
    let mut irq = 0;

    // Check if there is a pending interrupt.
    if mask != 0 {
        // Find the highest priority interrupt.
        while i < (mem::size_of::<[u32; 12]>()).wrapping_div(mem::size_of::<u32>()) {
            irq = mask & IRQ_PRIORITY[i];
            if irq != 0 {
                irq = cntlzw(irq);
                break;
            } else {
                i += 1
            }
        }

        // Check if the global IRQ handler has a handle.
        if GLOBAL_IRQ_HANDLER[irq as usize].handle.is_some() {
            GLOBAL_IRQ_HANDLER[irq as usize].handle.unwrap()(
                irq,
                GLOBAL_IRQ_HANDLER[irq as usize].context,
            );
        }
    }
}
