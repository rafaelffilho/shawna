use x86_64::structures::paging::{PageTable, PhysFrame, MapperAllSizes, MappedPageTable};
use x86_64::structures::paging::{Page, Size4KiB, Mapper, FrameAllocator};
use x86_64::registers::control::Cr3;
use x86_64::{PhysAddr, VirtAddr};

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator {
	memory_map: &'static MemoryMap,
	next: usize
}

impl BootInfoFrameAllocator {
	pub unsafe fn init (memory_map: &'static MemoryMap) -> Self {
		BootInfoFrameAllocator {
			memory_map,
			next: 0
		}
	}
	
	fn usable_frames (&self) -> impl Iterator<Item = PhysFrame> {
		let regions = self.memory_map.iter();
		let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
		let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
		let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

		frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
	}
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
	fn allocate_frame(&mut self) -> Option<PhysFrame> {
		let frame = self.usable_frames().nth(self.next);
		self.next += 1;
		frame
	}
}

pub unsafe fn init (physical_memory_offset: u64) -> impl MapperAllSizes {
	let level_4_table = active_level_4_table(physical_memory_offset);
	let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
		let phys = frame.start_address().as_u64();
		let virt = VirtAddr::new(phys + physical_memory_offset);
		virt.as_mut_ptr()
	};
	MappedPageTable::new(level_4_table, phys_to_virt)
}

unsafe fn active_level_4_table (physical_memory_offset: u64) -> &'static mut PageTable {

	let (level_4_table_frame, _) = Cr3::read();

	let phys = level_4_table_frame.start_address();
	let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	&mut *page_table_ptr
}