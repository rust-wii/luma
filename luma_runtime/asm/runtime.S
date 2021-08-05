// ======================== //
//	   Runtime Assembly	    //
// ======================== //

.extern _realmode
.extern InitCache,InitPS,InitFPRS

// --------------------------------------------------------------- //

.global InitHardware
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

.global InitGPRS
InitGPRS:
	// Clear all GPRs
	.irp i, 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31
	li \i,0
	.endr

	lis 1,_stack_bot@h ; ori 1,1,_stack_bot@l
	stwu 0,-64(1)

	lis     2,_SDA2_BASE_@h
	ori     2,2,_SDA2_BASE_@l   // Set the Small Data 2 (Read Only) base register.
	lis     13,_SDA_BASE_@h
	ori     13,13,_SDA_BASE_@l  // Set the Small Data (Read\Write) base register.	
	blr

.global ClearBATS
ClearBATS:
	mflr	31
	oris	31,31,0x8000
	lis		3,ConfigBATS@h
	ori		3,3,ConfigBATS@l
	bl		_realmode
	mtlr	31
	blr

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
	ori		3,3,0x30 # MSR_DR|MSR_IR
	mtsrr1	3
	mflr	3
	oris    3,3,0x8000
	mtsrr0	3

	rfi