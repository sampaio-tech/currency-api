# Setup Guide

This guide walks you through everything needed to deploy your own instance of
currency-api. Follow each step in order.

---

## Prerequisites

- A [GitHub](https://github.com) account
- A [Cloudflare](https://cloudflare.com) account (free tier is enough)
- Rust installed locally — https://rustup.rs

---

## Step 1 — Fork or push this repo to GitHub

If you cloned this project locally, create a new GitHub repository and push it:

```bash
git remote add origin https://github.com/YOUR_USERNAME/currency-api.git
git branch -M main
git push -u origin main
```

---

## Step 2 — Get your fiat exchange rate API key

We use **ExchangeRate-API** as the fiat source. It supports EUR as a base and
has a free tier of 1,500 requests/month (more than enough for daily runs).

1. Go to https://www.exchangerate-api.com
2. Click **"Get Free Key"**
3. Sign up with your email
4. After confirming your email, you will see your **API key** on the dashboard
5. Copy it — you will need it in Step 5

The free URL format is:
```
https://v6.exchangerate-api.com/v6/YOUR_API_KEY/latest/EUR
```

> If you prefer not to use an API key, the default fiat source
> (`https://open.er-api.com/v6/latest/EUR`) works without one,
> but has stricter rate limits.

---

## Step 3 — Get your MetalpriceAPI key (optional — for precious metals)

[metalpriceapi.com](https://metalpriceapi.com) provides precious metal rates (XAU, XAG, XPT, XPD).
The free tier includes 100 requests/month — enough for the default schedule (every 8 hours, ~93 req/month).

1. Go to https://metalpriceapi.com
2. Click **"Get Free API Key"**
3. Sign up with your email
4. Copy the API key from your dashboard — you will need it in Step 5

> If you skip this step, precious metals (XAU, XAG, XPT, XPD) will be omitted from the output.
> All other currencies (fiat and crypto) are unaffected.

---

## Step 4 — Get your CoinGecko API key (optional but recommended)

CoinGecko provides crypto prices. The free tier works without a key, but
adding one gives you higher rate limits and more reliability.

1. Go to https://www.coingecko.com/en/api
2. Click **"Get Your Free API Key"**
3. Sign up and verify your email
4. Go to your dashboard → **API Keys** → copy your **Demo API key**

> Without a key the default CoinGecko URL still works, just leave
> `CRYPTO_API_KEY` empty in the next steps.

---

## Step 5 — Create your Cloudflare Pages project

Cloudflare Pages will host all the generated JSON files for free.

### 4a. Get your Cloudflare Account ID

1. Log in at https://dash.cloudflare.com
2. On the right sidebar of the homepage you will see **Account ID**
3. Copy it

### 4b. Create an API Token

1. Click your profile icon (top right) → **My Profile**
2. Go to **API Tokens** → **Create Token**
3. Click **"Use template"** next to **"Edit Cloudflare Workers"**
4. Under **Account Resources**, select your account
5. Under **Zone Resources**, select **All zones**
6. Click **Continue to Summary** → **Create Token**
7. Copy the token — it is only shown once

### 4c. Create the Pages project

Install Wrangler (Cloudflare's CLI) and create the project:

```bash
npm install -g wrangler
wrangler login
wrangler pages project create currency-api
```

When asked for the production branch name, enter: `latest`

> The project name `currency-api` will become part of your URL:
> `currency-api.pages.dev` (production / latest rates)
>
> In Cloudflare Pages, the **production branch** is always served at the root domain
> (`{project}.pages.dev`) — there is no `latest.` subdomain. Date and timestamp
> snapshots are deployed as branch aliases and get their own subdomain automatically:
> `{YYYY-MM-DD}.currency-api.pages.dev`, `{YYYY-MM-DDtHH-MM}.currency-api.pages.dev`, etc.
>
> You can choose any project name you want — just keep it consistent.

---

## Step 6 — Add secrets to your GitHub repository

GitHub Actions needs these values to fetch rates and deploy.

1. On GitHub, go to your repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **"New repository secret"** and add each one below:

| Secret name      | Value                                                              |
|------------------|--------------------------------------------------------------------|
| `CF_API_TOKEN`   | The Cloudflare API token from Step 5b                             |
| `CF_ACCOUNT_ID`  | Your Cloudflare Account ID from Step 5a                           |
| `FIAT_API_URL`   | `https://v6.exchangerate-api.com/v6/{key}/latest/EUR` *(optional — see note below)* |
| `FIAT_API_KEY`   | Your ExchangeRate-API key *(required if using the URL above)*     |
| `CRYPTO_API_URL` | `https://api.coingecko.com/api/v3/simple/price` *(optional)*      |
| `CRYPTO_API_KEY` | Your CoinGecko Demo API key from Step 4 *(optional)*              |
| `METALS_API_KEY` | Your MetalpriceAPI key from Step 3 *(optional — omit to skip metals)* |

> **Fiat API:** If you don't set `FIAT_API_URL`, the default (`open.er-api.com`) is used — no key needed.
> To use ExchangeRate-API instead, set `FIAT_API_URL` to the `{key}` template above and set `FIAT_API_KEY`
> to your key. The `{key}` placeholder will be substituted at runtime; the key is never sent as a header.
>
> **Metals API:** `METALS_API_URL` has a sensible default — you only need to set `METALS_API_KEY`.
> If omitted, XAU/XAG/XPT/XPD are silently skipped.

---

## Step 7 — Deploy static data (once)

Before the scheduled workflows run, you need to publish the static files (flags, names, symbols) to Cloudflare Pages:

1. On GitHub, go to your repository
2. Click the **Actions** tab
3. Click **"Deploy Static Data"** in the left sidebar
4. Click **"Run workflow"** → **"Run workflow"**

Wait about 1–2 minutes. This only needs to be done once (or whenever you update the files in `data/`).

---

## Step 8 — Wait for the schedule (or trigger daily manually)

There are two workflows:

| Workflow | File | Schedule | Manual trigger? |
|---|---|---|---|
| **Daily Rate Fetch & Deploy** | `daily.yml` | No — manual only | Yes |
| **Sub-daily Rate Fetch & Deploy** | `sub-daily.yml` | Every 8 hours *(configurable)* | Yes |

To trigger the **daily** workflow right now:

1. On GitHub, go to your repository
2. Click **Actions** tab
3. Click **"Daily Rate Fetch & Deploy"** in the left sidebar
4. Click **"Run workflow"** → **"Run workflow"**

Wait about 2–3 minutes. When it turns green, your API is live at:

```
https://currency-api.pages.dev/v1/currencies.json
https://currency-api.pages.dev/v1/currencies/usd.json
```

### Changing the sub-daily update frequency

Open `.github/workflows/sub-daily.yml` and edit the `cron` expression:

```yaml
- cron: "0 */8 * * *"  # Every 8 hours (3×/day)
```

**Free-tier budget** (sub-daily only, worst-case 31-day month — daily is manual):

| Expression | Runs/month | Fiat | Crypto | Metals | Fits all free limits? |
|---|---|---|---|---|---|
| `0 */8 * * *` | ~93 | ~93 | ~93 | ~93 | ✅ Yes *(default)* |
| `0 */6 * * *` | ~124 | ~124 | ~124 | ~124 | ✅ Yes |
| `0 */4 * * *` | ~186 | ~186 | ~186 | ~186 | ✅ Yes |
| `0 */2 * * *` | ~372 | ~372 | ~372 | ~372 | ✅ Yes |
| `0 * * * *` | ~744 | ~744 | ~744 | ~744 | ❌ Exceeds metals limit (100/month) |

> Free-tier limits: fiat (open.er-api.com) — unlimited; CoinGecko — 10 000/month;
> metalpriceapi.com — 100/month. The metals limit is the bottleneck.
>
> If `METALS_API_KEY` is not set, metals are skipped and you can safely run more frequently.
>
> GitHub Actions may delay scheduled runs by a few minutes under load — this is normal.

---

## Step 9 — Verify it works

Open your browser and visit:

```
https://currency-api.pages.dev/v1/currencies/eur.json
```

You should see a JSON response with today's date and exchange rates.

---

## Troubleshooting

### The workflow failed

1. Click the failed workflow run on the **Actions** tab
2. Click the failing step to see the error log
3. Common causes:
   - Wrong `CF_API_TOKEN` → re-create the token in Cloudflare
   - Wrong `FIAT_API_URL` → double-check the URL, your API key, and that the base is `/latest/EUR` not `/latest/USD`
   - CoinGecko rate limit → add a `CRYPTO_API_KEY`

### I see an old date in the response

Cloudflare CDN caches files. Wait a few minutes after deployment or append
`?nocache=1` to the URL to bypass the cache temporarily.

### I want to use a custom domain

1. Go to https://dash.cloudflare.com → **Pages** → your project
2. Click **Custom domains** → **Set up a custom domain**
3. Follow the instructions to add a CNAME record in your DNS

---

## Environment variables reference

For local testing, copy `.env.example` to `.env`, fill in your values, and run:

```bash
source .env && cargo run -- --dry-run
```

For CI, set these as GitHub repository secrets (Settings → Secrets → Actions):

| Variable          | Required | Default                                                                      | Description                                              |
|-------------------|----------|------------------------------------------------------------------------------|----------------------------------------------------------|
| `FIAT_API_URL`    | No       | `https://open.er-api.com/v6/latest/EUR`                                      | EUR-based fiat endpoint. Supports `{key}` placeholder for exchangerate-api.com (e.g. `https://v6.exchangerate-api.com/v6/{key}/latest/EUR`). Must use EUR as base. |
| `FIAT_API_KEY`    | No       | *(none)*                                                                     | Substituted into `{key}` in `FIAT_API_URL`, or sent as Bearer header if no placeholder is present |
| `CRYPTO_API_URL`  | No       | `https://api.coingecko.com/api/v3/simple/price`                              | CoinGecko-compatible price endpoint                      |
| `CRYPTO_API_KEY`  | No       | *(none)*                                                                     | `x-cg-demo-api-key` header value                         |
| `METALS_API_URL`  | No       | `https://api.metalpriceapi.com/v1/latest?base=EUR&currencies=XAU,XAG,XPT,XPD` | Precious metals endpoint (metalpriceapi.com-compatible)  |
| `METALS_API_KEY`  | No       | *(none)*                                                                     | `X-API-KEY` header value. If not set, metals are skipped |
| `CF_API_TOKEN`    | Yes*     | *(none)*                                                                     | Cloudflare API token for deployment                      |
| `CF_ACCOUNT_ID`   | Yes*     | *(none)*                                                                     | Cloudflare account ID                                    |

*Required only in GitHub Actions for deployment. Not needed for local runs.
