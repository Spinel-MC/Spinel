use crate::scheduler::task::TaskState;
use crate::scheduler::{ExecutionType, Task, TaskBuilder, TaskId, TaskSchedule};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

type ScheduledCallback<T> = Box<dyn FnMut(&mut T) -> TaskSchedule + Send>;
type ScheduledTask<T> = Arc<Mutex<ScheduledTaskState<T>>>;

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

pub struct Scheduler {
    inner: ContextScheduler<()>,
}

pub struct ContextScheduler<T> {
    inner: Arc<Mutex<SchedulerInner<T>>>,
}

struct SchedulerInner<T> {
    tick_state: u64,
    tick_start_queue: BTreeMap<u64, Vec<TaskId>>,
    tick_end_queue: BTreeMap<u64, Vec<TaskId>>,
    tick_start_ready: VecDeque<TaskId>,
    tick_end_ready: VecDeque<TaskId>,
    tasks: HashMap<TaskId, ScheduledTask<T>>,
}

struct ScheduledTaskState<T> {
    execution_type: ExecutionType,
    task: ScheduledCallback<T>,
    state: Arc<TaskState>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            inner: ContextScheduler::new(),
        }
    }

    pub fn process(&mut self) {
        self.inner.process(&mut ());
    }

    pub fn process_tick(&mut self) {
        self.inner.process_tick(&mut ());
    }

    pub fn process_tick_end(&mut self) {
        self.inner.process_tick_end(&mut ());
    }

    pub fn submit_task(
        &mut self,
        mut task: impl FnMut() -> TaskSchedule + Send + 'static,
        execution_type: ExecutionType,
    ) -> Task {
        let task = self.inner.submit_task(move |_| task(), execution_type);
        self.inner.process(&mut ());
        task
    }

    pub fn build_task(&mut self, mut task: impl FnMut() + Send + 'static) -> TaskBuilder<'_, ()> {
        self.inner.build_task(move |_| {
            task();
            TaskSchedule::stop()
        })
    }

    pub fn schedule_task(
        &mut self,
        mut task: impl FnMut() + Send + 'static,
        delay: TaskSchedule,
        repeat: TaskSchedule,
        execution_type: ExecutionType,
    ) -> Task {
        self.inner.schedule_task(
            move |_| {
                task();
                TaskSchedule::stop()
            },
            delay,
            repeat,
            execution_type,
        )
    }

    pub fn schedule_next_tick(
        &mut self,
        task: impl FnMut() + Send + 'static,
        execution_type: ExecutionType,
    ) -> Task {
        self.schedule_task(
            task,
            TaskSchedule::next_tick(),
            TaskSchedule::stop(),
            execution_type,
        )
    }

    pub fn schedule_end_of_tick(&mut self, task: impl FnMut() + Send + 'static) -> Task {
        self.schedule_next_process(task, ExecutionType::TickEnd)
    }

    pub fn schedule_next_process(
        &mut self,
        task: impl FnMut() + Send + 'static,
        execution_type: ExecutionType,
    ) -> Task {
        self.schedule_task(
            task,
            TaskSchedule::immediate(),
            TaskSchedule::stop(),
            execution_type,
        )
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + 'static> ContextScheduler<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SchedulerInner {
                tick_state: 0,
                tick_start_queue: BTreeMap::new(),
                tick_end_queue: BTreeMap::new(),
                tick_start_ready: VecDeque::new(),
                tick_end_ready: VecDeque::new(),
                tasks: HashMap::new(),
            })),
        }
    }

    pub fn process(&mut self, context: &mut T) {
        self.process_tick_delta(context, 0);
    }

    pub fn process_tick(&mut self, context: &mut T) {
        self.process_tick_delta(context, 1);
    }

    pub fn process_tick_end(&mut self, context: &mut T) {
        self.move_due_tick_end_tasks_to_ready();
        self.run_ready_tasks(context, ExecutionType::TickEnd);
    }

    pub fn submit_task(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
        execution_type: ExecutionType,
    ) -> Task {
        let (task_id, _task_state, task_handle) = self.insert_task(task, execution_type);
        enqueue_task(&self.inner, task_id, execution_type);
        task_handle
    }

    pub(crate) fn schedule_task_with_repeat_override(
        &mut self,
        mut task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
        delay: TaskSchedule,
        repeat: TaskSchedule,
        execution_type: ExecutionType,
        has_repeat_override: bool,
    ) -> Task {
        let task = move |context: &mut T| {
            let schedule = task(context);
            if has_repeat_override {
                return repeat.clone();
            }
            schedule
        };
        let (task_id, task_state, task_handle) = self.insert_task(task, execution_type);
        self.apply_schedule(task_id, execution_type, task_state, delay);
        task_handle
    }

    fn insert_task(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
        execution_type: ExecutionType,
    ) -> (TaskId, Arc<TaskState>, Task) {
        let id = TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst));
        let state = TaskState::new();
        let scheduler = Arc::downgrade(&self.inner);
        let unpark_task = Arc::new(move |task_id| {
            if let Some(scheduler) = scheduler.upgrade() {
                enqueue_unparked_task(&scheduler, task_id);
            }
        });
        let scheduled_task = Arc::new(Mutex::new(ScheduledTaskState {
            execution_type,
            task: Box::new(task),
            state: state.clone(),
        }));
        if let Ok(mut inner) = self.inner.lock() {
            inner.tasks.insert(id, scheduled_task);
        }
        let task_handle = Task::new(id, execution_type, state, unpark_task);
        let task_state = self
            .task(id)
            .and_then(|task| task.lock().ok().map(|task| task.state.clone()))
            .unwrap_or_else(TaskState::new);
        (id, task_state, task_handle)
    }

    pub fn build_task(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
    ) -> TaskBuilder<'_, T> {
        TaskBuilder::new(self, task)
    }

    pub fn schedule_task(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
        delay: TaskSchedule,
        repeat: TaskSchedule,
        execution_type: ExecutionType,
    ) -> Task {
        self.schedule_task_with_repeat_override(task, delay, repeat, execution_type, true)
    }

    pub fn schedule_next_tick(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.schedule_task(
            task,
            TaskSchedule::next_tick(),
            TaskSchedule::stop(),
            ExecutionType::TickStart,
        )
    }

    pub fn schedule_tick_end(
        &mut self,
        task: impl FnMut(&mut T) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.schedule_task(
            task,
            TaskSchedule::immediate(),
            TaskSchedule::stop(),
            ExecutionType::TickEnd,
        )
    }

    pub fn task_count(&self) -> usize {
        self.inner
            .lock()
            .map(|inner| inner.tasks.len())
            .unwrap_or_default()
    }

    fn process_tick_delta(&mut self, context: &mut T, tick_delta: u64) {
        self.move_due_tick_start_tasks_to_ready(tick_delta);
        self.run_ready_tasks(context, ExecutionType::TickStart);
    }

    fn move_due_tick_start_tasks_to_ready(&self, tick_delta: u64) {
        let Ok(mut inner) = self.inner.lock() else {
            return;
        };
        inner.tick_state = inner.tick_state.saturating_add(tick_delta);
        let due_ticks = inner
            .tick_start_queue
            .keys()
            .copied()
            .take_while(|target_tick| *target_tick <= inner.tick_state)
            .collect::<Vec<_>>();
        due_ticks.into_iter().for_each(|target_tick| {
            if let Some(task_ids) = inner.tick_start_queue.remove(&target_tick) {
                inner.tick_start_ready.extend(task_ids);
            }
        });
    }

    fn move_due_tick_end_tasks_to_ready(&self) {
        let Ok(mut inner) = self.inner.lock() else {
            return;
        };
        let due_ticks = inner
            .tick_end_queue
            .keys()
            .copied()
            .take_while(|target_tick| *target_tick <= inner.tick_state)
            .collect::<Vec<_>>();
        due_ticks.into_iter().for_each(|target_tick| {
            if let Some(task_ids) = inner.tick_end_queue.remove(&target_tick) {
                inner.tick_end_ready.extend(task_ids);
            }
        });
    }

    fn run_ready_tasks(&mut self, context: &mut T, execution_type: ExecutionType) {
        while let Some(task_id) = self.pop_ready_task(execution_type) {
            self.handle_task(context, task_id);
        }
    }

    fn pop_ready_task(&self, execution_type: ExecutionType) -> Option<TaskId> {
        self.inner
            .lock()
            .ok()
            .and_then(|mut inner| match execution_type {
                ExecutionType::TickStart => inner.tick_start_ready.pop_front(),
                ExecutionType::TickEnd => inner.tick_end_ready.pop_front(),
            })
    }

    fn handle_task(&mut self, context: &mut T, task_id: TaskId) {
        let Some(task) = self.task(task_id) else {
            return;
        };
        let Ok(mut task) = task.lock() else {
            self.remove_task(task_id);
            return;
        };
        if !task.state.alive.load(Ordering::SeqCst) {
            self.remove_task(task_id);
            return;
        }
        let schedule = (task.task)(context);
        let execution_type = task.execution_type;
        let task_state = task.state.clone();
        drop(task);
        self.apply_schedule(task_id, execution_type, task_state, schedule);
    }

    fn task(&self, task_id: TaskId) -> Option<ScheduledTask<T>> {
        self.inner
            .lock()
            .ok()
            .and_then(|inner| inner.tasks.get(&task_id).cloned())
    }

    fn apply_schedule(
        &self,
        task_id: TaskId,
        execution_type: ExecutionType,
        task_state: Arc<TaskState>,
        schedule: TaskSchedule,
    ) {
        match schedule {
            TaskSchedule::Duration(duration) => {
                let scheduler = Arc::downgrade(&self.inner);
                crate::scheduler::timer::schedule_after(duration, move || {
                    if let Some(scheduler) = scheduler.upgrade() {
                        enqueue_task(&scheduler, task_id, execution_type);
                    }
                });
            }
            TaskSchedule::Future(task_future) => {
                let scheduler = Arc::downgrade(&self.inner);
                task_future.then_run(move || {
                    if let Some(scheduler) = scheduler.upgrade() {
                        enqueue_task(&scheduler, task_id, execution_type);
                    }
                });
            }
            TaskSchedule::Immediate => {
                enqueue_task(&self.inner, task_id, execution_type);
            }
            TaskSchedule::Park => {
                task_state.parked.store(true, Ordering::SeqCst);
            }
            TaskSchedule::Stop => {
                task_state.alive.store(false, Ordering::SeqCst);
                self.remove_task(task_id);
            }
            TaskSchedule::Tick(ticks) => {
                self.schedule_for_tick(task_id, execution_type, ticks);
            }
        }
    }

    fn schedule_for_tick(&self, task_id: TaskId, execution_type: ExecutionType, ticks: u64) {
        let Ok(mut inner) = self.inner.lock() else {
            return;
        };
        let target_tick = inner.tick_state.saturating_add(ticks);
        let queue = match execution_type {
            ExecutionType::TickStart => &mut inner.tick_start_queue,
            ExecutionType::TickEnd => &mut inner.tick_end_queue,
        };
        queue.entry(target_tick).or_default().push(task_id);
    }

    fn remove_task(&self, task_id: TaskId) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.tasks.remove(&task_id);
        }
    }
}

impl<T: Send + 'static> Default for ContextScheduler<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn enqueue_unparked_task<T>(scheduler: &Arc<Mutex<SchedulerInner<T>>>, task_id: TaskId) {
    let Some(task_state) = scheduler
        .lock()
        .ok()
        .and_then(|inner| inner.tasks.get(&task_id).cloned())
        .and_then(|task| task.lock().ok().map(|task| TaskState::weak(&task.state)))
        .and_then(|state| Weak::upgrade(&state))
    else {
        return;
    };
    if !task_state.parked.swap(false, Ordering::SeqCst) {
        return;
    }
    enqueue_task(scheduler, task_id, ExecutionType::TickStart);
}

fn enqueue_task<T>(
    scheduler: &Arc<Mutex<SchedulerInner<T>>>,
    task_id: TaskId,
    execution_type: ExecutionType,
) {
    let Ok(mut inner) = scheduler.lock() else {
        return;
    };
    match execution_type {
        ExecutionType::TickStart => inner.tick_start_ready.push_back(task_id),
        ExecutionType::TickEnd => inner.tick_end_ready.push_back(task_id),
    }
}
