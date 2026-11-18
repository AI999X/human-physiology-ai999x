//! Human Physiology: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p human-physiology-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&human_physiology_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "hp_red_cell_indices", "inputs": {"Hct": "0.45", "Hb": "15", "RBC": "5"}}));
    println!();
    show(json!({"key": "hp_hydrostatic_weighing", "inputs": {"mass_air": "70 kg", "mass_water": "3 kg", "water_density": "0.9957 g/mL", "RV": "1.2 L"}}));
    println!();
    show(json!({"key": "hp_body_fluid_compartments", "inputs": {"weight": "70 kg"}}));
    println!();
    show(json!({"key": "hp_bernoulli_pressure_drop", "inputs": {"v1": "1 m/s", "v2": "4 m/s", "rho": "1060 kg/m^3"}}));
    println!();
    show(json!({"key": "hp_bmr_harris_benedict", "inputs": {"weight": "70 kg", "height": "175 cm", "age": "30", "sex": "male", "version": "original"}}));
    println!();
    show(json!({"key": "hp_membrane_charge", "inputs": {"Cm": "1 uF/cm^2", "dV": "100 mV", "area": "1000 um^2"}}));
    println!();
    show(json!({"key": "hp_plasma_osmolality", "inputs": {"Na": "140 mmol/L", "glucose": "90", "BUN": "14"}}));
    println!();
    show(json!({"key": "hp_alveolar_gas_equation", "inputs": {"FiO2": "0.21", "Patm": "760 mmHg", "PH2O": "47 mmHg", "PaCO2": "40 mmHg", "RQ": "0.8"}}));
    println!();
    show(json!({"key": "hp_body_fat_from_density", "inputs": {"density": "1.05 g/mL"}}));
    println!();
    show(json!({"key": "hp_gibbs_donnan", "inputs": {"A": "20", "c": "150 mmol/L"}}));
    println!();
    show(json!({"key": "hp_cardiac_index", "inputs": {"CO": "5.04 L/min", "BSA": "1.8 m^2", "HR": "72"}}));
    println!();
    show(json!({"key": "hp_bmr_katch_mcardle", "inputs": {"weight": "70 kg", "body_fat": "0.2"}}));
    println!();
    show(json!({"key": "hp_conduction_velocity_scaling", "inputs": {"v1": "60 m/s", "d1": "10 um", "d2": "5 um", "fiber": "myelinated"}}));
    println!();
    show(json!({"key": "hp_cockcroft_gault", "inputs": {"age": "60", "weight": "72 kg", "Scr": "1", "sex": "male"}}));
    println!();
    show(json!({"key": "hp_alveolar_ventilation_co2", "inputs": {"VCO2": "200 mL/min", "PaCO2": "40 mmHg"}}));
}
