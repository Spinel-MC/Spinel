use std::sync::{Arc, Mutex, OnceLock, mpsc};

type ChunkLoadingTask = Box<dyn FnOnce() + Send + 'static>;

const MINIMUM_CHUNK_LOADING_WORKERS: usize = 16;

pub(crate) struct ChunkLoadingExecutor {
    task_sender: mpsc::Sender<ChunkLoadingTask>,
}

impl ChunkLoadingExecutor {
    pub(crate) fn global() -> &'static Self {
        static EXECUTOR: OnceLock<ChunkLoadingExecutor> = OnceLock::new();
        EXECUTOR.get_or_init(Self::new)
    }

    pub(crate) fn execute(&self, task: impl FnOnce() + Send + 'static) {
        let _ = self.task_sender.send(Box::new(task));
    }

    fn new() -> Self {
        let (task_sender, task_receiver) = mpsc::channel::<ChunkLoadingTask>();
        let task_receiver = Arc::new(Mutex::new(task_receiver));
        let worker_count = std::thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
            .max(MINIMUM_CHUNK_LOADING_WORKERS);

        (0..worker_count).for_each(|_| {
            let task_receiver = task_receiver.clone();
            std::thread::spawn(move || {
                while let Ok(task) = task_receiver
                    .lock()
                    .map_err(|_| ())
                    .and_then(|receiver| receiver.recv().map_err(|_| ()))
                {
                    task();
                }
            });
        });

        Self { task_sender }
    }
}
