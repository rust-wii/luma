// ======================== //
//	   Runtime Assembly	    //
// ======================== //

.extern __stack_addr
.extern InitCache,InitPS,InitFPRS

// --------------------------------------------------------------- //

.globl InitHardware
InitHardware:
	// Enable the Floating Point Registers
	mfmsr   3
	ori     3,3,0x00002000 // MSR_FP			
	mtmsr   3

	mflr	31
	bl		InitPS 		// Initialize Paired Singles
	bl		InitFPRS	// Initialize the FPR's
	bl		InitCache 	// Initialize the system caches
	mtlr	31
	blr

.globl ClearGPRS
ClearGPRS:
	// Clear all GPRs
	li      0,0
	li      3,0
	li      4,0
	li      5,0
	li      6,0
	li      7,0
	li      8,0
	li      9,0
	li      10,0
	li      11,0
	li      12,0
	li      14,0
	li      15,0
	li      16,0
	li      17,0
	li      18,0
	li      19,0
	li      20,0
	li      21,0
	li      22,0
	li      23,0
	li      24,0
	li      25,0
	li      26,0
	li      27,0
	li      28,0
	li      29,0
	li      30,0
	li      31,0

	lis		1,__stack_addr@h	// Use a 128 KiB stack, defined in linker.ld.
	ori		1,1,__stack_addr@l
	addi	1,1,-4
	stw		0,0(1)
	stwu	1,-56(1)

	lis     2,_SDA2_BASE_@h
	ori     2,2,_SDA2_BASE_@l   // Set the Small Data 2 (Read Only) base register.
	lis     13,_SDA_BASE_@h
	ori     13,13,_SDA_BASE_@l  // Set the Small Data (Read\Write) base register.	
	blr

.globl ClearBATS
ClearBATS:
	mflr	31
	oris	31,31,0x8000
	lis		3,ConfigBATS@h
	ori		3,3,ConfigBATS@l
	bl		__realmode
	mtlr	31
	blr

__realmode:
	clrlwi	3,3,2
	mtsrr0	3
	mfmsr	3
	rlwinm	3,3,0,28,25
	mtsrr1	3
	rfi

ConfigBATS:
	// HID0 = 00110c64:
	// bus checkstops off, sleep modes off,
	// caches off, caches invalidate,
	// store gathering off, enable data cache
	// flush assist, enable branch target cache,
	// enable branch history table
	lis 3,0x0011 ; ori 3,3,0x0c64 ; mtspr 1008,3 ; isync
	
	lis	3,0x8200    // bits set: H4A(HID4 access), SBE(2nd BAT enabled)
	mtspr 1011,3    // HID4
	isync

	// clear all BATs
	li 0,0
	mtspr 528,0 ; mtspr 530,0 ; mtspr 532,0 ; mtspr 534,0 // IBATU 0..3
	mtspr 536,0 ; mtspr 538,0 ; mtspr 540,0 ; mtspr 542,0 // DBATU 0..3
	mtspr 560,0 ; mtspr 562,0 ; mtspr 564,0 ; mtspr 566,0 // IBATU 4..7
	mtspr 568,0 ; mtspr 570,0 ; mtspr 572,0 ; mtspr 574,0 // DBATU 4..7
	isync

	// clear all SRs
	lis 0,0x8000
	mtsr  0,0 ; mtsr  1,0 ; mtsr  2,0 ; mtsr  3,0
	mtsr  4,0 ; mtsr  5,0 ; mtsr  6,0 ; mtsr  7,0
	mtsr  8,0 ; mtsr  9,0 ; mtsr 10,0 ; mtsr 11,0
	mtsr 12,0 ; mtsr 13,0 ; mtsr 14,0 ; mtsr 15,0
	isync

	// set [DI]BAT0 for 256MB@80000000,
	// real 00000000, WIMG=0000, R/W
	li		3,2
	lis		4,0x8000
	ori		4,4,0x1fff
	mtspr	529,3   // IBAT0L
	mtspr	528,4   // IBAT0U
	mtspr	537,3   // DBAT0L
	mtspr	536,4   // DBAT0U
	isync
	
	// set [DI]BAT4 for 256MB@90000000,
	// real 10000000, WIMG=0000, R/W
	addis	3,3,0x1000
	addis	4,4,0x1000
	mtspr	561,3   // IBAT4L
	mtspr	560,4   // IBAT4U
	mtspr	568,3   // DBAT4L
	mtspr	569,4   // DBAT4U
	isync
	
	// set DBAT1 for 256MB@c0000000,
	// real 00000000, WIMG=0101, R/W
	li		3,0x2a
	lis		4,0xc000
	ori		4,4,0x1fff
	mtspr	539,3   // DBAT1L
	mtspr	538,4   // DBAT1U
	isync
	
	// set DBAT5 for 256MB@d0000000,
	// real 10000000, WIMG=0101, R/W
	addis	3,3,0x1000
	addis	4,4,0x1000
	mtspr	571,3   // DBAT5L
	mtspr	570,4   // DBAT5U
	isync

	mfmsr	3
	ori		3,3,0x30 // MSR_DR|MSR_IR
	mtsrr1	3
	mflr	3
	oris    3,3,0x8000
	mtsrr0	3

	rfi