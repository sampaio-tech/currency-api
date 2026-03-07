use crate::types::RateMap;
use std::collections::BTreeMap;

/// Merges fiat and crypto EUR-based rates into one map.
/// Crypto values override fiat if there is a key collision.
pub fn merge_rates(fiat: RateMap, crypto: RateMap) -> RateMap {
    let mut merged = fiat;
    for (k, v) in crypto {
        merged.insert(k, v);
    }
    merged
}

/// Computes cross rates for every currency in `supported` from the EUR-based map.
///
/// Formula: rate(C → X) = eur_rates[X] / eur_rates[C]
///
/// Returns: code → { target_code → rate }
/// Only currencies present in both `eur_rates` and `supported` are included.
pub fn compute_cross_rates(
    eur_rates: &RateMap,
    supported: &[String],
) -> BTreeMap<String, RateMap> {
    let mut all: BTreeMap<String, RateMap> = BTreeMap::new();

    for base in supported {
        let Some(&base_rate) = eur_rates.get(base) else {
            continue;
        };
        if base_rate == 0.0 {
            continue;
        }

        let cross: RateMap = supported
            .iter()
            .filter_map(|target| {
                let &target_rate = eur_rates.get(target)?;
                Some((target.clone(), target_rate / base_rate))
            })
            .collect();

        all.insert(base.clone(), cross);
    }

    all
}
