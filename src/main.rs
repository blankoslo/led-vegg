use artnet_protocol::*;
use std::net::{ToSocketAddrs, UdpSocket};

mod renderer;

fn main() {
    use futures::executor::block_on;
    let mut wgpu_state = block_on(renderer::WGPUState::new(10, 10));

    let result = block_on(wgpu_state.render());
    let mut count = 0;
    for r in result {
        print!("{:?} ", r);

        if count % 10 == 0 {
            println!(" ");
        }
        count += 1;
    }
}
