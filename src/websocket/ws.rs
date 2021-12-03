use futures::StreamExt;
use serde::Deserialize;
use serde_json::to_string;
use futures::SinkExt;
use spmc::Receiver;
use warp::ws::{Message, WebSocket};

use serde::{Serialize};

#[derive(Serialize)]
pub struct PixelData {
    data: Vec<u8>
}

#[derive(Deserialize, Debug)]
pub struct TopicsRequest {
    topics: Vec<String>,
}

pub async fn client_connection(ws: WebSocket, led_rx: Receiver<Vec<u8>>) {
    let (mut client_ws_sender, _) = ws.split();

    println!("connected");

    loop {
        let led_data = led_rx.recv().unwrap();
        println!("sending data");
        let string_data = to_string(&led_data).unwrap();
        let _ = client_ws_sender.send(Message::text(string_data)).await;
        println!("done sending");
    }

    println!("disconnected");
}
