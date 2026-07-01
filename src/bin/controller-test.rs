//! A test executable to verify GameCube controller input via the Serial Interface.

#![no_std]

extern crate luma_core;
extern crate luma_runtime;

use core::fmt::Write;

use luma_core::pad::Gamepad;
use luma_core::println;
use luma_core::serial_interface::SiPort;

fn main() {
    println!("GameCube Controller Test\n");
    println!("Press buttons on the controller in Port 1.\n");

    let pad1 = Gamepad::new(SiPort::One);

    let mut last_state = pad1.poll();

    loop {
        let current_state = pad1.poll();

        // Avoiding infinite loop spam
        if current_state != last_state {
            println!(
                "A: {} | B: {} | X: {} | Y: {} | Stick: ({:03}, {:03}) | C-Stick: ({:03}, {:03}) | L/R: ({:03}, {:03})\n",
                current_state.button_a() as u8,
                current_state.button_b() as u8,
                current_state.button_x() as u8,
                current_state.button_y() as u8,
                current_state.stick_x(),
                current_state.stick_y(),
                current_state.c_stick_x(),
                current_state.c_stick_y(),
                current_state.trigger_l(),
                current_state.trigger_r(),
            );

            last_state = current_state;
        }
    }
}
