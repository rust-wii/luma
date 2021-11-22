//! This is an example of how to communicate with an USB Gecko using Luma.

#![no_std]

extern crate luma_core;
extern crate luma_runtime;

use luma_core::exi::usb_gecko::UsbGecko;
use luma_core::exi::Exi;

fn main() {
    let exi = Exi::init();
    let gecko = UsbGecko::new(&exi).unwrap();
    loop {
        // TODO: use interrupts here, instead of a busy loop.
        let buf = match gecko.receive() {
            Ok(buf) => buf,
            Err(_) => continue,
        };
        gecko.send(&buf).unwrap();
    }
}
