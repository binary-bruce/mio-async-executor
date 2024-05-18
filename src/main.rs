use udp_socket::UdpSocket;

use crate::core::new_executor_spawner;

mod core;
mod delay;
mod executor;
mod reactor;
mod spawner;
mod task;
mod udp_socket;
mod waker;

fn main() {
    let (executor, spawner) = new_executor_spawner();
    spawner.spawn(async_delay());
    spawner.spawn(async_main());
    // Drop this spawner, so that the `run` method can stop as soon as all other
    // spawners (stored within tasks) are dropped
    drop(spawner);

    println!("executor is running...");
    executor.run();
}

async fn async_main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).await.unwrap();

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, src).await.unwrap();
}

async fn async_delay() {
    println!("delay 10 seconds..");
    delay::delay(10).await;
    println!("time to work..")
}
