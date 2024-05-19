use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
    task::{Context, Poll},
};

pub async fn yield_now() {
    Yield::new().await;
}

struct Yield {
    yielded: Arc<AtomicBool>,
}

impl Yield {
    fn new() -> Self {
        Yield {
            yielded: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded.load(Relaxed) {
            Poll::Ready(())
        } else {
            self.yielded.swap(true, Relaxed);
            cx.waker().clone().wake();
            Poll::Pending
        }
    }
}
