use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Match {
    external_id: String,
    competition: String,
    season: String,
    date: Option<String>,
    home_team: String,
    away_team: String,
    result: String,
    home_goals: Option<i32>,
    away_goals: Option<i32>,
    home_xg: f64,
    away_xg: f64,
    home_shots: Option<i32>,
    away_shots: Option<i32>,
    home_shots_on_target: i32,
    away_shots_on_target: i32,
    home_corners: Option<i32>,
    away_corners: Option<i32>,
    obs_home_odds: Option<f64>,
    obs_draw_odds: Option<f64>,
    obs_away_odds: Option<f64>,
    data_source: Option<String>,
}

pub fn migrate_json_to_sqlite(json_path: &str, db_path: &str) -> Result<()> {
    println!("📂 Leyendo JSON: {}", json_path);
    let content = fs::read_to_string(json_path).expect("No se pudo leer el JSON");
    let matches: Vec<Match> = serde_json::from_str(&content).expect("JSON inválido");
    println!("✅ {} partidos cargados en memoria", matches.len());

    println!("🗄️  Conectando a SQLite: {}", db_path);
    let conn = Connection::open(db_path)?;

    // Crear tabla principal
    conn.execute_batch("
        DROP TABLE IF EXISTS matches;
        CREATE TABLE matches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            external_id TEXT UNIQUE NOT NULL,
            competition TEXT NOT NULL,
            season TEXT NOT NULL,
            date TEXT,
            home_team TEXT NOT NULL,
            away_team TEXT NOT NULL,
            result TEXT NOT NULL CHECK(result IN ('home_win', 'away_win', 'draw')),
            home_goals INTEGER,
            away_goals INTEGER,
            home_xg REAL NOT NULL,
            away_xg REAL NOT NULL,
            home_shots INTEGER,
            away_shots INTEGER,
            home_shots_on_target INTEGER NOT NULL,
            away_shots_on_target INTEGER NOT NULL,
            home_corners INTEGER,
            away_corners INTEGER,
            obs_home_odds REAL,
            obs_draw_odds REAL,
            obs_away_odds REAL,
            data_source TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        -- Índices para queries rápidos
        CREATE INDEX idx_competition ON matches(competition);
        CREATE INDEX idx_season ON matches(season);
        CREATE INDEX idx_result ON matches(result);
        CREATE INDEX idx_home_team ON matches(home_team);
        CREATE INDEX idx_away_team ON matches(away_team);
        CREATE INDEX idx_external_id ON matches(external_id);

        -- Tabla para guardar predicciones IFA
        CREATE TABLE IF NOT EXISTS predictions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_external_id TEXT NOT NULL,
            model_version TEXT NOT NULL,
            predicted_winner TEXT NOT NULL,
            confidence_score REAL NOT NULL,
            home_prob REAL,
            away_prob REAL,
            weights_used TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (match_external_id) REFERENCES matches(external_id)
        );

        -- Tabla de calibración del modelo
        CREATE TABLE IF NOT EXISTS model_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            version TEXT NOT NULL,
            total_matches INTEGER NOT NULL,
            accuracy REAL NOT NULL,
            weights_off REAL NOT NULL,
            weights_trans REAL NOT NULL,
            weights_def REAL NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    ")?;

    println!("📝 Insertando partidos...");
    let tx = conn.unchecked_transaction()?;
    
    {
        let mut stmt = tx.prepare("
            INSERT OR REPLACE INTO matches (
                external_id, competition, season, date, home_team, away_team, result,
                home_goals, away_goals, home_xg, away_xg,
                home_shots, away_shots, home_shots_on_target, away_shots_on_target,
                home_corners, away_corners,
                obs_home_odds, obs_draw_odds, obs_away_odds, data_source
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)
        ")?;

        for (i, m) in matches.iter().enumerate() {
            stmt.execute(rusqlite::params![
                m.external_id, m.competition, m.season, m.date,
                m.home_team, m.away_team, m.result,
                m.home_goals, m.away_goals, m.home_xg, m.away_xg,
                m.home_shots, m.away_shots, m.home_shots_on_target, m.away_shots_on_target,
                m.home_corners, m.away_corners,
                m.obs_home_odds, m.obs_draw_odds, m.obs_away_odds, m.data_source
            ])?;
            
            if (i + 1) % 5000 == 0 {
                println!("   ... {} partidos insertados", i + 1);
            }
        }
    }
    
    tx.commit()?;
    println!("✅ Migración completada: {} partidos en SQLite", matches.len());

    // Mostrar estadísticas
    println!("\n📊 Estadísticas de la base de datos:");
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM matches", [], |row| row.get(0))?;
    println!("   Total partidos: {}", total);

    let home_wins: i64 = conn.query_row(
        "SELECT COUNT(*) FROM matches WHERE result = 'home_win'", [], |row| row.get(0)
    )?;
    let away_wins: i64 = conn.query_row(
        "SELECT COUNT(*) FROM matches WHERE result = 'away_win'", [], |row| row.get(0)
    )?;
    let draws: i64 = conn.query_row(
        "SELECT COUNT(*) FROM matches WHERE result = 'draw'", [], |row| row.get(0)
    )?;
    println!("   Victorias local: {} ({:.1}%)", home_wins, home_wins as f64 / total as f64 * 100.0);
    println!("   Victorias visitante: {} ({:.1}%)", away_wins, away_wins as f64 / total as f64 * 100.0);
    println!("   Empates: {} ({:.1}%)", draws, draws as f64 / total as f64 * 100.0);

    println!("\n🏆 Top 10 Ligas:");
    let mut stmt = conn.prepare("
        SELECT competition, COUNT(*) as cnt 
        FROM matches 
        GROUP BY competition 
        ORDER BY cnt DESC 
        LIMIT 10
    ")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;
    for row in rows {
        let (comp, cnt) = row?;
        println!("   {}: {} partidos", comp, cnt);
    }

    Ok(())
}

fn main() -> Result<()> {
    migrate_json_to_sqlite("data/ifa_expanded_dataset.json", "data/ifa_database.db")
}