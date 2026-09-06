use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use bootloader_api::BootInfo;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

const PAGE_SIZE: u64 = 4096;
const MAX_RANGES: usize = 128;

#[derive(Clone, Copy, Debug)]
struct UsableRange { next: u64, end: u64 }

#[derive(Clone, Copy, Debug)]
struct PhysicalAllocator {
    ranges: [UsableRange; MAX_RANGES],
    range_count: usize,
    current: usize,
    allocated: u64,
}

impl PhysicalAllocator {
    const EMPTY: UsableRange = UsableRange { next: 0, end: 0 };

    const fn new() -> Self {
        Self { ranges: [Self::EMPTY; MAX_RANGES], range_count: 0, current: 0, allocated: 0 }
    }

    fn initialize(&mut self, regions: &[MemoryRegion]) {
        *self = Self::new();
        for region in regions {
            if region.kind != MemoryRegionKind::Usable || region.start >= region.end || self.range_count == MAX_RANGES { continue; }
            let start = align_up(region.start, PAGE_SIZE);
            let end = align_down(region.end, PAGE_SIZE);
            if start < end {
                self.ranges[self.range_count] = UsableRange { next: start, end };
                self.range_count += 1;
            }
        }
    }

    fn allocate_next(&mut self) -> Option<PhysFrame<Size4KiB>> {
        while self.current < self.range_count {
            let range = &mut self.ranges[self.current];
            if range.next < range.end {
                let address = range.next;
                range.next += PAGE_SIZE;
                self.allocated += 1;
                return PhysFrame::from_start_address(PhysAddr::new(address)).ok();
            }
            self.current += 1;
        }
        None
    }

    fn remaining_bytes(&self) -> u64 {
        self.ranges[..self.range_count].iter().map(|range| range.end - range.next).sum()
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysicalAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> { self.allocate_next() }
}

static ALLOCATOR: Mutex<PhysicalAllocator> = Mutex::new(PhysicalAllocator::new());

pub fn init(boot_info: &'static mut BootInfo) {
    let mut allocator = ALLOCATOR.lock();
    allocator.initialize(&boot_info.memory_regions);
    serial_println!("memory: usable_regions={} remaining_bytes={}", allocator.range_count, allocator.remaining_bytes());
}

pub fn allocate_frame() -> Option<PhysFrame<Size4KiB>> { ALLOCATOR.lock().allocate_next() }
pub fn allocated_frames() -> u64 { ALLOCATOR.lock().allocated }
pub fn remaining_bytes() -> u64 { ALLOCATOR.lock().remaining_bytes() }

pub fn describe() {
    let allocator = ALLOCATOR.lock();
    serial_println!("memory: ranges={} allocated_frames={} remaining_bytes={}", allocator.range_count, allocator.allocated, allocator.remaining_bytes());
}

const fn align_up(value: u64, alignment: u64) -> u64 { (value + alignment - 1) & !(alignment - 1) }
const fn align_down(value: u64, alignment: u64) -> u64 { value & !(alignment - 1) }
