mod config;
mod generate;
mod normalize;
mod sources;
mod types;

use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(
    name = "fetcher",
    about = "Fetches daily currency exchange rates and generates static JSON files",
    long_about = "Fetches EUR-based fiat and crypto rates, computes cross rates for every \
                  supported currency, and writes a static file tree ready for CDN deployment."
)]
struct Cli {
    /// Output directory where dist/v1/ will be written
    #[arg(long, default_value = "dist")]
    output: PathBuf,

    /// Override today's date (format: YYYY-MM-DD). Defaults to the system date.
    #[arg(long)]
    date: Option<String>,

    /// Fetch and parse data but skip writing any files
    #[arg(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("currency_api=info".parse()?),
        )
        .init();

    let cli = Cli::parse();
    let cfg = config::Config::from_env();

    let date = cli
        .date
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    info!("Date: {date}");
    info!("Output: {}", cli.output.display());

    // Load master currency list (baked into the binary at compile time)
    let currency_list: types::CurrencyList =
        serde_json::from_str(include_str!("../data/currencies.json"))
            .expect("data/currencies.json must be valid JSON");

    // Load CoinGecko ID mapping (baked into the binary at compile time)
    let crypto_ids: BTreeMap<String, String> =
        serde_json::from_str(include_str!("../data/crypto_ids.json"))
            .expect("data/crypto_ids.json must be valid JSON");

    info!("Master list: {} currencies", currency_list.len());

    // --- Fetch ---
    info!("Fetching fiat rates from {}", cfg.fiat_api_url);
    let fiat_rates =
        sources::fiat::fetch_fiat_rates(&cfg.fiat_api_url, cfg.fiat_api_key.as_deref()).await?;
    info!("  -> {} fiat rates", fiat_rates.len());

    info!("Fetching crypto rates from {}", cfg.crypto_api_url);
    let crypto_rates = sources::crypto::fetch_crypto_rates(
        &cfg.crypto_api_url,
        cfg.crypto_api_key.as_deref(),
        &crypto_ids,
    )
    .await?;
    info!("  -> {} crypto rates", crypto_rates.len());

    // --- Merge ---
    let eur_rates = normalize::merge_rates(fiat_rates, crypto_rates);
    info!("Total EUR-based rates: {}", eur_rates.len());

    if cli.dry_run {
        info!("Dry-run mode -- skipping file generation");
        return Ok(());
    }

    // --- Generate ---
    generate::generate_all(&cli.output, &date, &eur_rates, &currency_list).await?;

    Ok(())
}
