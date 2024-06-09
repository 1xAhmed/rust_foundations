use anyhow::Ok;
use serde::Deserialize;

#[derive(Deserialize, Debug)]

struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "";
    let response = reqwest::get(URL).await?;
    let weather: serde_json::Value = response.json().await?;
    println!("{weather:#?}");

    Ok(())
}
