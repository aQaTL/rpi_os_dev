#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]

use core::{slice, str};

mod asm;
mod serial;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	serial_println!("{}", info);
	loop {}
}

#[no_mangle]
extern "C" fn main(msg: *const u8, msg_len: usize) -> ! {
	let line = [b'='; 80];
	let line = unsafe { str::from_utf8_unchecked(&line) };
	serial_println!("{}", line);
	serial_println!("asm_msg len: {}", msg_len);
	let msg = unsafe { str::from_utf8_unchecked(slice::from_raw_parts(msg, msg_len)) };
	serial_println!("{}", msg);
	serial_println!();
	serial_println!("Exception level: {}", exception_level());
	// switch_to_exception_level_1();
	serial_println!("Exception level: {}", exception_level());

	let value = test_asm();
	serial_println!("Value: {}", value);

	loop {}
}

const FOO: u64 = 818939904;

fn test_asm() -> u64 {
	unsafe {
		let value;
		asm!(
			"mov x0, {FOO}",
			"mov {value}, x0",
			FOO = in(reg) FOO,
			value = out(reg) value,
		);
		value
	}
}

fn exception_level() -> u64 {
	unsafe {
		let el: u64;
		asm!(
			"mrs x0, CurrentEL",
			"lsr {el}, x0, #2",
			el = out(reg) el
		);
		el
	}
}
