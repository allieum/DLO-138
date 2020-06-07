use core::panic::PanicInfo;
use core::fmt::Write;


use crate::ctypes::c_char;

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
    pub fn new() -> HackStr {
	HackStr {
	    length: 0,
	    buf: [0; 100]
	}
    }

    pub fn as_cstr(&self) -> *const c_char {
	self.buf.as_ptr()
    }
}

impl Write for HackStr {
    // todo bound check
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	for c in s.chars() {
	    self.buf[self.length] = c as c_char;
	    self.length += 1;
	}
	Ok(())
    }
}
