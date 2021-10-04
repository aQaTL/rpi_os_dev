.section ".text.boot"

.globl _start

_start:
    // Set stack before our code
    ldr     x5, =_start
    mov     sp, x5

	// Load processor id
	mrs	x0, mpidr_el1
	and x0, x0, #0xFFF
	// Hang if not running on the primary CPU
	cbnz x0, proc_hang

	// Clear bss
	adr	x0, bss_start
	adr x1, bss_end
	sub x1, x1, x0
	bl memzero

	// Jump to Rust
	bl main
	b proc_hang

proc_hang:
	b proc_hang

memzero:
	str xzr, [x0], #8
	subs x1, x1, #8
	b.gt memzero
	ret

.section ".text.boot"

