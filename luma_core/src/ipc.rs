use core::ptr;

//bits 0..=31 = physical address of ipc request
const HW_IPC_PPCMSG: *mut u32 = ptr::without_provenance_mut(0xCD00_0000);

//bit 0 = X1 | Execute IPC request
//bit 1 = Y2 | Acknowledge IPC request
//bit 2 = Y1 | IPC request reply available
//bit 3 = X2 | Relaunch IPC
//bit 4 = IY1 | IPC request reply send out IPC interrupt
//bit 5 = IY2 | IPC request acknowledge sends out IPC interrupt
const HW_IPC_PPCCTRL: *mut u32 = ptr::without_provenance_mut(0xCD00_0004);

//bits 0..=31 = physical address of ipc request
const HW_IPC_ARMMSG: *mut u32 = ptr::without_provenance_mut(0xCD00_0008);

//bit 0 = Y1 | IPC request reply available
//bit 1 = X2 | Relauch IPC
//bit 2 = X1 | Execute IPC request
//bit 3 = Y2 | Acknowledge IPC request
//bit 4 = IX1 | Execute ipc request send IPC interrupt
//bit 5 = IX2 | Relaunch IPC sends IPC interrupt
const HW_IPC_ARMCTRL: *mut u32 = ptr::without_provenance_mut(0xCD00_000C);

/// IPC Message Address (for BOTH ARM AND PPC)
#[repr(transparent)]
pub struct IpcMessageAddress(u32);

impl IpcMessageAddress {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn read_ppc() -> Self {
        Self(unsafe { HW_IPC_PPCMSG.read_volatile() })
    }

    pub fn write_ppc(self) {
        unsafe { HW_IPC_PPCMSG.write_volatile(self.0) }
    }

    pub fn read_arm() -> Self {
        Self(unsafe { HW_IPC_ARMMSG.read_volatile() })
    }

    pub fn write_arm(self) {
        unsafe { HW_IPC_ARMMSG.write_volatile(self.0) }
    }

    pub fn address(&self) -> u32 {
        self.0
    }

    /// # Panics:
    /// This function will panic if `address` is not in the MEM2 physical address space
    /// (0x1000_0000 - 0x13FF_FFFF)
    pub fn with_address(&mut self, address: u32) -> &mut Self {
        assert!(
            (0x1000_0000..0x1400_0000).contains(&address),
            "Address must be in physical space"
        );

        self.0 = bitfrob::u32_with_value(0, 31, self.0, address);
        self
    }
}

/// PowerPC IPC Control
#[repr(transparent)]
pub struct PpcIpcControl(u32);

impl PpcIpcControl {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn read() -> Self {
        Self(unsafe { HW_IPC_PPCCTRL.read_volatile() })
    }

    pub fn write(self) {
        unsafe { HW_IPC_PPCCTRL.write_volatile(self.0) }
    }

    pub fn execute(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 0)
    }

    pub fn with_execute(&mut self, execute: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 0, execute);
        self
    }

    pub fn acknowledge(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 1)
    }

    pub fn with_acknowledge(&mut self, acknowledge: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 1, acknowledge);
        self
    }

    pub fn reply(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 2)
    }

    pub fn with_reply(&mut self, reply: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 2, reply);
        self
    }

    pub fn relaunch(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 3)
    }

    pub fn with_relaunch(&mut self, relaunch: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 3, relaunch);
        self
    }

    pub fn reply_interrupt(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 4)
    }

    pub fn with_reply_interrupt(&mut self, reply_interrupt: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 4, reply_interrupt);
        self
    }

    pub fn acknowledge_interrupt(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 5)
    }

    pub fn with_acknowledge_interrupt(&mut self, acknowledge_interrupt: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 5, acknowledge_interrupt);
        self
    }
}

/// ARM IPC Control
#[repr(transparent)]
pub struct ArmIpcControl(u32);

impl ArmIpcControl {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn read() -> Self {
        Self(unsafe { HW_IPC_ARMCTRL.read_volatile() })
    }

    pub fn write(self) {
        unsafe { HW_IPC_ARMCTRL.write_volatile(self.0) }
    }

    pub fn execute(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 2)
    }

    pub fn with_execute(&mut self, execute: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 2, execute);
        self
    }

    pub fn acknowledge(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 3)
    }

    pub fn with_acknowledge(&mut self, acknowledge: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 3, acknowledge);
        self
    }

    pub fn reply(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 0)
    }

    pub fn with_reply(&mut self, reply: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 0, reply);
        self
    }

    pub fn relaunch(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 1)
    }

    pub fn with_relaunch(&mut self, relaunch: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 1, relaunch);
        self
    }

    pub fn execute_interrupt(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 4)
    }

    pub fn with_execute_interrupt(&mut self, execute_interrupt: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 4, execute_interrupt);
        self
    }

    pub fn relaunch_interrupt(&self) -> bool {
        bitfrob::u32_get_bit(self.0, 5)
    }

    pub fn with_relaunch_interrupt(&mut self, relaunch_interrupt: bool) -> &mut Self {
        self.0 = bitfrob::u32_with_bit(self.0, 5, relaunch_interrupt);
        self
    }
}
