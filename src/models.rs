use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sport {
    pub key: String,
    pub active: bool,
    pub group: String,
    pub description: String,
    pub title: String,
    pub has_outrights: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub sport_key: String,
    pub sport_title: String,
    pub commence_time: String,
    pub home_team: Option<String>,
    pub away_team: Option<String>,
    pub bookmakers: Vec<Bookmaker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmaker {
    pub key: String,
    pub title: String,
    pub last_update: String,
    pub markets: Vec<Market>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Market {
    pub key: String, // e.g. "outrights"
    pub last_update: String,
    pub outcomes: Vec<Outcome>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Outcome {
    pub name: String,
    pub price: f64,
}
