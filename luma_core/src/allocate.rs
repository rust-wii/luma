use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use core::pin::Pin;

const CACHELINE: usize = 32;

/// Allocate a slice aligned to a cacheline, and return it pinned.
pub fn alloc_aligned(size: usize) -> Pin<Box<[u8]>> {
    let layout = Layout::from_size_align(size, CACHELINE).unwrap();
    let ptr = unsafe { alloc(layout) };
    let boxed = unsafe { Box::from_raw(ptr) };
    let slice = Box::into_boxed_slice(boxed);
    Pin::from(slice)
}

/// Allocate an array aligned to a cacheline, and return it pinned.
pub fn alloc_array_aligned<const LENGTH: usize>() -> Pin<Box<[u8; LENGTH]>> {
    let layout = Layout::from_size_align(LENGTH, CACHELINE).unwrap();
    let ptr = unsafe { alloc(layout) };
    let array = ptr as *mut [u8; LENGTH];
    let boxed = unsafe { Box::from_raw(array) };
    Pin::from(boxed)
}

/// Convert a raw pointer and its length into a pinned array.
pub unsafe fn ptr_as_pinned_array<T: Copy, const LENGTH: usize>(
    ptr: *mut T,
) -> Pin<Box<[T; LENGTH]>> {
    let array = ptr as *mut [T; LENGTH];
    let boxed = Box::from_raw(array);
    Pin::from(boxed)
}
