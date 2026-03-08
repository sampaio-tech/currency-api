# Currency Names (i18n)

This document describes the currency name translation endpoints available on the CDN.

## Data source

Translation files under `data/currency-names/` are committed directly to this repository.
They were sourced from [`umpirsky/currency-list`](https://github.com/umpirsky/currency-list) —
a community-maintained dataset of ISO 4217 currency names translated into 566 locales,
built from Unicode CLDR data.

Each file is named `{locale}.json` (e.g. `en.json`, `pt_BR.json`, `zh.json`) and maps
ISO 4217 currency codes to their localized names.

## Endpoints

```
GET /v1/currencies/names/{locale}.json
```

**Examples:**
```
https://currency-api.pages.dev/v1/currencies/names/en.json
https://currency-api.pages.dev/v1/currencies/names/pt_BR.json
https://currency-api.pages.dev/v1/currencies/names/zh.json
```

## Response format

A flat JSON object mapping uppercase ISO 4217 currency codes to their localized name:

**`/v1/currencies/names/en.json`**
```json
{
  "BRL": "Brazilian Real",
  "EUR": "Euro",
  "GBP": "British Pound Sterling",
  "USD": "US Dollar"
}
```

**`/v1/currencies/names/pt_BR.json`**
```json
{
  "BRL": "Real Brasileiro",
  "EUR": "Euro",
  "GBP": "Libra Esterlina Britânica",
  "USD": "Dólar Americano"
}
```

## Available locales

566 locales are available, covering all major languages and regional variants.
Common examples:

| Locale | Language |
|--------|----------|
| `en` | English |
| `pt` | Portuguese |
| `pt_BR` | Portuguese (Brazil) |
| `es` | Spanish |
| `fr` | French |
| `de` | German |
| `zh` | Chinese (Simplified) |
| `zh_Hant` | Chinese (Traditional) |
| `ja` | Japanese |
| `ar` | Arabic |
| `ru` | Russian |
| `hi` | Hindi |

Browse all available locale files in [`data/currency-names/`](../data/currency-names/).

## Updating the data

1. To refresh from upstream, re-download all files from
   [`umpirsky/currency-list`](https://github.com/umpirsky/currency-list) into `data/currency-names/`
   and open a PR.
2. After merging, trigger the **Deploy Currency Names (i18n)** workflow manually from the
   GitHub Actions tab to publish the changes to the CDN.

## Workflow

The `i18n-deploy.yml` workflow:

- Is triggered **manually only** (`workflow_dispatch`) — no scheduled runs.
- Copies all `data/currency-names/*.json` files into `dist/v1/currencies/names/`.
- Deploys to Cloudflare Pages on the `latest` branch (production CDN).

> **Note:** The rate-fetch workflows (`daily.yml`, `sub-daily.yml`) deploy their own
> generated `dist/` and will overwrite the `latest` branch on their next run. If you need
> translations to always coexist with rate data, add a
> `cp -r data/currency-names/ dist/v1/currencies/names/` step to those workflows before
> their deploy step.
