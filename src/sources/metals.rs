use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::types::RateMap;

#[derive(Deserialize)]
struct MetalPriceResponse {
    rates: BTreeMap<String, f64>,
}

/// Fetches precious metal rates (XAU, XAG, XPT, XPD) from metalpriceapi.com.
///
/// Returns a `RateMap` where every value means "1 EUR = N troy ounces of that metal".
pub async fn fetch_metal_rates(api_url: &str, api_key: &str) -> Result<RateMap> {
    let client = reqwest::Client::builder()
        .user_agent("currency-api-rs/1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let resp = client
        .get(api_url)
        .header("X-API-KEY", api_key)
        .send()
        .await
        .context("Failed to send metals API request")?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Metals API returned HTTP {status}: {body}");
    }

    let body: MetalPriceResponse = resp
        .json()
        .await
        .context("Failed to parse metals API response as JSON")?;

    let rates: RateMap = body
        .rates
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();

    Ok(rates)
}
