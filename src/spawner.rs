use std::{
    future::Future,
    sync::{mpsc, Arc, Mutex},
};

use crate::task::Task;

#[derive(Clone)]
pub struct Spawner {
    pub task_sender: mpsc::SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            spawner: self.clone(),
        });

        self.spawn_task(task)
    }

    pub(crate) fn spawn_task(&self, task: Arc<Task>) {
        self.task_sender.send(task).expect("too many tasks queued");
    }
}
