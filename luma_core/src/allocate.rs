use alloc::alloc::{alloc, Layout};
use alloc::boxed::Box;
use core::pin::Pin;
use core::slice;

const CACHELINE: usize = 32;

/// Allocate a slice aligned to a cacheline, and return it pinned.
pub fn alloc_aligned<T: Copy>(size: usize) -> Pin<Box<[T]>> {
    let layout = Layout::array::<T>(size)
        .unwrap()
        .align_to(CACHELINE)
        .unwrap();
    let ptr = unsafe { alloc(layout) } as *mut T;
    let slice = unsafe { slice::from_raw_parts(ptr, size) };
    let boxed = Box::from(slice);
    Pin::from(boxed)
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
