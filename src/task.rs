use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{RawWaker, Waker},
};

use crate::{spawner::Spawner, waker::get_waker_vtable};

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    pub spawner: Spawner,
}

impl Task {
    pub fn waker(self: Arc<Self>) -> Waker {
        let opaque_ptr = Arc::into_raw(self) as *const ();
        let vtable = get_waker_vtable();

        unsafe { Waker::from_raw(RawWaker::new(opaque_ptr, vtable)) }
    }
}
