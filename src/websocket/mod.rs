use std::convert::Infallible;
use tokio::sync::mpsc;
use warp::{ws::Message, Filter, Rejection};
use spmc::Receiver;

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
pub async fn mainz(led_rx: Receiver<Vec<u8>>) {
    let health_route = warp::path!("health").and_then(handler::health_handler);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_reciever(led_rx.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(ws_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_reciever(led_rx: Receiver<Vec<u8>>) -> impl Filter<Extract = (Receiver<Vec<u8>>,), Error = Infallible> + Clone {
    warp::any().map(move || led_rx.clone())
}
