use crate::runtime::futures::delay::delay;
use crate::runtime::futures::udp_socket2::UdpSocket;
use crate::runtime::futures::yield_now::yield_now;

const N: u8 = 3;

pub async fn tick() {
    for num in 0..N {
        println!("tick - {num}");
        yield_now().await;
    }
}

pub async fn tock() {
    for num in 0..N {
        println!("tock - {num}");
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
