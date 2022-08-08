// ======================== //
//		CRT0 Assembly		//
// ======================== //

.text
.section .init
.global _start

.extern ClearBATS, ClearGPRS, InitHardware, InitSystem

// --------------------------------------------------------------- //

__argv:
	.long 0		// argv magic
	.long 0		// command line
	.long 0		// command line length
	.long 0		// argc
	.long 0		// argv
	.long 0		// end address of argv

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

	.section .bss
	.balign 8
	.space 0x4000

	.globl __system_argv
	.section	.sdata,"aw",@progbits
	.align 2
	.type	__system_argv, @object
	.size	__system_argv, 4
__system_argv:
	.long	__argv