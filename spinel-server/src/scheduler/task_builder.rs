use crate::scheduler::{ContextScheduler, ExecutionType, Task, TaskSchedule};
use std::time::Duration;

pub struct TaskBuilder<'a, T> {
    scheduler: &'a mut ContextScheduler<T>,
    task: Box<dyn FnMut(&mut T) -> TaskSchedule + Send>,
    delay: TaskSchedule,
    repeat: TaskSchedule,
    execution_type: ExecutionType,
    has_repeat_override: bool,
}

impl<'a, T: Send + 'static> TaskBuilder<'a, T> {
    pub(crate) fn new(
        scheduler: &'a mut ContextScheduler<T>,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
    ) -> Self {
        Self {
            scheduler,
            task: Box::new(task),
            delay: TaskSchedule::immediate(),
            repeat: TaskSchedule::stop(),
            execution_type: ExecutionType::TickStart,
            has_repeat_override: false,
        }
    }

    pub fn execution_type(mut self, execution_type: ExecutionType) -> Self {
        self.execution_type = execution_type;
        self
    }

    pub fn delay(mut self, delay: TaskSchedule) -> Self {
        self.delay = delay;
        self
    }

    pub fn repeat(mut self, repeat: TaskSchedule) -> Self {
        self.repeat = repeat;
        self.has_repeat_override = true;
        self
    }

    pub fn delay_duration(self, duration: Duration) -> Self {
        self.delay(TaskSchedule::duration(duration))
    }

    pub fn repeat_duration(self, duration: Duration) -> Self {
        self.repeat(TaskSchedule::duration(duration))
    }

    pub fn schedule(self) -> Task {
        let delay = self.delay.clone();
        let repeat = self.repeat.clone();
        let has_repeat_override = self.has_repeat_override;
        let mut task = self.task;
        self.scheduler.schedule_task_with_repeat_override(
            move |context| task(context),
            delay,
            repeat,
            self.execution_type,
            has_repeat_override,
        )
    }
}
