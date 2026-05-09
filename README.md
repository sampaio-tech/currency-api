# currency-api

[![Sub-daily Rate Fetch & Deploy](https://github.com/sampaio-tech/currency-api/actions/workflows/sub-daily.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/sub-daily.yml) [![Daily Rate Fetch & Deploy](https://github.com/sampaio-tech/currency-api/actions/workflows/daily.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/daily.yml) [![Deploy Static Data](https://github.com/sampaio-tech/currency-api/actions/workflows/static-deploy.yml/badge.svg)](https://github.com/sampaio-tech/currency-api/actions/workflows/static-deploy.yml)

A free, open-source currency exchange rate API — rebuilt in Rust.

Inspired by [fawazahmed0/exchange-api](https://github.com/fawazahmed0/exchange-api).

GitHub Actions fetches live exchange rates every hour, generates static JSON files,
and deploys them to Cloudflare Pages. No server. No database. No rate limits.

The goal of this project is to let anyone **fork or clone it and have their own currency API running in minutes** — just plug in your API keys, configure the GitHub secrets, and you're live.

See [docs/SETUP.md](./docs/SETUP.md) to get started.

---

## How it works

```
sub-daily.yml (every 8 h, schedule)    daily.yml (manual, from Actions tab)
    │                                       │
    ├── fetches fiat rates    (exchangerate-api.com v6 or open.er-api.com)
    ├── fetches crypto rates  (CoinGecko)
    ├── fetches metal rates   (metalpriceapi.com — XAU, XAG, XPT, XPD)
    │
    ├── computes cross rates for every currency pair
    ├── writes static JSON files to dist/
    ├── bundles flags, names, symbols from data/ into dist/
    │
    ├── deploys to Cloudflare Pages → your-project.pages.dev          (production / latest)
    ├── deploys to Cloudflare Pages → {YYYY-MM-DD}.your-project.pages.dev
    └── [sub-daily only] → {YYYY-MM-DDtHH-MM}.your-project.pages.dev
```

---

## API endpoints

Replace `{project}` with your Cloudflare Pages project name.

| Endpoint | URL |
|---|---|
| All currencies (latest) | `https://{project}.pages.dev/v1/currencies.json` |
| Rates for USD (latest) | `https://{project}.pages.dev/v1/currencies/usd.json` |
| Rates for EUR (latest) | `https://{project}.pages.dev/v1/currencies/eur.json` |
| Historical (date) | `https://{YYYY-MM-DD}.{project}.pages.dev/v1/currencies/usd.json` |
| Historical (timestamp) | `https://{YYYY-MM-DDtHH-MM}.{project}.pages.dev/v1/currencies/usd.json` |
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
  "date": "YYYY-MM-DD",
  "timestamp": "YYYY-MM-DDTHH:MM:00Z",
  "usd": {
    "eur": 0.92,
    "gbp": 0.79,
    "btc": 0.000011,
    "jpy": 149.2
  }
}
```

The `timestamp` field is present on sub-daily runs. Date-only runs omit it for backward compatibility.

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
│       ├── crypto.rs    # fetches crypto rates (CoinGecko)
│       └── metals.rs    # fetches precious metal rates (metalpriceapi.com)
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
│   ├── BRUNO.md                  # Bruno API collection guide
│   ├── CURRENCY-SYMBOLS.md       # symbols endpoint reference
│   ├── CURRENCY-NAMES.md         # i18n names endpoint reference
│   └── CURRENCY-FLAGS.md         # flag SVGs endpoint reference
├── bruno/
│   ├── environments/             # Production, Date Snapshot, Timestamp Snapshot, Local
│   ├── Rates/                    # exchange rate requests (pretty + minified)
│   ├── Static/                   # symbols, names, flags
│   └── Historical/               # date and timestamp snapshot requests
├── .github/
│   └── workflows/
│       ├── daily.yml             # manual: date snapshots (run from Actions tab)
│       ├── sub-daily.yml         # schedule: runs every hour (configurable), manual trigger allowed
│       └── static-deploy.yml     # manual: publishes flags, names, and symbols to CDN
└── Cargo.toml
```

---

## Testing with Bruno

A [Bruno](https://www.usebruno.com/) collection in `bruno/` covers every endpoint with assertions.

```
bruno/
├── Rates/      — all currencies index, USD/EUR/BTC rates (pretty + minified)
├── Static/     — symbols, names (en/pt_BR/zh), flags (USD/EUR/BTC)
└── Historical/ — USD/EUR by date and by timestamp
```

**4 environments**: Production, Date Snapshot, Timestamp Snapshot, Local.
Switch the snapshot target by editing `dateBaseUrl` / `timestampBaseUrl` in the environment file.

See [docs/BRUNO.md](./docs/BRUNO.md) for full setup and local testing instructions.

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

## Quick setup

1. **Fork** this repository (or push it to a new GitHub repo)
2. **Create a Cloudflare Pages project** with production branch `latest`:
   ```bash
   npm install -g wrangler && wrangler login
   wrangler pages project create currency-api
   ```
3. **Add GitHub secrets**: `CF_API_TOKEN`, `CF_ACCOUNT_ID`, `FIAT_API_URL`, `CRYPTO_API_URL` (and optionally `FIAT_API_KEY`, `CRYPTO_API_KEY`, `METALS_API_KEY`)
4. **Run `static-deploy.yml` once** from the Actions tab to publish flags, names, and symbols to `latest`
5. **Done** — `daily.yml` and `sub-daily.yml` run automatically on schedule

See [docs/SETUP.md](./docs/SETUP.md) for the full step-by-step guide including how to get API keys.

---

## Static data

Flags, currency names (i18n), and symbols are bundled into every deployment alongside the exchange rates. They live in `data/` and do not change per run.

To update static files (e.g. after adding a new flag or translation):

1. Commit the changes to `data/`
2. Run **Deploy Static Data** (`static-deploy.yml`) from the Actions tab — it deploys immediately to `latest`
3. The next scheduled run of `daily.yml` or `sub-daily.yml` will also include the updated files

> Cloudflare Pages does not support selective file deletion — each deployment is a complete
> snapshot of `dist/`. To "remove" a file, redeploy without it.

---

## Deployment

See [docs/SETUP.md](./docs/SETUP.md) for a full step-by-step guide on how to:
- Get your API keys
- Create a Cloudflare Pages project
- Configure GitHub secrets
- Trigger your first deployment

---

## Update frequency

The sub-daily workflow (`sub-daily.yml`) runs every 8 hours by default (3×/day) and also supports manual triggers from the Actions tab. To change the frequency, edit the `cron` expression in `.github/workflows/sub-daily.yml`:

```yaml
- cron: "0 */8 * * *"  # Every 8 hours (3×/day)
```

**Free-tier budget** (sub-daily only, worst-case 31-day month — daily is manual so not counted):

| Expression | Runs/month | Fiat requests | Metal requests | Fits all free limits? |
|---|---|---|---|---|
| `0 */8 * * *` | ~93 | ~93 | ~93 | ✅ Yes *(default)* |
| `0 */6 * * *` | ~124 | ~124 | ~124 | ✅ Yes |
| `0 */4 * * *` | ~186 | ~186 | ~186 | ✅ Yes |
| `0 */2 * * *` | ~372 | ~372 | ~372 | ✅ Yes |
| `0 * * * *` | ~744 | ~744 | ~744 | ❌ Exceeds metalpriceapi.com (100/month) |

> Each run makes one request per source: fiat (open.er-api.com — free, no key), crypto
> (CoinGecko — 10 000/month free), and precious metals (metalpriceapi.com — 100/month free).
> The metals limit is the bottleneck — every 8 hours is the highest frequency that stays
> within the free tier across all three sources.
>
> If you don't set `METALS_API_KEY`, precious metals are skipped and you can run as
> frequently as CoinGecko allows.

The daily workflow (`daily.yml`) is **manual-only** and produces date snapshots without a `timestamp` field.
