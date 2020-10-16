use std::sync::mpsc;
use std::thread;

mod art_net;
mod earthquakes;

fn main() {
    let art_net = art_net::ArtNet::new();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        earthquakes::watch_earthquakes(tx);
    });

    loop {
        let received = rx.recv().unwrap();

        art_net.send_data(vec![(255. / 10. * received) as u8; 270])
    }
}
