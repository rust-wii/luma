// ======================== //
//		CRT0 Assembly		//
// ======================== //

.text
.section .init
.global _start

.extern ClearBATS, ClearGPRS, InitHardware, InitSystem

// --------------------------------------------------------------- //

__argv:
	.long	0		# argv magic
	.long 0		# command line
	.long 0		# command line length
	.long	0		# argc
	.long	0		#	argv
	.long 0		# end address of argv

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
	bl		_memset

	// Clear the BSS section!
	lis		3,__bss_start@h
	ori		3,3,__bss_start@l
	li		4,0
	lis		5,__bss_end@h
	ori		5,5,__bss_end@l
	sub		5,5,3
	bl		_memset
	
	bl main 	// Branch to the user code!
	b .			// If the main function returns, then just loop endlessly.

_memset:
	clrlwi.	6,5,29
	srwi	5,5,2
	subi	3,3,4
	mtctr	5
1:	stwu	4,4(3)
	bdnz	1b
	cmplwi	6,0
	beq		3f
2:	stbu	4,1(3)
	addic.	6,6,-1
	bne+	2b
3:	blr

	.section .bss
	.balign 8
__crt0stack_end:
	.space 0x4000
__crt0stack:

	.globl __system_argv
	.section	.sdata,"aw",@progbits
	.align 2
	.type	__system_argv, @object
	.size	__system_argv, 4
__system_argv:
	.long	__argv