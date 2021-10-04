#![no_std]

mod serial;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	serial_println!("{}", info);
	loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
	let line = [b'='; 80];
	let line = unsafe { core::str::from_utf8_unchecked(&line) };
	serial_println!("{}", line);
	serial_println!("Kernel w Ruście dla Raspberry Pi.");
	serial_println!();
	serial_println!("Ala ma kota");
	try_sth();

	loop {}
}

fn try_sth() {
	panic!("Oh no");
}
