//! ``luma_runtime`` is the runtime module of ``luma``.
//!
//! This module implements runtime functions and allocators required for ``no_std`` on the Wii.
//! This module also includes a crt0 implementation for bootstrapping the program.
//!
//! **NOTE**: This is currently in a very experimental state and is subject to change.
#![no_std]
#![feature(global_asm, lang_items, llvm_asm)]

use core::panic::PanicInfo;

// crt0 Implementation
global_asm!(include_str!("../asm/crt0.S"));
global_asm!(include_str!("../asm/runtime.S"));
global_asm!(include_str!("../asm/system.S"));

/// This is the executable start function, which directly follows the entry point.
#[cfg_attr(not(test), lang = "start")]
#[cfg(not(test))]
extern "C" fn start<T>(user_main: *const (), _argc: isize, _argv: *const *const u8) -> !
where
    T: Termination,
{
    let user_main: fn() -> T = unsafe { core::mem::transmute(user_main) };
    user_main();

    panic!("main() cannot return");
}

/// Termination trait required for the start function.
#[cfg_attr(not(test), lang = "termination")]
trait Termination {}

/// This implementation does the bare minimum to satisfy the executable start function.
impl Termination for () {}

/// This function is called on panic.
#[cfg_attr(not(test), panic_handler)]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // A loop without side effects may be optimized away by LLVM. This issue can be avoided with
        // a volatile no-op. See: https://github.com/rust-lang/rust/issues/28728
        unsafe { llvm_asm!("" :::: "volatile") };
    }
}

/// Re-export libc's memset to be used in ``crt0.S``.
///
/// # Safety
///
/// Refer to the platform's libc implementation for more information.
#[no_mangle]
pub unsafe extern "C" fn memset(
    dest: *mut libc::c_void,
    c: libc::c_int,
    n: libc::size_t,
) -> *mut libc::c_void {
    libc::memset(dest, c, n)
}
