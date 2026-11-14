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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_107(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_laplace_pressure" => {
            let r = s_139({ let mut d = vec![]; for k in ["gamma", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_159(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_ventilation_co2" => {
            let r = s_191({ let mut d = vec![]; for k in ["VCO2", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_anion_gap" => {
            let r = s_243({ let mut d = vec![]; for k in ["Na", "Cl", "HCO3", "albumin"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_atwater_energy" => {
            let r = s_295({ let mut d = vec![]; for k in ["carbohydrate", "fat", "protein", "alcohol"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_317(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_barometric_altitude" => {
            let r = s_349({ let mut d = vec![]; for k in ["h", "FiO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_378(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bernoulli_pressure_drop" => {
            let r = s_410({ let mut d = vec![]; for k in ["v1", "v2", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_434(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bicarbonate_from_ph" => {
            let r = s_466({ let mut d = vec![]; for k in ["pH", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_489(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_blood_velocity_area" => {
            let r = s_521({ let mut d = vec![]; for k in ["Q", "A"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_541(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmi" => {
            let r = s_573({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_harris_benedict" => {
            let r = s_625({ let mut d = vec![]; for k in ["weight", "height", "age", "sex", "version"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_655(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_katch_mcardle" => {
            let r = s_687({ let mut d = vec![]; for k in ["weight", "body_fat"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_716(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_mifflin_st_jeor" => {
            let r = s_748({ let mut d = vec![]; for k in ["weight", "height", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fat_from_density" => {
            let r = s_806({ let mut d = vec![]; for k in ["density"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_826(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fluid_compartments" => {
            let r = s_858({ let mut d = vec![]; for k in ["weight", "TBW_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_887(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_surface_area" => {
            let r = s_919({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_939(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bohr_dead_space" => {
            let r = s_971({ let mut d = vec![]; for k in ["PaCO2", "PECO2", "VT"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_995(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_index" => {
            let r = s_1027({ let mut d = vec![]; for k in ["CO", "BSA", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1049(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_output" => {
            let r = s_1081({ let mut d = vec![]; for k in ["HR", "SV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1101(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_chord_conductance" => {
            let r = s_1133({ let mut d = vec![]; for k in ["gK", "gNa", "gCl", "EK", "ENa", "ECl"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1156(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cockcroft_gault" => {
            let r = s_1188({ let mut d = vec![]; for k in ["age", "weight", "Scr", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1215(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_conduction_velocity_scaling" => {
            let r = s_1247({ let mut d = vec![]; for k in ["v1", "d1", "d2", "fiber"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1271(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_creatinine_clearance_timed" => {
            let r = s_1303({ let mut d = vec![]; for k in ["Ucr", "Vurine", "hours", "Pcr", "BSA"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ecg_heart_rate" => {
            let r = s_1355({ let mut d = vec![]; for k in ["RR", "small_boxes", "large_boxes", "speed"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021" => {
            let r = f_egfr_ckd_epi_2021(match v.get("Scr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sex") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1440(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021_cys" => {
            let r = s_1472({ let mut d = vec![]; for k in ["Scr", "Scys", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_elimination_half_life" => {
            let r = s_1533({ let mut d = vec![]; for k in ["CL", "Vd", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1557(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fev1_fvc_ratio" => {
            let r = s_1589({ let mut d = vec![]; for k in ["FEV1", "FVC"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1611(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_cardiac_output" => {
            let r = s_1643({ let mut d = vec![]; for k in ["VO2", "CaO2", "CvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1667(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_diffusion" => {
            let r = s_1699({ let mut d = vec![]; for k in ["D", "A", "dC", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_saturations" => {
            let r = s_1751({ let mut d = vec![]; for k in ["VO2", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1786(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_filtration_fraction" => {
            let r = s_1818({ let mut d = vec![]; for k in ["GFR", "RPF"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1840(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fluid_shift" => {
            let r = s_1872({ let mut d = vec![]; for k in ["TBW", "Posm", "V_added", "C_added", "ICF_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1904(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fractional_excretion" => {
            let r = f_fractional_excretion(match v.get("Ux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ucr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pcr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1950(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_free_water_clearance" => {
            let r = s_1982({ let mut d = vec![]; for k in ["V", "Uosm", "Posm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gas_volume_conversion" => {
            let r = s_2034({ let mut d = vec![]; for k in ["V", "T", "Patm", "PH2O"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ghk_voltage" => {
            let r = s_2097({ let mut d = vec![]; for k in ["PK", "PNa", "PCl", "K_o", "K_i", "Na_o", "Na_i", "Cl_o", "Cl_i", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2124(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gibbs_donnan" => {
            let r = s_2156({ let mut d = vec![]; for k in ["A", "c", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2179(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_glucose_titration" => {
            let r = s_2211({ let mut d = vec![]; for k in ["GFR", "Pglucose", "Tm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_heart_rate_reserve" => {
            let r = f_heart_rate_reserve(match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HR_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intensity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_fit" => {
            let r = s_2323({ let mut d = vec![]; for k in ["P1", "S1", "P2", "S2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_force_velocity" => {
            let r = s_2392({ let mut d = vec![]; for k in ["F0", "a", "b", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2416(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_po2" => {
            let r = s_2448({ let mut d = vec![]; for k in ["SO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2477(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_saturation" => {
            let r = s_2509({ let mut d = vec![]; for k in ["PO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2531(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_pressure" => {
            let r = s_2563({ let mut d = vec![]; for k in ["h", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_weighing" => {
            let r = s_2615({ let mut d = vec![]; for k in ["mass_air", "mass_water", "water_density", "RV", "GI"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2648(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ionic_current" => {
            let r = s_2680({ let mut d = vec![]; for k in ["g", "Vm", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2700(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_laplace_wall_tension" => {
            let r = s_2732({ let mut d = vec![]; for k in ["P", "r", "shape", "h"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2758(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lean_body_mass_boer" => {
            let r = s_2790({ let mut d = vec![]; for k in ["weight", "height", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_length_constant" => {
            let r = s_2849({ let mut d = vec![]; for k in ["Rm", "Ri", "d", "x"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2873(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lung_capacities" => {
            let r = s_2905({ let mut d = vec![]; for k in ["TV", "IRV", "ERV", "RV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2927(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_mean_arterial_pressure" => {
            let r = s_2959({ let mut d = vec![]; for k in ["SBP", "DBP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_charge" => {
            let r = s_3013({ let mut d = vec![]; for k in ["Cm", "dV", "area"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3035(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_time_constant" => {
            let r = s_3067({ let mut d = vec![]; for k in ["Rm", "Cm", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3091(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_met_energy" => {
            let r = s_3123({ let mut d = vec![]; for k in ["MET", "weight", "duration"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_minute_ventilation" => {
            let r = s_3175({ let mut d = vec![]; for k in ["VT", "RR", "VD"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_moens_korteweg" => {
            let r = s_3231({ let mut d = vec![]; for k in ["E", "h", "D", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3251(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_muscle_max_power" => {
            let r = s_3283({ let mut d = vec![]; for k in ["F0", "a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3303(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nernst_potential" => {
            let r = s_3335({ let mut d = vec![]; for k in ["z", "Cout", "Cin", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3364(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nerve_conduction_velocity" => {
            let r = s_3396({ let mut d = vec![]; for k in ["distance", "latency_proximal", "latency_distal"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3420(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_osmotic_pressure" => {
            let r = s_3452({ let mut d = vec![]; for k in ["osm", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_consumption" => {
            let r = s_3507({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3542(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_content" => {
            let r = s_3574({ let mut d = vec![]; for k in ["Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3606(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_delivery" => {
            let r = s_3638({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pah_renal_flow" => {
            let r = s_3701({ let mut d = vec![]; for k in ["U_PAH", "V", "P_PAH", "Hct", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3731(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pco2_from_ph" => {
            let r = s_3763({ let mut d = vec![]; for k in ["pH", "HCO3"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3786(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ph_henderson_hasselbalch" => {
            let r = s_3818({ let mut d = vec![]; for k in ["HCO3", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_plasma_osmolality" => {
            let r = s_3870({ let mut d = vec![]; for k in ["Na", "glucose", "BUN", "measured"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3894(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_flow" => {
            let r = s_3926({ let mut d = vec![]; for k in ["dP", "eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_resistance" => {
            let r = s_3978({ let mut d = vec![]; for k in ["eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pulmonary_vascular_resistance" => {
            let r = s_4030({ let mut d = vec![]; for k in ["mPAP", "PCWP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4054(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qp_qs" => {
            let r = f_qp_qs(match v.get("SaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpaO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qrs_axis" => {
            let r = f_qrs_axis(match v.get("lead_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("aVF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4157(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qt_correction" => {
            let r = s_4189({ let mut d = vec![]; for k in ["QT", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_radius_flow_ratio" => {
            let r = f_radius_flow_ratio(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_red_cell_indices" => {
            let r = f_red_cell_indices(match v.get("Hct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RBC") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_renal_clearance" => {
            let r = s_4338({ let mut d = vec![]; for k in ["U", "V", "P"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_mechanics" => {
            let r = s_4392({ let mut d = vec![]; for k in ["VT", "Ppeak", "Pplat", "PEEP", "flow"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4417(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_time_constant" => {
            let r = s_4449({ let mut d = vec![]; for k in ["R", "C", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_po2" => {
            let r = f_severinghaus_po2(match v.get("SO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4525(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_saturation" => {
            let r = s_4557({ let mut d = vec![]; for k in ["PO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4576(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_shunt_fraction" => {
            let r = f_shunt_fraction(match v.get("CcO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CvO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4625(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_solution_osmolarity" => {
            let r = s_4657({ let mut d = vec![]; for k in ["conc", "molar_mass", "i", "phi"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4687(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_starling_filtration" => {
            let r = s_4719({ let mut d = vec![]; for k in ["Pc", "Pi", "pic", "pii", "Kf", "sigma"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4750(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_strength_duration" => {
            let r = s_4782({ let mut d = vec![]; for k in ["rheobase", "chronaxie", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4802(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_volume_ef" => {
            let r = s_4834({ let mut d = vec![]; for k in ["EDV", "ESV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_work" => {
            let r = s_4890({ let mut d = vec![]; for k in ["SV", "MAP", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4912(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_systemic_vascular_resistance" => {
            let r = s_4944({ let mut d = vec![]; for k in ["MAP", "CVP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4968(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_teichholz_volumes" => {
            let r = s_5000({ let mut d = vec![]; for k in ["LVIDd", "LVIDs"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5023(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_tubular_handling" => {
            let r = s_5055({ let mut d = vec![]; for k in ["GFR", "Px", "Ux", "V"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5078(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vascular_compliance" => {
            let r = s_5110({ let mut d = vec![]; for k in ["dV", "dP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessel_reynolds" => {
            let r = s_5162({ let mut d = vec![]; for k in ["rho", "v", "D", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5182(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessels_series_parallel" => {
            let r = f_network_resistance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5235(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_virtual_po2" => {
            let r = s_5267({ let mut d = vec![]; for k in ["PO2", "T", "pH", "PCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5294(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_watson_total_body_water" => {
            let r = s_5326({ let mut d = vec![]; for k in ["age", "height", "weight", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5352(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_weir_energy_expenditure" => {
            let r = s_5384({ let mut d = vec![]; for k in ["VO2", "VCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5404(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_womersley_number" => {
            let r = s_5436({ let mut d = vec![]; for k in ["r", "HR", "rho", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5456(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t5464 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t5464.truthy() { t5464 } else { { let t5463 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5463.truthy() { t5463 } else { { let t5462 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5462.truthy() { t5462 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t5465 = v_a.clone(); Py::Bool({ let t5466 = v_b.clone(); if py::eq(&t5465, &t5466) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t5467 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5467.truthy() { t5467 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5471 = py::not(&{ let t5468 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5468.truthy() { t5468 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5471.truthy() { t5471 } else { { let t5469 = py::len(&v_a.clone())?; Py::Bool({ let t5470 = py::len(&v_b.clone())?; if py::ne(&t5469, &t5470) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        v_w = Py::Float(0.0);
        for t5472 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t5473 = (t5472).items()?;
            if t5473.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t5473[0].clone();
            v_y = t5473[1].clone();
            v_w = py::minmax(&[v_w.clone(), f__diff(v_x.clone(), v_y.clone())?], true)?;
        }
        return Ok(v_w.clone());
    }
    if ({ let t5478 = { let t5474 = v_a.clone(); Py::Bool({ let t5475 = Py::None; if py::is(&t5474, &t5475) { true } else { false } }) }; if t5478.truthy() { t5478 } else { { let t5476 = v_b.clone(); Py::Bool({ let t5477 = Py::None; if py::is(&t5476, &t5477) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5479 = v_a.clone(); Py::Bool({ let t5480 = v_b.clone(); if py::is(&t5479, &t5480) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t5482: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5481 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5481[0].clone();
            v_b = t5481[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5482 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t5483 = v_a.clone(); Py::Bool({ let t5484 = v_b.clone(); if py::eq(&t5483, &t5484) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5485 = v_d.clone(); Py::Bool({ let t5486 = v_d.clone(); if py::ne(&t5485, &t5486) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py, mut v_name: Py, mut v_open_lo: Py, mut v_open_hi: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t4735 = (if (v_open_lo.clone()).truthy() { { let t4727 = v_v.clone(); Py::Bool({ let t4728 = Py::Int(0i64); if py::gt(&t4727, &t4728)? { true } else { false } }) } } else { { let t4729 = v_v.clone(); Py::Bool({ let t4730 = Py::Int(0i64); if py::ge(&t4729, &t4730)? { true } else { false } }) } }); if !t4735.truthy() { t4735 } else { (if (v_open_hi.clone()).truthy() { { let t4731 = v_v.clone(); Py::Bool({ let t4732 = Py::Int(1i64); if py::lt(&t4731, &t4732)? { true } else { false } }) } } else { { let t4733 = v_v.clone(); Py::Bool({ let t4734 = Py::Int(1i64); if py::le(&t4733, &t4734)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
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
    for t5450 in py::iter(&(py::method(&v_conv.clone(), "items", &[])?))? {
        let t5454 = (t5450).items()?;
        if t5454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5454[0].clone();
        let t5455 = (t5454[1].clone()).items()?;
        if t5455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t5455[0].clone();
        v_off = t5455[1].clone();
        v_v = py::method(&v_out.clone(), "get", &[v_k.clone()])?;
        if ({ let t5453 = { let t5451 = v_v.clone(); Py::Bool({ let t5452 = Py::None; if !py::is(&t5451, &t5452) { true } else { false } }) }; if !t5453.truthy() { t5453 } else { py::not(&Py::Bool(matches!(v_v.clone(), Py::Str(_) | Py::Bool(_)))) } }).truthy() {
            py::setitem(&v_out.clone(), &v_k.clone(), py::truediv(&py::sub(&v_v.clone(), &v_off.clone())?, &v_f.clone())?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int_ok(mut v_v: Py, mut v_name: Py) -> R {
    v_v = (if ({ let t4665 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4665.truthy() { t4665 } else { { let t4664 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4664.truthy() { t4664 } else { { let t4662 = py::float(&v_v.clone())?; Py::Bool({ let t4663 = py::int(&v_v.clone())?; if py::eq(&t4662, &t4663) { true } else { false } }) } } } } }).truthy() { py::int(&v_v.clone())? } else { Py::None });
    let _ = f__need({ let t4671 = { let t4666 = v_v.clone(); Py::Bool({ let t4667 = Py::None; if !py::is(&t4666, &t4667) { true } else { false } }) }; if !t4671.truthy() { t4671 } else { { let t4668 = Py::Int(1i64); Py::Bool({ let t4669 = v_v.clone(); if py::le(&t4668, &t4669)? { { let t4670 = Py::Int(6i64); if py::le(&t4669, &t4670)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of particles from 1 to 6".to_string()].concat()))?;
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
    let _ = f__need({ let t5060 = v_v.clone(); Py::Bool({ let t5061 = Py::Int(0i64); if py::ge(&t5060, &t5061)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t5438 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5438.truthy() { t5438 } else { { let t5437 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5437.truthy() { t5437 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
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
    let _ = f__need({ let t5439 = v_v.clone(); Py::Bool({ let t5440 = Py::Int(0i64); if py::gt(&t5439, &t5440)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
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
    if ({ let t5445 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5445.truthy() { t5445 } else { { let t5444 = Py::Bool(matches!(v_v.clone(), Py::Str(_))); if t5444.truthy() { t5444 } else { { let t5442 = v_v.clone(); Py::Bool({ let t5443 = Py::None; if py::is(&t5442, &t5443) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        return Ok(f__r12(v_v.clone())?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5446 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5446;
out.push(f__rnd(v_x.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5447 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t5448 = (t5447).items()?;
if t5448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5448[0].clone();
v_x = t5448[1].clone();
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
    let _ = f__need({ let t5332 = v_v.clone(); Py::Bool({ let t5333 = Py::tuple(vec![Py::str("female"), Py::str("male")]); if py::contains(&t5333, &t5332)? { true } else { false } }) }, Py::str("sex must be female or male"))?;
    return Ok({ let t5334 = v_v.clone(); Py::Bool({ let t5335 = Py::str("female"); if py::eq(&t5334, &t5335) { true } else { false } }) });
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
    v_out = { let mut out = vec![]; for t5214 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5214;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) };
    let _ = f__need({ let t5217 = v_lo.clone(); Py::Bool({ let t5218 = py::len(&v_out.clone())?; if py::le(&t5217, &t5218)? { { let t5219 = v_hi.clone(); if py::le(&t5218, &t5219)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " needs between ".to_string(), py::format(&v_lo.clone(), "")?, " and ".to_string(), py::format(&v_hi.clone(), "")?, " entries".to_string()].concat()))?;
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
    let _ = f__need({ let t91 = v_simp.clone(); Py::Bool({ let t92 = Py::Int(0i64); if py::gt(&t91, &t92)? { true } else { false } }) }, Py::str("the computed alveolar PO2 is not positive: the PaCO2 is too high for this inspired O2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PiO2"), v_pio2.clone()), (Py::str("PAO2"), v_simp.clone()), (Py::str("PAO2_full"), py::add(&v_simp.clone(), &py::truediv(&py::mul(&py::mul(&v_PaCO2.clone(), &v_FiO2.clone())?, &py::sub(&Py::Float(1.0), &v_RQ.clone())?)?, &v_RQ.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_laplace(mut v_gamma: Py, mut v_r: Py) -> R {
    let mut v_p: Py = Py::None;
    let t144 = (Py::tuple(vec![f__pos(v_gamma.clone(), Py::str("gamma"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gamma = t144[0].clone();
    v_r = t144[1].clone();
    v_p = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_gamma.clone())?, &Py::Float(0.001))?, &py::mul(&v_r.clone(), &Py::Float(1e-06))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("P"), v_p.clone()), (Py::str("P_cmH2O"), py::truediv(&v_p.clone(), &c_CMH2O()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_ventilation_co2(mut v_VCO2: Py, mut v_PaCO2: Py) -> R {
    let t196 = (Py::tuple(vec![f__pos(v_VCO2.clone(), Py::str("VCO2"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VCO2 = t196[0].clone();
    v_PaCO2 = t196[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("VA"), py::truediv(&py::truediv(&py::mul(&v_VCO2.clone(), &c_K_ALV()?)?, &v_PaCO2.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_anion_gap(mut v_Na: Py, mut v_Cl: Py, mut v_HCO3: Py, mut v_albumin: Py) -> R {
    let mut v_ag: Py = Py::None;
    let t248 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__pos(v_Cl.clone(), Py::str("Cl"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_albumin.clone(), Py::str("albumin"))?])).items()?;
    if t248.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t248[0].clone();
    v_Cl = t248[1].clone();
    v_HCO3 = t248[2].clone();
    v_albumin = t248[3].clone();
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
    let t300 = (Py::tuple(vec![f__nonneg(v_carbohydrate.clone(), Py::str("carbohydrate"))?, f__nonneg(v_fat.clone(), Py::str("fat"))?, f__nonneg(v_protein.clone(), Py::str("protein"))?, f__nonneg(v_alcohol.clone(), Py::str("alcohol"))?])).items()?;
    if t300.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c = t300[0].clone();
    v_f = t300[1].clone();
    v_p = t300[2].clone();
    v_a = t300[3].clone();
    v_k = py::add(&py::add(&py::add(&py::mul(&Py::Float(4.0), &v_c.clone())?, &py::mul(&Py::Float(9.0), &v_f.clone())?)?, &py::mul(&Py::Float(4.0), &v_p.clone())?)?, &py::mul(&Py::Float(7.0), &v_a.clone())?)?;
    let _ = f__need({ let t301 = v_k.clone(); Py::Bool({ let t302 = Py::Int(0i64); if py::gt(&t301, &t302)? { true } else { false } }) }, Py::str("at least one amount must be positive"))?;
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
    let _ = f__need({ let t361 = Py::Int(0i64); Py::Bool({ let t362 = v_h.clone(); if py::le(&t361, &t362)? { { let t363 = Py::Int(11000i64); if py::le(&t362, &t363)? { true } else { false } } } else { false } }) }, Py::str("altitude must be between 0 and 11000 m (troposphere)"))?;
    v_p = py::mul(&Py::Float(101325.0), &py::pow(&py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(0.0065), &v_h.clone())?, &Py::Float(288.15))?)?, &c__ISA_EXP()?)?)?;
    v_pm = py::truediv(&v_p.clone(), &c_MMHG()?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Patm"), v_pm.clone()), (Py::str("PiO2"), py::mul(&v_FiO2.clone(), &py::sub(&v_pm.clone(), &Py::Float(47.0))?)?), (Py::str("Patm_kPa"), py::truediv(&v_p.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli_drop(mut v_v1: Py, mut v_v2: Py, mut v_rho: Py) -> R {
    let mut v_dp: Py = Py::None;
    let t417 = (Py::tuple(vec![f__nonneg(v_v1.clone(), Py::str("v1"))?, f__pos(v_v2.clone(), Py::str("v2"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t417.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t417[0].clone();
    v_v2 = t417[1].clone();
    v_rho = t417[2].clone();
    let _ = f__need({ let t418 = v_v2.clone(); Py::Bool({ let t419 = v_v1.clone(); if py::gt(&t418, &t419)? { true } else { false } }) }, Py::str("the downstream (jet) velocity v2 must exceed v1"))?;
    v_dp = py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dP"), v_dp.clone()), (Py::str("dP_mmHg"), py::truediv(&v_dp.clone(), &c_MMHG()?)?), (Py::str("dP_simplified"), py::mul(&py::mul(&Py::Float(4.0), &v_v2.clone())?, &v_v2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bicarbonate_from_ph(mut v_pH: Py, mut v_PaCO2: Py) -> R {
    let t471 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t471[0].clone();
    v_PaCO2 = t471[1].clone();
    let _ = f__need({ let t472 = Py::Float(6.5); Py::Bool({ let t473 = v_pH.clone(); if py::le(&t472, &t473)? { { let t474 = Py::Float(8.0); if py::le(&t473, &t474)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HCO3"), py::mul(&py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_blood_velocity_area(mut v_Q: Py, mut v_A: Py) -> R {
    let t526 = (Py::tuple(vec![f__pos(v_Q.clone(), Py::str("Q"))?, f__pos(v_A.clone(), Py::str("A"))?])).items()?;
    if t526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t526[0].clone();
    v_A = t526[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("v"), py::truediv(&py::truediv(&py::mul(&v_Q.clone(), &Py::Float(1000.0))?, &Py::Float(60.0))?, &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmi(mut v_weight: Py, mut v_height: Py) -> R {
    let t578 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t578.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t578[0].clone();
    v_height = t578[1].clone();
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
    let t630 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t630.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t630[0].clone();
    v_H = t630[1].clone();
    v_A = t630[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t635 = v_version.clone(); Py::Bool({ let t636 = Py::tuple(vec![Py::str("original"), Py::str("revised")]); if py::contains(&t636, &t635)? { true } else { false } }) }, Py::str("version must be original (1919) or revised (1984)"))?;
    let _ = f__need({ let t637 = v_A.clone(); Py::Bool({ let t638 = Py::Int(18i64); if py::ge(&t637, &t638)? { true } else { false } }) }, Py::str("the equations are for adults"))?;
    if ({ let t639 = v_version.clone(); Py::Bool({ let t640 = Py::str("original"); if py::eq(&t639, &t640) { true } else { false } }) }).truthy() {
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
    let t701 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_body_fat.clone(), Py::str("body_fat"), Py::Bool(false), Py::Bool(true))?])).items()?;
    if t701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t701[0].clone();
    v_f = t701[1].clone();
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
    let t753 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t753.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t753[0].clone();
    v_H = t753[1].clone();
    v_A = t753[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t758 = v_A.clone(); Py::Bool({ let t759 = Py::Int(18i64); if py::ge(&t758, &t759)? { true } else { false } }) }, Py::str("the equation is for adults"))?;
    v_v = py::add(&py::sub(&py::add(&py::mul(&Py::Float(10.0), &v_W.clone())?, &py::mul(&Py::Float(6.25), &v_H.clone())?)?, &py::mul(&Py::Float(5.0), &v_A.clone())?)?, &(if (v_fem.clone()).truthy() { py::neg(&Py::Float(161.0))? } else { Py::Float(5.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("REE"), v_v.clone()), (Py::str("REE_kJ"), py::mul(&v_v.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_fat_density(mut v_density: Py) -> R {
    let mut v_D: Py = Py::None;
    v_D = f__num(v_density.clone(), Py::str("density"))?;
    let _ = f__need({ let t809 = Py::Float(0.9); Py::Bool({ let t810 = v_D.clone(); if py::lt(&t809, &t810)? { { let t811 = Py::Float(1.1); if py::le(&t810, &t811)? { true } else { false } } } else { false } }) }, Py::str("body density must be above 0.9 and at most 1.1 g/mL (the two-compartment equations give 0-100 % fat there)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?), (Py::str("fat_Brozek"), py::sub(&py::truediv(&Py::Float(457.0), &v_D.clone())?, &Py::Float(414.2))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_surface_area(mut v_weight: Py, mut v_height: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let t924 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t924[0].clone();
    v_H = t924[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("BSA_DuBois"), py::mul(&py::mul(&Py::Float(0.007184), &py::pow(&v_W.clone(), &Py::Float(0.425))?)?, &py::pow(&v_H.clone(), &Py::Float(0.725))?)?), (Py::str("BSA_Mosteller"), py::math1("sqrt", &py::truediv(&py::mul(&v_H.clone(), &v_W.clone())?, &Py::Float(3600.0))?)?), (Py::str("BSA_Haycock"), py::mul(&py::mul(&Py::Float(0.024265), &py::pow(&v_W.clone(), &Py::Float(0.5378))?)?, &py::pow(&v_H.clone(), &Py::Float(0.3964))?)?), (Py::str("BSA_GehanGeorge"), py::mul(&py::mul(&Py::Float(0.0235), &py::pow(&v_W.clone(), &Py::Float(0.51456))?)?, &py::pow(&v_H.clone(), &Py::Float(0.42246))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bohr_dead_space(mut v_PaCO2: Py, mut v_PECO2: Py, mut v_VT: Py) -> R {
    let mut v_f: Py = Py::None;
    let t978 = (Py::tuple(vec![f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?, f__nonneg(v_PECO2.clone(), Py::str("PECO2"))?, f__pos(v_VT.clone(), Py::str("VT"))?])).items()?;
    if t978.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PaCO2 = t978[0].clone();
    v_PECO2 = t978[1].clone();
    v_VT = t978[2].clone();
    let _ = f__need({ let t979 = v_PECO2.clone(); Py::Bool({ let t980 = v_PaCO2.clone(); if py::lt(&t979, &t980)? { true } else { false } }) }, Py::str("the mixed expired PCO2 must be below the arterial PCO2"))?;
    v_f = py::truediv(&py::sub(&v_PaCO2.clone(), &v_PECO2.clone())?, &v_PaCO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VD_VT"), v_f.clone()), (Py::str("VD"), py::mul(&v_f.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cardiac_index(mut v_CO: Py, mut v_BSA: Py, mut v_HR: Py) -> R {
    let mut v_ci: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1032 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t1032[0].clone();
    v_BSA = t1032[1].clone();
    v_ci = py::truediv(&v_CO.clone(), &v_BSA.clone())?;
    v_out = Py::dict(vec![(Py::str("CI"), v_ci.clone())]);
    if ({ let t1033 = v_HR.clone(); Py::Bool({ let t1034 = Py::None; if !py::is(&t1033, &t1034) { true } else { false } }) }).truthy() {
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
    let t1086 = (Py::tuple(vec![f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_SV.clone(), Py::str("SV"))?])).items()?;
    if t1086.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HR = t1086[0].clone();
    v_SV = t1086[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&py::mul(&v_HR.clone(), &v_SV.clone())?, &Py::Float(1000.0))?), (Py::str("CO_mL"), py::mul(&v_HR.clone(), &v_SV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chord_conductance(mut v_gK: Py, mut v_gNa: Py, mut v_gCl: Py, mut v_EK: Py, mut v_ENa: Py, mut v_ECl: Py) -> R {
    let mut v_g: Py = Py::None;
    let t1138 = (Py::tuple(vec![f__nonneg(v_gK.clone(), Py::str("gK"))?, f__nonneg(v_gNa.clone(), Py::str("gNa"))?, f__nonneg(v_gCl.clone(), Py::str("gCl"))?])).items()?;
    if t1138.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_gK = t1138[0].clone();
    v_gNa = t1138[1].clone();
    v_gCl = t1138[2].clone();
    let t1139 = (Py::tuple(vec![f__num(v_EK.clone(), Py::str("EK"))?, f__num(v_ENa.clone(), Py::str("ENa"))?, f__num(v_ECl.clone(), Py::str("ECl"))?])).items()?;
    if t1139.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_EK = t1139[0].clone();
    v_ENa = t1139[1].clone();
    v_ECl = t1139[2].clone();
    v_g = py::add(&py::add(&v_gK.clone(), &v_gNa.clone())?, &v_gCl.clone())?;
    let _ = f__need({ let t1140 = v_g.clone(); Py::Bool({ let t1141 = Py::Int(0i64); if py::gt(&t1140, &t1141)? { true } else { false } }) }, Py::str("at least one conductance must be positive"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Vm"), py::truediv(&py::add(&py::add(&py::mul(&v_gK.clone(), &v_EK.clone())?, &py::mul(&v_gNa.clone(), &v_ENa.clone())?)?, &py::mul(&v_gCl.clone(), &v_ECl.clone())?)?, &v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cockcroft_gault(mut v_age: Py, mut v_weight: Py, mut v_Scr: Py, mut v_sex: Py) -> R {
    let mut v_fem: Py = Py::None;
    let t1193 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_Scr.clone(), Py::str("Scr"))?])).items()?;
    if t1193.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t1193[0].clone();
    v_weight = t1193[1].clone();
    v_Scr = t1193[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1198 = Py::Int(18i64); Py::Bool({ let t1199 = v_age.clone(); if py::le(&t1198, &t1199)? { { let t1200 = Py::Int(140i64); if py::lt(&t1199, &t1200)? { true } else { false } } } else { false } }) }, Py::str("age must be at least 18 and below 140"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CrCl"), py::mul(&py::truediv(&py::mul(&py::sub(&Py::Float(140.0), &v_age.clone())?, &v_weight.clone())?, &py::mul(&Py::Float(72.0), &v_Scr.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.85) } else { Py::Float(1.0) }))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compliance(mut v_dV: Py, mut v_dP: Py) -> R {
    let t5115 = (Py::tuple(vec![f__pos(v_dV.clone(), Py::str("dV"))?, f__pos(v_dP.clone(), Py::str("dP"))?])).items()?;
    if t5115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dV = t5115[0].clone();
    v_dP = t5115[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&v_dV.clone(), &v_dP.clone())?), (Py::str("E"), py::truediv(&v_dP.clone(), &v_dV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conduction_velocity_scaling(mut v_v1: Py, mut v_d1: Py, mut v_d2: Py, mut v_fiber: Py) -> R {
    let mut v_k: Py = Py::None;
    let t1252 = (Py::tuple(vec![f__pos(v_v1.clone(), Py::str("v1"))?, f__pos(v_d1.clone(), Py::str("d1"))?, f__pos(v_d2.clone(), Py::str("d2"))?])).items()?;
    if t1252.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1252[0].clone();
    v_d1 = t1252[1].clone();
    v_d2 = t1252[2].clone();
    let _ = f__need({ let t1253 = v_fiber.clone(); Py::Bool({ let t1254 = Py::tuple(vec![Py::str("myelinated"), Py::str("unmyelinated")]); if py::contains(&t1254, &t1253)? { true } else { false } }) }, Py::str("fiber must be myelinated or unmyelinated"))?;
    v_k = (if ({ let t1255 = v_fiber.clone(); Py::Bool({ let t1256 = Py::str("myelinated"); if py::eq(&t1255, &t1256) { true } else { false } }) }).truthy() { py::truediv(&v_d2.clone(), &v_d1.clone())? } else { py::math1("sqrt", &py::truediv(&v_d2.clone(), &v_d1.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("v2"), py::mul(&v_v1.clone(), &v_k.clone())?), (Py::str("ratio"), v_k.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_creatinine_clearance_timed(mut v_Ucr: Py, mut v_Vurine: Py, mut v_hours: Py, mut v_Pcr: Py, mut v_BSA: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1308 = (Py::tuple(vec![f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Vurine.clone(), Py::str("Vurine"))?, f__pos(v_hours.clone(), Py::str("hours"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1308.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ucr = t1308[0].clone();
    v_Vurine = t1308[1].clone();
    v_hours = t1308[2].clone();
    v_Pcr = t1308[3].clone();
    v_BSA = t1308[4].clone();
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
    for t1356 in py::iter(&(Py::tuple(vec![v_RR.clone(), v_small_boxes.clone(), v_large_boxes.clone()])))? {
        v_v = t1356;
        if ({ let t1357 = v_v.clone(); Py::Bool({ let t1358 = Py::None; if !py::is(&t1357, &t1358) { true } else { false } }) }).truthy() {
            v_given = py::iadd(&v_given, &Py::Int(1i64))?;
        }
    }
    let _ = f__need({ let t1359 = v_given.clone(); Py::Bool({ let t1360 = Py::Int(1i64); if py::eq(&t1359, &t1360) { true } else { false } }) }, Py::str("give exactly one of the R-R interval RR, the number of small boxes or the number of large boxes"))?;
    v_sp = (if ({ let t1361 = v_speed.clone(); Py::Bool({ let t1362 = Py::None; if py::is(&t1361, &t1362) { true } else { false } }) }).truthy() { Py::Float(25.0) } else { f__pos(v_speed.clone(), Py::str("speed"))? });
    if ({ let t1367 = v_RR.clone(); Py::Bool({ let t1368 = Py::None; if !py::is(&t1367, &t1368) { true } else { false } }) }).truthy() {
        v_rr = f__pos(v_RR.clone(), Py::str("RR"))?;
    } else {
        if ({ let t1369 = v_small_boxes.clone(); Py::Bool({ let t1370 = Py::None; if !py::is(&t1369, &t1370) { true } else { false } }) }).truthy() {
            v_rr = py::truediv(&py::mul(&f__pos(v_small_boxes.clone(), Py::str("small_boxes"))?, &Py::Float(1.0))?, &v_sp.clone())?;
        } else {
            v_rr = py::truediv(&py::mul(&f__pos(v_large_boxes.clone(), Py::str("large_boxes"))?, &Py::Float(5.0))?, &v_sp.clone())?;
        }
    }
    v_hr = py::truediv(&Py::Float(60.0), &v_rr.clone())?;
    let _ = f__need({ let t1371 = Py::Float(20.0); Py::Bool({ let t1372 = v_hr.clone(); if py::le(&t1371, &t1372)? { { let t1373 = Py::Float(300.0); if py::le(&t1372, &t1373)? { true } else { false } } } else { false } }) }, Py::str("the heart rate must lie between 20 and 300 beats/min"))?;
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
    let t1424 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1424[0].clone();
    v_age = t1424[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1429 = Py::Int(18i64); Py::Bool({ let t1430 = v_age.clone(); if py::le(&t1429, &t1430)? { { let t1431 = Py::Int(120i64); if py::le(&t1430, &t1431)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1432 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.241))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.302))?]) })).items()?;
    if t1432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1432[0].clone();
    v_a = t1432[1].clone();
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
    let t1477 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__pos(v_Scys.clone(), Py::str("Scys"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1477.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1477[0].clone();
    v_Scys = t1477[1].clone();
    v_age = t1477[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1482 = Py::Int(18i64); Py::Bool({ let t1483 = v_age.clone(); if py::le(&t1482, &t1483)? { { let t1484 = Py::Int(120i64); if py::le(&t1483, &t1484)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1485 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.219))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.144))?]) })).items()?;
    if t1485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1485[0].clone();
    v_a = t1485[1].clone();
    let t1486 = (Py::tuple(vec![py::truediv(&v_Scr.clone(), &v_k.clone())?, py::truediv(&v_Scys.clone(), &Py::Float(0.8))?])).items()?;
    if t1486.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1486[0].clone();
    v_c = t1486[1].clone();
    v_v = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(135.0), &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], false)?, &v_a.clone())?)?, &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.544))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], false)?, &py::neg(&Py::Float(0.323))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.778))?)?)?, &py::pow(&Py::Float(0.9961), &v_age.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.963) } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eGFR"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elimination_half_life(mut v_CL: Py, mut v_Vd: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1538 = (Py::tuple(vec![f__pos(v_CL.clone(), Py::str("CL"))?, f__pos(v_Vd.clone(), Py::str("Vd"))?])).items()?;
    if t1538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CL = t1538[0].clone();
    v_Vd = t1538[1].clone();
    v_k = py::truediv(&v_CL.clone(), &v_Vd.clone())?;
    v_out = Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("half_life"), py::truediv(&py::math1("log", &Py::Float(2.0))?, &v_k.clone())?)]);
    if ({ let t1539 = v_t.clone(); Py::Bool({ let t1540 = Py::None; if !py::is(&t1539, &t1540) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction_remaining"), py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &f__nonneg(v_t.clone(), Py::str("t"))?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fev1_fvc(mut v_FEV1: Py, mut v_FVC: Py) -> R {
    let t1594 = (Py::tuple(vec![f__pos(v_FEV1.clone(), Py::str("FEV1"))?, f__pos(v_FVC.clone(), Py::str("FVC"))?])).items()?;
    if t1594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FEV1 = t1594[0].clone();
    v_FVC = t1594[1].clone();
    let _ = f__need({ let t1595 = v_FEV1.clone(); Py::Bool({ let t1596 = v_FVC.clone(); if py::le(&t1595, &t1596)? { true } else { false } }) }, Py::str("FEV1 cannot exceed FVC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("ratio"), py::truediv(&v_FEV1.clone(), &v_FVC.clone())?), (Py::str("percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_FEV1.clone())?, &v_FVC.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_cardiac_output(mut v_VO2: Py, mut v_CaO2: Py, mut v_CvO2: Py) -> R {
    let mut v_d: Py = Py::None;
    let t1650 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t1650.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1650[0].clone();
    v_CaO2 = t1650[1].clone();
    v_CvO2 = t1650[2].clone();
    let _ = f__need({ let t1651 = v_CvO2.clone(); Py::Bool({ let t1652 = v_CaO2.clone(); if py::lt(&t1651, &t1652)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    v_d = py::sub(&v_CaO2.clone(), &v_CvO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&v_d.clone(), &Py::Float(10.0))?)?), (Py::str("avDO2"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_diffusion(mut v_D: Py, mut v_A: Py, mut v_dC: Py, mut v_T: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1704 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_A.clone(), Py::str("A"))?, f__num(v_dC.clone(), Py::str("dC"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t1704.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1704[0].clone();
    v_A = t1704[1].clone();
    v_dC = t1704[2].clone();
    v_T = t1704[3].clone();
    v_x = py::mul(&v_T.clone(), &Py::Float(0.0001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("J"), py::truediv(&py::mul(&py::mul(&v_D.clone(), &v_A.clone())?, &py::mul(&v_dC.clone(), &Py::Float(0.001))?)?, &v_x.clone())?), (Py::str("t_diffusion"), py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&Py::Float(2.0), &v_D.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_saturations(mut v_VO2: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t1758 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t1758.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1758[0].clone();
    v_Hb = t1758[1].clone();
    v_PaO2 = t1758[2].clone();
    v_PvO2 = t1758[3].clone();
    let t1768 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t1768.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t1768[0].clone();
    v_SvO2 = t1768[1].clone();
    let t1769 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t1769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t1769[0].clone();
    v_cv = t1769[1].clone();
    let _ = f__need({ let t1770 = v_cv.clone(); Py::Bool({ let t1771 = v_ca.clone(); if py::lt(&t1770, &t1771)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&py::sub(&v_ca.clone(), &v_cv.clone())?, &Py::Float(10.0))?)?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_fraction(mut v_GFR: Py, mut v_RPF: Py) -> R {
    let t1823 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__pos(v_RPF.clone(), Py::str("RPF"))?])).items()?;
    if t1823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t1823[0].clone();
    v_RPF = t1823[1].clone();
    let _ = f__need({ let t1824 = v_GFR.clone(); Py::Bool({ let t1825 = v_RPF.clone(); if py::lt(&t1824, &t1825)? { true } else { false } }) }, Py::str("GFR must be smaller than the renal plasma flow"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FF"), py::truediv(&v_GFR.clone(), &v_RPF.clone())?), (Py::str("FF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_GFR.clone())?, &v_RPF.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fluid_compartments(mut v_weight: Py, mut v_TBW_fraction: Py) -> R {
    let mut v_ecf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_tbw: Py = Py::None;
    let t872 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_TBW_fraction.clone(), Py::str("TBW_fraction"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t872[0].clone();
    v_f = t872[1].clone();
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
    let t1879 = (Py::tuple(vec![f__pos(v_TBW.clone(), Py::str("TBW"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?, f__pos(v_V_added.clone(), Py::str("V_added"))?, f__nonneg(v_C_added.clone(), Py::str("C_added"))?])).items()?;
    if t1879.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TBW = t1879[0].clone();
    v_Posm = t1879[1].clone();
    v_V = t1879[2].clone();
    v_C = t1879[3].clone();
    v_f = f__fr(v_ICF_fraction.clone(), Py::str("ICF_fraction"), Py::Bool(true), Py::Bool(true))?;
    let t1889 = (Py::tuple(vec![py::mul(&v_f.clone(), &v_TBW.clone())?, py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &v_TBW.clone())?])).items()?;
    if t1889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_icf0 = t1889[0].clone();
    v_ecf0 = t1889[1].clone();
    v_osm = py::truediv(&py::add(&py::mul(&v_TBW.clone(), &v_Posm.clone())?, &py::mul(&v_V.clone(), &v_C.clone())?)?, &py::add(&v_TBW.clone(), &v_V.clone())?)?;
    v_icf = py::truediv(&py::mul(&v_icf0.clone(), &v_Posm.clone())?, &v_osm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("osm_final"), v_osm.clone()), (Py::str("ICF"), v_icf.clone()), (Py::str("ECF"), py::sub(&py::add(&v_TBW.clone(), &v_V.clone())?, &v_icf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fractional_excretion(mut v_Ux: Py, mut v_Px: Py, mut v_Ucr: Py, mut v_Pcr: Py) -> R {
    let mut v_fe: Py = Py::None;
    let t1942 = (Py::tuple(vec![f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_Px.clone(), Py::str("Px"))?, f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?])).items()?;
    if t1942.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Ux = t1942[0].clone();
    v_Px = t1942[1].clone();
    v_Ucr = t1942[2].clone();
    v_Pcr = t1942[3].clone();
    v_fe = py::truediv(&py::mul(&v_Ux.clone(), &v_Pcr.clone())?, &py::mul(&v_Px.clone(), &v_Ucr.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FE"), v_fe.clone()), (Py::str("FE_percent"), py::mul(&Py::Float(100.0), &v_fe.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_water_clearance(mut v_V: Py, mut v_Uosm: Py, mut v_Posm: Py) -> R {
    let mut v_cosm: Py = Py::None;
    let t1987 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_Uosm.clone(), Py::str("Uosm"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?])).items()?;
    if t1987.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1987[0].clone();
    v_Uosm = t1987[1].clone();
    v_Posm = t1987[2].clone();
    v_cosm = py::truediv(&py::mul(&v_Uosm.clone(), &v_V.clone())?, &v_Posm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Cosm"), v_cosm.clone()), (Py::str("CH2O"), py::sub(&v_V.clone(), &v_cosm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_volume_conversion(mut v_V: Py, mut v_T: Py, mut v_Patm: Py, mut v_PH2O: Py) -> R {
    let mut v_tk: Py = Py::None;
    let t2041 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__num(v_T.clone(), Py::str("T"))?, f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?])).items()?;
    if t2041.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t2041[0].clone();
    v_T = t2041[1].clone();
    v_Patm = t2041[2].clone();
    v_PH2O = t2041[3].clone();
    let _ = f__need({ let t2050 = { let t2042 = py::neg(&Py::Int(50i64))?; Py::Bool({ let t2043 = v_T.clone(); if py::lt(&t2042, &t2043)? { { let t2044 = Py::Int(60i64); if py::lt(&t2043, &t2044)? { true } else { false } } } else { false } }) }; if !t2050.truthy() { t2050 } else { { let t2049 = { let t2045 = v_Patm.clone(); Py::Bool({ let t2046 = v_PH2O.clone(); if py::gt(&t2045, &t2046)? { true } else { false } }) }; if !t2049.truthy() { t2049 } else { { let t2047 = v_Patm.clone(); Py::Bool({ let t2048 = Py::Int(47i64); if py::gt(&t2047, &t2048)? { true } else { false } }) } } } } }, Py::str("temperature -50..60 degC, Patm above the vapour pressures"))?;
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
    let t2104 = (Py::tuple(vec![f__pos(v_PK.clone(), Py::str("PK"))?, f__nonneg(v_PNa.clone(), Py::str("PNa"))?, f__nonneg(v_PCl.clone(), Py::str("PCl"))?])).items()?;
    if t2104.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PK = t2104[0].clone();
    v_PNa = t2104[1].clone();
    v_PCl = t2104[2].clone();
    v_vals = { let mut out = vec![]; for t2105 in py::iter(&(Py::tuple(vec![v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone()])))? { let mut v_x: Py; 
v_x = t2105;
out.push(f__pos(v_x.clone(), Py::str("concentrations"))?); } Py::list(out) };
    let t2106 = (v_vals.clone()).items()?;
    if t2106.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_K_o = t2106[0].clone();
    v_K_i = t2106[1].clone();
    v_Na_o = t2106[2].clone();
    v_Na_i = t2106[3].clone();
    v_Cl_o = t2106[4].clone();
    v_Cl_i = t2106[5].clone();
    v_T = f__num(v_T.clone(), Py::str("T"))?;
    let _ = f__need({ let t2107 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2108 = v_T.clone(); if py::lt(&t2107, &t2108)? { { let t2109 = Py::Int(100i64); if py::lt(&t2108, &t2109)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2161 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("A"))?, f__pos(v_c.clone(), Py::str("c"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t2161.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2161[0].clone();
    v_c = t2161[1].clone();
    v_T = t2161[2].clone();
    let _ = f__need({ let t2162 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2163 = v_T.clone(); if py::lt(&t2162, &t2163)? { { let t2164 = Py::Int(100i64); if py::lt(&t2163, &t2164)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2218 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Pglucose.clone(), Py::str("Pglucose"))?, f__pos(v_Tm.clone(), Py::str("Tm"))?])).items()?;
    if t2218.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t2218[0].clone();
    v_P = t2218[1].clone();
    v_Tm = t2218[2].clone();
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
    let t2278 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_HR_rest.clone(), Py::str("HR_rest"))?, f__fr(v_intensity.clone(), Py::str("intensity"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2278.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2278[0].clone();
    v_r = t2278[1].clone();
    v_f = t2278[2].clone();
    let _ = f__need({ let t2279 = Py::Int(5i64); Py::Bool({ let t2280 = v_A.clone(); if py::le(&t2279, &t2280)? { { let t2281 = Py::Int(100i64); if py::le(&t2280, &t2281)? { true } else { false } } } else { false } }) }, Py::str("age must be between 5 and 100"))?;
    v_mx = py::sub(&Py::Float(220.0), &v_A.clone())?;
    let _ = f__need({ let t2282 = v_r.clone(); Py::Bool({ let t2283 = v_mx.clone(); if py::lt(&t2282, &t2283)? { true } else { false } }) }, Py::str("the resting heart rate must be below the estimated maximum"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HRmax"), v_mx.clone()), (Py::str("HRmax_Tanaka"), py::sub(&Py::Float(208.0), &py::mul(&Py::Float(0.7), &v_A.clone())?)?), (Py::str("HRR"), py::sub(&v_mx.clone(), &v_r.clone())?), (Py::str("target"), py::add(&v_r.clone(), &py::mul(&v_f.clone(), &py::sub(&v_mx.clone(), &v_r.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_fit(mut v_P1: Py, mut v_S1: Py, mut v_P2: Py, mut v_S2: Py) -> R {
    let mut v_n: Py = Py::None;
    let t2328 = (Py::tuple(vec![f__pos(v_P1.clone(), Py::str("P1"))?, f__pos(v_P2.clone(), Py::str("P2"))?])).items()?;
    if t2328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2328[0].clone();
    v_P2 = t2328[1].clone();
    let t2338 = (Py::tuple(vec![f__fr(v_S1.clone(), Py::str("S1"), Py::Bool(true), Py::Bool(true))?, f__fr(v_S2.clone(), Py::str("S2"), Py::Bool(true), Py::Bool(true))?])).items()?;
    if t2338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S1 = t2338[0].clone();
    v_S2 = t2338[1].clone();
    let _ = f__need({ let t2343 = { let t2339 = v_P1.clone(); Py::Bool({ let t2340 = v_P2.clone(); if py::ne(&t2339, &t2340) { true } else { false } }) }; if !t2343.truthy() { t2343 } else { { let t2341 = v_S1.clone(); Py::Bool({ let t2342 = v_S2.clone(); if py::ne(&t2341, &t2342) { true } else { false } }) } } }, Py::str("the two points must differ"))?;
    let _ = f__need({ let t2344 = py::mul(&py::sub(&v_P2.clone(), &v_P1.clone())?, &py::sub(&v_S2.clone(), &v_S1.clone())?)?; Py::Bool({ let t2345 = Py::Int(0i64); if py::gt(&t2344, &t2345)? { true } else { false } }) }, Py::str("saturation must increase with PO2"))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&py::truediv(&v_S2.clone(), &py::sub(&Py::Int(1i64), &v_S2.clone())?)?, &py::truediv(&v_S1.clone(), &py::sub(&Py::Int(1i64), &v_S1.clone())?)?)?)?, &py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("P50"), py::mul(&v_P1.clone(), &py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_S1.clone())?, &v_S1.clone())?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_force_velocity(mut v_F0: Py, mut v_a: Py, mut v_b: Py, mut v_v: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let t2399 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__nonneg(v_v.clone(), Py::str("v"))?])).items()?;
    if t2399.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t2399[0].clone();
    v_a = t2399[1].clone();
    v_b = t2399[2].clone();
    v_v = t2399[3].clone();
    v_vmax = py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?;
    let _ = f__need({ let t2400 = v_v.clone(); Py::Bool({ let t2401 = v_vmax.clone(); if py::le(&t2400, &t2401)? { true } else { false } }) }, Py::str("the shortening velocity cannot exceed vmax = b F0/a"))?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("power"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), v_vmax.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_po2(mut v_SO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let t2462 = (Py::tuple(vec![f__fr(v_SO2.clone(), Py::str("SO2"), Py::Bool(true), Py::Bool(true))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2462.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_SO2 = t2462[0].clone();
    v_P50 = t2462[1].clone();
    v_n = t2462[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2"), py::mul(&v_P50.clone(), &py::pow(&py::truediv(&v_SO2.clone(), &py::sub(&Py::Float(1.0), &v_SO2.clone())?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_saturation(mut v_PO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let mut v_x: Py = Py::None;
    let t2516 = (Py::tuple(vec![f__nonneg(v_PO2.clone(), Py::str("PO2"))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2516.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t2516[0].clone();
    v_P50 = t2516[1].clone();
    v_n = t2516[2].clone();
    v_x = py::pow(&py::truediv(&v_PO2.clone(), &v_P50.clone())?, &v_n.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SO2"), py::truediv(&v_x.clone(), &py::add(&Py::Float(1.0), &v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrostatic_column(mut v_h: Py, mut v_rho: Py) -> R {
    let mut v_p: Py = Py::None;
    let t2568 = (Py::tuple(vec![f__num(v_h.clone(), Py::str("h"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t2568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2568[0].clone();
    v_rho = t2568[1].clone();
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
    let t2622 = (Py::tuple(vec![f__pos(v_mass_air.clone(), Py::str("mass_air"))?, f__num(v_mass_water.clone(), Py::str("mass_water"))?, f__pos(v_water_density.clone(), Py::str("water_density"))?, f__nonneg(v_RV.clone(), Py::str("RV"))?, f__nonneg(v_GI.clone(), Py::str("GI"))?])).items()?;
    if t2622.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ma = t2622[0].clone();
    v_Mw = t2622[1].clone();
    v_rho = t2622[2].clone();
    v_RV = t2622[3].clone();
    v_GI = t2622[4].clone();
    let _ = f__need({ let t2628 = { let t2623 = v_Mw.clone(); Py::Bool({ let t2624 = v_Ma.clone(); if py::lt(&t2623, &t2624)? { true } else { false } }) }; if !t2628.truthy() { t2628 } else { { let t2625 = Py::Float(0.95); Py::Bool({ let t2626 = v_rho.clone(); if py::lt(&t2625, &t2626)? { { let t2627 = Py::Float(1.0); if py::le(&t2626, &t2627)? { true } else { false } } } else { false } }) } } }, Py::str("underwater mass below the mass in air; water density 0.95-1.0 g/mL"))?;
    v_V = py::sub(&py::sub(&py::truediv(&py::sub(&v_Ma.clone(), &v_Mw.clone())?, &v_rho.clone())?, &v_RV.clone())?, &v_GI.clone())?;
    let _ = f__need({ let t2629 = v_V.clone(); Py::Bool({ let t2630 = Py::Int(0i64); if py::gt(&t2629, &t2630)? { true } else { false } }) }, Py::str("the computed body volume is not positive"))?;
    v_D = py::truediv(&v_Ma.clone(), &v_V.clone())?;
    let _ = f__need({ let t2631 = Py::Float(0.9); Py::Bool({ let t2632 = v_D.clone(); if py::lt(&t2631, &t2632)? { { let t2633 = Py::Float(1.1); if py::le(&t2632, &t2633)? { true } else { false } } } else { false } }) }, Py::str("the computed density is outside 0.9-1.1 g/mL (Siri gives a body fat outside 0-100 %)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("volume"), v_V.clone()), (Py::str("density"), v_D.clone()), (Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ionic_current(mut v_g: Py, mut v_Vm: Py, mut v_E: Py) -> R {
    let t2685 = (Py::tuple(vec![f__nonneg(v_g.clone(), Py::str("g"))?, f__num(v_Vm.clone(), Py::str("Vm"))?, f__num(v_E.clone(), Py::str("E"))?])).items()?;
    if t2685.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2685[0].clone();
    v_Vm = t2685[1].clone();
    v_E = t2685[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I"), py::mul(&v_g.clone(), &py::sub(&v_Vm.clone(), &v_E.clone())?)?), (Py::str("driving_force"), py::sub(&v_Vm.clone(), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_wall_tension(mut v_P: Py, mut v_r: Py, mut v_shape: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2737 = (Py::tuple(vec![f__pos(v_P.clone(), Py::str("P"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t2737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2737[0].clone();
    v_r = t2737[1].clone();
    let _ = f__need({ let t2738 = v_shape.clone(); Py::Bool({ let t2739 = Py::tuple(vec![Py::str("cylinder"), Py::str("sphere")]); if py::contains(&t2739, &t2738)? { true } else { false } }) }, Py::str("shape must be cylinder or sphere"))?;
    v_T = py::mul(&py::truediv(&py::mul(&py::mul(&v_P.clone(), &c_MMHG()?)?, &v_r.clone())?, &Py::Float(100.0))?, &(if ({ let t2740 = v_shape.clone(); Py::Bool({ let t2741 = Py::str("cylinder"); if py::eq(&t2740, &t2741) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.5) }))?;
    v_out = Py::dict(vec![(Py::str("T"), v_T.clone())]);
    if ({ let t2742 = v_h.clone(); Py::Bool({ let t2743 = Py::None; if !py::is(&t2742, &t2743) { true } else { false } }) }).truthy() {
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
    let t2795 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2795.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t2795[0].clone();
    v_H = t2795[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::mul(&Py::Float(0.252), &v_W.clone())?, &py::mul(&Py::Float(0.473), &v_H.clone())?)?, &Py::Float(48.3))? } else { py::sub(&py::add(&py::mul(&Py::Float(0.407), &v_W.clone())?, &py::mul(&Py::Float(0.267), &v_H.clone())?)?, &Py::Float(19.2))? });
    let _ = f__need({ let t2800 = Py::Int(0i64); Py::Bool({ let t2801 = v_v.clone(); if py::lt(&t2800, &t2801)? { { let t2802 = v_W.clone(); if py::lt(&t2801, &t2802)? { true } else { false } } } else { false } }) }, Py::str("the formula gives no valid lean mass for these values"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("LBM"), v_v.clone()), (Py::str("fat_mass"), py::sub(&v_W.clone(), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_length_constant(mut v_Rm: Py, mut v_Ri: Py, mut v_d: Py, mut v_x: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2854 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Ri.clone(), Py::str("Ri"))?, f__pos(v_d.clone(), Py::str("d"))?])).items()?;
    if t2854.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t2854[0].clone();
    v_Ri = t2854[1].clone();
    v_d = t2854[2].clone();
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&v_Rm.clone(), &py::mul(&v_d.clone(), &Py::Float(0.0001))?)?, &py::mul(&Py::Float(4.0), &v_Ri.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("lambda"), py::mul(&v_lam.clone(), &Py::Float(10.0))?)]);
    if ({ let t2855 = v_x.clone(); Py::Bool({ let t2856 = Py::None; if !py::is(&t2855, &t2856) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("attenuation"), py::math1("exp", &py::truediv(&py::neg(&py::truediv(&f__nonneg(v_x.clone(), Py::str("x"))?, &Py::Float(10.0))?)?, &v_lam.clone())?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lung_volumes(mut v_TV: Py, mut v_IRV: Py, mut v_ERV: Py, mut v_RV: Py) -> R {
    let t2912 = (Py::tuple(vec![f__pos(v_TV.clone(), Py::str("TV"))?, f__nonneg(v_IRV.clone(), Py::str("IRV"))?, f__nonneg(v_ERV.clone(), Py::str("ERV"))?, f__pos(v_RV.clone(), Py::str("RV"))?])).items()?;
    if t2912.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TV = t2912[0].clone();
    v_IRV = t2912[1].clone();
    v_ERV = t2912[2].clone();
    v_RV = t2912[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("IC"), py::add(&v_TV.clone(), &v_IRV.clone())?), (Py::str("FRC"), py::add(&v_ERV.clone(), &v_RV.clone())?), (Py::str("VC"), py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?), (Py::str("TLC"), py::add(&py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?, &v_RV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_arterial_pressure(mut v_SBP: Py, mut v_DBP: Py) -> R {
    let t2964 = (Py::tuple(vec![f__pos(v_SBP.clone(), Py::str("SBP"))?, f__pos(v_DBP.clone(), Py::str("DBP"))?])).items()?;
    if t2964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SBP = t2964[0].clone();
    v_DBP = t2964[1].clone();
    let _ = f__need({ let t2965 = v_SBP.clone(); Py::Bool({ let t2966 = v_DBP.clone(); if py::gt(&t2965, &t2966)? { true } else { false } }) }, Py::str("the systolic pressure must exceed the diastolic"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("MAP"), py::add(&v_DBP.clone(), &py::truediv(&py::sub(&v_SBP.clone(), &v_DBP.clone())?, &Py::Float(3.0))?)?), (Py::str("PP"), py::sub(&v_SBP.clone(), &v_DBP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_charge(mut v_Cm: Py, mut v_dV: Py, mut v_area: Py) -> R {
    let mut v_q: Py = Py::None;
    let t3018 = (Py::tuple(vec![f__pos(v_Cm.clone(), Py::str("Cm"))?, f__num(v_dV.clone(), Py::str("dV"))?, f__pos(v_area.clone(), Py::str("area"))?])).items()?;
    if t3018.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cm = t3018[0].clone();
    v_dV = t3018[1].clone();
    v_area = t3018[2].clone();
    let _ = f__need({ let t3019 = v_dV.clone(); Py::Bool({ let t3020 = Py::Int(0i64); if py::ne(&t3019, &t3020) { true } else { false } }) }, Py::str("the voltage change must not be zero"))?;
    v_q = py::mul(&py::mul(&py::mul(&v_Cm.clone(), &Py::Float(1e-06))?, &py::mul(&v_area.clone(), &Py::Float(1e-08))?)?, &py::mul(&v_dV.clone(), &Py::Float(0.001))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q_pC"), py::mul(&v_q.clone(), &Py::Float(1000000000000.0))?), (Py::str("ions"), py::truediv(&py::abs(&v_q.clone())?, &c_E_CHARGE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_time_constant(mut v_Rm: Py, mut v_Cm: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t3072 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Cm.clone(), Py::str("Cm"))?])).items()?;
    if t3072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t3072[0].clone();
    v_Cm = t3072[1].clone();
    v_tau = py::mul(&py::mul(&v_Rm.clone(), &v_Cm.clone())?, &Py::Float(0.001))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone())]);
    if ({ let t3073 = v_t.clone(); Py::Bool({ let t3074 = Py::None; if !py::is(&t3073, &t3074) { true } else { false } }) }).truthy() {
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
    let t3128 = (Py::tuple(vec![f__pos(v_MET.clone(), Py::str("MET"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_duration.clone(), Py::str("duration"))?])).items()?;
    if t3128.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MET = t3128[0].clone();
    v_W = t3128[1].clone();
    v_m = t3128[2].clone();
    v_vo2 = py::mul(&py::mul(&v_MET.clone(), &Py::Float(3.5))?, &v_W.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), v_vo2.clone()), (Py::str("kcal_per_min"), py::truediv(&v_vo2.clone(), &Py::Float(200.0))?), (Py::str("kcal"), py::mul(&py::truediv(&v_vo2.clone(), &Py::Float(200.0))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minute_ventilation(mut v_VT: Py, mut v_RR: Py, mut v_VD: Py) -> R {
    let t3182 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__pos(v_RR.clone(), Py::str("RR"))?, f__nonneg(v_VD.clone(), Py::str("VD"))?])).items()?;
    if t3182.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t3182[0].clone();
    v_RR = t3182[1].clone();
    v_VD = t3182[2].clone();
    let _ = f__need({ let t3183 = v_VD.clone(); Py::Bool({ let t3184 = v_VT.clone(); if py::lt(&t3183, &t3184)? { true } else { false } }) }, Py::str("the dead space must be smaller than the tidal volume"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VE"), py::truediv(&py::mul(&v_VT.clone(), &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VA"), py::truediv(&py::mul(&py::sub(&v_VT.clone(), &v_VD.clone())?, &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VD_VT"), py::truediv(&v_VD.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moens_korteweg(mut v_E: Py, mut v_h: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let t3236 = (Py::tuple(vec![f__pos(v_E.clone(), Py::str("E"))?, f__pos(v_h.clone(), Py::str("h"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t3236.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3236[0].clone();
    v_h = t3236[1].clone();
    v_D = t3236[2].clone();
    v_rho = t3236[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PWV"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_E.clone(), &Py::Float(1000.0))?, &py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_muscle_max_power(mut v_F0: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3288 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?])).items()?;
    if t3288.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t3288[0].clone();
    v_a = t3288[1].clone();
    v_b = t3288[2].clone();
    v_v = py::mul(&v_b.clone(), &py::sub(&py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_F0.clone(), &v_a.clone())?)?)?, &Py::Float(1.0))?)?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("v_opt"), v_v.clone()), (Py::str("F_opt"), v_F.clone()), (Py::str("P_max"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst(mut v_z: Py, mut v_Cout: Py, mut v_Cin: Py, mut v_T: Py) -> R {
    v_z = py::int(&v_z.clone())?;
    let t3340 = (Py::tuple(vec![f__pos(v_Cout.clone(), Py::str("Cout"))?, f__pos(v_Cin.clone(), Py::str("Cin"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3340.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cout = t3340[0].clone();
    v_Cin = t3340[1].clone();
    v_T = t3340[2].clone();
    let _ = f__need({ let t3346 = { let t3341 = v_z.clone(); Py::Bool({ let t3342 = Py::Int(0i64); if py::ne(&t3341, &t3342) { true } else { false } }) }; if !t3346.truthy() { t3346 } else { { let t3343 = py::neg(&Py::Int(3i64))?; Py::Bool({ let t3344 = v_z.clone(); if py::le(&t3343, &t3344)? { { let t3345 = Py::Int(3i64); if py::le(&t3344, &t3345)? { true } else { false } } } else { false } }) } } }, Py::str("the valence must be a nonzero whole number from -3 to 3"))?;
    let _ = f__need({ let t3347 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3348 = v_T.clone(); if py::lt(&t3347, &t3348)? { { let t3349 = Py::Int(100i64); if py::lt(&t3348, &t3349)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("E"), py::mul(&py::truediv(&f__rtf(v_T.clone())?, &v_z.clone())?, &py::math1("log", &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?)?), (Py::str("RT_F"), f__rtf(v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nerve_conduction_velocity(mut v_distance: Py, mut v_latency_proximal: Py, mut v_latency_distal: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t3403 = (Py::tuple(vec![f__pos(v_distance.clone(), Py::str("distance"))?, f__pos(v_latency_proximal.clone(), Py::str("latency_proximal"))?, f__nonneg(v_latency_distal.clone(), Py::str("latency_distal"))?])).items()?;
    if t3403.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t3403[0].clone();
    v_lp = t3403[1].clone();
    v_ld = t3403[2].clone();
    let _ = f__need({ let t3404 = v_lp.clone(); Py::Bool({ let t3405 = v_ld.clone(); if py::gt(&t3404, &t3405)? { true } else { false } }) }, Py::str("the proximal latency must exceed the distal latency"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("NCV"), py::truediv(&v_D.clone(), &py::sub(&v_lp.clone(), &v_ld.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_network_resistance(mut v_R: Py, mut v_arrangement: Py) -> R {
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_R = f__vec(v_R.clone(), Py::str("R"), Py::Int(1i64), Py::Int(10i64))?;
    for t5220 in py::iter(&(v_R.clone()))? {
        v_x = t5220;
        let _ = f__need({ let t5221 = v_x.clone(); Py::Bool({ let t5222 = Py::Int(0i64); if py::gt(&t5221, &t5222)? { true } else { false } }) }, Py::str("every resistance must be positive"))?;
    }
    let _ = f__need({ let t5223 = v_arrangement.clone(); Py::Bool({ let t5224 = Py::tuple(vec![Py::str("series"), Py::str("parallel")]); if py::contains(&t5224, &t5223)? { true } else { false } }) }, Py::str("arrangement must be series or parallel"))?;
    v_tot = (if ({ let t5225 = v_arrangement.clone(); Py::Bool({ let t5226 = Py::str("series"); if py::eq(&t5225, &t5226) { true } else { false } }) }).truthy() { py::sum(&v_R.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t5227 in py::iter(&(v_R.clone()))? { let mut v_x: Py; 
v_x = t5227;
out.push(py::truediv(&Py::Float(1.0), &v_x.clone())?); } Py::list(out) }, None)?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("R_total"), v_tot.clone()), (Py::str("G_total"), py::truediv(&Py::Float(1.0), &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_osm: Py, mut v_T: Py) -> R {
    let mut v_p: Py = Py::None;
    let t3457 = (Py::tuple(vec![f__nonneg(v_osm.clone(), Py::str("osm"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_osm = t3457[0].clone();
    v_T = t3457[1].clone();
    let _ = f__need({ let t3458 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3459 = v_T.clone(); if py::lt(&t3458, &t3459)? { { let t3460 = Py::Int(100i64); if py::lt(&t3459, &t3460)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_p = py::mul(&py::mul(&v_osm.clone(), &c_R_GAS()?)?, &py::add(&v_T.clone(), &c_T0C()?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pi"), v_p.clone()), (Py::str("pi_mmHg"), py::truediv(&v_p.clone(), &c_MMHG()?)?), (Py::str("pi_atm"), py::truediv(&v_p.clone(), &Py::Float(101325.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_consumption(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t3514 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t3514.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3514[0].clone();
    v_Hb = t3514[1].clone();
    v_PaO2 = t3514[2].clone();
    v_PvO2 = t3514[3].clone();
    let t3524 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t3524.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t3524[0].clone();
    v_SvO2 = t3524[1].clone();
    let t3525 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t3525.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t3525[0].clone();
    v_cv = t3525[1].clone();
    let _ = f__need({ let t3526 = v_cv.clone(); Py::Bool({ let t3527 = v_ca.clone(); if py::lt(&t3526, &t3527)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), py::mul(&py::mul(&v_CO.clone(), &py::sub(&v_ca.clone(), &v_cv.clone())?)?, &Py::Float(10.0))?), (Py::str("O2ER"), py::truediv(&py::sub(&v_ca.clone(), &v_cv.clone())?, &v_ca.clone())?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_content(mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t3590 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3590.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t3590[0].clone();
    v_SaO2 = t3590[1].clone();
    v_PaO2 = t3590[2].clone();
    let t3591 = (Py::tuple(vec![py::mul(&py::mul(&c_O2_HB()?, &v_Hb.clone())?, &v_SaO2.clone())?, py::mul(&c_O2_SOL()?, &v_PaO2.clone())?])).items()?;
    if t3591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3591[0].clone();
    v_d = t3591[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CaO2"), py::add(&v_b.clone(), &v_d.clone())?), (Py::str("bound"), v_b.clone()), (Py::str("dissolved"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_delivery(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_c: Py = Py::None;
    let t3654 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3654.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3654[0].clone();
    v_Hb = t3654[1].clone();
    v_SaO2 = t3654[2].clone();
    v_PaO2 = t3654[3].clone();
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
    let t3706 = (Py::tuple(vec![f__pos(v_U_PAH.clone(), Py::str("U_PAH"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P_PAH.clone(), Py::str("P_PAH"))?])).items()?;
    if t3706.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t3706[0].clone();
    v_V = t3706[1].clone();
    v_P = t3706[2].clone();
    let t3716 = (Py::tuple(vec![f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(false), Py::Bool(true))?, f__fr(v_E.clone(), Py::str("E"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t3716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hct = t3716[0].clone();
    v_E = t3716[1].clone();
    v_erpf = py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?;
    v_rpf = py::truediv(&v_erpf.clone(), &v_E.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eRPF"), v_erpf.clone()), (Py::str("RPF"), v_rpf.clone()), (Py::str("RBF"), py::truediv(&v_rpf.clone(), &py::sub(&Py::Float(1.0), &v_Hct.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pco2_from_ph(mut v_pH: Py, mut v_HCO3: Py) -> R {
    let t3768 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?])).items()?;
    if t3768.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t3768[0].clone();
    v_HCO3 = t3768[1].clone();
    let _ = f__need({ let t3769 = Py::Float(6.5); Py::Bool({ let t3770 = v_pH.clone(); if py::le(&t3769, &t3770)? { { let t3771 = Py::Float(8.0); if py::le(&t3770, &t3771)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PaCO2"), py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_henderson(mut v_HCO3: Py, mut v_PaCO2: Py) -> R {
    let mut v_ph: Py = Py::None;
    let t3823 = (Py::tuple(vec![f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t3823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HCO3 = t3823[0].clone();
    v_PaCO2 = t3823[1].clone();
    v_ph = py::add(&c_PK_CO2()?, &py::math1("log10", &py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pH"), v_ph.clone()), (Py::str("H"), py::pow(&Py::Float(10.0), &py::sub(&Py::Float(9.0), &v_ph.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_osmolality(mut v_Na: Py, mut v_glucose: Py, mut v_BUN: Py, mut v_measured: Py) -> R {
    let mut v_calc: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t3877 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__nonneg(v_glucose.clone(), Py::str("glucose"))?, f__nonneg(v_BUN.clone(), Py::str("BUN"))?])).items()?;
    if t3877.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t3877[0].clone();
    v_glucose = t3877[1].clone();
    v_BUN = t3877[2].clone();
    v_calc = py::add(&py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?, &py::truediv(&v_BUN.clone(), &Py::Float(2.8))?)?;
    v_out = Py::dict(vec![(Py::str("calculated"), v_calc.clone()), (Py::str("effective"), py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?)]);
    if ({ let t3878 = v_measured.clone(); Py::Bool({ let t3879 = Py::None; if !py::is(&t3878, &t3879) { true } else { false } }) }).truthy() {
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
    let t3931 = (Py::tuple(vec![f__pos(v_dP.clone(), Py::str("dP"))?, f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3931.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dP = t3931[0].clone();
    v_eta = t3931[1].clone();
    v_L = t3931[2].clone();
    v_r = t3931[3].clone();
    v_Q = py::truediv(&py::mul(&v_dP.clone(), &c_MMHG()?)?, &f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?)?;
    v_a = py::mul(&v_r.clone(), &Py::Float(0.001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q"), py::mul(&v_Q.clone(), &Py::Float(1000000.0))?), (Py::str("Q_mL_min"), py::mul(&v_Q.clone(), &Py::Float(60000000.0))?), (Py::str("v_mean"), py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&c_PI()?, &v_a.clone())?, &v_a.clone())?)?, &Py::Float(100.0))?), (Py::str("tau_wall"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_eta.clone())?, &Py::Float(0.001))?, &v_Q.clone())?, &py::mul(&c_PI()?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poiseuille_resistance(mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    let mut v_R: Py = Py::None;
    let t3983 = (Py::tuple(vec![f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3983.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eta = t3983[0].clone();
    v_L = t3983[1].clone();
    v_r = t3983[2].clone();
    v_R = f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_mmHg"), py::truediv(&py::truediv(&py::mul(&v_R.clone(), &Py::Float(1e-06))?, &Py::Float(60.0))?, &c_MMHG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pulmonary_vascular_resistance(mut v_mPAP: Py, mut v_PCWP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4037 = (Py::tuple(vec![f__num(v_mPAP.clone(), Py::str("mPAP"))?, f__nonneg(v_PCWP.clone(), Py::str("PCWP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4037.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mPAP = t4037[0].clone();
    v_PCWP = t4037[1].clone();
    v_CO = t4037[2].clone();
    let _ = f__need({ let t4038 = v_mPAP.clone(); Py::Bool({ let t4039 = v_PCWP.clone(); if py::gt(&t4038, &t4039)? { true } else { false } }) }, Py::str("the mean pulmonary artery pressure must exceed the wedge pressure"))?;
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
    let t4097 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpvO2.clone(), Py::str("SpvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpaO2.clone(), Py::str("SpaO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4097.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Sa = t4097[0].clone();
    v_Sv = t4097[1].clone();
    v_Spv = t4097[2].clone();
    v_Spa = t4097[3].clone();
    let _ = f__need({ let t4102 = { let t4098 = v_Sa.clone(); Py::Bool({ let t4099 = v_Sv.clone(); if py::gt(&t4098, &t4099)? { true } else { false } }) }; if !t4102.truthy() { t4102 } else { { let t4100 = v_Spv.clone(); Py::Bool({ let t4101 = v_Spa.clone(); if py::gt(&t4100, &t4101)? { true } else { false } }) } } }, Py::str("need SaO2 > SvO2 and SpvO2 > SpaO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qp_Qs"), py::truediv(&py::sub(&v_Sa.clone(), &v_Sv.clone())?, &py::sub(&v_Spv.clone(), &v_Spa.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qrs_axis(mut v_lead_I: Py, mut v_aVF: Py) -> R {
    let t4144 = (Py::tuple(vec![f__num(v_lead_I.clone(), Py::str("lead_I"))?, f__num(v_aVF.clone(), Py::str("aVF"))?])).items()?;
    if t4144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lead_I = t4144[0].clone();
    v_aVF = t4144[1].clone();
    let _ = f__need({ let t4149 = { let t4145 = v_lead_I.clone(); Py::Bool({ let t4146 = Py::Int(0i64); if py::ne(&t4145, &t4146) { true } else { false } }) }; if t4149.truthy() { t4149 } else { { let t4147 = v_aVF.clone(); Py::Bool({ let t4148 = Py::Int(0i64); if py::ne(&t4147, &t4148) { true } else { false } }) } } }, Py::str("at least one net deflection must be nonzero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("axis"), py::math1("degrees", &py::atan2(&v_aVF.clone(), &v_lead_I.clone())?)?), (Py::str("amplitude"), py::hypot(&[v_lead_I.clone(), v_aVF.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qt_correction(mut v_QT: Py, mut v_HR: Py) -> R {
    let mut v_rr: Py = Py::None;
    let t4194 = (Py::tuple(vec![f__pos(v_QT.clone(), Py::str("QT"))?, f__pos(v_HR.clone(), Py::str("HR"))?])).items()?;
    if t4194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_QT = t4194[0].clone();
    v_HR = t4194[1].clone();
    v_rr = py::truediv(&Py::Float(60.0), &v_HR.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RR"), v_rr.clone()), (Py::str("QTc_Bazett"), py::truediv(&v_QT.clone(), &py::math1("sqrt", &v_rr.clone())?)?), (Py::str("QTc_Fridericia"), py::truediv(&v_QT.clone(), &py::pow(&v_rr.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?), (Py::str("QTc_Framingham"), py::add(&v_QT.clone(), &py::mul(&Py::Float(154.0), &py::sub(&Py::Float(1.0), &v_rr.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radius_flow_ratio(mut v_r1: Py, mut v_r2: Py) -> R {
    let mut v_q: Py = Py::None;
    let t4245 = (Py::tuple(vec![f__pos(v_r1.clone(), Py::str("r1"))?, f__pos(v_r2.clone(), Py::str("r2"))?])).items()?;
    if t4245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t4245[0].clone();
    v_r2 = t4245[1].clone();
    v_q = py::pow(&py::truediv(&v_r2.clone(), &v_r1.clone())?, &Py::Int(4i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("flow_ratio"), v_q.clone()), (Py::str("resistance_ratio"), py::truediv(&Py::Float(1.0), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_red_cell_indices(mut v_Hct: Py, mut v_Hb: Py, mut v_RBC: Py) -> R {
    v_Hct = f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(true), Py::Bool(true))?;
    let t4298 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__pos(v_RBC.clone(), Py::str("RBC"))?])).items()?;
    if t4298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t4298[0].clone();
    v_RBC = t4298[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("MCV"), py::truediv(&py::mul(&v_Hct.clone(), &Py::Float(1000.0))?, &v_RBC.clone())?), (Py::str("MCH"), py::truediv(&py::mul(&v_Hb.clone(), &Py::Float(10.0))?, &v_RBC.clone())?), (Py::str("MCHC"), py::truediv(&v_Hb.clone(), &v_Hct.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_renal_clearance(mut v_U: Py, mut v_V: Py, mut v_P: Py) -> R {
    let t4345 = (Py::tuple(vec![f__nonneg(v_U.clone(), Py::str("U"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P.clone(), Py::str("P"))?])).items()?;
    if t4345.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4345[0].clone();
    v_V = t4345[1].clone();
    v_P = t4345[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?), (Py::str("excretion"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_mechanics(mut v_VT: Py, mut v_Ppeak: Py, mut v_Pplat: Py, mut v_PEEP: Py, mut v_flow: Py) -> R {
    let t4399 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__num(v_Ppeak.clone(), Py::str("Ppeak"))?, f__num(v_Pplat.clone(), Py::str("Pplat"))?, f__nonneg(v_PEEP.clone(), Py::str("PEEP"))?, f__pos(v_flow.clone(), Py::str("flow"))?])).items()?;
    if t4399.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t4399[0].clone();
    v_Ppeak = t4399[1].clone();
    v_Pplat = t4399[2].clone();
    v_PEEP = t4399[3].clone();
    v_flow = t4399[4].clone();
    let _ = f__need({ let t4400 = v_Ppeak.clone(); Py::Bool({ let t4401 = v_Pplat.clone(); if py::ge(&t4400, &t4401)? { { let t4402 = v_PEEP.clone(); if py::gt(&t4401, &t4402)? { true } else { false } } } else { false } }) }, Py::str("need Ppeak >= Pplat > PEEP"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("C_static"), py::truediv(&v_VT.clone(), &py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)?), (Py::str("C_dynamic"), py::truediv(&v_VT.clone(), &py::sub(&v_Ppeak.clone(), &v_PEEP.clone())?)?), (Py::str("R_aw"), py::truediv(&py::sub(&v_Ppeak.clone(), &v_Pplat.clone())?, &py::truediv(&v_flow.clone(), &Py::Float(60.0))?)?), (Py::str("driving_pressure"), py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_time_constant(mut v_R: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t4454 = (Py::tuple(vec![f__pos(v_R.clone(), Py::str("R"))?, f__pos(v_C.clone(), Py::str("C"))?])).items()?;
    if t4454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t4454[0].clone();
    v_C = t4454[1].clone();
    v_tau = py::truediv(&py::mul(&v_R.clone(), &v_C.clone())?, &Py::Float(1000.0))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("t_95"), py::mul(&Py::Float(3.0), &v_tau.clone())?)]);
    if ({ let t4455 = v_t.clone(); Py::Bool({ let t4456 = Py::None; if !py::is(&t4455, &t4456) { true } else { false } }) }).truthy() {
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
    let t4516 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?, py::sub(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?])).items()?;
    if t4516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4516[0].clone();
    v_b = t4516[1].clone();
    v_P = py::add(&py::copysign(&py::pow(&py::abs(&v_a.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_a.clone())?, &py::copysign(&py::pow(&py::abs(&v_b.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_b.clone())?)?;
    for t4517 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t4517;
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
    let t4614 = (Py::tuple(vec![f__pos(v_CcO2.clone(), Py::str("CcO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t4614.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cc = t4614[0].clone();
    v_Ca = t4614[1].clone();
    v_Cv = t4614[2].clone();
    let _ = f__need({ let t4615 = v_Cc.clone(); Py::Bool({ let t4616 = v_Ca.clone(); if py::ge(&t4615, &t4616)? { { let t4617 = v_Cv.clone(); if py::gt(&t4616, &t4617)? { true } else { false } } } else { false } }) }, Py::str("need CcO2 >= CaO2 > CvO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qs_Qt"), py::truediv(&py::sub(&v_Cc.clone(), &v_Ca.clone())?, &py::sub(&v_Cc.clone(), &v_Cv.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solution_osmolarity(mut v_conc: Py, mut v_molar_mass: Py, mut v_i: Py, mut v_phi: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4672 = (Py::tuple(vec![f__pos(v_conc.clone(), Py::str("conc"))?, f__pos(v_molar_mass.clone(), Py::str("molar_mass"))?, f__int_ok(v_i.clone(), Py::str("i"))?, f__pos(v_phi.clone(), Py::str("phi"))?])).items()?;
    if t4672.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_conc = t4672[0].clone();
    v_M = t4672[1].clone();
    v_i = t4672[2].clone();
    v_phi = t4672[3].clone();
    v_m = py::mul(&py::truediv(&v_conc.clone(), &v_M.clone())?, &Py::Float(1000.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("molarity"), v_m.clone()), (Py::str("osmolarity"), py::mul(&py::mul(&v_m.clone(), &v_i.clone())?, &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_starling_filtration(mut v_Pc: Py, mut v_Pi: Py, mut v_pic: Py, mut v_pii: Py, mut v_Kf: Py, mut v_sigma: Py) -> R {
    let mut v_nfp: Py = Py::None;
    let t4726 = (Py::tuple(vec![f__num(v_Pc.clone(), Py::str("Pc"))?, f__num(v_Pi.clone(), Py::str("Pi"))?, f__nonneg(v_pic.clone(), Py::str("pic"))?, f__nonneg(v_pii.clone(), Py::str("pii"))?, f__pos(v_Kf.clone(), Py::str("Kf"))?])).items()?;
    if t4726.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t4726[0].clone();
    v_Pi = t4726[1].clone();
    v_pic = t4726[2].clone();
    v_pii = t4726[3].clone();
    v_Kf = t4726[4].clone();
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
    let t4787 = (Py::tuple(vec![f__pos(v_rheobase.clone(), Py::str("rheobase"))?, f__pos(v_chronaxie.clone(), Py::str("chronaxie"))?, f__pos(v_t.clone(), Py::str("t"))?])).items()?;
    if t4787.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t4787[0].clone();
    v_c = t4787[1].clone();
    v_t = t4787[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I_lapicque"), py::truediv(&v_I0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_t.clone())?, &py::math1("log", &Py::Float(2.0))?)?, &v_c.clone())?)?)?)?), (Py::str("I_weiss"), py::mul(&v_I0.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_c.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_volume_ef(mut v_EDV: Py, mut v_ESV: Py) -> R {
    let mut v_sv: Py = Py::None;
    let t4841 = (Py::tuple(vec![f__pos(v_EDV.clone(), Py::str("EDV"))?, f__nonneg(v_ESV.clone(), Py::str("ESV"))?])).items()?;
    if t4841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_EDV = t4841[0].clone();
    v_ESV = t4841[1].clone();
    let _ = f__need({ let t4842 = v_ESV.clone(); Py::Bool({ let t4843 = v_EDV.clone(); if py::lt(&t4842, &t4843)? { true } else { false } }) }, Py::str("ESV must be smaller than EDV"))?;
    v_sv = py::sub(&v_EDV.clone(), &v_ESV.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SV"), v_sv.clone()), (Py::str("EF"), py::truediv(&v_sv.clone(), &v_EDV.clone())?), (Py::str("EF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_sv.clone())?, &v_EDV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_work(mut v_SV: Py, mut v_MAP: Py, mut v_HR: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4895 = (Py::tuple(vec![f__pos(v_SV.clone(), Py::str("SV"))?, f__pos(v_MAP.clone(), Py::str("MAP"))?])).items()?;
    if t4895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SV = t4895[0].clone();
    v_MAP = t4895[1].clone();
    v_w = py::mul(&py::mul(&py::mul(&v_MAP.clone(), &c_MMHG()?)?, &v_SV.clone())?, &Py::Float(1e-06))?;
    v_out = Py::dict(vec![(Py::str("SW"), v_w.clone()), (Py::str("SW_gm"), py::truediv(&v_w.clone(), &py::mul(&c_G0()?, &Py::Float(0.001))?)?)]);
    if ({ let t4896 = v_HR.clone(); Py::Bool({ let t4897 = Py::None; if !py::is(&t4896, &t4897) { true } else { false } }) }).truthy() {
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
    let t5005 = (Py::tuple(vec![f__pos(v_LVIDd.clone(), Py::str("LVIDd"))?, f__pos(v_LVIDs.clone(), Py::str("LVIDs"))?])).items()?;
    if t5005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_LVIDd = t5005[0].clone();
    v_LVIDs = t5005[1].clone();
    let _ = f__need({ let t5006 = v_LVIDs.clone(); Py::Bool({ let t5007 = v_LVIDd.clone(); if py::lt(&t5006, &t5007)? { true } else { false } }) }, Py::str("the end-systolic diameter must be smaller than the end-diastolic"))?;
    let t5008 = (Py::tuple(vec![f__teich(v_LVIDd.clone())?, f__teich(v_LVIDs.clone())?])).items()?;
    if t5008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edv = t5008[0].clone();
    v_esv = t5008[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("EDV"), v_edv.clone()), (Py::str("ESV"), v_esv.clone()), (Py::str("EF"), py::truediv(&py::sub(&v_edv.clone(), &v_esv.clone())?, &v_edv.clone())?), (Py::str("FS"), py::truediv(&py::sub(&v_LVIDd.clone(), &v_LVIDs.clone())?, &v_LVIDd.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tubular_handling(mut v_GFR: Py, mut v_Px: Py, mut v_Ux: Py, mut v_V: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let t5062 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Px.clone(), Py::str("Px"))?, f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_V.clone(), Py::str("V"))?])).items()?;
    if t5062.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t5062[0].clone();
    v_Px = t5062[1].clone();
    v_Ux = t5062[2].clone();
    v_V = t5062[3].clone();
    let t5063 = (Py::tuple(vec![py::truediv(&py::mul(&v_GFR.clone(), &v_Px.clone())?, &Py::Float(100.0))?, py::truediv(&py::mul(&v_Ux.clone(), &v_V.clone())?, &Py::Float(100.0))?])).items()?;
    if t5063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fl = t5063[0].clone();
    v_ex = t5063[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("filtered_load"), v_fl.clone()), (Py::str("excreted"), v_ex.clone()), (Py::str("reabsorbed"), py::sub(&v_fl.clone(), &v_ex.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vascular_resistance(mut v_MAP: Py, mut v_CVP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4951 = (Py::tuple(vec![f__num(v_MAP.clone(), Py::str("MAP"))?, f__nonneg(v_CVP.clone(), Py::str("CVP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4951.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MAP = t4951[0].clone();
    v_CVP = t4951[1].clone();
    v_CO = t4951[2].clone();
    let _ = f__need({ let t4952 = v_MAP.clone(); Py::Bool({ let t4953 = v_CVP.clone(); if py::gt(&t4952, &t4953)? { true } else { false } }) }, Py::str("the mean arterial pressure must exceed the venous pressure"))?;
    v_w = py::truediv(&py::sub(&v_MAP.clone(), &v_CVP.clone())?, &v_CO.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SVR"), v_w.clone()), (Py::str("SVR_dyn"), py::mul(&v_w.clone(), &c_WOOD_DYN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vessel_reynolds(mut v_rho: Py, mut v_v: Py, mut v_D: Py, mut v_eta: Py) -> R {
    let mut v_Re: Py = Py::None;
    let t5167 = (Py::tuple(vec![f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_v.clone(), Py::str("v"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5167.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t5167[0].clone();
    v_v = t5167[1].clone();
    v_D = t5167[2].clone();
    v_eta = t5167[3].clone();
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &py::truediv(&v_v.clone(), &Py::Float(100.0))?)?, &py::truediv(&v_D.clone(), &Py::Float(1000.0))?)?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Re"), v_Re.clone()), (Py::str("v_critical"), py::mul(&py::truediv(&py::mul(&Py::Float(2000.0), &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_virtual_po2(mut v_PO2: Py, mut v_T: Py, mut v_pH: Py, mut v_PCO2: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let t5272 = (Py::tuple(vec![f__pos(v_PO2.clone(), Py::str("PO2"))?, f__num(v_T.clone(), Py::str("T"))?, f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PCO2.clone(), Py::str("PCO2"))?])).items()?;
    if t5272.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t5272[0].clone();
    v_T = t5272[1].clone();
    v_pH = t5272[2].clone();
    v_PCO2 = t5272[3].clone();
    let _ = f__need({ let t5279 = { let t5273 = Py::Int(25i64); Py::Bool({ let t5274 = v_T.clone(); if py::le(&t5273, &t5274)? { { let t5275 = Py::Int(43i64); if py::le(&t5274, &t5275)? { true } else { false } } } else { false } }) }; if !t5279.truthy() { t5279 } else { { let t5276 = Py::Float(6.8); Py::Bool({ let t5277 = v_pH.clone(); if py::le(&t5276, &t5277)? { { let t5278 = Py::Float(7.8); if py::le(&t5277, &t5278)? { true } else { false } } } else { false } }) } } }, Py::str("temperature 25-43 degC and pH 6.8-7.8"))?;
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
    let t5331 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_height.clone(), Py::str("height"))?, f__pos(v_weight.clone(), Py::str("weight"))?])).items()?;
    if t5331.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t5331[0].clone();
    v_height = t5331[1].clone();
    v_weight = t5331[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t5336 = v_age.clone(); Py::Bool({ let t5337 = Py::Int(18i64); if py::ge(&t5336, &t5337)? { true } else { false } }) }, Py::str("the Watson equations are for adults"))?;
    v_v = (if (v_fem.clone()).truthy() { py::add(&py::add(&py::neg(&Py::Float(2.097))?, &py::mul(&Py::Float(0.1069), &v_height.clone())?)?, &py::mul(&Py::Float(0.2466), &v_weight.clone())?)? } else { py::add(&py::add(&py::sub(&Py::Float(2.447), &py::mul(&Py::Float(0.09516), &v_age.clone())?)?, &py::mul(&Py::Float(0.1074), &v_height.clone())?)?, &py::mul(&Py::Float(0.3362), &v_weight.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("TBW"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weir_rq(mut v_VO2: Py, mut v_VCO2: Py) -> R {
    let mut v_ee: Py = Py::None;
    let t5389 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_VCO2.clone(), Py::str("VCO2"))?])).items()?;
    if t5389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t5389[0].clone();
    v_VCO2 = t5389[1].clone();
    v_ee = py::mul(&py::add(&py::truediv(&py::mul(&Py::Float(3.941), &v_VO2.clone())?, &Py::Float(1000.0))?, &py::truediv(&py::mul(&Py::Float(1.106), &v_VCO2.clone())?, &Py::Float(1000.0))?)?, &Py::Float(1440.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RQ"), py::truediv(&v_VCO2.clone(), &v_VO2.clone())?), (Py::str("EE"), v_ee.clone()), (Py::str("EE_kJ"), py::mul(&v_ee.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_womersley(mut v_r: Py, mut v_HR: Py, mut v_rho: Py, mut v_eta: Py) -> R {
    let mut v_w: Py = Py::None;
    let t5441 = (Py::tuple(vec![f__pos(v_r.clone(), Py::str("r"))?, f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5441.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_r = t5441[0].clone();
    v_HR = t5441[1].clone();
    v_rho = t5441[2].clone();
    v_eta = t5441[3].clone();
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
fn s_1027(mut v_kw: Py) -> R {
    return Ok({ let t1042 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_index(py::getitem(&t1042, &Py::str("CO"))?, py::getitem(&t1042, &Py::str("BSA"))?, py::method(&t1042, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1049(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1051: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1027(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1051 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1052 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1080 = (t1052).items()?;
        if t1080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1080[0].clone();
        v_w = t1080[1].clone();
        if ({ let t1053 = v_k.clone(); Py::Bool({ let t1054 = v_outputs.clone(); if !py::contains(&t1054, &t1053)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_107(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t109: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_70(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t109 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t110 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t138 = (t110).items()?;
        if t138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t138[0].clone();
        v_w = t138[1].clone();
        if ({ let t111 = v_k.clone(); Py::Bool({ let t112 = v_outputs.clone(); if !py::contains(&t112, &t111)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1081(mut v_kw: Py) -> R {
    return Ok({ let t1094 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_output(py::getitem(&t1094, &Py::str("HR"))?, py::getitem(&t1094, &Py::str("SV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1101(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1103: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1081(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1103 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1104 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1132 = (t1104).items()?;
        if t1132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1132[0].clone();
        v_w = t1132[1].clone();
        if ({ let t1105 = v_k.clone(); Py::Bool({ let t1106 = v_outputs.clone(); if !py::contains(&t1106, &t1105)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1133(mut v_kw: Py) -> R {
    return Ok({ let t1149 = f__from_si(Py::dict(vec![(Py::str("EK"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ENa"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ECl"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_chord_conductance(py::getitem(&t1149, &Py::str("gK"))?, py::getitem(&t1149, &Py::str("gNa"))?, py::getitem(&t1149, &Py::str("gCl"))?, py::getitem(&t1149, &Py::str("EK"))?, py::getitem(&t1149, &Py::str("ENa"))?, py::getitem(&t1149, &Py::str("ECl"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1156(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1158: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1133(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1158 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1159 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1187 = (t1159).items()?;
        if t1187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1187[0].clone();
        v_w = t1187[1].clone();
        if ({ let t1160 = v_k.clone(); Py::Bool({ let t1161 = v_outputs.clone(); if !py::contains(&t1161, &t1160)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1188(mut v_kw: Py) -> R {
    return Ok({ let t1208 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cockcroft_gault(py::getitem(&t1208, &Py::str("age"))?, py::getitem(&t1208, &Py::str("weight"))?, py::getitem(&t1208, &Py::str("Scr"))?, py::getitem(&t1208, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1215(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1217: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1188(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1217 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1218 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1246 = (t1218).items()?;
        if t1246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1246[0].clone();
        v_w = t1246[1].clone();
        if ({ let t1219 = v_k.clone(); Py::Bool({ let t1220 = v_outputs.clone(); if !py::contains(&t1220, &t1219)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1247(mut v_kw: Py) -> R {
    return Ok({ let t1264 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("d1"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("d2"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_conduction_velocity_scaling(py::getitem(&t1264, &Py::str("v1"))?, py::getitem(&t1264, &Py::str("d1"))?, py::getitem(&t1264, &Py::str("d2"))?, py::getitem(&t1264, &Py::str("fiber"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1271(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1273: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1247(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1273 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1274 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1302 = (t1274).items()?;
        if t1302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1302[0].clone();
        v_w = t1302[1].clone();
        if ({ let t1275 = v_k.clone(); Py::Bool({ let t1276 = v_outputs.clone(); if !py::contains(&t1276, &t1275)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1303(mut v_kw: Py) -> R {
    return Ok({ let t1316 = f__from_si(Py::dict(vec![(Py::str("Vurine"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("hours"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_creatinine_clearance_timed(py::getitem(&t1316, &Py::str("Ucr"))?, py::getitem(&t1316, &Py::str("Vurine"))?, py::getitem(&t1316, &Py::str("hours"))?, py::getitem(&t1316, &Py::str("Pcr"))?, py::method(&t1316, "get", &[Py::str("BSA"), Py::Float(1.73)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1303(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1325 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1326 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1354 = (t1326).items()?;
        if t1354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1354[0].clone();
        v_w = t1354[1].clone();
        if ({ let t1327 = v_k.clone(); Py::Bool({ let t1328 = v_outputs.clone(); if !py::contains(&t1328, &t1327)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1355(mut v_kw: Py) -> R {
    return Ok({ let t1381 = f__from_si(Py::dict(vec![(Py::str("RR"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_ecg_heart_rate(py::method(&t1381, "get", &[Py::str("RR"), Py::None])?, py::method(&t1381, "get", &[Py::str("small_boxes"), Py::None])?, py::method(&t1381, "get", &[Py::str("large_boxes"), Py::None])?, py::method(&t1381, "get", &[Py::str("speed"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1355(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1419 = (t1391).items()?;
        if t1419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1419[0].clone();
        v_w = t1419[1].clone();
        if ({ let t1392 = v_k.clone(); Py::Bool({ let t1393 = v_outputs.clone(); if !py::contains(&t1393, &t1392)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_139(mut v_kw: Py) -> R {
    return Ok({ let t152 = f__from_si(Py::dict(vec![(Py::str("gamma"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_laplace(py::getitem(&t152, &Py::str("gamma"))?, py::getitem(&t152, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1440(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1442: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1441 = v_inputs.clone(); f_egfr_ckd_epi_2021(py::getitem(&t1441, &Py::str("Scr"))?, py::getitem(&t1441, &Py::str("age"))?, py::getitem(&t1441, &Py::str("sex"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1442 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1443 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1471 = (t1443).items()?;
        if t1471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1471[0].clone();
        v_w = t1471[1].clone();
        if ({ let t1444 = v_k.clone(); Py::Bool({ let t1445 = v_outputs.clone(); if !py::contains(&t1445, &t1444)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1472(mut v_kw: Py) -> R {
    return Ok({ let t1494 = f__from_si(Py::dict(vec![(Py::str("Scys"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_egfr_ckd_epi_2021_cys(py::getitem(&t1494, &Py::str("Scr"))?, py::getitem(&t1494, &Py::str("Scys"))?, py::getitem(&t1494, &Py::str("age"))?, py::getitem(&t1494, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1472(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1504 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1532 = (t1504).items()?;
        if t1532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1532[0].clone();
        v_w = t1532[1].clone();
        if ({ let t1505 = v_k.clone(); Py::Bool({ let t1506 = v_outputs.clone(); if !py::contains(&t1506, &t1505)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1533(mut v_kw: Py) -> R {
    return Ok({ let t1550 = f__from_si(Py::dict(vec![(Py::str("CL"), Py::tuple(vec![Py::Float(2.7777777777777776e-07), Py::Float(0.0)])), (Py::str("Vd"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)]))]), v_kw.clone())?; f_elimination_half_life(py::getitem(&t1550, &Py::str("CL"))?, py::getitem(&t1550, &Py::str("Vd"))?, py::method(&t1550, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1557(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1559: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1533(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1559 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1560 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1588 = (t1560).items()?;
        if t1588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1588[0].clone();
        v_w = t1588[1].clone();
        if ({ let t1561 = v_k.clone(); Py::Bool({ let t1562 = v_outputs.clone(); if !py::contains(&t1562, &t1561)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1589(mut v_kw: Py) -> R {
    return Ok({ let t1604 = f__from_si(Py::dict(vec![(Py::str("FEV1"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("FVC"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fev1_fvc(py::getitem(&t1604, &Py::str("FEV1"))?, py::getitem(&t1604, &Py::str("FVC"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_159(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t161: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_139(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t161 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t162 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t190 = (t162).items()?;
        if t190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t190[0].clone();
        v_w = t190[1].clone();
        if ({ let t163 = v_k.clone(); Py::Bool({ let t164 = v_outputs.clone(); if !py::contains(&t164, &t163)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1611(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1613: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1589(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1613 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1614 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1642 = (t1614).items()?;
        if t1642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1642[0].clone();
        v_w = t1642[1].clone();
        if ({ let t1615 = v_k.clone(); Py::Bool({ let t1616 = v_outputs.clone(); if !py::contains(&t1616, &t1615)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1643(mut v_kw: Py) -> R {
    return Ok({ let t1660 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_cardiac_output(py::getitem(&t1660, &Py::str("VO2"))?, py::getitem(&t1660, &Py::str("CaO2"))?, py::getitem(&t1660, &Py::str("CvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1667(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1669: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1643(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1669 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1670 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1698 = (t1670).items()?;
        if t1698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1698[0].clone();
        v_w = t1698[1].clone();
        if ({ let t1671 = v_k.clone(); Py::Bool({ let t1672 = v_outputs.clone(); if !py::contains(&t1672, &t1671)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1699(mut v_kw: Py) -> R {
    return Ok({ let t1712 = f__from_si(Py::dict(vec![(Py::str("D"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("dC"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_diffusion(py::getitem(&t1712, &Py::str("D"))?, py::getitem(&t1712, &Py::str("A"))?, py::getitem(&t1712, &Py::str("dC"))?, py::getitem(&t1712, &Py::str("T"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1699(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1721 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1722 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1750 = (t1722).items()?;
        if t1750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1750[0].clone();
        v_w = t1750[1].clone();
        if ({ let t1723 = v_k.clone(); Py::Bool({ let t1724 = v_outputs.clone(); if !py::contains(&t1724, &t1723)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1751(mut v_kw: Py) -> R {
    return Ok({ let t1779 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_saturations(py::getitem(&t1779, &Py::str("VO2"))?, py::getitem(&t1779, &Py::str("Hb"))?, py::getitem(&t1779, &Py::str("SaO2"))?, py::getitem(&t1779, &Py::str("SvO2"))?, py::getitem(&t1779, &Py::str("PaO2"))?, py::getitem(&t1779, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1786(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1751(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1788 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1789 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1817 = (t1789).items()?;
        if t1817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1817[0].clone();
        v_w = t1817[1].clone();
        if ({ let t1790 = v_k.clone(); Py::Bool({ let t1791 = v_outputs.clone(); if !py::contains(&t1791, &t1790)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1818(mut v_kw: Py) -> R {
    return Ok({ let t1833 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("RPF"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_filtration_fraction(py::getitem(&t1833, &Py::str("GFR"))?, py::getitem(&t1833, &Py::str("RPF"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1840(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1818(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1842 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1843 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1871 = (t1843).items()?;
        if t1871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1871[0].clone();
        v_w = t1871[1].clone();
        if ({ let t1844 = v_k.clone(); Py::Bool({ let t1845 = v_outputs.clone(); if !py::contains(&t1845, &t1844)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1872(mut v_kw: Py) -> R {
    return Ok({ let t1897 = f__from_si(Py::dict(vec![(Py::str("TBW"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("V_added"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_shift(py::getitem(&t1897, &Py::str("TBW"))?, py::getitem(&t1897, &Py::str("Posm"))?, py::getitem(&t1897, &Py::str("V_added"))?, py::getitem(&t1897, &Py::str("C_added"))?, py::method(&t1897, "get", &[Py::str("ICF_fraction"), py::truediv(&Py::Float(2.0), &Py::Float(3.0))?])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1904(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1906: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1872(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1906 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1907 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1935 = (t1907).items()?;
        if t1935.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1935[0].clone();
        v_w = t1935[1].clone();
        if ({ let t1908 = v_k.clone(); Py::Bool({ let t1909 = v_outputs.clone(); if !py::contains(&t1909, &t1908)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_191(mut v_kw: Py) -> R {
    return Ok({ let t204 = f__from_si(Py::dict(vec![(Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_ventilation_co2(py::getitem(&t204, &Py::str("VCO2"))?, py::getitem(&t204, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1950(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1952: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1951 = v_inputs.clone(); f_fractional_excretion(py::getitem(&t1951, &Py::str("Ux"))?, py::getitem(&t1951, &Py::str("Px"))?, py::getitem(&t1951, &Py::str("Ucr"))?, py::getitem(&t1951, &Py::str("Pcr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1952 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1953 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1981 = (t1953).items()?;
        if t1981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1981[0].clone();
        v_w = t1981[1].clone();
        if ({ let t1954 = v_k.clone(); Py::Bool({ let t1955 = v_outputs.clone(); if !py::contains(&t1955, &t1954)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1982(mut v_kw: Py) -> R {
    return Ok({ let t1995 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_free_water_clearance(py::getitem(&t1995, &Py::str("V"))?, py::getitem(&t1995, &Py::str("Uosm"))?, py::getitem(&t1995, &Py::str("Posm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2004: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1982(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2004 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2005 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2033 = (t2005).items()?;
        if t2033.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2033[0].clone();
        v_w = t2033[1].clone();
        if ({ let t2006 = v_k.clone(); Py::Bool({ let t2007 = v_outputs.clone(); if !py::contains(&t2007, &t2006)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2034(mut v_kw: Py) -> R {
    return Ok({ let t2058 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_gas_volume_conversion(py::getitem(&t2058, &Py::str("V"))?, py::getitem(&t2058, &Py::str("T"))?, py::getitem(&t2058, &Py::str("Patm"))?, py::getitem(&t2058, &Py::str("PH2O"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2067: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2034(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2067 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2068 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2096 = (t2068).items()?;
        if t2096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2096[0].clone();
        v_w = t2096[1].clone();
        if ({ let t2069 = v_k.clone(); Py::Bool({ let t2070 = v_outputs.clone(); if !py::contains(&t2070, &t2069)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2097(mut v_kw: Py) -> R {
    return Ok({ let t2117 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_ghk_voltage(py::getitem(&t2117, &Py::str("PK"))?, py::getitem(&t2117, &Py::str("PNa"))?, py::getitem(&t2117, &Py::str("PCl"))?, py::getitem(&t2117, &Py::str("K_o"))?, py::getitem(&t2117, &Py::str("K_i"))?, py::getitem(&t2117, &Py::str("Na_o"))?, py::getitem(&t2117, &Py::str("Na_i"))?, py::getitem(&t2117, &Py::str("Cl_o"))?, py::getitem(&t2117, &Py::str("Cl_i"))?, py::method(&t2117, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_191(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t213 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t214 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t242 = (t214).items()?;
        if t242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t242[0].clone();
        v_w = t242[1].clone();
        if ({ let t215 = v_k.clone(); Py::Bool({ let t216 = v_outputs.clone(); if !py::contains(&t216, &t215)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2124(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2097(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2126 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2127 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2155 = (t2127).items()?;
        if t2155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2155[0].clone();
        v_w = t2155[1].clone();
        if ({ let t2128 = v_k.clone(); Py::Bool({ let t2129 = v_outputs.clone(); if !py::contains(&t2129, &t2128)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2156(mut v_kw: Py) -> R {
    return Ok({ let t2172 = f__from_si(Py::dict(vec![(Py::str("c"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_gibbs_donnan(py::getitem(&t2172, &Py::str("A"))?, py::getitem(&t2172, &Py::str("c"))?, py::method(&t2172, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2179(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2181: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2156(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2181 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2182 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2210 = (t2182).items()?;
        if t2210.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2210[0].clone();
        v_w = t2210[1].clone();
        if ({ let t2183 = v_k.clone(); Py::Bool({ let t2184 = v_outputs.clone(); if !py::contains(&t2184, &t2183)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2211(mut v_kw: Py) -> R {
    return Ok({ let t2226 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("Tm"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_glucose_titration(py::getitem(&t2226, &Py::str("GFR"))?, py::getitem(&t2226, &Py::str("Pglucose"))?, py::getitem(&t2226, &Py::str("Tm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2211(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2236 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2264 = (t2236).items()?;
        if t2264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2264[0].clone();
        v_w = t2264[1].clone();
        if ({ let t2237 = v_k.clone(); Py::Bool({ let t2238 = v_outputs.clone(); if !py::contains(&t2238, &t2237)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2291(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2293: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2292 = v_inputs.clone(); f_heart_rate_reserve(py::getitem(&t2292, &Py::str("age"))?, py::getitem(&t2292, &Py::str("HR_rest"))?, py::getitem(&t2292, &Py::str("intensity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2293 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2294 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2322 = (t2294).items()?;
        if t2322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2322[0].clone();
        v_w = t2322[1].clone();
        if ({ let t2295 = v_k.clone(); Py::Bool({ let t2296 = v_outputs.clone(); if !py::contains(&t2296, &t2295)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2323(mut v_kw: Py) -> R {
    return Ok({ let t2353 = f__from_si(Py::dict(vec![(Py::str("P1"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_fit(py::getitem(&t2353, &Py::str("P1"))?, py::getitem(&t2353, &Py::str("S1"))?, py::getitem(&t2353, &Py::str("P2"))?, py::getitem(&t2353, &Py::str("S2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2360(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2362: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2323(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2362 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2363 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2391 = (t2363).items()?;
        if t2391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2391[0].clone();
        v_w = t2391[1].clone();
        if ({ let t2364 = v_k.clone(); Py::Bool({ let t2365 = v_outputs.clone(); if !py::contains(&t2365, &t2364)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2392(mut v_kw: Py) -> R {
    return Ok({ let t2409 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_force_velocity(py::getitem(&t2409, &Py::str("F0"))?, py::getitem(&t2409, &Py::str("a"))?, py::getitem(&t2409, &Py::str("b"))?, py::getitem(&t2409, &Py::str("v"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2418: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2392(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2418 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2419 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2447 = (t2419).items()?;
        if t2447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2447[0].clone();
        v_w = t2447[1].clone();
        if ({ let t2420 = v_k.clone(); Py::Bool({ let t2421 = v_outputs.clone(); if !py::contains(&t2421, &t2420)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_243(mut v_kw: Py) -> R {
    return Ok({ let t256 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("Cl"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_anion_gap(py::getitem(&t256, &Py::str("Na"))?, py::getitem(&t256, &Py::str("Cl"))?, py::getitem(&t256, &Py::str("HCO3"))?, py::method(&t256, "get", &[Py::str("albumin"), Py::Float(4.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2448(mut v_kw: Py) -> R {
    return Ok({ let t2470 = f__from_si(Py::dict(vec![(Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_po2(py::getitem(&t2470, &Py::str("SO2"))?, py::getitem(&t2470, &Py::str("P50"))?, py::getitem(&t2470, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2477(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2479: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2448(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2479 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2480 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2508 = (t2480).items()?;
        if t2508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2508[0].clone();
        v_w = t2508[1].clone();
        if ({ let t2481 = v_k.clone(); Py::Bool({ let t2482 = v_outputs.clone(); if !py::contains(&t2482, &t2481)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2509(mut v_kw: Py) -> R {
    return Ok({ let t2524 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_saturation(py::getitem(&t2524, &Py::str("PO2"))?, py::getitem(&t2524, &Py::str("P50"))?, py::getitem(&t2524, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2531(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2533: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2509(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2533 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2534 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2562 = (t2534).items()?;
        if t2562.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2562[0].clone();
        v_w = t2562[1].clone();
        if ({ let t2535 = v_k.clone(); Py::Bool({ let t2536 = v_outputs.clone(); if !py::contains(&t2536, &t2535)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2563(mut v_kw: Py) -> R {
    return Ok({ let t2576 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_column(py::getitem(&t2576, &Py::str("h"))?, py::getitem(&t2576, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2585: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2563(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2585 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2586 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2614 = (t2586).items()?;
        if t2614.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2614[0].clone();
        v_w = t2614[1].clone();
        if ({ let t2587 = v_k.clone(); Py::Bool({ let t2588 = v_outputs.clone(); if !py::contains(&t2588, &t2587)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2615(mut v_kw: Py) -> R {
    return Ok({ let t2641 = f__from_si(Py::dict(vec![(Py::str("mass_air"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("mass_water"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("water_density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("GI"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_weighing(py::getitem(&t2641, &Py::str("mass_air"))?, py::getitem(&t2641, &Py::str("mass_water"))?, py::getitem(&t2641, &Py::str("water_density"))?, py::getitem(&t2641, &Py::str("RV"))?, py::method(&t2641, "get", &[Py::str("GI"), Py::Float(0.1)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t265: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_243(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t265 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t266 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t294 = (t266).items()?;
        if t294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t294[0].clone();
        v_w = t294[1].clone();
        if ({ let t267 = v_k.clone(); Py::Bool({ let t268 = v_outputs.clone(); if !py::contains(&t268, &t267)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2648(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2650: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2615(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2650 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2651 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2679 = (t2651).items()?;
        if t2679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2679[0].clone();
        v_w = t2679[1].clone();
        if ({ let t2652 = v_k.clone(); Py::Bool({ let t2653 = v_outputs.clone(); if !py::contains(&t2653, &t2652)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2680(mut v_kw: Py) -> R {
    return Ok({ let t2693 = f__from_si(Py::dict(vec![(Py::str("g"), Py::tuple(vec![Py::Float(1e-09), Py::Float(0.0)])), (Py::str("Vm"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("E"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_ionic_current(py::getitem(&t2693, &Py::str("g"))?, py::getitem(&t2693, &Py::str("Vm"))?, py::getitem(&t2693, &Py::str("E"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2700(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2702: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2680(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2702 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2703 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2731 = (t2703).items()?;
        if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2731[0].clone();
        v_w = t2731[1].clone();
        if ({ let t2704 = v_k.clone(); Py::Bool({ let t2705 = v_outputs.clone(); if !py::contains(&t2705, &t2704)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2732(mut v_kw: Py) -> R {
    return Ok({ let t2751 = f__from_si(Py::dict(vec![(Py::str("P"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_laplace_wall_tension(py::getitem(&t2751, &Py::str("P"))?, py::getitem(&t2751, &Py::str("r"))?, py::getitem(&t2751, &Py::str("shape"))?, py::method(&t2751, "get", &[Py::str("h"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2758(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2760: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2732(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2760 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2761 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2789 = (t2761).items()?;
        if t2789.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2789[0].clone();
        v_w = t2789[1].clone();
        if ({ let t2762 = v_k.clone(); Py::Bool({ let t2763 = v_outputs.clone(); if !py::contains(&t2763, &t2762)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2790(mut v_kw: Py) -> R {
    return Ok({ let t2810 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_lean_body_mass_boer(py::getitem(&t2810, &Py::str("weight"))?, py::getitem(&t2810, &Py::str("height"))?, py::getitem(&t2810, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2819: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2790(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2819 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2820 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2848 = (t2820).items()?;
        if t2848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2848[0].clone();
        v_w = t2848[1].clone();
        if ({ let t2821 = v_k.clone(); Py::Bool({ let t2822 = v_outputs.clone(); if !py::contains(&t2822, &t2821)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2849(mut v_kw: Py) -> R {
    return Ok({ let t2866 = f__from_si(Py::dict(vec![(Py::str("d"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("x"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_length_constant(py::getitem(&t2866, &Py::str("Rm"))?, py::getitem(&t2866, &Py::str("Ri"))?, py::getitem(&t2866, &Py::str("d"))?, py::method(&t2866, "get", &[Py::str("x"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2873(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2875: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2849(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2875 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2876 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2904 = (t2876).items()?;
        if t2904.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2904[0].clone();
        v_w = t2904[1].clone();
        if ({ let t2877 = v_k.clone(); Py::Bool({ let t2878 = v_outputs.clone(); if !py::contains(&t2878, &t2877)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2905(mut v_kw: Py) -> R {
    return Ok({ let t2920 = f__from_si(Py::dict(vec![(Py::str("TV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("IRV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ERV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_lung_volumes(py::getitem(&t2920, &Py::str("TV"))?, py::getitem(&t2920, &Py::str("IRV"))?, py::getitem(&t2920, &Py::str("ERV"))?, py::getitem(&t2920, &Py::str("RV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2927(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2929: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2905(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2929 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2930 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2958 = (t2930).items()?;
        if t2958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2958[0].clone();
        v_w = t2958[1].clone();
        if ({ let t2931 = v_k.clone(); Py::Bool({ let t2932 = v_outputs.clone(); if !py::contains(&t2932, &t2931)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_295(mut v_kw: Py) -> R {
    return Ok({ let t310 = f__from_si(Py::dict(vec![(Py::str("carbohydrate"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("fat"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("protein"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("alcohol"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_atwater_energy(py::getitem(&t310, &Py::str("carbohydrate"))?, py::getitem(&t310, &Py::str("fat"))?, py::getitem(&t310, &Py::str("protein"))?, py::method(&t310, "get", &[Py::str("alcohol"), Py::Float(0.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2959(mut v_kw: Py) -> R {
    return Ok({ let t2974 = f__from_si(Py::dict(vec![(Py::str("SBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("DBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_mean_arterial_pressure(py::getitem(&t2974, &Py::str("SBP"))?, py::getitem(&t2974, &Py::str("DBP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2983: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2959(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2983 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2984 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3012 = (t2984).items()?;
        if t3012.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3012[0].clone();
        v_w = t3012[1].clone();
        if ({ let t2985 = v_k.clone(); Py::Bool({ let t2986 = v_outputs.clone(); if !py::contains(&t2986, &t2985)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3013(mut v_kw: Py) -> R {
    return Ok({ let t3028 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("dV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("area"), Py::tuple(vec![Py::Float(1e-12), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_charge(py::getitem(&t3028, &Py::str("Cm"))?, py::getitem(&t3028, &Py::str("dV"))?, py::getitem(&t3028, &Py::str("area"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3035(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3037: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3013(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3037 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3038 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3066 = (t3038).items()?;
        if t3066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3066[0].clone();
        v_w = t3066[1].clone();
        if ({ let t3039 = v_k.clone(); Py::Bool({ let t3040 = v_outputs.clone(); if !py::contains(&t3040, &t3039)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3067(mut v_kw: Py) -> R {
    return Ok({ let t3084 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_time_constant(py::getitem(&t3084, &Py::str("Rm"))?, py::getitem(&t3084, &Py::str("Cm"))?, py::method(&t3084, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3091(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3093: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3067(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3093 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3094 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3122 = (t3094).items()?;
        if t3122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3122[0].clone();
        v_w = t3122[1].clone();
        if ({ let t3095 = v_k.clone(); Py::Bool({ let t3096 = v_outputs.clone(); if !py::contains(&t3096, &t3095)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3123(mut v_kw: Py) -> R {
    return Ok({ let t3136 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("duration"), Py::tuple(vec![Py::Float(60.0), Py::Float(0.0)]))]), v_kw.clone())?; f_met_energy(py::getitem(&t3136, &Py::str("MET"))?, py::getitem(&t3136, &Py::str("weight"))?, py::getitem(&t3136, &Py::str("duration"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3145: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3123(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3145 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3146 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3174 = (t3146).items()?;
        if t3174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3174[0].clone();
        v_w = t3174[1].clone();
        if ({ let t3147 = v_k.clone(); Py::Bool({ let t3148 = v_outputs.clone(); if !py::contains(&t3148, &t3147)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_317(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t319: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_295(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t319 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t320 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t348 = (t320).items()?;
        if t348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t348[0].clone();
        v_w = t348[1].clone();
        if ({ let t321 = v_k.clone(); Py::Bool({ let t322 = v_outputs.clone(); if !py::contains(&t322, &t321)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3175(mut v_kw: Py) -> R {
    return Ok({ let t3192 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("VD"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_minute_ventilation(py::getitem(&t3192, &Py::str("VT"))?, py::getitem(&t3192, &Py::str("RR"))?, py::getitem(&t3192, &Py::str("VD"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3175(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3202 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3230 = (t3202).items()?;
        if t3230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3230[0].clone();
        v_w = t3230[1].clone();
        if ({ let t3203 = v_k.clone(); Py::Bool({ let t3204 = v_outputs.clone(); if !py::contains(&t3204, &t3203)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3231(mut v_kw: Py) -> R {
    return Ok({ let t3244 = f__from_si(Py::dict(vec![(Py::str("E"), Py::tuple(vec![Py::Float(1000.0), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_moens_korteweg(py::getitem(&t3244, &Py::str("E"))?, py::getitem(&t3244, &Py::str("h"))?, py::getitem(&t3244, &Py::str("D"))?, py::getitem(&t3244, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3251(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3253: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3231(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3253 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3254 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3282 = (t3254).items()?;
        if t3282.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3282[0].clone();
        v_w = t3282[1].clone();
        if ({ let t3255 = v_k.clone(); Py::Bool({ let t3256 = v_outputs.clone(); if !py::contains(&t3256, &t3255)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3283(mut v_kw: Py) -> R {
    return Ok({ let t3296 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_muscle_max_power(py::getitem(&t3296, &Py::str("F0"))?, py::getitem(&t3296, &Py::str("a"))?, py::getitem(&t3296, &Py::str("b"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3303(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3305: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3283(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3305 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3306 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3334 = (t3306).items()?;
        if t3334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3334[0].clone();
        v_w = t3334[1].clone();
        if ({ let t3307 = v_k.clone(); Py::Bool({ let t3308 = v_outputs.clone(); if !py::contains(&t3308, &t3307)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3335(mut v_kw: Py) -> R {
    return Ok({ let t3357 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_nernst(py::getitem(&t3357, &Py::str("z"))?, py::getitem(&t3357, &Py::str("Cout"))?, py::getitem(&t3357, &Py::str("Cin"))?, py::method(&t3357, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3364(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3366: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3335(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3366 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3367 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3395 = (t3367).items()?;
        if t3395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3395[0].clone();
        v_w = t3395[1].clone();
        if ({ let t3368 = v_k.clone(); Py::Bool({ let t3369 = v_outputs.clone(); if !py::contains(&t3369, &t3368)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3396(mut v_kw: Py) -> R {
    return Ok({ let t3413 = f__from_si(Py::dict(vec![(Py::str("distance"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_proximal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_distal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_nerve_conduction_velocity(py::getitem(&t3413, &Py::str("distance"))?, py::getitem(&t3413, &Py::str("latency_proximal"))?, py::getitem(&t3413, &Py::str("latency_distal"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3420(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3396(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3422 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3423 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3451 = (t3423).items()?;
        if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3451[0].clone();
        v_w = t3451[1].clone();
        if ({ let t3424 = v_k.clone(); Py::Bool({ let t3425 = v_outputs.clone(); if !py::contains(&t3425, &t3424)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3452(mut v_kw: Py) -> R {
    return Ok({ let t3468 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_osmotic_pressure(py::getitem(&t3468, &Py::str("osm"))?, py::method(&t3468, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3452(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3506 = (t3478).items()?;
        if t3506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3506[0].clone();
        v_w = t3506[1].clone();
        if ({ let t3479 = v_k.clone(); Py::Bool({ let t3480 = v_outputs.clone(); if !py::contains(&t3480, &t3479)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_349(mut v_kw: Py) -> R {
    return Ok({ let t371 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_barometric_altitude(py::getitem(&t371, &Py::str("h"))?, py::method(&t371, "get", &[Py::str("FiO2"), Py::Float(0.2095)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3507(mut v_kw: Py) -> R {
    return Ok({ let t3535 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_consumption(py::getitem(&t3535, &Py::str("CO"))?, py::getitem(&t3535, &Py::str("Hb"))?, py::getitem(&t3535, &Py::str("SaO2"))?, py::getitem(&t3535, &Py::str("SvO2"))?, py::getitem(&t3535, &Py::str("PaO2"))?, py::getitem(&t3535, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3542(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3544: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3507(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3544 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3545 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3573 = (t3545).items()?;
        if t3573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3573[0].clone();
        v_w = t3573[1].clone();
        if ({ let t3546 = v_k.clone(); Py::Bool({ let t3547 = v_outputs.clone(); if !py::contains(&t3547, &t3546)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3574(mut v_kw: Py) -> R {
    return Ok({ let t3599 = f__from_si(Py::dict(vec![(Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_content(py::getitem(&t3599, &Py::str("Hb"))?, py::getitem(&t3599, &Py::str("SaO2"))?, py::getitem(&t3599, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3606(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3608: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3574(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3608 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3609 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3637 = (t3609).items()?;
        if t3637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3637[0].clone();
        v_w = t3637[1].clone();
        if ({ let t3610 = v_k.clone(); Py::Bool({ let t3611 = v_outputs.clone(); if !py::contains(&t3611, &t3610)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3638(mut v_kw: Py) -> R {
    return Ok({ let t3662 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_delivery(py::getitem(&t3662, &Py::str("CO"))?, py::getitem(&t3662, &Py::str("Hb"))?, py::getitem(&t3662, &Py::str("SaO2"))?, py::getitem(&t3662, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3671: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3638(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3671 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3672 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3700 = (t3672).items()?;
        if t3700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3700[0].clone();
        v_w = t3700[1].clone();
        if ({ let t3673 = v_k.clone(); Py::Bool({ let t3674 = v_outputs.clone(); if !py::contains(&t3674, &t3673)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3701(mut v_kw: Py) -> R {
    return Ok({ let t3724 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_pah_renal_flow(py::getitem(&t3724, &Py::str("U_PAH"))?, py::getitem(&t3724, &Py::str("V"))?, py::getitem(&t3724, &Py::str("P_PAH"))?, py::getitem(&t3724, &Py::str("Hct"))?, py::method(&t3724, "get", &[Py::str("E"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3731(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3733: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3701(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3733 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3734 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3762 = (t3734).items()?;
        if t3762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3762[0].clone();
        v_w = t3762[1].clone();
        if ({ let t3735 = v_k.clone(); Py::Bool({ let t3736 = v_outputs.clone(); if !py::contains(&t3736, &t3735)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3763(mut v_kw: Py) -> R {
    return Ok({ let t3779 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_pco2_from_ph(py::getitem(&t3779, &Py::str("pH"))?, py::getitem(&t3779, &Py::str("HCO3"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_378(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t380: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_349(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t380 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t381 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t409 = (t381).items()?;
        if t409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t409[0].clone();
        v_w = t409[1].clone();
        if ({ let t382 = v_k.clone(); Py::Bool({ let t383 = v_outputs.clone(); if !py::contains(&t383, &t382)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3786(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3763(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3788 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3789 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3817 = (t3789).items()?;
        if t3817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3817[0].clone();
        v_w = t3817[1].clone();
        if ({ let t3790 = v_k.clone(); Py::Bool({ let t3791 = v_outputs.clone(); if !py::contains(&t3791, &t3790)? { true } else { false } }) }).truthy() {
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
fn s_3818(mut v_kw: Py) -> R {
    return Ok({ let t3831 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_ph_henderson(py::getitem(&t3831, &Py::str("HCO3"))?, py::getitem(&t3831, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3840: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3818(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3840 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3841 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3869 = (t3841).items()?;
        if t3869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3869[0].clone();
        v_w = t3869[1].clone();
        if ({ let t3842 = v_k.clone(); Py::Bool({ let t3843 = v_outputs.clone(); if !py::contains(&t3843, &t3842)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3870(mut v_kw: Py) -> R {
    return Ok({ let t3887 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_plasma_osmolality(py::getitem(&t3887, &Py::str("Na"))?, py::getitem(&t3887, &Py::str("glucose"))?, py::getitem(&t3887, &Py::str("BUN"))?, py::method(&t3887, "get", &[Py::str("measured"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3894(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3896: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3870(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3896 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3897 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3925 = (t3897).items()?;
        if t3925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3925[0].clone();
        v_w = t3925[1].clone();
        if ({ let t3898 = v_k.clone(); Py::Bool({ let t3899 = v_outputs.clone(); if !py::contains(&t3899, &t3898)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3926(mut v_kw: Py) -> R {
    return Ok({ let t3939 = f__from_si(Py::dict(vec![(Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_flow(py::getitem(&t3939, &Py::str("dP"))?, py::getitem(&t3939, &Py::str("eta"))?, py::getitem(&t3939, &Py::str("L"))?, py::getitem(&t3939, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3926(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3977 = (t3949).items()?;
        if t3977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3977[0].clone();
        v_w = t3977[1].clone();
        if ({ let t3950 = v_k.clone(); Py::Bool({ let t3951 = v_outputs.clone(); if !py::contains(&t3951, &t3950)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3978(mut v_kw: Py) -> R {
    return Ok({ let t3991 = f__from_si(Py::dict(vec![(Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_resistance(py::getitem(&t3991, &Py::str("eta"))?, py::getitem(&t3991, &Py::str("L"))?, py::getitem(&t3991, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3998(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4000: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3978(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4000 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4001 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4029 = (t4001).items()?;
        if t4029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4029[0].clone();
        v_w = t4029[1].clone();
        if ({ let t4002 = v_k.clone(); Py::Bool({ let t4003 = v_outputs.clone(); if !py::contains(&t4003, &t4002)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4030(mut v_kw: Py) -> R {
    return Ok({ let t4047 = f__from_si(Py::dict(vec![(Py::str("mPAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PCWP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_pulmonary_vascular_resistance(py::getitem(&t4047, &Py::str("mPAP"))?, py::getitem(&t4047, &Py::str("PCWP"))?, py::getitem(&t4047, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4054(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4056: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4030(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4056 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4057 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4085 = (t4057).items()?;
        if t4085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4085[0].clone();
        v_w = t4085[1].clone();
        if ({ let t4058 = v_k.clone(); Py::Bool({ let t4059 = v_outputs.clone(); if !py::contains(&t4059, &t4058)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_410(mut v_kw: Py) -> R {
    return Ok({ let t427 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v2"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bernoulli_drop(py::getitem(&t427, &Py::str("v1"))?, py::getitem(&t427, &Py::str("v2"))?, py::getitem(&t427, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4110(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4111 = v_inputs.clone(); f_qp_qs(py::getitem(&t4111, &Py::str("SaO2"))?, py::getitem(&t4111, &Py::str("SvO2"))?, py::getitem(&t4111, &Py::str("SpvO2"))?, py::getitem(&t4111, &Py::str("SpaO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4112 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4113 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4141 = (t4113).items()?;
        if t4141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4141[0].clone();
        v_w = t4141[1].clone();
        if ({ let t4114 = v_k.clone(); Py::Bool({ let t4115 = v_outputs.clone(); if !py::contains(&t4115, &t4114)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4158 = v_inputs.clone(); f_qrs_axis(py::getitem(&t4158, &Py::str("lead_I"))?, py::getitem(&t4158, &Py::str("aVF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4160 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4188 = (t4160).items()?;
        if t4188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4188[0].clone();
        v_w = t4188[1].clone();
        if ({ let t4161 = v_k.clone(); Py::Bool({ let t4162 = v_outputs.clone(); if !py::contains(&t4162, &t4161)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4189(mut v_kw: Py) -> R {
    return Ok({ let t4202 = f__from_si(Py::dict(vec![(Py::str("QT"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_qt_correction(py::getitem(&t4202, &Py::str("QT"))?, py::getitem(&t4202, &Py::str("HR"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4211: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4189(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4211 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4212 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4240 = (t4212).items()?;
        if t4240.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4240[0].clone();
        v_w = t4240[1].clone();
        if ({ let t4213 = v_k.clone(); Py::Bool({ let t4214 = v_outputs.clone(); if !py::contains(&t4214, &t4213)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4254 = v_inputs.clone(); f_radius_flow_ratio(py::getitem(&t4254, &Py::str("r1"))?, py::getitem(&t4254, &Py::str("r2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4284 = (t4256).items()?;
        if t4284.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4284[0].clone();
        v_w = t4284[1].clone();
        if ({ let t4257 = v_k.clone(); Py::Bool({ let t4258 = v_outputs.clone(); if !py::contains(&t4258, &t4257)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4308: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4307 = v_inputs.clone(); f_red_cell_indices(py::getitem(&t4307, &Py::str("Hct"))?, py::getitem(&t4307, &Py::str("Hb"))?, py::getitem(&t4307, &Py::str("RBC"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4308 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4309 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4337 = (t4309).items()?;
        if t4337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4337[0].clone();
        v_w = t4337[1].clone();
        if ({ let t4310 = v_k.clone(); Py::Bool({ let t4311 = v_outputs.clone(); if !py::contains(&t4311, &t4310)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4338(mut v_kw: Py) -> R {
    return Ok({ let t4353 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_renal_clearance(py::getitem(&t4353, &Py::str("U"))?, py::getitem(&t4353, &Py::str("V"))?, py::getitem(&t4353, &Py::str("P"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_434(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t436: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_410(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t436 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t437 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t465 = (t437).items()?;
        if t465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t465[0].clone();
        v_w = t465[1].clone();
        if ({ let t438 = v_k.clone(); Py::Bool({ let t439 = v_outputs.clone(); if !py::contains(&t439, &t438)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4360(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4362: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4338(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4362 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4363 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4391 = (t4363).items()?;
        if t4391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4391[0].clone();
        v_w = t4391[1].clone();
        if ({ let t4364 = v_k.clone(); Py::Bool({ let t4365 = v_outputs.clone(); if !py::contains(&t4365, &t4364)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4392(mut v_kw: Py) -> R {
    return Ok({ let t4410 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("flow"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_mechanics(py::getitem(&t4410, &Py::str("VT"))?, py::getitem(&t4410, &Py::str("Ppeak"))?, py::getitem(&t4410, &Py::str("Pplat"))?, py::getitem(&t4410, &Py::str("PEEP"))?, py::getitem(&t4410, &Py::str("flow"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4417(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4419: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4392(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4419 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4420 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4448 = (t4420).items()?;
        if t4448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4448[0].clone();
        v_w = t4448[1].clone();
        if ({ let t4421 = v_k.clone(); Py::Bool({ let t4422 = v_outputs.clone(); if !py::contains(&t4422, &t4421)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4449(mut v_kw: Py) -> R {
    return Ok({ let t4466 = f__from_si(Py::dict(vec![(Py::str("t"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_time_constant(py::getitem(&t4466, &Py::str("R"))?, py::getitem(&t4466, &Py::str("C"))?, py::method(&t4466, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4449(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4504 = (t4476).items()?;
        if t4504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4504[0].clone();
        v_w = t4504[1].clone();
        if ({ let t4477 = v_k.clone(); Py::Bool({ let t4478 = v_outputs.clone(); if !py::contains(&t4478, &t4477)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4527: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4526 = v_inputs.clone(); f_severinghaus_po2(py::getitem(&t4526, &Py::str("SO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4527 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4528 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4556 = (t4528).items()?;
        if t4556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4556[0].clone();
        v_w = t4556[1].clone();
        if ({ let t4529 = v_k.clone(); Py::Bool({ let t4530 = v_outputs.clone(); if !py::contains(&t4530, &t4529)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4557(mut v_kw: Py) -> R {
    return Ok({ let t4569 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_severinghaus_saturation(py::getitem(&t4569, &Py::str("PO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4576(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4578: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4557(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4578 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4579 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4607 = (t4579).items()?;
        if t4607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4607[0].clone();
        v_w = t4607[1].clone();
        if ({ let t4580 = v_k.clone(); Py::Bool({ let t4581 = v_outputs.clone(); if !py::contains(&t4581, &t4580)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4625(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4627: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4626 = v_inputs.clone(); f_shunt_fraction(py::getitem(&t4626, &Py::str("CcO2"))?, py::getitem(&t4626, &Py::str("CaO2"))?, py::getitem(&t4626, &Py::str("CvO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4627 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4628 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4656 = (t4628).items()?;
        if t4656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4656[0].clone();
        v_w = t4656[1].clone();
        if ({ let t4629 = v_k.clone(); Py::Bool({ let t4630 = v_outputs.clone(); if !py::contains(&t4630, &t4629)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4657(mut v_kw: Py) -> R {
    return Ok({ let t4680 = f__from_si(Py::dict(vec![(Py::str("conc"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("molar_mass"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_solution_osmolarity(py::getitem(&t4680, &Py::str("conc"))?, py::getitem(&t4680, &Py::str("molar_mass"))?, py::getitem(&t4680, &Py::str("i"))?, py::method(&t4680, "get", &[Py::str("phi"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_466(mut v_kw: Py) -> R {
    return Ok({ let t482 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_bicarbonate_from_ph(py::getitem(&t482, &Py::str("pH"))?, py::getitem(&t482, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4687(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4689: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4657(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4689 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4690 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4718 = (t4690).items()?;
        if t4718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4718[0].clone();
        v_w = t4718[1].clone();
        if ({ let t4691 = v_k.clone(); Py::Bool({ let t4692 = v_outputs.clone(); if !py::contains(&t4692, &t4691)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4719(mut v_kw: Py) -> R {
    return Ok({ let t4743 = f__from_si(Py::dict(vec![(Py::str("Pc"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("Pi"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pic"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pii"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_starling_filtration(py::getitem(&t4743, &Py::str("Pc"))?, py::getitem(&t4743, &Py::str("Pi"))?, py::getitem(&t4743, &Py::str("pic"))?, py::getitem(&t4743, &Py::str("pii"))?, py::getitem(&t4743, &Py::str("Kf"))?, py::method(&t4743, "get", &[Py::str("sigma"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4752: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4719(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4752 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4753 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4781 = (t4753).items()?;
        if t4781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4781[0].clone();
        v_w = t4781[1].clone();
        if ({ let t4754 = v_k.clone(); Py::Bool({ let t4755 = v_outputs.clone(); if !py::contains(&t4755, &t4754)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4782(mut v_kw: Py) -> R {
    return Ok({ let t4795 = f__from_si(Py::dict(vec![(Py::str("chronaxie"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_strength_duration(py::getitem(&t4795, &Py::str("rheobase"))?, py::getitem(&t4795, &Py::str("chronaxie"))?, py::getitem(&t4795, &Py::str("t"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4802(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4804: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4782(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4804 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4805 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4833 = (t4805).items()?;
        if t4833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4833[0].clone();
        v_w = t4833[1].clone();
        if ({ let t4806 = v_k.clone(); Py::Bool({ let t4807 = v_outputs.clone(); if !py::contains(&t4807, &t4806)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4834(mut v_kw: Py) -> R {
    return Ok({ let t4851 = f__from_si(Py::dict(vec![(Py::str("EDV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ESV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_volume_ef(py::getitem(&t4851, &Py::str("EDV"))?, py::getitem(&t4851, &Py::str("ESV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4834(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4861 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4889 = (t4861).items()?;
        if t4889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4889[0].clone();
        v_w = t4889[1].clone();
        if ({ let t4862 = v_k.clone(); Py::Bool({ let t4863 = v_outputs.clone(); if !py::contains(&t4863, &t4862)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_489(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_466(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t491 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t492 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t520 = (t492).items()?;
        if t520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t520[0].clone();
        v_w = t520[1].clone();
        if ({ let t493 = v_k.clone(); Py::Bool({ let t494 = v_outputs.clone(); if !py::contains(&t494, &t493)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4890(mut v_kw: Py) -> R {
    return Ok({ let t4905 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_work(py::getitem(&t4905, &Py::str("SV"))?, py::getitem(&t4905, &Py::str("MAP"))?, py::method(&t4905, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4912(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4914: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4890(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4914 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4915 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4943 = (t4915).items()?;
        if t4943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4943[0].clone();
        v_w = t4943[1].clone();
        if ({ let t4916 = v_k.clone(); Py::Bool({ let t4917 = v_outputs.clone(); if !py::contains(&t4917, &t4916)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4944(mut v_kw: Py) -> R {
    return Ok({ let t4961 = f__from_si(Py::dict(vec![(Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CVP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_vascular_resistance(py::getitem(&t4961, &Py::str("MAP"))?, py::getitem(&t4961, &Py::str("CVP"))?, py::getitem(&t4961, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4968(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4970: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4944(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4970 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4971 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4999 = (t4971).items()?;
        if t4999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4999[0].clone();
        v_w = t4999[1].clone();
        if ({ let t4972 = v_k.clone(); Py::Bool({ let t4973 = v_outputs.clone(); if !py::contains(&t4973, &t4972)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5000(mut v_kw: Py) -> R {
    return Ok({ let t5016 = f__from_si(Py::dict(vec![(Py::str("LVIDd"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("LVIDs"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_teichholz(py::getitem(&t5016, &Py::str("LVIDd"))?, py::getitem(&t5016, &Py::str("LVIDs"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5023(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5025: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5000(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5025 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5026 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5054 = (t5026).items()?;
        if t5054.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5054[0].clone();
        v_w = t5054[1].clone();
        if ({ let t5027 = v_k.clone(); Py::Bool({ let t5028 = v_outputs.clone(); if !py::contains(&t5028, &t5027)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5055(mut v_kw: Py) -> R {
    return Ok({ let t5071 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_tubular_handling(py::getitem(&t5071, &Py::str("GFR"))?, py::getitem(&t5071, &Py::str("Px"))?, py::getitem(&t5071, &Py::str("Ux"))?, py::getitem(&t5071, &Py::str("V"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5078(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5080: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5055(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5080 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5081 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5109 = (t5081).items()?;
        if t5109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5109[0].clone();
        v_w = t5109[1].clone();
        if ({ let t5082 = v_k.clone(); Py::Bool({ let t5083 = v_outputs.clone(); if !py::contains(&t5083, &t5082)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5110(mut v_kw: Py) -> R {
    return Ok({ let t5123 = f__from_si(Py::dict(vec![(Py::str("dV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_compliance(py::getitem(&t5123, &Py::str("dV"))?, py::getitem(&t5123, &Py::str("dP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5132: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5110(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5132 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5133 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5161 = (t5133).items()?;
        if t5161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5161[0].clone();
        v_w = t5161[1].clone();
        if ({ let t5134 = v_k.clone(); Py::Bool({ let t5135 = v_outputs.clone(); if !py::contains(&t5135, &t5134)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5162(mut v_kw: Py) -> R {
    return Ok({ let t5175 = f__from_si(Py::dict(vec![(Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_vessel_reynolds(py::getitem(&t5175, &Py::str("rho"))?, py::getitem(&t5175, &Py::str("v"))?, py::getitem(&t5175, &Py::str("D"))?, py::getitem(&t5175, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5182(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5162(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5185 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5213 = (t5185).items()?;
        if t5213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5213[0].clone();
        v_w = t5213[1].clone();
        if ({ let t5186 = v_k.clone(); Py::Bool({ let t5187 = v_outputs.clone(); if !py::contains(&t5187, &t5186)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_521(mut v_kw: Py) -> R {
    return Ok({ let t534 = f__from_si(Py::dict(vec![(Py::str("Q"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)]))]), v_kw.clone())?; f_blood_velocity_area(py::getitem(&t534, &Py::str("Q"))?, py::getitem(&t534, &Py::str("A"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5235(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5237: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5236 = v_inputs.clone(); f_network_resistance(py::getitem(&t5236, &Py::str("R"))?, py::getitem(&t5236, &Py::str("arrangement"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5237 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5238 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5266 = (t5238).items()?;
        if t5266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5266[0].clone();
        v_w = t5266[1].clone();
        if ({ let t5239 = v_k.clone(); Py::Bool({ let t5240 = v_outputs.clone(); if !py::contains(&t5240, &t5239)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5267(mut v_kw: Py) -> R {
    return Ok({ let t5287 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("PCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_virtual_po2(py::getitem(&t5287, &Py::str("PO2"))?, py::getitem(&t5287, &Py::str("T"))?, py::getitem(&t5287, &Py::str("pH"))?, py::getitem(&t5287, &Py::str("PCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5294(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5296: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5267(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5296 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5297 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5325 = (t5297).items()?;
        if t5325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5325[0].clone();
        v_w = t5325[1].clone();
        if ({ let t5298 = v_k.clone(); Py::Bool({ let t5299 = v_outputs.clone(); if !py::contains(&t5299, &t5298)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5326(mut v_kw: Py) -> R {
    return Ok({ let t5345 = f__from_si(Py::dict(vec![(Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_watson_tbw(py::getitem(&t5345, &Py::str("age"))?, py::getitem(&t5345, &Py::str("height"))?, py::getitem(&t5345, &Py::str("weight"))?, py::getitem(&t5345, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5352(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5354: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5326(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5354 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5355 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5383 = (t5355).items()?;
        if t5383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5383[0].clone();
        v_w = t5383[1].clone();
        if ({ let t5356 = v_k.clone(); Py::Bool({ let t5357 = v_outputs.clone(); if !py::contains(&t5357, &t5356)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5384(mut v_kw: Py) -> R {
    return Ok({ let t5397 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_weir_rq(py::getitem(&t5397, &Py::str("VO2"))?, py::getitem(&t5397, &Py::str("VCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5404(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5406: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5384(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5406 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5407 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5435 = (t5407).items()?;
        if t5435.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5435[0].clone();
        v_w = t5435[1].clone();
        if ({ let t5408 = v_k.clone(); Py::Bool({ let t5409 = v_outputs.clone(); if !py::contains(&t5409, &t5408)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_541(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t543: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_521(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t543 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t544 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t572 = (t544).items()?;
        if t572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t572[0].clone();
        v_w = t572[1].clone();
        if ({ let t545 = v_k.clone(); Py::Bool({ let t546 = v_outputs.clone(); if !py::contains(&t546, &t545)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5436(mut v_kw: Py) -> R {
    return Ok({ let t5449 = f__from_si(Py::dict(vec![(Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_womersley(py::getitem(&t5449, &Py::str("r"))?, py::getitem(&t5449, &Py::str("HR"))?, py::getitem(&t5449, &Py::str("rho"))?, py::getitem(&t5449, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5456(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5458: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5436(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5458 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5487 = (t5459).items()?;
        if t5487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5487[0].clone();
        v_w = t5487[1].clone();
        if ({ let t5460 = v_k.clone(); Py::Bool({ let t5461 = v_outputs.clone(); if !py::contains(&t5461, &t5460)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_573(mut v_kw: Py) -> R {
    return Ok({ let t586 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmi(py::getitem(&t586, &Py::str("weight"))?, py::getitem(&t586, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_573(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t596 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t624 = (t596).items()?;
        if t624.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t624[0].clone();
        v_w = t624[1].clone();
        if ({ let t597 = v_k.clone(); Py::Bool({ let t598 = v_outputs.clone(); if !py::contains(&t598, &t597)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_625(mut v_kw: Py) -> R {
    return Ok({ let t648 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_harris_benedict(py::getitem(&t648, &Py::str("weight"))?, py::getitem(&t648, &Py::str("height"))?, py::getitem(&t648, &Py::str("age"))?, py::getitem(&t648, &Py::str("sex"))?, py::getitem(&t648, &Py::str("version"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_655(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t657: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_625(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t657 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t658 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t686 = (t658).items()?;
        if t686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t686[0].clone();
        v_w = t686[1].clone();
        if ({ let t659 = v_k.clone(); Py::Bool({ let t660 = v_outputs.clone(); if !py::contains(&t660, &t659)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_687(mut v_kw: Py) -> R {
    return Ok({ let t709 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_katch_mcardle(py::getitem(&t709, &Py::str("weight"))?, py::getitem(&t709, &Py::str("body_fat"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_70(mut v_kw: Py) -> R {
    return Ok({ let t100 = f__from_si(Py::dict(vec![(Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_gas(py::getitem(&t100, &Py::str("FiO2"))?, py::getitem(&t100, &Py::str("Patm"))?, py::getitem(&t100, &Py::str("PH2O"))?, py::getitem(&t100, &Py::str("PaCO2"))?, py::getitem(&t100, &Py::str("RQ"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_716(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_687(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t718 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t719 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t747 = (t719).items()?;
        if t747.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t747[0].clone();
        v_w = t747[1].clone();
        if ({ let t720 = v_k.clone(); Py::Bool({ let t721 = v_outputs.clone(); if !py::contains(&t721, &t720)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_748(mut v_kw: Py) -> R {
    return Ok({ let t767 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_mifflin(py::getitem(&t767, &Py::str("weight"))?, py::getitem(&t767, &Py::str("height"))?, py::getitem(&t767, &Py::str("age"))?, py::getitem(&t767, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_748(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t777 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t805 = (t777).items()?;
        if t805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t805[0].clone();
        v_w = t805[1].clone();
        if ({ let t778 = v_k.clone(); Py::Bool({ let t779 = v_outputs.clone(); if !py::contains(&t779, &t778)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_806(mut v_kw: Py) -> R {
    return Ok({ let t819 = f__from_si(Py::dict(vec![(Py::str("density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)]))]), v_kw.clone())?; f_body_fat_density(py::getitem(&t819, &Py::str("density"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_826(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t828: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_806(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t828 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t829 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t857 = (t829).items()?;
        if t857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t857[0].clone();
        v_w = t857[1].clone();
        if ({ let t830 = v_k.clone(); Py::Bool({ let t831 = v_outputs.clone(); if !py::contains(&t831, &t830)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_858(mut v_kw: Py) -> R {
    return Ok({ let t880 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_compartments(py::getitem(&t880, &Py::str("weight"))?, py::method(&t880, "get", &[Py::str("TBW_fraction"), Py::Float(0.6)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_887(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t889: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_858(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t889 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t890 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t918 = (t890).items()?;
        if t918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t918[0].clone();
        v_w = t918[1].clone();
        if ({ let t891 = v_k.clone(); Py::Bool({ let t892 = v_outputs.clone(); if !py::contains(&t892, &t891)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_919(mut v_kw: Py) -> R {
    return Ok({ let t932 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_body_surface_area(py::getitem(&t932, &Py::str("weight"))?, py::getitem(&t932, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_939(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t941: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_919(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t941 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t942 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t970 = (t942).items()?;
        if t970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t970[0].clone();
        v_w = t970[1].clone();
        if ({ let t943 = v_k.clone(); Py::Bool({ let t944 = v_outputs.clone(); if !py::contains(&t944, &t943)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_971(mut v_kw: Py) -> R {
    return Ok({ let t988 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PECO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_bohr_dead_space(py::getitem(&t988, &Py::str("PaCO2"))?, py::getitem(&t988, &Py::str("PECO2"))?, py::getitem(&t988, &Py::str("VT"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_995(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t997: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_971(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t997 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t998 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1026 = (t998).items()?;
        if t1026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1026[0].clone();
        v_w = t1026[1].clone();
        if ({ let t999 = v_k.clone(); Py::Bool({ let t1000 = v_outputs.clone(); if !py::contains(&t1000, &t999)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
