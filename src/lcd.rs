use crate::bindings::Adafruit_GFX_drawLine;
use crate::bindings::Adafruit_TFTLCD_8bit_STM32;
use crate::bindings::Adafruit_TFTLCD_8bit_STM32_fillScreen;

// No libc support on our compile target, so define types to make rustbindgen happy
mod ctypes {
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub use core::ffi::c_void;
}

use ctypes::*;


// todo use the constructor avoid option?
static mut ADAFRUIT_LCD: Option<Adafruit_TFTLCD_8bit_STM32> = None;


unsafe fn adafruit_lcd() -> &'static mut Adafruit_TFTLCD_8bit_STM32 {
    ADAFRUIT_LCD.as_mut().unwrap()
}

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: i16 = 240;

impl Adafruit_TFTLCD_8bit_STM32 {
    pub unsafe fn print(&mut self, msg: &str) {
	self._base._base.print2(msg.as_ptr() as *const i8);
    }

    // meh
    pub unsafe fn printc(&mut self, msg: *const i8) {
	self._base._base.print2(msg);
    }

    pub unsafe fn set_cursor(&mut self, x: i16, y: i16) {
	self._base.cursor_x = x;
	self._base.cursor_y = y;
    }

    pub unsafe fn set_text_color(&mut self, color: u16) {
	self._base.textcolor = color;
	self._base.textbgcolor = color;
    }

    pub unsafe fn set_text_size(&mut self, s: u8) {
	self._base.setTextSize(s);
    }

    // todo look into typedefs and also from/into for pointer conversion
    pub unsafe fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u16) {
	let this = self as *mut Adafruit_TFTLCD_8bit_STM32 as *mut c_void;
	Adafruit_GFX_drawLine(this, x0, y0, x1, y1, color);
    }

    pub unsafe fn fill_screen(&mut self, color: u16) {
	let this = self as *mut Adafruit_TFTLCD_8bit_STM32 as *mut c_void;
	Adafruit_TFTLCD_8bit_STM32_fillScreen(this, color);
    }
}
