use artnet_protocol::{ArtCommand, Output};
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

// The IP of the device running this SW
const DEVICE_IP: &str = "192.168.1.223";

const ART_NET_CONTROLLER_IP: &str = "192.168.1.122";

#[derive(Debug)]
pub struct ArtNet {
    socket: UdpSocket,
    broadcast_addr: SocketAddr,
}

impl ArtNet {
    pub fn new() -> Result<Self, Error> {
        let socket = UdpSocket::bind((DEVICE_IP, 6454))?;

        let broadcast_addr = (ART_NET_CONTROLLER_IP, 6454)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();
        socket.set_broadcast(true).unwrap();

        Ok(Self {
            socket: socket,
            broadcast_addr: broadcast_addr,
        })
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
