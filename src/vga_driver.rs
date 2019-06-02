use core::fmt;
use volatile::Volatile;
use spin::Mutex;
use lazy_static::lazy_static;

const HEIGHT: usize = 25;
const WIDTH:  usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	LightGray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xa,
	LightCyan = 0xb,
	LightRed = 0xc,
	Pink = 0xd,
	Yellow = 0xe,
	White = 0xf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode (u8);

impl ColorCode {
	fn new (fg: Color, bg: Color) -> ColorCode {
		ColorCode((bg as u8) << 4 | fg as u8)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_code: u8,
	color_code: ColorCode
}

#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; WIDTH]; HEIGHT]
}

pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer
}

impl Writer {
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

				self.buffer.chars[row][col].write(ScreenChar {
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
		let blank = ScreenChar {
			ascii_code: b' ',
			color_code: self.color_code
		};

		for col in 0..WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer {
	fn write_str (&mut self, string: &str) -> fmt::Result {
		self.write_string(string);
		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::White, Color::Black),
		buffer: unsafe { &mut *(0xB8000 as *mut Buffer) }
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
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	});
}