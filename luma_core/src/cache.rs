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
    /// 
    /// **NOTE**: Cache access is blocked during this time.
    pub fn DCFlashInvalidate();

    /// Locks the current contents of the L1 d-cache.
    /// 
    /// **NOTE**: Most cache operations, will still execute regardless of whether the cache is frozen.
    pub fn DCLock();

    /// Unlocks the current contents of the L1 d-cache.
    pub fn DCUnlock();
}