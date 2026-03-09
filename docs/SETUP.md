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

## Step 3 — Get your CoinGecko API key (optional but recommended)

CoinGecko provides crypto prices. The free tier works without a key, but
adding one gives you higher rate limits and more reliability.

1. Go to https://www.coingecko.com/en/api
2. Click **"Get Your Free API Key"**
3. Sign up and verify your email
4. Go to your dashboard → **API Keys** → copy your **Demo API key**

> Without a key the default CoinGecko URL still works, just leave
> `CRYPTO_API_KEY` empty in the next steps.

---

## Step 4 — Create your Cloudflare Pages project

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

## Step 5 — Add secrets to your GitHub repository

GitHub Actions needs these values to fetch rates and deploy.

1. On GitHub, go to your repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **"New repository secret"** and add each one below:

| Secret name      | Value                                                              |
|------------------|--------------------------------------------------------------------|
| `CF_API_TOKEN`   | The Cloudflare API token from Step 4b                             |
| `CF_ACCOUNT_ID`  | Your Cloudflare Account ID from Step 4a                           |
| `FIAT_API_URL`   | `https://v6.exchangerate-api.com/v6/YOUR_API_KEY/latest/EUR`      |
| `FIAT_API_KEY`   | *(leave empty or skip if using the URL above with key embedded)*  |
| `CRYPTO_API_URL` | `https://api.coingecko.com/api/v3/simple/price`                   |
| `CRYPTO_API_KEY` | Your CoinGecko Demo API key from Step 3 *(optional)*              |

> For `FIAT_API_URL`, just embed the key directly in the URL as shown.
> You don't need to set `FIAT_API_KEY` separately.

---

## Step 6 — Deploy static data (once)

Before the scheduled workflows run, you need to publish the static files (flags, names, symbols) to Cloudflare Pages:

1. On GitHub, go to your repository
2. Click the **Actions** tab
3. Click **"Deploy Static Data"** in the left sidebar
4. Click **"Run workflow"** → **"Run workflow"**

Wait about 1–2 minutes. This only needs to be done once (or whenever you update the files in `data/`).

---

## Step 7 — Wait for the schedule (or trigger daily manually)

There are two workflows:

| Workflow | File | Schedule | Manual trigger? |
|---|---|---|---|
| **Daily Rate Fetch & Deploy** | `daily.yml` | No — manual only | Yes |
| **Sub-daily Rate Fetch & Deploy** | `sub-daily.yml` | Every hour *(configurable)* | No — schedule only |

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

Open `.github/workflows/sub-daily.yml` and edit the `cron` expression on line 4:

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

> Each run makes one fiat request (ExchangeRate-API, 1 500/month free) and one crypto
> request (CoinGecko, 10 000/month free). The fiat limit is the bottleneck.
> Every hour is the highest safe frequency on the free tier.
>
> GitHub Actions may delay scheduled runs by a few minutes under load — this is normal.

---

## Step 8 — Verify it works

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

| Variable         | Required | Default                                         | Description                                              |
|------------------|----------|-------------------------------------------------|----------------------------------------------------------|
| `FIAT_API_URL`   | No       | `https://open.er-api.com/v6/latest/EUR`         | EUR-based fiat endpoint — **must use `/latest/EUR`**     |
| `FIAT_API_KEY`   | No       | *(none)*                                        | Bearer token for fiat source                             |
| `CRYPTO_API_URL` | No       | `https://api.coingecko.com/api/v3/simple/price` | CoinGecko-compatible price endpoint                      |
| `CRYPTO_API_KEY` | No       | *(none)*                                        | `x-cg-demo-api-key` header value                         |
| `CF_API_TOKEN`   | Yes*     | *(none)*                                        | Cloudflare API token for deployment                      |
| `CF_ACCOUNT_ID`  | Yes*     | *(none)*                                        | Cloudflare account ID                                    |

*Required only in GitHub Actions for deployment. Not needed for local runs.

> **Important:** `FIAT_API_URL` must always use EUR as the base currency (e.g. `/latest/EUR`).
> Using a different base (e.g. `/latest/USD`) will produce incorrect cross rates.
