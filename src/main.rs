use futures::executor::block_on;

mod earthquakes;

fn main() {
    let properties = block_on(earthquakes::get_earthquake());
}