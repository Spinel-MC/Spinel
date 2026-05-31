use crate::scheduler::{ContextScheduler, ExecutionType, Task, TaskSchedule};
use crate::world::World;

#[derive(Default)]
pub struct WorldScheduler {
    scheduler: ContextScheduler<World>,
}

impl WorldScheduler {
    pub fn submit_task(
        &mut self,
        task: impl FnMut(&mut World) -> TaskSchedule + Send + 'static,
        execution_type: ExecutionType,
    ) -> Task {
        self.scheduler.submit_task(task, execution_type)
    }

    pub fn schedule_task(
        &mut self,
        task: impl FnMut(&mut World) -> TaskSchedule + Send + 'static,
        delay: TaskSchedule,
        repeat: TaskSchedule,
        execution_type: ExecutionType,
    ) -> Task {
        self.scheduler
            .schedule_task(task, delay, repeat, execution_type)
    }

    pub fn schedule_next_tick(
        &mut self,
        mut callback: impl FnMut(&mut World) + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_next_tick(move |world| {
            callback(world);
            TaskSchedule::stop()
        })
    }

    pub fn schedule_tick_end(
        &mut self,
        mut callback: impl FnMut(&mut World) + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_tick_end(move |world| {
            callback(world);
            TaskSchedule::stop()
        })
    }

    pub fn process(&mut self, world: &mut World) {
        self.scheduler.process(world);
    }

    pub fn process_tick(&mut self, world: &mut World) {
        self.scheduler.process_tick(world);
    }

    pub fn process_tick_end(&mut self, world: &mut World) {
        self.scheduler.process_tick_end(world);
    }

    pub fn next_tick_callback_count(&self) -> usize {
        self.scheduler.task_count()
    }

    pub fn tick_end_callback_count(&self) -> usize {
        self.scheduler.task_count()
    }
}
