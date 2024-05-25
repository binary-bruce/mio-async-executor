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
    use std::sync::mpsc;

    use super::new_executor_spawner;

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
            let _ = sender.send("hello".to_owned());
        });

        spawner.spawn(async move {
            let received = receiver.recv().unwrap(); // yeah, it's blocking, should looking for some async altermative
            assert_eq!("hello", received);
        });

        drop(spawner);
        executor.run();
    }

    #[test]
    fn test_udp_client_and_server() {
        let (executor, spawner) = new_executor_spawner();
        spawner.spawn(udp_server("hello".to_owned()));
        spawner.spawn(udp_client("hello".to_owned()));

        drop(spawner);
        executor.run();
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
        let _ = socket
            .send_to(greeting.as_bytes(), "127.0.0.1:8888".parse().unwrap())
            .await;
    }
}
