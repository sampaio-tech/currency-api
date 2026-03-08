# Currency Flags

This document describes the currency flag SVG endpoints available on the CDN.

## Data source

SVG flag files under `data/currency-flags/` are committed directly to this repository.
The dataset covers 1214 currencies and crypto tokens, sourced from a Firebase Storage bucket.

## Endpoint

```
GET /v1/currencies/flags/{code}.svg
```

**Examples:**
```
https://currency-api.pages.dev/v1/currencies/flags/eur.svg
https://currency-api.pages.dev/v1/currencies/flags/btc.svg
https://currency-api.pages.dev/v1/currencies/flags/ada.svg
```

Currency codes in the filename are **lowercase**.

## Coverage

1214 SVG files covering fiat currencies and crypto tokens. Browse all available flags in
[`data/currency-flags/`](../data/currency-flags/).

## Updating the data

1. Add or replace SVG files in `data/currency-flags/` and open a PR.
2. After merging, trigger the **Deploy Currency Flags** workflow manually from the
   GitHub Actions tab to publish the changes to the CDN.

## Workflow

The `flags-deploy.yml` workflow:

- Is triggered **manually only** (`workflow_dispatch`) — no scheduled runs.
- Copies all `data/currency-flags/*.svg` files into `dist/v1/currencies/flags/`.
- Deploys to Cloudflare Pages on the `latest` branch (production CDN).

> **Note:** The rate-fetch workflows (`daily.yml`, `sub-daily.yml`) deploy their own
> generated `dist/` and will overwrite the `latest` branch on their next run. If you need
> flags to always coexist with rate data, add a
> `cp -r data/currency-flags/ dist/v1/currencies/flags/` step to those workflows before
> their deploy step.
