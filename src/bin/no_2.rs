use chrono::NaiveDateTime;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.openweathermap.org/data/2.5/forecast";
    let api_key = "bd5e378503939ddaee76f12ad7a97608";
    let params = [
        ("appid", api_key),
        ("lon", "106.84513"),
        ("lat", "-6.21462"),
    ];
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let resp = reqwest::get(url).await?.error_for_status()?;
    let body: Value = resp.json().await?;

    println!("Weather Forecast:");
    if let Some(list) = body["list"].as_array() {
        for item in list {
            let dt_txt = item["dt_txt"].as_str().unwrap_or_default();
            if dt_txt.ends_with("18:00:00") {
                let dt = NaiveDateTime::parse_from_str(dt_txt, "%Y-%m-%d %H:%M:%S")?;
                let date = dt.format("%a, %d %b %Y");
                let temp_k = item["main"]["temp"].as_f64().unwrap_or_default();
                let temp_c = temp_k - 273.15;
                println!("{}: {:.2}°C", date, temp_c);
            }
        }
    }

    Ok(())
}
