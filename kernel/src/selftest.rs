use crate::storage::{BlockDevice, RamDisk, StorageError, BLOCK_SIZE};
use crate::syscall::{self, SyscallResult, SYS_GET_TICKS, SYS_WRITE};
use crate::tasks::{self, TaskState};

pub fn run() {
    let frame_ok = crate::memory::allocate_frame().is_some();
    crate::serial_println!("selftest: physical-frame-allocation={}", frame_ok);

    let spawned = tasks::spawn_kernel_task().is_some();
    crate::serial_println!("selftest: task-spawn={}", spawned);

    let _blocked = TaskState::Blocked;
    let _exited = TaskState::Exited;

    let mut disk = RamDisk::<2>::new();
    let mut write_buffer = [0u8; BLOCK_SIZE];
    write_buffer[0] = 0x4F;
    write_buffer[1] = 0x52;
    write_buffer[2] = 0x42;
    write_buffer[3] = 0x49;
    write_buffer[4] = 0x54;

    let storage_ok = disk.write_block(0, &write_buffer).is_ok();
    let mut read_buffer = [0u8; BLOCK_SIZE];
    let read_ok = disk.read_block(0, &mut read_buffer).is_ok();
    let data_ok = read_buffer[..5] == write_buffer[..5];
    let range_error = matches!(
        disk.read_block(9, &mut read_buffer),
        Err(StorageError::OutOfRange)
    );
    crate::serial_println!(
        "selftest: ramdisk write={} read={} data={} range-check={}",
        storage_ok,
        read_ok,
        data_ok,
        range_error
    );

    let write_call = syscall::dispatch(SYS_WRITE, 0x4F52424954, 0, 0);
    let tick_call = syscall::dispatch(SYS_GET_TICKS, 0, 0, 0);
    let syscall_ok = matches!(write_call, SyscallResult::Value(_))
        && matches!(tick_call, SyscallResult::Value(_));
    crate::serial_println!("selftest: syscall-dispatch={}", syscall_ok);

    crate::serial_println!(
        "selftest: result={} frames={} syscall_calls={} ready_tasks={}",
        frame_ok && spawned && storage_ok && read_ok && data_ok && range_error && syscall_ok,
        crate::memory::allocated_frames(),
        syscall::count(),
        tasks::ready_tasks()
    );
}
