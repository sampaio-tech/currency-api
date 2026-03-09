# Bruno Collection

[Bruno](https://www.usebruno.com/) is an open-source API client that stores collections as plain files alongside your code. The `bruno/` directory at the project root contains a ready-to-use collection covering every endpoint.

---

## Installation

Download Bruno from https://www.usebruno.com/downloads or install via Homebrew:

```bash
brew install bruno
```

---

## Opening the collection

1. Open Bruno
2. Click **Open Collection**
3. Select the `bruno/` folder inside this repository

---

## Environments

Switch environments using the dropdown in the top-right corner of Bruno.

| Environment | Base URL | Use for |
|---|---|---|
| **Production** | `https://currency-api.pages.dev` | Testing the live latest deployment |
| **Date Snapshot** | `https://2026-03-08.currency-api.pages.dev` | Testing a specific date snapshot |
| **Timestamp Snapshot** | `https://2026-03-08t14-30.currency-api.pages.dev` | Testing a specific hourly snapshot |
| **Local** | `http://localhost:3000` | Testing a locally generated `dist/` |

### Changing the target snapshot

Open `bruno/environments/Date Snapshot.bru` or `Timestamp Snapshot.bru` and edit the `baseUrl`, `dateBaseUrl`, and `timestampBaseUrl` vars to point at any snapshot you want to inspect:

```
vars {
  baseUrl: https://2026-01-15.currency-api.pages.dev
  dateBaseUrl: https://2026-01-15.currency-api.pages.dev
  timestampBaseUrl: https://2026-01-15t08-00.currency-api.pages.dev
}
```

> All three vars exist in every environment so that Historical requests
> (`{{dateBaseUrl}}`, `{{timestampBaseUrl}}`) always resolve correctly
> regardless of which environment is active.

---

## Folders

### Rates

Exchange rate endpoints for the currently active environment's base URL.

| Request | Path |
|---|---|
| All Currencies | `/v1/currencies.json` |
| All Currencies (min) | `/v1/currencies.min.json` |
| USD Rates | `/v1/currencies/usd.json` |
| USD Rates (min) | `/v1/currencies/usd.min.json` |
| EUR Rates | `/v1/currencies/eur.json` |
| EUR Rates (min) | `/v1/currencies/eur.min.json` |
| BTC Rates | `/v1/currencies/btc.json` |
| BTC Rates (min) | `/v1/currencies/btc.min.json` |

### Static

Static data that does not change per run.

| Request | Path |
|---|---|
| Currency Symbols | `/v1/currencies/symbols.json` |
| Currency Names (en) | `/v1/currencies/names/en.json` |
| Currency Names (pt_BR) | `/v1/currencies/names/pt_BR.json` |
| Currency Names (zh) | `/v1/currencies/names/zh.json` |
| Flag - USD | `/v1/currencies/flags/usd.svg` |
| Flag - EUR | `/v1/currencies/flags/eur.svg` |
| Flag - BTC | `/v1/currencies/flags/btc.svg` |

### Historical

Requests that target a specific past snapshot using `{{dateBaseUrl}}` or `{{timestampBaseUrl}}`. Change those vars in the active environment to switch between snapshots.

| Request | Var used | Snapshot type |
|---|---|---|
| USD Rates by Date | `{{dateBaseUrl}}` | Daily run (no `timestamp` field) |
| EUR Rates by Date | `{{dateBaseUrl}}` | Daily run (no `timestamp` field) |
| USD Rates by Timestamp | `{{timestampBaseUrl}}` | Sub-daily run (has `timestamp` field) |
| EUR Rates by Timestamp | `{{timestampBaseUrl}}` | Sub-daily run (has `timestamp` field) |

---

## Testing locally

Generate the static files and serve them:

```bash
# 1. Generate dist/
source .env && cargo run --release -- --output dist/

# 2. Copy static data
mkdir -p dist/v1/currencies/flags dist/v1/currencies/names
cp data/currency-flags/*.svg dist/v1/currencies/flags/
cp data/currency-names/*.json dist/v1/currencies/names/
cp data/currency-symbols/currency_symbols.json dist/v1/currencies/symbols.json

# 3. Serve
npx serve dist --listen 3000
# or: python3 -m http.server 3000 --directory dist
```

Then select the **Local** environment in Bruno and send any request.

---

## Running tests

Each request includes `assert` and `tests` blocks that verify:
- HTTP status is 200
- Response contains expected top-level keys (`date`, `usd`, `eur`, etc.)
- Static data has correct values (e.g. USD symbol is `$`, English name is `US Dollar`)
- Date-only snapshots do **not** contain a `timestamp` field
- Sub-daily snapshots **do** contain a `timestamp` field

To run all tests at once, click **Run Collection** in Bruno's sidebar.
