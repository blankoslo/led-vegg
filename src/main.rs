use std::sync::mpsc;
use std::thread;
use spmc::channel;

mod art_net;
mod earthquakes;
mod websocket;

mod renderer;

fn main() {
    let (earthquake_tx, earthquake_rx) = mpsc::channel();

    let (mut ws_tx, ws_rx) = channel::<Vec<u8>>();

    thread::spawn(move || {
        earthquakes::watch_earthquakes(earthquake_tx);
    });

    thread::spawn(move || {
        websocket::mainz(ws_rx);
    });

    use futures::executor::block_on;
    let mut wgpu_state = block_on(renderer::WGPUState::new(30, 30));
    loop {
        let earthquake = earthquake_rx.recv().unwrap();
        println!("Received earthquake: {}", earthquake);

        let result = block_on(wgpu_state.render());

        let data: Vec<u8> = result
            .into_iter()
            .map(|r| (r as f32 / 10. * earthquake) as u8)
            .collect();

        println!("{:?}", data);

        ws_tx.send(data).unwrap();

        /*match art_net_controller.as_ref() {
            Ok(anc) => anc.send_data(data),
            Err(err) => {
                println!("Error initializing art_net: {:?}", err);
                art_net_controller = art_net::ArtNet::new();
            }
        }*/
    }
}
