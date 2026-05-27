use anyhow::{Context, Result};
use reqwest::Client;
use std::env;

use crate::cache::Cache;
use crate::models::{Event, Sport};

pub struct OddsApi {
    client: Client,
    api_key: String,
    cache: Cache,
}

impl OddsApi {
    pub fn new() -> Result<Self> {
        let api_key = env::var("ODDS_API_KEY")
            .context("Variabile ODDS_API_KEY non trovata. Controlla il file .env")?;

        Ok(Self {
            client: Client::new(),
            api_key,
            cache: Cache::new()?,
        })
    }

    pub async fn fetch_sports(&self) -> Result<Vec<Sport>> {
        let cache_key = "sports";
        if let Some(data) = self.cache.get(cache_key) {
            return Ok(serde_json::from_str(&data)?);
        }

        let url = format!(
            "https://api.the-odds-api.com/v4/sports/?apiKey={}",
            self.api_key
        );
        let res = self.client.get(&url).send().await?.text().await?;

        self.cache.set(cache_key, &res)?;
        let sports: Vec<Sport> = serde_json::from_str(&res)?;
        Ok(sports)
    }

    pub async fn fetch_outrights(&self, sport_key: &str) -> Result<Vec<Event>> {
        let cache_key = format!("outrights_{}", sport_key);
        if let Some(data) = self.cache.get(&cache_key) {
            return Ok(serde_json::from_str(&data)?);
        }

        let url = format!(
            "https://api.the-odds-api.com/v4/sports/{}/odds/?apiKey={}&regions=eu&markets=outrights",
            sport_key, self.api_key
        );
        let res = self.client.get(&url).send().await?.text().await?;

        self.cache.set(&cache_key, &res)?;
        let events: Vec<Event> = serde_json::from_str(&res)?;
        Ok(events)
    }
}
