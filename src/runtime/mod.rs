use std::sync::mpsc;

use self::core::{executor::Executor, spawner::Spawner};

pub mod core;
pub mod futures;

pub fn new_executor_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;

    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    use super::{core::executor::Executor, futures::delay::delay, new_executor_spawner};

    static GREETING: &str = "hello";

    #[test]
    fn the_answer_is_42() {
        let (executor, spawner) = new_executor_spawner();
        spawner.spawn(async {
            assert_eq!(42, 42);
        });
        drop(spawner);
        executor.run();
    }

    #[test]
    fn test_async_communicate_by_channel() {
        let (executor, spawner) = new_executor_spawner();

        // though channel is not for async tasks, this is only for testing purpose
        let (sender, receiver) = mpsc::channel::<String>();
        spawner.spawn(async move {
            let _ = sender.send(GREETING.to_owned());
        });

        spawner.spawn(async move {
            // yeah, it's blocking, should looking for some async alternative
            let received = receiver.recv().unwrap();
            assert_eq!(GREETING, received);
        });

        drop(spawner);
        executor.run();
    }

    #[test]
    fn test_udp_client_and_server() {
        let (executor, spawner) = new_executor_spawner();
        spawner.spawn(udp_server(GREETING.to_owned()));
        spawner.spawn(udp_client(GREETING.to_owned()));

        drop(spawner);
        executor.run();
    }

    // run `cargo test -- --test-threads 1`
    #[test]
    fn block_the_udp_server() {
        let (executor, spawner) = new_executor_spawner();
        spawner.spawn(async {
            delay(1).await; // wait the blocking server to start up
            udp_client(GREETING.to_owned()).await;
        });

        drop(spawner);
        // run the execturo in another thread so that below code can run
        let handle = thread::spawn(move || executor.run());

        Executor::block_on(udp_server(GREETING.to_owned()));
        let _ = handle.join();
    }

    pub async fn udp_server(expected: String) {
        use super::futures::udp_socket2::UdpSocket;
        let socket = UdpSocket::bind("127.0.0.1:8888").unwrap();

        let mut buf = [0; 100];
        let (size, _) = socket.recv_from(&mut buf).await.unwrap();

        let received = unsafe { std::str::from_utf8_unchecked(&buf[0..size]) };
        assert_eq!(expected, received);
    }

    async fn udp_client(greeting: String) {
        use super::futures::udp_socket::UdpSocket;

        let socket = UdpSocket::bind("127.0.0.1:9999").unwrap();
        let server_addr = "127.0.0.1:8888".parse().unwrap();
        let _ = socket.send_to(greeting.as_bytes(), server_addr).await;
    }
}
