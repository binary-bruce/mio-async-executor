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
    let task = unsafe { Arc::from_raw(ptr as *const Task) };
    std::mem::forget(task.clone());

    RawWaker::new(Arc::into_raw(task) as *const (), &WAKER_VTABLE)
}

fn wake(ptr: *const ()) {
    let task: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
    let spawner = task.spawner.clone();

    spawner.spawn_task(task);
}

fn wake_by_ref(ptr: *const ()) {
    let task: &Arc<Task> = unsafe { &Arc::from_raw(ptr as _) };
    task.spawner.spawn_task(task.clone());
}

fn drop(ptr: *const ()) {
    let _: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
}
