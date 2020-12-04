use std::sync::mpsc;
use std::thread;

mod art_net;
mod earthquakes;

mod renderer;

fn main() {
    let art_net = art_net::ArtNet::new();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        earthquakes::watch_earthquakes(tx);
    });

    use futures::executor::block_on;
    let mut wgpu_state = block_on(renderer::WGPUState::new(10, 10));

    loop {
        let received = rx.recv().unwrap();

        let result = block_on(wgpu_state.render());
        let mut count = 0;
        for r in result {
            print!("{:?} ", r);

            if count % 10 == 0 {
                println!(" ");
            }
            count += 1;
        }

        // TODO
        //art_net.send_data(vec![(255. / 10. * received) as u8; 270])
    }
}
