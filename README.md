# currency-api

A free, open-source currency exchange rate API — rebuilt in Rust.

Inspired by [fawazahmed0/exchange-api](https://github.com/fawazahmed0/exchange-api).

Every day a GitHub Action fetches live exchange rates, generates static JSON files,
and deploys them to Cloudflare Pages. No server. No database. No rate limits.

The goal of this project is to let anyone **fork or clone it and have their own currency API running in minutes** — just plug in your API keys, configure the GitHub secrets, and you're live.

See [docs/SETUP.md](./docs/SETUP.md) to get started.

---

## How it works

```
GitHub Actions (daily cron)
    │
    ├── fetches fiat rates   (exchangerate-api.com v6 or open.er-api.com)
    ├── fetches crypto rates (CoinGecko)
    │
    ├── computes cross rates for every currency
    ├── writes static JSON files to dist/
    │
    ├── deploys to Cloudflare Pages → latest.your-project.pages.dev
    └── deploys to Cloudflare Pages → 2026-03-06.your-project.pages.dev
```

---

## API endpoints

Replace `{project}` with your Cloudflare Pages project name.

| Endpoint | URL |
|---|---|
| All currencies | `https://latest.{project}.pages.dev/v1/currencies.json` |
| Rates for USD | `https://latest.{project}.pages.dev/v1/currencies/usd.json` |
| Rates for EUR | `https://latest.{project}.pages.dev/v1/currencies/eur.json` |
| Historical (date) | `https://2026-03-06.{project}.pages.dev/v1/currencies/usd.json` |

Every endpoint also has a minified version: replace `.json` with `.min.json`.

### Response format

**`/v1/currencies.json`**
```json
{
  "btc": "Bitcoin",
  "eur": "Euro",
  "usd": "US Dollar"
}
```

**`/v1/currencies/usd.json`**
```json
{
  "date": "2026-03-06",
  "usd": {
    "eur": 0.92,
    "gbp": 0.79,
    "btc": 0.000011,
    "jpy": 149.2
  }
}
```

---

## Project structure

```
currency-api/
├── src/
│   ├── main.rs          # CLI entry point (--output, --date, --dry-run)
│   ├── config.rs        # reads env vars for API keys
│   ├── types.rs         # shared types
│   ├── normalize.rs     # merges sources + computes cross rates
│   ├── generate.rs      # writes the dist/ file tree
│   └── sources/
│       ├── fiat.rs      # fetches fiat rates (exchangerate-api.com v6 / open.er-api.com)
│       └── crypto.rs    # fetches crypto rates (CoinGecko)
├── data/
│   ├── currencies.json  # master list: currency code → name
│   └── crypto_ids.json  # maps currency code → CoinGecko ID
├── docs/
│   ├── CLI.md           # how the CLI works, all flags, data flow
│   └── SETUP.md         # step-by-step deployment guide
├── .github/
│   └── workflows/
│       └── daily.yml    # runs every day at 00:00 UTC
└── Cargo.toml
```

---

## Running locally

```bash
# Copy the example env file and fill in your API keys
cp .env.example .env

# Load env vars and run
source .env && cargo run --release

# Preview what would be fetched without writing files
source .env && cargo run -- --dry-run
```

See [docs/CLI.md](./docs/CLI.md) for all flags and examples.

---

## Deployment

See [docs/SETUP.md](./docs/SETUP.md) for a full step-by-step guide on how to:
- Get your API keys
- Create a Cloudflare Pages project
- Configure GitHub secrets
- Trigger your first deployment

---

## Roadmap

### Sub-daily updates

Currently the API updates once per day. The goal is to support configurable update intervals — minute-by-minute, hour-by-hour, or at a fixed schedule — while keeping full backward compatibility with date-based URLs.

Planned URL scheme:

| Snapshot | URL |
|---|---|
| Latest | `https://latest.{project}.pages.dev/v1/currencies/usd.json` |
| By date | `https://2026-03-07.{project}.pages.dev/v1/currencies/usd.json` |
| By timestamp | `https://2026-03-07t14-30.{project}.pages.dev/v1/currencies/usd.json` |

The `--date` flag would be extended to accept an optional time component (`2026-03-07T14:30`) so each run stamps its output with the exact fetch time. The GitHub Actions workflow would be updated to support custom cron intervals (e.g. every 15 minutes, every hour) configured via a single variable.
