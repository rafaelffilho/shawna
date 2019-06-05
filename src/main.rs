#![cfg_attr(not(test),  no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused_imports)]
#![allow(unused_attributes)]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use shawna::*;

#[cfg(not(test))]
entry_point!(kmain);

#[cfg(not(test))]
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
	println!("{}", _info);
	shawna::hlt_loop();
}

#[cfg(not(test))]
#[no_mangle]
fn kmain(_args: &'static BootInfo) -> ! {

	shawna::init();

	println!("Hello World!");

	shawna::hlt_loop();
}
