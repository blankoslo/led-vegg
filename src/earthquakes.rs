use serde::{Deserialize};

#[derive(Deserialize, std::fmt::Debug, Copy, Clone)]
pub struct Properties {
    mag: f64,
    time: u64,
}

#[derive(Deserialize, std::fmt::Debug)]
struct Feature {
    properties: Properties,
}

#[derive(Deserialize, std::fmt::Debug)]
struct Response {
    features: Vec<Feature>,
}

const URL: &str = "https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&limit=1&starttime=2020-09-25T10:30:47.002Z";

#[tokio::main]
pub async fn get_earthquake() -> Result<Properties, Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?.json::<Response>().await?;
    //println!("{:?}", resp);

    Ok(resp.features[0].properties)
}