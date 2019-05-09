#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::fmt;
use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

const HEIGHT: usize = 25;
const WIDTH:  usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum color {
	black = 0x0,
	blue = 0x1,
	green = 0x2,
	cyan = 0x3,
	red = 0x4,
	magenta = 0x5,
	brown = 0x6,
	light_gray = 0x7,
	dark_gray = 0x8,
	light_blue = 0x9,
	light_green = 0xa,
	light_cyan = 0xb,
	light_red = 0xc,
	pink = 0xd,
	yellow = 0xe,
	white = 0xf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct color_code (u8);

impl color_code {
	fn new (fg: color, bg: color) -> color_code {
		color_code((bg as u8) << 4 | fg as u8)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct screen_char {
	ascii_code: u8,
	color_code: color_code
}

#[repr(transparent)]
struct buffer {
	chars: [[Volatile<screen_char>; WIDTH]; HEIGHT]
}

pub struct writer {
	column_position: usize,
	color_code: color_code,
	buffer: &'static mut buffer
}

impl writer {
	pub fn write_byte (&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= WIDTH {
					self.new_line();
				}

				let row = HEIGHT - 1;
				let col = self.column_position;
				let color = self.color_code;

				self.buffer.chars[row][col].write(screen_char {
					ascii_code: byte,
					color_code: color
				});
				self.column_position += 1;
			}
		}
	}

	pub fn write_string (&mut self, string: &str){
		for byte in string.bytes() {
			match byte {
				0x20...0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0x7E)
			}
		}
	}

	fn new_line (&mut self) {
		for row in 1..HEIGHT{
			for col in 0..WIDTH {
				let c = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(c);
			}
		}
		self.clear_row(HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row (&mut self, row: usize) {
		let blank = screen_char {
			ascii_code: b' ',
			color_code: self.color_code
		};

		for col in 0..WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for writer {
	fn write_str (&mut self, string: &str) -> fmt::Result {
		self.write_string(string);
		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<writer> = Mutex::new(writer {
		column_position: 0,
		color_code: color_code::new(color::white, color::black),
		buffer: unsafe { &mut *(0xB8000 as *mut buffer) }
	});
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_driver::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments){
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}