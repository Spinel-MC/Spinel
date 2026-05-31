use std::sync::OnceLock;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::{Duration, Instant};

type TimerCallback = Box<dyn FnOnce() + Send>;

struct TimerMessage {
    deadline: Instant,
    callback: TimerCallback,
}

static TIMER_SENDER: OnceLock<Sender<TimerMessage>> = OnceLock::new();

pub(crate) fn schedule_after(duration: Duration, callback: impl FnOnce() + Send + 'static) {
    let sender = TIMER_SENDER.get_or_init(start_timer_worker);
    let _ = sender.send(TimerMessage {
        deadline: Instant::now() + duration,
        callback: Box::new(callback),
    });
}

fn start_timer_worker() -> Sender<TimerMessage> {
    let (sender, receiver) = mpsc::channel::<TimerMessage>();
    thread::spawn(move || {
        let mut timers = Vec::<TimerMessage>::new();
        loop {
            timers.sort_by_key(|message| message.deadline);
            let timeout = timers
                .first()
                .map(|message| message.deadline.saturating_duration_since(Instant::now()));
            let received_message = match timeout {
                Some(timeout) => receiver.recv_timeout(timeout),
                None => receiver.recv().map_err(|_| RecvTimeoutError::Disconnected),
            };
            match received_message {
                Ok(message) => timers.push(message),
                Err(RecvTimeoutError::Timeout) => run_due_timers(&mut timers),
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    });
    sender
}

fn run_due_timers(timers: &mut Vec<TimerMessage>) {
    let now = Instant::now();
    let mut timer_index = 0;
    while timer_index < timers.len() {
        if timers[timer_index].deadline > now {
            timer_index += 1;
            continue;
        }
        let timer = timers.remove(timer_index);
        (timer.callback)();
    }
}
