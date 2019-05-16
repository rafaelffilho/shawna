#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]

pub mod vga_driver;
pub mod interrupts;

pub fn init() {
	interrupts::init_idt();
}