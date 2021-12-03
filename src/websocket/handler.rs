use crate::websocket::{ws, Result};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, Reply};
use spmc::Receiver;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, led_rx: Receiver<Vec<u8>>) -> Result<impl Reply> {
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, led_rx)))
}

