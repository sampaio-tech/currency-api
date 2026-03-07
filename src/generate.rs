use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::Path;
use tokio::fs;
use tracing::info;

use crate::normalize::compute_cross_rates;
use crate::types::{CurrencyList, CurrencyOutput, RateMap};

/// Writes all output files into `{output_dir}/v1/`.
///
/// Layout produced:
/// ```
/// {output_dir}/v1/
///   currencies.json
///   currencies.min.json
///   countries.json          (if data/countries.json exists)
///   currencies/
///     {code}.json
///     {code}.min.json
///     ...
/// ```
pub async fn generate_all(
    output_dir: &Path,
    date: &str,
    eur_rates: &RateMap,
    currency_list: &CurrencyList,
) -> Result<()> {
    let v1_dir = output_dir.join("v1");
    let currencies_dir = v1_dir.join("currencies");

    fs::create_dir_all(&currencies_dir)
        .await
        .context("Failed to create output directories")?;

    // Only include currencies that are both in our master list and have rate data
    let supported: Vec<String> = currency_list
        .keys()
        .filter(|code| eur_rates.contains_key(*code))
        .cloned()
        .collect();

    info!("Generating files for {} currencies", supported.len());

    let cross_rates = compute_cross_rates(eur_rates, &supported);

    // currencies.json / currencies.min.json — the full code→name listing
    // Filter the list to only currencies we actually have rates for
    let available_list: CurrencyList = currency_list
        .iter()
        .filter(|(code, _)| cross_rates.contains_key(*code))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    write_json_pair(
        &v1_dir.join("currencies.json"),
        &v1_dir.join("currencies.min.json"),
        &available_list,
    )
    .await
    .context("Failed to write currencies index")?;

    // One file pair per currency
    for (base_code, rates) in &cross_rates {
        let output = CurrencyOutput {
            date: date.to_string(),
            rates: {
                let mut m = BTreeMap::new();
                m.insert(base_code.clone(), rates.clone());
                m
            },
        };

        write_json_pair(
            &currencies_dir.join(format!("{base_code}.json")),
            &currencies_dir.join(format!("{base_code}.min.json")),
            &output,
        )
        .await
        .with_context(|| format!("Failed to write {base_code}.json"))?;
    }

    // Pass-through countries.json if it exists
    let countries_src = Path::new("data/countries.json");
    if countries_src.exists() {
        let content = fs::read_to_string(countries_src)
            .await
            .context("Failed to read data/countries.json")?;
        fs::write(v1_dir.join("countries.json"), content)
            .await
            .context("Failed to write countries.json to output")?;
        info!("Copied countries.json to output");
    }

    info!(
        "Done — {} currency files in {}",
        cross_rates.len(),
        currencies_dir.display()
    );

    Ok(())
}

/// Writes both a pretty-printed `.json` and a minified `.min.json` for the same value.
async fn write_json_pair<T: serde::Serialize>(
    pretty_path: &Path,
    min_path: &Path,
    value: &T,
) -> Result<()> {
    let pretty = serde_json::to_string_pretty(value).context("Failed to serialize (pretty)")?;
    let min = serde_json::to_string(value).context("Failed to serialize (min)")?;
    fs::write(pretty_path, pretty).await?;
    fs::write(min_path, min).await?;
    Ok(())
}
