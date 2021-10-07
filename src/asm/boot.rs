global_asm!(
	r###"
.section ".text.boot"

.globl	_start

_start:
	// Load processor id
	mrs	x0, mpidr_el1
	and	x0, x0, #0xFFF

	// Hang if not running on the primary CPU
	cbnz x0, proc_hang

	// Clear bss
	adr x0, bss_start
	adr x1, bss_end
	sub x1, x1, x0
	bl memzero

	// Stack pointer
	ldr	x0, =_start
	mov	sp, x0

	// `main` function arguments

	// first argument is a pointer the the first char
	ldr x0, HELLO_MSG
	// second argument is the message length
	ldr x1, =HELLO_MSG
	ldr x1, [x1, #8]

	// Jump to Rust code.
	b	main

proc_hang:
    wfe
	b	proc_hang

memzero:
	str xzr, [x0], #8
	subs x1, x1, #8
	b.gt memzero
	ret

"###
);

#[no_mangle]
static HELLO_MSG: &str = "Kernel w Ruście dla Raspberry Pi.";
