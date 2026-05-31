use crate::scheduler::TaskFuture;
use std::time::Duration;

#[derive(Clone)]
pub enum TaskSchedule {
    Duration(Duration),
    Future(TaskFuture),
    Immediate,
    Park,
    Stop,
    Tick(u64),
}

impl TaskSchedule {
    pub const fn duration(duration: Duration) -> Self {
        Self::Duration(duration)
    }

    pub const fn tick(ticks: u64) -> Self {
        Self::Tick(ticks)
    }

    pub const fn future(task_future: TaskFuture) -> Self {
        Self::Future(task_future)
    }

    pub const fn park() -> Self {
        Self::Park
    }

    pub const fn stop() -> Self {
        Self::Stop
    }

    pub const fn immediate() -> Self {
        Self::Immediate
    }

    pub const fn next_tick() -> Self {
        Self::Tick(1)
    }

    pub const fn millis(millis: u64) -> Self {
        Self::Duration(Duration::from_millis(millis))
    }

    pub const fn seconds(seconds: u64) -> Self {
        Self::Duration(Duration::from_secs(seconds))
    }
}
