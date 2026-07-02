//! A test executable to verify GameCube controller input via the Serial Interface.

#![no_std]

extern crate luma_core;
extern crate luma_runtime;

use core::fmt::Write;

use luma_core::pad::Gamepad;
use luma_core::println;
use luma_core::serial_interface::SiPort;

fn main() {
    println!("HELLO");
    loop {}
}
