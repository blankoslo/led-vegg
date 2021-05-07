use chrono::prelude::NaiveDateTime;
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;
use std::sync::mpsc::Sender;
use std::{thread, time};

#[derive(Deserialize, std::fmt::Debug, Copy, Clone)]
pub struct Properties {
    mag: f32,
    time: i64,
}

#[derive(Deserialize, std::fmt::Debug, Copy, Clone)]
struct Feature {
    properties: Properties,
}

#[derive(Deserialize, std::fmt::Debug, Clone)]
struct Response {
    features: Vec<Feature>,
}

const BASE_URL: &str =
    "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&limit=1&starttime=";
const POLLING_TIME: time::Duration = time::Duration::from_secs(10);

#[tokio::main]
async fn get_earthquake(time: NaiveDateTime) -> Result<Vec<Feature>, Box<dyn Error + Send + Sync>> {
    let url = format!("{}{}", BASE_URL, time.format("%Y-%m-%dT%H:%M:%S"));
    let resp = reqwest::get(&url).await?.json::<Response>().await?;

    Ok(resp.features)
}

pub fn watch_earthquakes(tx: Sender<f32>) -> () {
    let mut last_earthquake = Utc::now().naive_utc();
    //let mut last_earthquake = NaiveDateTime::from_timestamp(1602851133, 0);

    loop {
        let new_earthquakes = match get_earthquake(last_earthquake) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to earthquake: {}", e);
                vec![]
            }
        };

        if new_earthquakes.len() > 0 {
            let timestamp = (new_earthquakes[0].clone().properties.time / 1000) + 1 as i64;
            last_earthquake = NaiveDateTime::from_timestamp(timestamp, 0);

            tx.send(new_earthquakes[0].properties.mag).unwrap();
        }

        thread::sleep(POLLING_TIME);
    }
}
