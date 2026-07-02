//! ``pad`` module of ``luma_core``.
//!
//! Contains an abstraction for reading GameCube
//! controllers over the Serial Interface (SI).
//! In a future PR would like to enhance to a more generalized
//! traits-based "JoyBusDevice" setup.

use crate::serial_interface::{SiChannel, SiPort};

/// The standard Joy-Bus command to read controller state without rumble.
const CMD_READ_STATUS: u32 = 0x4003_0000;

/// A snapshot of the controller's state at a point in time.
/// - Holds the raw 8-byte payload from the Serial Interface.
#[derive(Copy, Clone, PartialEq)]
pub struct PadState {
    pub word1: u32,
    pub word2: u32,
}

impl PadState {
    // Face Buttons
    #[inline(always)]
    pub fn button_start(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 28)
    }

    #[inline(always)]
    pub fn button_y(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 27)
    }

    #[inline(always)]
    pub fn button_x(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 26)
    }

    #[inline(always)]
    pub fn button_b(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 25)
    }

    #[inline(always)]
    pub fn button_a(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 24)
    }

    // Shoulder Buttons (Digital Clicks)

    #[inline(always)]
    pub fn shoulder_l(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 22)
    }

    #[inline(always)]
    pub fn shoulder_r(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 21)
    }

    #[inline(always)]
    pub fn button_z(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 20)
    }

    // Direction Pad

    #[inline(always)]
    pub fn dpad_up(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 19)
    }

    #[inline(always)]
    pub fn dpad_down(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 18)
    }

    #[inline(always)]
    pub fn dpad_right(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 17)
    }

    #[inline(always)]
    pub fn dpad_left(&self) -> bool {
        bitfrob::u32_get_bit(self.word1, 16)
    }

    // Main Stick (Analogue 0-255, center is ~128)

    #[inline(always)]
    pub fn stick_x(&self) -> u8 {
        bitfrob::u32_get_value(8, 15, self.word1) as u8
    }

    #[inline(always)]
    pub fn stick_y(&self) -> u8 {
        bitfrob::u32_get_value(0, 7, self.word1) as u8
    }

    // C-Stick (Analogue 0-255, center is ~128)

    #[inline(always)]
    pub fn c_stick_x(&self) -> u8 {
        bitfrob::u32_get_value(24, 31, self.word2) as u8
    }

    #[inline(always)]
    pub fn c_stick_y(&self) -> u8 {
        bitfrob::u32_get_value(16, 23, self.word2) as u8
    }

    // Triggers (Analogue 0-255)

    #[inline(always)]
    pub fn trigger_l(&self) -> u8 {
        bitfrob::u32_get_value(8, 15, self.word2) as u8
    }

    #[inline(always)]
    pub fn trigger_r(&self) -> u8 {
        bitfrob::u32_get_value(0, 7, self.word2) as u8
    }
}

/// Stateless hardware connection to a specific GameCube controller port.
pub struct Gamepad {
    channel: SiChannel,
}

impl Gamepad {
    /// Binds Gamepad connection to one of the 4 physical console ports.
    #[inline(always)]
    pub const fn new(port: SiPort) -> Self {
        Self {
            channel: SiChannel::new(port),
        }
    }

    /// Starts a synchronous hardware transfer to request input data.
    ///
    /// Blocks the CPU for the duration of the Joy-Bus transfer, returning
    /// a `PadState` snapshot with the state of the controller.
    #[inline(always)]
    pub fn poll(&self) -> PadState {
        self.channel.write(CMD_READ_STATUS);

        self.channel.begin_transfer(3, 8);

        let (word1, word2) = self.channel.read();

        PadState { word1, word2 }
    }
}
