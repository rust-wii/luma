//! ``exi`` module of ``luma_core``.
//!
//! Contains functions to access the EXI bus.

use crate::io::{read32, write32};

// Are we really on a GameCube in Dolphin?
// TODO: Use 0xcd006800 instead when on a Wii.
const EXI: u32 = 0xcc006800;

const EXI_CSR: u32 = EXI + 0;
const EXI_DMA_ADDR: u32 = EXI + 4;
const EXI_DMA_SIZE: u32 = EXI + 8;
const EXI_CR: u32 = EXI + 12;
const EXI_DATA: u32 = EXI + 16;

bitflags::bitflags! {
    struct Clock: u32 {
        const CLOCK_1MHZ = 0;
        const CLOCK_2MHZ = 1;
        const CLOCK_4MHZ = 2;
        const CLOCK_8MHZ = 3;
        const CLOCK_16MHZ = 4;
        const CLOCK_32MHZ = 5;
    }
}

bitflags::bitflags! {
    struct Cr: u32 {
        const TLEN8 = 0x00;
        const TLEN16 = 0x10;
        const TLEN24 = 0x20;
        const TLEN32 = 0x30;
        const READ = 0x00;
        const WRITE = 0x04;
        const RW = 0x08;
        const IMMEDIATE = 0x00;
        const DMA = 0x02;
        const TSTART = 0x01;
    }
}

/// Empty struct which represents initialised access to the EXI bus.
pub struct Exi;

impl Exi {
    /// Initialises the EXI bus.
    pub fn init() -> Exi {
        let exi = Exi;
        for i in 0..2 {
            exi.get_channel(i).deselect_device();
        }
        exi
    }

    fn get_channel(&self, channel: u32) -> Channel {
        Channel { exi: self, channel }
    }
}

struct Channel<'a> {
    channel: u32,
    exi: &'a Exi,
}

impl<'a> Channel<'a> {
    fn select_device(&self, device: u32, clock: Clock) {
        write32(
            EXI_CSR + 20 * self.channel,
            ((1 << device) << 7) | (clock.bits() << 4),
        );
    }

    fn deselect_device(&self) {
        write32(EXI_CSR + 20 * self.channel, 0);
    }

    fn write_data(&self, data: u32) {
        write32(EXI_DATA + 20 * self.channel, data);
    }

    fn read_data(&self) -> u32 {
        read32(EXI_DATA + 20 * self.channel)
    }

    fn write_cr(&self, cr: Cr) {
        write32(EXI_CR + 20 * self.channel, cr.bits());
    }

    fn wait_for_completion(&self) {
        while (read32(EXI_CR + 20 * self.channel) & Cr::TSTART.bits()) != 0 {}
    }
}
