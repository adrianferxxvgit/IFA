use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRecord {
    pub id: Option<i64>,
    pub external_id: String,
    pub provider: String,
    pub competition: String,
    pub season: i32,
    pub match_date: String,
    pub home_team: String,
    pub away_team: String,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
    pub result: Option<String>,
    pub home_xg: Option<f64>,
    pub away_xg: Option<f64>,
    pub home_xg_against: Option<f64>,
    pub away_xg_against: Option<f64>,
    pub home_possession: Option<f64>,
    pub away_possession: Option<f64>,
    pub home_shots_on_target: Option<i32>,
    pub away_shots_on_target: Option<i32>,
    pub home_ppda: Option<f64>,
    pub away_ppda: Option<f64>,
}
