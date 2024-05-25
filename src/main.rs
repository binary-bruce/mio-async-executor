use crate::runtime::new_executor_spawner;
use crate::example::{tick, tock, udp_client, udp_server};

mod delay;
mod example;
mod executor;
mod reactor;
mod spawner;
mod task;
mod udp_socket;
mod waker;
mod yield_now;
mod runtime;

fn main() {
    let (executor, spawner) = new_executor_spawner();
    spawner.spawn(tick());
    spawner.spawn(tock());
    spawner.spawn(udp_server());
    spawner.spawn(udp_client());

    // Drop this spawner, so that the `run` method can stop as soon as all other
    // spawners (stored within tasks) are dropped
    drop(spawner);

    println!("executor is running...");
    executor.run();
}
