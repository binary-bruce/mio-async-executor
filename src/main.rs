use delay::delay;
use udp_socket::UdpSocket;
use yield_now::yield_now;

use crate::core::new_executor_spawner;

mod core;
mod delay;
mod executor;
mod reactor;
mod spawner;
mod task;
mod udp_socket;
mod waker;
mod yield_now;

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

async fn tick() {
    loop {
        println!("tick");
        yield_now().await;
    }
}

async fn tock() {
    loop {
        println!("tock");
        yield_now().await;
    }
}

async fn udp_server() {
    let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut buf = [0; 100];
    let (size, _) = socket.recv_from(&mut buf).await.unwrap();

    println!("server: received data from client - {}", unsafe {
        std::str::from_utf8_unchecked(&buf[0..size])
    });
}

async fn udp_client() {
    let socket = UdpSocket::bind("127.0.0.1:8001").unwrap();
    let data = "hello and goodbye".to_owned();
    let _ = socket
        .send_to(data.as_bytes(), "127.0.0.1:8080".parse().unwrap())
        .await;
    println!("client: sent data to server - {}", data);

    delay(1).await;
}
