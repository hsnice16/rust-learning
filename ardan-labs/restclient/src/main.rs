// #[derive(Deserialize, Debug)]
// struct CurrentWeather {
//     temperature: f64,
//     windspeed: f64,
// }

// #[derive(Deserialize, Debug)]
// struct Weather {
//     latitude: f64,
//     longitude: f64,
//     current_weather: CurrentWeather,
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast";
    let response = reqwest::get(URL).await?;
    // let weather: serde_json::Value = response.text().await?;
    let weather = response.text().await?;

    println!("{weather:#?}");

    Ok(())
}
