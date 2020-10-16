use artnet_protocol::{ArtCommand, Output};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

pub struct ArtNet {
    socket: UdpSocket,
    broadcast_addr: SocketAddr,
}

impl ArtNet {
    pub fn new() -> Self {
        let socket = UdpSocket::bind(("2.0.0.1", 6454)).unwrap();
        let broadcast_addr = ("2.80.100.185", 6454)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        socket.set_broadcast(true).unwrap();

        Self {
            socket: socket,
            broadcast_addr: broadcast_addr,
        }
    }

    pub fn send_data(&self, data: Vec<u8>) {
        let command = ArtCommand::Output(Output {
            length: data.len() as u16,
            data,
            ..Output::default()
        });

        let bytes = command.into_buffer().unwrap();
        self.socket.send_to(&bytes, self.broadcast_addr).unwrap();
    }
}
