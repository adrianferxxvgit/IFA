use clap::{Parser, Subcommand};
use ifa_data_pipeline::{database::Database, providers::creativesdev::CreativesDevProvider};
use ifa_data_pipeline::models::MatchRecord;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "ifa-fetch")]
#[command(about = "IFA Data Pipeline - Gestión de datos históricos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Descargar partidos desde la API de RapidAPI (CreativesDev)
    Fetch {
        #[arg(short, long, default_value = "112")]
        league_id: i32,
        #[arg(short, long, default_value = "2024")]
        season: i32,
    },
    /// Importar partidos desde un archivo JSON
    ImportJson {
        file: String,
    },
    /// Mostrar estadísticas de la base de datos
    Stats,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let db = Database::new("data/ifa_history.db")?;

    match cli.command {
        Commands::Fetch { league_id, season } => {
            let api_key = env::var("CREATIVESDEV_API_KEY")
                .expect("ERROR: Configura CREATIVESDEV_API_KEY en el archivo .env o en la sesión");
            
            println!("Conectando a CreativesDev API (League: {}, Season: {})...", league_id, season);
            let provider = CreativesDevProvider::new(api_key);
            
            match provider.fetch_fixtures(league_id, season).await {
                Ok(records) => {
                    println!("{} partidos procesados de la respuesta.", records.len());
                    let mut inserted = 0;
                    for record in records {
                        if let Err(e) = db.insert_match(&record) {
                            eprintln!("   Error insertando {}: {}", record.external_id, e);
                        } else {
                            inserted += 1;
                        }
                    }
                    println!("{} partidos guardados exitosamente en data/ifa_history.db", inserted);
                }
                Err(e) => {
                    eprintln!("Error al obtener datos: {}", e);
                }
            }
        }
        Commands::ImportJson { file } => {
            import_json(&db, &file)?;
        }
        Commands::Stats => {
            show_stats(&db)?;
        }
    }
    Ok(())
}

fn import_json(db: &Database, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file);
    if !path.exists() {
        return Err(format!("Archivo no encontrado: {}", file).into());
    }
    println!("Leyendo archivo: {}", file);
    let content = fs::read_to_string(path)?;
    let records: Vec<MatchRecord> = serde_json::from_str(&content)?;
    println!("{} registros parseados del JSON", records.len());
    let mut inserted = 0;
    for record in &records {
        if db.insert_match(record).is_ok() { inserted += 1; }
    }
    println!("Insertados: {}", inserted);
    Ok(())
}

fn show_stats(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let total = db.count_matches()?;
    println!("\nEstadísticas de la Base de Datos IFA:");
    println!("  Total de partidos históricos: {}", total);
    let comps = db.list_competitions()?;
    if !comps.is_empty() {
        println!("\n  Desglose por competición:");
        for (comp, count) in comps {
            println!("    • {}: {} partidos", comp, count);
        }
    }
    Ok(())
}