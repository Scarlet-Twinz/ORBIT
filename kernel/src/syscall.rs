use core::sync::atomic::{AtomicU64, Ordering};

pub const SYS_WRITE: u64 = 1;
pub const SYS_YIELD: u64 = 2;
pub const SYS_EXIT: u64 = 3;
pub const SYS_GET_TICKS: u64 = 4;

static SYSCALLS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyscallResult { Value(u64), Unsupported }

pub fn dispatch(number: u64, arg0: u64, _arg1: u64, _arg2: u64) -> SyscallResult {
    SYSCALLS.fetch_add(1, Ordering::Relaxed);
    match number {
        SYS_WRITE | SYS_YIELD | SYS_EXIT => SyscallResult::Value(arg0),
        SYS_GET_TICKS => SyscallResult::Value(crate::interrupts::ticks()),
        _ => SyscallResult::Unsupported,
    }
}

pub fn count() -> u64 { SYSCALLS.load(Ordering::Relaxed) }

pub fn describe() {
    crate::serial_println!("syscall: ABI=register-contract calls={}", count());
}
