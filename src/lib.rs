#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]

pub mod vga_driver;
pub mod interrupts;
pub mod gdt;

pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe {
		interrupts::PICS.lock().initialize()
	};
	x86_64::instructions::interrupts::enable();
}