# currency-api

[![Sub-daily Rate Fetch & Deploy](https://github.com/sampaio-tech/currency-api/actions/workflows/sub-daily.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/sub-daily.yml) [![Daily Rate Fetch & Deploy](https://github.com/sampaio-tech/currency-api/actions/workflows/daily.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/daily.yml) [![Deploy Currency Flags](https://github.com/sampaio-tech/currency-api/actions/workflows/flags-deploy.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/flags-deploy.yml) [![Deploy Currency Names (i18n)](https://github.com/sampaio-tech/currency-api/actions/workflows/i18n-deploy.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/i18n-deploy.yml) [![Deploy Currency Symbols](https://github.com/sampaio-tech/currency-api/actions/workflows/symbols-deploy.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/symbols-deploy.yml)

A free, open-source currency exchange rate API — rebuilt in Rust.

Inspired by [fawazahmed0/exchange-api](https://github.com/fawazahmed0/exchange-api).

GitHub Actions fetches live exchange rates every hour, generates static JSON files,
and deploys them to Cloudflare Pages. No server. No database. No rate limits.

The goal of this project is to let anyone **fork or clone it and have their own currency API running in minutes** — just plug in your API keys, configure the GitHub secrets, and you're live.

See [docs/SETUP.md](./docs/SETUP.md) to get started.

---

## How it works

```
GitHub Actions (every 30 min — configurable)
    │
    ├── fetches fiat rates   (exchangerate-api.com v6 or open.er-api.com)
    ├── fetches crypto rates (CoinGecko)
    │
    ├── computes cross rates for every currency
    ├── writes static JSON files to dist/
    │
    ├── deploys to Cloudflare Pages → your-project.pages.dev          (production / latest)
    ├── deploys to Cloudflare Pages → 2026-03-07.your-project.pages.dev
    └── deploys to Cloudflare Pages → 2026-03-07t14-30.your-project.pages.dev
```

---

## API endpoints

Replace `{project}` with your Cloudflare Pages project name.

| Endpoint | URL |
|---|---|
| All currencies (latest) | `https://{project}.pages.dev/v1/currencies.json` |
| Rates for USD (latest) | `https://{project}.pages.dev/v1/currencies/usd.json` |
| Rates for EUR (latest) | `https://{project}.pages.dev/v1/currencies/eur.json` |
| Historical (date) | `https://2026-03-07.{project}.pages.dev/v1/currencies/usd.json` |
| Historical (timestamp) | `https://2026-03-07t14-30.{project}.pages.dev/v1/currencies/usd.json` |
| Currency symbols | `https://{project}.pages.dev/v1/currencies/symbols.json` |
| Currency names (locale) | `https://{project}.pages.dev/v1/currencies/names/{locale}.json` |
| Currency flag (SVG) | `https://{project}.pages.dev/v1/currencies/flags/{code}.svg` |

> The production branch (`latest`) is served at the **root domain** `{project}.pages.dev`.
> Date and timestamp snapshots get their own subdomain via Cloudflare Pages branch aliases.

Every exchange-rate endpoint also has a minified version: replace `.json` with `.min.json`.

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
  "date": "2026-03-07",
  "timestamp": "2026-03-07T14:30:00Z",
  "usd": {
    "eur": 0.92,
    "gbp": 0.79,
    "btc": 0.000011,
    "jpy": 149.2
  }
}
```

The `timestamp` field is present on sub-daily runs. Date-only runs (e.g. `--date 2026-03-07`) omit it for backward compatibility.

**`/v1/currencies/symbols.json`**
```json
{
  "BRL": "R$",
  "EUR": "€",
  "GBP": "£",
  "USD": "$"
}
```

See [docs/CURRENCY-SYMBOLS.md](./docs/CURRENCY-SYMBOLS.md) for the full endpoint reference and update instructions.

**`/v1/currencies/names/{locale}.json`**
```json
{
  "BRL": "Brazilian Real",
  "EUR": "Euro",
  "USD": "US Dollar"
}
```

566 locales available (e.g. `en`, `pt_BR`, `zh`, `ar`). See [docs/CURRENCY-NAMES.md](./docs/CURRENCY-NAMES.md) for the full reference.

**`/v1/currencies/flags/{code}.svg`** — returns an SVG image. 1214 files covering fiat currencies and crypto tokens (lowercase codes). See [docs/CURRENCY-FLAGS.md](./docs/CURRENCY-FLAGS.md) for the full reference.

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
│   ├── currencies.json           # master list: currency code → name
│   ├── crypto_ids.json           # maps currency code → CoinGecko ID
│   ├── currency-symbols/
│   │   └── currency_symbols.json # ISO 4217 code → display symbol (~170 currencies)
│   ├── currency-names/
│   │   └── {locale}.json         # localized currency names (566 locales)
│   └── currency-flags/
│       └── {code}.svg            # currency/crypto flag SVGs (1214 files)
├── docs/
│   ├── CLI.md                    # how the CLI works, all flags, data flow
│   ├── SETUP.md                  # step-by-step deployment guide
│   ├── CURRENCY-SYMBOLS.md       # symbols endpoint reference
│   ├── CURRENCY-NAMES.md         # i18n names endpoint reference
│   └── CURRENCY-FLAGS.md         # flag SVGs endpoint reference
├── .github/
│   └── workflows/
│       ├── daily.yml             # runs every day at 00:00 UTC (date snapshots)
│       ├── sub-daily.yml         # runs every 30 min (timestamp snapshots, configurable)
│       ├── symbols-deploy.yml    # manual: publishes currency symbols to CDN
│       ├── i18n-deploy.yml       # manual: publishes currency name translations to CDN
│       └── flags-deploy.yml      # manual: publishes currency flag SVGs to CDN
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

## Update frequency

The sub-daily workflow (`sub-daily.yml`) runs every hour by default. To change it, edit the `cron` expression in `.github/workflows/sub-daily.yml`:

```yaml
- cron: "0 * * * *"   # Every hour
```

**Free-tier budget** (both workflows combined, worst-case 31-day month):

| Expression | Runs/month | Fiat requests | Fits 1 500 limit? |
|---|---|---|---|
| `0 * * * *` | ~775 | ~775 | ✅ Yes *(default)* |
| `*/30 * * * *` | ~1 519 | ~1 519 | ❌ No (31-day months) |
| `0 */2 * * *` | ~403 | ~403 | ✅ Yes |
| `0 */6 * * *` | ~155 | ~155 | ✅ Yes |

> Each run makes one request to ExchangeRate-API (1 500/month free) and one to CoinGecko
> (10 000/month free). The fiat limit is the bottleneck — every-hour is the highest
> frequency that stays safely within the free tier.

The daily workflow (`daily.yml`) continues to run independently at 00:00 UTC and produces date-only snapshots (no `timestamp` field) for full backward compatibility.
