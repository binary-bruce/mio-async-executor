use std::{
    future::Future,
    sync::{mpsc, Arc, Mutex},
};

use crate::task::Task;

#[derive(Clone)]
pub struct Spawner {
    pub task_sender: mpsc::SyncSender<Arc<Task>>,
}

/// Spawner creates a task based on the passed-in future and send it to the sender channel so that the receiver end(executor) can be notified
impl Spawner {
    /// a task is spawned by wrapping the future and send the task to the channel
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            spawner: self.clone(),
        });

        self.spawn_task(task)
    }

    /// send the task(which may be ready to make some progress) to the channel
    pub(crate) fn spawn_task(&self, task: Arc<Task>) {
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
