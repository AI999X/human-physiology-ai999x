//! human_physiology: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/human_physiology/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "hp_aa_gradient",
    "hp_alveolar_gas_equation",
    "hp_alveolar_laplace_pressure",
    "hp_alveolar_ventilation_co2",
    "hp_anion_gap",
    "hp_atwater_energy",
    "hp_barometric_altitude",
    "hp_bernoulli_pressure_drop",
    "hp_bicarbonate_from_ph",
    "hp_blood_velocity_area",
    "hp_bmi",
    "hp_bmr_harris_benedict",
    "hp_bmr_katch_mcardle",
    "hp_bmr_mifflin_st_jeor",
    "hp_body_fat_from_density",
    "hp_body_fluid_compartments",
    "hp_body_surface_area",
    "hp_bohr_dead_space",
    "hp_cardiac_index",
    "hp_cardiac_output",
    "hp_chord_conductance",
    "hp_cockcroft_gault",
    "hp_conduction_velocity_scaling",
    "hp_creatinine_clearance_timed",
    "hp_ecg_heart_rate",
    "hp_egfr_ckd_epi_2021",
    "hp_egfr_ckd_epi_2021_cys",
    "hp_elimination_half_life",
    "hp_fev1_fvc_ratio",
    "hp_fick_cardiac_output",
    "hp_fick_diffusion",
    "hp_fick_saturations",
    "hp_filtration_fraction",
    "hp_fluid_shift",
    "hp_fractional_excretion",
    "hp_free_water_clearance",
    "hp_gas_volume_conversion",
    "hp_ghk_voltage",
    "hp_gibbs_donnan",
    "hp_glucose_titration",
    "hp_heart_rate_reserve",
    "hp_hill_fit",
    "hp_hill_force_velocity",
    "hp_hill_po2",
    "hp_hill_saturation",
    "hp_hydrostatic_pressure",
    "hp_hydrostatic_weighing",
    "hp_ionic_current",
    "hp_laplace_wall_tension",
    "hp_lean_body_mass_boer",
    "hp_length_constant",
    "hp_lung_capacities",
    "hp_mean_arterial_pressure",
    "hp_membrane_charge",
    "hp_membrane_time_constant",
    "hp_met_energy",
    "hp_minute_ventilation",
    "hp_moens_korteweg",
    "hp_muscle_max_power",
    "hp_nernst_potential",
    "hp_nerve_conduction_velocity",
    "hp_osmotic_pressure",
    "hp_oxygen_consumption",
    "hp_oxygen_content",
    "hp_oxygen_delivery",
    "hp_pah_renal_flow",
    "hp_pco2_from_ph",
    "hp_ph_henderson_hasselbalch",
    "hp_plasma_osmolality",
    "hp_poiseuille_flow",
    "hp_poiseuille_resistance",
    "hp_pulmonary_vascular_resistance",
    "hp_qp_qs",
    "hp_qrs_axis",
    "hp_qt_correction",
    "hp_radius_flow_ratio",
    "hp_red_cell_indices",
    "hp_renal_clearance",
    "hp_respiratory_mechanics",
    "hp_respiratory_time_constant",
    "hp_severinghaus_po2",
    "hp_severinghaus_saturation",
    "hp_shunt_fraction",
    "hp_solution_osmolarity",
    "hp_starling_filtration",
    "hp_strength_duration",
    "hp_stroke_volume_ef",
    "hp_stroke_work",
    "hp_systemic_vascular_resistance",
    "hp_teichholz_volumes",
    "hp_tubular_handling",
    "hp_vascular_compliance",
    "hp_vessel_reynolds",
    "hp_vessels_series_parallel",
    "hp_virtual_po2",
    "hp_watson_total_body_water",
    "hp_weir_energy_expenditure",
    "hp_womersley_number",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "hp_aa_gradient" => {
            let r = s_1({ let mut d = vec![]; for k in ["FiO2", "Patm", "PH2O", "PaCO2", "PaO2", "RQ"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_38(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_gas_equation" => {
            let r = s_70({ let mut d = vec![]; for k in ["FiO2", "Patm", "PH2O", "PaCO2", "RQ"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_105(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_laplace_pressure" => {
            let r = s_137({ let mut d = vec![]; for k in ["gamma", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_157(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_ventilation_co2" => {
            let r = s_189({ let mut d = vec![]; for k in ["VCO2", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_anion_gap" => {
            let r = s_241({ let mut d = vec![]; for k in ["Na", "Cl", "HCO3", "albumin"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_261(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_atwater_energy" => {
            let r = s_293({ let mut d = vec![]; for k in ["carbohydrate", "fat", "protein", "alcohol"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_315(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_barometric_altitude" => {
            let r = s_347({ let mut d = vec![]; for k in ["h", "FiO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_376(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bernoulli_pressure_drop" => {
            let r = s_408({ let mut d = vec![]; for k in ["v1", "v2", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bicarbonate_from_ph" => {
            let r = s_464({ let mut d = vec![]; for k in ["pH", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_487(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_blood_velocity_area" => {
            let r = s_519({ let mut d = vec![]; for k in ["Q", "A"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmi" => {
            let r = s_571({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_591(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_harris_benedict" => {
            let r = s_623({ let mut d = vec![]; for k in ["weight", "height", "age", "sex", "version"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_653(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_katch_mcardle" => {
            let r = s_685({ let mut d = vec![]; for k in ["weight", "body_fat"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_714(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_mifflin_st_jeor" => {
            let r = s_746({ let mut d = vec![]; for k in ["weight", "height", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_772(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fat_from_density" => {
            let r = s_804({ let mut d = vec![]; for k in ["density"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_824(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fluid_compartments" => {
            let r = s_856({ let mut d = vec![]; for k in ["weight", "TBW_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_surface_area" => {
            let r = s_917({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bohr_dead_space" => {
            let r = s_969({ let mut d = vec![]; for k in ["PaCO2", "PECO2", "VT"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_993(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_index" => {
            let r = s_1025({ let mut d = vec![]; for k in ["CO", "BSA", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1047(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_output" => {
            let r = s_1079({ let mut d = vec![]; for k in ["HR", "SV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1099(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_chord_conductance" => {
            let r = s_1131({ let mut d = vec![]; for k in ["gK", "gNa", "gCl", "EK", "ENa", "ECl"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1154(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cockcroft_gault" => {
            let r = s_1186({ let mut d = vec![]; for k in ["age", "weight", "Scr", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1213(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_conduction_velocity_scaling" => {
            let r = s_1245({ let mut d = vec![]; for k in ["v1", "d1", "d2", "fiber"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1269(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_creatinine_clearance_timed" => {
            let r = s_1301({ let mut d = vec![]; for k in ["Ucr", "Vurine", "hours", "Pcr", "BSA"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ecg_heart_rate" => {
            let r = s_1353({ let mut d = vec![]; for k in ["RR", "small_boxes", "large_boxes", "speed"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021" => {
            let r = f_egfr_ckd_epi_2021(match v.get("Scr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sex") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1438(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021_cys" => {
            let r = s_1470({ let mut d = vec![]; for k in ["Scr", "Scys", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1499(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_elimination_half_life" => {
            let r = s_1531({ let mut d = vec![]; for k in ["CL", "Vd", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1555(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fev1_fvc_ratio" => {
            let r = s_1587({ let mut d = vec![]; for k in ["FEV1", "FVC"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1609(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_cardiac_output" => {
            let r = s_1641({ let mut d = vec![]; for k in ["VO2", "CaO2", "CvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1665(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_diffusion" => {
            let r = s_1697({ let mut d = vec![]; for k in ["D", "A", "dC", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_saturations" => {
            let r = s_1749({ let mut d = vec![]; for k in ["VO2", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_filtration_fraction" => {
            let r = s_1816({ let mut d = vec![]; for k in ["GFR", "RPF"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fluid_shift" => {
            let r = s_1870({ let mut d = vec![]; for k in ["TBW", "Posm", "V_added", "C_added", "ICF_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1902(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fractional_excretion" => {
            let r = f_fractional_excretion(match v.get("Ux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ucr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pcr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1948(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_free_water_clearance" => {
            let r = s_1980({ let mut d = vec![]; for k in ["V", "Uosm", "Posm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2000(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gas_volume_conversion" => {
            let r = s_2032({ let mut d = vec![]; for k in ["V", "T", "Patm", "PH2O"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2063(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ghk_voltage" => {
            let r = s_2095({ let mut d = vec![]; for k in ["PK", "PNa", "PCl", "K_o", "K_i", "Na_o", "Na_i", "Cl_o", "Cl_i", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2122(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gibbs_donnan" => {
            let r = s_2154({ let mut d = vec![]; for k in ["A", "c", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_glucose_titration" => {
            let r = s_2209({ let mut d = vec![]; for k in ["GFR", "Pglucose", "Tm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2231(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_heart_rate_reserve" => {
            let r = f_heart_rate_reserve(match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HR_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intensity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2289(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_fit" => {
            let r = s_2321({ let mut d = vec![]; for k in ["P1", "S1", "P2", "S2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2358(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_force_velocity" => {
            let r = s_2390({ let mut d = vec![]; for k in ["F0", "a", "b", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2414(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_po2" => {
            let r = s_2446({ let mut d = vec![]; for k in ["SO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_saturation" => {
            let r = s_2507({ let mut d = vec![]; for k in ["PO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_pressure" => {
            let r = s_2561({ let mut d = vec![]; for k in ["h", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2581(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_weighing" => {
            let r = s_2613({ let mut d = vec![]; for k in ["mass_air", "mass_water", "water_density", "RV", "GI"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2646(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ionic_current" => {
            let r = s_2678({ let mut d = vec![]; for k in ["g", "Vm", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_laplace_wall_tension" => {
            let r = s_2730({ let mut d = vec![]; for k in ["P", "r", "shape", "h"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2756(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lean_body_mass_boer" => {
            let r = s_2788({ let mut d = vec![]; for k in ["weight", "height", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2815(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_length_constant" => {
            let r = s_2847({ let mut d = vec![]; for k in ["Rm", "Ri", "d", "x"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2871(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lung_capacities" => {
            let r = s_2903({ let mut d = vec![]; for k in ["TV", "IRV", "ERV", "RV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2925(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_mean_arterial_pressure" => {
            let r = s_2957({ let mut d = vec![]; for k in ["SBP", "DBP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2979(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_charge" => {
            let r = s_3011({ let mut d = vec![]; for k in ["Cm", "dV", "area"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3033(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_time_constant" => {
            let r = s_3065({ let mut d = vec![]; for k in ["Rm", "Cm", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3089(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_met_energy" => {
            let r = s_3121({ let mut d = vec![]; for k in ["MET", "weight", "duration"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3141(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_minute_ventilation" => {
            let r = s_3173({ let mut d = vec![]; for k in ["VT", "RR", "VD"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3197(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_moens_korteweg" => {
            let r = s_3229({ let mut d = vec![]; for k in ["E", "h", "D", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_muscle_max_power" => {
            let r = s_3281({ let mut d = vec![]; for k in ["F0", "a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nernst_potential" => {
            let r = s_3333({ let mut d = vec![]; for k in ["z", "Cout", "Cin", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3362(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nerve_conduction_velocity" => {
            let r = s_3394({ let mut d = vec![]; for k in ["distance", "latency_proximal", "latency_distal"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3418(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_osmotic_pressure" => {
            let r = s_3450({ let mut d = vec![]; for k in ["osm", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_consumption" => {
            let r = s_3505({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_content" => {
            let r = s_3572({ let mut d = vec![]; for k in ["Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3604(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_delivery" => {
            let r = s_3636({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3667(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pah_renal_flow" => {
            let r = s_3699({ let mut d = vec![]; for k in ["U_PAH", "V", "P_PAH", "Hct", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3729(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pco2_from_ph" => {
            let r = s_3761({ let mut d = vec![]; for k in ["pH", "HCO3"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ph_henderson_hasselbalch" => {
            let r = s_3816({ let mut d = vec![]; for k in ["HCO3", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_plasma_osmolality" => {
            let r = s_3868({ let mut d = vec![]; for k in ["Na", "glucose", "BUN", "measured"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_flow" => {
            let r = s_3924({ let mut d = vec![]; for k in ["dP", "eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3944(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_resistance" => {
            let r = s_3976({ let mut d = vec![]; for k in ["eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3996(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pulmonary_vascular_resistance" => {
            let r = s_4028({ let mut d = vec![]; for k in ["mPAP", "PCWP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4052(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qp_qs" => {
            let r = f_qp_qs(match v.get("SaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpaO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4108(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qrs_axis" => {
            let r = f_qrs_axis(match v.get("lead_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("aVF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qt_correction" => {
            let r = s_4187({ let mut d = vec![]; for k in ["QT", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_radius_flow_ratio" => {
            let r = f_radius_flow_ratio(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4251(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_red_cell_indices" => {
            let r = f_red_cell_indices(match v.get("Hct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RBC") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4304(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_renal_clearance" => {
            let r = s_4336({ let mut d = vec![]; for k in ["U", "V", "P"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4358(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_mechanics" => {
            let r = s_4390({ let mut d = vec![]; for k in ["VT", "Ppeak", "Pplat", "PEEP", "flow"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_time_constant" => {
            let r = s_4447({ let mut d = vec![]; for k in ["R", "C", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4471(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_po2" => {
            let r = f_severinghaus_po2(match v.get("SO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4523(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_saturation" => {
            let r = s_4555({ let mut d = vec![]; for k in ["PO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4574(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_shunt_fraction" => {
            let r = f_shunt_fraction(match v.get("CcO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CvO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4623(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_solution_osmolarity" => {
            let r = s_4655({ let mut d = vec![]; for k in ["conc", "molar_mass", "i", "phi"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4685(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_starling_filtration" => {
            let r = s_4717({ let mut d = vec![]; for k in ["Pc", "Pi", "pic", "pii", "Kf", "sigma"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4748(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_strength_duration" => {
            let r = s_4780({ let mut d = vec![]; for k in ["rheobase", "chronaxie", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4800(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_volume_ef" => {
            let r = s_4832({ let mut d = vec![]; for k in ["EDV", "ESV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4856(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_work" => {
            let r = s_4888({ let mut d = vec![]; for k in ["SV", "MAP", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4910(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_systemic_vascular_resistance" => {
            let r = s_4942({ let mut d = vec![]; for k in ["MAP", "CVP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4966(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_teichholz_volumes" => {
            let r = s_4998({ let mut d = vec![]; for k in ["LVIDd", "LVIDs"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5021(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_tubular_handling" => {
            let r = s_5053({ let mut d = vec![]; for k in ["GFR", "Px", "Ux", "V"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5076(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vascular_compliance" => {
            let r = s_5108({ let mut d = vec![]; for k in ["dV", "dP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessel_reynolds" => {
            let r = s_5160({ let mut d = vec![]; for k in ["rho", "v", "D", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessels_series_parallel" => {
            let r = f_network_resistance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_virtual_po2" => {
            let r = s_5265({ let mut d = vec![]; for k in ["PO2", "T", "pH", "PCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5292(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_watson_total_body_water" => {
            let r = s_5324({ let mut d = vec![]; for k in ["age", "height", "weight", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5350(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_weir_energy_expenditure" => {
            let r = s_5382({ let mut d = vec![]; for k in ["VO2", "VCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_womersley_number" => {
            let r = s_5434({ let mut d = vec![]; for k in ["r", "HR", "rho", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_CMH2O() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(98.0665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_CO2_SOL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.03)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_E_CHARGE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_FARADAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212331001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_INF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(f64::INFINITY)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_K_ALV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(862.9470986637378)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MMHG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(133.322387415)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_O2_HB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_O2_SOL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.003)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.141592653589793)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_PK_CO2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.1)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_GAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.314462618)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(273.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T_BODY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(310.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_WOOD_DYN() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(79.993432449)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ISA_EXP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(5.255785959221189)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    if ({ let t5462 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t5462.truthy() { t5462 } else { { let t5461 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5461.truthy() { t5461 } else { { let t5460 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5460.truthy() { t5460 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t5463 = v_a.clone(); Py::Bool({ let t5464 = v_b.clone(); if py::eq(&t5463, &t5464) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t5465 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5465.truthy() { t5465 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5469 = py::not(&{ let t5466 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5466.truthy() { t5466 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5469.truthy() { t5469 } else { { let t5467 = py::len(&v_a.clone())?; Py::Bool({ let t5468 = py::len(&v_b.clone())?; if py::ne(&t5467, &t5468) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        v_w = Py::Float(0.0);
        for t5470 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t5471 = (t5470).items()?;
            if t5471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t5471[0].clone();
            v_y = t5471[1].clone();
            v_w = py::minmax(&[v_w.clone(), f__diff(v_x.clone(), v_y.clone())?], true)?;
        }
        return Ok(v_w.clone());
    }
    if ({ let t5476 = { let t5472 = v_a.clone(); Py::Bool({ let t5473 = Py::None; if py::is(&t5472, &t5473) { true } else { false } }) }; if t5476.truthy() { t5476 } else { { let t5474 = v_b.clone(); Py::Bool({ let t5475 = Py::None; if py::is(&t5474, &t5475) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5477 = v_a.clone(); Py::Bool({ let t5478 = v_b.clone(); if py::is(&t5477, &t5478) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t5480: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5479 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5479[0].clone();
            v_b = t5479[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5480 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t5481 = v_a.clone(); Py::Bool({ let t5482 = v_b.clone(); if py::eq(&t5481, &t5482) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5483 = v_d.clone(); Py::Bool({ let t5484 = v_d.clone(); if py::ne(&t5483, &t5484) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py, mut v_name: Py, mut v_open_lo: Py, mut v_open_hi: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t4733 = (if (v_open_lo.clone()).truthy() { { let t4725 = v_v.clone(); Py::Bool({ let t4726 = Py::Int(0i64); if py::gt(&t4725, &t4726)? { true } else { false } }) } } else { { let t4727 = v_v.clone(); Py::Bool({ let t4728 = Py::Int(0i64); if py::ge(&t4727, &t4728)? { true } else { false } }) } }); if !t4733.truthy() { t4733 } else { (if (v_open_hi.clone()).truthy() { { let t4729 = v_v.clone(); Py::Bool({ let t4730 = Py::Int(1i64); if py::lt(&t4729, &t4730)? { true } else { false } }) } } else { { let t4731 = v_v.clone(); Py::Bool({ let t4732 = Py::Int(1i64); if py::le(&t4731, &t4732)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__from_si(mut v_conv: Py, mut v_kw: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = py::dict_from(&v_kw.clone(), vec![])?;
    for t5448 in py::iter(&(py::method(&v_conv.clone(), "items", &[])?))? {
        let t5452 = (t5448).items()?;
        if t5452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5452[0].clone();
        let t5453 = (t5452[1].clone()).items()?;
        if t5453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t5453[0].clone();
        v_off = t5453[1].clone();
        v_v = py::method(&v_out.clone(), "get", &[v_k.clone()])?;
        if ({ let t5451 = { let t5449 = v_v.clone(); Py::Bool({ let t5450 = Py::None; if !py::is(&t5449, &t5450) { true } else { false } }) }; if !t5451.truthy() { t5451 } else { py::not(&Py::Bool(matches!(v_v.clone(), Py::Str(_) | Py::Bool(_)))) } }).truthy() {
            py::setitem(&v_out.clone(), &v_k.clone(), py::truediv(&py::sub(&v_v.clone(), &v_off.clone())?, &v_f.clone())?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int_ok(mut v_v: Py, mut v_name: Py) -> R {
    v_v = (if ({ let t4663 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4663.truthy() { t4663 } else { { let t4662 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4662.truthy() { t4662 } else { { let t4660 = py::float(&v_v.clone())?; Py::Bool({ let t4661 = py::int(&v_v.clone())?; if py::eq(&t4660, &t4661) { true } else { false } }) } } } } }).truthy() { py::int(&v_v.clone())? } else { Py::None });
    let _ = f__need({ let t4669 = { let t4664 = v_v.clone(); Py::Bool({ let t4665 = Py::None; if !py::is(&t4664, &t4665) { true } else { false } }) }; if !t4669.truthy() { t4669 } else { { let t4666 = Py::Int(1i64); Py::Bool({ let t4667 = v_v.clone(); if py::le(&t4666, &t4667)? { { let t4668 = Py::Int(6i64); if py::le(&t4667, &t4668)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of particles from 1 to 6".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t5058 = v_v.clone(); Py::Bool({ let t5059 = Py::Int(0i64); if py::ge(&t5058, &t5059)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t5436 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5436.truthy() { t5436 } else { { let t5435 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5435.truthy() { t5435 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__o2c(mut v_Hb: Py, mut v_S: Py, mut v_P: Py) -> R {
    return Ok(py::add(&py::mul(&py::mul(&c_O2_HB()?, &v_Hb.clone())?, &v_S.clone())?, &py::mul(&c_O2_SOL()?, &v_P.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_R(mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::mul(&Py::Float(8.0), &py::mul(&v_eta.clone(), &Py::Float(0.001))?)?, &py::mul(&v_L.clone(), &Py::Float(0.01))?)?, &py::mul(&c_PI()?, &py::pow(&py::mul(&v_r.clone(), &Py::Float(0.001))?, &Py::Int(4i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t5437 = v_v.clone(); Py::Bool({ let t5438 = Py::Int(0i64); if py::gt(&t5437, &t5438)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rnd(mut v_v: Py) -> R {
    if ({ let t5443 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5443.truthy() { t5443 } else { { let t5442 = Py::Bool(matches!(v_v.clone(), Py::Str(_))); if t5442.truthy() { t5442 } else { { let t5440 = v_v.clone(); Py::Bool({ let t5441 = Py::None; if py::is(&t5440, &t5441) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        return Ok(f__r12(v_v.clone())?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5444 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5444;
out.push(f__rnd(v_x.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5445 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t5446 = (t5445).items()?;
if t5446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5446[0].clone();
v_x = t5446[1].clone();
out.push((v_k.clone(), f__rnd(v_x.clone())?)); } Py::dict(out) });
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rtf(mut v_T: Py) -> R {
    return Ok(py::mul(&py::truediv(&py::mul(&c_R_GAS()?, &py::add(&v_T.clone(), &c_T0C()?)?)?, &c_FARADAY()?)?, &Py::Float(1000.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sev(mut v_P: Py) -> R {
    return Ok(py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&Py::Float(23400.0), &py::add(&py::pow(&v_P.clone(), &Py::Int(3i64))?, &py::mul(&Py::Float(150.0), &v_P.clone())?)?)?, &Py::Float(1.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sex(mut v_v: Py) -> R {
    let _ = f__need({ let t5330 = v_v.clone(); Py::Bool({ let t5331 = Py::tuple(vec![Py::str("female"), Py::str("male")]); if py::contains(&t5331, &t5330)? { true } else { false } }) }, Py::str("sex must be female or male"))?;
    return Ok({ let t5332 = v_v.clone(); Py::Bool({ let t5333 = Py::str("female"); if py::eq(&t5332, &t5333) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__teich(mut v_D: Py) -> R {
    return Ok(py::mul(&py::truediv(&Py::Float(7.0), &py::add(&Py::Float(2.4), &v_D.clone())?)?, &py::pow(&v_D.clone(), &Py::Int(3i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__need(Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_))), Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))?;
    v_out = { let mut out = vec![]; for t5212 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5212;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) };
    let _ = f__need({ let t5215 = v_lo.clone(); Py::Bool({ let t5216 = py::len(&v_out.clone())?; if py::le(&t5215, &t5216)? { { let t5217 = v_hi.clone(); if py::le(&t5216, &t5217)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " needs between ".to_string(), py::format(&v_lo.clone(), "")?, " and ".to_string(), py::format(&v_hi.clone(), "")?, " entries".to_string()].concat()))?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_aa_gradient(mut v_FiO2: Py, mut v_Patm: Py, mut v_PH2O: Py, mut v_PaCO2: Py, mut v_PaO2: Py, mut v_RQ: Py) -> R {
    let mut v_pa: Py = Py::None;
    v_FiO2 = f__fr(v_FiO2.clone(), Py::str("FiO2"), Py::Bool(true), Py::Bool(false))?;
    let t17 = (Py::tuple(vec![f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?, f__nonneg(v_PaCO2.clone(), Py::str("PaCO2"))?, f__pos(v_PaO2.clone(), Py::str("PaO2"))?, f__pos(v_RQ.clone(), Py::str("RQ"))?])).items()?;
    if t17.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Patm = t17[0].clone();
    v_PH2O = t17[1].clone();
    v_PaCO2 = t17[2].clone();
    v_PaO2 = t17[3].clone();
    v_RQ = t17[4].clone();
    let _ = f__need({ let t18 = v_Patm.clone(); Py::Bool({ let t19 = v_PH2O.clone(); if py::gt(&t18, &t19)? { true } else { false } }) }, Py::str("the barometric pressure must exceed the water vapour pressure"))?;
    v_pa = py::sub(&py::mul(&v_FiO2.clone(), &py::sub(&v_Patm.clone(), &v_PH2O.clone())?)?, &py::truediv(&v_PaCO2.clone(), &v_RQ.clone())?)?;
    let _ = f__need({ let t20 = v_pa.clone(); Py::Bool({ let t21 = Py::Int(0i64); if py::gt(&t20, &t21)? { true } else { false } }) }, Py::str("the computed alveolar PO2 is not positive"))?;
    let _ = f__need({ let t22 = v_PaO2.clone(); Py::Bool({ let t23 = v_pa.clone(); if py::le(&t22, &t23)? { true } else { false } }) }, Py::str("the arterial PO2 exceeds the computed alveolar PO2: check FiO2, PaCO2 or the respiratory quotient"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PAO2"), v_pa.clone()), (Py::str("Aa_gradient"), py::sub(&v_pa.clone(), &v_PaO2.clone())?), (Py::str("a_A_ratio"), py::truediv(&v_PaO2.clone(), &v_pa.clone())?), (Py::str("PF_ratio"), py::truediv(&v_PaO2.clone(), &v_FiO2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_gas(mut v_FiO2: Py, mut v_Patm: Py, mut v_PH2O: Py, mut v_PaCO2: Py, mut v_RQ: Py) -> R {
    let mut v_pio2: Py = Py::None;
    let mut v_simp: Py = Py::None;
    v_FiO2 = f__fr(v_FiO2.clone(), Py::str("FiO2"), Py::Bool(true), Py::Bool(false))?;
    let t86 = (Py::tuple(vec![f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?, f__nonneg(v_PaCO2.clone(), Py::str("PaCO2"))?, f__pos(v_RQ.clone(), Py::str("RQ"))?])).items()?;
    if t86.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Patm = t86[0].clone();
    v_PH2O = t86[1].clone();
    v_PaCO2 = t86[2].clone();
    v_RQ = t86[3].clone();
    let _ = f__need({ let t87 = v_Patm.clone(); Py::Bool({ let t88 = v_PH2O.clone(); if py::gt(&t87, &t88)? { true } else { false } }) }, Py::str("the barometric pressure must exceed the water vapour pressure"))?;
    let _ = f__need({ let t89 = v_RQ.clone(); Py::Bool({ let t90 = Py::Float(1.5); if py::le(&t89, &t90)? { true } else { false } }) }, Py::str("RQ must be at most 1.5"))?;
    v_pio2 = py::mul(&v_FiO2.clone(), &py::sub(&v_Patm.clone(), &v_PH2O.clone())?)?;
    v_simp = py::sub(&v_pio2.clone(), &py::truediv(&v_PaCO2.clone(), &v_RQ.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PiO2"), v_pio2.clone()), (Py::str("PAO2"), v_simp.clone()), (Py::str("PAO2_full"), py::add(&v_simp.clone(), &py::truediv(&py::mul(&py::mul(&v_PaCO2.clone(), &v_FiO2.clone())?, &py::sub(&Py::Float(1.0), &v_RQ.clone())?)?, &v_RQ.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_laplace(mut v_gamma: Py, mut v_r: Py) -> R {
    let mut v_p: Py = Py::None;
    let t142 = (Py::tuple(vec![f__pos(v_gamma.clone(), Py::str("gamma"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gamma = t142[0].clone();
    v_r = t142[1].clone();
    v_p = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_gamma.clone())?, &Py::Float(0.001))?, &py::mul(&v_r.clone(), &Py::Float(1e-06))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("P"), v_p.clone()), (Py::str("P_cmH2O"), py::truediv(&v_p.clone(), &c_CMH2O()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_ventilation_co2(mut v_VCO2: Py, mut v_PaCO2: Py) -> R {
    let t194 = (Py::tuple(vec![f__pos(v_VCO2.clone(), Py::str("VCO2"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VCO2 = t194[0].clone();
    v_PaCO2 = t194[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("VA"), py::truediv(&py::truediv(&py::mul(&v_VCO2.clone(), &c_K_ALV()?)?, &v_PaCO2.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_anion_gap(mut v_Na: Py, mut v_Cl: Py, mut v_HCO3: Py, mut v_albumin: Py) -> R {
    let mut v_ag: Py = Py::None;
    let t246 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__pos(v_Cl.clone(), Py::str("Cl"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_albumin.clone(), Py::str("albumin"))?])).items()?;
    if t246.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t246[0].clone();
    v_Cl = t246[1].clone();
    v_HCO3 = t246[2].clone();
    v_albumin = t246[3].clone();
    v_ag = py::sub(&v_Na.clone(), &py::add(&v_Cl.clone(), &v_HCO3.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("AG"), v_ag.clone()), (Py::str("AG_corrected"), py::add(&v_ag.clone(), &py::mul(&Py::Float(2.5), &py::sub(&Py::Float(4.0), &v_albumin.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_atwater_energy(mut v_carbohydrate: Py, mut v_fat: Py, mut v_protein: Py, mut v_alcohol: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t298 = (Py::tuple(vec![f__nonneg(v_carbohydrate.clone(), Py::str("carbohydrate"))?, f__nonneg(v_fat.clone(), Py::str("fat"))?, f__nonneg(v_protein.clone(), Py::str("protein"))?, f__nonneg(v_alcohol.clone(), Py::str("alcohol"))?])).items()?;
    if t298.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c = t298[0].clone();
    v_f = t298[1].clone();
    v_p = t298[2].clone();
    v_a = t298[3].clone();
    v_k = py::add(&py::add(&py::add(&py::mul(&Py::Float(4.0), &v_c.clone())?, &py::mul(&Py::Float(9.0), &v_f.clone())?)?, &py::mul(&Py::Float(4.0), &v_p.clone())?)?, &py::mul(&Py::Float(7.0), &v_a.clone())?)?;
    let _ = f__need({ let t299 = v_k.clone(); Py::Bool({ let t300 = Py::Int(0i64); if py::gt(&t299, &t300)? { true } else { false } }) }, Py::str("at least one amount must be positive"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("kcal"), v_k.clone()), (Py::str("kJ"), py::mul(&v_k.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_barometric_altitude(mut v_h: Py, mut v_FiO2: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_pm: Py = Py::None;
    v_h = f__num(v_h.clone(), Py::str("h"))?;
    v_FiO2 = f__fr(v_FiO2.clone(), Py::str("FiO2"), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t359 = Py::Int(0i64); Py::Bool({ let t360 = v_h.clone(); if py::le(&t359, &t360)? { { let t361 = Py::Int(11000i64); if py::le(&t360, &t361)? { true } else { false } } } else { false } }) }, Py::str("altitude must be between 0 and 11000 m (troposphere)"))?;
    v_p = py::mul(&Py::Float(101325.0), &py::pow(&py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(0.0065), &v_h.clone())?, &Py::Float(288.15))?)?, &c__ISA_EXP()?)?)?;
    v_pm = py::truediv(&v_p.clone(), &c_MMHG()?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Patm"), v_pm.clone()), (Py::str("PiO2"), py::mul(&v_FiO2.clone(), &py::sub(&v_pm.clone(), &Py::Float(47.0))?)?), (Py::str("Patm_kPa"), py::truediv(&v_p.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli_drop(mut v_v1: Py, mut v_v2: Py, mut v_rho: Py) -> R {
    let mut v_dp: Py = Py::None;
    let t415 = (Py::tuple(vec![f__nonneg(v_v1.clone(), Py::str("v1"))?, f__pos(v_v2.clone(), Py::str("v2"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t415.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t415[0].clone();
    v_v2 = t415[1].clone();
    v_rho = t415[2].clone();
    let _ = f__need({ let t416 = v_v2.clone(); Py::Bool({ let t417 = v_v1.clone(); if py::gt(&t416, &t417)? { true } else { false } }) }, Py::str("the downstream (jet) velocity v2 must exceed v1"))?;
    v_dp = py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dP"), v_dp.clone()), (Py::str("dP_mmHg"), py::truediv(&v_dp.clone(), &c_MMHG()?)?), (Py::str("dP_simplified"), py::mul(&py::mul(&Py::Float(4.0), &v_v2.clone())?, &v_v2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bicarbonate_from_ph(mut v_pH: Py, mut v_PaCO2: Py) -> R {
    let t469 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t469[0].clone();
    v_PaCO2 = t469[1].clone();
    let _ = f__need({ let t470 = Py::Float(6.5); Py::Bool({ let t471 = v_pH.clone(); if py::le(&t470, &t471)? { { let t472 = Py::Float(8.0); if py::le(&t471, &t472)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HCO3"), py::mul(&py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_blood_velocity_area(mut v_Q: Py, mut v_A: Py) -> R {
    let t524 = (Py::tuple(vec![f__pos(v_Q.clone(), Py::str("Q"))?, f__pos(v_A.clone(), Py::str("A"))?])).items()?;
    if t524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t524[0].clone();
    v_A = t524[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("v"), py::truediv(&py::truediv(&py::mul(&v_Q.clone(), &Py::Float(1000.0))?, &Py::Float(60.0))?, &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmi(mut v_weight: Py, mut v_height: Py) -> R {
    let t576 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t576[0].clone();
    v_height = t576[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("BMI"), py::truediv(&v_weight.clone(), &py::pow(&py::truediv(&v_height.clone(), &Py::Float(100.0))?, &Py::Int(2i64))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmr_harris_benedict(mut v_weight: Py, mut v_height: Py, mut v_age: Py, mut v_sex: Py, mut v_version: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t628 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t628.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t628[0].clone();
    v_H = t628[1].clone();
    v_A = t628[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t633 = v_version.clone(); Py::Bool({ let t634 = Py::tuple(vec![Py::str("original"), Py::str("revised")]); if py::contains(&t634, &t633)? { true } else { false } }) }, Py::str("version must be original (1919) or revised (1984)"))?;
    let _ = f__need({ let t635 = v_A.clone(); Py::Bool({ let t636 = Py::Int(18i64); if py::ge(&t635, &t636)? { true } else { false } }) }, Py::str("the equations are for adults"))?;
    if ({ let t637 = v_version.clone(); Py::Bool({ let t638 = Py::str("original"); if py::eq(&t637, &t638) { true } else { false } }) }).truthy() {
        v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::add(&Py::Float(655.0955), &py::mul(&Py::Float(9.5634), &v_W.clone())?)?, &py::mul(&Py::Float(1.8496), &v_H.clone())?)?, &py::mul(&Py::Float(4.6756), &v_A.clone())?)? } else { py::sub(&py::add(&py::add(&Py::Float(66.473), &py::mul(&Py::Float(13.7516), &v_W.clone())?)?, &py::mul(&Py::Float(5.0033), &v_H.clone())?)?, &py::mul(&Py::Float(6.755), &v_A.clone())?)? });
    } else {
        v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::add(&Py::Float(447.593), &py::mul(&Py::Float(9.247), &v_W.clone())?)?, &py::mul(&Py::Float(3.098), &v_H.clone())?)?, &py::mul(&Py::Float(4.33), &v_A.clone())?)? } else { py::sub(&py::add(&py::add(&Py::Float(88.362), &py::mul(&Py::Float(13.397), &v_W.clone())?)?, &py::mul(&Py::Float(4.799), &v_H.clone())?)?, &py::mul(&Py::Float(5.677), &v_A.clone())?)? });
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("BMR"), v_v.clone()), (Py::str("BMR_kJ"), py::mul(&v_v.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmr_katch_mcardle(mut v_weight: Py, mut v_body_fat: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_lbm: Py = Py::None;
    let t699 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_body_fat.clone(), Py::str("body_fat"), Py::Bool(false), Py::Bool(true))?])).items()?;
    if t699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t699[0].clone();
    v_f = t699[1].clone();
    v_lbm = py::mul(&v_W.clone(), &py::sub(&Py::Float(1.0), &v_f.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("LBM"), v_lbm.clone()), (Py::str("BMR"), py::add(&Py::Float(370.0), &py::mul(&Py::Float(21.6), &v_lbm.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmr_mifflin(mut v_weight: Py, mut v_height: Py, mut v_age: Py, mut v_sex: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t751 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t751.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t751[0].clone();
    v_H = t751[1].clone();
    v_A = t751[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t756 = v_A.clone(); Py::Bool({ let t757 = Py::Int(18i64); if py::ge(&t756, &t757)? { true } else { false } }) }, Py::str("the equation is for adults"))?;
    v_v = py::add(&py::sub(&py::add(&py::mul(&Py::Float(10.0), &v_W.clone())?, &py::mul(&Py::Float(6.25), &v_H.clone())?)?, &py::mul(&Py::Float(5.0), &v_A.clone())?)?, &(if (v_fem.clone()).truthy() { py::neg(&Py::Float(161.0))? } else { Py::Float(5.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("REE"), v_v.clone()), (Py::str("REE_kJ"), py::mul(&v_v.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_fat_density(mut v_density: Py) -> R {
    let mut v_D: Py = Py::None;
    v_D = f__num(v_density.clone(), Py::str("density"))?;
    let _ = f__need({ let t807 = Py::Float(0.9); Py::Bool({ let t808 = v_D.clone(); if py::lt(&t807, &t808)? { { let t809 = Py::Float(1.1); if py::le(&t808, &t809)? { true } else { false } } } else { false } }) }, Py::str("body density must be above 0.9 and at most 1.1 g/mL (the two-compartment equations give 0-100 % fat there)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?), (Py::str("fat_Brozek"), py::sub(&py::truediv(&Py::Float(457.0), &v_D.clone())?, &Py::Float(414.2))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_surface_area(mut v_weight: Py, mut v_height: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let t922 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t922[0].clone();
    v_H = t922[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("BSA_DuBois"), py::mul(&py::mul(&Py::Float(0.007184), &py::pow(&v_W.clone(), &Py::Float(0.425))?)?, &py::pow(&v_H.clone(), &Py::Float(0.725))?)?), (Py::str("BSA_Mosteller"), py::math1("sqrt", &py::truediv(&py::mul(&v_H.clone(), &v_W.clone())?, &Py::Float(3600.0))?)?), (Py::str("BSA_Haycock"), py::mul(&py::mul(&Py::Float(0.024265), &py::pow(&v_W.clone(), &Py::Float(0.5378))?)?, &py::pow(&v_H.clone(), &Py::Float(0.3964))?)?), (Py::str("BSA_GehanGeorge"), py::mul(&py::mul(&Py::Float(0.0235), &py::pow(&v_W.clone(), &Py::Float(0.51456))?)?, &py::pow(&v_H.clone(), &Py::Float(0.42246))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bohr_dead_space(mut v_PaCO2: Py, mut v_PECO2: Py, mut v_VT: Py) -> R {
    let mut v_f: Py = Py::None;
    let t976 = (Py::tuple(vec![f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?, f__nonneg(v_PECO2.clone(), Py::str("PECO2"))?, f__pos(v_VT.clone(), Py::str("VT"))?])).items()?;
    if t976.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PaCO2 = t976[0].clone();
    v_PECO2 = t976[1].clone();
    v_VT = t976[2].clone();
    let _ = f__need({ let t977 = v_PECO2.clone(); Py::Bool({ let t978 = v_PaCO2.clone(); if py::lt(&t977, &t978)? { true } else { false } }) }, Py::str("the mixed expired PCO2 must be below the arterial PCO2"))?;
    v_f = py::truediv(&py::sub(&v_PaCO2.clone(), &v_PECO2.clone())?, &v_PaCO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VD_VT"), v_f.clone()), (Py::str("VD"), py::mul(&v_f.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cardiac_index(mut v_CO: Py, mut v_BSA: Py, mut v_HR: Py) -> R {
    let mut v_ci: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1030 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t1030[0].clone();
    v_BSA = t1030[1].clone();
    v_ci = py::truediv(&v_CO.clone(), &v_BSA.clone())?;
    v_out = Py::dict(vec![(Py::str("CI"), v_ci.clone())]);
    if ({ let t1031 = v_HR.clone(); Py::Bool({ let t1032 = Py::None; if !py::is(&t1031, &t1032) { true } else { false } }) }).truthy() {
        v_HR = f__pos(v_HR.clone(), Py::str("HR"))?;
        py::setitem(&v_out.clone(), &Py::str("SVI"), py::truediv(&py::mul(&v_ci.clone(), &Py::Float(1000.0))?, &v_HR.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("SV"), py::truediv(&py::mul(&v_CO.clone(), &Py::Float(1000.0))?, &v_HR.clone())?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cardiac_output(mut v_HR: Py, mut v_SV: Py) -> R {
    let t1084 = (Py::tuple(vec![f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_SV.clone(), Py::str("SV"))?])).items()?;
    if t1084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HR = t1084[0].clone();
    v_SV = t1084[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&py::mul(&v_HR.clone(), &v_SV.clone())?, &Py::Float(1000.0))?), (Py::str("CO_mL"), py::mul(&v_HR.clone(), &v_SV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chord_conductance(mut v_gK: Py, mut v_gNa: Py, mut v_gCl: Py, mut v_EK: Py, mut v_ENa: Py, mut v_ECl: Py) -> R {
    let mut v_g: Py = Py::None;
    let t1136 = (Py::tuple(vec![f__nonneg(v_gK.clone(), Py::str("gK"))?, f__nonneg(v_gNa.clone(), Py::str("gNa"))?, f__nonneg(v_gCl.clone(), Py::str("gCl"))?])).items()?;
    if t1136.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_gK = t1136[0].clone();
    v_gNa = t1136[1].clone();
    v_gCl = t1136[2].clone();
    let t1137 = (Py::tuple(vec![f__num(v_EK.clone(), Py::str("EK"))?, f__num(v_ENa.clone(), Py::str("ENa"))?, f__num(v_ECl.clone(), Py::str("ECl"))?])).items()?;
    if t1137.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_EK = t1137[0].clone();
    v_ENa = t1137[1].clone();
    v_ECl = t1137[2].clone();
    v_g = py::add(&py::add(&v_gK.clone(), &v_gNa.clone())?, &v_gCl.clone())?;
    let _ = f__need({ let t1138 = v_g.clone(); Py::Bool({ let t1139 = Py::Int(0i64); if py::gt(&t1138, &t1139)? { true } else { false } }) }, Py::str("at least one conductance must be positive"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Vm"), py::truediv(&py::add(&py::add(&py::mul(&v_gK.clone(), &v_EK.clone())?, &py::mul(&v_gNa.clone(), &v_ENa.clone())?)?, &py::mul(&v_gCl.clone(), &v_ECl.clone())?)?, &v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cockcroft_gault(mut v_age: Py, mut v_weight: Py, mut v_Scr: Py, mut v_sex: Py) -> R {
    let mut v_fem: Py = Py::None;
    let t1191 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_Scr.clone(), Py::str("Scr"))?])).items()?;
    if t1191.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t1191[0].clone();
    v_weight = t1191[1].clone();
    v_Scr = t1191[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1196 = Py::Int(18i64); Py::Bool({ let t1197 = v_age.clone(); if py::le(&t1196, &t1197)? { { let t1198 = Py::Int(140i64); if py::lt(&t1197, &t1198)? { true } else { false } } } else { false } }) }, Py::str("age must be at least 18 and below 140"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CrCl"), py::mul(&py::truediv(&py::mul(&py::sub(&Py::Float(140.0), &v_age.clone())?, &v_weight.clone())?, &py::mul(&Py::Float(72.0), &v_Scr.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.85) } else { Py::Float(1.0) }))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compliance(mut v_dV: Py, mut v_dP: Py) -> R {
    let t5113 = (Py::tuple(vec![f__pos(v_dV.clone(), Py::str("dV"))?, f__pos(v_dP.clone(), Py::str("dP"))?])).items()?;
    if t5113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dV = t5113[0].clone();
    v_dP = t5113[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&v_dV.clone(), &v_dP.clone())?), (Py::str("E"), py::truediv(&v_dP.clone(), &v_dV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conduction_velocity_scaling(mut v_v1: Py, mut v_d1: Py, mut v_d2: Py, mut v_fiber: Py) -> R {
    let mut v_k: Py = Py::None;
    let t1250 = (Py::tuple(vec![f__pos(v_v1.clone(), Py::str("v1"))?, f__pos(v_d1.clone(), Py::str("d1"))?, f__pos(v_d2.clone(), Py::str("d2"))?])).items()?;
    if t1250.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1250[0].clone();
    v_d1 = t1250[1].clone();
    v_d2 = t1250[2].clone();
    let _ = f__need({ let t1251 = v_fiber.clone(); Py::Bool({ let t1252 = Py::tuple(vec![Py::str("myelinated"), Py::str("unmyelinated")]); if py::contains(&t1252, &t1251)? { true } else { false } }) }, Py::str("fiber must be myelinated or unmyelinated"))?;
    v_k = (if ({ let t1253 = v_fiber.clone(); Py::Bool({ let t1254 = Py::str("myelinated"); if py::eq(&t1253, &t1254) { true } else { false } }) }).truthy() { py::truediv(&v_d2.clone(), &v_d1.clone())? } else { py::math1("sqrt", &py::truediv(&v_d2.clone(), &v_d1.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("v2"), py::mul(&v_v1.clone(), &v_k.clone())?), (Py::str("ratio"), v_k.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_creatinine_clearance_timed(mut v_Ucr: Py, mut v_Vurine: Py, mut v_hours: Py, mut v_Pcr: Py, mut v_BSA: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1306 = (Py::tuple(vec![f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Vurine.clone(), Py::str("Vurine"))?, f__pos(v_hours.clone(), Py::str("hours"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1306.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ucr = t1306[0].clone();
    v_Vurine = t1306[1].clone();
    v_hours = t1306[2].clone();
    v_Pcr = t1306[3].clone();
    v_BSA = t1306[4].clone();
    v_v = py::truediv(&v_Vurine.clone(), &py::mul(&v_hours.clone(), &Py::Float(60.0))?)?;
    v_c = py::truediv(&py::mul(&v_Ucr.clone(), &v_v.clone())?, &v_Pcr.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("urine_flow"), v_v.clone()), (Py::str("CrCl"), v_c.clone()), (Py::str("CrCl_normalized"), py::truediv(&py::mul(&v_c.clone(), &Py::Float(1.73))?, &v_BSA.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ecg_heart_rate(mut v_RR: Py, mut v_small_boxes: Py, mut v_large_boxes: Py, mut v_speed: Py) -> R {
    let mut v_given: Py = Py::None;
    let mut v_hr: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_given = Py::Int(0i64);
    for t1354 in py::iter(&(Py::tuple(vec![v_RR.clone(), v_small_boxes.clone(), v_large_boxes.clone()])))? {
        v_v = t1354;
        if ({ let t1355 = v_v.clone(); Py::Bool({ let t1356 = Py::None; if !py::is(&t1355, &t1356) { true } else { false } }) }).truthy() {
            v_given = py::iadd(&v_given, &Py::Int(1i64))?;
        }
    }
    let _ = f__need({ let t1357 = v_given.clone(); Py::Bool({ let t1358 = Py::Int(1i64); if py::eq(&t1357, &t1358) { true } else { false } }) }, Py::str("give exactly one of the R-R interval RR, the number of small boxes or the number of large boxes"))?;
    v_sp = (if ({ let t1359 = v_speed.clone(); Py::Bool({ let t1360 = Py::None; if py::is(&t1359, &t1360) { true } else { false } }) }).truthy() { Py::Float(25.0) } else { f__pos(v_speed.clone(), Py::str("speed"))? });
    if ({ let t1365 = v_RR.clone(); Py::Bool({ let t1366 = Py::None; if !py::is(&t1365, &t1366) { true } else { false } }) }).truthy() {
        v_rr = f__pos(v_RR.clone(), Py::str("RR"))?;
    } else {
        if ({ let t1367 = v_small_boxes.clone(); Py::Bool({ let t1368 = Py::None; if !py::is(&t1367, &t1368) { true } else { false } }) }).truthy() {
            v_rr = py::truediv(&py::mul(&f__pos(v_small_boxes.clone(), Py::str("small_boxes"))?, &Py::Float(1.0))?, &v_sp.clone())?;
        } else {
            v_rr = py::truediv(&py::mul(&f__pos(v_large_boxes.clone(), Py::str("large_boxes"))?, &Py::Float(5.0))?, &v_sp.clone())?;
        }
    }
    v_hr = py::truediv(&Py::Float(60.0), &v_rr.clone())?;
    let _ = f__need({ let t1369 = Py::Float(20.0); Py::Bool({ let t1370 = v_hr.clone(); if py::le(&t1369, &t1370)? { { let t1371 = Py::Float(300.0); if py::le(&t1370, &t1371)? { true } else { false } } } else { false } }) }, Py::str("the heart rate must lie between 20 and 300 beats/min"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HR"), v_hr.clone()), (Py::str("RR"), v_rr.clone()), (Py::str("RR_ms"), py::mul(&v_rr.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_egfr_ckd_epi_2021(mut v_Scr: Py, mut v_age: Py, mut v_sex: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1422 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1422[0].clone();
    v_age = t1422[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1427 = Py::Int(18i64); Py::Bool({ let t1428 = v_age.clone(); if py::le(&t1427, &t1428)? { { let t1429 = Py::Int(120i64); if py::le(&t1428, &t1429)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1430 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.241))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.302))?]) })).items()?;
    if t1430.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1430[0].clone();
    v_a = t1430[1].clone();
    v_s = py::truediv(&v_Scr.clone(), &v_k.clone())?;
    v_v = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(142.0), &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], false)?, &v_a.clone())?)?, &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(1.2))?)?)?, &py::pow(&Py::Float(0.9938), &v_age.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(1.012) } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eGFR"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_egfr_ckd_epi_2021_cys(mut v_Scr: Py, mut v_Scys: Py, mut v_age: Py, mut v_sex: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1475 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__pos(v_Scys.clone(), Py::str("Scys"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1475.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1475[0].clone();
    v_Scys = t1475[1].clone();
    v_age = t1475[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1480 = Py::Int(18i64); Py::Bool({ let t1481 = v_age.clone(); if py::le(&t1480, &t1481)? { { let t1482 = Py::Int(120i64); if py::le(&t1481, &t1482)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1483 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.219))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.144))?]) })).items()?;
    if t1483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1483[0].clone();
    v_a = t1483[1].clone();
    let t1484 = (Py::tuple(vec![py::truediv(&v_Scr.clone(), &v_k.clone())?, py::truediv(&v_Scys.clone(), &Py::Float(0.8))?])).items()?;
    if t1484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1484[0].clone();
    v_c = t1484[1].clone();
    v_v = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(135.0), &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], false)?, &v_a.clone())?)?, &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.544))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], false)?, &py::neg(&Py::Float(0.323))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.778))?)?)?, &py::pow(&Py::Float(0.9961), &v_age.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.963) } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eGFR"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elimination_half_life(mut v_CL: Py, mut v_Vd: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1536 = (Py::tuple(vec![f__pos(v_CL.clone(), Py::str("CL"))?, f__pos(v_Vd.clone(), Py::str("Vd"))?])).items()?;
    if t1536.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CL = t1536[0].clone();
    v_Vd = t1536[1].clone();
    v_k = py::truediv(&v_CL.clone(), &v_Vd.clone())?;
    v_out = Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("half_life"), py::truediv(&py::math1("log", &Py::Float(2.0))?, &v_k.clone())?)]);
    if ({ let t1537 = v_t.clone(); Py::Bool({ let t1538 = Py::None; if !py::is(&t1537, &t1538) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction_remaining"), py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &f__nonneg(v_t.clone(), Py::str("t"))?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fev1_fvc(mut v_FEV1: Py, mut v_FVC: Py) -> R {
    let t1592 = (Py::tuple(vec![f__pos(v_FEV1.clone(), Py::str("FEV1"))?, f__pos(v_FVC.clone(), Py::str("FVC"))?])).items()?;
    if t1592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FEV1 = t1592[0].clone();
    v_FVC = t1592[1].clone();
    let _ = f__need({ let t1593 = v_FEV1.clone(); Py::Bool({ let t1594 = v_FVC.clone(); if py::le(&t1593, &t1594)? { true } else { false } }) }, Py::str("FEV1 cannot exceed FVC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("ratio"), py::truediv(&v_FEV1.clone(), &v_FVC.clone())?), (Py::str("percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_FEV1.clone())?, &v_FVC.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_cardiac_output(mut v_VO2: Py, mut v_CaO2: Py, mut v_CvO2: Py) -> R {
    let mut v_d: Py = Py::None;
    let t1648 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t1648.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1648[0].clone();
    v_CaO2 = t1648[1].clone();
    v_CvO2 = t1648[2].clone();
    let _ = f__need({ let t1649 = v_CvO2.clone(); Py::Bool({ let t1650 = v_CaO2.clone(); if py::lt(&t1649, &t1650)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    v_d = py::sub(&v_CaO2.clone(), &v_CvO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&v_d.clone(), &Py::Float(10.0))?)?), (Py::str("avDO2"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_diffusion(mut v_D: Py, mut v_A: Py, mut v_dC: Py, mut v_T: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1702 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_A.clone(), Py::str("A"))?, f__num(v_dC.clone(), Py::str("dC"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t1702.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1702[0].clone();
    v_A = t1702[1].clone();
    v_dC = t1702[2].clone();
    v_T = t1702[3].clone();
    v_x = py::mul(&v_T.clone(), &Py::Float(0.0001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("J"), py::truediv(&py::mul(&py::mul(&v_D.clone(), &v_A.clone())?, &py::mul(&v_dC.clone(), &Py::Float(0.001))?)?, &v_x.clone())?), (Py::str("t_diffusion"), py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&Py::Float(2.0), &v_D.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_saturations(mut v_VO2: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t1756 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t1756.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1756[0].clone();
    v_Hb = t1756[1].clone();
    v_PaO2 = t1756[2].clone();
    v_PvO2 = t1756[3].clone();
    let t1766 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t1766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t1766[0].clone();
    v_SvO2 = t1766[1].clone();
    let t1767 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t1767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t1767[0].clone();
    v_cv = t1767[1].clone();
    let _ = f__need({ let t1768 = v_cv.clone(); Py::Bool({ let t1769 = v_ca.clone(); if py::lt(&t1768, &t1769)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&py::sub(&v_ca.clone(), &v_cv.clone())?, &Py::Float(10.0))?)?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_fraction(mut v_GFR: Py, mut v_RPF: Py) -> R {
    let t1821 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__pos(v_RPF.clone(), Py::str("RPF"))?])).items()?;
    if t1821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t1821[0].clone();
    v_RPF = t1821[1].clone();
    let _ = f__need({ let t1822 = v_GFR.clone(); Py::Bool({ let t1823 = v_RPF.clone(); if py::lt(&t1822, &t1823)? { true } else { false } }) }, Py::str("GFR must be smaller than the renal plasma flow"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FF"), py::truediv(&v_GFR.clone(), &v_RPF.clone())?), (Py::str("FF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_GFR.clone())?, &v_RPF.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fluid_compartments(mut v_weight: Py, mut v_TBW_fraction: Py) -> R {
    let mut v_ecf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_tbw: Py = Py::None;
    let t870 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_TBW_fraction.clone(), Py::str("TBW_fraction"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t870.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t870[0].clone();
    v_f = t870[1].clone();
    v_tbw = py::mul(&v_f.clone(), &v_weight.clone())?;
    v_ecf = py::truediv(&v_tbw.clone(), &Py::Float(3.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("TBW"), v_tbw.clone()), (Py::str("ICF"), py::truediv(&py::mul(&Py::Float(2.0), &v_tbw.clone())?, &Py::Float(3.0))?), (Py::str("ECF"), v_ecf.clone()), (Py::str("ISF"), py::mul(&Py::Float(0.75), &v_ecf.clone())?), (Py::str("plasma"), py::mul(&Py::Float(0.25), &v_ecf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fluid_shift(mut v_TBW: Py, mut v_Posm: Py, mut v_V_added: Py, mut v_C_added: Py, mut v_ICF_fraction: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_ecf0: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_icf: Py = Py::None;
    let mut v_icf0: Py = Py::None;
    let mut v_osm: Py = Py::None;
    let t1877 = (Py::tuple(vec![f__pos(v_TBW.clone(), Py::str("TBW"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?, f__pos(v_V_added.clone(), Py::str("V_added"))?, f__nonneg(v_C_added.clone(), Py::str("C_added"))?])).items()?;
    if t1877.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TBW = t1877[0].clone();
    v_Posm = t1877[1].clone();
    v_V = t1877[2].clone();
    v_C = t1877[3].clone();
    v_f = f__fr(v_ICF_fraction.clone(), Py::str("ICF_fraction"), Py::Bool(true), Py::Bool(true))?;
    let t1887 = (Py::tuple(vec![py::mul(&v_f.clone(), &v_TBW.clone())?, py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &v_TBW.clone())?])).items()?;
    if t1887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_icf0 = t1887[0].clone();
    v_ecf0 = t1887[1].clone();
    v_osm = py::truediv(&py::add(&py::mul(&v_TBW.clone(), &v_Posm.clone())?, &py::mul(&v_V.clone(), &v_C.clone())?)?, &py::add(&v_TBW.clone(), &v_V.clone())?)?;
    v_icf = py::truediv(&py::mul(&v_icf0.clone(), &v_Posm.clone())?, &v_osm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("osm_final"), v_osm.clone()), (Py::str("ICF"), v_icf.clone()), (Py::str("ECF"), py::sub(&py::add(&v_TBW.clone(), &v_V.clone())?, &v_icf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fractional_excretion(mut v_Ux: Py, mut v_Px: Py, mut v_Ucr: Py, mut v_Pcr: Py) -> R {
    let mut v_fe: Py = Py::None;
    let t1940 = (Py::tuple(vec![f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_Px.clone(), Py::str("Px"))?, f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?])).items()?;
    if t1940.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Ux = t1940[0].clone();
    v_Px = t1940[1].clone();
    v_Ucr = t1940[2].clone();
    v_Pcr = t1940[3].clone();
    v_fe = py::truediv(&py::mul(&v_Ux.clone(), &v_Pcr.clone())?, &py::mul(&v_Px.clone(), &v_Ucr.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FE"), v_fe.clone()), (Py::str("FE_percent"), py::mul(&Py::Float(100.0), &v_fe.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_water_clearance(mut v_V: Py, mut v_Uosm: Py, mut v_Posm: Py) -> R {
    let mut v_cosm: Py = Py::None;
    let t1985 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_Uosm.clone(), Py::str("Uosm"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?])).items()?;
    if t1985.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1985[0].clone();
    v_Uosm = t1985[1].clone();
    v_Posm = t1985[2].clone();
    v_cosm = py::truediv(&py::mul(&v_Uosm.clone(), &v_V.clone())?, &v_Posm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Cosm"), v_cosm.clone()), (Py::str("CH2O"), py::sub(&v_V.clone(), &v_cosm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_volume_conversion(mut v_V: Py, mut v_T: Py, mut v_Patm: Py, mut v_PH2O: Py) -> R {
    let mut v_tk: Py = Py::None;
    let t2039 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__num(v_T.clone(), Py::str("T"))?, f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?])).items()?;
    if t2039.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t2039[0].clone();
    v_T = t2039[1].clone();
    v_Patm = t2039[2].clone();
    v_PH2O = t2039[3].clone();
    let _ = f__need({ let t2048 = { let t2040 = py::neg(&Py::Int(50i64))?; Py::Bool({ let t2041 = v_T.clone(); if py::lt(&t2040, &t2041)? { { let t2042 = Py::Int(60i64); if py::lt(&t2041, &t2042)? { true } else { false } } } else { false } }) }; if !t2048.truthy() { t2048 } else { { let t2047 = { let t2043 = v_Patm.clone(); Py::Bool({ let t2044 = v_PH2O.clone(); if py::gt(&t2043, &t2044)? { true } else { false } }) }; if !t2047.truthy() { t2047 } else { { let t2045 = v_Patm.clone(); Py::Bool({ let t2046 = Py::Int(47i64); if py::gt(&t2045, &t2046)? { true } else { false } }) } } } } }, Py::str("temperature -50..60 degC, Patm above the vapour pressures"))?;
    v_tk = py::add(&v_T.clone(), &c_T0C()?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("V_BTPS"), py::truediv(&py::mul(&py::mul(&v_V.clone(), &py::truediv(&c_T_BODY()?, &v_tk.clone())?)?, &py::sub(&v_Patm.clone(), &v_PH2O.clone())?)?, &py::sub(&v_Patm.clone(), &Py::Float(47.0))?)?), (Py::str("V_STPD"), py::truediv(&py::mul(&py::mul(&v_V.clone(), &py::truediv(&c_T0C()?, &v_tk.clone())?)?, &py::sub(&v_Patm.clone(), &v_PH2O.clone())?)?, &Py::Float(760.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ghk_voltage(mut v_PK: Py, mut v_PNa: Py, mut v_PCl: Py, mut v_K_o: Py, mut v_K_i: Py, mut v_Na_o: Py, mut v_Na_i: Py, mut v_Cl_o: Py, mut v_Cl_i: Py, mut v_T: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let t2102 = (Py::tuple(vec![f__pos(v_PK.clone(), Py::str("PK"))?, f__nonneg(v_PNa.clone(), Py::str("PNa"))?, f__nonneg(v_PCl.clone(), Py::str("PCl"))?])).items()?;
    if t2102.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PK = t2102[0].clone();
    v_PNa = t2102[1].clone();
    v_PCl = t2102[2].clone();
    v_vals = { let mut out = vec![]; for t2103 in py::iter(&(Py::tuple(vec![v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone()])))? { let mut v_x: Py; 
v_x = t2103;
out.push(f__pos(v_x.clone(), Py::str("concentrations"))?); } Py::list(out) };
    let t2104 = (v_vals.clone()).items()?;
    if t2104.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_K_o = t2104[0].clone();
    v_K_i = t2104[1].clone();
    v_Na_o = t2104[2].clone();
    v_Na_i = t2104[3].clone();
    v_Cl_o = t2104[4].clone();
    v_Cl_i = t2104[5].clone();
    v_T = f__num(v_T.clone(), Py::str("T"))?;
    let _ = f__need({ let t2105 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2106 = v_T.clone(); if py::lt(&t2105, &t2106)? { { let t2107 = Py::Int(100i64); if py::lt(&t2106, &t2107)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_num = py::add(&py::add(&py::mul(&v_PK.clone(), &v_K_o.clone())?, &py::mul(&v_PNa.clone(), &v_Na_o.clone())?)?, &py::mul(&v_PCl.clone(), &v_Cl_i.clone())?)?;
    v_den = py::add(&py::add(&py::mul(&v_PK.clone(), &v_K_i.clone())?, &py::mul(&v_PNa.clone(), &v_Na_i.clone())?)?, &py::mul(&v_PCl.clone(), &v_Cl_o.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Vm"), py::mul(&f__rtf(v_T.clone())?, &py::math1("log", &py::truediv(&v_num.clone(), &v_den.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gibbs_donnan(mut v_A: Py, mut v_c: Py, mut v_T: Py) -> R {
    let mut v_cl: Py = Py::None;
    let mut v_na: Py = Py::None;
    let t2159 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("A"))?, f__pos(v_c.clone(), Py::str("c"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t2159.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2159[0].clone();
    v_c = t2159[1].clone();
    v_T = t2159[2].clone();
    let _ = f__need({ let t2160 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2161 = v_T.clone(); if py::lt(&t2160, &t2161)? { { let t2162 = Py::Int(100i64); if py::lt(&t2161, &t2162)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_cl = py::truediv(&py::add(&py::neg(&v_A.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_c.clone())?, &v_c.clone())?)?)?)?, &Py::Float(2.0))?;
    v_na = py::add(&v_cl.clone(), &v_A.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Na_in"), v_na.clone()), (Py::str("Cl_in"), v_cl.clone()), (Py::str("r"), py::truediv(&v_na.clone(), &v_c.clone())?), (Py::str("V_mV"), py::mul(&py::mul(&py::truediv(&py::mul(&py::neg(&c_R_GAS()?)?, &py::add(&v_T.clone(), &c_T0C()?)?)?, &c_FARADAY()?)?, &py::math1("log", &py::truediv(&v_na.clone(), &v_c.clone())?)?)?, &Py::Float(1000.0))?), (Py::str("osmotic_excess"), py::sub(&py::add(&v_na.clone(), &v_cl.clone())?, &py::mul(&Py::Float(2.0), &v_c.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_glucose_titration(mut v_GFR: Py, mut v_Pglucose: Py, mut v_Tm: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let t2216 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Pglucose.clone(), Py::str("Pglucose"))?, f__pos(v_Tm.clone(), Py::str("Tm"))?])).items()?;
    if t2216.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t2216[0].clone();
    v_P = t2216[1].clone();
    v_Tm = t2216[2].clone();
    v_fl = py::truediv(&py::mul(&v_GFR.clone(), &v_P.clone())?, &Py::Float(100.0))?;
    v_re_ = py::minmax(&[v_fl.clone(), v_Tm.clone()], false)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("filtered_load"), v_fl.clone()), (Py::str("reabsorbed"), v_re_.clone()), (Py::str("excreted"), py::sub(&v_fl.clone(), &v_re_.clone())?), (Py::str("threshold"), py::mul(&py::truediv(&v_Tm.clone(), &v_GFR.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heart_rate_reserve(mut v_age: Py, mut v_HR_rest: Py, mut v_intensity: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2276 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_HR_rest.clone(), Py::str("HR_rest"))?, f__fr(v_intensity.clone(), Py::str("intensity"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2276.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2276[0].clone();
    v_r = t2276[1].clone();
    v_f = t2276[2].clone();
    let _ = f__need({ let t2277 = Py::Int(5i64); Py::Bool({ let t2278 = v_A.clone(); if py::le(&t2277, &t2278)? { { let t2279 = Py::Int(100i64); if py::le(&t2278, &t2279)? { true } else { false } } } else { false } }) }, Py::str("age must be between 5 and 100"))?;
    v_mx = py::sub(&Py::Float(220.0), &v_A.clone())?;
    let _ = f__need({ let t2280 = v_r.clone(); Py::Bool({ let t2281 = v_mx.clone(); if py::lt(&t2280, &t2281)? { true } else { false } }) }, Py::str("the resting heart rate must be below the estimated maximum"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HRmax"), v_mx.clone()), (Py::str("HRmax_Tanaka"), py::sub(&Py::Float(208.0), &py::mul(&Py::Float(0.7), &v_A.clone())?)?), (Py::str("HRR"), py::sub(&v_mx.clone(), &v_r.clone())?), (Py::str("target"), py::add(&v_r.clone(), &py::mul(&v_f.clone(), &py::sub(&v_mx.clone(), &v_r.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_fit(mut v_P1: Py, mut v_S1: Py, mut v_P2: Py, mut v_S2: Py) -> R {
    let mut v_n: Py = Py::None;
    let t2326 = (Py::tuple(vec![f__pos(v_P1.clone(), Py::str("P1"))?, f__pos(v_P2.clone(), Py::str("P2"))?])).items()?;
    if t2326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2326[0].clone();
    v_P2 = t2326[1].clone();
    let t2336 = (Py::tuple(vec![f__fr(v_S1.clone(), Py::str("S1"), Py::Bool(true), Py::Bool(true))?, f__fr(v_S2.clone(), Py::str("S2"), Py::Bool(true), Py::Bool(true))?])).items()?;
    if t2336.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S1 = t2336[0].clone();
    v_S2 = t2336[1].clone();
    let _ = f__need({ let t2341 = { let t2337 = v_P1.clone(); Py::Bool({ let t2338 = v_P2.clone(); if py::ne(&t2337, &t2338) { true } else { false } }) }; if !t2341.truthy() { t2341 } else { { let t2339 = v_S1.clone(); Py::Bool({ let t2340 = v_S2.clone(); if py::ne(&t2339, &t2340) { true } else { false } }) } } }, Py::str("the two points must differ"))?;
    let _ = f__need({ let t2342 = py::mul(&py::sub(&v_P2.clone(), &v_P1.clone())?, &py::sub(&v_S2.clone(), &v_S1.clone())?)?; Py::Bool({ let t2343 = Py::Int(0i64); if py::gt(&t2342, &t2343)? { true } else { false } }) }, Py::str("saturation must increase with PO2"))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&py::truediv(&v_S2.clone(), &py::sub(&Py::Int(1i64), &v_S2.clone())?)?, &py::truediv(&v_S1.clone(), &py::sub(&Py::Int(1i64), &v_S1.clone())?)?)?)?, &py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("P50"), py::mul(&v_P1.clone(), &py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_S1.clone())?, &v_S1.clone())?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_force_velocity(mut v_F0: Py, mut v_a: Py, mut v_b: Py, mut v_v: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let t2397 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__nonneg(v_v.clone(), Py::str("v"))?])).items()?;
    if t2397.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t2397[0].clone();
    v_a = t2397[1].clone();
    v_b = t2397[2].clone();
    v_v = t2397[3].clone();
    v_vmax = py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?;
    let _ = f__need({ let t2398 = v_v.clone(); Py::Bool({ let t2399 = v_vmax.clone(); if py::le(&t2398, &t2399)? { true } else { false } }) }, Py::str("the shortening velocity cannot exceed vmax = b F0/a"))?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("power"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), v_vmax.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_po2(mut v_SO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let t2460 = (Py::tuple(vec![f__fr(v_SO2.clone(), Py::str("SO2"), Py::Bool(true), Py::Bool(true))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2460.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_SO2 = t2460[0].clone();
    v_P50 = t2460[1].clone();
    v_n = t2460[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2"), py::mul(&v_P50.clone(), &py::pow(&py::truediv(&v_SO2.clone(), &py::sub(&Py::Float(1.0), &v_SO2.clone())?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_saturation(mut v_PO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let mut v_x: Py = Py::None;
    let t2514 = (Py::tuple(vec![f__nonneg(v_PO2.clone(), Py::str("PO2"))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2514.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t2514[0].clone();
    v_P50 = t2514[1].clone();
    v_n = t2514[2].clone();
    v_x = py::pow(&py::truediv(&v_PO2.clone(), &v_P50.clone())?, &v_n.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SO2"), py::truediv(&v_x.clone(), &py::add(&Py::Float(1.0), &v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrostatic_column(mut v_h: Py, mut v_rho: Py) -> R {
    let mut v_p: Py = Py::None;
    let t2566 = (Py::tuple(vec![f__num(v_h.clone(), Py::str("h"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t2566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2566[0].clone();
    v_rho = t2566[1].clone();
    v_p = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_h.clone())?, &Py::Float(100.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dP"), v_p.clone()), (Py::str("dP_mmHg"), py::truediv(&v_p.clone(), &c_MMHG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrostatic_weighing(mut v_mass_air: Py, mut v_mass_water: Py, mut v_water_density: Py, mut v_RV: Py, mut v_GI: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Ma: Py = Py::None;
    let mut v_Mw: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let t2620 = (Py::tuple(vec![f__pos(v_mass_air.clone(), Py::str("mass_air"))?, f__num(v_mass_water.clone(), Py::str("mass_water"))?, f__pos(v_water_density.clone(), Py::str("water_density"))?, f__nonneg(v_RV.clone(), Py::str("RV"))?, f__nonneg(v_GI.clone(), Py::str("GI"))?])).items()?;
    if t2620.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ma = t2620[0].clone();
    v_Mw = t2620[1].clone();
    v_rho = t2620[2].clone();
    v_RV = t2620[3].clone();
    v_GI = t2620[4].clone();
    let _ = f__need({ let t2626 = { let t2621 = v_Mw.clone(); Py::Bool({ let t2622 = v_Ma.clone(); if py::lt(&t2621, &t2622)? { true } else { false } }) }; if !t2626.truthy() { t2626 } else { { let t2623 = Py::Float(0.95); Py::Bool({ let t2624 = v_rho.clone(); if py::lt(&t2623, &t2624)? { { let t2625 = Py::Float(1.0); if py::le(&t2624, &t2625)? { true } else { false } } } else { false } }) } } }, Py::str("underwater mass below the mass in air; water density 0.95-1.0 g/mL"))?;
    v_V = py::sub(&py::sub(&py::truediv(&py::sub(&v_Ma.clone(), &v_Mw.clone())?, &v_rho.clone())?, &v_RV.clone())?, &v_GI.clone())?;
    let _ = f__need({ let t2627 = v_V.clone(); Py::Bool({ let t2628 = Py::Int(0i64); if py::gt(&t2627, &t2628)? { true } else { false } }) }, Py::str("the computed body volume is not positive"))?;
    v_D = py::truediv(&v_Ma.clone(), &v_V.clone())?;
    let _ = f__need({ let t2629 = Py::Float(0.9); Py::Bool({ let t2630 = v_D.clone(); if py::lt(&t2629, &t2630)? { { let t2631 = Py::Float(1.1); if py::le(&t2630, &t2631)? { true } else { false } } } else { false } }) }, Py::str("the computed density is outside 0.9-1.1 g/mL (Siri gives a body fat outside 0-100 %)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("volume"), v_V.clone()), (Py::str("density"), v_D.clone()), (Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ionic_current(mut v_g: Py, mut v_Vm: Py, mut v_E: Py) -> R {
    let t2683 = (Py::tuple(vec![f__nonneg(v_g.clone(), Py::str("g"))?, f__num(v_Vm.clone(), Py::str("Vm"))?, f__num(v_E.clone(), Py::str("E"))?])).items()?;
    if t2683.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2683[0].clone();
    v_Vm = t2683[1].clone();
    v_E = t2683[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I"), py::mul(&v_g.clone(), &py::sub(&v_Vm.clone(), &v_E.clone())?)?), (Py::str("driving_force"), py::sub(&v_Vm.clone(), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_wall_tension(mut v_P: Py, mut v_r: Py, mut v_shape: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2735 = (Py::tuple(vec![f__pos(v_P.clone(), Py::str("P"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t2735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2735[0].clone();
    v_r = t2735[1].clone();
    let _ = f__need({ let t2736 = v_shape.clone(); Py::Bool({ let t2737 = Py::tuple(vec![Py::str("cylinder"), Py::str("sphere")]); if py::contains(&t2737, &t2736)? { true } else { false } }) }, Py::str("shape must be cylinder or sphere"))?;
    v_T = py::mul(&py::truediv(&py::mul(&py::mul(&v_P.clone(), &c_MMHG()?)?, &v_r.clone())?, &Py::Float(100.0))?, &(if ({ let t2738 = v_shape.clone(); Py::Bool({ let t2739 = Py::str("cylinder"); if py::eq(&t2738, &t2739) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.5) }))?;
    v_out = Py::dict(vec![(Py::str("T"), v_T.clone())]);
    if ({ let t2740 = v_h.clone(); Py::Bool({ let t2741 = Py::None; if !py::is(&t2740, &t2741) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("stress"), py::truediv(&py::truediv(&v_T.clone(), &py::truediv(&f__pos(v_h.clone(), Py::str("h"))?, &Py::Float(1000.0))?)?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lean_body_mass_boer(mut v_weight: Py, mut v_height: Py, mut v_sex: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t2793 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t2793[0].clone();
    v_H = t2793[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::mul(&Py::Float(0.252), &v_W.clone())?, &py::mul(&Py::Float(0.473), &v_H.clone())?)?, &Py::Float(48.3))? } else { py::sub(&py::add(&py::mul(&Py::Float(0.407), &v_W.clone())?, &py::mul(&Py::Float(0.267), &v_H.clone())?)?, &Py::Float(19.2))? });
    let _ = f__need({ let t2798 = Py::Int(0i64); Py::Bool({ let t2799 = v_v.clone(); if py::lt(&t2798, &t2799)? { { let t2800 = v_W.clone(); if py::lt(&t2799, &t2800)? { true } else { false } } } else { false } }) }, Py::str("the formula gives no valid lean mass for these values"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("LBM"), v_v.clone()), (Py::str("fat_mass"), py::sub(&v_W.clone(), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_length_constant(mut v_Rm: Py, mut v_Ri: Py, mut v_d: Py, mut v_x: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2852 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Ri.clone(), Py::str("Ri"))?, f__pos(v_d.clone(), Py::str("d"))?])).items()?;
    if t2852.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t2852[0].clone();
    v_Ri = t2852[1].clone();
    v_d = t2852[2].clone();
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&v_Rm.clone(), &py::mul(&v_d.clone(), &Py::Float(0.0001))?)?, &py::mul(&Py::Float(4.0), &v_Ri.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("lambda"), py::mul(&v_lam.clone(), &Py::Float(10.0))?)]);
    if ({ let t2853 = v_x.clone(); Py::Bool({ let t2854 = Py::None; if !py::is(&t2853, &t2854) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("attenuation"), py::math1("exp", &py::truediv(&py::neg(&py::truediv(&f__nonneg(v_x.clone(), Py::str("x"))?, &Py::Float(10.0))?)?, &v_lam.clone())?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lung_volumes(mut v_TV: Py, mut v_IRV: Py, mut v_ERV: Py, mut v_RV: Py) -> R {
    let t2910 = (Py::tuple(vec![f__pos(v_TV.clone(), Py::str("TV"))?, f__nonneg(v_IRV.clone(), Py::str("IRV"))?, f__nonneg(v_ERV.clone(), Py::str("ERV"))?, f__pos(v_RV.clone(), Py::str("RV"))?])).items()?;
    if t2910.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TV = t2910[0].clone();
    v_IRV = t2910[1].clone();
    v_ERV = t2910[2].clone();
    v_RV = t2910[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("IC"), py::add(&v_TV.clone(), &v_IRV.clone())?), (Py::str("FRC"), py::add(&v_ERV.clone(), &v_RV.clone())?), (Py::str("VC"), py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?), (Py::str("TLC"), py::add(&py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?, &v_RV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_arterial_pressure(mut v_SBP: Py, mut v_DBP: Py) -> R {
    let t2962 = (Py::tuple(vec![f__pos(v_SBP.clone(), Py::str("SBP"))?, f__pos(v_DBP.clone(), Py::str("DBP"))?])).items()?;
    if t2962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SBP = t2962[0].clone();
    v_DBP = t2962[1].clone();
    let _ = f__need({ let t2963 = v_SBP.clone(); Py::Bool({ let t2964 = v_DBP.clone(); if py::gt(&t2963, &t2964)? { true } else { false } }) }, Py::str("the systolic pressure must exceed the diastolic"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("MAP"), py::add(&v_DBP.clone(), &py::truediv(&py::sub(&v_SBP.clone(), &v_DBP.clone())?, &Py::Float(3.0))?)?), (Py::str("PP"), py::sub(&v_SBP.clone(), &v_DBP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_charge(mut v_Cm: Py, mut v_dV: Py, mut v_area: Py) -> R {
    let mut v_q: Py = Py::None;
    let t3016 = (Py::tuple(vec![f__pos(v_Cm.clone(), Py::str("Cm"))?, f__num(v_dV.clone(), Py::str("dV"))?, f__pos(v_area.clone(), Py::str("area"))?])).items()?;
    if t3016.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cm = t3016[0].clone();
    v_dV = t3016[1].clone();
    v_area = t3016[2].clone();
    let _ = f__need({ let t3017 = v_dV.clone(); Py::Bool({ let t3018 = Py::Int(0i64); if py::ne(&t3017, &t3018) { true } else { false } }) }, Py::str("the voltage change must not be zero"))?;
    v_q = py::mul(&py::mul(&py::mul(&v_Cm.clone(), &Py::Float(1e-06))?, &py::mul(&v_area.clone(), &Py::Float(1e-08))?)?, &py::mul(&v_dV.clone(), &Py::Float(0.001))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q_pC"), py::mul(&v_q.clone(), &Py::Float(1000000000000.0))?), (Py::str("ions"), py::truediv(&py::abs(&v_q.clone())?, &c_E_CHARGE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_time_constant(mut v_Rm: Py, mut v_Cm: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t3070 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Cm.clone(), Py::str("Cm"))?])).items()?;
    if t3070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t3070[0].clone();
    v_Cm = t3070[1].clone();
    v_tau = py::mul(&py::mul(&v_Rm.clone(), &v_Cm.clone())?, &Py::Float(0.001))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone())]);
    if ({ let t3071 = v_t.clone(); Py::Bool({ let t3072 = Py::None; if !py::is(&t3071, &t3072) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction"), py::neg(&py::math1("expm1", &py::truediv(&py::neg(&f__nonneg(v_t.clone(), Py::str("t"))?)?, &v_tau.clone())?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_met_energy(mut v_MET: Py, mut v_weight: Py, mut v_duration: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_vo2: Py = Py::None;
    let t3126 = (Py::tuple(vec![f__pos(v_MET.clone(), Py::str("MET"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_duration.clone(), Py::str("duration"))?])).items()?;
    if t3126.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MET = t3126[0].clone();
    v_W = t3126[1].clone();
    v_m = t3126[2].clone();
    v_vo2 = py::mul(&py::mul(&v_MET.clone(), &Py::Float(3.5))?, &v_W.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), v_vo2.clone()), (Py::str("kcal_per_min"), py::truediv(&v_vo2.clone(), &Py::Float(200.0))?), (Py::str("kcal"), py::mul(&py::truediv(&v_vo2.clone(), &Py::Float(200.0))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minute_ventilation(mut v_VT: Py, mut v_RR: Py, mut v_VD: Py) -> R {
    let t3180 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__pos(v_RR.clone(), Py::str("RR"))?, f__nonneg(v_VD.clone(), Py::str("VD"))?])).items()?;
    if t3180.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t3180[0].clone();
    v_RR = t3180[1].clone();
    v_VD = t3180[2].clone();
    let _ = f__need({ let t3181 = v_VD.clone(); Py::Bool({ let t3182 = v_VT.clone(); if py::lt(&t3181, &t3182)? { true } else { false } }) }, Py::str("the dead space must be smaller than the tidal volume"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VE"), py::truediv(&py::mul(&v_VT.clone(), &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VA"), py::truediv(&py::mul(&py::sub(&v_VT.clone(), &v_VD.clone())?, &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VD_VT"), py::truediv(&v_VD.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moens_korteweg(mut v_E: Py, mut v_h: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let t3234 = (Py::tuple(vec![f__pos(v_E.clone(), Py::str("E"))?, f__pos(v_h.clone(), Py::str("h"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t3234.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3234[0].clone();
    v_h = t3234[1].clone();
    v_D = t3234[2].clone();
    v_rho = t3234[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PWV"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_E.clone(), &Py::Float(1000.0))?, &py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_muscle_max_power(mut v_F0: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3286 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?])).items()?;
    if t3286.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t3286[0].clone();
    v_a = t3286[1].clone();
    v_b = t3286[2].clone();
    v_v = py::mul(&v_b.clone(), &py::sub(&py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_F0.clone(), &v_a.clone())?)?)?, &Py::Float(1.0))?)?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("v_opt"), v_v.clone()), (Py::str("F_opt"), v_F.clone()), (Py::str("P_max"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst(mut v_z: Py, mut v_Cout: Py, mut v_Cin: Py, mut v_T: Py) -> R {
    v_z = py::int(&v_z.clone())?;
    let t3338 = (Py::tuple(vec![f__pos(v_Cout.clone(), Py::str("Cout"))?, f__pos(v_Cin.clone(), Py::str("Cin"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3338.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cout = t3338[0].clone();
    v_Cin = t3338[1].clone();
    v_T = t3338[2].clone();
    let _ = f__need({ let t3344 = { let t3339 = v_z.clone(); Py::Bool({ let t3340 = Py::Int(0i64); if py::ne(&t3339, &t3340) { true } else { false } }) }; if !t3344.truthy() { t3344 } else { { let t3341 = py::neg(&Py::Int(3i64))?; Py::Bool({ let t3342 = v_z.clone(); if py::le(&t3341, &t3342)? { { let t3343 = Py::Int(3i64); if py::le(&t3342, &t3343)? { true } else { false } } } else { false } }) } } }, Py::str("the valence must be a nonzero whole number from -3 to 3"))?;
    let _ = f__need({ let t3345 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3346 = v_T.clone(); if py::lt(&t3345, &t3346)? { { let t3347 = Py::Int(100i64); if py::lt(&t3346, &t3347)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("E"), py::mul(&py::truediv(&f__rtf(v_T.clone())?, &v_z.clone())?, &py::math1("log", &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?)?), (Py::str("RT_F"), f__rtf(v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nerve_conduction_velocity(mut v_distance: Py, mut v_latency_proximal: Py, mut v_latency_distal: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t3401 = (Py::tuple(vec![f__pos(v_distance.clone(), Py::str("distance"))?, f__pos(v_latency_proximal.clone(), Py::str("latency_proximal"))?, f__nonneg(v_latency_distal.clone(), Py::str("latency_distal"))?])).items()?;
    if t3401.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t3401[0].clone();
    v_lp = t3401[1].clone();
    v_ld = t3401[2].clone();
    let _ = f__need({ let t3402 = v_lp.clone(); Py::Bool({ let t3403 = v_ld.clone(); if py::gt(&t3402, &t3403)? { true } else { false } }) }, Py::str("the proximal latency must exceed the distal latency"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("NCV"), py::truediv(&v_D.clone(), &py::sub(&v_lp.clone(), &v_ld.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_network_resistance(mut v_R: Py, mut v_arrangement: Py) -> R {
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_R = f__vec(v_R.clone(), Py::str("R"), Py::Int(1i64), Py::Int(10i64))?;
    for t5218 in py::iter(&(v_R.clone()))? {
        v_x = t5218;
        let _ = f__need({ let t5219 = v_x.clone(); Py::Bool({ let t5220 = Py::Int(0i64); if py::gt(&t5219, &t5220)? { true } else { false } }) }, Py::str("every resistance must be positive"))?;
    }
    let _ = f__need({ let t5221 = v_arrangement.clone(); Py::Bool({ let t5222 = Py::tuple(vec![Py::str("series"), Py::str("parallel")]); if py::contains(&t5222, &t5221)? { true } else { false } }) }, Py::str("arrangement must be series or parallel"))?;
    v_tot = (if ({ let t5223 = v_arrangement.clone(); Py::Bool({ let t5224 = Py::str("series"); if py::eq(&t5223, &t5224) { true } else { false } }) }).truthy() { py::sum(&v_R.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t5225 in py::iter(&(v_R.clone()))? { let mut v_x: Py; 
v_x = t5225;
out.push(py::truediv(&Py::Float(1.0), &v_x.clone())?); } Py::list(out) }, None)?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("R_total"), v_tot.clone()), (Py::str("G_total"), py::truediv(&Py::Float(1.0), &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_osm: Py, mut v_T: Py) -> R {
    let mut v_p: Py = Py::None;
    let t3455 = (Py::tuple(vec![f__nonneg(v_osm.clone(), Py::str("osm"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_osm = t3455[0].clone();
    v_T = t3455[1].clone();
    let _ = f__need({ let t3456 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3457 = v_T.clone(); if py::lt(&t3456, &t3457)? { { let t3458 = Py::Int(100i64); if py::lt(&t3457, &t3458)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_p = py::mul(&py::mul(&v_osm.clone(), &c_R_GAS()?)?, &py::add(&v_T.clone(), &c_T0C()?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pi"), v_p.clone()), (Py::str("pi_mmHg"), py::truediv(&v_p.clone(), &c_MMHG()?)?), (Py::str("pi_atm"), py::truediv(&v_p.clone(), &Py::Float(101325.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_consumption(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t3512 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t3512.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3512[0].clone();
    v_Hb = t3512[1].clone();
    v_PaO2 = t3512[2].clone();
    v_PvO2 = t3512[3].clone();
    let t3522 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t3522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t3522[0].clone();
    v_SvO2 = t3522[1].clone();
    let t3523 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t3523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t3523[0].clone();
    v_cv = t3523[1].clone();
    let _ = f__need({ let t3524 = v_cv.clone(); Py::Bool({ let t3525 = v_ca.clone(); if py::lt(&t3524, &t3525)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), py::mul(&py::mul(&v_CO.clone(), &py::sub(&v_ca.clone(), &v_cv.clone())?)?, &Py::Float(10.0))?), (Py::str("O2ER"), py::truediv(&py::sub(&v_ca.clone(), &v_cv.clone())?, &v_ca.clone())?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_content(mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t3588 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3588.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t3588[0].clone();
    v_SaO2 = t3588[1].clone();
    v_PaO2 = t3588[2].clone();
    let t3589 = (Py::tuple(vec![py::mul(&py::mul(&c_O2_HB()?, &v_Hb.clone())?, &v_SaO2.clone())?, py::mul(&c_O2_SOL()?, &v_PaO2.clone())?])).items()?;
    if t3589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3589[0].clone();
    v_d = t3589[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CaO2"), py::add(&v_b.clone(), &v_d.clone())?), (Py::str("bound"), v_b.clone()), (Py::str("dissolved"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_delivery(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_c: Py = Py::None;
    let t3652 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3652.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3652[0].clone();
    v_Hb = t3652[1].clone();
    v_SaO2 = t3652[2].clone();
    v_PaO2 = t3652[3].clone();
    v_c = f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("DO2"), py::mul(&py::mul(&v_CO.clone(), &v_c.clone())?, &Py::Float(10.0))?), (Py::str("CaO2"), v_c.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pah_renal_flow(mut v_U_PAH: Py, mut v_V: Py, mut v_P_PAH: Py, mut v_Hct: Py, mut v_E: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_erpf: Py = Py::None;
    let mut v_rpf: Py = Py::None;
    let t3704 = (Py::tuple(vec![f__pos(v_U_PAH.clone(), Py::str("U_PAH"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P_PAH.clone(), Py::str("P_PAH"))?])).items()?;
    if t3704.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t3704[0].clone();
    v_V = t3704[1].clone();
    v_P = t3704[2].clone();
    let t3714 = (Py::tuple(vec![f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(false), Py::Bool(true))?, f__fr(v_E.clone(), Py::str("E"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t3714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hct = t3714[0].clone();
    v_E = t3714[1].clone();
    v_erpf = py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?;
    v_rpf = py::truediv(&v_erpf.clone(), &v_E.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eRPF"), v_erpf.clone()), (Py::str("RPF"), v_rpf.clone()), (Py::str("RBF"), py::truediv(&v_rpf.clone(), &py::sub(&Py::Float(1.0), &v_Hct.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pco2_from_ph(mut v_pH: Py, mut v_HCO3: Py) -> R {
    let t3766 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?])).items()?;
    if t3766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t3766[0].clone();
    v_HCO3 = t3766[1].clone();
    let _ = f__need({ let t3767 = Py::Float(6.5); Py::Bool({ let t3768 = v_pH.clone(); if py::le(&t3767, &t3768)? { { let t3769 = Py::Float(8.0); if py::le(&t3768, &t3769)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PaCO2"), py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_henderson(mut v_HCO3: Py, mut v_PaCO2: Py) -> R {
    let mut v_ph: Py = Py::None;
    let t3821 = (Py::tuple(vec![f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t3821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HCO3 = t3821[0].clone();
    v_PaCO2 = t3821[1].clone();
    v_ph = py::add(&c_PK_CO2()?, &py::math1("log10", &py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pH"), v_ph.clone()), (Py::str("H"), py::pow(&Py::Float(10.0), &py::sub(&Py::Float(9.0), &v_ph.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_osmolality(mut v_Na: Py, mut v_glucose: Py, mut v_BUN: Py, mut v_measured: Py) -> R {
    let mut v_calc: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t3875 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__nonneg(v_glucose.clone(), Py::str("glucose"))?, f__nonneg(v_BUN.clone(), Py::str("BUN"))?])).items()?;
    if t3875.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t3875[0].clone();
    v_glucose = t3875[1].clone();
    v_BUN = t3875[2].clone();
    v_calc = py::add(&py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?, &py::truediv(&v_BUN.clone(), &Py::Float(2.8))?)?;
    v_out = Py::dict(vec![(Py::str("calculated"), v_calc.clone()), (Py::str("effective"), py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?)]);
    if ({ let t3876 = v_measured.clone(); Py::Bool({ let t3877 = Py::None; if !py::is(&t3876, &t3877) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("osmolal_gap"), py::sub(&f__pos(v_measured.clone(), Py::str("measured"))?, &v_calc.clone())?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poiseuille_flow(mut v_dP: Py, mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_a: Py = Py::None;
    let t3929 = (Py::tuple(vec![f__pos(v_dP.clone(), Py::str("dP"))?, f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3929.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dP = t3929[0].clone();
    v_eta = t3929[1].clone();
    v_L = t3929[2].clone();
    v_r = t3929[3].clone();
    v_Q = py::truediv(&py::mul(&v_dP.clone(), &c_MMHG()?)?, &f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?)?;
    v_a = py::mul(&v_r.clone(), &Py::Float(0.001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q"), py::mul(&v_Q.clone(), &Py::Float(1000000.0))?), (Py::str("Q_mL_min"), py::mul(&v_Q.clone(), &Py::Float(60000000.0))?), (Py::str("v_mean"), py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&c_PI()?, &v_a.clone())?, &v_a.clone())?)?, &Py::Float(100.0))?), (Py::str("tau_wall"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_eta.clone())?, &Py::Float(0.001))?, &v_Q.clone())?, &py::mul(&c_PI()?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poiseuille_resistance(mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    let mut v_R: Py = Py::None;
    let t3981 = (Py::tuple(vec![f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3981.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eta = t3981[0].clone();
    v_L = t3981[1].clone();
    v_r = t3981[2].clone();
    v_R = f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_mmHg"), py::truediv(&py::truediv(&py::mul(&v_R.clone(), &Py::Float(1e-06))?, &Py::Float(60.0))?, &c_MMHG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pulmonary_vascular_resistance(mut v_mPAP: Py, mut v_PCWP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4035 = (Py::tuple(vec![f__num(v_mPAP.clone(), Py::str("mPAP"))?, f__nonneg(v_PCWP.clone(), Py::str("PCWP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4035.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mPAP = t4035[0].clone();
    v_PCWP = t4035[1].clone();
    v_CO = t4035[2].clone();
    let _ = f__need({ let t4036 = v_mPAP.clone(); Py::Bool({ let t4037 = v_PCWP.clone(); if py::gt(&t4036, &t4037)? { true } else { false } }) }, Py::str("the mean pulmonary artery pressure must exceed the wedge pressure"))?;
    v_w = py::truediv(&py::sub(&v_mPAP.clone(), &v_PCWP.clone())?, &v_CO.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PVR"), v_w.clone()), (Py::str("PVR_dyn"), py::mul(&v_w.clone(), &c_WOOD_DYN()?)?), (Py::str("TPG"), py::sub(&v_mPAP.clone(), &v_PCWP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qp_qs(mut v_SaO2: Py, mut v_SvO2: Py, mut v_SpvO2: Py, mut v_SpaO2: Py) -> R {
    let mut v_Sa: Py = Py::None;
    let mut v_Spa: Py = Py::None;
    let mut v_Spv: Py = Py::None;
    let mut v_Sv: Py = Py::None;
    let t4095 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpvO2.clone(), Py::str("SpvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpaO2.clone(), Py::str("SpaO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4095.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Sa = t4095[0].clone();
    v_Sv = t4095[1].clone();
    v_Spv = t4095[2].clone();
    v_Spa = t4095[3].clone();
    let _ = f__need({ let t4100 = { let t4096 = v_Sa.clone(); Py::Bool({ let t4097 = v_Sv.clone(); if py::gt(&t4096, &t4097)? { true } else { false } }) }; if !t4100.truthy() { t4100 } else { { let t4098 = v_Spv.clone(); Py::Bool({ let t4099 = v_Spa.clone(); if py::gt(&t4098, &t4099)? { true } else { false } }) } } }, Py::str("need SaO2 > SvO2 and SpvO2 > SpaO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qp_Qs"), py::truediv(&py::sub(&v_Sa.clone(), &v_Sv.clone())?, &py::sub(&v_Spv.clone(), &v_Spa.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qrs_axis(mut v_lead_I: Py, mut v_aVF: Py) -> R {
    let t4142 = (Py::tuple(vec![f__num(v_lead_I.clone(), Py::str("lead_I"))?, f__num(v_aVF.clone(), Py::str("aVF"))?])).items()?;
    if t4142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lead_I = t4142[0].clone();
    v_aVF = t4142[1].clone();
    let _ = f__need({ let t4147 = { let t4143 = v_lead_I.clone(); Py::Bool({ let t4144 = Py::Int(0i64); if py::ne(&t4143, &t4144) { true } else { false } }) }; if t4147.truthy() { t4147 } else { { let t4145 = v_aVF.clone(); Py::Bool({ let t4146 = Py::Int(0i64); if py::ne(&t4145, &t4146) { true } else { false } }) } } }, Py::str("at least one net deflection must be nonzero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("axis"), py::math1("degrees", &py::atan2(&v_aVF.clone(), &v_lead_I.clone())?)?), (Py::str("amplitude"), py::hypot(&[v_lead_I.clone(), v_aVF.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qt_correction(mut v_QT: Py, mut v_HR: Py) -> R {
    let mut v_rr: Py = Py::None;
    let t4192 = (Py::tuple(vec![f__pos(v_QT.clone(), Py::str("QT"))?, f__pos(v_HR.clone(), Py::str("HR"))?])).items()?;
    if t4192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_QT = t4192[0].clone();
    v_HR = t4192[1].clone();
    v_rr = py::truediv(&Py::Float(60.0), &v_HR.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RR"), v_rr.clone()), (Py::str("QTc_Bazett"), py::truediv(&v_QT.clone(), &py::math1("sqrt", &v_rr.clone())?)?), (Py::str("QTc_Fridericia"), py::truediv(&v_QT.clone(), &py::pow(&v_rr.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?), (Py::str("QTc_Framingham"), py::add(&v_QT.clone(), &py::mul(&Py::Float(154.0), &py::sub(&Py::Float(1.0), &v_rr.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radius_flow_ratio(mut v_r1: Py, mut v_r2: Py) -> R {
    let mut v_q: Py = Py::None;
    let t4243 = (Py::tuple(vec![f__pos(v_r1.clone(), Py::str("r1"))?, f__pos(v_r2.clone(), Py::str("r2"))?])).items()?;
    if t4243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t4243[0].clone();
    v_r2 = t4243[1].clone();
    v_q = py::pow(&py::truediv(&v_r2.clone(), &v_r1.clone())?, &Py::Int(4i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("flow_ratio"), v_q.clone()), (Py::str("resistance_ratio"), py::truediv(&Py::Float(1.0), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_red_cell_indices(mut v_Hct: Py, mut v_Hb: Py, mut v_RBC: Py) -> R {
    v_Hct = f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(true), Py::Bool(true))?;
    let t4296 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__pos(v_RBC.clone(), Py::str("RBC"))?])).items()?;
    if t4296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t4296[0].clone();
    v_RBC = t4296[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("MCV"), py::truediv(&py::mul(&v_Hct.clone(), &Py::Float(1000.0))?, &v_RBC.clone())?), (Py::str("MCH"), py::truediv(&py::mul(&v_Hb.clone(), &Py::Float(10.0))?, &v_RBC.clone())?), (Py::str("MCHC"), py::truediv(&v_Hb.clone(), &v_Hct.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_renal_clearance(mut v_U: Py, mut v_V: Py, mut v_P: Py) -> R {
    let t4343 = (Py::tuple(vec![f__nonneg(v_U.clone(), Py::str("U"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P.clone(), Py::str("P"))?])).items()?;
    if t4343.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4343[0].clone();
    v_V = t4343[1].clone();
    v_P = t4343[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?), (Py::str("excretion"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_mechanics(mut v_VT: Py, mut v_Ppeak: Py, mut v_Pplat: Py, mut v_PEEP: Py, mut v_flow: Py) -> R {
    let t4397 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__num(v_Ppeak.clone(), Py::str("Ppeak"))?, f__num(v_Pplat.clone(), Py::str("Pplat"))?, f__nonneg(v_PEEP.clone(), Py::str("PEEP"))?, f__pos(v_flow.clone(), Py::str("flow"))?])).items()?;
    if t4397.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t4397[0].clone();
    v_Ppeak = t4397[1].clone();
    v_Pplat = t4397[2].clone();
    v_PEEP = t4397[3].clone();
    v_flow = t4397[4].clone();
    let _ = f__need({ let t4398 = v_Ppeak.clone(); Py::Bool({ let t4399 = v_Pplat.clone(); if py::ge(&t4398, &t4399)? { { let t4400 = v_PEEP.clone(); if py::gt(&t4399, &t4400)? { true } else { false } } } else { false } }) }, Py::str("need Ppeak >= Pplat > PEEP"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("C_static"), py::truediv(&v_VT.clone(), &py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)?), (Py::str("C_dynamic"), py::truediv(&v_VT.clone(), &py::sub(&v_Ppeak.clone(), &v_PEEP.clone())?)?), (Py::str("R_aw"), py::truediv(&py::sub(&v_Ppeak.clone(), &v_Pplat.clone())?, &py::truediv(&v_flow.clone(), &Py::Float(60.0))?)?), (Py::str("driving_pressure"), py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_time_constant(mut v_R: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t4452 = (Py::tuple(vec![f__pos(v_R.clone(), Py::str("R"))?, f__pos(v_C.clone(), Py::str("C"))?])).items()?;
    if t4452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t4452[0].clone();
    v_C = t4452[1].clone();
    v_tau = py::truediv(&py::mul(&v_R.clone(), &v_C.clone())?, &Py::Float(1000.0))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("t_95"), py::mul(&Py::Float(3.0), &v_tau.clone())?)]);
    if ({ let t4453 = v_t.clone(); Py::Bool({ let t4454 = Py::None; if !py::is(&t4453, &t4454) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction_exhaled"), py::neg(&py::math1("expm1", &py::truediv(&py::neg(&f__nonneg(v_t.clone(), Py::str("t"))?)?, &v_tau.clone())?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_severinghaus_po2(mut v_SO2: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_SO2 = f__fr(v_SO2.clone(), Py::str("SO2"), Py::Bool(true), Py::Bool(true))?;
    v_K = py::truediv(&py::mul(&Py::Float(23400.0), &v_SO2.clone())?, &py::sub(&Py::Float(1.0), &v_SO2.clone())?)?;
    v_q = py::neg(&v_K.clone())?;
    v_d = py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &Py::Float(4.0))?, &py::truediv(&py::pow(&Py::Float(150.0), &Py::Int(3i64))?, &Py::Float(27.0))?)?)?;
    let t4514 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?, py::sub(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?])).items()?;
    if t4514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4514[0].clone();
    v_b = t4514[1].clone();
    v_P = py::add(&py::copysign(&py::pow(&py::abs(&v_a.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_a.clone())?, &py::copysign(&py::pow(&py::abs(&v_b.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_b.clone())?)?;
    for t4515 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t4515;
        v_P = py::sub(&v_P, &py::truediv(&py::sub(&py::add(&py::pow(&v_P.clone(), &Py::Int(3i64))?, &py::mul(&Py::Float(150.0), &v_P.clone())?)?, &v_K.clone())?, &py::add(&py::mul(&py::mul(&Py::Int(3i64), &v_P.clone())?, &v_P.clone())?, &Py::Float(150.0))?)?)?;
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2"), v_P.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_severinghaus_saturation(mut v_PO2: Py) -> R {
    v_PO2 = f__pos(v_PO2.clone(), Py::str("PO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SO2"), f__sev(v_PO2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shunt_fraction(mut v_CcO2: Py, mut v_CaO2: Py, mut v_CvO2: Py) -> R {
    let mut v_Ca: Py = Py::None;
    let mut v_Cc: Py = Py::None;
    let mut v_Cv: Py = Py::None;
    let t4612 = (Py::tuple(vec![f__pos(v_CcO2.clone(), Py::str("CcO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t4612.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cc = t4612[0].clone();
    v_Ca = t4612[1].clone();
    v_Cv = t4612[2].clone();
    let _ = f__need({ let t4613 = v_Cc.clone(); Py::Bool({ let t4614 = v_Ca.clone(); if py::ge(&t4613, &t4614)? { { let t4615 = v_Cv.clone(); if py::gt(&t4614, &t4615)? { true } else { false } } } else { false } }) }, Py::str("need CcO2 >= CaO2 > CvO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qs_Qt"), py::truediv(&py::sub(&v_Cc.clone(), &v_Ca.clone())?, &py::sub(&v_Cc.clone(), &v_Cv.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solution_osmolarity(mut v_conc: Py, mut v_molar_mass: Py, mut v_i: Py, mut v_phi: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4670 = (Py::tuple(vec![f__pos(v_conc.clone(), Py::str("conc"))?, f__pos(v_molar_mass.clone(), Py::str("molar_mass"))?, f__int_ok(v_i.clone(), Py::str("i"))?, f__pos(v_phi.clone(), Py::str("phi"))?])).items()?;
    if t4670.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_conc = t4670[0].clone();
    v_M = t4670[1].clone();
    v_i = t4670[2].clone();
    v_phi = t4670[3].clone();
    v_m = py::mul(&py::truediv(&v_conc.clone(), &v_M.clone())?, &Py::Float(1000.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("molarity"), v_m.clone()), (Py::str("osmolarity"), py::mul(&py::mul(&v_m.clone(), &v_i.clone())?, &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_starling_filtration(mut v_Pc: Py, mut v_Pi: Py, mut v_pic: Py, mut v_pii: Py, mut v_Kf: Py, mut v_sigma: Py) -> R {
    let mut v_nfp: Py = Py::None;
    let t4724 = (Py::tuple(vec![f__num(v_Pc.clone(), Py::str("Pc"))?, f__num(v_Pi.clone(), Py::str("Pi"))?, f__nonneg(v_pic.clone(), Py::str("pic"))?, f__nonneg(v_pii.clone(), Py::str("pii"))?, f__pos(v_Kf.clone(), Py::str("Kf"))?])).items()?;
    if t4724.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t4724[0].clone();
    v_Pi = t4724[1].clone();
    v_pic = t4724[2].clone();
    v_pii = t4724[3].clone();
    v_Kf = t4724[4].clone();
    v_sigma = f__fr(v_sigma.clone(), Py::str("sigma"), Py::Bool(false), Py::Bool(false))?;
    v_nfp = py::sub(&py::sub(&v_Pc.clone(), &v_Pi.clone())?, &py::mul(&v_sigma.clone(), &py::sub(&v_pic.clone(), &v_pii.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("NFP"), v_nfp.clone()), (Py::str("Jv"), py::mul(&v_Kf.clone(), &v_nfp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_strength_duration(mut v_rheobase: Py, mut v_chronaxie: Py, mut v_t: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t4785 = (Py::tuple(vec![f__pos(v_rheobase.clone(), Py::str("rheobase"))?, f__pos(v_chronaxie.clone(), Py::str("chronaxie"))?, f__pos(v_t.clone(), Py::str("t"))?])).items()?;
    if t4785.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t4785[0].clone();
    v_c = t4785[1].clone();
    v_t = t4785[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I_lapicque"), py::truediv(&v_I0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_t.clone())?, &py::math1("log", &Py::Float(2.0))?)?, &v_c.clone())?)?)?)?), (Py::str("I_weiss"), py::mul(&v_I0.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_c.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_volume_ef(mut v_EDV: Py, mut v_ESV: Py) -> R {
    let mut v_sv: Py = Py::None;
    let t4839 = (Py::tuple(vec![f__pos(v_EDV.clone(), Py::str("EDV"))?, f__nonneg(v_ESV.clone(), Py::str("ESV"))?])).items()?;
    if t4839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_EDV = t4839[0].clone();
    v_ESV = t4839[1].clone();
    let _ = f__need({ let t4840 = v_ESV.clone(); Py::Bool({ let t4841 = v_EDV.clone(); if py::lt(&t4840, &t4841)? { true } else { false } }) }, Py::str("ESV must be smaller than EDV"))?;
    v_sv = py::sub(&v_EDV.clone(), &v_ESV.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SV"), v_sv.clone()), (Py::str("EF"), py::truediv(&v_sv.clone(), &v_EDV.clone())?), (Py::str("EF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_sv.clone())?, &v_EDV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_work(mut v_SV: Py, mut v_MAP: Py, mut v_HR: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4893 = (Py::tuple(vec![f__pos(v_SV.clone(), Py::str("SV"))?, f__pos(v_MAP.clone(), Py::str("MAP"))?])).items()?;
    if t4893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SV = t4893[0].clone();
    v_MAP = t4893[1].clone();
    v_w = py::mul(&py::mul(&py::mul(&v_MAP.clone(), &c_MMHG()?)?, &v_SV.clone())?, &Py::Float(1e-06))?;
    v_out = Py::dict(vec![(Py::str("SW"), v_w.clone()), (Py::str("SW_gm"), py::truediv(&v_w.clone(), &py::mul(&c_G0()?, &Py::Float(0.001))?)?)]);
    if ({ let t4894 = v_HR.clone(); Py::Bool({ let t4895 = Py::None; if !py::is(&t4894, &t4895) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("power"), py::truediv(&py::mul(&v_w.clone(), &f__pos(v_HR.clone(), Py::str("HR"))?)?, &Py::Float(60.0))?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_teichholz(mut v_LVIDd: Py, mut v_LVIDs: Py) -> R {
    let mut v_edv: Py = Py::None;
    let mut v_esv: Py = Py::None;
    let t5003 = (Py::tuple(vec![f__pos(v_LVIDd.clone(), Py::str("LVIDd"))?, f__pos(v_LVIDs.clone(), Py::str("LVIDs"))?])).items()?;
    if t5003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_LVIDd = t5003[0].clone();
    v_LVIDs = t5003[1].clone();
    let _ = f__need({ let t5004 = v_LVIDs.clone(); Py::Bool({ let t5005 = v_LVIDd.clone(); if py::lt(&t5004, &t5005)? { true } else { false } }) }, Py::str("the end-systolic diameter must be smaller than the end-diastolic"))?;
    let t5006 = (Py::tuple(vec![f__teich(v_LVIDd.clone())?, f__teich(v_LVIDs.clone())?])).items()?;
    if t5006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edv = t5006[0].clone();
    v_esv = t5006[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("EDV"), v_edv.clone()), (Py::str("ESV"), v_esv.clone()), (Py::str("EF"), py::truediv(&py::sub(&v_edv.clone(), &v_esv.clone())?, &v_edv.clone())?), (Py::str("FS"), py::truediv(&py::sub(&v_LVIDd.clone(), &v_LVIDs.clone())?, &v_LVIDd.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tubular_handling(mut v_GFR: Py, mut v_Px: Py, mut v_Ux: Py, mut v_V: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let t5060 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Px.clone(), Py::str("Px"))?, f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_V.clone(), Py::str("V"))?])).items()?;
    if t5060.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t5060[0].clone();
    v_Px = t5060[1].clone();
    v_Ux = t5060[2].clone();
    v_V = t5060[3].clone();
    let t5061 = (Py::tuple(vec![py::truediv(&py::mul(&v_GFR.clone(), &v_Px.clone())?, &Py::Float(100.0))?, py::truediv(&py::mul(&v_Ux.clone(), &v_V.clone())?, &Py::Float(100.0))?])).items()?;
    if t5061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fl = t5061[0].clone();
    v_ex = t5061[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("filtered_load"), v_fl.clone()), (Py::str("excreted"), v_ex.clone()), (Py::str("reabsorbed"), py::sub(&v_fl.clone(), &v_ex.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vascular_resistance(mut v_MAP: Py, mut v_CVP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4949 = (Py::tuple(vec![f__num(v_MAP.clone(), Py::str("MAP"))?, f__nonneg(v_CVP.clone(), Py::str("CVP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4949.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MAP = t4949[0].clone();
    v_CVP = t4949[1].clone();
    v_CO = t4949[2].clone();
    let _ = f__need({ let t4950 = v_MAP.clone(); Py::Bool({ let t4951 = v_CVP.clone(); if py::gt(&t4950, &t4951)? { true } else { false } }) }, Py::str("the mean arterial pressure must exceed the venous pressure"))?;
    v_w = py::truediv(&py::sub(&v_MAP.clone(), &v_CVP.clone())?, &v_CO.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SVR"), v_w.clone()), (Py::str("SVR_dyn"), py::mul(&v_w.clone(), &c_WOOD_DYN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vessel_reynolds(mut v_rho: Py, mut v_v: Py, mut v_D: Py, mut v_eta: Py) -> R {
    let mut v_Re: Py = Py::None;
    let t5165 = (Py::tuple(vec![f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_v.clone(), Py::str("v"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5165.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t5165[0].clone();
    v_v = t5165[1].clone();
    v_D = t5165[2].clone();
    v_eta = t5165[3].clone();
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &py::truediv(&v_v.clone(), &Py::Float(100.0))?)?, &py::truediv(&v_D.clone(), &Py::Float(1000.0))?)?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Re"), v_Re.clone()), (Py::str("v_critical"), py::mul(&py::truediv(&py::mul(&Py::Float(2000.0), &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_virtual_po2(mut v_PO2: Py, mut v_T: Py, mut v_pH: Py, mut v_PCO2: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let t5270 = (Py::tuple(vec![f__pos(v_PO2.clone(), Py::str("PO2"))?, f__num(v_T.clone(), Py::str("T"))?, f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PCO2.clone(), Py::str("PCO2"))?])).items()?;
    if t5270.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t5270[0].clone();
    v_T = t5270[1].clone();
    v_pH = t5270[2].clone();
    v_PCO2 = t5270[3].clone();
    let _ = f__need({ let t5277 = { let t5271 = Py::Int(25i64); Py::Bool({ let t5272 = v_T.clone(); if py::le(&t5271, &t5272)? { { let t5273 = Py::Int(43i64); if py::le(&t5272, &t5273)? { true } else { false } } } else { false } }) }; if !t5277.truthy() { t5277 } else { { let t5274 = Py::Float(6.8); Py::Bool({ let t5275 = v_pH.clone(); if py::le(&t5274, &t5275)? { { let t5276 = Py::Float(7.8); if py::le(&t5275, &t5276)? { true } else { false } } } else { false } }) } } }, Py::str("temperature 25-43 degC and pH 6.8-7.8"))?;
    v_f = py::pow(&Py::Float(10.0), &py::add(&py::add(&py::mul(&Py::Float(0.024), &py::sub(&Py::Float(37.0), &v_T.clone())?)?, &py::mul(&Py::Float(0.4), &py::sub(&v_pH.clone(), &Py::Float(7.4))?)?)?, &py::mul(&Py::Float(0.06), &py::math1("log10", &py::truediv(&Py::Float(40.0), &v_PCO2.clone())?)?)?)?)?;
    v_pv = py::mul(&v_PO2.clone(), &v_f.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2_virtual"), v_pv.clone()), (Py::str("SO2"), f__sev(v_pv.clone())?), (Py::str("factor"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_watson_tbw(mut v_age: Py, mut v_height: Py, mut v_weight: Py, mut v_sex: Py) -> R {
    let mut v_fem: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t5329 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_height.clone(), Py::str("height"))?, f__pos(v_weight.clone(), Py::str("weight"))?])).items()?;
    if t5329.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t5329[0].clone();
    v_height = t5329[1].clone();
    v_weight = t5329[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t5334 = v_age.clone(); Py::Bool({ let t5335 = Py::Int(18i64); if py::ge(&t5334, &t5335)? { true } else { false } }) }, Py::str("the Watson equations are for adults"))?;
    v_v = (if (v_fem.clone()).truthy() { py::add(&py::add(&py::neg(&Py::Float(2.097))?, &py::mul(&Py::Float(0.1069), &v_height.clone())?)?, &py::mul(&Py::Float(0.2466), &v_weight.clone())?)? } else { py::add(&py::add(&py::sub(&Py::Float(2.447), &py::mul(&Py::Float(0.09516), &v_age.clone())?)?, &py::mul(&Py::Float(0.1074), &v_height.clone())?)?, &py::mul(&Py::Float(0.3362), &v_weight.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("TBW"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weir_rq(mut v_VO2: Py, mut v_VCO2: Py) -> R {
    let mut v_ee: Py = Py::None;
    let t5387 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_VCO2.clone(), Py::str("VCO2"))?])).items()?;
    if t5387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t5387[0].clone();
    v_VCO2 = t5387[1].clone();
    v_ee = py::mul(&py::add(&py::truediv(&py::mul(&Py::Float(3.941), &v_VO2.clone())?, &Py::Float(1000.0))?, &py::truediv(&py::mul(&Py::Float(1.106), &v_VCO2.clone())?, &Py::Float(1000.0))?)?, &Py::Float(1440.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RQ"), py::truediv(&v_VCO2.clone(), &v_VO2.clone())?), (Py::str("EE"), v_ee.clone()), (Py::str("EE_kJ"), py::mul(&v_ee.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_womersley(mut v_r: Py, mut v_HR: Py, mut v_rho: Py, mut v_eta: Py) -> R {
    let mut v_w: Py = Py::None;
    let t5439 = (Py::tuple(vec![f__pos(v_r.clone(), Py::str("r"))?, f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5439.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_r = t5439[0].clone();
    v_HR = t5439[1].clone();
    v_rho = t5439[2].clone();
    v_eta = t5439[3].clone();
    v_w = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &c_PI()?)?, &v_HR.clone())?, &Py::Float(60.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("alpha"), py::mul(&py::truediv(&v_r.clone(), &Py::Float(1000.0))?, &py::math1("sqrt", &py::truediv(&py::mul(&v_w.clone(), &v_rho.clone())?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?)?)?), (Py::str("omega"), v_w.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1(mut v_kw: Py) -> R {
    return Ok({ let t31 = f__from_si(Py::dict(vec![(Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_aa_gradient(py::getitem(&t31, &Py::str("FiO2"))?, py::getitem(&t31, &Py::str("Patm"))?, py::getitem(&t31, &Py::str("PH2O"))?, py::getitem(&t31, &Py::str("PaCO2"))?, py::getitem(&t31, &Py::str("PaO2"))?, py::getitem(&t31, &Py::str("RQ"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1025(mut v_kw: Py) -> R {
    return Ok({ let t1040 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_index(py::getitem(&t1040, &Py::str("CO"))?, py::getitem(&t1040, &Py::str("BSA"))?, py::method(&t1040, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1047(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1049: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1025(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1049 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1050 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1078 = (t1050).items()?;
        if t1078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1078[0].clone();
        v_w = t1078[1].clone();
        if ({ let t1051 = v_k.clone(); Py::Bool({ let t1052 = v_outputs.clone(); if !py::contains(&t1052, &t1051)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_105(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_70(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t108 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t136 = (t108).items()?;
        if t136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t136[0].clone();
        v_w = t136[1].clone();
        if ({ let t109 = v_k.clone(); Py::Bool({ let t110 = v_outputs.clone(); if !py::contains(&t110, &t109)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1079(mut v_kw: Py) -> R {
    return Ok({ let t1092 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_output(py::getitem(&t1092, &Py::str("HR"))?, py::getitem(&t1092, &Py::str("SV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1099(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1101: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1079(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1101 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1102 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1130 = (t1102).items()?;
        if t1130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1130[0].clone();
        v_w = t1130[1].clone();
        if ({ let t1103 = v_k.clone(); Py::Bool({ let t1104 = v_outputs.clone(); if !py::contains(&t1104, &t1103)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1131(mut v_kw: Py) -> R {
    return Ok({ let t1147 = f__from_si(Py::dict(vec![(Py::str("EK"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ENa"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ECl"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_chord_conductance(py::getitem(&t1147, &Py::str("gK"))?, py::getitem(&t1147, &Py::str("gNa"))?, py::getitem(&t1147, &Py::str("gCl"))?, py::getitem(&t1147, &Py::str("EK"))?, py::getitem(&t1147, &Py::str("ENa"))?, py::getitem(&t1147, &Py::str("ECl"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1154(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1156: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1131(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1156 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1157 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1185 = (t1157).items()?;
        if t1185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1185[0].clone();
        v_w = t1185[1].clone();
        if ({ let t1158 = v_k.clone(); Py::Bool({ let t1159 = v_outputs.clone(); if !py::contains(&t1159, &t1158)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1186(mut v_kw: Py) -> R {
    return Ok({ let t1206 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cockcroft_gault(py::getitem(&t1206, &Py::str("age"))?, py::getitem(&t1206, &Py::str("weight"))?, py::getitem(&t1206, &Py::str("Scr"))?, py::getitem(&t1206, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1213(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1215: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1186(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1215 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1216 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1244 = (t1216).items()?;
        if t1244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1244[0].clone();
        v_w = t1244[1].clone();
        if ({ let t1217 = v_k.clone(); Py::Bool({ let t1218 = v_outputs.clone(); if !py::contains(&t1218, &t1217)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1245(mut v_kw: Py) -> R {
    return Ok({ let t1262 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("d1"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("d2"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_conduction_velocity_scaling(py::getitem(&t1262, &Py::str("v1"))?, py::getitem(&t1262, &Py::str("d1"))?, py::getitem(&t1262, &Py::str("d2"))?, py::getitem(&t1262, &Py::str("fiber"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1269(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1245(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1300 = (t1272).items()?;
        if t1300.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1300[0].clone();
        v_w = t1300[1].clone();
        if ({ let t1273 = v_k.clone(); Py::Bool({ let t1274 = v_outputs.clone(); if !py::contains(&t1274, &t1273)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1301(mut v_kw: Py) -> R {
    return Ok({ let t1314 = f__from_si(Py::dict(vec![(Py::str("Vurine"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("hours"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_creatinine_clearance_timed(py::getitem(&t1314, &Py::str("Ucr"))?, py::getitem(&t1314, &Py::str("Vurine"))?, py::getitem(&t1314, &Py::str("hours"))?, py::getitem(&t1314, &Py::str("Pcr"))?, py::method(&t1314, "get", &[Py::str("BSA"), Py::Float(1.73)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1301(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1352 = (t1324).items()?;
        if t1352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1352[0].clone();
        v_w = t1352[1].clone();
        if ({ let t1325 = v_k.clone(); Py::Bool({ let t1326 = v_outputs.clone(); if !py::contains(&t1326, &t1325)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1353(mut v_kw: Py) -> R {
    return Ok({ let t1379 = f__from_si(Py::dict(vec![(Py::str("RR"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_ecg_heart_rate(py::method(&t1379, "get", &[Py::str("RR"), Py::None])?, py::method(&t1379, "get", &[Py::str("small_boxes"), Py::None])?, py::method(&t1379, "get", &[Py::str("large_boxes"), Py::None])?, py::method(&t1379, "get", &[Py::str("speed"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_137(mut v_kw: Py) -> R {
    return Ok({ let t150 = f__from_si(Py::dict(vec![(Py::str("gamma"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_laplace(py::getitem(&t150, &Py::str("gamma"))?, py::getitem(&t150, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1353(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1389 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1417 = (t1389).items()?;
        if t1417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1417[0].clone();
        v_w = t1417[1].clone();
        if ({ let t1390 = v_k.clone(); Py::Bool({ let t1391 = v_outputs.clone(); if !py::contains(&t1391, &t1390)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1438(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1440: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1439 = v_inputs.clone(); f_egfr_ckd_epi_2021(py::getitem(&t1439, &Py::str("Scr"))?, py::getitem(&t1439, &Py::str("age"))?, py::getitem(&t1439, &Py::str("sex"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1440 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1441 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1469 = (t1441).items()?;
        if t1469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1469[0].clone();
        v_w = t1469[1].clone();
        if ({ let t1442 = v_k.clone(); Py::Bool({ let t1443 = v_outputs.clone(); if !py::contains(&t1443, &t1442)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1470(mut v_kw: Py) -> R {
    return Ok({ let t1492 = f__from_si(Py::dict(vec![(Py::str("Scys"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_egfr_ckd_epi_2021_cys(py::getitem(&t1492, &Py::str("Scr"))?, py::getitem(&t1492, &Py::str("Scys"))?, py::getitem(&t1492, &Py::str("age"))?, py::getitem(&t1492, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1499(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1470(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1501 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1502 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1530 = (t1502).items()?;
        if t1530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1530[0].clone();
        v_w = t1530[1].clone();
        if ({ let t1503 = v_k.clone(); Py::Bool({ let t1504 = v_outputs.clone(); if !py::contains(&t1504, &t1503)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1531(mut v_kw: Py) -> R {
    return Ok({ let t1548 = f__from_si(Py::dict(vec![(Py::str("CL"), Py::tuple(vec![Py::Float(2.7777777777777776e-07), Py::Float(0.0)])), (Py::str("Vd"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)]))]), v_kw.clone())?; f_elimination_half_life(py::getitem(&t1548, &Py::str("CL"))?, py::getitem(&t1548, &Py::str("Vd"))?, py::method(&t1548, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1555(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1557: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1531(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1557 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1558 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1586 = (t1558).items()?;
        if t1586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1586[0].clone();
        v_w = t1586[1].clone();
        if ({ let t1559 = v_k.clone(); Py::Bool({ let t1560 = v_outputs.clone(); if !py::contains(&t1560, &t1559)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_137(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t160 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t188 = (t160).items()?;
        if t188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t188[0].clone();
        v_w = t188[1].clone();
        if ({ let t161 = v_k.clone(); Py::Bool({ let t162 = v_outputs.clone(); if !py::contains(&t162, &t161)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1587(mut v_kw: Py) -> R {
    return Ok({ let t1602 = f__from_si(Py::dict(vec![(Py::str("FEV1"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("FVC"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fev1_fvc(py::getitem(&t1602, &Py::str("FEV1"))?, py::getitem(&t1602, &Py::str("FVC"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1609(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1611: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1587(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1611 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1612 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1640 = (t1612).items()?;
        if t1640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1640[0].clone();
        v_w = t1640[1].clone();
        if ({ let t1613 = v_k.clone(); Py::Bool({ let t1614 = v_outputs.clone(); if !py::contains(&t1614, &t1613)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1641(mut v_kw: Py) -> R {
    return Ok({ let t1658 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_cardiac_output(py::getitem(&t1658, &Py::str("VO2"))?, py::getitem(&t1658, &Py::str("CaO2"))?, py::getitem(&t1658, &Py::str("CvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1665(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1667: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1641(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1667 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1668 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1696 = (t1668).items()?;
        if t1696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1696[0].clone();
        v_w = t1696[1].clone();
        if ({ let t1669 = v_k.clone(); Py::Bool({ let t1670 = v_outputs.clone(); if !py::contains(&t1670, &t1669)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1697(mut v_kw: Py) -> R {
    return Ok({ let t1710 = f__from_si(Py::dict(vec![(Py::str("D"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("dC"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_diffusion(py::getitem(&t1710, &Py::str("D"))?, py::getitem(&t1710, &Py::str("A"))?, py::getitem(&t1710, &Py::str("dC"))?, py::getitem(&t1710, &Py::str("T"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1697(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1748 = (t1720).items()?;
        if t1748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1748[0].clone();
        v_w = t1748[1].clone();
        if ({ let t1721 = v_k.clone(); Py::Bool({ let t1722 = v_outputs.clone(); if !py::contains(&t1722, &t1721)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1749(mut v_kw: Py) -> R {
    return Ok({ let t1777 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_saturations(py::getitem(&t1777, &Py::str("VO2"))?, py::getitem(&t1777, &Py::str("Hb"))?, py::getitem(&t1777, &Py::str("SaO2"))?, py::getitem(&t1777, &Py::str("SvO2"))?, py::getitem(&t1777, &Py::str("PaO2"))?, py::getitem(&t1777, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1749(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1787 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1815 = (t1787).items()?;
        if t1815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1815[0].clone();
        v_w = t1815[1].clone();
        if ({ let t1788 = v_k.clone(); Py::Bool({ let t1789 = v_outputs.clone(); if !py::contains(&t1789, &t1788)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1816(mut v_kw: Py) -> R {
    return Ok({ let t1831 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("RPF"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_filtration_fraction(py::getitem(&t1831, &Py::str("GFR"))?, py::getitem(&t1831, &Py::str("RPF"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1840: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1816(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1840 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1841 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1869 = (t1841).items()?;
        if t1869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1869[0].clone();
        v_w = t1869[1].clone();
        if ({ let t1842 = v_k.clone(); Py::Bool({ let t1843 = v_outputs.clone(); if !py::contains(&t1843, &t1842)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1870(mut v_kw: Py) -> R {
    return Ok({ let t1895 = f__from_si(Py::dict(vec![(Py::str("TBW"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("V_added"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_shift(py::getitem(&t1895, &Py::str("TBW"))?, py::getitem(&t1895, &Py::str("Posm"))?, py::getitem(&t1895, &Py::str("V_added"))?, py::getitem(&t1895, &Py::str("C_added"))?, py::method(&t1895, "get", &[Py::str("ICF_fraction"), py::truediv(&Py::Float(2.0), &Py::Float(3.0))?])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_189(mut v_kw: Py) -> R {
    return Ok({ let t202 = f__from_si(Py::dict(vec![(Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_ventilation_co2(py::getitem(&t202, &Py::str("VCO2"))?, py::getitem(&t202, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1902(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1904: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1870(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1904 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1905 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1933 = (t1905).items()?;
        if t1933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1933[0].clone();
        v_w = t1933[1].clone();
        if ({ let t1906 = v_k.clone(); Py::Bool({ let t1907 = v_outputs.clone(); if !py::contains(&t1907, &t1906)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1948(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1950: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1949 = v_inputs.clone(); f_fractional_excretion(py::getitem(&t1949, &Py::str("Ux"))?, py::getitem(&t1949, &Py::str("Px"))?, py::getitem(&t1949, &Py::str("Ucr"))?, py::getitem(&t1949, &Py::str("Pcr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1950 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1951 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1979 = (t1951).items()?;
        if t1979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1979[0].clone();
        v_w = t1979[1].clone();
        if ({ let t1952 = v_k.clone(); Py::Bool({ let t1953 = v_outputs.clone(); if !py::contains(&t1953, &t1952)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1980(mut v_kw: Py) -> R {
    return Ok({ let t1993 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_free_water_clearance(py::getitem(&t1993, &Py::str("V"))?, py::getitem(&t1993, &Py::str("Uosm"))?, py::getitem(&t1993, &Py::str("Posm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2000(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2002: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1980(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2002 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2003 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2031 = (t2003).items()?;
        if t2031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2031[0].clone();
        v_w = t2031[1].clone();
        if ({ let t2004 = v_k.clone(); Py::Bool({ let t2005 = v_outputs.clone(); if !py::contains(&t2005, &t2004)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2032(mut v_kw: Py) -> R {
    return Ok({ let t2056 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_gas_volume_conversion(py::getitem(&t2056, &Py::str("V"))?, py::getitem(&t2056, &Py::str("T"))?, py::getitem(&t2056, &Py::str("Patm"))?, py::getitem(&t2056, &Py::str("PH2O"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2063(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2065: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2032(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2065 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2066 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2094 = (t2066).items()?;
        if t2094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2094[0].clone();
        v_w = t2094[1].clone();
        if ({ let t2067 = v_k.clone(); Py::Bool({ let t2068 = v_outputs.clone(); if !py::contains(&t2068, &t2067)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t211: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_189(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t211 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t212 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t240 = (t212).items()?;
        if t240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t240[0].clone();
        v_w = t240[1].clone();
        if ({ let t213 = v_k.clone(); Py::Bool({ let t214 = v_outputs.clone(); if !py::contains(&t214, &t213)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2095(mut v_kw: Py) -> R {
    return Ok({ let t2115 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_ghk_voltage(py::getitem(&t2115, &Py::str("PK"))?, py::getitem(&t2115, &Py::str("PNa"))?, py::getitem(&t2115, &Py::str("PCl"))?, py::getitem(&t2115, &Py::str("K_o"))?, py::getitem(&t2115, &Py::str("K_i"))?, py::getitem(&t2115, &Py::str("Na_o"))?, py::getitem(&t2115, &Py::str("Na_i"))?, py::getitem(&t2115, &Py::str("Cl_o"))?, py::getitem(&t2115, &Py::str("Cl_i"))?, py::method(&t2115, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2122(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2124: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2095(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2124 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2125 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2153 = (t2125).items()?;
        if t2153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2153[0].clone();
        v_w = t2153[1].clone();
        if ({ let t2126 = v_k.clone(); Py::Bool({ let t2127 = v_outputs.clone(); if !py::contains(&t2127, &t2126)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2154(mut v_kw: Py) -> R {
    return Ok({ let t2170 = f__from_si(Py::dict(vec![(Py::str("c"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_gibbs_donnan(py::getitem(&t2170, &Py::str("A"))?, py::getitem(&t2170, &Py::str("c"))?, py::method(&t2170, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2179: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2154(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2179 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2180 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2208 = (t2180).items()?;
        if t2208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2208[0].clone();
        v_w = t2208[1].clone();
        if ({ let t2181 = v_k.clone(); Py::Bool({ let t2182 = v_outputs.clone(); if !py::contains(&t2182, &t2181)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2209(mut v_kw: Py) -> R {
    return Ok({ let t2224 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("Tm"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_glucose_titration(py::getitem(&t2224, &Py::str("GFR"))?, py::getitem(&t2224, &Py::str("Pglucose"))?, py::getitem(&t2224, &Py::str("Tm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2231(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2233: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2209(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2233 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2234 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2262 = (t2234).items()?;
        if t2262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2262[0].clone();
        v_w = t2262[1].clone();
        if ({ let t2235 = v_k.clone(); Py::Bool({ let t2236 = v_outputs.clone(); if !py::contains(&t2236, &t2235)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2289(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2291: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2290 = v_inputs.clone(); f_heart_rate_reserve(py::getitem(&t2290, &Py::str("age"))?, py::getitem(&t2290, &Py::str("HR_rest"))?, py::getitem(&t2290, &Py::str("intensity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2291 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2292 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2320 = (t2292).items()?;
        if t2320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2320[0].clone();
        v_w = t2320[1].clone();
        if ({ let t2293 = v_k.clone(); Py::Bool({ let t2294 = v_outputs.clone(); if !py::contains(&t2294, &t2293)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2321(mut v_kw: Py) -> R {
    return Ok({ let t2351 = f__from_si(Py::dict(vec![(Py::str("P1"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_fit(py::getitem(&t2351, &Py::str("P1"))?, py::getitem(&t2351, &Py::str("S1"))?, py::getitem(&t2351, &Py::str("P2"))?, py::getitem(&t2351, &Py::str("S2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2358(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2360: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2321(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2360 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2361 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2389 = (t2361).items()?;
        if t2389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2389[0].clone();
        v_w = t2389[1].clone();
        if ({ let t2362 = v_k.clone(); Py::Bool({ let t2363 = v_outputs.clone(); if !py::contains(&t2363, &t2362)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2390(mut v_kw: Py) -> R {
    return Ok({ let t2407 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_force_velocity(py::getitem(&t2407, &Py::str("F0"))?, py::getitem(&t2407, &Py::str("a"))?, py::getitem(&t2407, &Py::str("b"))?, py::getitem(&t2407, &Py::str("v"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_241(mut v_kw: Py) -> R {
    return Ok({ let t254 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("Cl"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_anion_gap(py::getitem(&t254, &Py::str("Na"))?, py::getitem(&t254, &Py::str("Cl"))?, py::getitem(&t254, &Py::str("HCO3"))?, py::method(&t254, "get", &[Py::str("albumin"), Py::Float(4.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2414(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2416: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2390(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2416 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2417 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2445 = (t2417).items()?;
        if t2445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2445[0].clone();
        v_w = t2445[1].clone();
        if ({ let t2418 = v_k.clone(); Py::Bool({ let t2419 = v_outputs.clone(); if !py::contains(&t2419, &t2418)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2446(mut v_kw: Py) -> R {
    return Ok({ let t2468 = f__from_si(Py::dict(vec![(Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_po2(py::getitem(&t2468, &Py::str("SO2"))?, py::getitem(&t2468, &Py::str("P50"))?, py::getitem(&t2468, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2446(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2506 = (t2478).items()?;
        if t2506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2506[0].clone();
        v_w = t2506[1].clone();
        if ({ let t2479 = v_k.clone(); Py::Bool({ let t2480 = v_outputs.clone(); if !py::contains(&t2480, &t2479)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2507(mut v_kw: Py) -> R {
    return Ok({ let t2522 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_saturation(py::getitem(&t2522, &Py::str("PO2"))?, py::getitem(&t2522, &Py::str("P50"))?, py::getitem(&t2522, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2507(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2560 = (t2532).items()?;
        if t2560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2560[0].clone();
        v_w = t2560[1].clone();
        if ({ let t2533 = v_k.clone(); Py::Bool({ let t2534 = v_outputs.clone(); if !py::contains(&t2534, &t2533)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2561(mut v_kw: Py) -> R {
    return Ok({ let t2574 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_column(py::getitem(&t2574, &Py::str("h"))?, py::getitem(&t2574, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2581(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2583: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2561(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2583 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2584 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2612 = (t2584).items()?;
        if t2612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2612[0].clone();
        v_w = t2612[1].clone();
        if ({ let t2585 = v_k.clone(); Py::Bool({ let t2586 = v_outputs.clone(); if !py::contains(&t2586, &t2585)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_261(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t263: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_241(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t263 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t264 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t292 = (t264).items()?;
        if t292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t292[0].clone();
        v_w = t292[1].clone();
        if ({ let t265 = v_k.clone(); Py::Bool({ let t266 = v_outputs.clone(); if !py::contains(&t266, &t265)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2613(mut v_kw: Py) -> R {
    return Ok({ let t2639 = f__from_si(Py::dict(vec![(Py::str("mass_air"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("mass_water"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("water_density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("GI"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_weighing(py::getitem(&t2639, &Py::str("mass_air"))?, py::getitem(&t2639, &Py::str("mass_water"))?, py::getitem(&t2639, &Py::str("water_density"))?, py::getitem(&t2639, &Py::str("RV"))?, py::method(&t2639, "get", &[Py::str("GI"), Py::Float(0.1)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2646(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2648: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2613(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2648 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2649 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2677 = (t2649).items()?;
        if t2677.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2677[0].clone();
        v_w = t2677[1].clone();
        if ({ let t2650 = v_k.clone(); Py::Bool({ let t2651 = v_outputs.clone(); if !py::contains(&t2651, &t2650)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2678(mut v_kw: Py) -> R {
    return Ok({ let t2691 = f__from_si(Py::dict(vec![(Py::str("g"), Py::tuple(vec![Py::Float(1e-09), Py::Float(0.0)])), (Py::str("Vm"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("E"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_ionic_current(py::getitem(&t2691, &Py::str("g"))?, py::getitem(&t2691, &Py::str("Vm"))?, py::getitem(&t2691, &Py::str("E"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2678(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2729 = (t2701).items()?;
        if t2729.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2729[0].clone();
        v_w = t2729[1].clone();
        if ({ let t2702 = v_k.clone(); Py::Bool({ let t2703 = v_outputs.clone(); if !py::contains(&t2703, &t2702)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2730(mut v_kw: Py) -> R {
    return Ok({ let t2749 = f__from_si(Py::dict(vec![(Py::str("P"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_laplace_wall_tension(py::getitem(&t2749, &Py::str("P"))?, py::getitem(&t2749, &Py::str("r"))?, py::getitem(&t2749, &Py::str("shape"))?, py::method(&t2749, "get", &[Py::str("h"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2756(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2758: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2730(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2758 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2759 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2787 = (t2759).items()?;
        if t2787.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2787[0].clone();
        v_w = t2787[1].clone();
        if ({ let t2760 = v_k.clone(); Py::Bool({ let t2761 = v_outputs.clone(); if !py::contains(&t2761, &t2760)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2788(mut v_kw: Py) -> R {
    return Ok({ let t2808 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_lean_body_mass_boer(py::getitem(&t2808, &Py::str("weight"))?, py::getitem(&t2808, &Py::str("height"))?, py::getitem(&t2808, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2815(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2817: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2788(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2817 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2818 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2846 = (t2818).items()?;
        if t2846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2846[0].clone();
        v_w = t2846[1].clone();
        if ({ let t2819 = v_k.clone(); Py::Bool({ let t2820 = v_outputs.clone(); if !py::contains(&t2820, &t2819)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2847(mut v_kw: Py) -> R {
    return Ok({ let t2864 = f__from_si(Py::dict(vec![(Py::str("d"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("x"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_length_constant(py::getitem(&t2864, &Py::str("Rm"))?, py::getitem(&t2864, &Py::str("Ri"))?, py::getitem(&t2864, &Py::str("d"))?, py::method(&t2864, "get", &[Py::str("x"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2871(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2873: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2847(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2873 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2874 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2902 = (t2874).items()?;
        if t2902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2902[0].clone();
        v_w = t2902[1].clone();
        if ({ let t2875 = v_k.clone(); Py::Bool({ let t2876 = v_outputs.clone(); if !py::contains(&t2876, &t2875)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2903(mut v_kw: Py) -> R {
    return Ok({ let t2918 = f__from_si(Py::dict(vec![(Py::str("TV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("IRV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ERV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_lung_volumes(py::getitem(&t2918, &Py::str("TV"))?, py::getitem(&t2918, &Py::str("IRV"))?, py::getitem(&t2918, &Py::str("ERV"))?, py::getitem(&t2918, &Py::str("RV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2925(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2927: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2903(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2927 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2928 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2956 = (t2928).items()?;
        if t2956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2956[0].clone();
        v_w = t2956[1].clone();
        if ({ let t2929 = v_k.clone(); Py::Bool({ let t2930 = v_outputs.clone(); if !py::contains(&t2930, &t2929)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_293(mut v_kw: Py) -> R {
    return Ok({ let t308 = f__from_si(Py::dict(vec![(Py::str("carbohydrate"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("fat"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("protein"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("alcohol"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_atwater_energy(py::getitem(&t308, &Py::str("carbohydrate"))?, py::getitem(&t308, &Py::str("fat"))?, py::getitem(&t308, &Py::str("protein"))?, py::method(&t308, "get", &[Py::str("alcohol"), Py::Float(0.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2957(mut v_kw: Py) -> R {
    return Ok({ let t2972 = f__from_si(Py::dict(vec![(Py::str("SBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("DBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_mean_arterial_pressure(py::getitem(&t2972, &Py::str("SBP"))?, py::getitem(&t2972, &Py::str("DBP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2979(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2981: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2957(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2981 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2982 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3010 = (t2982).items()?;
        if t3010.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3010[0].clone();
        v_w = t3010[1].clone();
        if ({ let t2983 = v_k.clone(); Py::Bool({ let t2984 = v_outputs.clone(); if !py::contains(&t2984, &t2983)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3011(mut v_kw: Py) -> R {
    return Ok({ let t3026 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("dV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("area"), Py::tuple(vec![Py::Float(1e-12), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_charge(py::getitem(&t3026, &Py::str("Cm"))?, py::getitem(&t3026, &Py::str("dV"))?, py::getitem(&t3026, &Py::str("area"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3033(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3035: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3011(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3035 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3036 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3064 = (t3036).items()?;
        if t3064.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3064[0].clone();
        v_w = t3064[1].clone();
        if ({ let t3037 = v_k.clone(); Py::Bool({ let t3038 = v_outputs.clone(); if !py::contains(&t3038, &t3037)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3065(mut v_kw: Py) -> R {
    return Ok({ let t3082 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_time_constant(py::getitem(&t3082, &Py::str("Rm"))?, py::getitem(&t3082, &Py::str("Cm"))?, py::method(&t3082, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3089(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3091: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3065(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3091 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3092 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3120 = (t3092).items()?;
        if t3120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3120[0].clone();
        v_w = t3120[1].clone();
        if ({ let t3093 = v_k.clone(); Py::Bool({ let t3094 = v_outputs.clone(); if !py::contains(&t3094, &t3093)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3121(mut v_kw: Py) -> R {
    return Ok({ let t3134 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("duration"), Py::tuple(vec![Py::Float(60.0), Py::Float(0.0)]))]), v_kw.clone())?; f_met_energy(py::getitem(&t3134, &Py::str("MET"))?, py::getitem(&t3134, &Py::str("weight"))?, py::getitem(&t3134, &Py::str("duration"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3141(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3143: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3121(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3143 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3144 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3172 = (t3144).items()?;
        if t3172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3172[0].clone();
        v_w = t3172[1].clone();
        if ({ let t3145 = v_k.clone(); Py::Bool({ let t3146 = v_outputs.clone(); if !py::contains(&t3146, &t3145)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_315(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t317: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_293(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t317 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t318 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t346 = (t318).items()?;
        if t346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t346[0].clone();
        v_w = t346[1].clone();
        if ({ let t319 = v_k.clone(); Py::Bool({ let t320 = v_outputs.clone(); if !py::contains(&t320, &t319)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3173(mut v_kw: Py) -> R {
    return Ok({ let t3190 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("VD"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_minute_ventilation(py::getitem(&t3190, &Py::str("VT"))?, py::getitem(&t3190, &Py::str("RR"))?, py::getitem(&t3190, &Py::str("VD"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3197(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3199: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3173(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3199 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3200 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3228 = (t3200).items()?;
        if t3228.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3228[0].clone();
        v_w = t3228[1].clone();
        if ({ let t3201 = v_k.clone(); Py::Bool({ let t3202 = v_outputs.clone(); if !py::contains(&t3202, &t3201)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3229(mut v_kw: Py) -> R {
    return Ok({ let t3242 = f__from_si(Py::dict(vec![(Py::str("E"), Py::tuple(vec![Py::Float(1000.0), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_moens_korteweg(py::getitem(&t3242, &Py::str("E"))?, py::getitem(&t3242, &Py::str("h"))?, py::getitem(&t3242, &Py::str("D"))?, py::getitem(&t3242, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3251: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3229(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3251 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3252 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3280 = (t3252).items()?;
        if t3280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3280[0].clone();
        v_w = t3280[1].clone();
        if ({ let t3253 = v_k.clone(); Py::Bool({ let t3254 = v_outputs.clone(); if !py::contains(&t3254, &t3253)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3281(mut v_kw: Py) -> R {
    return Ok({ let t3294 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_muscle_max_power(py::getitem(&t3294, &Py::str("F0"))?, py::getitem(&t3294, &Py::str("a"))?, py::getitem(&t3294, &Py::str("b"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3301(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3303: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3281(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3303 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3304 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3332 = (t3304).items()?;
        if t3332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3332[0].clone();
        v_w = t3332[1].clone();
        if ({ let t3305 = v_k.clone(); Py::Bool({ let t3306 = v_outputs.clone(); if !py::contains(&t3306, &t3305)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3333(mut v_kw: Py) -> R {
    return Ok({ let t3355 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_nernst(py::getitem(&t3355, &Py::str("z"))?, py::getitem(&t3355, &Py::str("Cout"))?, py::getitem(&t3355, &Py::str("Cin"))?, py::method(&t3355, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3362(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3364: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3333(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3364 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3365 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3393 = (t3365).items()?;
        if t3393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3393[0].clone();
        v_w = t3393[1].clone();
        if ({ let t3366 = v_k.clone(); Py::Bool({ let t3367 = v_outputs.clone(); if !py::contains(&t3367, &t3366)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3394(mut v_kw: Py) -> R {
    return Ok({ let t3411 = f__from_si(Py::dict(vec![(Py::str("distance"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_proximal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_distal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_nerve_conduction_velocity(py::getitem(&t3411, &Py::str("distance"))?, py::getitem(&t3411, &Py::str("latency_proximal"))?, py::getitem(&t3411, &Py::str("latency_distal"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3418(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3420: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3394(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3420 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3421 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3449 = (t3421).items()?;
        if t3449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3449[0].clone();
        v_w = t3449[1].clone();
        if ({ let t3422 = v_k.clone(); Py::Bool({ let t3423 = v_outputs.clone(); if !py::contains(&t3423, &t3422)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3450(mut v_kw: Py) -> R {
    return Ok({ let t3466 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_osmotic_pressure(py::getitem(&t3466, &Py::str("osm"))?, py::method(&t3466, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_347(mut v_kw: Py) -> R {
    return Ok({ let t369 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_barometric_altitude(py::getitem(&t369, &Py::str("h"))?, py::method(&t369, "get", &[Py::str("FiO2"), Py::Float(0.2095)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3450(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3504 = (t3476).items()?;
        if t3504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3504[0].clone();
        v_w = t3504[1].clone();
        if ({ let t3477 = v_k.clone(); Py::Bool({ let t3478 = v_outputs.clone(); if !py::contains(&t3478, &t3477)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3505(mut v_kw: Py) -> R {
    return Ok({ let t3533 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_consumption(py::getitem(&t3533, &Py::str("CO"))?, py::getitem(&t3533, &Py::str("Hb"))?, py::getitem(&t3533, &Py::str("SaO2"))?, py::getitem(&t3533, &Py::str("SvO2"))?, py::getitem(&t3533, &Py::str("PaO2"))?, py::getitem(&t3533, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3540(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3505(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3543 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3571 = (t3543).items()?;
        if t3571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3571[0].clone();
        v_w = t3571[1].clone();
        if ({ let t3544 = v_k.clone(); Py::Bool({ let t3545 = v_outputs.clone(); if !py::contains(&t3545, &t3544)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3572(mut v_kw: Py) -> R {
    return Ok({ let t3597 = f__from_si(Py::dict(vec![(Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_content(py::getitem(&t3597, &Py::str("Hb"))?, py::getitem(&t3597, &Py::str("SaO2"))?, py::getitem(&t3597, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3604(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3606: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3572(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3606 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3607 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3635 = (t3607).items()?;
        if t3635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3635[0].clone();
        v_w = t3635[1].clone();
        if ({ let t3608 = v_k.clone(); Py::Bool({ let t3609 = v_outputs.clone(); if !py::contains(&t3609, &t3608)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3636(mut v_kw: Py) -> R {
    return Ok({ let t3660 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_delivery(py::getitem(&t3660, &Py::str("CO"))?, py::getitem(&t3660, &Py::str("Hb"))?, py::getitem(&t3660, &Py::str("SaO2"))?, py::getitem(&t3660, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3667(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3669: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3636(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3669 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3670 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3698 = (t3670).items()?;
        if t3698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3698[0].clone();
        v_w = t3698[1].clone();
        if ({ let t3671 = v_k.clone(); Py::Bool({ let t3672 = v_outputs.clone(); if !py::contains(&t3672, &t3671)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3699(mut v_kw: Py) -> R {
    return Ok({ let t3722 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_pah_renal_flow(py::getitem(&t3722, &Py::str("U_PAH"))?, py::getitem(&t3722, &Py::str("V"))?, py::getitem(&t3722, &Py::str("P_PAH"))?, py::getitem(&t3722, &Py::str("Hct"))?, py::method(&t3722, "get", &[Py::str("E"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3729(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3731: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3699(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3731 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3732 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3760 = (t3732).items()?;
        if t3760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3760[0].clone();
        v_w = t3760[1].clone();
        if ({ let t3733 = v_k.clone(); Py::Bool({ let t3734 = v_outputs.clone(); if !py::contains(&t3734, &t3733)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_376(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t378: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_347(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t378 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t379 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t407 = (t379).items()?;
        if t407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t407[0].clone();
        v_w = t407[1].clone();
        if ({ let t380 = v_k.clone(); Py::Bool({ let t381 = v_outputs.clone(); if !py::contains(&t381, &t380)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3761(mut v_kw: Py) -> R {
    return Ok({ let t3777 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_pco2_from_ph(py::getitem(&t3777, &Py::str("pH"))?, py::getitem(&t3777, &Py::str("HCO3"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3761(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3787 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3815 = (t3787).items()?;
        if t3815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3815[0].clone();
        v_w = t3815[1].clone();
        if ({ let t3788 = v_k.clone(); Py::Bool({ let t3789 = v_outputs.clone(); if !py::contains(&t3789, &t3788)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_38(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t40: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t40 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t41 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t69 = (t41).items()?;
        if t69.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t69[0].clone();
        v_w = t69[1].clone();
        if ({ let t42 = v_k.clone(); Py::Bool({ let t43 = v_outputs.clone(); if !py::contains(&t43, &t42)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3816(mut v_kw: Py) -> R {
    return Ok({ let t3829 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_ph_henderson(py::getitem(&t3829, &Py::str("HCO3"))?, py::getitem(&t3829, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3838: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3816(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3838 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3839 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3867 = (t3839).items()?;
        if t3867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3867[0].clone();
        v_w = t3867[1].clone();
        if ({ let t3840 = v_k.clone(); Py::Bool({ let t3841 = v_outputs.clone(); if !py::contains(&t3841, &t3840)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3868(mut v_kw: Py) -> R {
    return Ok({ let t3885 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_plasma_osmolality(py::getitem(&t3885, &Py::str("Na"))?, py::getitem(&t3885, &Py::str("glucose"))?, py::getitem(&t3885, &Py::str("BUN"))?, py::method(&t3885, "get", &[Py::str("measured"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3894: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3868(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3894 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3895 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3923 = (t3895).items()?;
        if t3923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3923[0].clone();
        v_w = t3923[1].clone();
        if ({ let t3896 = v_k.clone(); Py::Bool({ let t3897 = v_outputs.clone(); if !py::contains(&t3897, &t3896)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3924(mut v_kw: Py) -> R {
    return Ok({ let t3937 = f__from_si(Py::dict(vec![(Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_flow(py::getitem(&t3937, &Py::str("dP"))?, py::getitem(&t3937, &Py::str("eta"))?, py::getitem(&t3937, &Py::str("L"))?, py::getitem(&t3937, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3944(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3946: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3924(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3946 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3947 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3975 = (t3947).items()?;
        if t3975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3975[0].clone();
        v_w = t3975[1].clone();
        if ({ let t3948 = v_k.clone(); Py::Bool({ let t3949 = v_outputs.clone(); if !py::contains(&t3949, &t3948)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3976(mut v_kw: Py) -> R {
    return Ok({ let t3989 = f__from_si(Py::dict(vec![(Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_resistance(py::getitem(&t3989, &Py::str("eta"))?, py::getitem(&t3989, &Py::str("L"))?, py::getitem(&t3989, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3996(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3998: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3976(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3998 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3999 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4027 = (t3999).items()?;
        if t4027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4027[0].clone();
        v_w = t4027[1].clone();
        if ({ let t4000 = v_k.clone(); Py::Bool({ let t4001 = v_outputs.clone(); if !py::contains(&t4001, &t4000)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4028(mut v_kw: Py) -> R {
    return Ok({ let t4045 = f__from_si(Py::dict(vec![(Py::str("mPAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PCWP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_pulmonary_vascular_resistance(py::getitem(&t4045, &Py::str("mPAP"))?, py::getitem(&t4045, &Py::str("PCWP"))?, py::getitem(&t4045, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4052(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4028(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4054 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4055 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4083 = (t4055).items()?;
        if t4083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4083[0].clone();
        v_w = t4083[1].clone();
        if ({ let t4056 = v_k.clone(); Py::Bool({ let t4057 = v_outputs.clone(); if !py::contains(&t4057, &t4056)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_408(mut v_kw: Py) -> R {
    return Ok({ let t425 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v2"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bernoulli_drop(py::getitem(&t425, &Py::str("v1"))?, py::getitem(&t425, &Py::str("v2"))?, py::getitem(&t425, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4108(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4110: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4109 = v_inputs.clone(); f_qp_qs(py::getitem(&t4109, &Py::str("SaO2"))?, py::getitem(&t4109, &Py::str("SvO2"))?, py::getitem(&t4109, &Py::str("SpvO2"))?, py::getitem(&t4109, &Py::str("SpaO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4110 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4111 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4139 = (t4111).items()?;
        if t4139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4139[0].clone();
        v_w = t4139[1].clone();
        if ({ let t4112 = v_k.clone(); Py::Bool({ let t4113 = v_outputs.clone(); if !py::contains(&t4113, &t4112)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4156 = v_inputs.clone(); f_qrs_axis(py::getitem(&t4156, &Py::str("lead_I"))?, py::getitem(&t4156, &Py::str("aVF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4158 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4186 = (t4158).items()?;
        if t4186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4186[0].clone();
        v_w = t4186[1].clone();
        if ({ let t4159 = v_k.clone(); Py::Bool({ let t4160 = v_outputs.clone(); if !py::contains(&t4160, &t4159)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4187(mut v_kw: Py) -> R {
    return Ok({ let t4200 = f__from_si(Py::dict(vec![(Py::str("QT"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_qt_correction(py::getitem(&t4200, &Py::str("QT"))?, py::getitem(&t4200, &Py::str("HR"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4209: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4187(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4209 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4210 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4238 = (t4210).items()?;
        if t4238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4238[0].clone();
        v_w = t4238[1].clone();
        if ({ let t4211 = v_k.clone(); Py::Bool({ let t4212 = v_outputs.clone(); if !py::contains(&t4212, &t4211)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4251(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4253: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4252 = v_inputs.clone(); f_radius_flow_ratio(py::getitem(&t4252, &Py::str("r1"))?, py::getitem(&t4252, &Py::str("r2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4253 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4254 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4282 = (t4254).items()?;
        if t4282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4282[0].clone();
        v_w = t4282[1].clone();
        if ({ let t4255 = v_k.clone(); Py::Bool({ let t4256 = v_outputs.clone(); if !py::contains(&t4256, &t4255)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4304(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4306: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4305 = v_inputs.clone(); f_red_cell_indices(py::getitem(&t4305, &Py::str("Hct"))?, py::getitem(&t4305, &Py::str("Hb"))?, py::getitem(&t4305, &Py::str("RBC"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4306 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4307 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4335 = (t4307).items()?;
        if t4335.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4335[0].clone();
        v_w = t4335[1].clone();
        if ({ let t4308 = v_k.clone(); Py::Bool({ let t4309 = v_outputs.clone(); if !py::contains(&t4309, &t4308)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_408(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t463 = (t435).items()?;
        if t463.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t463[0].clone();
        v_w = t463[1].clone();
        if ({ let t436 = v_k.clone(); Py::Bool({ let t437 = v_outputs.clone(); if !py::contains(&t437, &t436)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4336(mut v_kw: Py) -> R {
    return Ok({ let t4351 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_renal_clearance(py::getitem(&t4351, &Py::str("U"))?, py::getitem(&t4351, &Py::str("V"))?, py::getitem(&t4351, &Py::str("P"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4358(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4360: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4336(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4360 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4361 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4389 = (t4361).items()?;
        if t4389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4389[0].clone();
        v_w = t4389[1].clone();
        if ({ let t4362 = v_k.clone(); Py::Bool({ let t4363 = v_outputs.clone(); if !py::contains(&t4363, &t4362)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4390(mut v_kw: Py) -> R {
    return Ok({ let t4408 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("flow"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_mechanics(py::getitem(&t4408, &Py::str("VT"))?, py::getitem(&t4408, &Py::str("Ppeak"))?, py::getitem(&t4408, &Py::str("Pplat"))?, py::getitem(&t4408, &Py::str("PEEP"))?, py::getitem(&t4408, &Py::str("flow"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4417: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4390(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4417 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4418 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4446 = (t4418).items()?;
        if t4446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4446[0].clone();
        v_w = t4446[1].clone();
        if ({ let t4419 = v_k.clone(); Py::Bool({ let t4420 = v_outputs.clone(); if !py::contains(&t4420, &t4419)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4447(mut v_kw: Py) -> R {
    return Ok({ let t4464 = f__from_si(Py::dict(vec![(Py::str("t"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_time_constant(py::getitem(&t4464, &Py::str("R"))?, py::getitem(&t4464, &Py::str("C"))?, py::method(&t4464, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4471(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4473: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4447(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4473 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4474 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4502 = (t4474).items()?;
        if t4502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4502[0].clone();
        v_w = t4502[1].clone();
        if ({ let t4475 = v_k.clone(); Py::Bool({ let t4476 = v_outputs.clone(); if !py::contains(&t4476, &t4475)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4523(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4525: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4524 = v_inputs.clone(); f_severinghaus_po2(py::getitem(&t4524, &Py::str("SO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4525 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4526 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4554 = (t4526).items()?;
        if t4554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4554[0].clone();
        v_w = t4554[1].clone();
        if ({ let t4527 = v_k.clone(); Py::Bool({ let t4528 = v_outputs.clone(); if !py::contains(&t4528, &t4527)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4555(mut v_kw: Py) -> R {
    return Ok({ let t4567 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_severinghaus_saturation(py::getitem(&t4567, &Py::str("PO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4574(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4576: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4555(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4576 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4577 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4605 = (t4577).items()?;
        if t4605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4605[0].clone();
        v_w = t4605[1].clone();
        if ({ let t4578 = v_k.clone(); Py::Bool({ let t4579 = v_outputs.clone(); if !py::contains(&t4579, &t4578)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4623(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4625: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4624 = v_inputs.clone(); f_shunt_fraction(py::getitem(&t4624, &Py::str("CcO2"))?, py::getitem(&t4624, &Py::str("CaO2"))?, py::getitem(&t4624, &Py::str("CvO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4625 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4626 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4654 = (t4626).items()?;
        if t4654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4654[0].clone();
        v_w = t4654[1].clone();
        if ({ let t4627 = v_k.clone(); Py::Bool({ let t4628 = v_outputs.clone(); if !py::contains(&t4628, &t4627)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_464(mut v_kw: Py) -> R {
    return Ok({ let t480 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_bicarbonate_from_ph(py::getitem(&t480, &Py::str("pH"))?, py::getitem(&t480, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4655(mut v_kw: Py) -> R {
    return Ok({ let t4678 = f__from_si(Py::dict(vec![(Py::str("conc"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("molar_mass"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_solution_osmolarity(py::getitem(&t4678, &Py::str("conc"))?, py::getitem(&t4678, &Py::str("molar_mass"))?, py::getitem(&t4678, &Py::str("i"))?, py::method(&t4678, "get", &[Py::str("phi"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4685(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4687: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4655(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4687 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4688 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4716 = (t4688).items()?;
        if t4716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4716[0].clone();
        v_w = t4716[1].clone();
        if ({ let t4689 = v_k.clone(); Py::Bool({ let t4690 = v_outputs.clone(); if !py::contains(&t4690, &t4689)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4717(mut v_kw: Py) -> R {
    return Ok({ let t4741 = f__from_si(Py::dict(vec![(Py::str("Pc"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("Pi"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pic"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pii"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_starling_filtration(py::getitem(&t4741, &Py::str("Pc"))?, py::getitem(&t4741, &Py::str("Pi"))?, py::getitem(&t4741, &Py::str("pic"))?, py::getitem(&t4741, &Py::str("pii"))?, py::getitem(&t4741, &Py::str("Kf"))?, py::method(&t4741, "get", &[Py::str("sigma"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4748(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4750: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4717(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4750 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4751 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4779 = (t4751).items()?;
        if t4779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4779[0].clone();
        v_w = t4779[1].clone();
        if ({ let t4752 = v_k.clone(); Py::Bool({ let t4753 = v_outputs.clone(); if !py::contains(&t4753, &t4752)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4780(mut v_kw: Py) -> R {
    return Ok({ let t4793 = f__from_si(Py::dict(vec![(Py::str("chronaxie"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_strength_duration(py::getitem(&t4793, &Py::str("rheobase"))?, py::getitem(&t4793, &Py::str("chronaxie"))?, py::getitem(&t4793, &Py::str("t"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4800(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4802: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4780(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4802 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4803 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4831 = (t4803).items()?;
        if t4831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4831[0].clone();
        v_w = t4831[1].clone();
        if ({ let t4804 = v_k.clone(); Py::Bool({ let t4805 = v_outputs.clone(); if !py::contains(&t4805, &t4804)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4832(mut v_kw: Py) -> R {
    return Ok({ let t4849 = f__from_si(Py::dict(vec![(Py::str("EDV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ESV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_volume_ef(py::getitem(&t4849, &Py::str("EDV"))?, py::getitem(&t4849, &Py::str("ESV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4856(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4858: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4832(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4858 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4859 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4887 = (t4859).items()?;
        if t4887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4887[0].clone();
        v_w = t4887[1].clone();
        if ({ let t4860 = v_k.clone(); Py::Bool({ let t4861 = v_outputs.clone(); if !py::contains(&t4861, &t4860)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_487(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t489: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_464(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t489 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t490 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t518 = (t490).items()?;
        if t518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t518[0].clone();
        v_w = t518[1].clone();
        if ({ let t491 = v_k.clone(); Py::Bool({ let t492 = v_outputs.clone(); if !py::contains(&t492, &t491)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4888(mut v_kw: Py) -> R {
    return Ok({ let t4903 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_work(py::getitem(&t4903, &Py::str("SV"))?, py::getitem(&t4903, &Py::str("MAP"))?, py::method(&t4903, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4910(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4912: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4888(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4912 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4913 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4941 = (t4913).items()?;
        if t4941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4941[0].clone();
        v_w = t4941[1].clone();
        if ({ let t4914 = v_k.clone(); Py::Bool({ let t4915 = v_outputs.clone(); if !py::contains(&t4915, &t4914)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4942(mut v_kw: Py) -> R {
    return Ok({ let t4959 = f__from_si(Py::dict(vec![(Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CVP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_vascular_resistance(py::getitem(&t4959, &Py::str("MAP"))?, py::getitem(&t4959, &Py::str("CVP"))?, py::getitem(&t4959, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4966(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4968: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4942(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4968 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4969 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4997 = (t4969).items()?;
        if t4997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4997[0].clone();
        v_w = t4997[1].clone();
        if ({ let t4970 = v_k.clone(); Py::Bool({ let t4971 = v_outputs.clone(); if !py::contains(&t4971, &t4970)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4998(mut v_kw: Py) -> R {
    return Ok({ let t5014 = f__from_si(Py::dict(vec![(Py::str("LVIDd"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("LVIDs"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_teichholz(py::getitem(&t5014, &Py::str("LVIDd"))?, py::getitem(&t5014, &Py::str("LVIDs"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5021(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5023: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4998(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5023 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5024 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5052 = (t5024).items()?;
        if t5052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5052[0].clone();
        v_w = t5052[1].clone();
        if ({ let t5025 = v_k.clone(); Py::Bool({ let t5026 = v_outputs.clone(); if !py::contains(&t5026, &t5025)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5053(mut v_kw: Py) -> R {
    return Ok({ let t5069 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_tubular_handling(py::getitem(&t5069, &Py::str("GFR"))?, py::getitem(&t5069, &Py::str("Px"))?, py::getitem(&t5069, &Py::str("Ux"))?, py::getitem(&t5069, &Py::str("V"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5076(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5078: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5053(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5078 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5079 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5107 = (t5079).items()?;
        if t5107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5107[0].clone();
        v_w = t5107[1].clone();
        if ({ let t5080 = v_k.clone(); Py::Bool({ let t5081 = v_outputs.clone(); if !py::contains(&t5081, &t5080)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5108(mut v_kw: Py) -> R {
    return Ok({ let t5121 = f__from_si(Py::dict(vec![(Py::str("dV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_compliance(py::getitem(&t5121, &Py::str("dV"))?, py::getitem(&t5121, &Py::str("dP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5108(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5131 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5159 = (t5131).items()?;
        if t5159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5159[0].clone();
        v_w = t5159[1].clone();
        if ({ let t5132 = v_k.clone(); Py::Bool({ let t5133 = v_outputs.clone(); if !py::contains(&t5133, &t5132)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5160(mut v_kw: Py) -> R {
    return Ok({ let t5173 = f__from_si(Py::dict(vec![(Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_vessel_reynolds(py::getitem(&t5173, &Py::str("rho"))?, py::getitem(&t5173, &Py::str("v"))?, py::getitem(&t5173, &Py::str("D"))?, py::getitem(&t5173, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5182: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5160(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5182 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5183 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5211 = (t5183).items()?;
        if t5211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5211[0].clone();
        v_w = t5211[1].clone();
        if ({ let t5184 = v_k.clone(); Py::Bool({ let t5185 = v_outputs.clone(); if !py::contains(&t5185, &t5184)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_519(mut v_kw: Py) -> R {
    return Ok({ let t532 = f__from_si(Py::dict(vec![(Py::str("Q"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)]))]), v_kw.clone())?; f_blood_velocity_area(py::getitem(&t532, &Py::str("Q"))?, py::getitem(&t532, &Py::str("A"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5234 = v_inputs.clone(); f_network_resistance(py::getitem(&t5234, &Py::str("R"))?, py::getitem(&t5234, &Py::str("arrangement"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5236 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5264 = (t5236).items()?;
        if t5264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5264[0].clone();
        v_w = t5264[1].clone();
        if ({ let t5237 = v_k.clone(); Py::Bool({ let t5238 = v_outputs.clone(); if !py::contains(&t5238, &t5237)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5265(mut v_kw: Py) -> R {
    return Ok({ let t5285 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("PCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_virtual_po2(py::getitem(&t5285, &Py::str("PO2"))?, py::getitem(&t5285, &Py::str("T"))?, py::getitem(&t5285, &Py::str("pH"))?, py::getitem(&t5285, &Py::str("PCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5292(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5294: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5265(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5294 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5295 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5323 = (t5295).items()?;
        if t5323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5323[0].clone();
        v_w = t5323[1].clone();
        if ({ let t5296 = v_k.clone(); Py::Bool({ let t5297 = v_outputs.clone(); if !py::contains(&t5297, &t5296)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5324(mut v_kw: Py) -> R {
    return Ok({ let t5343 = f__from_si(Py::dict(vec![(Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_watson_tbw(py::getitem(&t5343, &Py::str("age"))?, py::getitem(&t5343, &Py::str("height"))?, py::getitem(&t5343, &Py::str("weight"))?, py::getitem(&t5343, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5350(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5352: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5324(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5352 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5353 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5381 = (t5353).items()?;
        if t5381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5381[0].clone();
        v_w = t5381[1].clone();
        if ({ let t5354 = v_k.clone(); Py::Bool({ let t5355 = v_outputs.clone(); if !py::contains(&t5355, &t5354)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5382(mut v_kw: Py) -> R {
    return Ok({ let t5395 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_weir_rq(py::getitem(&t5395, &Py::str("VO2"))?, py::getitem(&t5395, &Py::str("VCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t541: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_519(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t541 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t542 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t570 = (t542).items()?;
        if t570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t570[0].clone();
        v_w = t570[1].clone();
        if ({ let t543 = v_k.clone(); Py::Bool({ let t544 = v_outputs.clone(); if !py::contains(&t544, &t543)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5402(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5404: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5382(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5404 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5405 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5433 = (t5405).items()?;
        if t5433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5433[0].clone();
        v_w = t5433[1].clone();
        if ({ let t5406 = v_k.clone(); Py::Bool({ let t5407 = v_outputs.clone(); if !py::contains(&t5407, &t5406)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5434(mut v_kw: Py) -> R {
    return Ok({ let t5447 = f__from_si(Py::dict(vec![(Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_womersley(py::getitem(&t5447, &Py::str("r"))?, py::getitem(&t5447, &Py::str("HR"))?, py::getitem(&t5447, &Py::str("rho"))?, py::getitem(&t5447, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5434(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5456 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5457 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5485 = (t5457).items()?;
        if t5485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5485[0].clone();
        v_w = t5485[1].clone();
        if ({ let t5458 = v_k.clone(); Py::Bool({ let t5459 = v_outputs.clone(); if !py::contains(&t5459, &t5458)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_571(mut v_kw: Py) -> R {
    return Ok({ let t584 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmi(py::getitem(&t584, &Py::str("weight"))?, py::getitem(&t584, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_591(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t593: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_571(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t593 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t594 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t622 = (t594).items()?;
        if t622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t622[0].clone();
        v_w = t622[1].clone();
        if ({ let t595 = v_k.clone(); Py::Bool({ let t596 = v_outputs.clone(); if !py::contains(&t596, &t595)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_623(mut v_kw: Py) -> R {
    return Ok({ let t646 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_harris_benedict(py::getitem(&t646, &Py::str("weight"))?, py::getitem(&t646, &Py::str("height"))?, py::getitem(&t646, &Py::str("age"))?, py::getitem(&t646, &Py::str("sex"))?, py::getitem(&t646, &Py::str("version"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_653(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t655: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_623(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t655 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t656 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t684 = (t656).items()?;
        if t684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t684[0].clone();
        v_w = t684[1].clone();
        if ({ let t657 = v_k.clone(); Py::Bool({ let t658 = v_outputs.clone(); if !py::contains(&t658, &t657)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_685(mut v_kw: Py) -> R {
    return Ok({ let t707 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_katch_mcardle(py::getitem(&t707, &Py::str("weight"))?, py::getitem(&t707, &Py::str("body_fat"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_70(mut v_kw: Py) -> R {
    return Ok({ let t98 = f__from_si(Py::dict(vec![(Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_gas(py::getitem(&t98, &Py::str("FiO2"))?, py::getitem(&t98, &Py::str("Patm"))?, py::getitem(&t98, &Py::str("PH2O"))?, py::getitem(&t98, &Py::str("PaCO2"))?, py::getitem(&t98, &Py::str("RQ"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_714(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t716: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_685(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t716 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t717 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t745 = (t717).items()?;
        if t745.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t745[0].clone();
        v_w = t745[1].clone();
        if ({ let t718 = v_k.clone(); Py::Bool({ let t719 = v_outputs.clone(); if !py::contains(&t719, &t718)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_746(mut v_kw: Py) -> R {
    return Ok({ let t765 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_mifflin(py::getitem(&t765, &Py::str("weight"))?, py::getitem(&t765, &Py::str("height"))?, py::getitem(&t765, &Py::str("age"))?, py::getitem(&t765, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_772(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t774: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_746(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t774 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t775 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t803 = (t775).items()?;
        if t803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t803[0].clone();
        v_w = t803[1].clone();
        if ({ let t776 = v_k.clone(); Py::Bool({ let t777 = v_outputs.clone(); if !py::contains(&t777, &t776)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_804(mut v_kw: Py) -> R {
    return Ok({ let t817 = f__from_si(Py::dict(vec![(Py::str("density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)]))]), v_kw.clone())?; f_body_fat_density(py::getitem(&t817, &Py::str("density"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_824(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t826: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_804(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t826 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t827 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t855 = (t827).items()?;
        if t855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t855[0].clone();
        v_w = t855[1].clone();
        if ({ let t828 = v_k.clone(); Py::Bool({ let t829 = v_outputs.clone(); if !py::contains(&t829, &t828)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_856(mut v_kw: Py) -> R {
    return Ok({ let t878 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_compartments(py::getitem(&t878, &Py::str("weight"))?, py::method(&t878, "get", &[Py::str("TBW_fraction"), Py::Float(0.6)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t887: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_856(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t887 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t916 = (t888).items()?;
        if t916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t916[0].clone();
        v_w = t916[1].clone();
        if ({ let t889 = v_k.clone(); Py::Bool({ let t890 = v_outputs.clone(); if !py::contains(&t890, &t889)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_917(mut v_kw: Py) -> R {
    return Ok({ let t930 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_body_surface_area(py::getitem(&t930, &Py::str("weight"))?, py::getitem(&t930, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_937(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t939: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_917(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t939 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t940 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t968 = (t940).items()?;
        if t968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t968[0].clone();
        v_w = t968[1].clone();
        if ({ let t941 = v_k.clone(); Py::Bool({ let t942 = v_outputs.clone(); if !py::contains(&t942, &t941)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_969(mut v_kw: Py) -> R {
    return Ok({ let t986 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PECO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_bohr_dead_space(py::getitem(&t986, &Py::str("PaCO2"))?, py::getitem(&t986, &Py::str("PECO2"))?, py::getitem(&t986, &Py::str("VT"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_993(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t995: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_969(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t995 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t996 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1024 = (t996).items()?;
        if t1024.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1024[0].clone();
        v_w = t1024[1].clone();
        if ({ let t997 = v_k.clone(); Py::Bool({ let t998 = v_outputs.clone(); if !py::contains(&t998, &t997)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
