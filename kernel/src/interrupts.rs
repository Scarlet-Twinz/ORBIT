use core::sync::atomic::{AtomicU64, Ordering};
use spin::Once;
use x86_64::instructions::{hlt, interrupts as cpu_interrupts, port::Port};
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
};

pub const TIMER_VECTOR: u8 = 32;
pub const KEYBOARD_VECTOR: u8 = 33;

static IDT: Once<InterruptDescriptorTable> = Once::new();
static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    remap_pic();
    let idt = IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[usize::from(TIMER_VECTOR)].set_handler_fn(timer_handler);
        idt[usize::from(KEYBOARD_VECTOR)].set_handler_fn(keyboard_handler);
        idt
    });
    idt.load();
    unmask_irq(0);
    unmask_irq(1);
    crate::serial_println!("interrupts: IDT loaded, PIC remapped, IRQ0/IRQ1 enabled");
}

pub fn enable() {
    cpu_interrupts::enable();
}

pub fn ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

extern "x86-interrupt" fn breakpoint_handler(_frame: InterruptStackFrame) {
    crate::serial_println!("interrupt: breakpoint");
}

extern "x86-interrupt" fn page_fault_handler(
    frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let address = Cr2::read();
    match address {
        Ok(address) => crate::serial_println!(
            "fatal: page fault addr={:#x} error={:?} ip={:#x}",
            address.as_u64(),
            error_code,
            frame.instruction_pointer.as_u64()
        ),
        Err(_) => crate::serial_println!(
            "fatal: page fault addr=<invalid> error={:?} ip={:#x}",
            error_code,
            frame.instruction_pointer.as_u64()
        ),
    }
    halt_forever();
}

extern "x86-interrupt" fn double_fault_handler(
    _frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::serial_println!("fatal: double fault error={:#x}", error_code);
    halt_forever()
}

extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    crate::tasks::on_timer_tick();
    send_eoi(0);
}

extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    let mut data_port = Port::new(0x60);
    let scancode: u8 = unsafe { data_port.read() };
    crate::keyboard::handle_scancode(scancode);
    send_eoi(1);
}

fn remap_pic() {
    let mut command1 = Port::new(0x20);
    let mut data1 = Port::new(0x21);
    let mut command2 = Port::new(0xA0);
    let mut data2 = Port::new(0xA1);
    unsafe {
        command1.write(0x11u8);
        command2.write(0x11u8);
        data1.write(0x20u8);
        data2.write(0x28u8);
        data1.write(0x04u8);
        data2.write(0x02u8);
        data1.write(0x01u8);
        data2.write(0x01u8);
        data1.write(0xFCu8);
        data2.write(0xFFu8);
    }
}

fn unmask_irq(irq: u8) {
    let (port, bit) = if irq < 8 {
        (0x21, irq)
    } else {
        (0xA1, irq - 8)
    };
    let mut data = Port::new(port);
    let current: u8 = unsafe { data.read() };
    unsafe {
        data.write(current & !(1 << bit));
    }
}

fn send_eoi(irq: u8) {
    if irq >= 8 {
        let mut slave = Port::new(0xA0);
        unsafe {
            slave.write(0x20u8);
        }
    }
    let mut master = Port::new(0x20);
    unsafe {
        master.write(0x20u8);
    }
}

fn halt_forever() -> ! {
    loop {
        hlt();
    }
}

pub fn init_pit(hz: u32) {
    let divisor = (1_193_182u32 / hz.max(1)).min(u16::MAX as u32) as u16;
    let mut command = Port::new(0x43);
    let mut channel0 = Port::new(0x40);
    unsafe {
        command.write(0x36u8);
        channel0.write((divisor & 0xFF) as u8);
        channel0.write((divisor >> 8) as u8);
    }
    crate::serial_println!("timer: PIT configured at {} Hz", hz.max(1));
}
