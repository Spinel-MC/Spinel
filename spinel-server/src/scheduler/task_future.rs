use std::sync::{Arc, Mutex, Weak};

type CompletionCallback = Box<dyn FnOnce() + Send>;

#[derive(Clone)]
pub struct TaskFuture {
    state: Arc<Mutex<TaskFutureState>>,
}

#[derive(Clone)]
pub struct TaskFutureCompleter {
    state: Weak<Mutex<TaskFutureState>>,
}

struct TaskFutureState {
    is_complete: bool,
    callbacks: Vec<CompletionCallback>,
}

impl TaskFuture {
    pub fn new() -> (Self, TaskFutureCompleter) {
        let state = Arc::new(Mutex::new(TaskFutureState {
            is_complete: false,
            callbacks: Vec::new(),
        }));
        (
            Self {
                state: state.clone(),
            },
            TaskFutureCompleter {
                state: Arc::downgrade(&state),
            },
        )
    }

    pub fn is_complete(&self) -> bool {
        self.state
            .lock()
            .map(|state| state.is_complete)
            .unwrap_or(true)
    }

    pub(crate) fn then_run(&self, callback: impl FnOnce() + Send + 'static) {
        let Ok(mut state) = self.state.lock() else {
            callback();
            return;
        };
        if state.is_complete {
            drop(state);
            callback();
            return;
        }
        state.callbacks.push(Box::new(callback));
    }
}

impl TaskFutureCompleter {
    pub fn complete(&self) -> bool {
        let Some(state) = self.state.upgrade() else {
            return false;
        };
        let Ok(mut state) = state.lock() else {
            return false;
        };
        if state.is_complete {
            return false;
        }
        state.is_complete = true;
        let callbacks = std::mem::take(&mut state.callbacks);
        drop(state);
        callbacks.into_iter().for_each(|callback| callback());
        true
    }
}
