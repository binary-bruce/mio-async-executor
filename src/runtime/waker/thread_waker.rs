// port from extreme

use std::{
    sync::{Arc, Condvar, Mutex},
    task::{RawWaker, RawWakerVTable, Waker},
};

pub fn new_park() -> Arc<Park> {
    Arc::new(Park::default())
}

pub fn construct_waker(park: Arc<Park>) -> Waker {
    let sender = Arc::into_raw(park.clone());
    let raw_waker = RawWaker::new(sender as *const _, &VTABLE);

    unsafe { Waker::from_raw(raw_waker) }
}

#[derive(Default)]
pub struct Park(Mutex<bool>, Condvar);

impl Park {
    pub fn park(&self) {
        let mut runnable = self.0.lock().unwrap();
        while !*runnable {
            runnable = self.1.wait(runnable).unwrap();
        }
        *runnable = false;
    }

    pub fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

fn clone(ptr: *const ()) -> RawWaker {
    let park = unsafe { Arc::from_raw(ptr as *const Park) };
    std::mem::forget(park.clone());
    RawWaker::new(Arc::into_raw(park) as *const (), &VTABLE)
}

fn wake(ptr: *const ()) {
    let park: Arc<Park> = unsafe { Arc::from_raw(ptr as _) };
    park.unpark();
}

fn wake_by_ref(ptr: *const ()) {
    let park: &Arc<Park> = unsafe { &Arc::from_raw(ptr as _) };
    park.unpark();

    // we don't actually have ownership of this park value
    // therefore we must not drop `arc`
    std::mem::forget(park)
}

fn drop(ptr: *const ()) {
    let _: Arc<Park> = unsafe { Arc::from_raw(ptr as _) };
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::new_park;

    #[test]
    fn should_not_deadlock() {
        let park = new_park();
        let park_cloned = park.clone();
        let handle = thread::spawn(move || park_cloned.unpark());
        park.park(); // to be unparked by the spawned thread

        let _ = handle.join();
    }
}
