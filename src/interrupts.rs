use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use lazy_static::lazy_static;

use crate::println;
use crate::gdt;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault).
				set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt
	};
}

pub fn init_idt () {
	IDT.load();
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