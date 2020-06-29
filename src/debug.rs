use cortex_m_semihosting::dbg;

// use core::fmt;
use core::panic::PanicInfo;
// use crate::ctypes::c_char;

use stm32f1xx_hal::serial::Tx;
use stm32f1::stm32f103::USART1;

///// Create and return a FixedCStr using a format string
//#[macro_export]
// macro_rules! c_str {
//     ($($args:tt),*) => {{
// 	use core::fmt::Write;

// 	let mut cstr = $crate::debug::FixedCStr::new();
// 	write!(&mut cstr, $($args),*).expect("write! failed in c_str! macro");

// 	cstr
//     }}
// }

/// Send a format string over serial, preferring UART if it's
/// available (fast). Otherwise fall back on semihosting (slow)
#[macro_export]
macro_rules! serial {
    ($($args:tt)*) => {{
	use core::fmt::Write;
	use cortex_m_semihosting::hprintln;

	#[allow(unsafe_code, unused_unsafe)]
	unsafe {
	    match $crate::debug::TX_UART.as_mut() {
		Some(tx) => {
		    tx.write_fmt(format_args!($($args)*)).unwrap();
		    writeln!(tx).unwrap();
		},
		None => hprintln!($($args)*).unwrap()
	    };
	}

    }}
}
pub static mut TX_UART: Option<Tx<USART1>> = None;

// Pointer to a wrapper around Arduino's Serial.print()
// pub unsafe fn init(print_serial: fn(*const c_char)) {
    // PRINT_SERIAL = Some(print_serial);
// }

pub fn init(tx_uart: Tx<USART1>) {
    unsafe { TX_UART = Some(tx_uart) };
}

/// Write debug message to screen and over serial.
#[panic_handler]
unsafe fn stm32_panic(info: &PanicInfo) -> ! {
    // let location = info.location().unwrap();
    // let (file, line, column) = (location.file(), location.line(), location.column());

    dbg!("{}", info);

    loop {};
}

// /// Fixed-size C string with a stack-based buffer
// pub struct FixedCStr {
//     length: usize,
//     buf: [c_char; FixedCStr::SIZE]
// }

// impl FixedCStr {
//     const SIZE: usize = 100;

//     pub const fn new() -> FixedCStr {
// 	FixedCStr {
// 	    length: 0,
// 	    buf: [0; FixedCStr::SIZE]
// 	}
//     }

//     // needed? if so, should zero out buf
//     pub fn _clear(&mut self) {
// 	self.length = 0;
//     }

//     pub fn as_ptr(&self) -> *const c_char {
// 	self.buf.as_ptr()
//     }
// }

// impl fmt::Write for FixedCStr {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
// 	// Allow one character fewer to allow for a terminating null
// 	if self.length > FixedCStr::SIZE - 1 {
// 	    // todo return fmt::Error instead of panic here
// 	    panic!("Writing {} to {} would overflow maximum size of {}",
// 		   s, self, FixedCStr::SIZE);
// 	}

// 	for (i, c) in s.chars().enumerate() {
// 	    self.buf[self.length + i] = c as c_char;
// 	}
// 	self.length += s.chars().count();

// 	Ok(())
//     }
// }

// impl fmt::Display for FixedCStr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 	write!(f, "FixedCStr(length: {}, pointer: {:?})", self.length, self.as_ptr())
//     }
// }
