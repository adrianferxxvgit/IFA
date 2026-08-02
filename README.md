# ⚽ Integral Football Analysis (IFA) v6.1 Alpha

> **Motor de Inferencia Competitiva con Calibración Empírica**  
> Accuracy predictiva: **55.74%** (N=967 partidos reales, 5 ligas europeas)

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![StatsBomb](https://img.shields.io/badge/Data-StatsBomb_Open_Data-blue.svg)](https://github.com/statsbomb/open-data)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 🎯 ¿Qué es IFA?

IFA es un sistema de análisis deportivo independiente que predice resultados de partidos de fútbol utilizando un modelo multidimensional calibrado empíricamente sobre datos reales de StatsBomb.

### Arquitectura

```text
┌─────────────────────────────────────────────────────────────┐
│  1. Pipeline de Datos (PowerShell + Rust)                   │
│     ↓ Descarga de StatsBomb Open Data                       │
│     ↓ Cálculo de xG evento por evento                       │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  2. Motor IFA 6.1 Alpha (Rust)                              │
│     ↓ Perfiles Competitivos (ICE, ICD)                      │
│     ↓ Dominancia Dimensional (3 ejes)                       │
│     ↓ Calibración Empírica (Grid Search)                    │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  3. Backtesting & Validación                                │
│     ↓ N=967 partidos reales                                 │
│     ↓ Accuracy: 55.74%                                      │
└─────────────────────────────────────────────────────────────┘
