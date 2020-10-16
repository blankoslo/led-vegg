use artnet_protocol::{ArtCommand, Output};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

// The IP of the device running this SW
const DEVICE_IP: &str = "192.168.1.223";

const ART_NET_CONTROLLER_IP: &str = "192.168.1.122";

pub struct ArtNet {
    socket: UdpSocket,
    broadcast_addr: SocketAddr,
}

impl ArtNet {
    pub fn new() -> Self {
        let socket = UdpSocket::bind((DEVICE_IP, 6454)).unwrap();
        let broadcast_addr = (ART_NET_CONTROLLER_IP, 6454)
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
