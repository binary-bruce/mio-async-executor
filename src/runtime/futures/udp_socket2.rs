use std::{
    future::Future,
    io::{Error, ErrorKind},
    net::{SocketAddr, ToSocketAddrs},
    pin::Pin,
    task::{Context, Poll},
};

use mio::{Interest, Token};

use crate::runtime::core::reactor::Reactor;

struct UdpSocketReadiness {
    token: Token,
}

impl UdpSocketReadiness {
    fn new(token: Token) -> Self {
        Self { token }
    }
}

impl Future for UdpSocketReadiness {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("polling UdpSocketReadiness");
        Reactor::get().poll(self.token, cx) // register waker
    }
}

pub struct UdpSocket {
    socket: mio::net::UdpSocket,
    token: Token,
}

impl UdpSocket {
    pub fn bind(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let std_socket = std::net::UdpSocket::bind(addr)?;
        std_socket.set_nonblocking(true)?;

        let mut socket = mio::net::UdpSocket::from_std(std_socket);

        let reactor = Reactor::get();
        let token = reactor.unique_token();

        Reactor::get().registry.register(
            &mut socket,
            token,
            Interest::READABLE | Interest::WRITABLE,
        )?;

        Ok(self::UdpSocket { socket, token })
    }

    pub async fn send_to(&self, buf: &[u8], dest: SocketAddr) -> std::io::Result<usize> {
        loop {
            println!("running send_to");
            match self.socket.send_to(buf, dest) {
                Ok(value) => return Ok(value),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    println!("send_to - WouldBlock");
                    //std::future::poll_fn(|cx| Reactor::get().poll(self.token, cx)).await?
                    UdpSocketReadiness::new(self.token).await?
                }
                Err(error) => return Err(error),
            }
        }
    }

    pub async fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        loop {
            println!("running recv_from");
            match self.socket.recv_from(buf) {
                Ok(value) => return Ok(value),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    println!("recv_from - WouldBlock");
                    //std::future::poll_fn(|cx| Reactor::get().poll(self.token, cx)).await?
                    UdpSocketReadiness::new(self.token).await?
                }
                Err(error) => return Err(error),
            }
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        let _ = Reactor::get().registry.deregister(&mut self.socket);
    }
}
