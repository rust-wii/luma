//! ``irq`` module of ``luma_core``.
//!
//! Contains constant values for the IRQ controller.

/// EXI Register
// TODO: Depending on Gamecube or Wii, this address might be different.
pub const EXI_REG: *mut u32 = 0xcc006800 as *mut u32;

/// AI Register
// TODO: Depending on Gamecube or Wii, this address might be different.
pub const AI_REG: *mut u32 = 0xCC006C00 as *mut u32;

/// MEM Register
pub const MEM_REG: *mut u16 = 0xCC004000 as *mut u16;

/// PI Register
pub const PI_REG: *mut u32 = 0xCC003000 as *mut u32;

/// DSP Register
pub const DSP_REG: *mut u16 = 0xCC005000 as *mut u16;

// IRQ Masks
pub const IRQ_MEM0: u32 = 0;
pub const IRQ_MEM1: u32 = 1;
pub const IRQ_MEM2: u32 = 2;
pub const IRQ_MEM3: u32 = 3;
pub const IRQ_MEMADDRESS: u32 = 4;
pub const IRQ_DSP_AI: u32 = 5;
pub const IRQ_DSP_ARAM: u32 = 6;
pub const IRQ_DSP_DSP: u32 = 7;
pub const IRQ_AI: u32 = 8;
pub const IRQ_EXI0_EXI: u32 = 9;
pub const IRQ_EXI0_TC: u32 = 10;
pub const IRQ_EXI0_EXT: u32 = 11;
pub const IRQ_EXI1_EXI: u32 = 12;
pub const IRQ_EXI1_TC: u32 = 13;
pub const IRQ_EXI1_EXT: u32 = 14;
pub const IRQ_EXI2_EXI: u32 = 15;
pub const IRQ_EXI2_TC: u32 = 16;
pub const IRQ_PI_CP: u32 = 17;
pub const IRQ_PI_PETOKEN: u32 = 18;
pub const IRQ_PI_PEFINISH: u32 = 19;
pub const IRQ_PI_SI: u32 = 20;
pub const IRQ_PI_DI: u32 = 21;
pub const IRQ_PI_RSW: u32 = 22;
pub const IRQ_PI_ERROR: u32 = 23;
pub const IRQ_PI_VI: u32 = 24;
pub const IRQ_PI_DEBUG: u32 = 25;
pub const IRQ_PI_HSP: u32 = 26;
pub const IRQ_MAX: u32 = 32;

// Macro declarations for getting address from mask.
macro_rules! irqmask {
    ($mask: expr) => {
        (0x80000000u32 >> $mask) as u32
    };
}

// MEM IRQs
pub const IM_MEM0: u32 = irqmask!(IRQ_MEM0);
pub const IM_MEM1: u32 = irqmask!(IRQ_MEM1);
pub const IM_MEM2: u32 = irqmask!(IRQ_MEM2);
pub const IM_MEM3: u32 = irqmask!(IRQ_MEM3);
pub const IM_MEMADDRESS: u32 = irqmask!(IRQ_MEMADDRESS);
pub const IM_MEM: u32 = IM_MEM0 | IM_MEM1 | IM_MEM2 | IM_MEM3 | IM_MEMADDRESS;

// DSP IRQs
pub const IM_DSP_AI: u32 = irqmask!(IRQ_DSP_AI);
pub const IM_DSP_ARAM: u32 = irqmask!(IRQ_DSP_ARAM);
pub const IM_DSP_DSP: u32 = irqmask!(IRQ_DSP_DSP);
pub const IM_DSP: u32 = IM_DSP_AI | IM_DSP_ARAM | IM_DSP_DSP;

// Streaming IRQ
pub const IM_AI: u32 = irqmask!(IRQ_AI);

// EXI IRQs
pub const IM_EXI0_EXI: u32 = irqmask!(IRQ_EXI0_EXI);
pub const IM_EXI0_TC: u32 = irqmask!(IRQ_EXI0_TC);
pub const IM_EXI0_EXT: u32 = irqmask!(IRQ_EXI0_EXT);
pub const IM_EXI0: u32 = IM_EXI0_EXI | IM_EXI0_TC | IM_EXI0_EXT;

pub const IM_EXI1_EXI: u32 = irqmask!(IRQ_EXI1_EXI);
pub const IM_EXI1_TC: u32 = irqmask!(IRQ_EXI1_TC);
pub const IM_EXI1_EXT: u32 = irqmask!(IRQ_EXI1_EXT);
pub const IM_EXI1: u32 = IM_EXI1_EXI | IM_EXI1_TC | IM_EXI1_EXT;

pub const IM_EXI2_EXI: u32 = irqmask!(IRQ_EXI2_EXI);
pub const IM_EXI2_TC: u32 = irqmask!(IRQ_EXI2_TC);
pub const IM_EXI2: u32 = IM_EXI2_EXI | IM_EXI2_TC;
pub const IM_EXI: u32 = IM_EXI0 | IM_EXI1 | IM_EXI2;
/* End EXI IRQs */

// Misc IRQs
pub const IM_PI_CP: u32 = irqmask!(IRQ_PI_CP);
pub const IM_PI_PETOKEN: u32 = irqmask!(IRQ_PI_PETOKEN);
pub const IM_PI_PEFINISH: u32 = irqmask!(IRQ_PI_PEFINISH);
pub const IM_PI_SI: u32 = irqmask!(IRQ_PI_SI);
pub const IM_PI_DI: u32 = irqmask!(IRQ_PI_DI);
pub const IM_PI_RSW: u32 = irqmask!(IRQ_PI_RSW);
pub const IM_PI_ERROR: u32 = irqmask!(IRQ_PI_ERROR);
pub const IM_PI_VI: u32 = irqmask!(IRQ_PI_VI);
pub const IM_PI_DEBUG: u32 = irqmask!(IRQ_PI_DEBUG);
pub const IM_PI_HSP: u32 = irqmask!(IRQ_PI_HSP);
// TODO: This is missing one IRQ for RVL.
pub const IM_PI: u32 = IM_PI_CP
    | IM_PI_PETOKEN
    | IM_PI_PEFINISH
    | IM_PI_SI
    | IM_PI_DI
    | IM_PI_RSW
    | IM_PI_ERROR
    | IM_PI_VI
    | IM_PI_DEBUG
    | IM_PI_HSP;
/* End Misc IRQs */

/// IRQ Priorities
pub(crate) static mut IRQ_PRIORITY: [u32; 12] = [
    IM_PI_ERROR,
    IM_PI_DEBUG,
    IM_MEM,
    IM_PI_RSW,
    IM_PI_VI,
    (IM_PI_PETOKEN | IM_PI_PEFINISH),
    IM_PI_HSP,
    (IM_DSP_ARAM | IM_DSP_DSP | IM_AI | IM_EXI | IM_PI_SI | IM_PI_DI),
    IM_DSP_AI,
    IM_PI_CP,
    0xffffffff, // TODO: This element is only available on RVL.
    0xffffffff,
];
