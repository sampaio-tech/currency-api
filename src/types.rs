use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Map of currency code (lowercase) → human-readable name.
pub type CurrencyList = BTreeMap<String, String>;

/// Map of target currency code → rate relative to a base currency.
/// BTreeMap ensures alphabetical key ordering in serialized JSON.
pub type RateMap = BTreeMap<String, f64>;

/// The JSON output structure written to `currencies/{code}.json`.
#[derive(Serialize, Deserialize)]
pub struct CurrencyOutput {
    pub date: String,
    /// The key is the base currency code, value is all cross rates from it.
    #[serde(flatten)]
    pub rates: BTreeMap<String, RateMap>,
}
