#![cfg_attr(not(test),  no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_attributes)]

use core::panic::PanicInfo;
use shawna::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info : & PanicInfo) -> ! {
	loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

	print!("Hello{}", " ");

	println!("World{}", "!");

	print!("New line HYPERS");

	loop {}
}
