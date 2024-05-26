use std::{
    sync::Arc,
    task::{RawWaker, RawWakerVTable},
};

use crate::runtime::core::task::Task;

const WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

pub fn get_waker_vtable() -> &'static RawWakerVTable {
    &WAKER_VTABLE
}

fn clone(ptr: *const ()) -> RawWaker {
    let original: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };

    // Increment the inner counter of the arc.
    let cloned = original.clone();

    // now forget the Arc<Task> so the refcount isn't decremented
    std::mem::forget(original);
    std::mem::forget(cloned);

    RawWaker::new(ptr, &WAKER_VTABLE)
}

fn drop(ptr: *const ()) {
    let _: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
}

fn wake(ptr: *const ()) {
    let task: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
    let spawner = task.spawner.clone();

    spawner.spawn_task(task);
}

fn wake_by_ref(ptr: *const ()) {
    let task: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };

    task.spawner.spawn_task(task.clone());

    // we don't actually have ownership of this arc value
    // therefore we must not drop `arc`
    std::mem::forget(task)
}
