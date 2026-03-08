# Currency Symbols

This document describes the currency symbols endpoint available on the CDN.

## Data source

The symbols dataset (`data/currency-symbols/currency_symbols.json`) is committed directly
to this repository. It maps 170+ ISO 4217 currency codes to their display symbols and is
updated manually via the `symbols-deploy` workflow.

Original data sourced from the community-maintained
[currency-converter Firebase dataset](https://github.com/umpirsky/currency-list).

## Endpoint

```
GET /v1/currencies/symbols.json
```

**Example:**
```
https://currency-api.pages.dev/v1/currencies/symbols.json
```

## Response format

A flat JSON object mapping uppercase ISO 4217 currency codes to their symbol string:

```json
{
  "AED": "د.إ",
  "BRL": "R$",
  "EUR": "€",
  "GBP": "£",
  "JPY": "¥",
  "USD": "$",
  "BTC": "₿"
}
```

Keys are sorted alphabetically. The file covers ~170 fiat currencies and a small set
of major cryptocurrencies (BTC, ETH, LTC, XBT).

## Updating the data

1. Edit `data/currency-symbols/currency_symbols.json` directly and open a PR.
2. After merging, trigger the **Deploy Currency Symbols** workflow manually from the
   GitHub Actions tab to publish the changes to the CDN.

## Workflow

The `symbols-deploy.yml` workflow:

- Is triggered **manually only** (`workflow_dispatch`) — no scheduled runs.
- Copies `data/currency-symbols/currency_symbols.json` into `dist/v1/currencies/symbols.json`.
- Deploys to Cloudflare Pages on the `latest` branch (production CDN).

> **Note:** The rate-fetch workflows (`daily.yml`, `sub-daily.yml`) deploy their own
> generated `dist/` and will overwrite the `latest` branch on their next run. If you need
> symbols to always coexist with rate data, add a `cp data/currency-symbols/currency_symbols.json dist/v1/currencies/symbols.json`
> step to those workflows before their deploy step.
