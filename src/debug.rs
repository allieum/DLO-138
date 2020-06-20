use core::fmt;
use core::panic::PanicInfo;
use crate::ctypes::c_char;

/// Create and return a FixedCStr using a format string
#[macro_export]
macro_rules! c_str {
    ($($args:tt),*) => {{
	use core::fmt::Write;

	let mut cstr = $crate::debug::FixedCStr::new();
	write!(&mut cstr, $($args),*).expect("write! failed in c_str! macro");

	cstr
    }}
}

/// Send a format string over serial
#[macro_export]
macro_rules! serial {
    ($($args:tt),*) => {{
	let cstr = $crate::c_str!($($args),*);
	$crate::debug::PRINT_SERIAL.unwrap()(cstr.as_ptr());
    }}
}

// Pointer to a wrapper around Arduino's Serial.print()
pub static mut PRINT_SERIAL: Option<fn(*const c_char)> = None;

pub unsafe fn init(print_serial: fn(*const c_char)) {
    PRINT_SERIAL = Some(print_serial);
}

#[panic_handler]
/// Write debug message to screen and over serial.
unsafe fn stm32_panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let (file, line, column) = (location.file(), location.line(), location.column());

    let msg = c_str!("panic @ {} {}:{}", file, line, column);
    crate::draw::draw_message(&msg);

    serial!("panic @ {} {}:{}", file, line, column);

    loop {};
}

/// Fixed-size c string with a stack-based buffer
pub struct FixedCStr {
    length: usize,
    buf: [c_char; FixedCStr::SIZE]
}

impl FixedCStr {
    const SIZE: usize = 100;

    pub const fn new() -> FixedCStr {
	FixedCStr {
	    length: 0,
	    buf: [0; FixedCStr::SIZE]
	}
    }

    // needed? if so, should zero out buf
    pub fn _clear(&mut self) {
	self.length = 0;
    }

    pub fn as_ptr(&self) -> *const c_char {
	self.buf.as_ptr()
    }
}

impl fmt::Write for FixedCStr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
	self.length += s.chars().count();

	// Allow one character fewer to allow for a terminating null
	if self.length > FixedCStr::SIZE - 1 {
	    panic!("Writing {} to {} would overflow maximum size of {}",
		   s, self, FixedCStr::SIZE);
	}

	for c in s.chars() {
	    self.buf[self.length] = c as c_char;
	}
	Ok(())
    }
}

impl fmt::Display for FixedCStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "FixedCStr(length: {}, pointer: {:?})", self.length, self.as_ptr())
    }
}
