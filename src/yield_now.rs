use crate::delay;

// use delay as implementation, but can be improved by not spawning a thread to sleep
pub async fn yield_now() {
    delay(1).await;
}
