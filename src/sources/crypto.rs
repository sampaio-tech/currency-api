use anyhow::{Context, Result};
use std::collections::BTreeMap;

use crate::types::RateMap;

/// Fetches crypto prices in EUR from a CoinGecko-compatible `/simple/price` endpoint.
///
/// `crypto_ids`: maps our lowercase currency code (e.g. `"btc"`) to the CoinGecko
/// coin ID (e.g. `"bitcoin"`). This mapping lives in `data/crypto_ids.json`.
///
/// Returns a `RateMap` where every value means "1 EUR = N units of that crypto".
pub async fn fetch_crypto_rates(
    api_url: &str,
    api_key: Option<&str>,
    crypto_ids: &BTreeMap<String, String>,
) -> Result<RateMap> {
    if crypto_ids.is_empty() {
        return Ok(RateMap::new());
    }

    let ids_param = crypto_ids.values().cloned().collect::<Vec<_>>().join(",");

    let client = reqwest::Client::builder()
        .user_agent("currency-api-rs/1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let mut req = client
        .get(api_url)
        .query(&[("ids", &ids_param), ("vs_currencies", &"eur".to_string())]);

    if let Some(key) = api_key {
        req = req.header("x-cg-demo-api-key", key);
    }

    let resp = req
        .send()
        .await
        .context("Failed to send crypto API request")?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Crypto API returned HTTP {status}: {body}");
    }

    // Response shape: { "bitcoin": { "eur": 45000.0 }, "ethereum": { "eur": 2500.0 }, ... }
    let body: BTreeMap<String, BTreeMap<String, f64>> = resp
        .json()
        .await
        .context("Failed to parse crypto API response as JSON")?;

    // Reverse map: coingecko_id → our currency code
    let id_to_code: BTreeMap<&str, &str> = crypto_ids
        .iter()
        .map(|(code, id)| (id.as_str(), code.as_str()))
        .collect();

    let mut rates = RateMap::new();

    for (coin_id, prices) in &body {
        let Some(code) = id_to_code.get(coin_id.as_str()) else {
            continue;
        };
        let Some(&eur_price) = prices.get("eur") else {
            continue;
        };
        if eur_price > 0.0 {
            // eur_price = "1 coin costs N EUR"  →  "1 EUR = 1/N coins"
            rates.insert(code.to_string(), 1.0 / eur_price);
        }
    }

    Ok(rates)
}
