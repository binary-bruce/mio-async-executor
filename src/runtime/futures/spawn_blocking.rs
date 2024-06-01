use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

pub fn spawn_blocking<T, F>(f: F) -> SpawnBlocking<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    let inner = Arc::new(Mutex::new(SpawnBlockingInner {
        value: None,
        waker: None,
    }));
    let inner_cloned = inner.clone();
    std::thread::spawn(move || {
        let value = f();
        let mut inner = inner_cloned.lock().unwrap();
        inner.value = Some(value);
        if let Some(waker) = inner.waker.take() {
            waker.wake()
        }
    });

    SpawnBlocking { inner }
}

pub struct SpawnBlocking<T> {
    inner: Arc<Mutex<SpawnBlockingInner<T>>>,
}

struct SpawnBlockingInner<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

impl<T: Send> Future for SpawnBlocking<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(value) = inner.value.take() {
            return Poll::Ready(value);
        }

        inner.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {

    use crate::runtime::core::executor::Executor;

    use super::spawn_blocking;

    #[test]
    fn test_long_running_computation() {
        let f = || 42;
        let answer = Executor::block_on(spawn_blocking(f));

        assert_eq!(42, answer);
    }
}
