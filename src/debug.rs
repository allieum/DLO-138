use core::fmt::Write;
use core::panic::PanicInfo;
use crate::ctypes::c_char;

macro_rules! serial {
    ($($args:tt),*) => {{
	use core::fmt::Write;

	let mut s = crate::debug::HackStr::new();
	write!(&mut s, $($args),*).expect("write! failed in serial! macro");
	s.write_str("\n").expect("write_str failed in serial! macro");

	crate::debug::PRINT_SERIAL.unwrap()(s.as_cstr());
    }}
}

pub static mut PRINT_SERIAL: Option<fn(*const c_char)> = None;

pub unsafe fn init(print_serial: fn(*const c_char)) {
    PRINT_SERIAL = Some(print_serial);
}

// pub unsafe fn print_serial<T: Display>(msg: T) {
//     let mut s = HackStr::new();
//     write!(&mut s, "{}\n", msg).expect("nothing");
//     PRINT_SERIAL.unwrap()(s.as_cstr());
// }

#[panic_handler]
unsafe fn candy_panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let (file, line, column) = (location.file(), location.line(), location.column());

    let mut s = HackStr::new();

    // todo abstract hackstr + write into another macro to learn how they work?
    write!(&mut s, "panic @ {} {}:{}", file, line, column).expect("didn't work");

    crate::draw::draw_message(&s);

    loop {};
}

pub struct HackStr {
    length: usize,
    buf: [c_char; 100]
}

impl HackStr {
    pub const fn new() -> HackStr {
	HackStr {
	    length: 0,
	    buf: [0; 100]
	}
    }

    // needed?
    pub fn _clear(&mut self) {
	self.length = 0;
    }

    pub fn as_cstr(&self) -> *const c_char {
	self.buf.as_ptr()
    }
}

impl core::fmt::Write for HackStr {
    // todo bound check
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	for c in s.chars() {
	    self.buf[self.length] = c as c_char;
	    self.length += 1;
	}
	Ok(())
    }
}
