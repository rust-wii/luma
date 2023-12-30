//! This is an example of how to shutdown the Wii using Luma.

#![no_std]

extern crate luma_core;
extern crate luma_runtime;

use luma_core::ios;

fn main() {
    let fd = ios::open("/dev/stm/immediate\0", ios::Mode::None).unwrap();
    ios::ioctl(fd, 0x2003, &[], &[]).unwrap();
    loop {}
}
