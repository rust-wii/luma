//! ``register`` module of ``luma_core``.
//!
//! Contains functions for register instructions.

/// PowerPC Architecture Defined SPRs
pub enum ArchSPR {
    /// Operating System Specific
    SPRG(u8),
}

/// PowerPC Implementation-Specific Registers
pub enum ImplRegister {
    /// Hardware Registers
    HID(u8),
    /// Performance Monitor Registers
    PMC(u8),
    /// Monitor Mode Control Registers
    MMCR(u8),
    /// Write Pipe Address Register
    WPAR,
}

impl ImplRegister {
    /// (`mfspr`) PowerPC Register Instruction
    pub fn mfspr(&self) -> u32 {
        // Create an output variable.
        let mut reg_value;

        // Match the register.
        match self {
            ImplRegister::HID(n) => {
                // Match the number of the register.
                match n {
                    0 => unsafe {
                        asm!("mfspr $0,HID0" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    1 => unsafe {
                        asm!("mfspr $0,HID1" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    2 => unsafe {
                        asm!("mfspr $0,HID2" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    4 => unsafe {
                        asm!("mfspr $0,HID4" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::PMC(n) => {
                // Match the number of the register.
                match n {
                    1 => unsafe {
                        asm!("mfspr $0,PMC1" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    2 => unsafe {
                        asm!("mfspr $0,PMC2" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    3 => unsafe {
                        asm!("mfspr $0,PMC3" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    4 => unsafe {
                        asm!("mfspr $0,PMC4" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::MMCR(n) => {
                // Match the number of the register.
                match n {
                    0 => unsafe {
                        asm!("mfspr $0,MMCR0" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    1 => unsafe {
                        asm!("mfspr $0,MMCR1" : "=r"(reg_value) ::: "volatile");
                        reg_value
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::WPAR => {
                // Run the assembly instruction.
                unsafe {
                    asm!("mfspr $0,WPAR" : "=r"(reg_value) ::: "volatile");
                    reg_value
                }
            }
        }
    }

    /// (`mtspr`) PowerPC Register Instruction
    pub fn mtspr(&self, value: u32) {
        // Match the register.
        match self {
            ImplRegister::HID(n) => {
                // Match the number of the register.
                match n {
                    0 => unsafe {
                        asm!("mtspr HID0,$0" :: "r"(value) :: "volatile");
                    },
                    1 => unsafe {
                        asm!("mtspr HID1,$0" :: "r"(value) :: "volatile");
                    },
                    2 => unsafe {
                        asm!("mtspr HID2,$0" :: "r"(value) :: "volatile");
                    },
                    4 => unsafe {
                        asm!("mtspr HID4,$0" :: "r"(value) :: "volatile");
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::PMC(n) => {
                // Match the number of the register.
                match n {
                    1 => unsafe {
                        asm!("mtspr PMC1,$0" :: "r"(value) :: "volatile");
                    },
                    2 => unsafe {
                        asm!("mtspr PMC2,$0" :: "r"(value) :: "volatile");
                    },
                    3 => unsafe {
                        asm!("mtspr PMC3,$0" :: "r"(value) :: "volatile");
                    },
                    4 => unsafe {
                        asm!("mtspr PMC4,$0" :: "r"(value) :: "volatile");
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::MMCR(n) => {
                // Match the number of the register.
                match n {
                    0 => unsafe {
                        asm!("mtspr MMCR0,$0" :: "r"(value) :: "volatile");
                    },
                    1 => unsafe {
                        asm!("mtspr MMCR1,$0" :: "r"(value) :: "volatile");
                    },
                    _ => panic!("Unknown register given"),
                }
            }
            ImplRegister::WPAR => {
                // Run the assembly instruction.
                unsafe {
                    asm!("mtspr WPAR,$0" :: "r"(value) :: "volatile");
                }
            }
        }
    }
}

/// (`mfpvr`) PowerPC Register Instruction
pub fn mfpvr() -> u32 {
    // Define a register output variable.
    let mut register;

    // Run the assembly instruction.
    unsafe {
        asm!("mfpvr $0" : "=r"(register) ::: "volatile");
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
        asm!("mfmsr $0" : "=r"(register) ::: "volatile");
    }

    // Return the register value.
    register
}

/// (`mtmsr`) PowerPC Register Instruction
#[inline(always)]
pub fn mtmsr(value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("mtmsr $0" :: "r"(value) :: "volatile");
    }
}

/// (`mtdec`) PowerPC Register Instruction
#[inline(always)]
pub fn mtdec(value: u32) {
    // Run the assembly instruction.
    unsafe {
        asm!("mtdec $0" :: "r"(value) :: "volatile");
    }
}