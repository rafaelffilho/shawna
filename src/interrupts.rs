use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use crate::hlt_loop;
use spin;

use crate::{println, print};
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET,
	Keyboard
}

impl InterruptIndex {
	fn as_u8(self) -> u8 {
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}

pub static PICS: spin::Mutex<ChainedPics> =
		spin::Mutex::new(
			unsafe{
				ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
			}
		);

static mut TIMER: u64 = 0;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.page_fault.set_handler_fn(page_fault_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler).
				set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
		idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
		idt
	};
}

pub fn init_idt () {
	IDT.load();
}

extern "x86-interrupt" fn keyboard_interrupt_handler (
	_stack_frame: &mut InterruptStackFrame
) {
	use spin::Mutex;
	use x86_64::instructions::port::Port;
	use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
			Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
	}

	let mut keyboard = KEYBOARD.lock();
	let port = Port::new(0x60);
	let scancode: u8 = unsafe { port.read() };
	
	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::Unicode(character) => print!("{}", character),
				_ => ()
			}
		}
	}

	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}

extern "x86-interrupt" fn timer_handler (
	_stack_frame: &mut InterruptStackFrame
) {
	unsafe {
		TIMER += 1;
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

extern "x86-interrupt" fn page_fault_handler (
	stack_frame: &mut InterruptStackFrame,
	_error_code: PageFaultErrorCode
) {
	use x86_64::registers::control::Cr2;

	println!("EXECPTION: PAGE FAULT:");
	println!("Accessed Address: {:?}", Cr2::read());
	println!("{:#?}", stack_frame);
	hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler (
	stack_frame: &mut InterruptStackFrame,
	_error_code: u64
) {
	panic!("EXCEPTION: DOUBLE FAULT:\n{:#?}", stack_frame);
	hlt_loop();
}

extern "x86-interrupt" fn breakpoint_handler (
	stack_frame: &mut InterruptStackFrame
) {
	println!("EXCEPTION: BREAKPOINT:\n{:#?}", stack_frame);
}