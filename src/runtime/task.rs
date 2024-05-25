use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{RawWaker, Waker},
};

use super::{spawner::Spawner, waker::get_waker_vtable};

/// a task is a pinned future that is to be polled by the executor
pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    pub spawner: Spawner,
}

impl Task {
    /// construct a waker from the task, with the waker, the task would be woken when it's ready to make some progress
    pub fn waker(self: Arc<Self>) -> Waker {
        let opaque_ptr = Arc::into_raw(self) as *const ();
        let vtable = get_waker_vtable();

        unsafe { Waker::from_raw(RawWaker::new(opaque_ptr, vtable)) }
    }
}
