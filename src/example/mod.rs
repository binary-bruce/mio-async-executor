use crate::delay::delay;
use crate::udp_socket::UdpSocket;
use crate::yield_now::yield_now;

pub async fn tick() {
    loop {
        println!("tick");
        yield_now().await;
    }
}

pub async fn tock() {
    loop {
        println!("tock");
        yield_now().await;
    }
}

pub async fn udp_server() {
    let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut buf = [0; 100];
    let (size, _) = socket.recv_from(&mut buf).await.unwrap();

    println!("server: received data from client - {}", unsafe {
        std::str::from_utf8_unchecked(&buf[0..size])
    });
}

pub async fn udp_client() {
    let socket = UdpSocket::bind("127.0.0.1:8001").unwrap();
    let data = "hello and goodbye".to_owned();
    let _ = socket
        .send_to(data.as_bytes(), "127.0.0.1:8080".parse().unwrap())
        .await;
    println!("client: sent data to server - {}", data);

    delay(1).await;
}
