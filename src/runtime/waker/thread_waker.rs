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
    let waker = unsafe { Waker::from_raw(raw_waker) };

    waker
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

fn unpark(park: &Park) {
    park.unpark()
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |clone_me| unsafe {
        let arc = Arc::from_raw(clone_me as *const Park);
        std::mem::forget(arc.clone());
        RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
    },
    |wake_me| unsafe { unpark(&Arc::from_raw(wake_me as *const Park)) },
    |wake_by_ref_me| unsafe { unpark(&*(wake_by_ref_me as *const Park)) },
    |drop_me| unsafe { drop(Arc::from_raw(drop_me as *const Park)) },
);

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
