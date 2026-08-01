use crate::models::MatchRecord;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct CreativesDevProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl CreativesDevProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://free-api-live-football-data.p.rapidapi.com".to_string(),
        }
    }

    pub async fn fetch_fixtures(&self, league_id: i32, season: i32) -> Result<Vec<MatchRecord>, Box<dyn Error>> {
        let url = format!("{}/fixtures?league={}&season={}", self.base_url, league_id, season);

        let response = self.client
            .get(&url)
            .header("x-rapidapi-key", &self.api_key)
            .header("x-rapidapi-host", "free-api-live-football-data.p.rapidapi.com")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Error HTTP: {}", response.status()).into());
        }

        let json: Value = response.json().await?;
        
        let matches_array = json.get("response")
            .and_then(|v| v.as_array())
            .ok_or("No se encontró el array 'response' en el JSON de la API")?;

        let mut records = Vec::new();

        for item in matches_array {
            let home_team = item.get("homeTeam").and_then(|t| t.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let away_team = item.get("awayTeam").and_then(|t| t.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            
            if home_team == "Unknown" || away_team == "Unknown" {
                continue;
            }

            let home_score = item.get("homeScore").and_then(|v| v.as_i64()).map(|v| v as i32);
            let away_score = item.get("awayScore").and_then(|v| v.as_i64()).map(|v| v as i32);

            let result = match (home_score, away_score) {
                (Some(h), Some(a)) if h > a => Some("home_win".to_string()),
                (Some(h), Some(a)) if a > h => Some("away_win".to_string()),
                (Some(h), Some(a)) if h == a => Some("draw".to_string()),
                _ => None,
            };

            let ext_id = item.get("id").and_then(|v| v.as_i64()).unwrap_or(0).to_string();
            
            let date_str = if let Some(ts) = item.get("timestamp").and_then(|v| v.as_i64()) {
                chrono::DateTime::from_timestamp(ts, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string())
            } else {
                item.get("time").and_then(|v| v.as_str()).unwrap_or("1970-01-01T00:00:00Z").to_string()
            };

            let league_name = item.get("league").and_then(|l| l.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown League").to_string();
            let season_val = item.get("league").and_then(|l| l.get("season")).and_then(|v| v.as_i64()).unwrap_or(season as i64) as i32;

            records.push(MatchRecord {
                id: None,
                external_id: ext_id,
                provider: "creativesdev".to_string(),
                competition: league_name,
                season: season_val,
                match_date: date_str,
                home_team,
                away_team,
                home_score,
                away_score,
                result,
                home_xg: None,
                away_xg: None,
                home_xg_against: None,
                away_xg_against: None,
                home_possession: None,
                away_possession: None,
                home_shots_on_target: None,
                away_shots_on_target: None,
                home_ppda: None,
                away_ppda: None,
            });
        }

        Ok(records)
    }
}
