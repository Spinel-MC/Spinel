use crate::scheduler::ExecutionType;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

#[derive(Clone)]
pub struct Task {
    id: TaskId,
    execution_type: ExecutionType,
    state: Arc<TaskState>,
    unpark_task: Arc<dyn Fn(TaskId) + Send + Sync>,
}

pub(crate) struct TaskState {
    pub(crate) alive: AtomicBool,
    pub(crate) parked: AtomicBool,
}

impl Task {
    pub(crate) fn new(
        id: TaskId,
        execution_type: ExecutionType,
        state: Arc<TaskState>,
        unpark_task: Arc<dyn Fn(TaskId) + Send + Sync>,
    ) -> Self {
        Self {
            id,
            execution_type,
            state,
            unpark_task,
        }
    }

    pub const fn id(&self) -> TaskId {
        self.id
    }

    pub const fn execution_type(&self) -> ExecutionType {
        self.execution_type
    }

    pub fn unpark(&self) {
        (self.unpark_task)(self.id);
    }

    pub fn is_parked(&self) -> bool {
        self.state.parked.load(Ordering::SeqCst)
    }

    pub fn cancel(&self) {
        self.state.alive.store(false, Ordering::SeqCst);
    }

    pub fn is_alive(&self) -> bool {
        self.state.alive.load(Ordering::SeqCst)
    }
}

impl TaskState {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            alive: AtomicBool::new(true),
            parked: AtomicBool::new(false),
        })
    }

    pub(crate) fn weak(state: &Arc<Self>) -> Weak<Self> {
        Arc::downgrade(state)
    }
}
