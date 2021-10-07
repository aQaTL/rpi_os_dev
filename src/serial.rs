use crate::null_lock::SpinLock;
use core::fmt::Write;
use core::{fmt, ptr};

static SERIAL_OUT: SerialOut = SerialOut::new();

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	SERIAL_OUT.0.lock().write_fmt(args).unwrap();
}

struct SerialOut(SpinLock<SerialOutInner>);

struct SerialOutInner {
	bytes_written: usize,
}

impl SerialOut {
	const fn new() -> Self {
		SerialOut(SpinLock::new(SerialOutInner::new()))
	}
}

impl SerialOutInner {
	const fn new() -> Self {
		SerialOutInner { bytes_written: 0 }
	}

	fn write_byte(&mut self, byte: u8) {
		unsafe {
			ptr::write_volatile(UART_DR as *mut u8, byte);
		}
		self.bytes_written += 1;
	}
}

impl Write for SerialOutInner {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for byte in s.bytes() {
			if byte == b'\n' {
				self.write_byte(b'\r');
			}
			self.write_byte(byte);
		}
		Ok(())
	}
}

const UART_DR: u32 = 0x3F201000;
const UART_FR: u32 = 0x3F201018;

fn mmio_write(reg: u32, val: u32) {
	unsafe {
		ptr::write_volatile(reg as *mut u32, val);
	}
}

fn mmio_read(reg: u32) -> u32 {
	unsafe { ptr::read_volatile(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
	mmio_read(UART_FR) & (1 << 5) > 0
}

fn _receive_fifo_empty() -> bool {
	mmio_read(UART_FR) & (1 << 4) > 0
}

fn write_byte(c: u8) {
	while transmit_fifo_full() {}
	mmio_write(UART_DR, c as u32);
}

fn _getc() -> u8 {
	while _receive_fifo_empty() {}
	mmio_read(UART_DR) as u8
}
