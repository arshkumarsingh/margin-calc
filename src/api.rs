use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct OptionMarginResponse(pub bool, pub HashMap<String, OptionMarginData>);

#[derive(Deserialize, Debug)]
pub struct OptionMarginData {
    pub initial: f64,
    pub maintenance: f64,
    pub total: f64,
}

#[derive(Deserialize, Debug)]
pub struct OptionSymbol {
    pub instrument_token: i64,
    pub tradingsymbol: String,
}

pub async fn get_option_margin(api_key: &str, access_token: &str, tradingsymbol: &str, exchange: &str, segment: &str) -> Result<OptionMarginResponse, reqwest::Error> {
    let client = Client::new();
    let url = "https://api.kite.trade/margins/orders";

    let response = client
        .get(url)
        .query(&[
            ("tradingsymbol", tradingsymbol),
            ("exchange", exchange),
            ("segment", segment),
        ])
        .header("X-Kite-Version", "3")
        .header("Authorization", format!("token {}:{}", api_key, access_token))
        .send()
        .await?;

    let option_margin: OptionMarginResponse = response.json().await?;
    Ok(option_margin)
}

pub async fn get_option_symbols(api_key: &str, access_token: &str) -> Result<Vec<OptionSymbol>, reqwest::Error> {
    let client = Client::new();
    let url = "https://api.kite.trade/instruments";

    let response = client
        .get(url)
        .header("X-Kite-Version", "3")
        .header("Authorization", format!("token {}:{}", api_key, access_token))
        .send()
        .await?;

    let symbols: Vec<OptionSymbol> = response.json().await?;
    Ok(symbols)
}
