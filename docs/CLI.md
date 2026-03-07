# CLI Reference

`fetcher` is the binary that drives the whole system. It fetches live exchange
rates from external APIs, computes cross rates for every supported currency, and
writes a tree of static JSON files ready to be deployed to a CDN.

---

## How the data flows

```
            ┌─────────────────────┐
            │   data/currencies   │  master list of supported currencies
            │   data/crypto_ids   │  maps code → CoinGecko ID
            └────────┬────────────┘
                     │
          ┌──────────▼──────────┐
          │   sources/fiat.rs   │  GET fiat API        → EUR-based fiat rates
          │   sources/crypto.rs │  GET CoinGecko       → EUR-based crypto rates
          └──────────┬──────────┘
                     │ merge
          ┌──────────▼──────────┐
          │    normalize.rs     │  computes cross rates for every currency pair
          └──────────┬──────────┘
                     │ write
          ┌──────────▼──────────┐
          │     generate.rs     │  writes dist/v1/ file tree
          └─────────────────────┘
```

Every rate in the system is first normalized to EUR as the base. To convert
between any two currencies A → B, the formula is:

```
rate(A → B) = eur_rate[B] / eur_rate[A]
```

This lets us derive all ~30,000 currency pairs from a single EUR-based map.

---

## Building

```bash
cargo build --release
```

The binary will be at `./target/release/fetcher`.

---

## Usage

```
fetcher [OPTIONS]

Options:
  --output <DIR>                   Output directory              [default: dist]
  --date <YYYY-MM-DD|YYYY-MM-DDTHH:MM>  Override snapshot datetime  [default: current UTC clock]
  --dry-run                        Fetch data but skip writing files
  -h, --help                       Print help
```

---

## Examples

### Normal daily run
```bash
cargo run --release
```
Fetches live rates and writes everything to `dist/v1/`.

### Dry run — test your API keys without writing files
```bash
cargo run -- --dry-run
```
Useful for verifying your environment variables are correct before deploying.

### Custom output directory
```bash
cargo run --release -- --output /tmp/my-output
```

### Backfill a specific date
```bash
cargo run --release -- --date 2026-01-15
```
Stamps each output file with `"date": "2026-01-15"`. No `timestamp` field is
included — output is identical to the old format. Useful for backfills.

### Sub-daily snapshot (date + timestamp)
```bash
cargo run --release -- --date 2026-03-07T14:30
```
Stamps output with both `"date": "2026-03-07"` and `"timestamp": "2026-03-07T14:30:00Z"`.
This is the format used by `sub-daily.yml` in GitHub Actions.

### No `--date` flag (default)
```bash
cargo run --release
```
Uses the current UTC clock. Produces both `date` and `timestamp` fields automatically.

### Enable debug logging
```bash
RUST_LOG=debug cargo run -- --dry-run
```

---

## Output file tree

After a successful run, `dist/` will look like this:

```
dist/
└── v1/
    ├── currencies.json          # full list: { "usd": "US Dollar", ... }
    ├── currencies.min.json      # same, minified
    ├── countries.json           # country data (copied from data/ if present)
    └── currencies/
        ├── eur.json             # { "date": "...", "timestamp": "...", "eur": { "usd": 1.08, ... } }
        ├── eur.min.json
        ├── usd.json
        ├── usd.min.json
        └── ...                  # one pair per supported currency
```

The entire `dist/` directory is what gets deployed to Cloudflare Pages.

---

## Environment variables

The CLI reads these at startup. Set them in your shell for local runs or as
GitHub secrets for CI (see [SETUP.md](./SETUP.md)).

| Variable         | Required | Default                                          | Description                                                  |
|------------------|----------|--------------------------------------------------|--------------------------------------------------------------|
| `FIAT_API_URL`   | No       | `https://open.er-api.com/v6/latest/EUR`          | EUR-based fiat endpoint. Compatible with open.er-api.com (`rates`) and exchangerate-api.com v6 (`conversion_rates`). Must use EUR as base. |
| `FIAT_API_KEY`   | No       | *(none)*                                         | Bearer token for fiat source                                 |
| `CRYPTO_API_URL` | No       | `https://api.coingecko.com/api/v3/simple/price`  | Crypto price endpoint                                        |
| `CRYPTO_API_KEY` | No       | *(none)*                                         | `x-cg-demo-api-key` header value                             |

### Local run with API keys

Copy `.env.example` to `.env`, fill in your keys, then:

```bash
source .env && cargo run --release
```

---

## Adding a new currency

1. Open `data/currencies.json` and add the entry:
   ```json
   "xyz": "My New Currency"
   ```
2. If it is a cryptocurrency, also open `data/crypto_ids.json` and map its
   CoinGecko ID:
   ```json
   "xyz": "my-new-currency-coingecko-id"
   ```
   You can find the CoinGecko ID at `https://www.coingecko.com` — it is the
   slug in the coin's URL, e.g. `coingecko.com/en/coins/bitcoin` → `bitcoin`.
3. Rebuild and run. The new currency will appear in the output automatically.

---

## Removing a currency

Delete its entry from `data/currencies.json` (and `data/crypto_ids.json` if
applicable). It will be excluded from the next run.
