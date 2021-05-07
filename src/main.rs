use std::sync::mpsc;
use std::thread;

mod art_net;
mod earthquakes;

mod renderer;

fn main() {
    let mut art_net_controller = art_net::ArtNet::new();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        earthquakes::watch_earthquakes(tx);
    });

    use futures::executor::block_on;
    let mut wgpu_state = block_on(renderer::WGPUState::new(90, 1));

    loop {
        let earthquake = rx.recv().unwrap();
        println!("Received earthquake: {}", earthquake);

        let result = block_on(wgpu_state.render());

        let data: Vec<u8> = result.into_iter().map(|r| (r as f32 / 10. * earthquake) as u8).collect();

        match art_net_controller.as_ref() {
            Ok(anc) => anc.send_data(data),
            Err(err) => {
                println!("Error initializing art_net: {:?}", err);
                art_net_controller = art_net::ArtNet::new();
            }
        }
    }
}
