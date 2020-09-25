use artnet_protocol::*;
use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    let socket = UdpSocket::bind(("2.0.0.1", 6454)).unwrap();
    let broadcast_addr = ("2.80.100.185", 6454)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    socket.set_broadcast(true).unwrap();
    //let buff = ArtCommand::Poll(Poll::default()).into_buffer().unwrap();
    //socket.send_to(&buff, &broadcast_addr).unwrap();

    //let mut buffer = [0u8; 1024];
    //let (length, addr) = socket.recv_from(&mut buffer).unwrap();
    //let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

    //println!("Received {:?}", command);
    // This is an ArtNet node on the network. We can send commands to it like this:
    let command = ArtCommand::Output(Output {
        length: 6,                 // must match your data.len()
        data: vec![255, 255, 0, 0, 255, 0], // The data we're sending to the node
        ..Output::default()
    });
    //let bytes = command.to_bytes().unwrap();
    let bytes = command.into_buffer().unwrap();
    socket.send_to(&bytes, broadcast_addr).unwrap();
}
