use clap::{Parser, Subcommand};
use ifa_data_pipeline::database::Database;
use ifa_data_pipeline::models::MatchRecord;
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
    /// Importar partidos desde un archivo JSON (formato MatchRecord)
    ImportJson {
        /// Ruta al archivo JSON
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

    println!("📂 Leyendo archivo: {}", file);
    let content = fs::read_to_string(path)?;
    
    let records: Vec<MatchRecord> = serde_json::from_str(&content)?;
    println!("✅ {} registros parseados del JSON", records.len());

    let mut inserted = 0;
    let mut errors = 0;

    for record in &records {
        match db.insert_match(record) {
            Ok(_) => inserted += 1,
            Err(e) => {
                eprintln!("   ⚠️  Error insertando {}: {}", record.external_id, e);
                errors += 1;
            }
        }
    }

    println!("\n📊 Resumen de importación:");
    println!("  ✅ Insertados: {}", inserted);
    if errors > 0 {
        println!("  ⚠️  Errores: {}", errors);
    }
    println!("  💾 Total en base de datos: {}", db.count_matches()?);

    Ok(())
}

fn show_stats(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let total = db.count_matches()?;
    println!("\n📊 Estadísticas de la Base de Datos IFA:");
    println!("  Total de partidos históricos: {}", total);
    
    let comps = db.list_competitions()?;
    if !comps.is_empty() {
        println!("\n  Desglose por competición:");
        for (comp, count) in comps {
            println!("    • {}: {} partidos", comp, count);
        }
    } else {
        println!("  (La base de datos está vacía)");
    }
    Ok(())
}