/*
 *  Cache Assembly
*/

.set r0,0;		.set r1,1;		.set r2,2;		.set r3,3;		.set r4,4;
.set r5,5;		.set r6,6;		.set r7,7;		.set r8,8;		.set r9,9;
.set r10,10;	.set r11,11;	.set r12,12;	.set r13,13;	.set r14,14;
.set r15,15;	.set r16,16;	.set r17,17;	.set r18,18;	.set r19,19;
.set r20,20;	.set r21,21;	.set r22,22;	.set r23,23;	.set r24,24;
.set r25,25;	.set r26,26;	.set r27,27;	.set r28,28;	.set r29,29;
.set r30,30;	.set r31,31;

.set L2CR,1017;	.set HID0,1008;

// --------------------------------------------------------------- //

.global DCEnable
DCEnable:
	sync                    # Prevents enabling in the middle of data access
	mfspr 	r3,HID0         # Move from register
	ori     r3,r3,0x4000    # HID0[DCE]
	mtspr   HID0,r3         # Move to register
	blr                     # Return

.global DCDisable
DCDisable:
    sync                    # Prevents disabling in the middle of data access
    mfspr	r3,HID0         # Move from register
	rlwinm	r3,r3,0,18,16   # Rotate left word then AND with mask
	mtspr	HID0,r3         # Move to register
	blr                     # Return

.global DCFlashInvalidate
DCFlashInvalidate:
    sync                    # Prevents invalidation in the middle of data access
	mfspr	r3,HID0         # Move from register
	ori		r3,r3,0x0400    # HID0[DCFI]
	mtspr	HID0,r3         # Move to register
	blr                     # Return

.global DCLock
DCLock:
	sync					# Prevents locking in the middle of data access
	mfspr	r3,HID0			# Move from register
	ori		r3,r3,0x1000	# HID0[DLOCK]
	mtspr	HID0,r3			# Move to register
	blr						# Return

.global DCUnlock
DCUnlock:
	sync					# Prevents unlocking in the middle of data access
	mfspr	r3,HID0			# Move from register
	rlwinm	r3,r3,0,20,18	# Rotate left word then AND with mask
	mtspr	HID0,r3			# Move to register
	blr						# Return

.global DCInvalidateRange
DCInvalidateRange:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	dcbi r0, r3			# Data cache block invalidate
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	blr					# Return

.global DCFlushRange
DCFlushRange:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	dcbf r0, r3			# Data cache block flush
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	sc					# Contextual Sync
	blr					# Return

.global DCStoreRange
DCStoreRange:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	dcbst r0, r3		# Data cache block store
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	sc					# Contextual Sync
	blr					# Return

.global DCFlushRangeNS
DCFlushRangeNS:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	dcbf r0, r3			# Data cache block flush
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	blr					# Return

.global DCStoreRangeNS
DCStoreRangeNS:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	dcbst r0, r3		# Data cache block store
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	blr					# Return

// --------------------------------------------------------------- //

.global ICEnable
ICEnable:
	mfspr	r3,HID0			# Move from register
	ori		r3,r3,0x8000	# HIDO[ICE]
	mtspr	HID0,r3			# Move to register
	isync					# Prevents enabling in the middle of an instruction fetch
	blr						# Return

.global ICDisable
ICDisable:
	mfspr	r3,HID0			# Move from register
	rlwinm	r3,r3,0,17,15	# Rotate left word then AND with mask
	mtspr	HID0,r3			# Move to register
	isync					# Prevents disabling in the middle of an instruction fetch
	blr						# Return

.global ICLock
ICLock:
	mfspr	r3,HID0			# Move from register
	ori		r3,r3,0x2000	# HID0[ILOCK]
	mtspr	HID0,r3			# Move to register
	isync					# Prevents locking in the middle of an instruction fetch
	blr						# Return

.global ICUnlock
ICUnlock:
	mfspr	r3,HID0			# Move from register
	rlwinm	r3,r3,0,19,17	# Rotate left word then AND with mask
	mtspr	HID0,r3			# Move to register
	isync					# Prevents unlocking in the middle of an instruction fetch
	blr						# Return

.global ICInvalidateRange
ICInvalidateRange:
	cmplwi r4, 0   		# Check for zero or negative size
	blelr				# Branch if less than or equal
	clrlwi. r5, r3, 27  # Check for lower bits set in address
	beq 0f				# Branch if equal or zero
	addi r4, r4, 0x20 	# Add immediate
0:
	addi r4, r4, 0x1f	# Add immediate
	srwi r4, r4, 5		# Shift right word immediate
	mtctr r4			# Move to register
1:
	icbi r0, r3			# Instruction cache block invalidate
	addi r3, r3, 0x20	# Add immediate
	bdnz 1b				# Branch if decrem CTR doesnt equal zero
	sync				# Sync
	isync				# Instruction sync
	blr					# Return

.global ICFlashInvalidate
ICFlashInvalidate:
	mfspr	r3,HID0			# Move from register
	ori		r3,r3,0x0800	# HID0[ICFI]
	mtspr	HID0,r3			# Move to register
	isync					# Instruction sync
	blr						# Return

.global ICBlockInvalidate
ICBlockInvalidate:
	icbi	r0,r3	# Instruction cache block invalidate
	blr				# Return

// --------------------------------------------------------------- //

.global L2Init
L2Init:
	mflr	r0				# Move from register
	stw		0,4(1)			# Store word
	stwu	1,-16(1)		# Store word with update
	stw		31,12(1)		# Store word
	mfmsr	r3				# Move from machine state register
	mr		r31,r3			# Move register
	sync					# Sync
	li		r3,48			# Load immediate
	mtmsr	r3				# Move to machine state register
	sync					# Sync
	bl		L2Disable		# Branch to L2Disable
	bl		L2Invalidate	# Branch to L2Invalidate
	mr		r3,r31			# Move register
	mtmsr	r3				# Move to machine state register
	lwz		0,20(1)			# Load word
	lwz		31,12(1)		# Load word
	mtlr	r0				# Move to register
	blr						# Return

.global L2Enable
L2Enable:
	sync					# Sync
	mfspr	r3,L2CR			# Move from register
	oris	r0,r3,0x8000	# L2CR[L2E]
	rlwinm	r3,r0,0,11,9	# L2CR[L2I]
	mtspr	L2CR,r3			# Move to register
	sync					# Sync
	blr						# Return

.global L2Disable
L2Disable:
	sync				# Sync
	mfspr	r3,L2CR		# Move from register
	clrlwi	r3,r3,1		# L2CR[L2E]
	mtspr	L2CR,r3		# Move to register
	sync				# Sync
	blr					# Return

.global L2Invalidate
L2Invalidate:
	mflr    r0				# Move from register
	stw     0,4(1)			# Store word
	stwu    1,-8(1)			# Store word with update
	bl      L2Disable		# Branch to L2Disable
	mfspr	r3,L2CR			# Move from register
	oris    r3,r3,0x0020	# L2CR[L2I]
	mtspr	L2CR,r3			# Move to register
1:
	mfspr	r3,L2CR			# Move from register
	clrlwi  r0,r3,31		# L2CR[L2IP]
	cmplwi  r0,0			# Compare logical immediate
	bne     1b				# Branch on not equal
	mfspr	r3,L2CR			# Move from register
	rlwinm  r3,r3,0,11,9	# L2CR[L2I]
	mtspr	L2CR,r3			# Move to register
2:
	mfspr	r3,L2CR			# Move from register
	clrlwi  r0,r3,31		# L2CR[L2IP]
	cmplwi  r0,0			# Compare logical immediate
	bne     2b				# Branch on not equal
	lwz     0,12(1)			# Load word and zero
	addi    r1,r1,8			# Add immediate
	mtlr    r0				# Move to register
	blr						# Return