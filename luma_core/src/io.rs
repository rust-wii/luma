//! ``io`` module of ``luma_core``.
//!
//! Contains functions for basic I/O.

/// Read a 32-bit value from an address.
#[inline(always)]
pub fn read32(address: u32) -> u32 {
    // Define an output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("lwz {0},0({1}) ; sync",
            lateout(reg) register, 
            in(reg) (0xc000_0000 | address),
            options(nostack));
    }

    // Return the register value.
    register
}

/// Write a 32-bit value to an address.
#[inline(always)]
pub fn write32(address: u32, value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("stw {0},0({1}) ; eieio", 
            in(reg) value, in(reg) (0xc000_0000 | address), 
            options(nostack));
    }
}

/// Read a 16-bit value from an address.
#[inline(always)]
pub fn read16(address: u32) -> u16 {
    // Define an output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("lhz {0},0({1}) ; sync", 
            lateout(reg) register,
            in(reg) (0xc000_0000 | address), 
            options(nostack));
    }

    // Return the register value.
    register
}

/// Write a 16-bit value to an address.
#[inline(always)]
pub fn write16(address: u32, value: u16) {
    // Run the assembly instruction.
    unsafe {
        asm!("sth {0},0({1}) ; eieio",
            in(reg) value, in(reg) (0xc000_0000 | address), 
            options(nostack));
    }
}

/// Read a 8-bit value from an address.
#[inline(always)]
pub fn read8(address: u32) -> u8 {
    // Define an output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("lbz {0},0({1}) ; sync",
            lateout(reg) register,
            in(reg) (0xc000_0000 | address),
            options(nostack));
    }

    // Return the register value.
    register
}

/// Write a 8-bit value to an address.
#[inline(always)]
pub fn write8(address: u32, value: u8) {
    // Run the assembly instruction.
    unsafe {
        asm!("stb {0},0({1}) ; eieio",
            in(reg) value, in(reg) (0xc000_0000 | address),
            options(nostack));
    }
}

/// Write a 32-bit floating value to an address.
#[inline(always)]
pub fn writef32(address: u32, value: f32) {
    // Run the assembly instruction.
    unsafe {
        asm!("stfs {0},0({1}) ; eieio",
            in(freg) value, in(reg) (0xc000_0000 | address),
            options(nostack));
    }
}
