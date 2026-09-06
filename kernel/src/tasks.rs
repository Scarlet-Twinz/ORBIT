use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

const MAX_TASKS: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Exited,
}

#[derive(Clone, Copy, Debug)]
pub struct TaskControlBlock {
    pub id: TaskId,
    pub state: TaskState,
    pub ticks: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Scheduler {
    tasks: [Option<TaskControlBlock>; MAX_TASKS],
    next_id: u64,
    current: usize,
}

impl Scheduler {
    const fn new() -> Self {
        Self {
            tasks: [None; MAX_TASKS],
            next_id: 1,
            current: 0,
        }
    }

    fn spawn(&mut self) -> Option<TaskId> {
        let slot = self.tasks.iter().position(Option::is_none)?;
        let id = TaskId(self.next_id);
        self.next_id += 1;
        self.tasks[slot] = Some(TaskControlBlock {
            id,
            state: TaskState::Ready,
            ticks: 0,
        });
        Some(id)
    }

    fn tick(&mut self) {
        if let Some(task) = self.tasks[self.current].as_mut() {
            if task.state == TaskState::Running {
                task.ticks += 1;
                task.state = TaskState::Ready;
            }
        }
        for offset in 1..=MAX_TASKS {
            let slot = (self.current + offset) % MAX_TASKS;
            if let Some(task) = self.tasks[slot].as_mut() {
                if task.state == TaskState::Ready {
                    task.state = TaskState::Running;
                    self.current = slot;
                    return;
                }
            }
        }
    }

    fn ready_count(&self) -> usize {
        self.tasks
            .iter()
            .flatten()
            .filter(|task| matches!(task.state, TaskState::Ready | TaskState::Running))
            .count()
    }
}

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
static SCHEDULE_TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init() {
    let mut scheduler = SCHEDULER.lock();
    let idle = scheduler.spawn();
    let bootstrap = scheduler.spawn();
    if let Some(id) = idle {
        crate::serial_println!("task: created idle task id={}", id.0);
    }
    if let Some(id) = bootstrap {
        crate::serial_println!("task: created kernel task id={}", id.0);
    }
}

pub fn on_timer_tick() {
    SCHEDULE_TICKS.fetch_add(1, Ordering::Relaxed);
    SCHEDULER.lock().tick();
}

pub fn spawn_kernel_task() -> Option<TaskId> {
    SCHEDULER.lock().spawn()
}
pub fn ready_tasks() -> usize {
    SCHEDULER.lock().ready_count()
}

pub fn describe() {
    crate::serial_println!(
        "scheduler: ready_tasks={} ticks={}",
        ready_tasks(),
        SCHEDULE_TICKS.load(Ordering::Relaxed)
    );
}
