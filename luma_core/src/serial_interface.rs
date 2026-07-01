use crate::io::{read32, write32};

const SI_BASE: u32 = 0xCC006400;

/// SI Control Registers
const SI_POLL_REG: u32 = SI_BASE + 0x30;
const SI_COMM_REG: u32 = SI_BASE + 0x34;
const SI_STATUS_REG: u32 = SI_BASE + 0x38;
const SI_EXI_CLOCK_LOCK: u32 = SI_BASE + 0x3C;

/// Extended Transfer Buffer
/// Shared 128-byte extended I/O buffer for higher-bandwidth SI devices such as:
/// - GBA Link Cable
/// - Gamecube Keyboard
/// - Wavebird Wireless Receiver
/// All 4 ports share this buffer! Be careful!
const SI_IO_BUFFER: u32 = SI_BASE + 0x80;

// SI Communication & Status Register Wrapper
#[repr(transparent)]
pub struct SiCommControl(u32);

impl SiCommControl {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn read() -> Self {
        Self(read32(SI_COMM_REG))
    }

    #[inline(always)]
    pub fn write(self) {
        write32(SI_COMM_REG, self.0)
    }

    /// Check if a transfer is currently active (TSTART bit)
    #[inline(always)]
    pub fn transfer_active(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 31)
    }

    /// Set the TSTART bit to begin a transfer
    #[inline(always)]
    pub fn with_start(&mut self, start: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 31, start);
        self
    }

    /// Set the target channel (0-3)
    #[inline(always)]
    pub fn with_channel(&mut self, channel: u32) -> &mut Self {
        self.0 = bitfrob::u32_with_value(28, 29, self.0, channel);
        self
    }

    /// Set the expected input length in bytes (usually 8 for normal controllers)
    #[inline(always)]
    pub fn with_in_len(&mut self, len: u32) -> &mut Self {
        self.0 = bitfrob::u32_with_value(16, 22, self.0, len);
        self
    }

    /// Set the output length in bytes (usually 3 for normal commands)
    #[inline(always)]
    pub fn with_out_len(&mut self, len: u32) -> &mut Self {
        self.0 = bitfrob::u32_with_value(0, 6, self.0, len);
        self
    }
}

/// Represents one of four Gamecube controller ports.
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SiPort {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

/// Represents a physical serial-interface channel
pub struct SiChannel {
    port_index: u32,
    base_addr: u32,
}

impl SiChannel {
    #[inline(always)]
    pub const fn new(port: SiPort) -> Self {
        let port_index = port as u32;
        Self {
            port_index,
            // Each channel's registers are spaced 0x0C apart
            base_addr: SI_BASE + (port_index * 0x0C),
        }
    }

    /// Writes data to the specific channel's output buffer
    #[inline(always)]
    pub fn write(&self, command: u32) {
        // SI_OUT_BUF is at offset 0x00 from the channel base
        write32(self.base_addr, command);
    }

    /// Reads the 8-byte response from this channel's input buffers
    #[inline(always)]
    pub fn read(&self) -> (u32, u32) {
        // SI_IN_BUF_HI is at offset 0x04
        // SI_IN_BUF_LO is at offset 0x08
        let hi = read32(self.base_addr + 0x04);
        let lo = read32(self.base_addr + 0x08);
        (hi, lo)
    }

    /// Starts a transfer and spins until completion.
    ///
    /// `out_bytes` is the size of the command written to `write `.
    /// `in_bytes` is the expected size of the response.
    #[inline(always)]
    pub fn begin_transfer(&self, out_bytes: u32, in_bytes: u32) {
        let mut comm_ctrl = SiCommControl::read();

        comm_ctrl
            .with_channel(self.port_index)
            .with_out_len(out_bytes)
            .with_in_len(in_bytes)
            .with_start(true);

        comm_ctrl.write();

        // Spin-lock until the hardware clears the TSTART bit
        while SiCommControl::read().transfer_active() {
            core::hint::spin_loop();
        }
    }
}
