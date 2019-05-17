use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;

use crate::{print, println};
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET
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

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault).
				set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer);
		idt
	};
}

pub fn init_idt () {
	IDT.load();
}

extern "x86-interrupt" fn timer (
	_stack_frame: &mut InterruptStackFrame
) {
	print!(".");

	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

extern "x86-interrupt" fn double_fault (
	stack_frame: &mut InterruptStackFrame,
	_error_code: u64
) {
	panic!("EXCEPTION: DOUBLE FAULT:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(
	stack_frame: &mut InterruptStackFrame
) {
	println!("EXCEPTION: BREAKPOINT:\n{:#?}", stack_frame);
}