use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("==========================================================");
        println!("  IFA Database Query Client");
        println!("==========================================================");
        println!("\nUso: cargo run --bin ifa-query --release -- \"<SQL QUERY>\"\n");
        println!("Ejemplos:");
        println!("  cargo run --bin ifa-query --release -- \"SELECT COUNT(*) FROM matches\"");
        println!("  cargo run --bin ifa-query --release -- \"SELECT competition, COUNT(*) FROM matches GROUP BY competition ORDER BY 2 DESC LIMIT 10\"");
        println!("\nTablas disponibles:");
        println!("  - matches (29,510 partidos)");
        println!("  - predictions (predicciones IFA)");
        println!("  - model_metrics (métricas del modelo)");
        return Ok(());
    }

    let query = &args[1];
    let conn = Connection::open("data/ifa_database.db")?;

    println!("🔍 Ejecutando query:\n   {}\n", query);
    println!("{}", "=".repeat(80));

    let mut stmt = conn.prepare(query)?;
    let column_count = stmt.column_count();
    
    // Imprimir encabezados
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    println!("{}", column_names.join(" | "));
    println!("{}", "-".repeat(80));

    // Imprimir filas
    let rows = stmt.query_map([], |row| {
        let mut values = Vec::new();
        for i in 0..column_count {
            let value: String = match row.get::<_, rusqlite::types::Value>(i)? {
                rusqlite::types::Value::Null => "NULL".to_string(),
                rusqlite::types::Value::Integer(i) => i.to_string(),
                rusqlite::types::Value::Real(f) => format!("{:.2}", f),
                rusqlite::types::Value::Text(s) => s,
                rusqlite::types::Value::Blob(_) => "[BLOB]".to_string(),
            };
            values.push(value);
        }
        Ok(values.join(" | "))
    })?;

    let mut row_count = 0;
    for row in rows {
        println!("{}", row?);
        row_count += 1;
    }

    println!("{}", "=".repeat(80));
    println!("✅ {} filas devueltas", row_count);

    Ok(())
}