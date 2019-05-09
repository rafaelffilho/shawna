#![cfg_attr(not(test), no_std) ]
#![cfg_attr(not(test), no_main)]
#![allow(dead_code)]
#![allow(unused_imports)]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info : & PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

	let vga_p = 0xB8000 as *mut u8;

	let a: &[u8] = b"I am working btw";

	for (i, &b) in a.iter().enumerate() {
		unsafe { 
			*vga_p.offset(i as isize * 2) = b;
			*vga_p.offset(i as isize * 2 + 1) = 0xB;
		}
	}


	loop {}
}
