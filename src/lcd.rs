use crate::bindings::Adafruit_GFX_drawLine;
use crate::bindings::Adafruit_TFTLCD_8bit_STM32;
use crate::bindings::Adafruit_TFTLCD_8bit_STM32_fillScreen;

use crate::ctypes::c_void;


// todo use the constructor avoid option?
static mut ADAFRUIT_LCD: Option<Adafruit_TFTLCD_8bit_STM32> = None;


unsafe fn adafruit_lcd() -> &'static mut Adafruit_TFTLCD_8bit_STM32 {
    ADAFRUIT_LCD.as_mut().unwrap()
}

pub fn draw_line() {}

pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: i16 = 240;

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
