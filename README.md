# Rusty Weather Cron

A small Rust web app that:

- Serves basic routes with **Axum**
- Connects to **Postgres** using `sqlx`
- Runs a **cron job every hour** to fetch Indore weather data
- Logs results into a Postgres table

## Features

- `/` → Hello route
- `/health` → Health check
- `/city` → Sample city info
- Hourly cron job → fetches weather via OpenWeather API and stores logs

## Setup

1. Clone repo:
   ```bash
   git clone https://github.com/bhaweshpnwar/rusty-weather-cron.git
   cd rusty-weather-cron
   ```
