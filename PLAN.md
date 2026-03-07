# Currency Exchange API — Rust Implementation Plan

A Rust reimplementation of [fawazahmed0/exchange-api](https://github.com/fawazahmed0/exchange-api).

---

## What the Original Project Does

The original project is a **static-file currency exchange rate API** with no backend server. Every day, a script:

1. Fetches exchange rates from multiple upstream sources (fiat, crypto, metals)
2. Normalizes everything to a EUR base
3. Generates one JSON file per currency (e.g. `currencies/usd.json`, `currencies/eur.json`)
4. Generates a `currencies.json` listing all available currency codes and names
5. Publishes the output as a versioned npm package (version = today's date as semver `YYYY.M.D`)
6. Deploys to Cloudflare Pages (one branch per date + a `latest` branch)

The result is a zero-auth, no-rate-limit API served entirely from CDNs:
- **Primary**: `https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@{date}/v1/{endpoint}`
- **Fallback**: `https://{date}.currency-api.pages.dev/v1/{endpoint}`

---

## API Surface (what we must reproduce)

### Endpoint 1 — List all currencies
```
GET /v1/currencies.json
GET /v1/currencies.min.json
```
Response:
```json
{
  "btc": "Bitcoin",
  "eur": "Euro",
  "usd": "US Dollar"
}
```

### Endpoint 2 — Exchange rates for a base currency
```
GET /v1/currencies/{code}.json
GET /v1/currencies/{code}.min.json
```
Response:
```json
{
  "date": "2026-03-06",
  "eur": {
    "usd": 1.16195804,
    "gbp": 0.86914352,
    "btc": 0.000016301522
  }
}
```

All keys are **lowercase**. All keys are **alphabetically sorted**. The base currency appears as the top-level key of the rates object.

---

## Rust Project Structure

```
currency-api/
├── Cargo.toml                   # Workspace root
├── Cargo.lock
├── PLAN.md                      # This file
├── README.md
├── .github/
│   └── workflows/
│       └── daily.yml            # GitHub Actions: fetch rates + deploy
├── data/
│   ├── currencies.json          # Master list: code → name (committed to repo)
│   └── countries.json           # Country data (committed to repo)
└── crates/
    └── fetcher/                 # Main binary crate
        ├── Cargo.toml
        └── src/
            ├── main.rs          # CLI entry point
            ├── sources/
            │   ├── mod.rs
            │   ├── fiat.rs      # Fetch fiat rates (EUR-based)
            │   ├── crypto.rs    # Fetch crypto rates (USD-based, convert to EUR)
            │   └── metals.rs    # Fetch precious metals (optional, via fiat source)
            ├── normalize.rs     # Merge all sources → single EUR-based HashMap
            ├── generate.rs      # Write output JSON files per currency
            └── types.rs         # Shared structs: CurrencyMap, RateMap, OutputFile
```

---

## Crate Dependencies (`fetcher/Cargo.toml`)

```toml
[dependencies]
reqwest  = { version = "0.12", features = ["json", "rustls-tls"] }
tokio    = { version = "1", features = ["full"] }
serde    = { version = "1", features = ["derive"] }
serde_json = "1"
chrono   = { version = "0.4", features = ["serde"] }
anyhow   = "1"
tracing  = "1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

No Playwright. No browser automation. All HTTP calls use `reqwest` async.

---

## Data Sources (to configure via environment variables)

| Variable          | Description                                         |
|-------------------|-----------------------------------------------------|
| `FIAT_API_URL`    | EUR-based fiat rates JSON endpoint                  |
| `CRYPTO_API_URL`  | USD-based crypto prices JSON endpoint               |
| `FIAT_API_KEY`    | Optional API key for fiat source (header/query)     |
| `CRYPTO_API_KEY`  | Optional API key for crypto source                  |

Sources must be configured as GitHub Actions secrets. The fetcher reads them from the environment at runtime.

### Recommended free sources (no key required):
- **Fiat**: `https://open.er-api.com/v6/latest/EUR` (1500 req/month free)
- **Crypto**: `https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,...&vs_currencies=eur`

---

## Core Implementation Steps

### Step 1 — Scaffold the Cargo workspace
```bash
cargo new --name currency-api .        # workspace root (no src/)
mkdir -p crates/fetcher
cargo new --bin crates/fetcher
```
Edit root `Cargo.toml` to declare a workspace with `members = ["crates/fetcher"]`.

### Step 2 — Define types (`types.rs`)
```rust
// Map of currency code -> human-readable name
pub type CurrencyList = std::collections::BTreeMap<String, String>;

// Map of currency code -> rate relative to EUR
pub type RateMap = std::collections::BTreeMap<String, f64>;

// The output file for a single base currency
#[derive(serde::Serialize)]
pub struct CurrencyOutput {
    pub date: String,
    #[serde(flatten)]
    pub rates: std::collections::BTreeMap<String, RateMap>,
}
```

Using `BTreeMap` (not `HashMap`) guarantees alphabetical key ordering in the serialized JSON output.

### Step 3 — Fetch fiat rates (`sources/fiat.rs`)
- `GET $FIAT_API_URL` → parse EUR-based rates
- Return `RateMap` with EUR as implicit base (all values are EUR → X rates)

### Step 4 — Fetch crypto rates (`sources/crypto.rs`)
- `GET $CRYPTO_API_URL` → parse USD-denominated prices
- Divide each USD price by the EUR/USD rate to convert to EUR base
- Merge into the same `RateMap`

### Step 5 — Normalize (`normalize.rs`)
- Merge fiat + crypto into one `RateMap` keyed by lowercase currency code
- For each currency C in the map, compute cross rates:
  - rate(C → X) = rate(EUR → X) / rate(EUR → C)
- This gives every currency its own complete `RateMap`

### Step 6 — Generate output files (`generate.rs`)
```
dist/
└── v1/
    ├── currencies.json
    ├── currencies.min.json
    └── currencies/
        ├── eur.json
        ├── eur.min.json
        ├── usd.json
        ├── usd.min.json
        └── ...
```
- `.json` = `serde_json::to_string_pretty`
- `.min.json` = `serde_json::to_string`
- Date field = today's date as `YYYY-MM-DD` (using `chrono::Local::now()`)
- Also copy `data/countries.json` into `dist/v1/`

### Step 7 — CLI entry point (`main.rs`)
```bash
# Usage
cargo run --release -- --output dist/
```
Flags:
- `--output <dir>` — output directory (default: `dist/`)
- `--date <YYYY-MM-DD>` — override date (default: today)
- `--dry-run` — fetch data but skip writing files

---

## Output Directory Layout (what gets deployed)

```
dist/
└── v1/
    ├── currencies.json          # Full list of all currency codes+names
    ├── currencies.min.json      # Same, minified
    ├── countries.json           # Country data passthrough
    └── currencies/
        ├── btc.json
        ├── btc.min.json
        ├── eur.json
        ├── eur.min.json
        ├── usd.json
        ├── usd.min.json
        └── ... (one file per supported currency)
```

This entire `dist/` directory is deployed to Cloudflare Pages.

---

## GitHub Actions Workflow (`.github/workflows/daily.yml`)

```yaml
name: Daily Rate Fetch & Deploy

on:
  schedule:
    - cron: '0 0 * * *'     # Every day at 00:00 UTC
  workflow_dispatch:          # Allow manual trigger

env:
  RUST_LOG: info

jobs:
  fetch-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repo
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build fetcher binary
        run: cargo build --release --bin fetcher

      - name: Fetch exchange rates and generate files
        env:
          FIAT_API_URL: ${{ secrets.FIAT_API_URL }}
          FIAT_API_KEY: ${{ secrets.FIAT_API_KEY }}
          CRYPTO_API_URL: ${{ secrets.CRYPTO_API_URL }}
          CRYPTO_API_KEY: ${{ secrets.CRYPTO_API_KEY }}
        run: ./target/release/fetcher --output dist/

      - name: Deploy to Cloudflare Pages (latest)
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CF_API_TOKEN }}
          accountId: ${{ secrets.CF_ACCOUNT_ID }}
          command: pages deploy dist/ --project-name=currency-api --branch=latest

      - name: Deploy to Cloudflare Pages (date branch)
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CF_API_TOKEN }}
          accountId: ${{ secrets.CF_ACCOUNT_ID }}
          command: pages deploy dist/ --project-name=currency-api --branch=${{ steps.date.outputs.date }}
```

Add a step before the deploy steps to capture today's date:
```yaml
      - name: Get today's date
        id: date
        run: echo "date=$(date +'%Y-%m-%d')" >> $GITHUB_OUTPUT
```

---

## Cloudflare Pages Setup

### One-time setup (manual, via Cloudflare dashboard or CLI):

1. **Create the Pages project**
   ```bash
   npx wrangler pages project create currency-api
   ```
   This creates the project `currency-api` under your Cloudflare account.

2. **Branch subdomain behavior** — Cloudflare Pages automatically creates subdomains for each branch:
   - `latest` branch → `latest.currency-api.pages.dev`
   - `2026-03-06` branch → `2026-03-06.currency-api.pages.dev`

3. **Required GitHub secrets**:

   | Secret Name      | Where to get it                                                      |
   |------------------|----------------------------------------------------------------------|
   | `CF_API_TOKEN`   | Cloudflare dashboard → My Profile → API Tokens → Create Token (use "Edit Cloudflare Workers" template, scope to Pages) |
   | `CF_ACCOUNT_ID`  | Cloudflare dashboard → right sidebar on any page shows Account ID    |
   | `FIAT_API_URL`   | URL of the fiat rate provider you choose                             |
   | `FIAT_API_KEY`   | API key for fiat provider (if required)                              |
   | `CRYPTO_API_URL` | URL of the crypto rate provider you choose                           |
   | `CRYPTO_API_KEY` | API key for crypto provider (if required)                            |

4. **Custom domain** (optional): In Pages dashboard → Custom Domains → add your own domain.

---

## Resulting API URLs

After deployment, the API will be available at:

```
# Latest rates
https://latest.currency-api.pages.dev/v1/currencies.json
https://latest.currency-api.pages.dev/v1/currencies/usd.json

# Historical rates (date-specific branch)
https://2026-03-06.currency-api.pages.dev/v1/currencies/eur.json
```

---

## Implementation Order (recommended)

1. `cargo new` workspace + fetcher crate
2. `types.rs` — define `BTreeMap`-based structs
3. `sources/fiat.rs` — fetch and parse fiat rates
4. `sources/crypto.rs` — fetch and parse crypto rates
5. `normalize.rs` — merge + compute cross rates for all currencies
6. `generate.rs` — write all output files to `dist/`
7. `main.rs` — wire everything up with CLI args
8. Local test: `cargo run -- --output dist/` and inspect `dist/v1/`
9. Commit `data/currencies.json` (master currency list with names)
10. Add `.github/workflows/daily.yml`
11. Set up Cloudflare Pages project (one-time)
12. Add all secrets to GitHub repository settings
13. Trigger workflow manually to verify first deployment

---

## Key Design Decisions vs. Original

| Aspect               | Original (JS)                     | This implementation (Rust)            |
|----------------------|-----------------------------------|---------------------------------------|
| Language             | Node.js + Playwright              | Rust (async, no browser)              |
| Browser automation   | Playwright (Firefox) for one source | Not needed — use REST APIs directly |
| JSON key ordering    | Manual `.sort()`                  | `BTreeMap` — sorted automatically     |
| Precision            | BigDecimal (100 decimal places)   | `f64` (sufficient for display)        |
| Build artifact       | npm package                       | Compiled binary (fast, zero deps)     |
| CDN primary          | jsDelivr (via npm publish)        | Cloudflare Pages only                 |
| CDN fallback         | Cloudflare Pages                  | Can add jsDelivr later via npm        |

---

## Notes on Precision

The original uses up to 100 decimal places for very small values (crypto rates like BTC→satoshi).
For this implementation, `f64` is used which gives ~15–17 significant digits — sufficient for all practical rate display. If higher precision is needed later, the `rust_decimal` or `bigdecimal` crates can be swapped in.
