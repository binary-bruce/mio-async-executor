use mio::Token;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) struct EventToken;

impl EventToken {
    pub(crate) fn next() -> Token {
        static CURRENT_TOKEN: AtomicUsize = AtomicUsize::new(0);

        let value = CURRENT_TOKEN.fetch_add(1, Ordering::Relaxed);
        Token(value)
    }
}
