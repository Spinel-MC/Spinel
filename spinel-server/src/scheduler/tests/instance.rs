use crate::scheduler::{ExecutionType, Scheduler, TaskFuture, TaskSchedule};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn immediate_task_runs_during_process() {
    let mut scheduler = Scheduler::new();
    let count = Arc::new(AtomicUsize::new(0));
    let task_count = count.clone();

    scheduler.submit_task(
        move || {
            task_count.fetch_add(1, Ordering::SeqCst);
            TaskSchedule::stop()
        },
        ExecutionType::TickStart,
    );

    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn next_tick_and_tick_end_tasks_run_in_minestom_order() {
    let mut scheduler = Scheduler::new();
    let events = Arc::new(Mutex::new(Vec::new()));
    let tick_start_events = events.clone();
    let tick_end_events = events.clone();

    scheduler.schedule_next_tick(
        move || {
            tick_start_events.lock().unwrap().push("start");
        },
        ExecutionType::TickStart,
    );
    scheduler.schedule_end_of_tick(move || {
        tick_end_events.lock().unwrap().push("end");
    });

    scheduler.process_tick();
    assert_eq!(events.lock().unwrap().as_slice(), &["start"]);
    scheduler.process_tick_end();
    assert_eq!(events.lock().unwrap().as_slice(), &["start", "end"]);
}

#[test]
fn repeating_task_runs_until_cancelled() {
    let mut scheduler = Scheduler::new();
    let count = Arc::new(AtomicUsize::new(0));
    let task_count = count.clone();
    let task = scheduler.schedule_task(
        move || {
            task_count.fetch_add(1, Ordering::SeqCst);
        },
        TaskSchedule::next_tick(),
        TaskSchedule::next_tick(),
        ExecutionType::TickStart,
    );

    scheduler.process_tick();
    scheduler.process_tick();
    task.cancel();
    scheduler.process_tick();

    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert!(!task.is_alive());
}

#[test]
fn parked_task_resumes_after_unpark() {
    let mut scheduler = Scheduler::new();
    let count = Arc::new(AtomicUsize::new(0));
    let task_count = count.clone();
    let task = scheduler.submit_task(
        move || {
            task_count.fetch_add(1, Ordering::SeqCst);
            TaskSchedule::park()
        },
        ExecutionType::TickStart,
    );

    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert!(task.is_parked());
    task.unpark();
    scheduler.process();

    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[test]
fn duration_and_future_schedules_resume_tasks() {
    let mut scheduler = Scheduler::new();
    let count = Arc::new(AtomicUsize::new(0));
    let duration_count = count.clone();
    let future_count = count.clone();
    let (task_future, completer) = TaskFuture::new();

    scheduler.submit_task(
        move || {
            let previous_count = duration_count.fetch_add(1, Ordering::SeqCst);
            if previous_count >= 2 {
                return TaskSchedule::stop();
            }
            TaskSchedule::duration(Duration::from_millis(1))
        },
        ExecutionType::TickStart,
    );
    scheduler.submit_task(
        move || {
            let previous_count = future_count.fetch_add(1, Ordering::SeqCst);
            if previous_count >= 3 {
                return TaskSchedule::stop();
            }
            TaskSchedule::future(task_future.clone())
        },
        ExecutionType::TickStart,
    );

    assert_eq!(count.load(Ordering::SeqCst), 2);
    std::thread::sleep(Duration::from_millis(10));
    scheduler.process();
    completer.complete();
    scheduler.process();

    assert!(count.load(Ordering::SeqCst) >= 4);
}
