//! Módulo de moduladores por liga para IFA v8.0
//! Basado en análisis estadístico de 29,510 partidos (2014-2019)

/// Modulador de ventaja local según la liga
pub fn get_home_advantage(competition: &str) -> f64 {
    match competition.to_lowercase().as_str() {
        "la liga" | "bundesliga" | "primeira liga" | "eredivisie" | "segunda division" => 0.015,
        "super lig" | "belgian pro league" | "premier league" | "ligue 1" | "serie a" => 0.012,
        "serie b" | "championship" | "scottish premier" | "league one" | "ligue 2" | "2. bundesliga" => 0.008,
        _ => 0.010,
    }
}

/// Probabilidad base de empate según la liga
pub fn get_draw_probability(competition: &str) -> f64 {
    match competition.to_lowercase().as_str() {
        "serie b" => 0.331,
        "segunda division" => 0.303,
        "ligue 2" | "2. bundesliga" => 0.294,
        "championship" => 0.279,
        "league one" | "ligue 1" | "serie a" => 0.261,
        "belgian pro league" | "super lig" | "bundesliga" => 0.252,
        "la liga" | "premier league" | "primeira liga" => 0.242,
        "eredivisie" => 0.231,
        _ => 0.267,
    }
}

/// Boost ofensivo para equipos históricamente goleadores
pub fn get_offensive_boost(team: &str) -> f64 {
    match team.to_lowercase().as_str() {
        "barcelona" => 1.15,
        "paris sg" | "psg" | "benfica" | "bayern munich" | "bayern" => 1.12,
        "ajax" | "real madrid" | "psv eindhoven" | "psv" => 1.10,
        "man city" | "manchester city" | "porto" | "celtic" => 1.08,
        "napoli" | "juventus" | "dortmund" | "liverpool" | "arsenal" | "lyon" => 1.05,
        "galatasaray" | "feyenoord" => 1.04,
        _ => 1.00,
    }
}

/// Clasificación táctica de la liga
pub fn get_league_profile(competition: &str) -> &'static str {
    match competition.to_lowercase().as_str() {
        "serie b" | "segunda division" | "ligue 2" | "2. bundesliga" => "Muy Táctica (alta prob. empate)",
        "championship" | "league one" | "ligue 1" | "serie a" => "Equilibrada",
        "belgian pro league" | "super lig" | "bundesliga" | "la liga" | "premier league" | "primeira liga" => "Ofensiva",
        "eredivisie" => "Muy Ofensiva (alta media de goles)",
        _ => "Estándar",
    }
}