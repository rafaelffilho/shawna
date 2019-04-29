#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info : & PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

	let vga_p = 0xB8000 as *mut u8;

	let a: &[u8] = b "I am working btw";

	for (i, &b) in a.iter().enumerate() {
		unsafe { 
			* vga_p.offset(i as isize * 2) = b;
			* vga_p.offset(i as isize * 2 + 1) = 0xB;
		}
	}


	loop {}
}
