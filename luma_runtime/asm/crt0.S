// ======================== //
//		CRT0 Assembly		//
// ======================== //

.text
.section .init

.extern ClearBATS, ClearGPRS, InitHardware, InitSystem

// --------------------------------------------------------------- //

.global _start
_start:
	bl ClearBATS		// Clear all BATs
	bl ClearGPRS 		// Clear all GPRs
	bl InitHardware		// Initialize the hardware
	bl InitSystem		// Initialize the system and disable interrupts.

	// Clear the SBSS section!
	lis		3,__sbss_start@h
	ori		3,3,__sbss_start@l
	li		4,0
	lis		5,__sbss_end@h
	ori		5,5,__sbss_end@l
	sub		5,5,3
	bl		memset

	// Clear the BSS section!
	lis		3,__bss_start@h
	ori		3,3,__bss_start@l
	li		4,0
	lis		5,__bss_end@h
	ori		5,5,__bss_end@l
	sub		5,5,3
	bl		memset
	
	bl main 	// Branch to the user code!
	b .			// If the main function returns, then just loop endlessly.

.global _realmode
_realmode:
	clrlwi	3,3,2
	mtsrr0	3
	mfmsr	3
	rlwinm	3,3,0,28,25
	mtsrr1	3
	rfi