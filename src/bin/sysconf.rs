//! This is an example of how to read the Wii system settings using Luma.

#![no_std]

extern crate alloc;
extern crate luma_core;
extern crate luma_runtime;

use core::fmt::Write;
use luma_core::println;
use luma_core::sysconf;

fn main() {
    let sysconf = sysconf::read_and_parse().unwrap();
    println!("Aspect ratio {:?}", sysconf.aspect_ratio());
    println!("{:?}", sysconf.refresh_rate());
    println!("{:?}", sysconf.progressive());

    loop {}
}
