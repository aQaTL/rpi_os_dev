#![no_std]
#![feature(asm)]
#![feature(global_asm)]

#[rustfmt::skip]
global_asm!(
	".section \".text.boot\"",
	".globl _start",
	"_start:",
		// Set stack before our code
		"ldr     x5, =_start",
		"mov     sp, x5",

		// Load processor id
		"mrs	x0, mpidr_el1",
		"and x0, x0, #0xFFF",

		// Hang if not running on the primary CPU
		"cbnz x0, proc_hang",

		// Clear bss
		"adr	x0, bss_start",
		"adr x1, bss_end",
		"sub x1, x1, x0",
		"bl memzero",

		// Jump to Rust
		"bl main",
		"b proc_hang",

    "proc_hang:",
		"b proc_hang",

    "memzero:",
		"str xzr, [x0], #8",
		"subs x1, x1, #8",
		"b.gt memzero",
		"ret",
	".section \".text.boot\"",
);

mod serial;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	serial_println!("{}", info);
	loop {}
}

unsafe fn get_el() -> u64 {
	let el: u64;
	asm!(
		"mrs x0, CurrentEL",
		"lsr {el}, x0, #2",
		el = out(reg) el
	);
	el
}

#[no_mangle]
extern "C" fn main() -> ! {
	let line = [b'='; 80];
	let line = unsafe { core::str::from_utf8_unchecked(&line) };
	serial_println!("{}", line);
	serial_println!("Kernel w Ruście dla Raspberry Pi.");
	serial_println!();
	serial_println!("Exception level: {}", unsafe { get_el() });

	loop {}
}
