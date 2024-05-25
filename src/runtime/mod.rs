use std::sync::mpsc;

use executor::Executor;
use spawner::Spawner;

pub mod delay;
pub mod executor;
pub mod spawner;
pub mod udp_socket;
pub mod yield_now;

mod reactor;
mod task;
mod waker;

pub fn new_executor_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;

    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}
