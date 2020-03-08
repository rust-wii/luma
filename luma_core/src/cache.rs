//! ``cache`` module of ``luma_core``.
//!
//! Contains functions for the L1, L2, Data and Instruction caches.

global_asm!(include_str!("../asm/cache.S"));

// Load cache functions from global assembly.
extern "C" {
    /// Enable the L1 d(ata)-cache.
    /// 
    /// **NOTE**: This function calls ``sync`` before enabling.
    pub fn DCEnable();

    /// Disable the L1 d(ata)-cache.
    /// 
    /// When the data cache is **disabled**, all accesses
    /// are propagated to the L2 cache or 60x bus.
    /// 
    /// **NOTE**: This function calls ``sync`` before disabling.
    pub fn DCDisable();
    
    /// Invalidate the L1 d(ata)-cache.
    /// 
    /// **NOTE**: This function calls ``sync`` before invalidating.
    pub fn DCFlashInvalidate();

    /// Locks the current contents of the L1 d(ata)-cache.
    /// 
    /// A data access that hits the locked data cache is serviced
    /// by the cache. However, all accesses that miss the locked cache
    /// are propagated to the L2 cache.
    /// 
    /// **NOTE**: This function calls ``sync`` before locking.
    pub fn DCLock();

    /// Unlocks the current contents of the L1 d(ata)-cache.
    /// 
    /// **NOTE**: This function calls ``sync`` before locking.
    pub fn DCUnlock();

    /// Invalidates a given range of the L1 d(ata)-cache.
    ///
    /// **NOTE**:
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn DCInvalidateRange(start_address: *const u32, length: u32);

    /// Flushes a given range of the L1 d(ata)-cache.
    /// 
    /// If any part of the range hits in the d(ata)-cache,
    /// the corresponding block will be flushed to main memory and invalidated.
    /// 
    /// **NOTE**:
    /// * This function invokes a "sync" after flushing the range.
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn DCFlushRange(start_address: *const u32, length: u32);

    /// Ensures a range of memory is updated with any modified data in the d(ata)-cache.
    /// 
    /// **NOTE**:
    /// * This function invokes a "sync" after storing the range.
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn DCStoreRange(start_address: *const u32, length: u32);

    /// Flushes a given range of the L1 d(ata)-cache without ``sync``.
    /// 
    /// If any part of the range hits in the d(ata)-cache,
    /// the corresponding block will be flushed to main memory and invalidated.
    /// 
    /// **WARNING**: This routine does not perform a "sync" after flushing the range.
    /// The flushed cache blocks are **NOT** guaranteed to be in memory by the time you run
    /// the next routine.
    /// 
    /// **NOTE**:
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn DCFlushRangeNS(start_address: *const u32, length: u32);

    /// Ensures a range of memory is updated with any modified data in the cache without ``sync``.
    ///
    /// **WARNING**: This routine does not perform a "sync" after storing the range.
    /// The flushed cache blocks are **NOT** guaranteed to be in memory by the time you run
    /// the next routine.
    /// 
    /// **NOTE**:
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn DCStoreRangeNS(start_address: *const u32, length: u32);

    /// Enable the L1 i(nstruction)-cache.
    /// 
    /// **NOTE**: This function is preceded by ``isync`` when enabling.
    pub fn ICEnable();

    /// Disable the L1 i(nstruction)-cache.
    /// 
    /// When the instruction cache is **disabled**, all instruction fetches
    /// are propagated to the L2 cache or 60x bus.
    /// 
    /// **NOTE**: This function is preceded by ``isync`` when disabling.
    pub fn ICDisable();

    /// Locks the current contents of the L1 i(nstruction)-cache.
    /// 
    /// A instruction fetch that hits the locked instruction cache is serviced
    /// by the cache. However, all accesses that miss the locked cache
    /// are propagated to the L2 cache.
    /// 
    /// **NOTE**: This function is preceded by ``isync`` when locking.
    pub fn ICLock();

    /// Unlocks the current contents of the L1 i(nstruction)-cache.
    /// 
    /// **NOTE**: This function is preceded by ``isync`` when locking.
    pub fn ICUnlock();

    /// Invalidate the L1 i(nstruction)-cache.
    /// 
    /// Cache access is **blocked** during this time. 
    /// Bus accesses to the cache are signaled as a miss during invalidate operations.
    /// 
    /// **NOTE**: This function is preceded by ``isync`` when locking.
    pub fn ICFlashInvalidate();

    /// Invalidates a block in the i(nstruction)-cache.
    /// 
    /// If the block hits in the range, the corresponding block will be invalidated.
    /// 
    /// **NOTE**:
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    pub fn ICBlockInvalidate(start_address: *const u32);

    /// Invalidate a range in the L1 i(nstruction)-cache.
    /// 
    /// **NOTE**:
    /// * The starting addreses given **MUST** be aligned on a 32 byte boundary.
    /// * The length of the range to invalidate should be a multiple of 32.
    pub fn ICInvalidateRange(start_address: *const u32, length: u32);
}