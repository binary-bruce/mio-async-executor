use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::Poll,
    thread,
    time::Duration,
};

pub async fn delay(seconds: u64) {
    Delay::new(seconds).await
}

struct Delay {
    seconds: u64,
    started: AtomicBool,
    completed: Arc<AtomicBool>,
}

impl Delay {
    fn new(seconds: u64) -> Self {
        Self {
            seconds,
            started: AtomicBool::new(false),
            completed: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if !self.started.load(Ordering::Relaxed) {
            self.started.swap(true, Ordering::Relaxed);

            let completed = self.completed.clone();
            let waker = cx.waker().clone();
            let seconds = self.seconds;
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(seconds));
                completed.swap(true, Ordering::Relaxed);

                waker.wake();
            });
        }

        if self.completed.load(Ordering::Relaxed) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
