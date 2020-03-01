/*
 *  Cache Assembly
*/

.set r3,3;

.global DCEnable
DCEnable:
	sync                    # Prevents enabling in the middle of data access
	mfspr 	r3,HID0        # Move from register
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