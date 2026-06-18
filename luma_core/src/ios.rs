//! ``ios`` module of ``luma_core``.
//!
//! Contains functions for access to the Starlet when running IOS

use crate::cache::DCFlushRange;
use crate::io::{read32, write32};
use alloc::boxed::Box;

const BASE: u32 = 0x0d000000;

const HW_IPC_PPCMSG: u32 = BASE + 0;
const HW_IPC_PPCCTRL: u32 = BASE + 4;
const HW_IPC_ARMMSG: u32 = BASE + 8;

/// The type of a file descriptor in IOS.
pub type RawFd = i32;

/// How to open a file.
#[repr(i32)]
pub enum Mode {
    /// With no read/write access.
    None = 0,

    /// With read only access.
    Read = 1,

    /// With write only access.
    Write = 2,

    /// With read/write access.
    ReadWrite = 3,
}

/// This is copied from std::io::SeekFrom, with the inner types changed to accomodate IOS.
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum SeekFrom {
    /// From the start of the file.
    Start(i32),

    /// From the current position.
    Current(i32),

    /// From the end of the file.
    End(i32),
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
enum Command {
    Open = 1,
    Close = 2,
    Read = 3,
    Write = 4,
    Seek = 5,
    Ioctl = 6,
    Ioctlv = 7,
    Async = 8,
}

#[derive(Debug, Clone)]
#[repr(C, align(32))]
struct Ipc {
    command: Command,
    ret: i32,
    fd: RawFd,
    args: [i32; 5],
}

impl Ipc {
    #[inline]
    fn new(command: Command, args: [i32; 5]) -> Box<Self> {
        Box::new(Ipc {
            command,
            ret: 0,
            fd: -1,
            args,
        })
    }

    #[inline]
    fn with_fd(command: Command, fd: i32, args: [i32; 5]) -> Box<Self> {
        Box::new(Ipc {
            command,
            ret: 0,
            fd,
            args,
        })
    }

    #[inline(never)]
    fn send(self: Box<Self>) -> Box<Self> {
        let ptr = Box::into_raw(self);

        // Flush the IPC data from its cache line, so that the Starlet will see it.
        unsafe { DCFlushRange(ptr as *const _, core::mem::size_of::<Self>() as u32) };

        // Pass the pointer to IOS.
        write32(HW_IPC_PPCMSG, ptr as u32 & 0x1fff_ffff);

        // Signal to IOS we sent a command.
        let ppcctrl = read32(HW_IPC_PPCCTRL);
        if ppcctrl & 2 == 2 {
            // TODO: Find out why Dolphin signals a command has been acknowledged already on boot.
            write32(HW_IPC_PPCCTRL, 3);
        } else {
            write32(HW_IPC_PPCCTRL, 1);
        }

        // Busy loop until the Starlet acknowledges our command.
        loop {
            let ppcctrl = read32(HW_IPC_PPCCTRL);
            if ppcctrl & 2 == 2 {
                // Our command got acknowledged!
                write32(HW_IPC_PPCCTRL, 2);
                break;
            }
        }

        // Busy loop until the Starlet replies to our command.
        //
        // TODO: provide an async API to avoid having to do that, for queries which are quite long.
        loop {
            let ppcctrl = read32(HW_IPC_PPCCTRL);
            if ppcctrl & 4 == 4 {
                // We got a reply!
                break;
            }
        }

        // Read the reply from IOS.
        let armmsg = read32(HW_IPC_ARMMSG);
        let command = unsafe { Box::from_raw((armmsg | 0x8000_0000) as *mut Ipc) };
        assert_eq!(command.command, Command::Async);

        // Acknowledge the reply.
        write32(HW_IPC_PPCCTRL, 4);

        command
    }
}

fn get_physical_and_len(buf: &[u8]) -> (i32, i32) {
    let addr = buf.as_ptr();
    let len = buf.len() as i32;
    unsafe { DCFlushRange(addr as *const _, len as u32) };
    (addr as i32 & 0x1fff_ffff, len)
}

/// Open a file and return a new fd.
pub fn open(filename: &str, mode: Mode) -> Result<RawFd, i32> {
    let (addr, len) = get_physical_and_len(filename.as_bytes());
    // XXX: why 0x40?
    assert!(len < 0x40);
    let request = Ipc::new(Command::Open, [addr, mode as i32, 0, 0, 0]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(reply.ret)
    }
}

/// Close an open fd.
pub fn close(fd: RawFd) -> Result<(), i32> {
    let request = Ipc::with_fd(Command::Close, fd, [0; 5]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(())
    }
}

/// Read from an open fd.
pub fn read(fd: RawFd, buf: &mut [u8]) -> Result<i32, i32> {
    let (addr, len) = get_physical_and_len(buf);
    let request = Ipc::with_fd(Command::Read, fd, [addr, len, 0, 0, 0]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(reply.ret)
    }
}

/// Write into an open fd.
pub fn write(fd: RawFd, buf: &[u8]) -> Result<i32, i32> {
    let (addr, len) = get_physical_and_len(buf);
    let request = Ipc::with_fd(Command::Write, fd, [addr, len, 0, 0, 0]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(reply.ret)
    }
}

/// Seek in an open fd.
pub fn seek(fd: RawFd, pos: SeekFrom) -> Result<(), i32> {
    let (pos, whence) = match pos {
        SeekFrom::Start(pos) => (pos, 0),
        SeekFrom::Current(pos) => (pos, 1),
        SeekFrom::End(pos) => (pos, 2),
    };
    let request = Ipc::with_fd(Command::Seek, fd, [pos, whence, 0, 0, 0]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(())
    }
}

/// Perform a specific action on an open fd.
pub fn ioctl(fd: RawFd, num: i32, buf1: &[u8], buf2: &[u8]) -> Result<i32, i32> {
    let (addr1, len1) = get_physical_and_len(buf1);
    let (addr2, len2) = get_physical_and_len(buf2);
    let request = Ipc::with_fd(Command::Ioctl, fd, [num, addr1, len1, addr2, len2]);
    let reply = request.send();
    if reply.ret < 0 {
        Err(reply.ret)
    } else {
        Ok(reply.ret)
    }
}
