use mio::Token;

pub(crate) struct EventToken;

impl EventToken {
    pub(crate) fn next() -> Token {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static CURRENT_TOKEN: AtomicUsize = AtomicUsize::new(0);

        Token(CURRENT_TOKEN.fetch_add(1, Ordering::Relaxed))
    }
}
