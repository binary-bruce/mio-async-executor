// REACTOR

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Mutex, OnceLock},
    task::{Context, Poll, Waker},
};

use mio::{Events, Registry, Token};

use super::event_token::EventToken;

enum Status {
    Awaited(Waker),
    Happened,
}

/// Reactor is an event loop, polling on a list of registered events in a dedicated thread
/// if an event happens, the reactor finds the respective waker and wake the task(send it to the channel to notify the executor)
pub struct Reactor {
    pub registry: Registry,
    statuses: Mutex<HashMap<Token, Status>>,
}

impl Reactor {
    pub fn get() -> &'static Self {
        static REACTOR: OnceLock<Reactor> = OnceLock::new();

        REACTOR.get_or_init(|| {
            let poll = mio::Poll::new().unwrap();
            let reactor = Reactor {
                registry: poll.registry().try_clone().unwrap(),
                statuses: Mutex::new(HashMap::new()),
            };

            std::thread::Builder::new()
                .name("reactor".to_owned())
                .spawn(|| run(poll))
                .unwrap();

            reactor
        })
    }

    /// register the waker
    pub fn register_waker(&self, token: Token, cx: &mut Context) -> Poll<std::io::Result<()>> {
        let mut guard = self.statuses.lock().unwrap();
        match guard.entry(token) {
            Entry::Vacant(vacant) => {
                vacant.insert(Status::Awaited(cx.waker().clone()));
                Poll::Pending
            }
            Entry::Occupied(mut occupied) => {
                match occupied.get() {
                    Status::Awaited(waker) => {
                        // Check if the new waker is the same, saving a `clone` if it is
                        if !waker.will_wake(cx.waker()) {
                            occupied.insert(Status::Awaited(cx.waker().clone()));
                        }
                        Poll::Pending
                    }
                    Status::Happened => {
                        occupied.remove();
                        Poll::Ready(Ok(()))
                    }
                }
            }
        }
    }

    pub fn next_token(&self) -> Token {
        EventToken::next()
    }
}

fn run(mut poll: mio::Poll) -> ! {
    let reactor = Reactor::get();
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            let mut guard = reactor.statuses.lock().unwrap();

            let previous = guard.insert(event.token(), Status::Happened);

            if let Some(Status::Awaited(waker)) = previous {
                waker.wake();
            }
        }
    }
}
