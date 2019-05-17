#![cfg_attr(not(test),  no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused_imports)]
#![allow(unused_attributes)]

use core::panic::PanicInfo;
use shawna::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
	println!("{}", _info);
	loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

	shawna::init();

	print!("Hello{}", " ");

	println!("World{}", "!");

	loop {}
}
