use std::{
    future::Future,
    sync::{mpsc, Arc},
    task::Context,
};

use super::task::Task;

pub struct Executor {
    pub ready_queue: mpsc::Receiver<Arc<Task>>,
}

/// Executor just pick the task from a receiver channel and try to poll it to make some progress
/// until the channel gets closed(no more task to poll)
impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future = task.future.lock().unwrap();

            // make a context (explained later)
            let waker = Arc::clone(&task).waker();
            let mut context = Context::from_waker(&waker);

            // Allow the future some CPU time to make progress
            let _ = future.as_mut().poll(&mut context);
        }
    }

    pub fn block_on<F: Future>(future: F) -> F::Output {
        extreme::run(future)
    }
}

#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_block_on() {
        let answer = Executor::block_on(async { 42 });

        assert_eq!(42, answer);
    }
}
