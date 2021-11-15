//! ``luma_core`` is the core module of ``luma``.
//!
//! This module contains core processor features.
//!
//! **NOTE**: This is currently in a very experimental state and is subject to change.
#![no_std]
#![allow(unused_attributes)]
#![feature(
    global_asm,
    asm,
    asm_experimental_arch,
    box_into_boxed_slice,
    allocator_api
)]

extern crate alloc;

// Broadway Processor Utilities
pub mod processor;

// Broadway Register Utilities
pub mod register;

// Broadway Integer Utilities
pub mod integer;

// Broadway Load and Store Utilities
pub mod loadstore;

// Broadway I/O Utilities
pub mod io;

// Broadway Cache Subsystem
pub mod cache;

// Helper functions to allocate aligned memory on the heap
pub mod allocate;

// VI Subsystem
pub mod vi;
