use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::types::RateMap;

/// Supports both exchangerate-api.com v6 (`conversion_rates`) and
/// open.er-api.com (`rates`).
#[derive(Deserialize)]
struct ErApiResponse {
    #[serde(alias = "rates")]
    conversion_rates: BTreeMap<String, f64>,
}

/// Fetches EUR-based fiat exchange rates from the configured endpoint.
///
/// Returns a `RateMap` where every value means "1 EUR = N units of that currency".
/// EUR itself is inserted as 1.0 to ensure it is always present.
pub async fn fetch_fiat_rates(api_url: &str, api_key: Option<&str>) -> Result<RateMap> {
    let client = reqwest::Client::builder()
        .user_agent("currency-api-rs/1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let mut req = client.get(api_url);

    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {key}"));
    }

    let resp = req
        .send()
        .await
        .context("Failed to send fiat API request")?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Fiat API returned HTTP {status}: {body}");
    }

    let body: ErApiResponse = resp
        .json()
        .await
        .context("Failed to parse fiat API response as JSON")?;

    // Lowercase all keys and ensure EUR = 1.0 is always present
    let mut rates: RateMap = body
        .conversion_rates
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();

    rates.insert("eur".to_string(), 1.0);

    Ok(rates)
}
