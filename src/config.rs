/// Runtime configuration loaded from environment variables.
pub struct Config {
    /// Full URL for the EUR-based fiat rate endpoint.
    /// Default: https://open.er-api.com/v6/latest/EUR (1500 req/month free, no key)
    pub fiat_api_url: String,

    /// Optional Bearer token / API key for the fiat source.
    pub fiat_api_key: Option<String>,

    /// Base URL for the crypto price endpoint (CoinGecko-compatible).
    /// Default: https://api.coingecko.com/api/v3/simple/price
    pub crypto_api_url: String,

    /// Optional API key sent as `x-cg-demo-api-key` header.
    pub crypto_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            fiat_api_url: std::env::var("FIAT_API_URL")
                .unwrap_or_else(|_| "https://open.er-api.com/v6/latest/EUR".to_string()),
            fiat_api_key: std::env::var("FIAT_API_KEY").ok(),
            crypto_api_url: std::env::var("CRYPTO_API_URL").unwrap_or_else(|_| {
                "https://api.coingecko.com/api/v3/simple/price".to_string()
            }),
            crypto_api_key: std::env::var("CRYPTO_API_KEY").ok(),
        }
    }
}
