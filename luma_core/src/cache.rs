//! ``cache`` module of ``luma_core``.
//!
//! Contains functions for the L1, L2, D and I caches.

global_asm!(include_str!("../asm/cache.S"));

// Load functions from global assembly.
extern "C" {
    /// Enable the L1 d-cache.
    pub fn DCEnable();

    /// Disable the L1 d-cache.
    pub fn DCDisable();
    
    /// Invalidate the L1 d-cache.
    pub fn DCFlashInvalidate();

    /// Locks the current contents of the L1 d-cache.
    pub fn DCLock();

    /// Unlocks the current contents of the L1 d-cache.
    pub fn DCUnlock();

    /// Invalidates a given range of the L1 d-cache.
    pub fn DCInvalidateRange(start_address: *const u32, length: u32);

    /// Flushes a given range of the L1 d-cache.
    pub fn DCFlushRange(start_address: *const u32, length: u32);

    /// Ensures a range of memory is updated with any modified data in the cache.
    pub fn DCStoreRange(start_address: *const u32, length: u32);

    /// Flushes a given range of the L1 d-cache.
    pub fn DCFlushRangeNS();

    /// Ensures a range of memory is updated with any modified data in the cache.
    pub fn DCStoreRangeNS();

    /// Enable the L1 i-cache.
    pub fn ICEnable();

    /// Disable the L1 i-cache.
    pub fn ICDisable();

    /// Locks the current contents of the L1 i-cache.
    pub fn ICLock();

    /// Unlocks the current contents of the L1 i-cache.
    pub fn ICUnlock();

    /// Invalidate the L1 i-cache.
    pub fn ICFlashInvalidate();

    /// Invalidates a block in the i-cache.
    pub fn ICBlockInvalidate(start_address: *const u32);

    /// Invalidate a range in the L1 i-cache.
    pub fn ICInvalidateRange(start_address: *const u32, length: u32);
}