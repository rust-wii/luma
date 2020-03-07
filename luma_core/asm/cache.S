/*
 *  Cache Assembly
*/

.set r0,0;
.set r1,1;
.set r2,2;
.set r3,3;
.set r4,4;
.set r5,5;

// --------------------------

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
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	dcbi r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	blr

.global DCFlushRange
DCFlushRange:
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	dcbf r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	sc
	blr

.global DCStoreRange
DCStoreRange:
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	dcbst r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	sc
	blr

.global DCFlushRangeNS
DCFlushRangeNS:
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	dcbf r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	blr

.global DCStoreRangeNS
DCStoreRangeNS:
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	dcbst r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	blr

// --------------------------

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
	cmplwi r4, 0   # zero or negative size?
	blelr
	clrlwi. r5, r3, 27  # check for lower bits set in address
	beq 0f
	addi r4, r4, 0x20 
0:
	addi r4, r4, 0x1f
	srwi r4, r4, 5
	mtctr r4
1:
	icbi r0, r3
	addi r3, r3, 0x20
	bdnz 1b
	sync
	isync
	blr

.global ICFlashInvalidate
ICFlashInvalidate:
	mfspr	r3,HID0
	ori		r3,r3,0x0800
	mtspr	HID0,r3
	isync
	blr

.global ICBlockInvalidate
ICBlockInvalidate:
	icbi	r0,r3
	blr

// --------------------------