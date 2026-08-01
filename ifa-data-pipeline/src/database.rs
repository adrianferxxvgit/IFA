use crate::models::MatchRecord;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        if let Some(parent) = Path::new(db_path).parent() {
            fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS matches (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                external_id TEXT UNIQUE,
                provider TEXT NOT NULL,
                competition TEXT NOT NULL,
                season INTEGER NOT NULL,
                match_date TEXT NOT NULL,
                home_team TEXT NOT NULL,
                away_team TEXT NOT NULL,
                home_score INTEGER,
                away_score INTEGER,
                result TEXT,
                home_xg REAL,
                away_xg REAL,
                home_xg_against REAL,
                away_xg_against REAL,
                home_possession REAL,
                away_possession REAL,
                home_shots_on_target INTEGER,
                away_shots_on_target INTEGER,
                home_ppda REAL,
                away_ppda REAL,
                created_at TEXT DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_matches_competition ON matches(competition);
            CREATE INDEX IF NOT EXISTS idx_matches_date ON matches(match_date);"
        )?;
        Ok(())
    }

    pub fn insert_match(&self, m: &MatchRecord) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO matches 
             (external_id, provider, competition, season, match_date, 
              home_team, away_team, home_score, away_score, result,
              home_xg, away_xg, home_xg_against, away_xg_against,
              home_possession, away_possession, 
              home_shots_on_target, away_shots_on_target,
              home_ppda, away_ppda)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                     ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            rusqlite::params![
                m.external_id, m.provider, m.competition, m.season, m.match_date,
                m.home_team, m.away_team, m.home_score, m.away_score, m.result,
                m.home_xg, m.away_xg, m.home_xg_against, m.away_xg_against,
                m.home_possession, m.away_possession,
                m.home_shots_on_target, m.away_shots_on_target,
                m.home_ppda, m.away_ppda
            ],
        )?;
        Ok(())
    }

    pub fn count_matches(&self) -> Result<usize> {
        let count: usize = self.conn.query_row(
            "SELECT COUNT(*) FROM matches",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn list_competitions(&self) -> Result<Vec<(String, usize)>> {
        let mut stmt = self.conn.prepare(
            "SELECT competition, COUNT(*) FROM matches GROUP BY competition ORDER BY COUNT(*) DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, usize>(1)?))
        })?;

        let mut competitions = Vec::new();
        for row in rows {
            competitions.push(row?);
        }
        Ok(competitions)
    }
}
