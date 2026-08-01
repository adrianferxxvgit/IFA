use serde::{Deserialize, Serialize};
use std::fs;
//

// === ESTRUCTURAS DE DATOS ===

#[derive(Debug, Deserialize)]
struct MatchInput {
    match_id: String,
    competition: String,
    home_team: String,
    away_team: String,
    home_metrics: TeamMetrics,
    away_metrics: TeamMetrics,
    market_data: MarketData,
    contextual_factors: ContextualFactors,
}

#[derive(Debug, Deserialize)]
struct TeamMetrics {
    xg_for_last_5: f64,
    xg_against_last_5: f64,
    ppda: f64,
    direct_transition_success_rate: f64,
    key_player_availability: f64,
    possession_rate: f64,
    shots_on_target_per_game: f64,
    leadership_crisis_level: f64,
}

#[derive(Debug, Deserialize)]
struct MarketData {
    home_win_odds: f64,
    draw_odds: f64,
    away_win_odds: f64,
}

#[derive(Debug, Deserialize)]
struct ContextualFactors {
    odds_discrepancy_factor: f64,
    geography_travel_factor: f64,
    competitive_pressure: f64,
    away_specific_underperformance: f64,
    historical_volatility: f64,
    market_movement_anomaly: f64,
}

#[derive(Debug, Clone)]
struct Indicator {
    name: String,
    value: f64,
    uncertainty: f64,
}

#[derive(Debug)]
struct CompetitiveProfile {
    team_name: String,
    offensive_pressure: Indicator,
    defensive_solidity: Indicator,
    structural_transitions: Indicator,
    ice: f64,
    leadership_crisis_level: f64,
}

#[derive(Debug)]
struct DimensionalComparison {
    dimension: String,
    delta: f64,
    combined_uncertainty: f64,
    dominance: String,
}

// === FUNCIONES DE NORMALIZACIÓN ===

fn normalize_type_a(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

fn normalize_type_b(value: f64, min: f64, max: f64) -> f64 {
    1.0 - ((value - min) / (max - min)).clamp(0.0, 1.0)
}

fn normalize_type_c(value: f64, optimal: f64, tolerance: f64) -> f64 {
    1.0 - ((value - optimal).abs() / tolerance).clamp(0.0, 1.0)
}

// === CONSTRUCCIÓN DE PERFILES ===

fn build_profile(team_name: &str, metrics: &TeamMetrics) -> CompetitiveProfile {
    // Normalización de componentes ofensivos
    let xg_component = normalize_type_a(metrics.xg_for_last_5, 0.0, 3.0);
    let shots_component = normalize_type_a(metrics.shots_on_target_per_game, 0.0, 8.0);
    let off_pressure_val = (xg_component * 0.4) + (shots_component * 0.3) + 
                           (metrics.direct_transition_success_rate * 0.3);

    // Normalización de componentes defensivos
    let xg_against_component = normalize_type_b(metrics.xg_against_last_5, 0.0, 2.5);
    let ppda_component = normalize_type_b(metrics.ppda, 5.0, 18.0);
    let def_solidity_val = (xg_against_component * 0.6) + (ppda_component * 0.4);

    // Normalización de transiciones
    let transitions_val = normalize_type_c(metrics.direct_transition_success_rate, 0.85, 0.15);

    // Cálculo de incertidumbre con penalización por ICD
    let availability_penalty = (1.0 - metrics.key_player_availability) * 0.12;
    let volatility_penalty = (1.0 - metrics.possession_rate.abs() - 0.5).abs() * 0.08;
    let crisis_uncertainty_penalty = metrics.leadership_crisis_level * 0.15;
    let total_uncertainty = 0.04 + availability_penalty + volatility_penalty + crisis_uncertainty_penalty;

    // Cálculo del ICE con depreciación por ICD
    let consistency_factor = 1.0 - ((metrics.xg_for_last_5 - 1.5).abs() / 1.5).min(1.0);
    let base_ice = (metrics.key_player_availability * 0.7) + (consistency_factor * 0.3);
    let ice = (base_ice * (1.0 - (metrics.leadership_crisis_level * 0.4))).clamp(0.0, 1.0);

    CompetitiveProfile {
        team_name: team_name.to_string(),
        offensive_pressure: Indicator {
            name: "Presión Ofensiva".to_string(),
            value: off_pressure_val.clamp(0.0, 1.0),
            uncertainty: total_uncertainty,
        },
        defensive_solidity: Indicator {
            name: "Solidez Defensiva".to_string(),
            value: def_solidity_val.clamp(0.0, 1.0),
            uncertainty: total_uncertainty,
        },
        structural_transitions: Indicator {
            name: "Transiciones".to_string(),
            value: transitions_val.clamp(0.0, 1.0),
            uncertainty: total_uncertainty + 0.05,
        },
        ice,
        leadership_crisis_level: metrics.leadership_crisis_level,
    }
}

// === EVALUACIÓN DE DOMINANCIA DIMENSIONAL ===

fn evaluate_matchup(home: &CompetitiveProfile, away: &CompetitiveProfile) -> Vec<DimensionalComparison> {
    let mut comparisons = Vec::new();

    // Dimensión 1: Ofensiva Local vs Defensiva Visitante
    let delta1 = home.offensive_pressure.value - away.defensive_solidity.value;
    let sigma1 = (home.offensive_pressure.uncertainty.powi(2) + away.defensive_solidity.uncertainty.powi(2)).sqrt();
    let dominance1 = if delta1 > sigma1 { "Home Dominance" } 
                     else if delta1 < -sigma1 { "Away Dominance" } 
                     else { "Statistical Equilibrium" };
    comparisons.push(DimensionalComparison {
        dimension: "Offensive vs Defensive".to_string(),
        delta: delta1,
        combined_uncertainty: sigma1,
        dominance: dominance1.to_string(),
    });

    // Dimensión 2: Transiciones Estructurales
    let delta2 = home.structural_transitions.value - away.structural_transitions.value;
    let sigma2 = (home.structural_transitions.uncertainty.powi(2) + away.structural_transitions.uncertainty.powi(2)).sqrt();
    let dominance2 = if delta2 > sigma2 { "Home Dominance" } 
                     else if delta2 < -sigma2 { "Away Dominance" } 
                     else { "Statistical Equilibrium" };
    comparisons.push(DimensionalComparison {
        dimension: "Structural Transitions".to_string(),
        delta: delta2,
        combined_uncertainty: sigma2,
        dominance: dominance2.to_string(),
    });

    // Dimensión 3: Defensiva Local vs Ofensiva Visitante
    let delta3 = home.defensive_solidity.value - away.offensive_pressure.value;
    let sigma3 = (home.defensive_solidity.uncertainty.powi(2) + away.offensive_pressure.uncertainty.powi(2)).sqrt();
    let dominance3 = if delta3 > sigma3 { "Home Dominance" } 
                     else if delta3 < -sigma3 { "Away Dominance" } 
                     else { "Statistical Equilibrium" };
    comparisons.push(DimensionalComparison {
        dimension: "Defensive vs Offensive".to_string(),
        delta: delta3,
        combined_uncertainty: sigma3,
        dominance: dominance3.to_string(),
    });

    comparisons
}

// === CÁLCULO DE ÍNDICES ===

fn calculate_ib(context: &ContextualFactors) -> f64 {
    let ib_raw = (0.25 * context.odds_discrepancy_factor) +
                 (0.20 * context.geography_travel_factor) +
                 (0.15 * context.competitive_pressure) +
                 (0.15 * context.away_specific_underperformance) +
                 (0.15 * context.historical_volatility) +
                 (0.10 * context.market_movement_anomaly);
    (ib_raw * 100.0).clamp(0.0, 100.0)
}

fn calculate_iev(home: &CompetitiveProfile, away: &CompetitiveProfile, market: &MarketData) -> (f64, f64, f64) {
    // Probabilidad implícita del mercado
    let p_market_home = 1.0 / market.home_win_odds;
    let p_market_away = 1.0 / market.away_win_odds;

    // Probabilidad estimada por IFA (basada en ICE y dominancia)
    let total_ice = home.ice + away.ice;
    let p_ifa_home = (home.ice / total_ice) * 0.85 + 0.075;
    let p_ifa_away = (away.ice / total_ice) * 0.85 + 0.075;

    let iev_home = p_ifa_home - p_market_home;
    let iev_away = p_ifa_away - p_market_away;

    (iev_home, iev_away, p_ifa_home)
}

// === GENERACIÓN DE INFORME ===

fn generate_report(input: &MatchInput, home: &CompetitiveProfile, away: &CompetitiveProfile, 
                   comparisons: &[DimensionalComparison], ib: f64, iev_home: f64, iev_away: f64) -> String {
    let mut report = String::new();

    report.push_str(&format!("# IFA Scientific Report: {} vs {}\n\n", input.home_team, input.away_team));
    report.push_str(&format!("**Competition**: {}\n", input.competition));
    report.push_str(&format!("**Match ID**: {}\n\n", input.match_id));

    report.push_str("## 1. Executive Summary\n\n");
    report.push_str("This report presents a rigorous competitive analysis using the IFA 6.1 Alpha framework, ");
    report.push_str("incorporating real-time metrics, institutional stability assessment, and market value evaluation.\n\n");

    report.push_str("## 2. Competitive Profiles\n\n");
    report.push_str(&format!("### 2.1 Home Team: {}\n\n", home.team_name));
    report.push_str("| Indicator | Value | Uncertainty (σ) |\n");
    report.push_str("| :--- | :---: | :---: |\n");
    report.push_str(&format!("| Offensive Pressure | {:.3} | ±{:.3} |\n", home.offensive_pressure.value, home.offensive_pressure.uncertainty));
    report.push_str(&format!("| Defensive Solidity | {:.3} | ±{:.3} |\n", home.defensive_solidity.value, home.defensive_solidity.uncertainty));
    report.push_str(&format!("| Structural Transitions | {:.3} | ±{:.3} |\n", home.structural_transitions.value, home.structural_transitions.uncertainty));
    report.push_str(&format!("| **ICE (Structural Confidence)** | **{:.3}** | — |\n", home.ice));
    report.push_str(&format!("| **ICD (Leadership Crisis)** | **{:.2}** | — |\n\n", home.leadership_crisis_level));

        report.push_str(&format!("### 2.2 Away Team: {}\n\n", away.team_name));
    report.push_str("| Indicator | Value | Uncertainty (σ) |\n");
    report.push_str("| :--- | :---: | :---: |\n");
    report.push_str(&format!("| Offensive Pressure | {:.3} | ±{:.3} |\n", away.offensive_pressure.value, away.offensive_pressure.uncertainty));
    report.push_str(&format!("| Defensive Solidity | {:.3} | ±{:.3} |\n", away.defensive_solidity.value, away.defensive_solidity.uncertainty));
    report.push_str(&format!("| Structural Transitions | {:.3} | ±{:.3} |\n", away.structural_transitions.value, away.structural_transitions.uncertainty));
    report.push_str(&format!("| **ICE (Structural Confidence)** | **{:.3}** | — |\n", away.ice));
    report.push_str(&format!("| **ICD (Leadership Crisis)** | **{:.2}** | — |\n\n", away.leadership_crisis_level));

    report.push_str("## 3. Dimensional Dominance Analysis\n\n");
    report.push_str("The model evaluates three competitive dimensions. A dimension is considered 'dominated' when |Δ| > σ_total.\n\n");
    report.push_str("| Dimension | Δ (Home - Away) | σ_total | Inference |\n");
    report.push_str("| :--- | :---: | :---: | :--- |\n");
    for comp in comparisons {
        report.push_str(&format!("| {} | {:.3} | ±{:.3} | **{}** |\n", comp.dimension, comp.delta, comp.combined_uncertainty, comp.dominance));
    }
    report.push('\n');

    let home_dominances = comparisons.iter().filter(|c| c.dominance == "Home Dominance").count();
    let away_dominances = comparisons.iter().filter(|c| c.dominance == "Away Dominance").count();
    report.push_str(&format!("**Result**: Home Dominances: {} | Away Dominances: {} | Equilibriums: {}\n\n", 
        home_dominances, away_dominances, 3 - home_dominances - away_dominances));

    report.push_str("## 4. Risk & Value Indices\n\n");
    report.push_str("### 4.1 Upset Index (IB)\n\n");
    report.push_str(&format!("**IB = {:.1} / 100**\n\n", ib));
    let ib_classification = if ib < 30.0 { "Low (Standard Confidence)" }
                            else if ib < 50.0 { "Moderate (Standard Precaution)" }
                            else if ib < 70.0 { "High (High Volatility Probability)" }
                            else { "Extreme (Chaos Scenario)" };
    report.push_str(&format!("**Classification**: {}\n\n", ib_classification));

    report.push_str("### 4.2 Expected Value Index (IEV)\n\n");
    report.push_str(&format!("| Team | IEV | Interpretation |\n"));
    report.push_str("| :--- | :---: | :--- |\n");
    let home_interpretation = if iev_home > 0.05 { "Positive Value (Market Underestimates)" }
                              else if iev_home < -0.05 { "Negative Value (Market Overestimates)" }
                              else { "Neutral (Market Aligned)" };
    let away_interpretation = if iev_away > 0.05 { "Positive Value (Market Underestimates)" }
                              else if iev_away < -0.05 { "Negative Value (Market Overestimates)" }
                              else { "Neutral (Market Aligned)" };
    report.push_str(&format!("| {} | {:.1}% | {} |\n", input.home_team, iev_home * 100.0, home_interpretation));
    report.push_str(&format!("| {} | {:.1}% | {} |\n\n", input.away_team, iev_away * 100.0, away_interpretation));

    report.push_str("## 5. Leadership Crisis Impact (ICD)\n\n");
    report.push_str("The Leadership Crisis Index directly inflates predictive uncertainty (σ) and depreciates the Structural Confidence Index (ICE).\n\n");
    report.push_str("| Team | ICD Level | σ Penalty | ICE Reduction |\n");
    report.push_str("| :--- | :---: | :---: | :---: |\n");
    report.push_str(&format!("| {} | {:.2} | +{:.0}% | -{:.0}% |\n", home.team_name, home.leadership_crisis_level, home.leadership_crisis_level * 15.0, home.leadership_crisis_level * 40.0));
    report.push_str(&format!("| {} | {:.2} | +{:.0}% | -{:.0}% |\n\n", away.team_name, away.leadership_crisis_level, away.leadership_crisis_level * 15.0, away.leadership_crisis_level * 40.0));

    report.push_str("## 6. Scientific Conclusion\n\n");
    let prediction = if home_dominances > away_dominances {
        format!("{} (Home)", input.home_team)
    } else if away_dominances > home_dominances {
        format!("{} (Away)", input.away_team)
    } else {
        "Draw / Statistical Equilibrium".to_string()
    };
    report.push_str(&format!("**Model Prediction**: {}\n\n", prediction));
    report.push_str("The IFA 6.1 Alpha model has processed real competitive metrics through rigorous uncertainty propagation. ");
    report.push_str("All inferences are falsifiable and decoupled from market noise.\n\n");
    report.push_str("---\n*Report generated by IFA 6.1 Alpha - Integral Football Analysis*\n");

    report
}

// === MAIN ===

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================================================");
    println!("  INTEGRAL FOOTBALL ANALYSIS (IFA) v6.1 Alpha");
    println!("  Motor de Inferencia Competitiva con Gestión de Riesgo");
    println!("==========================================================\n");

    // Leer el archivo de entrada
    let input_path = "sample_match.json";
    println!("📂 Leyendo datos de entrada: {}", input_path);
    let content = fs::read_to_string(input_path)?;
    let input: MatchInput = serde_json::from_str(&content)?;
    println!("✅ Datos parseados correctamente\n");

    // Construir perfiles competitivos
    println!("🔬 Construyendo perfiles competitivos...");
    let home_profile = build_profile(&input.home_team, &input.home_metrics);
    let away_profile = build_profile(&input.away_team, &input.away_metrics);
    println!("   {} (ICE: {:.3}, ICD: {:.2})", home_profile.team_name, home_profile.ice, home_profile.leadership_crisis_level);
    println!("   {} (ICE: {:.3}, ICD: {:.2})\n", away_profile.team_name, away_profile.ice, away_profile.leadership_crisis_level);

    // Evaluar dominancia dimensional
    println!("️  Evaluando dominancia dimensional...");
    let comparisons = evaluate_matchup(&home_profile, &away_profile);
    for comp in &comparisons {
        println!("   {}: Δ={:.3}, σ={:.3} → {}", comp.dimension, comp.delta, comp.combined_uncertainty, comp.dominance);
    }
    println!();

    // Calcular índices
    println!("📊 Calculando índices de riesgo y valor...");
    let ib = calculate_ib(&input.contextual_factors);
    let (iev_home, iev_away, _) = calculate_iev(&home_profile, &away_profile, &input.market_data);
    println!("   IB (Upset Index): {:.1}/100", ib);
    println!("   IEV Home: {:.1}%", iev_home * 100.0);
    println!("   IEV Away: {:.1}%\n", iev_away * 100.0);

    // Generar informe
    println!("📝 Generando informe científico...");
    let report = generate_report(&input, &home_profile, &away_profile, &comparisons, ib, iev_home, iev_away);
    
    let output_filename = format!("IFA_Report_{}_vs_{}.md", 
        input.home_team.replace(' ', "_"), 
        input.away_team.replace(' ', "_"));
    fs::write(&output_filename, &report)?;
    
    println!("\n ARTEFACTO CIENTÍFICO GENERADO: {}", output_filename);
    println!("==========================================================\n");

    Ok(())
}