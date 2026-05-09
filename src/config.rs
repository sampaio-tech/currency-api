/// Runtime configuration loaded from environment variables.
pub struct Config {
    /// Full URL for the EUR-based fiat rate endpoint.
    /// Default: https://open.er-api.com/v6/latest/EUR (1500 req/month free, no key)
    ///
    /// For exchangerate-api.com, embed the key via the `{key}` placeholder:
    ///   FIAT_API_URL=https://v6.exchangerate-api.com/v6/{key}/latest/EUR
    ///   FIAT_API_KEY=your_actual_key
    /// The placeholder will be substituted at startup; the key is NOT sent as a header.
    pub fiat_api_url: String,

    /// Optional Bearer token / API key for the fiat source.
    /// Set to None when the key was already embedded in `fiat_api_url` via `{key}`.
    pub fiat_api_key: Option<String>,

    /// Base URL for the crypto price endpoint (CoinGecko-compatible).
    /// Default: https://api.coingecko.com/api/v3/simple/price
    pub crypto_api_url: String,

    /// Optional API key sent as `x-cg-demo-api-key` header.
    pub crypto_api_key: Option<String>,

    /// URL for the precious metals endpoint (metalpriceapi.com-compatible).
    /// Default: https://api.metalpriceapi.com/v1/latest?base=EUR&currencies=XAU,XAG,XPT,XPD
    pub metals_api_url: String,

    /// API key sent as `X-API-KEY` header. If absent, metals are skipped.
    pub metals_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let raw_url = std::env::var("FIAT_API_URL")
            .unwrap_or_else(|_| "https://open.er-api.com/v6/latest/EUR".to_string());
        let raw_key = std::env::var("FIAT_API_KEY").ok();

        // If the URL contains `{key}`, substitute it and clear the key so it is
        // not also sent as a Bearer header (exchangerate-api.com uses key-in-URL).
        let (fiat_api_url, fiat_api_key) = match &raw_key {
            Some(key) if raw_url.contains("{key}") => {
                (raw_url.replace("{key}", key), None)
            }
            _ => (raw_url, raw_key),
        };

        Self {
            fiat_api_url,
            fiat_api_key,
            crypto_api_url: std::env::var("CRYPTO_API_URL").unwrap_or_else(|_| {
                "https://api.coingecko.com/api/v3/simple/price".to_string()
            }),
            crypto_api_key: std::env::var("CRYPTO_API_KEY").ok(),
            metals_api_url: std::env::var("METALS_API_URL").unwrap_or_else(|_| {
                "https://api.metalpriceapi.com/v1/latest?base=EUR&currencies=XAU,XAG,XPT,XPD"
                    .to_string()
            }),
            metals_api_key: std::env::var("METALS_API_KEY").ok(),
        }
    }
}
