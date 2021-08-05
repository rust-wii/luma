// ======================== //
//	   System Assembly	    //
// ======================== //

.extern ICEnable, DCEnable, L2Init, L2Enable, ICFlashInvalidate

// --------------------------------------------------------------- //

.globl InitSystem
InitSystem:
	mflr    0
	stw     0, 4(1)
	stwu    1, -24(1)
	stmw	29, 12(1)

	// Disable interrupts!
	mfmsr   3
	rlwinm  4,3,0,17,15
	rlwinm  4,4,0,26,24
	mtmsr   4

	// Clear various SPR's
	li       3,0
	mtspr    952, 3
	mtspr    956, 3
	mtspr    953, 3
	mtspr    954, 3
	mtspr    957, 3
	mtspr    958, 3
    isync
	
	mfspr	3,1011      // (HID4)
	oris	3,3,0x0190	// set additional bits in HID4: S0(store 0), LPE(PS LE exception), L2CFI(L2 castout prior to L2 inv. flash)
	mtspr	1011,3
	isync

	// Disable Speculative Bus Accesses to non-guarded 1ace from both caches.
	mfspr    3, 1008    // (HID0)
	ori      3, 3, 0x0200
	mtspr    1008, 3
	isync
	
	// Set the Non-IEEE mode in the FPSCR
	mtfsb1  29
	
	// Disable Write Gather Pipe
	mfspr   3,920    // (HID2)
	rlwinm 	3,3,0,2,0
	mtspr   920,3    // (HID2)
	isync
	
	// Restore the non-volatile registers to their previous values and return.
	lwz     0, 28(1)
	lmw	    29,12(1)
	addi    1,1,24
	mtlr    0
	blr

.globl InitPS
InitPS:
	mflr	0
	stw		0,4(1)
	stwu	1,-8(1)

	mfspr   3,920 // (HID2)
	oris    3,3,0xA000
	mtspr   920,3 // (HID2)
	isync

	// Set the Instruction Cache invalidation bit
	bl		ICFlashInvalidate
	sync

	// Clear various Special Purpose Registers
	li       3,0
	mtspr    912,3
	mtspr    913,3
	mtspr    914,3
	mtspr    915,3
	mtspr    916,3
	mtspr    917,3
	mtspr    918,3
	mtspr    919,3
	isync

	lwz		0,12(1)
	addi	1,1,8
	mtlr	0

	// Return
	blr

.globl InitFPRS
InitFPRS:
	// Enable the Floating Point Registers
	mfmsr    3
	ori      3,3,0x2000
	mtmsr    3
	
	// Clear all of the FPR's to 0
	lis	    3,zeroF@ha
	lfd	    0,zeroF@l(3)
	fmr      1,0
	fmr      2,0
	fmr      3,0
	fmr      4,0
	fmr      5,0
	fmr      6,0
	fmr      7,0
	fmr      8,0
	fmr      9,0
	fmr      10,0
	fmr      11,0
	fmr      12,0
	fmr      13,0
	fmr      14,0
	fmr      15,0
	fmr      16,0
	fmr      17,0
	fmr      18,0
	fmr      19,0
	fmr      20,0
	fmr      21,0
	fmr      22,0
	fmr      23,0
	fmr      24,0
	fmr      25,0
	fmr      26,0
	fmr      27,0
	fmr      28,0
	fmr      29,0
	fmr      30,0
	fmr      31,0
	mtfsf    255,0

	// Return
	blr

.globl InitCache
InitCache:
	mflr    0
	stw     0,4(1)
	stwu    1,-16(1)
	stw     31,12(1)

    // Check if the Instruction Cache has been enabled or not.
	mfspr   3,1008
	rlwinm. 0,3,0,16,16
	bne     ICEnabled
	bl		ICEnable

ICEnabled:
    // Check if the Data Cache has been enabled or not.
	mfspr   3,1008
	rlwinm. 0,3,0,17,17
    bne     DCEnabled
	bl      DCEnable

DCEnabled:
    // Check if the Locked Cache has been enabled or not.
	mfspr   3,1017
	clrrwi. 0,3,31  
	bne     L2Enabled

	bl		L2Init
	bl		L2Enable

L2Enabled:
	// Restore the non-volatile registers to their previous values and return.
	lwz     0,20(1)
	lwz     31,12(1)
	addi    1,1,16
	mtlr    0
	blr

zeroF:
	.double 0.0
zeroPS:
	.float 0.0,0.0