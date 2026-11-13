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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_36(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_gas_equation" => {
            let r = s_68({ let mut d = vec![]; for k in ["FiO2", "Patm", "PH2O", "PaCO2", "RQ"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_laplace_pressure" => {
            let r = s_135({ let mut d = vec![]; for k in ["gamma", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_alveolar_ventilation_co2" => {
            let r = s_187({ let mut d = vec![]; for k in ["VCO2", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_anion_gap" => {
            let r = s_239({ let mut d = vec![]; for k in ["Na", "Cl", "HCO3", "albumin"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_259(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_atwater_energy" => {
            let r = s_291({ let mut d = vec![]; for k in ["carbohydrate", "fat", "protein", "alcohol"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_barometric_altitude" => {
            let r = s_345({ let mut d = vec![]; for k in ["h", "FiO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_374(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bernoulli_pressure_drop" => {
            let r = s_406({ let mut d = vec![]; for k in ["v1", "v2", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_430(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bicarbonate_from_ph" => {
            let r = s_462({ let mut d = vec![]; for k in ["pH", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_485(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_blood_velocity_area" => {
            let r = s_517({ let mut d = vec![]; for k in ["Q", "A"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_537(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmi" => {
            let r = s_569({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_589(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_harris_benedict" => {
            let r = s_621({ let mut d = vec![]; for k in ["weight", "height", "age", "sex", "version"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_651(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_katch_mcardle" => {
            let r = s_683({ let mut d = vec![]; for k in ["weight", "body_fat"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_712(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bmr_mifflin_st_jeor" => {
            let r = s_744({ let mut d = vec![]; for k in ["weight", "height", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_770(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fat_from_density" => {
            let r = s_802({ let mut d = vec![]; for k in ["density"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_822(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_fluid_compartments" => {
            let r = s_854({ let mut d = vec![]; for k in ["weight", "TBW_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_883(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_body_surface_area" => {
            let r = s_915({ let mut d = vec![]; for k in ["weight", "height"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_935(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_bohr_dead_space" => {
            let r = s_967({ let mut d = vec![]; for k in ["PaCO2", "PECO2", "VT"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_991(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_index" => {
            let r = s_1023({ let mut d = vec![]; for k in ["CO", "BSA", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1045(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cardiac_output" => {
            let r = s_1077({ let mut d = vec![]; for k in ["HR", "SV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1097(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_chord_conductance" => {
            let r = s_1129({ let mut d = vec![]; for k in ["gK", "gNa", "gCl", "EK", "ENa", "ECl"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1152(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_cockcroft_gault" => {
            let r = s_1184({ let mut d = vec![]; for k in ["age", "weight", "Scr", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_conduction_velocity_scaling" => {
            let r = s_1243({ let mut d = vec![]; for k in ["v1", "d1", "d2", "fiber"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1267(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_creatinine_clearance_timed" => {
            let r = s_1299({ let mut d = vec![]; for k in ["Ucr", "Vurine", "hours", "Pcr", "BSA"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ecg_heart_rate" => {
            let r = s_1351({ let mut d = vec![]; for k in ["RR", "small_boxes", "large_boxes", "speed"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1384(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021" => {
            let r = f_egfr_ckd_epi_2021(match v.get("Scr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sex") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021_cys" => {
            let r = s_1468({ let mut d = vec![]; for k in ["Scr", "Scys", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_elimination_half_life" => {
            let r = s_1529({ let mut d = vec![]; for k in ["CL", "Vd", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1553(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fev1_fvc_ratio" => {
            let r = s_1585({ let mut d = vec![]; for k in ["FEV1", "FVC"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1607(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_cardiac_output" => {
            let r = s_1639({ let mut d = vec![]; for k in ["VO2", "CaO2", "CvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_diffusion" => {
            let r = s_1695({ let mut d = vec![]; for k in ["D", "A", "dC", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1715(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_saturations" => {
            let r = s_1747({ let mut d = vec![]; for k in ["VO2", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1782(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_filtration_fraction" => {
            let r = s_1814({ let mut d = vec![]; for k in ["GFR", "RPF"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fluid_shift" => {
            let r = s_1868({ let mut d = vec![]; for k in ["TBW", "Posm", "V_added", "C_added", "ICF_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fractional_excretion" => {
            let r = f_fractional_excretion(match v.get("Ux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ucr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pcr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_free_water_clearance" => {
            let r = s_1978({ let mut d = vec![]; for k in ["V", "Uosm", "Posm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gas_volume_conversion" => {
            let r = s_2030({ let mut d = vec![]; for k in ["V", "T", "Patm", "PH2O"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2061(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ghk_voltage" => {
            let r = s_2093({ let mut d = vec![]; for k in ["PK", "PNa", "PCl", "K_o", "K_i", "Na_o", "Na_i", "Cl_o", "Cl_i", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gibbs_donnan" => {
            let r = s_2152({ let mut d = vec![]; for k in ["A", "c", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_glucose_titration" => {
            let r = s_2207({ let mut d = vec![]; for k in ["GFR", "Pglucose", "Tm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2229(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_heart_rate_reserve" => {
            let r = f_heart_rate_reserve(match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HR_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intensity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2287(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_fit" => {
            let r = s_2319({ let mut d = vec![]; for k in ["P1", "S1", "P2", "S2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_force_velocity" => {
            let r = s_2388({ let mut d = vec![]; for k in ["F0", "a", "b", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2412(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_po2" => {
            let r = s_2444({ let mut d = vec![]; for k in ["SO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_saturation" => {
            let r = s_2505({ let mut d = vec![]; for k in ["PO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2527(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_pressure" => {
            let r = s_2559({ let mut d = vec![]; for k in ["h", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2579(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_weighing" => {
            let r = s_2611({ let mut d = vec![]; for k in ["mass_air", "mass_water", "water_density", "RV", "GI"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2644(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ionic_current" => {
            let r = s_2676({ let mut d = vec![]; for k in ["g", "Vm", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2696(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_laplace_wall_tension" => {
            let r = s_2728({ let mut d = vec![]; for k in ["P", "r", "shape", "h"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2754(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lean_body_mass_boer" => {
            let r = s_2786({ let mut d = vec![]; for k in ["weight", "height", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_length_constant" => {
            let r = s_2845({ let mut d = vec![]; for k in ["Rm", "Ri", "d", "x"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2869(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lung_capacities" => {
            let r = s_2901({ let mut d = vec![]; for k in ["TV", "IRV", "ERV", "RV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2923(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_mean_arterial_pressure" => {
            let r = s_2955({ let mut d = vec![]; for k in ["SBP", "DBP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2977(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_charge" => {
            let r = s_3009({ let mut d = vec![]; for k in ["Cm", "dV", "area"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3031(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_time_constant" => {
            let r = s_3063({ let mut d = vec![]; for k in ["Rm", "Cm", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3087(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_met_energy" => {
            let r = s_3119({ let mut d = vec![]; for k in ["MET", "weight", "duration"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3139(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_minute_ventilation" => {
            let r = s_3171({ let mut d = vec![]; for k in ["VT", "RR", "VD"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3195(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_moens_korteweg" => {
            let r = s_3227({ let mut d = vec![]; for k in ["E", "h", "D", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3247(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_muscle_max_power" => {
            let r = s_3279({ let mut d = vec![]; for k in ["F0", "a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3299(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nernst_potential" => {
            let r = s_3331({ let mut d = vec![]; for k in ["z", "Cout", "Cin", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nerve_conduction_velocity" => {
            let r = s_3392({ let mut d = vec![]; for k in ["distance", "latency_proximal", "latency_distal"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3416(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_osmotic_pressure" => {
            let r = s_3448({ let mut d = vec![]; for k in ["osm", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3471(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_consumption" => {
            let r = s_3503({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3538(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_content" => {
            let r = s_3570({ let mut d = vec![]; for k in ["Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3602(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_delivery" => {
            let r = s_3634({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3665(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pah_renal_flow" => {
            let r = s_3697({ let mut d = vec![]; for k in ["U_PAH", "V", "P_PAH", "Hct", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3727(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pco2_from_ph" => {
            let r = s_3759({ let mut d = vec![]; for k in ["pH", "HCO3"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3782(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ph_henderson_hasselbalch" => {
            let r = s_3814({ let mut d = vec![]; for k in ["HCO3", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3834(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_plasma_osmolality" => {
            let r = s_3866({ let mut d = vec![]; for k in ["Na", "glucose", "BUN", "measured"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3890(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_flow" => {
            let r = s_3922({ let mut d = vec![]; for k in ["dP", "eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3942(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_resistance" => {
            let r = s_3974({ let mut d = vec![]; for k in ["eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3994(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pulmonary_vascular_resistance" => {
            let r = s_4026({ let mut d = vec![]; for k in ["mPAP", "PCWP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4050(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qp_qs" => {
            let r = f_qp_qs(match v.get("SaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpaO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4106(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qrs_axis" => {
            let r = f_qrs_axis(match v.get("lead_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("aVF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qt_correction" => {
            let r = s_4185({ let mut d = vec![]; for k in ["QT", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_radius_flow_ratio" => {
            let r = f_radius_flow_ratio(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_red_cell_indices" => {
            let r = f_red_cell_indices(match v.get("Hct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RBC") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4302(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_renal_clearance" => {
            let r = s_4334({ let mut d = vec![]; for k in ["U", "V", "P"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_mechanics" => {
            let r = s_4388({ let mut d = vec![]; for k in ["VT", "Ppeak", "Pplat", "PEEP", "flow"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_time_constant" => {
            let r = s_4445({ let mut d = vec![]; for k in ["R", "C", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_po2" => {
            let r = f_severinghaus_po2(match v.get("SO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4521(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_saturation" => {
            let r = s_4553({ let mut d = vec![]; for k in ["PO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4572(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_shunt_fraction" => {
            let r = f_shunt_fraction(match v.get("CcO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CvO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_solution_osmolarity" => {
            let r = s_4653({ let mut d = vec![]; for k in ["conc", "molar_mass", "i", "phi"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_starling_filtration" => {
            let r = s_4715({ let mut d = vec![]; for k in ["Pc", "Pi", "pic", "pii", "Kf", "sigma"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4746(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_strength_duration" => {
            let r = s_4778({ let mut d = vec![]; for k in ["rheobase", "chronaxie", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4798(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_volume_ef" => {
            let r = s_4830({ let mut d = vec![]; for k in ["EDV", "ESV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4854(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_work" => {
            let r = s_4886({ let mut d = vec![]; for k in ["SV", "MAP", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4908(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_systemic_vascular_resistance" => {
            let r = s_4940({ let mut d = vec![]; for k in ["MAP", "CVP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_teichholz_volumes" => {
            let r = s_4996({ let mut d = vec![]; for k in ["LVIDd", "LVIDs"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_tubular_handling" => {
            let r = s_5051({ let mut d = vec![]; for k in ["GFR", "Px", "Ux", "V"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5074(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vascular_compliance" => {
            let r = s_5106({ let mut d = vec![]; for k in ["dV", "dP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5126(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessel_reynolds" => {
            let r = s_5158({ let mut d = vec![]; for k in ["rho", "v", "D", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5178(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessels_series_parallel" => {
            let r = f_network_resistance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5231(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_virtual_po2" => {
            let r = s_5263({ let mut d = vec![]; for k in ["PO2", "T", "pH", "PCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5290(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_watson_total_body_water" => {
            let r = s_5322({ let mut d = vec![]; for k in ["age", "height", "weight", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5348(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_weir_energy_expenditure" => {
            let r = s_5380({ let mut d = vec![]; for k in ["VO2", "VCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5400(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_womersley_number" => {
            let r = s_5432({ let mut d = vec![]; for k in ["r", "HR", "rho", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5452(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t5460 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t5460.truthy() { t5460 } else { { let t5459 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5459.truthy() { t5459 } else { { let t5458 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5458.truthy() { t5458 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t5461 = v_a.clone(); Py::Bool({ let t5462 = v_b.clone(); if py::eq(&t5461, &t5462) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t5463 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5463.truthy() { t5463 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5467 = py::not(&{ let t5464 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5464.truthy() { t5464 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5467.truthy() { t5467 } else { { let t5465 = py::len(&v_a.clone())?; Py::Bool({ let t5466 = py::len(&v_b.clone())?; if py::ne(&t5465, &t5466) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        v_w = Py::Float(0.0);
        for t5468 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t5469 = (t5468).items()?;
            if t5469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t5469[0].clone();
            v_y = t5469[1].clone();
            v_w = py::minmax(&[v_w.clone(), f__diff(v_x.clone(), v_y.clone())?], true)?;
        }
        return Ok(v_w.clone());
    }
    if ({ let t5474 = { let t5470 = v_a.clone(); Py::Bool({ let t5471 = Py::None; if py::is(&t5470, &t5471) { true } else { false } }) }; if t5474.truthy() { t5474 } else { { let t5472 = v_b.clone(); Py::Bool({ let t5473 = Py::None; if py::is(&t5472, &t5473) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5475 = v_a.clone(); Py::Bool({ let t5476 = v_b.clone(); if py::is(&t5475, &t5476) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t5478: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5477 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5477[0].clone();
            v_b = t5477[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5478 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t5479 = v_a.clone(); Py::Bool({ let t5480 = v_b.clone(); if py::eq(&t5479, &t5480) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5481 = v_d.clone(); Py::Bool({ let t5482 = v_d.clone(); if py::ne(&t5481, &t5482) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py, mut v_name: Py, mut v_open_lo: Py, mut v_open_hi: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t4731 = (if (v_open_lo.clone()).truthy() { { let t4723 = v_v.clone(); Py::Bool({ let t4724 = Py::Int(0i64); if py::gt(&t4723, &t4724)? { true } else { false } }) } } else { { let t4725 = v_v.clone(); Py::Bool({ let t4726 = Py::Int(0i64); if py::ge(&t4725, &t4726)? { true } else { false } }) } }); if !t4731.truthy() { t4731 } else { (if (v_open_hi.clone()).truthy() { { let t4727 = v_v.clone(); Py::Bool({ let t4728 = Py::Int(1i64); if py::lt(&t4727, &t4728)? { true } else { false } }) } } else { { let t4729 = v_v.clone(); Py::Bool({ let t4730 = Py::Int(1i64); if py::le(&t4729, &t4730)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
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
    for t5446 in py::iter(&(py::method(&v_conv.clone(), "items", &[])?))? {
        let t5450 = (t5446).items()?;
        if t5450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5450[0].clone();
        let t5451 = (t5450[1].clone()).items()?;
        if t5451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t5451[0].clone();
        v_off = t5451[1].clone();
        v_v = py::method(&v_out.clone(), "get", &[v_k.clone()])?;
        if ({ let t5449 = { let t5447 = v_v.clone(); Py::Bool({ let t5448 = Py::None; if !py::is(&t5447, &t5448) { true } else { false } }) }; if !t5449.truthy() { t5449 } else { py::not(&Py::Bool(matches!(v_v.clone(), Py::Str(_) | Py::Bool(_)))) } }).truthy() {
            py::setitem(&v_out.clone(), &v_k.clone(), py::truediv(&py::sub(&v_v.clone(), &v_off.clone())?, &v_f.clone())?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int_ok(mut v_v: Py, mut v_name: Py) -> R {
    v_v = (if ({ let t4661 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4661.truthy() { t4661 } else { { let t4660 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4660.truthy() { t4660 } else { { let t4658 = py::float(&v_v.clone())?; Py::Bool({ let t4659 = py::int(&v_v.clone())?; if py::eq(&t4658, &t4659) { true } else { false } }) } } } } }).truthy() { py::int(&v_v.clone())? } else { Py::None });
    let _ = f__need({ let t4667 = { let t4662 = v_v.clone(); Py::Bool({ let t4663 = Py::None; if !py::is(&t4662, &t4663) { true } else { false } }) }; if !t4667.truthy() { t4667 } else { { let t4664 = Py::Int(1i64); Py::Bool({ let t4665 = v_v.clone(); if py::le(&t4664, &t4665)? { { let t4666 = Py::Int(6i64); if py::le(&t4665, &t4666)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of particles from 1 to 6".to_string()].concat()))?;
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
    let _ = f__need({ let t5056 = v_v.clone(); Py::Bool({ let t5057 = Py::Int(0i64); if py::ge(&t5056, &t5057)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t5434 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5434.truthy() { t5434 } else { { let t5433 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5433.truthy() { t5433 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
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
    let _ = f__need({ let t5435 = v_v.clone(); Py::Bool({ let t5436 = Py::Int(0i64); if py::gt(&t5435, &t5436)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
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
    if ({ let t5441 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5441.truthy() { t5441 } else { { let t5440 = Py::Bool(matches!(v_v.clone(), Py::Str(_))); if t5440.truthy() { t5440 } else { { let t5438 = v_v.clone(); Py::Bool({ let t5439 = Py::None; if py::is(&t5438, &t5439) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        return Ok(f__r12(v_v.clone())?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5442 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5442;
out.push(f__rnd(v_x.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5443 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t5444 = (t5443).items()?;
if t5444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5444[0].clone();
v_x = t5444[1].clone();
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
    let _ = f__need({ let t5328 = v_v.clone(); Py::Bool({ let t5329 = Py::tuple(vec![Py::str("female"), Py::str("male")]); if py::contains(&t5329, &t5328)? { true } else { false } }) }, Py::str("sex must be female or male"))?;
    return Ok({ let t5330 = v_v.clone(); Py::Bool({ let t5331 = Py::str("female"); if py::eq(&t5330, &t5331) { true } else { false } }) });
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
    v_out = { let mut out = vec![]; for t5210 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5210;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) };
    let _ = f__need({ let t5213 = v_lo.clone(); Py::Bool({ let t5214 = py::len(&v_out.clone())?; if py::le(&t5213, &t5214)? { { let t5215 = v_hi.clone(); if py::le(&t5214, &t5215)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " needs between ".to_string(), py::format(&v_lo.clone(), "")?, " and ".to_string(), py::format(&v_hi.clone(), "")?, " entries".to_string()].concat()))?;
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
    return Ok(f__rnd(Py::dict(vec![(Py::str("PAO2"), v_pa.clone()), (Py::str("Aa_gradient"), py::sub(&v_pa.clone(), &v_PaO2.clone())?), (Py::str("a_A_ratio"), py::truediv(&v_PaO2.clone(), &v_pa.clone())?), (Py::str("PF_ratio"), py::truediv(&v_PaO2.clone(), &v_FiO2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_gas(mut v_FiO2: Py, mut v_Patm: Py, mut v_PH2O: Py, mut v_PaCO2: Py, mut v_RQ: Py) -> R {
    let mut v_pio2: Py = Py::None;
    let mut v_simp: Py = Py::None;
    v_FiO2 = f__fr(v_FiO2.clone(), Py::str("FiO2"), Py::Bool(true), Py::Bool(false))?;
    let t84 = (Py::tuple(vec![f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?, f__nonneg(v_PaCO2.clone(), Py::str("PaCO2"))?, f__pos(v_RQ.clone(), Py::str("RQ"))?])).items()?;
    if t84.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Patm = t84[0].clone();
    v_PH2O = t84[1].clone();
    v_PaCO2 = t84[2].clone();
    v_RQ = t84[3].clone();
    let _ = f__need({ let t85 = v_Patm.clone(); Py::Bool({ let t86 = v_PH2O.clone(); if py::gt(&t85, &t86)? { true } else { false } }) }, Py::str("the barometric pressure must exceed the water vapour pressure"))?;
    let _ = f__need({ let t87 = v_RQ.clone(); Py::Bool({ let t88 = Py::Float(1.5); if py::le(&t87, &t88)? { true } else { false } }) }, Py::str("RQ must be at most 1.5"))?;
    v_pio2 = py::mul(&v_FiO2.clone(), &py::sub(&v_Patm.clone(), &v_PH2O.clone())?)?;
    v_simp = py::sub(&v_pio2.clone(), &py::truediv(&v_PaCO2.clone(), &v_RQ.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PiO2"), v_pio2.clone()), (Py::str("PAO2"), v_simp.clone()), (Py::str("PAO2_full"), py::add(&v_simp.clone(), &py::truediv(&py::mul(&py::mul(&v_PaCO2.clone(), &v_FiO2.clone())?, &py::sub(&Py::Float(1.0), &v_RQ.clone())?)?, &v_RQ.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_laplace(mut v_gamma: Py, mut v_r: Py) -> R {
    let mut v_p: Py = Py::None;
    let t140 = (Py::tuple(vec![f__pos(v_gamma.clone(), Py::str("gamma"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gamma = t140[0].clone();
    v_r = t140[1].clone();
    v_p = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_gamma.clone())?, &Py::Float(0.001))?, &py::mul(&v_r.clone(), &Py::Float(1e-06))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("P"), v_p.clone()), (Py::str("P_cmH2O"), py::truediv(&v_p.clone(), &c_CMH2O()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alveolar_ventilation_co2(mut v_VCO2: Py, mut v_PaCO2: Py) -> R {
    let t192 = (Py::tuple(vec![f__pos(v_VCO2.clone(), Py::str("VCO2"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VCO2 = t192[0].clone();
    v_PaCO2 = t192[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("VA"), py::truediv(&py::truediv(&py::mul(&v_VCO2.clone(), &c_K_ALV()?)?, &v_PaCO2.clone())?, &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_anion_gap(mut v_Na: Py, mut v_Cl: Py, mut v_HCO3: Py, mut v_albumin: Py) -> R {
    let mut v_ag: Py = Py::None;
    let t244 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__pos(v_Cl.clone(), Py::str("Cl"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_albumin.clone(), Py::str("albumin"))?])).items()?;
    if t244.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t244[0].clone();
    v_Cl = t244[1].clone();
    v_HCO3 = t244[2].clone();
    v_albumin = t244[3].clone();
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
    let t296 = (Py::tuple(vec![f__nonneg(v_carbohydrate.clone(), Py::str("carbohydrate"))?, f__nonneg(v_fat.clone(), Py::str("fat"))?, f__nonneg(v_protein.clone(), Py::str("protein"))?, f__nonneg(v_alcohol.clone(), Py::str("alcohol"))?])).items()?;
    if t296.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c = t296[0].clone();
    v_f = t296[1].clone();
    v_p = t296[2].clone();
    v_a = t296[3].clone();
    v_k = py::add(&py::add(&py::add(&py::mul(&Py::Float(4.0), &v_c.clone())?, &py::mul(&Py::Float(9.0), &v_f.clone())?)?, &py::mul(&Py::Float(4.0), &v_p.clone())?)?, &py::mul(&Py::Float(7.0), &v_a.clone())?)?;
    let _ = f__need({ let t297 = v_k.clone(); Py::Bool({ let t298 = Py::Int(0i64); if py::gt(&t297, &t298)? { true } else { false } }) }, Py::str("at least one amount must be positive"))?;
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
    let _ = f__need({ let t357 = Py::Int(0i64); Py::Bool({ let t358 = v_h.clone(); if py::le(&t357, &t358)? { { let t359 = Py::Int(11000i64); if py::le(&t358, &t359)? { true } else { false } } } else { false } }) }, Py::str("altitude must be between 0 and 11000 m (troposphere)"))?;
    v_p = py::mul(&Py::Float(101325.0), &py::pow(&py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(0.0065), &v_h.clone())?, &Py::Float(288.15))?)?, &c__ISA_EXP()?)?)?;
    v_pm = py::truediv(&v_p.clone(), &c_MMHG()?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Patm"), v_pm.clone()), (Py::str("PiO2"), py::mul(&v_FiO2.clone(), &py::sub(&v_pm.clone(), &Py::Float(47.0))?)?), (Py::str("Patm_kPa"), py::truediv(&v_p.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli_drop(mut v_v1: Py, mut v_v2: Py, mut v_rho: Py) -> R {
    let mut v_dp: Py = Py::None;
    let t413 = (Py::tuple(vec![f__nonneg(v_v1.clone(), Py::str("v1"))?, f__pos(v_v2.clone(), Py::str("v2"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t413.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t413[0].clone();
    v_v2 = t413[1].clone();
    v_rho = t413[2].clone();
    let _ = f__need({ let t414 = v_v2.clone(); Py::Bool({ let t415 = v_v1.clone(); if py::gt(&t414, &t415)? { true } else { false } }) }, Py::str("the downstream (jet) velocity v2 must exceed v1"))?;
    v_dp = py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dP"), v_dp.clone()), (Py::str("dP_mmHg"), py::truediv(&v_dp.clone(), &c_MMHG()?)?), (Py::str("dP_simplified"), py::mul(&py::mul(&Py::Float(4.0), &v_v2.clone())?, &v_v2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bicarbonate_from_ph(mut v_pH: Py, mut v_PaCO2: Py) -> R {
    let t467 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t467[0].clone();
    v_PaCO2 = t467[1].clone();
    let _ = f__need({ let t468 = Py::Float(6.5); Py::Bool({ let t469 = v_pH.clone(); if py::le(&t468, &t469)? { { let t470 = Py::Float(8.0); if py::le(&t469, &t470)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HCO3"), py::mul(&py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_blood_velocity_area(mut v_Q: Py, mut v_A: Py) -> R {
    let t522 = (Py::tuple(vec![f__pos(v_Q.clone(), Py::str("Q"))?, f__pos(v_A.clone(), Py::str("A"))?])).items()?;
    if t522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t522[0].clone();
    v_A = t522[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("v"), py::truediv(&py::truediv(&py::mul(&v_Q.clone(), &Py::Float(1000.0))?, &Py::Float(60.0))?, &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmi(mut v_weight: Py, mut v_height: Py) -> R {
    let t574 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t574[0].clone();
    v_height = t574[1].clone();
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
    let t626 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t626.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t626[0].clone();
    v_H = t626[1].clone();
    v_A = t626[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t631 = v_version.clone(); Py::Bool({ let t632 = Py::tuple(vec![Py::str("original"), Py::str("revised")]); if py::contains(&t632, &t631)? { true } else { false } }) }, Py::str("version must be original (1919) or revised (1984)"))?;
    let _ = f__need({ let t633 = v_A.clone(); Py::Bool({ let t634 = Py::Int(18i64); if py::ge(&t633, &t634)? { true } else { false } }) }, Py::str("the equations are for adults"))?;
    if ({ let t635 = v_version.clone(); Py::Bool({ let t636 = Py::str("original"); if py::eq(&t635, &t636) { true } else { false } }) }).truthy() {
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
    let t697 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_body_fat.clone(), Py::str("body_fat"), Py::Bool(false), Py::Bool(true))?])).items()?;
    if t697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t697[0].clone();
    v_f = t697[1].clone();
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
    let t749 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t749.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_W = t749[0].clone();
    v_H = t749[1].clone();
    v_A = t749[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t754 = v_A.clone(); Py::Bool({ let t755 = Py::Int(18i64); if py::ge(&t754, &t755)? { true } else { false } }) }, Py::str("the equation is for adults"))?;
    v_v = py::add(&py::sub(&py::add(&py::mul(&Py::Float(10.0), &v_W.clone())?, &py::mul(&Py::Float(6.25), &v_H.clone())?)?, &py::mul(&Py::Float(5.0), &v_A.clone())?)?, &(if (v_fem.clone()).truthy() { py::neg(&Py::Float(161.0))? } else { Py::Float(5.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("REE"), v_v.clone()), (Py::str("REE_kJ"), py::mul(&v_v.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_fat_density(mut v_density: Py) -> R {
    let mut v_D: Py = Py::None;
    v_D = f__num(v_density.clone(), Py::str("density"))?;
    let _ = f__need({ let t805 = Py::Float(0.9); Py::Bool({ let t806 = v_D.clone(); if py::lt(&t805, &t806)? { { let t807 = Py::Float(1.2); if py::lt(&t806, &t807)? { true } else { false } } } else { false } }) }, Py::str("body density must be between 0.9 and 1.2 g/mL"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?), (Py::str("fat_Brozek"), py::sub(&py::truediv(&Py::Float(457.0), &v_D.clone())?, &Py::Float(414.2))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_body_surface_area(mut v_weight: Py, mut v_height: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_W: Py = Py::None;
    let t920 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t920.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t920[0].clone();
    v_H = t920[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("BSA_DuBois"), py::mul(&py::mul(&Py::Float(0.007184), &py::pow(&v_W.clone(), &Py::Float(0.425))?)?, &py::pow(&v_H.clone(), &Py::Float(0.725))?)?), (Py::str("BSA_Mosteller"), py::math1("sqrt", &py::truediv(&py::mul(&v_H.clone(), &v_W.clone())?, &Py::Float(3600.0))?)?), (Py::str("BSA_Haycock"), py::mul(&py::mul(&Py::Float(0.024265), &py::pow(&v_W.clone(), &Py::Float(0.5378))?)?, &py::pow(&v_H.clone(), &Py::Float(0.3964))?)?), (Py::str("BSA_GehanGeorge"), py::mul(&py::mul(&Py::Float(0.0235), &py::pow(&v_W.clone(), &Py::Float(0.51456))?)?, &py::pow(&v_H.clone(), &Py::Float(0.42246))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bohr_dead_space(mut v_PaCO2: Py, mut v_PECO2: Py, mut v_VT: Py) -> R {
    let mut v_f: Py = Py::None;
    let t974 = (Py::tuple(vec![f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?, f__nonneg(v_PECO2.clone(), Py::str("PECO2"))?, f__pos(v_VT.clone(), Py::str("VT"))?])).items()?;
    if t974.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PaCO2 = t974[0].clone();
    v_PECO2 = t974[1].clone();
    v_VT = t974[2].clone();
    let _ = f__need({ let t975 = v_PECO2.clone(); Py::Bool({ let t976 = v_PaCO2.clone(); if py::lt(&t975, &t976)? { true } else { false } }) }, Py::str("the mixed expired PCO2 must be below the arterial PCO2"))?;
    v_f = py::truediv(&py::sub(&v_PaCO2.clone(), &v_PECO2.clone())?, &v_PaCO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VD_VT"), v_f.clone()), (Py::str("VD"), py::mul(&v_f.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cardiac_index(mut v_CO: Py, mut v_BSA: Py, mut v_HR: Py) -> R {
    let mut v_ci: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1028 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t1028[0].clone();
    v_BSA = t1028[1].clone();
    v_ci = py::truediv(&v_CO.clone(), &v_BSA.clone())?;
    v_out = Py::dict(vec![(Py::str("CI"), v_ci.clone())]);
    if ({ let t1029 = v_HR.clone(); Py::Bool({ let t1030 = Py::None; if !py::is(&t1029, &t1030) { true } else { false } }) }).truthy() {
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
    let t1082 = (Py::tuple(vec![f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_SV.clone(), Py::str("SV"))?])).items()?;
    if t1082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HR = t1082[0].clone();
    v_SV = t1082[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&py::mul(&v_HR.clone(), &v_SV.clone())?, &Py::Float(1000.0))?), (Py::str("CO_mL"), py::mul(&v_HR.clone(), &v_SV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chord_conductance(mut v_gK: Py, mut v_gNa: Py, mut v_gCl: Py, mut v_EK: Py, mut v_ENa: Py, mut v_ECl: Py) -> R {
    let mut v_g: Py = Py::None;
    let t1134 = (Py::tuple(vec![f__nonneg(v_gK.clone(), Py::str("gK"))?, f__nonneg(v_gNa.clone(), Py::str("gNa"))?, f__nonneg(v_gCl.clone(), Py::str("gCl"))?])).items()?;
    if t1134.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_gK = t1134[0].clone();
    v_gNa = t1134[1].clone();
    v_gCl = t1134[2].clone();
    let t1135 = (Py::tuple(vec![f__num(v_EK.clone(), Py::str("EK"))?, f__num(v_ENa.clone(), Py::str("ENa"))?, f__num(v_ECl.clone(), Py::str("ECl"))?])).items()?;
    if t1135.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_EK = t1135[0].clone();
    v_ENa = t1135[1].clone();
    v_ECl = t1135[2].clone();
    v_g = py::add(&py::add(&v_gK.clone(), &v_gNa.clone())?, &v_gCl.clone())?;
    let _ = f__need({ let t1136 = v_g.clone(); Py::Bool({ let t1137 = Py::Int(0i64); if py::gt(&t1136, &t1137)? { true } else { false } }) }, Py::str("at least one conductance must be positive"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Vm"), py::truediv(&py::add(&py::add(&py::mul(&v_gK.clone(), &v_EK.clone())?, &py::mul(&v_gNa.clone(), &v_ENa.clone())?)?, &py::mul(&v_gCl.clone(), &v_ECl.clone())?)?, &v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cockcroft_gault(mut v_age: Py, mut v_weight: Py, mut v_Scr: Py, mut v_sex: Py) -> R {
    let mut v_fem: Py = Py::None;
    let t1189 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_Scr.clone(), Py::str("Scr"))?])).items()?;
    if t1189.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t1189[0].clone();
    v_weight = t1189[1].clone();
    v_Scr = t1189[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1194 = Py::Int(18i64); Py::Bool({ let t1195 = v_age.clone(); if py::le(&t1194, &t1195)? { { let t1196 = Py::Int(140i64); if py::lt(&t1195, &t1196)? { true } else { false } } } else { false } }) }, Py::str("age must be at least 18 and below 140"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CrCl"), py::mul(&py::truediv(&py::mul(&py::sub(&Py::Float(140.0), &v_age.clone())?, &v_weight.clone())?, &py::mul(&Py::Float(72.0), &v_Scr.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.85) } else { Py::Float(1.0) }))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_compliance(mut v_dV: Py, mut v_dP: Py) -> R {
    let t5111 = (Py::tuple(vec![f__pos(v_dV.clone(), Py::str("dV"))?, f__pos(v_dP.clone(), Py::str("dP"))?])).items()?;
    if t5111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dV = t5111[0].clone();
    v_dP = t5111[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&v_dV.clone(), &v_dP.clone())?), (Py::str("E"), py::truediv(&v_dP.clone(), &v_dV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conduction_velocity_scaling(mut v_v1: Py, mut v_d1: Py, mut v_d2: Py, mut v_fiber: Py) -> R {
    let mut v_k: Py = Py::None;
    let t1248 = (Py::tuple(vec![f__pos(v_v1.clone(), Py::str("v1"))?, f__pos(v_d1.clone(), Py::str("d1"))?, f__pos(v_d2.clone(), Py::str("d2"))?])).items()?;
    if t1248.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_v1 = t1248[0].clone();
    v_d1 = t1248[1].clone();
    v_d2 = t1248[2].clone();
    let _ = f__need({ let t1249 = v_fiber.clone(); Py::Bool({ let t1250 = Py::tuple(vec![Py::str("myelinated"), Py::str("unmyelinated")]); if py::contains(&t1250, &t1249)? { true } else { false } }) }, Py::str("fiber must be myelinated or unmyelinated"))?;
    v_k = (if ({ let t1251 = v_fiber.clone(); Py::Bool({ let t1252 = Py::str("myelinated"); if py::eq(&t1251, &t1252) { true } else { false } }) }).truthy() { py::truediv(&v_d2.clone(), &v_d1.clone())? } else { py::math1("sqrt", &py::truediv(&v_d2.clone(), &v_d1.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("v2"), py::mul(&v_v1.clone(), &v_k.clone())?), (Py::str("ratio"), v_k.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_creatinine_clearance_timed(mut v_Ucr: Py, mut v_Vurine: Py, mut v_hours: Py, mut v_Pcr: Py, mut v_BSA: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1304 = (Py::tuple(vec![f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Vurine.clone(), Py::str("Vurine"))?, f__pos(v_hours.clone(), Py::str("hours"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?, f__pos(v_BSA.clone(), Py::str("BSA"))?])).items()?;
    if t1304.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ucr = t1304[0].clone();
    v_Vurine = t1304[1].clone();
    v_hours = t1304[2].clone();
    v_Pcr = t1304[3].clone();
    v_BSA = t1304[4].clone();
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
    for t1352 in py::iter(&(Py::tuple(vec![v_RR.clone(), v_small_boxes.clone(), v_large_boxes.clone()])))? {
        v_v = t1352;
        if ({ let t1353 = v_v.clone(); Py::Bool({ let t1354 = Py::None; if !py::is(&t1353, &t1354) { true } else { false } }) }).truthy() {
            v_given = py::iadd(&v_given, &Py::Int(1i64))?;
        }
    }
    let _ = f__need({ let t1355 = v_given.clone(); Py::Bool({ let t1356 = Py::Int(1i64); if py::eq(&t1355, &t1356) { true } else { false } }) }, Py::str("give exactly one of the R-R interval RR, the number of small boxes or the number of large boxes"))?;
    v_sp = (if ({ let t1357 = v_speed.clone(); Py::Bool({ let t1358 = Py::None; if py::is(&t1357, &t1358) { true } else { false } }) }).truthy() { Py::Float(25.0) } else { f__pos(v_speed.clone(), Py::str("speed"))? });
    if ({ let t1363 = v_RR.clone(); Py::Bool({ let t1364 = Py::None; if !py::is(&t1363, &t1364) { true } else { false } }) }).truthy() {
        v_rr = f__pos(v_RR.clone(), Py::str("RR"))?;
    } else {
        if ({ let t1365 = v_small_boxes.clone(); Py::Bool({ let t1366 = Py::None; if !py::is(&t1365, &t1366) { true } else { false } }) }).truthy() {
            v_rr = py::truediv(&py::mul(&f__pos(v_small_boxes.clone(), Py::str("small_boxes"))?, &Py::Float(1.0))?, &v_sp.clone())?;
        } else {
            v_rr = py::truediv(&py::mul(&f__pos(v_large_boxes.clone(), Py::str("large_boxes"))?, &Py::Float(5.0))?, &v_sp.clone())?;
        }
    }
    v_hr = py::truediv(&Py::Float(60.0), &v_rr.clone())?;
    let _ = f__need({ let t1367 = Py::Float(20.0); Py::Bool({ let t1368 = v_hr.clone(); if py::le(&t1367, &t1368)? { { let t1369 = Py::Float(300.0); if py::le(&t1368, &t1369)? { true } else { false } } } else { false } }) }, Py::str("the heart rate must lie between 20 and 300 beats/min"))?;
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
    let t1420 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1420[0].clone();
    v_age = t1420[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1425 = Py::Int(18i64); Py::Bool({ let t1426 = v_age.clone(); if py::le(&t1425, &t1426)? { { let t1427 = Py::Int(120i64); if py::le(&t1426, &t1427)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1428 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.241))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.302))?]) })).items()?;
    if t1428.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1428[0].clone();
    v_a = t1428[1].clone();
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
    let t1473 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__pos(v_Scys.clone(), Py::str("Scys"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1473.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1473[0].clone();
    v_Scys = t1473[1].clone();
    v_age = t1473[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1478 = Py::Int(18i64); Py::Bool({ let t1479 = v_age.clone(); if py::le(&t1478, &t1479)? { { let t1480 = Py::Int(120i64); if py::le(&t1479, &t1480)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1481 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.219))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.144))?]) })).items()?;
    if t1481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1481[0].clone();
    v_a = t1481[1].clone();
    let t1482 = (Py::tuple(vec![py::truediv(&v_Scr.clone(), &v_k.clone())?, py::truediv(&v_Scys.clone(), &Py::Float(0.8))?])).items()?;
    if t1482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1482[0].clone();
    v_c = t1482[1].clone();
    v_v = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(135.0), &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], false)?, &v_a.clone())?)?, &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.544))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], false)?, &py::neg(&Py::Float(0.323))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.778))?)?)?, &py::pow(&Py::Float(0.9961), &v_age.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.963) } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eGFR"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elimination_half_life(mut v_CL: Py, mut v_Vd: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1534 = (Py::tuple(vec![f__pos(v_CL.clone(), Py::str("CL"))?, f__pos(v_Vd.clone(), Py::str("Vd"))?])).items()?;
    if t1534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CL = t1534[0].clone();
    v_Vd = t1534[1].clone();
    v_k = py::truediv(&v_CL.clone(), &v_Vd.clone())?;
    v_out = Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("half_life"), py::truediv(&py::math1("log", &Py::Float(2.0))?, &v_k.clone())?)]);
    if ({ let t1535 = v_t.clone(); Py::Bool({ let t1536 = Py::None; if !py::is(&t1535, &t1536) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction_remaining"), py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &f__nonneg(v_t.clone(), Py::str("t"))?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fev1_fvc(mut v_FEV1: Py, mut v_FVC: Py) -> R {
    let t1590 = (Py::tuple(vec![f__pos(v_FEV1.clone(), Py::str("FEV1"))?, f__pos(v_FVC.clone(), Py::str("FVC"))?])).items()?;
    if t1590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FEV1 = t1590[0].clone();
    v_FVC = t1590[1].clone();
    let _ = f__need({ let t1591 = v_FEV1.clone(); Py::Bool({ let t1592 = v_FVC.clone(); if py::le(&t1591, &t1592)? { true } else { false } }) }, Py::str("FEV1 cannot exceed FVC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("ratio"), py::truediv(&v_FEV1.clone(), &v_FVC.clone())?), (Py::str("percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_FEV1.clone())?, &v_FVC.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_cardiac_output(mut v_VO2: Py, mut v_CaO2: Py, mut v_CvO2: Py) -> R {
    let mut v_d: Py = Py::None;
    let t1646 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t1646.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1646[0].clone();
    v_CaO2 = t1646[1].clone();
    v_CvO2 = t1646[2].clone();
    let _ = f__need({ let t1647 = v_CvO2.clone(); Py::Bool({ let t1648 = v_CaO2.clone(); if py::lt(&t1647, &t1648)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    v_d = py::sub(&v_CaO2.clone(), &v_CvO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&v_d.clone(), &Py::Float(10.0))?)?), (Py::str("avDO2"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_diffusion(mut v_D: Py, mut v_A: Py, mut v_dC: Py, mut v_T: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1700 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_A.clone(), Py::str("A"))?, f__num(v_dC.clone(), Py::str("dC"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t1700.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1700[0].clone();
    v_A = t1700[1].clone();
    v_dC = t1700[2].clone();
    v_T = t1700[3].clone();
    v_x = py::mul(&v_T.clone(), &Py::Float(0.0001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("J"), py::truediv(&py::mul(&py::mul(&v_D.clone(), &v_A.clone())?, &py::mul(&v_dC.clone(), &Py::Float(0.001))?)?, &v_x.clone())?), (Py::str("t_diffusion"), py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&Py::Float(2.0), &v_D.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_saturations(mut v_VO2: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t1754 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t1754.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1754[0].clone();
    v_Hb = t1754[1].clone();
    v_PaO2 = t1754[2].clone();
    v_PvO2 = t1754[3].clone();
    let t1764 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t1764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t1764[0].clone();
    v_SvO2 = t1764[1].clone();
    let t1765 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t1765.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t1765[0].clone();
    v_cv = t1765[1].clone();
    let _ = f__need({ let t1766 = v_cv.clone(); Py::Bool({ let t1767 = v_ca.clone(); if py::lt(&t1766, &t1767)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&py::sub(&v_ca.clone(), &v_cv.clone())?, &Py::Float(10.0))?)?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_fraction(mut v_GFR: Py, mut v_RPF: Py) -> R {
    let t1819 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__pos(v_RPF.clone(), Py::str("RPF"))?])).items()?;
    if t1819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t1819[0].clone();
    v_RPF = t1819[1].clone();
    let _ = f__need({ let t1820 = v_GFR.clone(); Py::Bool({ let t1821 = v_RPF.clone(); if py::lt(&t1820, &t1821)? { true } else { false } }) }, Py::str("GFR must be smaller than the renal plasma flow"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FF"), py::truediv(&v_GFR.clone(), &v_RPF.clone())?), (Py::str("FF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_GFR.clone())?, &v_RPF.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fluid_compartments(mut v_weight: Py, mut v_TBW_fraction: Py) -> R {
    let mut v_ecf: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_tbw: Py = Py::None;
    let t868 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__fr(v_TBW_fraction.clone(), Py::str("TBW_fraction"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t868.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_weight = t868[0].clone();
    v_f = t868[1].clone();
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
    let t1875 = (Py::tuple(vec![f__pos(v_TBW.clone(), Py::str("TBW"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?, f__pos(v_V_added.clone(), Py::str("V_added"))?, f__nonneg(v_C_added.clone(), Py::str("C_added"))?])).items()?;
    if t1875.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TBW = t1875[0].clone();
    v_Posm = t1875[1].clone();
    v_V = t1875[2].clone();
    v_C = t1875[3].clone();
    v_f = f__fr(v_ICF_fraction.clone(), Py::str("ICF_fraction"), Py::Bool(true), Py::Bool(true))?;
    let t1885 = (Py::tuple(vec![py::mul(&v_f.clone(), &v_TBW.clone())?, py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &v_TBW.clone())?])).items()?;
    if t1885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_icf0 = t1885[0].clone();
    v_ecf0 = t1885[1].clone();
    v_osm = py::truediv(&py::add(&py::mul(&v_TBW.clone(), &v_Posm.clone())?, &py::mul(&v_V.clone(), &v_C.clone())?)?, &py::add(&v_TBW.clone(), &v_V.clone())?)?;
    v_icf = py::truediv(&py::mul(&v_icf0.clone(), &v_Posm.clone())?, &v_osm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("osm_final"), v_osm.clone()), (Py::str("ICF"), v_icf.clone()), (Py::str("ECF"), py::sub(&py::add(&v_TBW.clone(), &v_V.clone())?, &v_icf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fractional_excretion(mut v_Ux: Py, mut v_Px: Py, mut v_Ucr: Py, mut v_Pcr: Py) -> R {
    let mut v_fe: Py = Py::None;
    let t1938 = (Py::tuple(vec![f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_Px.clone(), Py::str("Px"))?, f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?])).items()?;
    if t1938.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Ux = t1938[0].clone();
    v_Px = t1938[1].clone();
    v_Ucr = t1938[2].clone();
    v_Pcr = t1938[3].clone();
    v_fe = py::truediv(&py::mul(&v_Ux.clone(), &v_Pcr.clone())?, &py::mul(&v_Px.clone(), &v_Ucr.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FE"), v_fe.clone()), (Py::str("FE_percent"), py::mul(&Py::Float(100.0), &v_fe.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_water_clearance(mut v_V: Py, mut v_Uosm: Py, mut v_Posm: Py) -> R {
    let mut v_cosm: Py = Py::None;
    let t1983 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_Uosm.clone(), Py::str("Uosm"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?])).items()?;
    if t1983.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1983[0].clone();
    v_Uosm = t1983[1].clone();
    v_Posm = t1983[2].clone();
    v_cosm = py::truediv(&py::mul(&v_Uosm.clone(), &v_V.clone())?, &v_Posm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Cosm"), v_cosm.clone()), (Py::str("CH2O"), py::sub(&v_V.clone(), &v_cosm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_volume_conversion(mut v_V: Py, mut v_T: Py, mut v_Patm: Py, mut v_PH2O: Py) -> R {
    let mut v_tk: Py = Py::None;
    let t2037 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__num(v_T.clone(), Py::str("T"))?, f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?])).items()?;
    if t2037.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t2037[0].clone();
    v_T = t2037[1].clone();
    v_Patm = t2037[2].clone();
    v_PH2O = t2037[3].clone();
    let _ = f__need({ let t2046 = { let t2038 = py::neg(&Py::Int(50i64))?; Py::Bool({ let t2039 = v_T.clone(); if py::lt(&t2038, &t2039)? { { let t2040 = Py::Int(60i64); if py::lt(&t2039, &t2040)? { true } else { false } } } else { false } }) }; if !t2046.truthy() { t2046 } else { { let t2045 = { let t2041 = v_Patm.clone(); Py::Bool({ let t2042 = v_PH2O.clone(); if py::gt(&t2041, &t2042)? { true } else { false } }) }; if !t2045.truthy() { t2045 } else { { let t2043 = v_Patm.clone(); Py::Bool({ let t2044 = Py::Int(47i64); if py::gt(&t2043, &t2044)? { true } else { false } }) } } } } }, Py::str("temperature -50..60 degC, Patm above the vapour pressures"))?;
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
    let t2100 = (Py::tuple(vec![f__pos(v_PK.clone(), Py::str("PK"))?, f__nonneg(v_PNa.clone(), Py::str("PNa"))?, f__nonneg(v_PCl.clone(), Py::str("PCl"))?])).items()?;
    if t2100.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PK = t2100[0].clone();
    v_PNa = t2100[1].clone();
    v_PCl = t2100[2].clone();
    v_vals = { let mut out = vec![]; for t2101 in py::iter(&(Py::tuple(vec![v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone()])))? { let mut v_x: Py; 
v_x = t2101;
out.push(f__pos(v_x.clone(), Py::str("concentrations"))?); } Py::list(out) };
    let t2102 = (v_vals.clone()).items()?;
    if t2102.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_K_o = t2102[0].clone();
    v_K_i = t2102[1].clone();
    v_Na_o = t2102[2].clone();
    v_Na_i = t2102[3].clone();
    v_Cl_o = t2102[4].clone();
    v_Cl_i = t2102[5].clone();
    v_T = f__num(v_T.clone(), Py::str("T"))?;
    let _ = f__need({ let t2103 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2104 = v_T.clone(); if py::lt(&t2103, &t2104)? { { let t2105 = Py::Int(100i64); if py::lt(&t2104, &t2105)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2157 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("A"))?, f__pos(v_c.clone(), Py::str("c"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t2157.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2157[0].clone();
    v_c = t2157[1].clone();
    v_T = t2157[2].clone();
    let _ = f__need({ let t2158 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2159 = v_T.clone(); if py::lt(&t2158, &t2159)? { { let t2160 = Py::Int(100i64); if py::lt(&t2159, &t2160)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2214 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Pglucose.clone(), Py::str("Pglucose"))?, f__pos(v_Tm.clone(), Py::str("Tm"))?])).items()?;
    if t2214.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t2214[0].clone();
    v_P = t2214[1].clone();
    v_Tm = t2214[2].clone();
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
    let t2274 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_HR_rest.clone(), Py::str("HR_rest"))?, f__fr(v_intensity.clone(), Py::str("intensity"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2274.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2274[0].clone();
    v_r = t2274[1].clone();
    v_f = t2274[2].clone();
    let _ = f__need({ let t2275 = Py::Int(5i64); Py::Bool({ let t2276 = v_A.clone(); if py::le(&t2275, &t2276)? { { let t2277 = Py::Int(100i64); if py::le(&t2276, &t2277)? { true } else { false } } } else { false } }) }, Py::str("age must be between 5 and 100"))?;
    v_mx = py::sub(&Py::Float(220.0), &v_A.clone())?;
    let _ = f__need({ let t2278 = v_r.clone(); Py::Bool({ let t2279 = v_mx.clone(); if py::lt(&t2278, &t2279)? { true } else { false } }) }, Py::str("the resting heart rate must be below the estimated maximum"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HRmax"), v_mx.clone()), (Py::str("HRmax_Tanaka"), py::sub(&Py::Float(208.0), &py::mul(&Py::Float(0.7), &v_A.clone())?)?), (Py::str("HRR"), py::sub(&v_mx.clone(), &v_r.clone())?), (Py::str("target"), py::add(&v_r.clone(), &py::mul(&v_f.clone(), &py::sub(&v_mx.clone(), &v_r.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_fit(mut v_P1: Py, mut v_S1: Py, mut v_P2: Py, mut v_S2: Py) -> R {
    let mut v_n: Py = Py::None;
    let t2324 = (Py::tuple(vec![f__pos(v_P1.clone(), Py::str("P1"))?, f__pos(v_P2.clone(), Py::str("P2"))?])).items()?;
    if t2324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2324[0].clone();
    v_P2 = t2324[1].clone();
    let t2334 = (Py::tuple(vec![f__fr(v_S1.clone(), Py::str("S1"), Py::Bool(true), Py::Bool(true))?, f__fr(v_S2.clone(), Py::str("S2"), Py::Bool(true), Py::Bool(true))?])).items()?;
    if t2334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S1 = t2334[0].clone();
    v_S2 = t2334[1].clone();
    let _ = f__need({ let t2339 = { let t2335 = v_P1.clone(); Py::Bool({ let t2336 = v_P2.clone(); if py::ne(&t2335, &t2336) { true } else { false } }) }; if !t2339.truthy() { t2339 } else { { let t2337 = v_S1.clone(); Py::Bool({ let t2338 = v_S2.clone(); if py::ne(&t2337, &t2338) { true } else { false } }) } } }, Py::str("the two points must differ"))?;
    let _ = f__need({ let t2340 = py::mul(&py::sub(&v_P2.clone(), &v_P1.clone())?, &py::sub(&v_S2.clone(), &v_S1.clone())?)?; Py::Bool({ let t2341 = Py::Int(0i64); if py::gt(&t2340, &t2341)? { true } else { false } }) }, Py::str("saturation must increase with PO2"))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&py::truediv(&v_S2.clone(), &py::sub(&Py::Int(1i64), &v_S2.clone())?)?, &py::truediv(&v_S1.clone(), &py::sub(&Py::Int(1i64), &v_S1.clone())?)?)?)?, &py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("P50"), py::mul(&v_P1.clone(), &py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_S1.clone())?, &v_S1.clone())?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_force_velocity(mut v_F0: Py, mut v_a: Py, mut v_b: Py, mut v_v: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let t2395 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__nonneg(v_v.clone(), Py::str("v"))?])).items()?;
    if t2395.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t2395[0].clone();
    v_a = t2395[1].clone();
    v_b = t2395[2].clone();
    v_v = t2395[3].clone();
    v_vmax = py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?;
    let _ = f__need({ let t2396 = v_v.clone(); Py::Bool({ let t2397 = v_vmax.clone(); if py::le(&t2396, &t2397)? { true } else { false } }) }, Py::str("the shortening velocity cannot exceed vmax = b F0/a"))?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("power"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), v_vmax.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_po2(mut v_SO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let t2458 = (Py::tuple(vec![f__fr(v_SO2.clone(), Py::str("SO2"), Py::Bool(true), Py::Bool(true))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2458.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_SO2 = t2458[0].clone();
    v_P50 = t2458[1].clone();
    v_n = t2458[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2"), py::mul(&v_P50.clone(), &py::pow(&py::truediv(&v_SO2.clone(), &py::sub(&Py::Float(1.0), &v_SO2.clone())?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_saturation(mut v_PO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let mut v_x: Py = Py::None;
    let t2512 = (Py::tuple(vec![f__nonneg(v_PO2.clone(), Py::str("PO2"))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2512.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t2512[0].clone();
    v_P50 = t2512[1].clone();
    v_n = t2512[2].clone();
    v_x = py::pow(&py::truediv(&v_PO2.clone(), &v_P50.clone())?, &v_n.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SO2"), py::truediv(&v_x.clone(), &py::add(&Py::Float(1.0), &v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrostatic_column(mut v_h: Py, mut v_rho: Py) -> R {
    let mut v_p: Py = Py::None;
    let t2564 = (Py::tuple(vec![f__num(v_h.clone(), Py::str("h"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t2564.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2564[0].clone();
    v_rho = t2564[1].clone();
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
    let t2618 = (Py::tuple(vec![f__pos(v_mass_air.clone(), Py::str("mass_air"))?, f__num(v_mass_water.clone(), Py::str("mass_water"))?, f__pos(v_water_density.clone(), Py::str("water_density"))?, f__nonneg(v_RV.clone(), Py::str("RV"))?, f__nonneg(v_GI.clone(), Py::str("GI"))?])).items()?;
    if t2618.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ma = t2618[0].clone();
    v_Mw = t2618[1].clone();
    v_rho = t2618[2].clone();
    v_RV = t2618[3].clone();
    v_GI = t2618[4].clone();
    let _ = f__need({ let t2624 = { let t2619 = v_Mw.clone(); Py::Bool({ let t2620 = v_Ma.clone(); if py::lt(&t2619, &t2620)? { true } else { false } }) }; if !t2624.truthy() { t2624 } else { { let t2621 = Py::Float(0.95); Py::Bool({ let t2622 = v_rho.clone(); if py::lt(&t2621, &t2622)? { { let t2623 = Py::Float(1.0); if py::le(&t2622, &t2623)? { true } else { false } } } else { false } }) } } }, Py::str("underwater mass below the mass in air; water density 0.95-1.0 g/mL"))?;
    v_V = py::sub(&py::sub(&py::truediv(&py::sub(&v_Ma.clone(), &v_Mw.clone())?, &v_rho.clone())?, &v_RV.clone())?, &v_GI.clone())?;
    let _ = f__need({ let t2625 = v_V.clone(); Py::Bool({ let t2626 = Py::Int(0i64); if py::gt(&t2625, &t2626)? { true } else { false } }) }, Py::str("the computed body volume is not positive"))?;
    v_D = py::truediv(&v_Ma.clone(), &v_V.clone())?;
    let _ = f__need({ let t2627 = Py::Float(0.9); Py::Bool({ let t2628 = v_D.clone(); if py::lt(&t2627, &t2628)? { { let t2629 = Py::Float(1.2); if py::lt(&t2628, &t2629)? { true } else { false } } } else { false } }) }, Py::str("the computed density is outside 0.9-1.2 g/mL"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("volume"), v_V.clone()), (Py::str("density"), v_D.clone()), (Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ionic_current(mut v_g: Py, mut v_Vm: Py, mut v_E: Py) -> R {
    let t2681 = (Py::tuple(vec![f__nonneg(v_g.clone(), Py::str("g"))?, f__num(v_Vm.clone(), Py::str("Vm"))?, f__num(v_E.clone(), Py::str("E"))?])).items()?;
    if t2681.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2681[0].clone();
    v_Vm = t2681[1].clone();
    v_E = t2681[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I"), py::mul(&v_g.clone(), &py::sub(&v_Vm.clone(), &v_E.clone())?)?), (Py::str("driving_force"), py::sub(&v_Vm.clone(), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_wall_tension(mut v_P: Py, mut v_r: Py, mut v_shape: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2733 = (Py::tuple(vec![f__pos(v_P.clone(), Py::str("P"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t2733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2733[0].clone();
    v_r = t2733[1].clone();
    let _ = f__need({ let t2734 = v_shape.clone(); Py::Bool({ let t2735 = Py::tuple(vec![Py::str("cylinder"), Py::str("sphere")]); if py::contains(&t2735, &t2734)? { true } else { false } }) }, Py::str("shape must be cylinder or sphere"))?;
    v_T = py::mul(&py::truediv(&py::mul(&py::mul(&v_P.clone(), &c_MMHG()?)?, &v_r.clone())?, &Py::Float(100.0))?, &(if ({ let t2736 = v_shape.clone(); Py::Bool({ let t2737 = Py::str("cylinder"); if py::eq(&t2736, &t2737) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.5) }))?;
    v_out = Py::dict(vec![(Py::str("T"), v_T.clone())]);
    if ({ let t2738 = v_h.clone(); Py::Bool({ let t2739 = Py::None; if !py::is(&t2738, &t2739) { true } else { false } }) }).truthy() {
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
    let t2791 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t2791[0].clone();
    v_H = t2791[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::mul(&Py::Float(0.252), &v_W.clone())?, &py::mul(&Py::Float(0.473), &v_H.clone())?)?, &Py::Float(48.3))? } else { py::sub(&py::add(&py::mul(&Py::Float(0.407), &v_W.clone())?, &py::mul(&Py::Float(0.267), &v_H.clone())?)?, &Py::Float(19.2))? });
    let _ = f__need({ let t2796 = Py::Int(0i64); Py::Bool({ let t2797 = v_v.clone(); if py::lt(&t2796, &t2797)? { { let t2798 = v_W.clone(); if py::lt(&t2797, &t2798)? { true } else { false } } } else { false } }) }, Py::str("the formula gives no valid lean mass for these values"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("LBM"), v_v.clone()), (Py::str("fat_mass"), py::sub(&v_W.clone(), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_length_constant(mut v_Rm: Py, mut v_Ri: Py, mut v_d: Py, mut v_x: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2850 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Ri.clone(), Py::str("Ri"))?, f__pos(v_d.clone(), Py::str("d"))?])).items()?;
    if t2850.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t2850[0].clone();
    v_Ri = t2850[1].clone();
    v_d = t2850[2].clone();
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&v_Rm.clone(), &py::mul(&v_d.clone(), &Py::Float(0.0001))?)?, &py::mul(&Py::Float(4.0), &v_Ri.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("lambda"), py::mul(&v_lam.clone(), &Py::Float(10.0))?)]);
    if ({ let t2851 = v_x.clone(); Py::Bool({ let t2852 = Py::None; if !py::is(&t2851, &t2852) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("attenuation"), py::math1("exp", &py::truediv(&py::neg(&py::truediv(&f__nonneg(v_x.clone(), Py::str("x"))?, &Py::Float(10.0))?)?, &v_lam.clone())?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lung_volumes(mut v_TV: Py, mut v_IRV: Py, mut v_ERV: Py, mut v_RV: Py) -> R {
    let t2908 = (Py::tuple(vec![f__pos(v_TV.clone(), Py::str("TV"))?, f__nonneg(v_IRV.clone(), Py::str("IRV"))?, f__nonneg(v_ERV.clone(), Py::str("ERV"))?, f__pos(v_RV.clone(), Py::str("RV"))?])).items()?;
    if t2908.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TV = t2908[0].clone();
    v_IRV = t2908[1].clone();
    v_ERV = t2908[2].clone();
    v_RV = t2908[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("IC"), py::add(&v_TV.clone(), &v_IRV.clone())?), (Py::str("FRC"), py::add(&v_ERV.clone(), &v_RV.clone())?), (Py::str("VC"), py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?), (Py::str("TLC"), py::add(&py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?, &v_RV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_arterial_pressure(mut v_SBP: Py, mut v_DBP: Py) -> R {
    let t2960 = (Py::tuple(vec![f__pos(v_SBP.clone(), Py::str("SBP"))?, f__pos(v_DBP.clone(), Py::str("DBP"))?])).items()?;
    if t2960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SBP = t2960[0].clone();
    v_DBP = t2960[1].clone();
    let _ = f__need({ let t2961 = v_SBP.clone(); Py::Bool({ let t2962 = v_DBP.clone(); if py::gt(&t2961, &t2962)? { true } else { false } }) }, Py::str("the systolic pressure must exceed the diastolic"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("MAP"), py::add(&v_DBP.clone(), &py::truediv(&py::sub(&v_SBP.clone(), &v_DBP.clone())?, &Py::Float(3.0))?)?), (Py::str("PP"), py::sub(&v_SBP.clone(), &v_DBP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_charge(mut v_Cm: Py, mut v_dV: Py, mut v_area: Py) -> R {
    let mut v_q: Py = Py::None;
    let t3014 = (Py::tuple(vec![f__pos(v_Cm.clone(), Py::str("Cm"))?, f__num(v_dV.clone(), Py::str("dV"))?, f__pos(v_area.clone(), Py::str("area"))?])).items()?;
    if t3014.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cm = t3014[0].clone();
    v_dV = t3014[1].clone();
    v_area = t3014[2].clone();
    let _ = f__need({ let t3015 = v_dV.clone(); Py::Bool({ let t3016 = Py::Int(0i64); if py::ne(&t3015, &t3016) { true } else { false } }) }, Py::str("the voltage change must not be zero"))?;
    v_q = py::mul(&py::mul(&py::mul(&v_Cm.clone(), &Py::Float(1e-06))?, &py::mul(&v_area.clone(), &Py::Float(1e-08))?)?, &py::mul(&v_dV.clone(), &Py::Float(0.001))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q_pC"), py::mul(&v_q.clone(), &Py::Float(1000000000000.0))?), (Py::str("ions"), py::truediv(&py::abs(&v_q.clone())?, &c_E_CHARGE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_time_constant(mut v_Rm: Py, mut v_Cm: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t3068 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Cm.clone(), Py::str("Cm"))?])).items()?;
    if t3068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t3068[0].clone();
    v_Cm = t3068[1].clone();
    v_tau = py::mul(&py::mul(&v_Rm.clone(), &v_Cm.clone())?, &Py::Float(0.001))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone())]);
    if ({ let t3069 = v_t.clone(); Py::Bool({ let t3070 = Py::None; if !py::is(&t3069, &t3070) { true } else { false } }) }).truthy() {
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
    let t3124 = (Py::tuple(vec![f__pos(v_MET.clone(), Py::str("MET"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_duration.clone(), Py::str("duration"))?])).items()?;
    if t3124.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MET = t3124[0].clone();
    v_W = t3124[1].clone();
    v_m = t3124[2].clone();
    v_vo2 = py::mul(&py::mul(&v_MET.clone(), &Py::Float(3.5))?, &v_W.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), v_vo2.clone()), (Py::str("kcal_per_min"), py::truediv(&v_vo2.clone(), &Py::Float(200.0))?), (Py::str("kcal"), py::mul(&py::truediv(&v_vo2.clone(), &Py::Float(200.0))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minute_ventilation(mut v_VT: Py, mut v_RR: Py, mut v_VD: Py) -> R {
    let t3178 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__pos(v_RR.clone(), Py::str("RR"))?, f__nonneg(v_VD.clone(), Py::str("VD"))?])).items()?;
    if t3178.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t3178[0].clone();
    v_RR = t3178[1].clone();
    v_VD = t3178[2].clone();
    let _ = f__need({ let t3179 = v_VD.clone(); Py::Bool({ let t3180 = v_VT.clone(); if py::lt(&t3179, &t3180)? { true } else { false } }) }, Py::str("the dead space must be smaller than the tidal volume"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VE"), py::truediv(&py::mul(&v_VT.clone(), &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VA"), py::truediv(&py::mul(&py::sub(&v_VT.clone(), &v_VD.clone())?, &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VD_VT"), py::truediv(&v_VD.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moens_korteweg(mut v_E: Py, mut v_h: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let t3232 = (Py::tuple(vec![f__pos(v_E.clone(), Py::str("E"))?, f__pos(v_h.clone(), Py::str("h"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t3232.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3232[0].clone();
    v_h = t3232[1].clone();
    v_D = t3232[2].clone();
    v_rho = t3232[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PWV"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_E.clone(), &Py::Float(1000.0))?, &py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_muscle_max_power(mut v_F0: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3284 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?])).items()?;
    if t3284.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t3284[0].clone();
    v_a = t3284[1].clone();
    v_b = t3284[2].clone();
    v_v = py::mul(&v_b.clone(), &py::sub(&py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_F0.clone(), &v_a.clone())?)?)?, &Py::Float(1.0))?)?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("v_opt"), v_v.clone()), (Py::str("F_opt"), v_F.clone()), (Py::str("P_max"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst(mut v_z: Py, mut v_Cout: Py, mut v_Cin: Py, mut v_T: Py) -> R {
    v_z = py::int(&v_z.clone())?;
    let t3336 = (Py::tuple(vec![f__pos(v_Cout.clone(), Py::str("Cout"))?, f__pos(v_Cin.clone(), Py::str("Cin"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3336.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cout = t3336[0].clone();
    v_Cin = t3336[1].clone();
    v_T = t3336[2].clone();
    let _ = f__need({ let t3342 = { let t3337 = v_z.clone(); Py::Bool({ let t3338 = Py::Int(0i64); if py::ne(&t3337, &t3338) { true } else { false } }) }; if !t3342.truthy() { t3342 } else { { let t3339 = py::neg(&Py::Int(3i64))?; Py::Bool({ let t3340 = v_z.clone(); if py::le(&t3339, &t3340)? { { let t3341 = Py::Int(3i64); if py::le(&t3340, &t3341)? { true } else { false } } } else { false } }) } } }, Py::str("the valence must be a nonzero whole number from -3 to 3"))?;
    let _ = f__need({ let t3343 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3344 = v_T.clone(); if py::lt(&t3343, &t3344)? { { let t3345 = Py::Int(100i64); if py::lt(&t3344, &t3345)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("E"), py::mul(&py::truediv(&f__rtf(v_T.clone())?, &v_z.clone())?, &py::math1("log", &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?)?), (Py::str("RT_F"), f__rtf(v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nerve_conduction_velocity(mut v_distance: Py, mut v_latency_proximal: Py, mut v_latency_distal: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t3399 = (Py::tuple(vec![f__pos(v_distance.clone(), Py::str("distance"))?, f__pos(v_latency_proximal.clone(), Py::str("latency_proximal"))?, f__nonneg(v_latency_distal.clone(), Py::str("latency_distal"))?])).items()?;
    if t3399.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t3399[0].clone();
    v_lp = t3399[1].clone();
    v_ld = t3399[2].clone();
    let _ = f__need({ let t3400 = v_lp.clone(); Py::Bool({ let t3401 = v_ld.clone(); if py::gt(&t3400, &t3401)? { true } else { false } }) }, Py::str("the proximal latency must exceed the distal latency"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("NCV"), py::truediv(&v_D.clone(), &py::sub(&v_lp.clone(), &v_ld.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_network_resistance(mut v_R: Py, mut v_arrangement: Py) -> R {
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_R = f__vec(v_R.clone(), Py::str("R"), Py::Int(1i64), Py::Int(10i64))?;
    for t5216 in py::iter(&(v_R.clone()))? {
        v_x = t5216;
        let _ = f__need({ let t5217 = v_x.clone(); Py::Bool({ let t5218 = Py::Int(0i64); if py::gt(&t5217, &t5218)? { true } else { false } }) }, Py::str("every resistance must be positive"))?;
    }
    let _ = f__need({ let t5219 = v_arrangement.clone(); Py::Bool({ let t5220 = Py::tuple(vec![Py::str("series"), Py::str("parallel")]); if py::contains(&t5220, &t5219)? { true } else { false } }) }, Py::str("arrangement must be series or parallel"))?;
    v_tot = (if ({ let t5221 = v_arrangement.clone(); Py::Bool({ let t5222 = Py::str("series"); if py::eq(&t5221, &t5222) { true } else { false } }) }).truthy() { py::sum(&v_R.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t5223 in py::iter(&(v_R.clone()))? { let mut v_x: Py; 
v_x = t5223;
out.push(py::truediv(&Py::Float(1.0), &v_x.clone())?); } Py::list(out) }, None)?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("R_total"), v_tot.clone()), (Py::str("G_total"), py::truediv(&Py::Float(1.0), &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_osm: Py, mut v_T: Py) -> R {
    let mut v_p: Py = Py::None;
    let t3453 = (Py::tuple(vec![f__nonneg(v_osm.clone(), Py::str("osm"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_osm = t3453[0].clone();
    v_T = t3453[1].clone();
    let _ = f__need({ let t3454 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3455 = v_T.clone(); if py::lt(&t3454, &t3455)? { { let t3456 = Py::Int(100i64); if py::lt(&t3455, &t3456)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_p = py::mul(&py::mul(&v_osm.clone(), &c_R_GAS()?)?, &py::add(&v_T.clone(), &c_T0C()?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pi"), v_p.clone()), (Py::str("pi_mmHg"), py::truediv(&v_p.clone(), &c_MMHG()?)?), (Py::str("pi_atm"), py::truediv(&v_p.clone(), &Py::Float(101325.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_consumption(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t3510 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t3510.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3510[0].clone();
    v_Hb = t3510[1].clone();
    v_PaO2 = t3510[2].clone();
    v_PvO2 = t3510[3].clone();
    let t3520 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t3520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t3520[0].clone();
    v_SvO2 = t3520[1].clone();
    let t3521 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t3521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t3521[0].clone();
    v_cv = t3521[1].clone();
    let _ = f__need({ let t3522 = v_cv.clone(); Py::Bool({ let t3523 = v_ca.clone(); if py::lt(&t3522, &t3523)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), py::mul(&py::mul(&v_CO.clone(), &py::sub(&v_ca.clone(), &v_cv.clone())?)?, &Py::Float(10.0))?), (Py::str("O2ER"), py::truediv(&py::sub(&v_ca.clone(), &v_cv.clone())?, &v_ca.clone())?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_content(mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t3586 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3586.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t3586[0].clone();
    v_SaO2 = t3586[1].clone();
    v_PaO2 = t3586[2].clone();
    let t3587 = (Py::tuple(vec![py::mul(&py::mul(&c_O2_HB()?, &v_Hb.clone())?, &v_SaO2.clone())?, py::mul(&c_O2_SOL()?, &v_PaO2.clone())?])).items()?;
    if t3587.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3587[0].clone();
    v_d = t3587[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CaO2"), py::add(&v_b.clone(), &v_d.clone())?), (Py::str("bound"), v_b.clone()), (Py::str("dissolved"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_delivery(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_c: Py = Py::None;
    let t3650 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3650.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3650[0].clone();
    v_Hb = t3650[1].clone();
    v_SaO2 = t3650[2].clone();
    v_PaO2 = t3650[3].clone();
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
    let t3702 = (Py::tuple(vec![f__pos(v_U_PAH.clone(), Py::str("U_PAH"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P_PAH.clone(), Py::str("P_PAH"))?])).items()?;
    if t3702.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t3702[0].clone();
    v_V = t3702[1].clone();
    v_P = t3702[2].clone();
    let t3712 = (Py::tuple(vec![f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(false), Py::Bool(true))?, f__fr(v_E.clone(), Py::str("E"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t3712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hct = t3712[0].clone();
    v_E = t3712[1].clone();
    v_erpf = py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?;
    v_rpf = py::truediv(&v_erpf.clone(), &v_E.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eRPF"), v_erpf.clone()), (Py::str("RPF"), v_rpf.clone()), (Py::str("RBF"), py::truediv(&v_rpf.clone(), &py::sub(&Py::Float(1.0), &v_Hct.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pco2_from_ph(mut v_pH: Py, mut v_HCO3: Py) -> R {
    let t3764 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?])).items()?;
    if t3764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t3764[0].clone();
    v_HCO3 = t3764[1].clone();
    let _ = f__need({ let t3765 = Py::Float(6.5); Py::Bool({ let t3766 = v_pH.clone(); if py::le(&t3765, &t3766)? { { let t3767 = Py::Float(8.0); if py::le(&t3766, &t3767)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PaCO2"), py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_henderson(mut v_HCO3: Py, mut v_PaCO2: Py) -> R {
    let mut v_ph: Py = Py::None;
    let t3819 = (Py::tuple(vec![f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t3819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HCO3 = t3819[0].clone();
    v_PaCO2 = t3819[1].clone();
    v_ph = py::add(&c_PK_CO2()?, &py::math1("log10", &py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pH"), v_ph.clone()), (Py::str("H"), py::pow(&Py::Float(10.0), &py::sub(&Py::Float(9.0), &v_ph.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_osmolality(mut v_Na: Py, mut v_glucose: Py, mut v_BUN: Py, mut v_measured: Py) -> R {
    let mut v_calc: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t3873 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__nonneg(v_glucose.clone(), Py::str("glucose"))?, f__nonneg(v_BUN.clone(), Py::str("BUN"))?])).items()?;
    if t3873.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t3873[0].clone();
    v_glucose = t3873[1].clone();
    v_BUN = t3873[2].clone();
    v_calc = py::add(&py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?, &py::truediv(&v_BUN.clone(), &Py::Float(2.8))?)?;
    v_out = Py::dict(vec![(Py::str("calculated"), v_calc.clone()), (Py::str("effective"), py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?)]);
    if ({ let t3874 = v_measured.clone(); Py::Bool({ let t3875 = Py::None; if !py::is(&t3874, &t3875) { true } else { false } }) }).truthy() {
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
    let t3927 = (Py::tuple(vec![f__pos(v_dP.clone(), Py::str("dP"))?, f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3927.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dP = t3927[0].clone();
    v_eta = t3927[1].clone();
    v_L = t3927[2].clone();
    v_r = t3927[3].clone();
    v_Q = py::truediv(&py::mul(&v_dP.clone(), &c_MMHG()?)?, &f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?)?;
    v_a = py::mul(&v_r.clone(), &Py::Float(0.001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q"), py::mul(&v_Q.clone(), &Py::Float(1000000.0))?), (Py::str("Q_mL_min"), py::mul(&v_Q.clone(), &Py::Float(60000000.0))?), (Py::str("v_mean"), py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&c_PI()?, &v_a.clone())?, &v_a.clone())?)?, &Py::Float(100.0))?), (Py::str("tau_wall"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_eta.clone())?, &Py::Float(0.001))?, &v_Q.clone())?, &py::mul(&c_PI()?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poiseuille_resistance(mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    let mut v_R: Py = Py::None;
    let t3979 = (Py::tuple(vec![f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3979.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eta = t3979[0].clone();
    v_L = t3979[1].clone();
    v_r = t3979[2].clone();
    v_R = f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_mmHg"), py::truediv(&py::truediv(&py::mul(&v_R.clone(), &Py::Float(1e-06))?, &Py::Float(60.0))?, &c_MMHG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pulmonary_vascular_resistance(mut v_mPAP: Py, mut v_PCWP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4033 = (Py::tuple(vec![f__num(v_mPAP.clone(), Py::str("mPAP"))?, f__nonneg(v_PCWP.clone(), Py::str("PCWP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4033.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mPAP = t4033[0].clone();
    v_PCWP = t4033[1].clone();
    v_CO = t4033[2].clone();
    let _ = f__need({ let t4034 = v_mPAP.clone(); Py::Bool({ let t4035 = v_PCWP.clone(); if py::gt(&t4034, &t4035)? { true } else { false } }) }, Py::str("the mean pulmonary artery pressure must exceed the wedge pressure"))?;
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
    let t4093 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpvO2.clone(), Py::str("SpvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpaO2.clone(), Py::str("SpaO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4093.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Sa = t4093[0].clone();
    v_Sv = t4093[1].clone();
    v_Spv = t4093[2].clone();
    v_Spa = t4093[3].clone();
    let _ = f__need({ let t4098 = { let t4094 = v_Sa.clone(); Py::Bool({ let t4095 = v_Sv.clone(); if py::gt(&t4094, &t4095)? { true } else { false } }) }; if !t4098.truthy() { t4098 } else { { let t4096 = v_Spv.clone(); Py::Bool({ let t4097 = v_Spa.clone(); if py::gt(&t4096, &t4097)? { true } else { false } }) } } }, Py::str("need SaO2 > SvO2 and SpvO2 > SpaO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qp_Qs"), py::truediv(&py::sub(&v_Sa.clone(), &v_Sv.clone())?, &py::sub(&v_Spv.clone(), &v_Spa.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qrs_axis(mut v_lead_I: Py, mut v_aVF: Py) -> R {
    let t4140 = (Py::tuple(vec![f__num(v_lead_I.clone(), Py::str("lead_I"))?, f__num(v_aVF.clone(), Py::str("aVF"))?])).items()?;
    if t4140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lead_I = t4140[0].clone();
    v_aVF = t4140[1].clone();
    let _ = f__need({ let t4145 = { let t4141 = v_lead_I.clone(); Py::Bool({ let t4142 = Py::Int(0i64); if py::ne(&t4141, &t4142) { true } else { false } }) }; if t4145.truthy() { t4145 } else { { let t4143 = v_aVF.clone(); Py::Bool({ let t4144 = Py::Int(0i64); if py::ne(&t4143, &t4144) { true } else { false } }) } } }, Py::str("at least one net deflection must be nonzero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("axis"), py::math1("degrees", &py::atan2(&v_aVF.clone(), &v_lead_I.clone())?)?), (Py::str("amplitude"), py::hypot(&[v_lead_I.clone(), v_aVF.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qt_correction(mut v_QT: Py, mut v_HR: Py) -> R {
    let mut v_rr: Py = Py::None;
    let t4190 = (Py::tuple(vec![f__pos(v_QT.clone(), Py::str("QT"))?, f__pos(v_HR.clone(), Py::str("HR"))?])).items()?;
    if t4190.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_QT = t4190[0].clone();
    v_HR = t4190[1].clone();
    v_rr = py::truediv(&Py::Float(60.0), &v_HR.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RR"), v_rr.clone()), (Py::str("QTc_Bazett"), py::truediv(&v_QT.clone(), &py::math1("sqrt", &v_rr.clone())?)?), (Py::str("QTc_Fridericia"), py::truediv(&v_QT.clone(), &py::pow(&v_rr.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?), (Py::str("QTc_Framingham"), py::add(&v_QT.clone(), &py::mul(&Py::Float(154.0), &py::sub(&Py::Float(1.0), &v_rr.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radius_flow_ratio(mut v_r1: Py, mut v_r2: Py) -> R {
    let mut v_q: Py = Py::None;
    let t4241 = (Py::tuple(vec![f__pos(v_r1.clone(), Py::str("r1"))?, f__pos(v_r2.clone(), Py::str("r2"))?])).items()?;
    if t4241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t4241[0].clone();
    v_r2 = t4241[1].clone();
    v_q = py::pow(&py::truediv(&v_r2.clone(), &v_r1.clone())?, &Py::Int(4i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("flow_ratio"), v_q.clone()), (Py::str("resistance_ratio"), py::truediv(&Py::Float(1.0), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_red_cell_indices(mut v_Hct: Py, mut v_Hb: Py, mut v_RBC: Py) -> R {
    v_Hct = f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(true), Py::Bool(true))?;
    let t4294 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__pos(v_RBC.clone(), Py::str("RBC"))?])).items()?;
    if t4294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t4294[0].clone();
    v_RBC = t4294[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("MCV"), py::truediv(&py::mul(&v_Hct.clone(), &Py::Float(1000.0))?, &v_RBC.clone())?), (Py::str("MCH"), py::truediv(&py::mul(&v_Hb.clone(), &Py::Float(10.0))?, &v_RBC.clone())?), (Py::str("MCHC"), py::truediv(&v_Hb.clone(), &v_Hct.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_renal_clearance(mut v_U: Py, mut v_V: Py, mut v_P: Py) -> R {
    let t4341 = (Py::tuple(vec![f__nonneg(v_U.clone(), Py::str("U"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P.clone(), Py::str("P"))?])).items()?;
    if t4341.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4341[0].clone();
    v_V = t4341[1].clone();
    v_P = t4341[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?), (Py::str("excretion"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_mechanics(mut v_VT: Py, mut v_Ppeak: Py, mut v_Pplat: Py, mut v_PEEP: Py, mut v_flow: Py) -> R {
    let t4395 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__num(v_Ppeak.clone(), Py::str("Ppeak"))?, f__num(v_Pplat.clone(), Py::str("Pplat"))?, f__nonneg(v_PEEP.clone(), Py::str("PEEP"))?, f__pos(v_flow.clone(), Py::str("flow"))?])).items()?;
    if t4395.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t4395[0].clone();
    v_Ppeak = t4395[1].clone();
    v_Pplat = t4395[2].clone();
    v_PEEP = t4395[3].clone();
    v_flow = t4395[4].clone();
    let _ = f__need({ let t4396 = v_Ppeak.clone(); Py::Bool({ let t4397 = v_Pplat.clone(); if py::ge(&t4396, &t4397)? { { let t4398 = v_PEEP.clone(); if py::gt(&t4397, &t4398)? { true } else { false } } } else { false } }) }, Py::str("need Ppeak >= Pplat > PEEP"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("C_static"), py::truediv(&v_VT.clone(), &py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)?), (Py::str("C_dynamic"), py::truediv(&v_VT.clone(), &py::sub(&v_Ppeak.clone(), &v_PEEP.clone())?)?), (Py::str("R_aw"), py::truediv(&py::sub(&v_Ppeak.clone(), &v_Pplat.clone())?, &py::truediv(&v_flow.clone(), &Py::Float(60.0))?)?), (Py::str("driving_pressure"), py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_time_constant(mut v_R: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t4450 = (Py::tuple(vec![f__pos(v_R.clone(), Py::str("R"))?, f__pos(v_C.clone(), Py::str("C"))?])).items()?;
    if t4450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t4450[0].clone();
    v_C = t4450[1].clone();
    v_tau = py::truediv(&py::mul(&v_R.clone(), &v_C.clone())?, &Py::Float(1000.0))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("t_95"), py::mul(&Py::Float(3.0), &v_tau.clone())?)]);
    if ({ let t4451 = v_t.clone(); Py::Bool({ let t4452 = Py::None; if !py::is(&t4451, &t4452) { true } else { false } }) }).truthy() {
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
    let t4512 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?, py::sub(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?])).items()?;
    if t4512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4512[0].clone();
    v_b = t4512[1].clone();
    v_P = py::add(&py::copysign(&py::pow(&py::abs(&v_a.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_a.clone())?, &py::copysign(&py::pow(&py::abs(&v_b.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_b.clone())?)?;
    for t4513 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t4513;
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
    let t4610 = (Py::tuple(vec![f__pos(v_CcO2.clone(), Py::str("CcO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t4610.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cc = t4610[0].clone();
    v_Ca = t4610[1].clone();
    v_Cv = t4610[2].clone();
    let _ = f__need({ let t4611 = v_Cc.clone(); Py::Bool({ let t4612 = v_Ca.clone(); if py::ge(&t4611, &t4612)? { { let t4613 = v_Cv.clone(); if py::gt(&t4612, &t4613)? { true } else { false } } } else { false } }) }, Py::str("need CcO2 >= CaO2 > CvO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qs_Qt"), py::truediv(&py::sub(&v_Cc.clone(), &v_Ca.clone())?, &py::sub(&v_Cc.clone(), &v_Cv.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solution_osmolarity(mut v_conc: Py, mut v_molar_mass: Py, mut v_i: Py, mut v_phi: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4668 = (Py::tuple(vec![f__pos(v_conc.clone(), Py::str("conc"))?, f__pos(v_molar_mass.clone(), Py::str("molar_mass"))?, f__int_ok(v_i.clone(), Py::str("i"))?, f__pos(v_phi.clone(), Py::str("phi"))?])).items()?;
    if t4668.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_conc = t4668[0].clone();
    v_M = t4668[1].clone();
    v_i = t4668[2].clone();
    v_phi = t4668[3].clone();
    v_m = py::mul(&py::truediv(&v_conc.clone(), &v_M.clone())?, &Py::Float(1000.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("molarity"), v_m.clone()), (Py::str("osmolarity"), py::mul(&py::mul(&v_m.clone(), &v_i.clone())?, &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_starling_filtration(mut v_Pc: Py, mut v_Pi: Py, mut v_pic: Py, mut v_pii: Py, mut v_Kf: Py, mut v_sigma: Py) -> R {
    let mut v_nfp: Py = Py::None;
    let t4722 = (Py::tuple(vec![f__num(v_Pc.clone(), Py::str("Pc"))?, f__num(v_Pi.clone(), Py::str("Pi"))?, f__nonneg(v_pic.clone(), Py::str("pic"))?, f__nonneg(v_pii.clone(), Py::str("pii"))?, f__pos(v_Kf.clone(), Py::str("Kf"))?])).items()?;
    if t4722.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t4722[0].clone();
    v_Pi = t4722[1].clone();
    v_pic = t4722[2].clone();
    v_pii = t4722[3].clone();
    v_Kf = t4722[4].clone();
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
    let t4783 = (Py::tuple(vec![f__pos(v_rheobase.clone(), Py::str("rheobase"))?, f__pos(v_chronaxie.clone(), Py::str("chronaxie"))?, f__pos(v_t.clone(), Py::str("t"))?])).items()?;
    if t4783.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t4783[0].clone();
    v_c = t4783[1].clone();
    v_t = t4783[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I_lapicque"), py::truediv(&v_I0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_t.clone())?, &py::math1("log", &Py::Float(2.0))?)?, &v_c.clone())?)?)?)?), (Py::str("I_weiss"), py::mul(&v_I0.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_c.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_volume_ef(mut v_EDV: Py, mut v_ESV: Py) -> R {
    let mut v_sv: Py = Py::None;
    let t4837 = (Py::tuple(vec![f__pos(v_EDV.clone(), Py::str("EDV"))?, f__nonneg(v_ESV.clone(), Py::str("ESV"))?])).items()?;
    if t4837.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_EDV = t4837[0].clone();
    v_ESV = t4837[1].clone();
    let _ = f__need({ let t4838 = v_ESV.clone(); Py::Bool({ let t4839 = v_EDV.clone(); if py::lt(&t4838, &t4839)? { true } else { false } }) }, Py::str("ESV must be smaller than EDV"))?;
    v_sv = py::sub(&v_EDV.clone(), &v_ESV.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SV"), v_sv.clone()), (Py::str("EF"), py::truediv(&v_sv.clone(), &v_EDV.clone())?), (Py::str("EF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_sv.clone())?, &v_EDV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_work(mut v_SV: Py, mut v_MAP: Py, mut v_HR: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4891 = (Py::tuple(vec![f__pos(v_SV.clone(), Py::str("SV"))?, f__pos(v_MAP.clone(), Py::str("MAP"))?])).items()?;
    if t4891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SV = t4891[0].clone();
    v_MAP = t4891[1].clone();
    v_w = py::mul(&py::mul(&py::mul(&v_MAP.clone(), &c_MMHG()?)?, &v_SV.clone())?, &Py::Float(1e-06))?;
    v_out = Py::dict(vec![(Py::str("SW"), v_w.clone()), (Py::str("SW_gm"), py::truediv(&v_w.clone(), &py::mul(&c_G0()?, &Py::Float(0.001))?)?)]);
    if ({ let t4892 = v_HR.clone(); Py::Bool({ let t4893 = Py::None; if !py::is(&t4892, &t4893) { true } else { false } }) }).truthy() {
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
    let t5001 = (Py::tuple(vec![f__pos(v_LVIDd.clone(), Py::str("LVIDd"))?, f__pos(v_LVIDs.clone(), Py::str("LVIDs"))?])).items()?;
    if t5001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_LVIDd = t5001[0].clone();
    v_LVIDs = t5001[1].clone();
    let _ = f__need({ let t5002 = v_LVIDs.clone(); Py::Bool({ let t5003 = v_LVIDd.clone(); if py::lt(&t5002, &t5003)? { true } else { false } }) }, Py::str("the end-systolic diameter must be smaller than the end-diastolic"))?;
    let t5004 = (Py::tuple(vec![f__teich(v_LVIDd.clone())?, f__teich(v_LVIDs.clone())?])).items()?;
    if t5004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edv = t5004[0].clone();
    v_esv = t5004[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("EDV"), v_edv.clone()), (Py::str("ESV"), v_esv.clone()), (Py::str("EF"), py::truediv(&py::sub(&v_edv.clone(), &v_esv.clone())?, &v_edv.clone())?), (Py::str("FS"), py::truediv(&py::sub(&v_LVIDd.clone(), &v_LVIDs.clone())?, &v_LVIDd.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tubular_handling(mut v_GFR: Py, mut v_Px: Py, mut v_Ux: Py, mut v_V: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let t5058 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Px.clone(), Py::str("Px"))?, f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_V.clone(), Py::str("V"))?])).items()?;
    if t5058.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t5058[0].clone();
    v_Px = t5058[1].clone();
    v_Ux = t5058[2].clone();
    v_V = t5058[3].clone();
    let t5059 = (Py::tuple(vec![py::truediv(&py::mul(&v_GFR.clone(), &v_Px.clone())?, &Py::Float(100.0))?, py::truediv(&py::mul(&v_Ux.clone(), &v_V.clone())?, &Py::Float(100.0))?])).items()?;
    if t5059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fl = t5059[0].clone();
    v_ex = t5059[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("filtered_load"), v_fl.clone()), (Py::str("excreted"), v_ex.clone()), (Py::str("reabsorbed"), py::sub(&v_fl.clone(), &v_ex.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vascular_resistance(mut v_MAP: Py, mut v_CVP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4947 = (Py::tuple(vec![f__num(v_MAP.clone(), Py::str("MAP"))?, f__nonneg(v_CVP.clone(), Py::str("CVP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4947.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MAP = t4947[0].clone();
    v_CVP = t4947[1].clone();
    v_CO = t4947[2].clone();
    let _ = f__need({ let t4948 = v_MAP.clone(); Py::Bool({ let t4949 = v_CVP.clone(); if py::gt(&t4948, &t4949)? { true } else { false } }) }, Py::str("the mean arterial pressure must exceed the venous pressure"))?;
    v_w = py::truediv(&py::sub(&v_MAP.clone(), &v_CVP.clone())?, &v_CO.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SVR"), v_w.clone()), (Py::str("SVR_dyn"), py::mul(&v_w.clone(), &c_WOOD_DYN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vessel_reynolds(mut v_rho: Py, mut v_v: Py, mut v_D: Py, mut v_eta: Py) -> R {
    let mut v_Re: Py = Py::None;
    let t5163 = (Py::tuple(vec![f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_v.clone(), Py::str("v"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5163.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t5163[0].clone();
    v_v = t5163[1].clone();
    v_D = t5163[2].clone();
    v_eta = t5163[3].clone();
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &py::truediv(&v_v.clone(), &Py::Float(100.0))?)?, &py::truediv(&v_D.clone(), &Py::Float(1000.0))?)?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Re"), v_Re.clone()), (Py::str("v_critical"), py::mul(&py::truediv(&py::mul(&Py::Float(2000.0), &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_virtual_po2(mut v_PO2: Py, mut v_T: Py, mut v_pH: Py, mut v_PCO2: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let t5268 = (Py::tuple(vec![f__pos(v_PO2.clone(), Py::str("PO2"))?, f__num(v_T.clone(), Py::str("T"))?, f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PCO2.clone(), Py::str("PCO2"))?])).items()?;
    if t5268.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t5268[0].clone();
    v_T = t5268[1].clone();
    v_pH = t5268[2].clone();
    v_PCO2 = t5268[3].clone();
    let _ = f__need({ let t5275 = { let t5269 = Py::Int(25i64); Py::Bool({ let t5270 = v_T.clone(); if py::le(&t5269, &t5270)? { { let t5271 = Py::Int(43i64); if py::le(&t5270, &t5271)? { true } else { false } } } else { false } }) }; if !t5275.truthy() { t5275 } else { { let t5272 = Py::Float(6.8); Py::Bool({ let t5273 = v_pH.clone(); if py::le(&t5272, &t5273)? { { let t5274 = Py::Float(7.8); if py::le(&t5273, &t5274)? { true } else { false } } } else { false } }) } } }, Py::str("temperature 25-43 degC and pH 6.8-7.8"))?;
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
    let t5327 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_height.clone(), Py::str("height"))?, f__pos(v_weight.clone(), Py::str("weight"))?])).items()?;
    if t5327.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t5327[0].clone();
    v_height = t5327[1].clone();
    v_weight = t5327[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t5332 = v_age.clone(); Py::Bool({ let t5333 = Py::Int(18i64); if py::ge(&t5332, &t5333)? { true } else { false } }) }, Py::str("the Watson equations are for adults"))?;
    v_v = (if (v_fem.clone()).truthy() { py::add(&py::add(&py::neg(&Py::Float(2.097))?, &py::mul(&Py::Float(0.1069), &v_height.clone())?)?, &py::mul(&Py::Float(0.2466), &v_weight.clone())?)? } else { py::add(&py::add(&py::sub(&Py::Float(2.447), &py::mul(&Py::Float(0.09516), &v_age.clone())?)?, &py::mul(&Py::Float(0.1074), &v_height.clone())?)?, &py::mul(&Py::Float(0.3362), &v_weight.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("TBW"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weir_rq(mut v_VO2: Py, mut v_VCO2: Py) -> R {
    let mut v_ee: Py = Py::None;
    let t5385 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_VCO2.clone(), Py::str("VCO2"))?])).items()?;
    if t5385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t5385[0].clone();
    v_VCO2 = t5385[1].clone();
    v_ee = py::mul(&py::add(&py::truediv(&py::mul(&Py::Float(3.941), &v_VO2.clone())?, &Py::Float(1000.0))?, &py::truediv(&py::mul(&Py::Float(1.106), &v_VCO2.clone())?, &Py::Float(1000.0))?)?, &Py::Float(1440.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RQ"), py::truediv(&v_VCO2.clone(), &v_VO2.clone())?), (Py::str("EE"), v_ee.clone()), (Py::str("EE_kJ"), py::mul(&v_ee.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_womersley(mut v_r: Py, mut v_HR: Py, mut v_rho: Py, mut v_eta: Py) -> R {
    let mut v_w: Py = Py::None;
    let t5437 = (Py::tuple(vec![f__pos(v_r.clone(), Py::str("r"))?, f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5437.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_r = t5437[0].clone();
    v_HR = t5437[1].clone();
    v_rho = t5437[2].clone();
    v_eta = t5437[3].clone();
    v_w = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &c_PI()?)?, &v_HR.clone())?, &Py::Float(60.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("alpha"), py::mul(&py::truediv(&v_r.clone(), &Py::Float(1000.0))?, &py::math1("sqrt", &py::truediv(&py::mul(&v_w.clone(), &v_rho.clone())?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?)?)?), (Py::str("omega"), v_w.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1(mut v_kw: Py) -> R {
    return Ok({ let t29 = f__from_si(Py::dict(vec![(Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_aa_gradient(py::getitem(&t29, &Py::str("FiO2"))?, py::getitem(&t29, &Py::str("Patm"))?, py::getitem(&t29, &Py::str("PH2O"))?, py::getitem(&t29, &Py::str("PaCO2"))?, py::getitem(&t29, &Py::str("PaO2"))?, py::getitem(&t29, &Py::str("RQ"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1023(mut v_kw: Py) -> R {
    return Ok({ let t1038 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_index(py::getitem(&t1038, &Py::str("CO"))?, py::getitem(&t1038, &Py::str("BSA"))?, py::method(&t1038, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_68(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t134 = (t106).items()?;
        if t134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t134[0].clone();
        v_w = t134[1].clone();
        if ({ let t107 = v_k.clone(); Py::Bool({ let t108 = v_outputs.clone(); if !py::contains(&t108, &t107)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1045(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1047: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1023(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1047 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1048 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1076 = (t1048).items()?;
        if t1076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1076[0].clone();
        v_w = t1076[1].clone();
        if ({ let t1049 = v_k.clone(); Py::Bool({ let t1050 = v_outputs.clone(); if !py::contains(&t1050, &t1049)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1077(mut v_kw: Py) -> R {
    return Ok({ let t1090 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_cardiac_output(py::getitem(&t1090, &Py::str("HR"))?, py::getitem(&t1090, &Py::str("SV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1097(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1099: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1077(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1099 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1100 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1128 = (t1100).items()?;
        if t1128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1128[0].clone();
        v_w = t1128[1].clone();
        if ({ let t1101 = v_k.clone(); Py::Bool({ let t1102 = v_outputs.clone(); if !py::contains(&t1102, &t1101)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1129(mut v_kw: Py) -> R {
    return Ok({ let t1145 = f__from_si(Py::dict(vec![(Py::str("EK"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ENa"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("ECl"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_chord_conductance(py::getitem(&t1145, &Py::str("gK"))?, py::getitem(&t1145, &Py::str("gNa"))?, py::getitem(&t1145, &Py::str("gCl"))?, py::getitem(&t1145, &Py::str("EK"))?, py::getitem(&t1145, &Py::str("ENa"))?, py::getitem(&t1145, &Py::str("ECl"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1152(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1154: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1129(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1154 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1155 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1183 = (t1155).items()?;
        if t1183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1183[0].clone();
        v_w = t1183[1].clone();
        if ({ let t1156 = v_k.clone(); Py::Bool({ let t1157 = v_outputs.clone(); if !py::contains(&t1157, &t1156)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1184(mut v_kw: Py) -> R {
    return Ok({ let t1204 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_cockcroft_gault(py::getitem(&t1204, &Py::str("age"))?, py::getitem(&t1204, &Py::str("weight"))?, py::getitem(&t1204, &Py::str("Scr"))?, py::getitem(&t1204, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1184(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1213 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1214 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1242 = (t1214).items()?;
        if t1242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1242[0].clone();
        v_w = t1242[1].clone();
        if ({ let t1215 = v_k.clone(); Py::Bool({ let t1216 = v_outputs.clone(); if !py::contains(&t1216, &t1215)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1243(mut v_kw: Py) -> R {
    return Ok({ let t1260 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("d1"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("d2"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_conduction_velocity_scaling(py::getitem(&t1260, &Py::str("v1"))?, py::getitem(&t1260, &Py::str("d1"))?, py::getitem(&t1260, &Py::str("d2"))?, py::getitem(&t1260, &Py::str("fiber"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1267(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1269: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1243(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1269 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1270 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1298 = (t1270).items()?;
        if t1298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1298[0].clone();
        v_w = t1298[1].clone();
        if ({ let t1271 = v_k.clone(); Py::Bool({ let t1272 = v_outputs.clone(); if !py::contains(&t1272, &t1271)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1299(mut v_kw: Py) -> R {
    return Ok({ let t1312 = f__from_si(Py::dict(vec![(Py::str("Vurine"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("hours"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)])), (Py::str("BSA"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_creatinine_clearance_timed(py::getitem(&t1312, &Py::str("Ucr"))?, py::getitem(&t1312, &Py::str("Vurine"))?, py::getitem(&t1312, &Py::str("hours"))?, py::getitem(&t1312, &Py::str("Pcr"))?, py::method(&t1312, "get", &[Py::str("BSA"), Py::Float(1.73)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1321: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1299(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1321 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1322 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1350 = (t1322).items()?;
        if t1350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1350[0].clone();
        v_w = t1350[1].clone();
        if ({ let t1323 = v_k.clone(); Py::Bool({ let t1324 = v_outputs.clone(); if !py::contains(&t1324, &t1323)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_135(mut v_kw: Py) -> R {
    return Ok({ let t148 = f__from_si(Py::dict(vec![(Py::str("gamma"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_laplace(py::getitem(&t148, &Py::str("gamma"))?, py::getitem(&t148, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1351(mut v_kw: Py) -> R {
    return Ok({ let t1377 = f__from_si(Py::dict(vec![(Py::str("RR"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_ecg_heart_rate(py::method(&t1377, "get", &[Py::str("RR"), Py::None])?, py::method(&t1377, "get", &[Py::str("small_boxes"), Py::None])?, py::method(&t1377, "get", &[Py::str("large_boxes"), Py::None])?, py::method(&t1377, "get", &[Py::str("speed"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1384(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1386: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1351(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1386 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1387 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1415 = (t1387).items()?;
        if t1415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1415[0].clone();
        v_w = t1415[1].clone();
        if ({ let t1388 = v_k.clone(); Py::Bool({ let t1389 = v_outputs.clone(); if !py::contains(&t1389, &t1388)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1437 = v_inputs.clone(); f_egfr_ckd_epi_2021(py::getitem(&t1437, &Py::str("Scr"))?, py::getitem(&t1437, &Py::str("age"))?, py::getitem(&t1437, &Py::str("sex"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1438 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1439 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1467 = (t1439).items()?;
        if t1467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1467[0].clone();
        v_w = t1467[1].clone();
        if ({ let t1440 = v_k.clone(); Py::Bool({ let t1441 = v_outputs.clone(); if !py::contains(&t1441, &t1440)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1468(mut v_kw: Py) -> R {
    return Ok({ let t1490 = f__from_si(Py::dict(vec![(Py::str("Scys"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_egfr_ckd_epi_2021_cys(py::getitem(&t1490, &Py::str("Scr"))?, py::getitem(&t1490, &Py::str("Scys"))?, py::getitem(&t1490, &Py::str("age"))?, py::getitem(&t1490, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1497(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1499: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1468(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1499 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1500 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1528 = (t1500).items()?;
        if t1528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1528[0].clone();
        v_w = t1528[1].clone();
        if ({ let t1501 = v_k.clone(); Py::Bool({ let t1502 = v_outputs.clone(); if !py::contains(&t1502, &t1501)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1529(mut v_kw: Py) -> R {
    return Ok({ let t1546 = f__from_si(Py::dict(vec![(Py::str("CL"), Py::tuple(vec![Py::Float(2.7777777777777776e-07), Py::Float(0.0)])), (Py::str("Vd"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)]))]), v_kw.clone())?; f_elimination_half_life(py::getitem(&t1546, &Py::str("CL"))?, py::getitem(&t1546, &Py::str("Vd"))?, py::method(&t1546, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_135(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t158 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t186 = (t158).items()?;
        if t186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t186[0].clone();
        v_w = t186[1].clone();
        if ({ let t159 = v_k.clone(); Py::Bool({ let t160 = v_outputs.clone(); if !py::contains(&t160, &t159)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1553(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1555: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1529(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1555 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1556 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1584 = (t1556).items()?;
        if t1584.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1584[0].clone();
        v_w = t1584[1].clone();
        if ({ let t1557 = v_k.clone(); Py::Bool({ let t1558 = v_outputs.clone(); if !py::contains(&t1558, &t1557)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1585(mut v_kw: Py) -> R {
    return Ok({ let t1600 = f__from_si(Py::dict(vec![(Py::str("FEV1"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("FVC"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fev1_fvc(py::getitem(&t1600, &Py::str("FEV1"))?, py::getitem(&t1600, &Py::str("FVC"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1607(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1609: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1585(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1609 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1610 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1638 = (t1610).items()?;
        if t1638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1638[0].clone();
        v_w = t1638[1].clone();
        if ({ let t1611 = v_k.clone(); Py::Bool({ let t1612 = v_outputs.clone(); if !py::contains(&t1612, &t1611)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1639(mut v_kw: Py) -> R {
    return Ok({ let t1656 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_cardiac_output(py::getitem(&t1656, &Py::str("VO2"))?, py::getitem(&t1656, &Py::str("CaO2"))?, py::getitem(&t1656, &Py::str("CvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1639(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1666 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1694 = (t1666).items()?;
        if t1694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1694[0].clone();
        v_w = t1694[1].clone();
        if ({ let t1667 = v_k.clone(); Py::Bool({ let t1668 = v_outputs.clone(); if !py::contains(&t1668, &t1667)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1695(mut v_kw: Py) -> R {
    return Ok({ let t1708 = f__from_si(Py::dict(vec![(Py::str("D"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("dC"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_diffusion(py::getitem(&t1708, &Py::str("D"))?, py::getitem(&t1708, &Py::str("A"))?, py::getitem(&t1708, &Py::str("dC"))?, py::getitem(&t1708, &Py::str("T"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1715(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1717: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1695(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1717 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1718 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1746 = (t1718).items()?;
        if t1746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1746[0].clone();
        v_w = t1746[1].clone();
        if ({ let t1719 = v_k.clone(); Py::Bool({ let t1720 = v_outputs.clone(); if !py::contains(&t1720, &t1719)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1747(mut v_kw: Py) -> R {
    return Ok({ let t1775 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_saturations(py::getitem(&t1775, &Py::str("VO2"))?, py::getitem(&t1775, &Py::str("Hb"))?, py::getitem(&t1775, &Py::str("SaO2"))?, py::getitem(&t1775, &Py::str("SvO2"))?, py::getitem(&t1775, &Py::str("PaO2"))?, py::getitem(&t1775, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1782(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1784: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1747(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1784 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1785 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1813 = (t1785).items()?;
        if t1813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1813[0].clone();
        v_w = t1813[1].clone();
        if ({ let t1786 = v_k.clone(); Py::Bool({ let t1787 = v_outputs.clone(); if !py::contains(&t1787, &t1786)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1814(mut v_kw: Py) -> R {
    return Ok({ let t1829 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("RPF"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_filtration_fraction(py::getitem(&t1829, &Py::str("GFR"))?, py::getitem(&t1829, &Py::str("RPF"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1838: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1814(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1838 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1839 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1867 = (t1839).items()?;
        if t1867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1867[0].clone();
        v_w = t1867[1].clone();
        if ({ let t1840 = v_k.clone(); Py::Bool({ let t1841 = v_outputs.clone(); if !py::contains(&t1841, &t1840)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1868(mut v_kw: Py) -> R {
    return Ok({ let t1893 = f__from_si(Py::dict(vec![(Py::str("TBW"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("V_added"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_shift(py::getitem(&t1893, &Py::str("TBW"))?, py::getitem(&t1893, &Py::str("Posm"))?, py::getitem(&t1893, &Py::str("V_added"))?, py::getitem(&t1893, &Py::str("C_added"))?, py::method(&t1893, "get", &[Py::str("ICF_fraction"), py::truediv(&Py::Float(2.0), &Py::Float(3.0))?])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_187(mut v_kw: Py) -> R {
    return Ok({ let t200 = f__from_si(Py::dict(vec![(Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_ventilation_co2(py::getitem(&t200, &Py::str("VCO2"))?, py::getitem(&t200, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1868(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1903 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1931 = (t1903).items()?;
        if t1931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1931[0].clone();
        v_w = t1931[1].clone();
        if ({ let t1904 = v_k.clone(); Py::Bool({ let t1905 = v_outputs.clone(); if !py::contains(&t1905, &t1904)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1947 = v_inputs.clone(); f_fractional_excretion(py::getitem(&t1947, &Py::str("Ux"))?, py::getitem(&t1947, &Py::str("Px"))?, py::getitem(&t1947, &Py::str("Ucr"))?, py::getitem(&t1947, &Py::str("Pcr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1977 = (t1949).items()?;
        if t1977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1977[0].clone();
        v_w = t1977[1].clone();
        if ({ let t1950 = v_k.clone(); Py::Bool({ let t1951 = v_outputs.clone(); if !py::contains(&t1951, &t1950)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1978(mut v_kw: Py) -> R {
    return Ok({ let t1991 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_free_water_clearance(py::getitem(&t1991, &Py::str("V"))?, py::getitem(&t1991, &Py::str("Uosm"))?, py::getitem(&t1991, &Py::str("Posm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1998(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2000: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1978(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2000 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2001 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2029 = (t2001).items()?;
        if t2029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2029[0].clone();
        v_w = t2029[1].clone();
        if ({ let t2002 = v_k.clone(); Py::Bool({ let t2003 = v_outputs.clone(); if !py::contains(&t2003, &t2002)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2030(mut v_kw: Py) -> R {
    return Ok({ let t2054 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_gas_volume_conversion(py::getitem(&t2054, &Py::str("V"))?, py::getitem(&t2054, &Py::str("T"))?, py::getitem(&t2054, &Py::str("Patm"))?, py::getitem(&t2054, &Py::str("PH2O"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2061(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2063: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2030(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2063 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2064 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2092 = (t2064).items()?;
        if t2092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2092[0].clone();
        v_w = t2092[1].clone();
        if ({ let t2065 = v_k.clone(); Py::Bool({ let t2066 = v_outputs.clone(); if !py::contains(&t2066, &t2065)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t209: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_187(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t209 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t210 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t238 = (t210).items()?;
        if t238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t238[0].clone();
        v_w = t238[1].clone();
        if ({ let t211 = v_k.clone(); Py::Bool({ let t212 = v_outputs.clone(); if !py::contains(&t212, &t211)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2093(mut v_kw: Py) -> R {
    return Ok({ let t2113 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_ghk_voltage(py::getitem(&t2113, &Py::str("PK"))?, py::getitem(&t2113, &Py::str("PNa"))?, py::getitem(&t2113, &Py::str("PCl"))?, py::getitem(&t2113, &Py::str("K_o"))?, py::getitem(&t2113, &Py::str("K_i"))?, py::getitem(&t2113, &Py::str("Na_o"))?, py::getitem(&t2113, &Py::str("Na_i"))?, py::getitem(&t2113, &Py::str("Cl_o"))?, py::getitem(&t2113, &Py::str("Cl_i"))?, py::method(&t2113, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2120(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2122: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2093(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2122 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2123 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2151 = (t2123).items()?;
        if t2151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2151[0].clone();
        v_w = t2151[1].clone();
        if ({ let t2124 = v_k.clone(); Py::Bool({ let t2125 = v_outputs.clone(); if !py::contains(&t2125, &t2124)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2152(mut v_kw: Py) -> R {
    return Ok({ let t2168 = f__from_si(Py::dict(vec![(Py::str("c"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_gibbs_donnan(py::getitem(&t2168, &Py::str("A"))?, py::getitem(&t2168, &Py::str("c"))?, py::method(&t2168, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2152(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2178 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2206 = (t2178).items()?;
        if t2206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2206[0].clone();
        v_w = t2206[1].clone();
        if ({ let t2179 = v_k.clone(); Py::Bool({ let t2180 = v_outputs.clone(); if !py::contains(&t2180, &t2179)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2207(mut v_kw: Py) -> R {
    return Ok({ let t2222 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("Tm"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_glucose_titration(py::getitem(&t2222, &Py::str("GFR"))?, py::getitem(&t2222, &Py::str("Pglucose"))?, py::getitem(&t2222, &Py::str("Tm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2229(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2231: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2207(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2231 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2232 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2260 = (t2232).items()?;
        if t2260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2260[0].clone();
        v_w = t2260[1].clone();
        if ({ let t2233 = v_k.clone(); Py::Bool({ let t2234 = v_outputs.clone(); if !py::contains(&t2234, &t2233)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2287(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2289: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2288 = v_inputs.clone(); f_heart_rate_reserve(py::getitem(&t2288, &Py::str("age"))?, py::getitem(&t2288, &Py::str("HR_rest"))?, py::getitem(&t2288, &Py::str("intensity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2289 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2290 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2318 = (t2290).items()?;
        if t2318.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2318[0].clone();
        v_w = t2318[1].clone();
        if ({ let t2291 = v_k.clone(); Py::Bool({ let t2292 = v_outputs.clone(); if !py::contains(&t2292, &t2291)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2319(mut v_kw: Py) -> R {
    return Ok({ let t2349 = f__from_si(Py::dict(vec![(Py::str("P1"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_fit(py::getitem(&t2349, &Py::str("P1"))?, py::getitem(&t2349, &Py::str("S1"))?, py::getitem(&t2349, &Py::str("P2"))?, py::getitem(&t2349, &Py::str("S2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2358: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2319(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2358 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2359 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2387 = (t2359).items()?;
        if t2387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2387[0].clone();
        v_w = t2387[1].clone();
        if ({ let t2360 = v_k.clone(); Py::Bool({ let t2361 = v_outputs.clone(); if !py::contains(&t2361, &t2360)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2388(mut v_kw: Py) -> R {
    return Ok({ let t2405 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_force_velocity(py::getitem(&t2405, &Py::str("F0"))?, py::getitem(&t2405, &Py::str("a"))?, py::getitem(&t2405, &Py::str("b"))?, py::getitem(&t2405, &Py::str("v"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_239(mut v_kw: Py) -> R {
    return Ok({ let t252 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("Cl"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_anion_gap(py::getitem(&t252, &Py::str("Na"))?, py::getitem(&t252, &Py::str("Cl"))?, py::getitem(&t252, &Py::str("HCO3"))?, py::method(&t252, "get", &[Py::str("albumin"), Py::Float(4.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2412(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2414: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2388(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2414 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2415 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2443 = (t2415).items()?;
        if t2443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2443[0].clone();
        v_w = t2443[1].clone();
        if ({ let t2416 = v_k.clone(); Py::Bool({ let t2417 = v_outputs.clone(); if !py::contains(&t2417, &t2416)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2444(mut v_kw: Py) -> R {
    return Ok({ let t2466 = f__from_si(Py::dict(vec![(Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_po2(py::getitem(&t2466, &Py::str("SO2"))?, py::getitem(&t2466, &Py::str("P50"))?, py::getitem(&t2466, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2444(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2504 = (t2476).items()?;
        if t2504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2504[0].clone();
        v_w = t2504[1].clone();
        if ({ let t2477 = v_k.clone(); Py::Bool({ let t2478 = v_outputs.clone(); if !py::contains(&t2478, &t2477)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2505(mut v_kw: Py) -> R {
    return Ok({ let t2520 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_saturation(py::getitem(&t2520, &Py::str("PO2"))?, py::getitem(&t2520, &Py::str("P50"))?, py::getitem(&t2520, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2527(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2529: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2505(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2529 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2530 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2558 = (t2530).items()?;
        if t2558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2558[0].clone();
        v_w = t2558[1].clone();
        if ({ let t2531 = v_k.clone(); Py::Bool({ let t2532 = v_outputs.clone(); if !py::contains(&t2532, &t2531)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2559(mut v_kw: Py) -> R {
    return Ok({ let t2572 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_column(py::getitem(&t2572, &Py::str("h"))?, py::getitem(&t2572, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2579(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2581: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2559(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2581 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2582 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2610 = (t2582).items()?;
        if t2610.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2610[0].clone();
        v_w = t2610[1].clone();
        if ({ let t2583 = v_k.clone(); Py::Bool({ let t2584 = v_outputs.clone(); if !py::contains(&t2584, &t2583)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_259(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t261: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_239(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t261 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t262 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t290 = (t262).items()?;
        if t290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t290[0].clone();
        v_w = t290[1].clone();
        if ({ let t263 = v_k.clone(); Py::Bool({ let t264 = v_outputs.clone(); if !py::contains(&t264, &t263)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2611(mut v_kw: Py) -> R {
    return Ok({ let t2637 = f__from_si(Py::dict(vec![(Py::str("mass_air"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("mass_water"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("water_density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("GI"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_weighing(py::getitem(&t2637, &Py::str("mass_air"))?, py::getitem(&t2637, &Py::str("mass_water"))?, py::getitem(&t2637, &Py::str("water_density"))?, py::getitem(&t2637, &Py::str("RV"))?, py::method(&t2637, "get", &[Py::str("GI"), Py::Float(0.1)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2644(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2646: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2611(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2646 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2647 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2675 = (t2647).items()?;
        if t2675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2675[0].clone();
        v_w = t2675[1].clone();
        if ({ let t2648 = v_k.clone(); Py::Bool({ let t2649 = v_outputs.clone(); if !py::contains(&t2649, &t2648)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2676(mut v_kw: Py) -> R {
    return Ok({ let t2689 = f__from_si(Py::dict(vec![(Py::str("g"), Py::tuple(vec![Py::Float(1e-09), Py::Float(0.0)])), (Py::str("Vm"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("E"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_ionic_current(py::getitem(&t2689, &Py::str("g"))?, py::getitem(&t2689, &Py::str("Vm"))?, py::getitem(&t2689, &Py::str("E"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2696(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2676(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2698 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2699 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2727 = (t2699).items()?;
        if t2727.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2727[0].clone();
        v_w = t2727[1].clone();
        if ({ let t2700 = v_k.clone(); Py::Bool({ let t2701 = v_outputs.clone(); if !py::contains(&t2701, &t2700)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2728(mut v_kw: Py) -> R {
    return Ok({ let t2747 = f__from_si(Py::dict(vec![(Py::str("P"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_laplace_wall_tension(py::getitem(&t2747, &Py::str("P"))?, py::getitem(&t2747, &Py::str("r"))?, py::getitem(&t2747, &Py::str("shape"))?, py::method(&t2747, "get", &[Py::str("h"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2754(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2756: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2728(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2756 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2757 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2785 = (t2757).items()?;
        if t2785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2785[0].clone();
        v_w = t2785[1].clone();
        if ({ let t2758 = v_k.clone(); Py::Bool({ let t2759 = v_outputs.clone(); if !py::contains(&t2759, &t2758)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2786(mut v_kw: Py) -> R {
    return Ok({ let t2806 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_lean_body_mass_boer(py::getitem(&t2806, &Py::str("weight"))?, py::getitem(&t2806, &Py::str("height"))?, py::getitem(&t2806, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2813(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2815: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2786(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2815 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2816 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2844 = (t2816).items()?;
        if t2844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2844[0].clone();
        v_w = t2844[1].clone();
        if ({ let t2817 = v_k.clone(); Py::Bool({ let t2818 = v_outputs.clone(); if !py::contains(&t2818, &t2817)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2845(mut v_kw: Py) -> R {
    return Ok({ let t2862 = f__from_si(Py::dict(vec![(Py::str("d"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("x"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_length_constant(py::getitem(&t2862, &Py::str("Rm"))?, py::getitem(&t2862, &Py::str("Ri"))?, py::getitem(&t2862, &Py::str("d"))?, py::method(&t2862, "get", &[Py::str("x"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2869(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2871: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2845(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2871 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2872 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2900 = (t2872).items()?;
        if t2900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2900[0].clone();
        v_w = t2900[1].clone();
        if ({ let t2873 = v_k.clone(); Py::Bool({ let t2874 = v_outputs.clone(); if !py::contains(&t2874, &t2873)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2901(mut v_kw: Py) -> R {
    return Ok({ let t2916 = f__from_si(Py::dict(vec![(Py::str("TV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("IRV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ERV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_lung_volumes(py::getitem(&t2916, &Py::str("TV"))?, py::getitem(&t2916, &Py::str("IRV"))?, py::getitem(&t2916, &Py::str("ERV"))?, py::getitem(&t2916, &Py::str("RV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_291(mut v_kw: Py) -> R {
    return Ok({ let t306 = f__from_si(Py::dict(vec![(Py::str("carbohydrate"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("fat"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("protein"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("alcohol"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_atwater_energy(py::getitem(&t306, &Py::str("carbohydrate"))?, py::getitem(&t306, &Py::str("fat"))?, py::getitem(&t306, &Py::str("protein"))?, py::method(&t306, "get", &[Py::str("alcohol"), Py::Float(0.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2923(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2925: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2901(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2925 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2926 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2954 = (t2926).items()?;
        if t2954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2954[0].clone();
        v_w = t2954[1].clone();
        if ({ let t2927 = v_k.clone(); Py::Bool({ let t2928 = v_outputs.clone(); if !py::contains(&t2928, &t2927)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2955(mut v_kw: Py) -> R {
    return Ok({ let t2970 = f__from_si(Py::dict(vec![(Py::str("SBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("DBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_mean_arterial_pressure(py::getitem(&t2970, &Py::str("SBP"))?, py::getitem(&t2970, &Py::str("DBP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2977(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2979: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2955(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2979 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2980 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3008 = (t2980).items()?;
        if t3008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3008[0].clone();
        v_w = t3008[1].clone();
        if ({ let t2981 = v_k.clone(); Py::Bool({ let t2982 = v_outputs.clone(); if !py::contains(&t2982, &t2981)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3009(mut v_kw: Py) -> R {
    return Ok({ let t3024 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("dV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("area"), Py::tuple(vec![Py::Float(1e-12), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_charge(py::getitem(&t3024, &Py::str("Cm"))?, py::getitem(&t3024, &Py::str("dV"))?, py::getitem(&t3024, &Py::str("area"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3031(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3033: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3009(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3033 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3034 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3062 = (t3034).items()?;
        if t3062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3062[0].clone();
        v_w = t3062[1].clone();
        if ({ let t3035 = v_k.clone(); Py::Bool({ let t3036 = v_outputs.clone(); if !py::contains(&t3036, &t3035)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3063(mut v_kw: Py) -> R {
    return Ok({ let t3080 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_time_constant(py::getitem(&t3080, &Py::str("Rm"))?, py::getitem(&t3080, &Py::str("Cm"))?, py::method(&t3080, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3087(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3089: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3063(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3089 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3090 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3118 = (t3090).items()?;
        if t3118.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3118[0].clone();
        v_w = t3118[1].clone();
        if ({ let t3091 = v_k.clone(); Py::Bool({ let t3092 = v_outputs.clone(); if !py::contains(&t3092, &t3091)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3119(mut v_kw: Py) -> R {
    return Ok({ let t3132 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("duration"), Py::tuple(vec![Py::Float(60.0), Py::Float(0.0)]))]), v_kw.clone())?; f_met_energy(py::getitem(&t3132, &Py::str("MET"))?, py::getitem(&t3132, &Py::str("weight"))?, py::getitem(&t3132, &Py::str("duration"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_313(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t315: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_291(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t315 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t316 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t344 = (t316).items()?;
        if t344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t344[0].clone();
        v_w = t344[1].clone();
        if ({ let t317 = v_k.clone(); Py::Bool({ let t318 = v_outputs.clone(); if !py::contains(&t318, &t317)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3139(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3141: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3119(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3141 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3142 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3170 = (t3142).items()?;
        if t3170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3170[0].clone();
        v_w = t3170[1].clone();
        if ({ let t3143 = v_k.clone(); Py::Bool({ let t3144 = v_outputs.clone(); if !py::contains(&t3144, &t3143)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3171(mut v_kw: Py) -> R {
    return Ok({ let t3188 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("VD"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_minute_ventilation(py::getitem(&t3188, &Py::str("VT"))?, py::getitem(&t3188, &Py::str("RR"))?, py::getitem(&t3188, &Py::str("VD"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3195(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3197: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3171(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3197 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3198 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3226 = (t3198).items()?;
        if t3226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3226[0].clone();
        v_w = t3226[1].clone();
        if ({ let t3199 = v_k.clone(); Py::Bool({ let t3200 = v_outputs.clone(); if !py::contains(&t3200, &t3199)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3227(mut v_kw: Py) -> R {
    return Ok({ let t3240 = f__from_si(Py::dict(vec![(Py::str("E"), Py::tuple(vec![Py::Float(1000.0), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_moens_korteweg(py::getitem(&t3240, &Py::str("E"))?, py::getitem(&t3240, &Py::str("h"))?, py::getitem(&t3240, &Py::str("D"))?, py::getitem(&t3240, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3247(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3249: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3227(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3249 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3250 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3278 = (t3250).items()?;
        if t3278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3278[0].clone();
        v_w = t3278[1].clone();
        if ({ let t3251 = v_k.clone(); Py::Bool({ let t3252 = v_outputs.clone(); if !py::contains(&t3252, &t3251)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3279(mut v_kw: Py) -> R {
    return Ok({ let t3292 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_muscle_max_power(py::getitem(&t3292, &Py::str("F0"))?, py::getitem(&t3292, &Py::str("a"))?, py::getitem(&t3292, &Py::str("b"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3299(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3301: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3279(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3301 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3302 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3330 = (t3302).items()?;
        if t3330.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3330[0].clone();
        v_w = t3330[1].clone();
        if ({ let t3303 = v_k.clone(); Py::Bool({ let t3304 = v_outputs.clone(); if !py::contains(&t3304, &t3303)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3331(mut v_kw: Py) -> R {
    return Ok({ let t3353 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_nernst(py::getitem(&t3353, &Py::str("z"))?, py::getitem(&t3353, &Py::str("Cout"))?, py::getitem(&t3353, &Py::str("Cin"))?, py::method(&t3353, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3360(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3362: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3331(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3362 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3363 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3391 = (t3363).items()?;
        if t3391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3391[0].clone();
        v_w = t3391[1].clone();
        if ({ let t3364 = v_k.clone(); Py::Bool({ let t3365 = v_outputs.clone(); if !py::contains(&t3365, &t3364)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3392(mut v_kw: Py) -> R {
    return Ok({ let t3409 = f__from_si(Py::dict(vec![(Py::str("distance"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_proximal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_distal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_nerve_conduction_velocity(py::getitem(&t3409, &Py::str("distance"))?, py::getitem(&t3409, &Py::str("latency_proximal"))?, py::getitem(&t3409, &Py::str("latency_distal"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3418: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3392(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3418 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3419 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3447 = (t3419).items()?;
        if t3447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3447[0].clone();
        v_w = t3447[1].clone();
        if ({ let t3420 = v_k.clone(); Py::Bool({ let t3421 = v_outputs.clone(); if !py::contains(&t3421, &t3420)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3448(mut v_kw: Py) -> R {
    return Ok({ let t3464 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_osmotic_pressure(py::getitem(&t3464, &Py::str("osm"))?, py::method(&t3464, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_345(mut v_kw: Py) -> R {
    return Ok({ let t367 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_barometric_altitude(py::getitem(&t367, &Py::str("h"))?, py::method(&t367, "get", &[Py::str("FiO2"), Py::Float(0.2095)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3471(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3473: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3448(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3473 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3474 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3502 = (t3474).items()?;
        if t3502.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3502[0].clone();
        v_w = t3502[1].clone();
        if ({ let t3475 = v_k.clone(); Py::Bool({ let t3476 = v_outputs.clone(); if !py::contains(&t3476, &t3475)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3503(mut v_kw: Py) -> R {
    return Ok({ let t3531 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_consumption(py::getitem(&t3531, &Py::str("CO"))?, py::getitem(&t3531, &Py::str("Hb"))?, py::getitem(&t3531, &Py::str("SaO2"))?, py::getitem(&t3531, &Py::str("SvO2"))?, py::getitem(&t3531, &Py::str("PaO2"))?, py::getitem(&t3531, &Py::str("PvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3538(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3540: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3503(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3540 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3541 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3569 = (t3541).items()?;
        if t3569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3569[0].clone();
        v_w = t3569[1].clone();
        if ({ let t3542 = v_k.clone(); Py::Bool({ let t3543 = v_outputs.clone(); if !py::contains(&t3543, &t3542)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3570(mut v_kw: Py) -> R {
    return Ok({ let t3595 = f__from_si(Py::dict(vec![(Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_content(py::getitem(&t3595, &Py::str("Hb"))?, py::getitem(&t3595, &Py::str("SaO2"))?, py::getitem(&t3595, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_36(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t38: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t38 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t39 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t67 = (t39).items()?;
        if t67.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t67[0].clone();
        v_w = t67[1].clone();
        if ({ let t40 = v_k.clone(); Py::Bool({ let t41 = v_outputs.clone(); if !py::contains(&t41, &t40)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3602(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3604: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3570(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3604 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3605 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3633 = (t3605).items()?;
        if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3633[0].clone();
        v_w = t3633[1].clone();
        if ({ let t3606 = v_k.clone(); Py::Bool({ let t3607 = v_outputs.clone(); if !py::contains(&t3607, &t3606)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3634(mut v_kw: Py) -> R {
    return Ok({ let t3658 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_delivery(py::getitem(&t3658, &Py::str("CO"))?, py::getitem(&t3658, &Py::str("Hb"))?, py::getitem(&t3658, &Py::str("SaO2"))?, py::getitem(&t3658, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3665(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3667: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3634(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3667 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3668 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3696 = (t3668).items()?;
        if t3696.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3696[0].clone();
        v_w = t3696[1].clone();
        if ({ let t3669 = v_k.clone(); Py::Bool({ let t3670 = v_outputs.clone(); if !py::contains(&t3670, &t3669)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3697(mut v_kw: Py) -> R {
    return Ok({ let t3720 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_pah_renal_flow(py::getitem(&t3720, &Py::str("U_PAH"))?, py::getitem(&t3720, &Py::str("V"))?, py::getitem(&t3720, &Py::str("P_PAH"))?, py::getitem(&t3720, &Py::str("Hct"))?, py::method(&t3720, "get", &[Py::str("E"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3727(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3729: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3697(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3729 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3730 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3758 = (t3730).items()?;
        if t3758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3758[0].clone();
        v_w = t3758[1].clone();
        if ({ let t3731 = v_k.clone(); Py::Bool({ let t3732 = v_outputs.clone(); if !py::contains(&t3732, &t3731)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_374(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t376: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_345(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t376 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t377 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t405 = (t377).items()?;
        if t405.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t405[0].clone();
        v_w = t405[1].clone();
        if ({ let t378 = v_k.clone(); Py::Bool({ let t379 = v_outputs.clone(); if !py::contains(&t379, &t378)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3759(mut v_kw: Py) -> R {
    return Ok({ let t3775 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_pco2_from_ph(py::getitem(&t3775, &Py::str("pH"))?, py::getitem(&t3775, &Py::str("HCO3"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3782(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3784: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3759(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3784 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3785 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3813 = (t3785).items()?;
        if t3813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3813[0].clone();
        v_w = t3813[1].clone();
        if ({ let t3786 = v_k.clone(); Py::Bool({ let t3787 = v_outputs.clone(); if !py::contains(&t3787, &t3786)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3814(mut v_kw: Py) -> R {
    return Ok({ let t3827 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_ph_henderson(py::getitem(&t3827, &Py::str("HCO3"))?, py::getitem(&t3827, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3834(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3836: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3814(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3836 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3837 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3865 = (t3837).items()?;
        if t3865.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3865[0].clone();
        v_w = t3865[1].clone();
        if ({ let t3838 = v_k.clone(); Py::Bool({ let t3839 = v_outputs.clone(); if !py::contains(&t3839, &t3838)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3866(mut v_kw: Py) -> R {
    return Ok({ let t3883 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_plasma_osmolality(py::getitem(&t3883, &Py::str("Na"))?, py::getitem(&t3883, &Py::str("glucose"))?, py::getitem(&t3883, &Py::str("BUN"))?, py::method(&t3883, "get", &[Py::str("measured"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3890(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3892: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3866(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3892 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3893 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3921 = (t3893).items()?;
        if t3921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3921[0].clone();
        v_w = t3921[1].clone();
        if ({ let t3894 = v_k.clone(); Py::Bool({ let t3895 = v_outputs.clone(); if !py::contains(&t3895, &t3894)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3922(mut v_kw: Py) -> R {
    return Ok({ let t3935 = f__from_si(Py::dict(vec![(Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_flow(py::getitem(&t3935, &Py::str("dP"))?, py::getitem(&t3935, &Py::str("eta"))?, py::getitem(&t3935, &Py::str("L"))?, py::getitem(&t3935, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3942(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3944: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3922(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3944 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3945 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3973 = (t3945).items()?;
        if t3973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3973[0].clone();
        v_w = t3973[1].clone();
        if ({ let t3946 = v_k.clone(); Py::Bool({ let t3947 = v_outputs.clone(); if !py::contains(&t3947, &t3946)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3974(mut v_kw: Py) -> R {
    return Ok({ let t3987 = f__from_si(Py::dict(vec![(Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_resistance(py::getitem(&t3987, &Py::str("eta"))?, py::getitem(&t3987, &Py::str("L"))?, py::getitem(&t3987, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3994(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3996: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3974(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3996 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3997 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4025 = (t3997).items()?;
        if t4025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4025[0].clone();
        v_w = t4025[1].clone();
        if ({ let t3998 = v_k.clone(); Py::Bool({ let t3999 = v_outputs.clone(); if !py::contains(&t3999, &t3998)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4026(mut v_kw: Py) -> R {
    return Ok({ let t4043 = f__from_si(Py::dict(vec![(Py::str("mPAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PCWP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_pulmonary_vascular_resistance(py::getitem(&t4043, &Py::str("mPAP"))?, py::getitem(&t4043, &Py::str("PCWP"))?, py::getitem(&t4043, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4050(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4052: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4026(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4052 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4053 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4081 = (t4053).items()?;
        if t4081.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4081[0].clone();
        v_w = t4081[1].clone();
        if ({ let t4054 = v_k.clone(); Py::Bool({ let t4055 = v_outputs.clone(); if !py::contains(&t4055, &t4054)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_406(mut v_kw: Py) -> R {
    return Ok({ let t423 = f__from_si(Py::dict(vec![(Py::str("v1"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v2"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bernoulli_drop(py::getitem(&t423, &Py::str("v1"))?, py::getitem(&t423, &Py::str("v2"))?, py::getitem(&t423, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4106(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4108: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4107 = v_inputs.clone(); f_qp_qs(py::getitem(&t4107, &Py::str("SaO2"))?, py::getitem(&t4107, &Py::str("SvO2"))?, py::getitem(&t4107, &Py::str("SpvO2"))?, py::getitem(&t4107, &Py::str("SpaO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4108 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4109 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4137 = (t4109).items()?;
        if t4137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4137[0].clone();
        v_w = t4137[1].clone();
        if ({ let t4110 = v_k.clone(); Py::Bool({ let t4111 = v_outputs.clone(); if !py::contains(&t4111, &t4110)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4155: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4154 = v_inputs.clone(); f_qrs_axis(py::getitem(&t4154, &Py::str("lead_I"))?, py::getitem(&t4154, &Py::str("aVF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4155 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4156 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4184 = (t4156).items()?;
        if t4184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4184[0].clone();
        v_w = t4184[1].clone();
        if ({ let t4157 = v_k.clone(); Py::Bool({ let t4158 = v_outputs.clone(); if !py::contains(&t4158, &t4157)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4185(mut v_kw: Py) -> R {
    return Ok({ let t4198 = f__from_si(Py::dict(vec![(Py::str("QT"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_qt_correction(py::getitem(&t4198, &Py::str("QT"))?, py::getitem(&t4198, &Py::str("HR"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4185(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4207 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4208 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4236 = (t4208).items()?;
        if t4236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4236[0].clone();
        v_w = t4236[1].clone();
        if ({ let t4209 = v_k.clone(); Py::Bool({ let t4210 = v_outputs.clone(); if !py::contains(&t4210, &t4209)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4251: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4250 = v_inputs.clone(); f_radius_flow_ratio(py::getitem(&t4250, &Py::str("r1"))?, py::getitem(&t4250, &Py::str("r2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4251 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4252 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4280 = (t4252).items()?;
        if t4280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4280[0].clone();
        v_w = t4280[1].clone();
        if ({ let t4253 = v_k.clone(); Py::Bool({ let t4254 = v_outputs.clone(); if !py::contains(&t4254, &t4253)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_430(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t432: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_406(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t432 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t433 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t461 = (t433).items()?;
        if t461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t461[0].clone();
        v_w = t461[1].clone();
        if ({ let t434 = v_k.clone(); Py::Bool({ let t435 = v_outputs.clone(); if !py::contains(&t435, &t434)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4302(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4304: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4303 = v_inputs.clone(); f_red_cell_indices(py::getitem(&t4303, &Py::str("Hct"))?, py::getitem(&t4303, &Py::str("Hb"))?, py::getitem(&t4303, &Py::str("RBC"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4304 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4305 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4333 = (t4305).items()?;
        if t4333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4333[0].clone();
        v_w = t4333[1].clone();
        if ({ let t4306 = v_k.clone(); Py::Bool({ let t4307 = v_outputs.clone(); if !py::contains(&t4307, &t4306)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4334(mut v_kw: Py) -> R {
    return Ok({ let t4349 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_renal_clearance(py::getitem(&t4349, &Py::str("U"))?, py::getitem(&t4349, &Py::str("V"))?, py::getitem(&t4349, &Py::str("P"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4358: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4334(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4358 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4359 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4387 = (t4359).items()?;
        if t4387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4387[0].clone();
        v_w = t4387[1].clone();
        if ({ let t4360 = v_k.clone(); Py::Bool({ let t4361 = v_outputs.clone(); if !py::contains(&t4361, &t4360)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4388(mut v_kw: Py) -> R {
    return Ok({ let t4406 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("flow"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_mechanics(py::getitem(&t4406, &Py::str("VT"))?, py::getitem(&t4406, &Py::str("Ppeak"))?, py::getitem(&t4406, &Py::str("Pplat"))?, py::getitem(&t4406, &Py::str("PEEP"))?, py::getitem(&t4406, &Py::str("flow"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4388(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4444 = (t4416).items()?;
        if t4444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4444[0].clone();
        v_w = t4444[1].clone();
        if ({ let t4417 = v_k.clone(); Py::Bool({ let t4418 = v_outputs.clone(); if !py::contains(&t4418, &t4417)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4445(mut v_kw: Py) -> R {
    return Ok({ let t4462 = f__from_si(Py::dict(vec![(Py::str("t"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_time_constant(py::getitem(&t4462, &Py::str("R"))?, py::getitem(&t4462, &Py::str("C"))?, py::method(&t4462, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4469(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4445(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4471 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4472 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4500 = (t4472).items()?;
        if t4500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4500[0].clone();
        v_w = t4500[1].clone();
        if ({ let t4473 = v_k.clone(); Py::Bool({ let t4474 = v_outputs.clone(); if !py::contains(&t4474, &t4473)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4523: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4522 = v_inputs.clone(); f_severinghaus_po2(py::getitem(&t4522, &Py::str("SO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4523 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4524 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4552 = (t4524).items()?;
        if t4552.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4552[0].clone();
        v_w = t4552[1].clone();
        if ({ let t4525 = v_k.clone(); Py::Bool({ let t4526 = v_outputs.clone(); if !py::contains(&t4526, &t4525)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4553(mut v_kw: Py) -> R {
    return Ok({ let t4565 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_severinghaus_saturation(py::getitem(&t4565, &Py::str("PO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4572(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4553(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4575 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4603 = (t4575).items()?;
        if t4603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4603[0].clone();
        v_w = t4603[1].clone();
        if ({ let t4576 = v_k.clone(); Py::Bool({ let t4577 = v_outputs.clone(); if !py::contains(&t4577, &t4576)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_462(mut v_kw: Py) -> R {
    return Ok({ let t478 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_bicarbonate_from_ph(py::getitem(&t478, &Py::str("pH"))?, py::getitem(&t478, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4622 = v_inputs.clone(); f_shunt_fraction(py::getitem(&t4622, &Py::str("CcO2"))?, py::getitem(&t4622, &Py::str("CaO2"))?, py::getitem(&t4622, &Py::str("CvO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4652 = (t4624).items()?;
        if t4652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4652[0].clone();
        v_w = t4652[1].clone();
        if ({ let t4625 = v_k.clone(); Py::Bool({ let t4626 = v_outputs.clone(); if !py::contains(&t4626, &t4625)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4653(mut v_kw: Py) -> R {
    return Ok({ let t4676 = f__from_si(Py::dict(vec![(Py::str("conc"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("molar_mass"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_solution_osmolarity(py::getitem(&t4676, &Py::str("conc"))?, py::getitem(&t4676, &Py::str("molar_mass"))?, py::getitem(&t4676, &Py::str("i"))?, py::method(&t4676, "get", &[Py::str("phi"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4683(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4685: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4653(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4685 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4686 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4714 = (t4686).items()?;
        if t4714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4714[0].clone();
        v_w = t4714[1].clone();
        if ({ let t4687 = v_k.clone(); Py::Bool({ let t4688 = v_outputs.clone(); if !py::contains(&t4688, &t4687)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4715(mut v_kw: Py) -> R {
    return Ok({ let t4739 = f__from_si(Py::dict(vec![(Py::str("Pc"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("Pi"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pic"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pii"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_starling_filtration(py::getitem(&t4739, &Py::str("Pc"))?, py::getitem(&t4739, &Py::str("Pi"))?, py::getitem(&t4739, &Py::str("pic"))?, py::getitem(&t4739, &Py::str("pii"))?, py::getitem(&t4739, &Py::str("Kf"))?, py::method(&t4739, "get", &[Py::str("sigma"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4746(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4748: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4715(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4748 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4749 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4777 = (t4749).items()?;
        if t4777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4777[0].clone();
        v_w = t4777[1].clone();
        if ({ let t4750 = v_k.clone(); Py::Bool({ let t4751 = v_outputs.clone(); if !py::contains(&t4751, &t4750)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4778(mut v_kw: Py) -> R {
    return Ok({ let t4791 = f__from_si(Py::dict(vec![(Py::str("chronaxie"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_strength_duration(py::getitem(&t4791, &Py::str("rheobase"))?, py::getitem(&t4791, &Py::str("chronaxie"))?, py::getitem(&t4791, &Py::str("t"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4798(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4800: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4778(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4800 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4801 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4829 = (t4801).items()?;
        if t4829.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4829[0].clone();
        v_w = t4829[1].clone();
        if ({ let t4802 = v_k.clone(); Py::Bool({ let t4803 = v_outputs.clone(); if !py::contains(&t4803, &t4802)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4830(mut v_kw: Py) -> R {
    return Ok({ let t4847 = f__from_si(Py::dict(vec![(Py::str("EDV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ESV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_volume_ef(py::getitem(&t4847, &Py::str("EDV"))?, py::getitem(&t4847, &Py::str("ESV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_485(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t487: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_462(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t487 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t488 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t516 = (t488).items()?;
        if t516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t516[0].clone();
        v_w = t516[1].clone();
        if ({ let t489 = v_k.clone(); Py::Bool({ let t490 = v_outputs.clone(); if !py::contains(&t490, &t489)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4854(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4856: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4830(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4856 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4857 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4885 = (t4857).items()?;
        if t4885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4885[0].clone();
        v_w = t4885[1].clone();
        if ({ let t4858 = v_k.clone(); Py::Bool({ let t4859 = v_outputs.clone(); if !py::contains(&t4859, &t4858)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4886(mut v_kw: Py) -> R {
    return Ok({ let t4901 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_work(py::getitem(&t4901, &Py::str("SV"))?, py::getitem(&t4901, &Py::str("MAP"))?, py::method(&t4901, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4908(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4910: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4886(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4910 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4911 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4939 = (t4911).items()?;
        if t4939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4939[0].clone();
        v_w = t4939[1].clone();
        if ({ let t4912 = v_k.clone(); Py::Bool({ let t4913 = v_outputs.clone(); if !py::contains(&t4913, &t4912)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4940(mut v_kw: Py) -> R {
    return Ok({ let t4957 = f__from_si(Py::dict(vec![(Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CVP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_vascular_resistance(py::getitem(&t4957, &Py::str("MAP"))?, py::getitem(&t4957, &Py::str("CVP"))?, py::getitem(&t4957, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4966: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4940(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4966 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4967 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4995 = (t4967).items()?;
        if t4995.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4995[0].clone();
        v_w = t4995[1].clone();
        if ({ let t4968 = v_k.clone(); Py::Bool({ let t4969 = v_outputs.clone(); if !py::contains(&t4969, &t4968)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4996(mut v_kw: Py) -> R {
    return Ok({ let t5012 = f__from_si(Py::dict(vec![(Py::str("LVIDd"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("LVIDs"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_teichholz(py::getitem(&t5012, &Py::str("LVIDd"))?, py::getitem(&t5012, &Py::str("LVIDs"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5021: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4996(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5021 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5022 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5050 = (t5022).items()?;
        if t5050.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5050[0].clone();
        v_w = t5050[1].clone();
        if ({ let t5023 = v_k.clone(); Py::Bool({ let t5024 = v_outputs.clone(); if !py::contains(&t5024, &t5023)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5051(mut v_kw: Py) -> R {
    return Ok({ let t5067 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_tubular_handling(py::getitem(&t5067, &Py::str("GFR"))?, py::getitem(&t5067, &Py::str("Px"))?, py::getitem(&t5067, &Py::str("Ux"))?, py::getitem(&t5067, &Py::str("V"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5074(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5076: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5051(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5076 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5077 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5105 = (t5077).items()?;
        if t5105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5105[0].clone();
        v_w = t5105[1].clone();
        if ({ let t5078 = v_k.clone(); Py::Bool({ let t5079 = v_outputs.clone(); if !py::contains(&t5079, &t5078)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5106(mut v_kw: Py) -> R {
    return Ok({ let t5119 = f__from_si(Py::dict(vec![(Py::str("dV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_compliance(py::getitem(&t5119, &Py::str("dV"))?, py::getitem(&t5119, &Py::str("dP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5126(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5106(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5128 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5129 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5157 = (t5129).items()?;
        if t5157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5157[0].clone();
        v_w = t5157[1].clone();
        if ({ let t5130 = v_k.clone(); Py::Bool({ let t5131 = v_outputs.clone(); if !py::contains(&t5131, &t5130)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5158(mut v_kw: Py) -> R {
    return Ok({ let t5171 = f__from_si(Py::dict(vec![(Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_vessel_reynolds(py::getitem(&t5171, &Py::str("rho"))?, py::getitem(&t5171, &Py::str("v"))?, py::getitem(&t5171, &Py::str("D"))?, py::getitem(&t5171, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_517(mut v_kw: Py) -> R {
    return Ok({ let t530 = f__from_si(Py::dict(vec![(Py::str("Q"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)]))]), v_kw.clone())?; f_blood_velocity_area(py::getitem(&t530, &Py::str("Q"))?, py::getitem(&t530, &Py::str("A"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5178(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5180: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5158(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5180 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5181 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5209 = (t5181).items()?;
        if t5209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5209[0].clone();
        v_w = t5209[1].clone();
        if ({ let t5182 = v_k.clone(); Py::Bool({ let t5183 = v_outputs.clone(); if !py::contains(&t5183, &t5182)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5231(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5233: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5232 = v_inputs.clone(); f_network_resistance(py::getitem(&t5232, &Py::str("R"))?, py::getitem(&t5232, &Py::str("arrangement"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5233 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5234 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5262 = (t5234).items()?;
        if t5262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5262[0].clone();
        v_w = t5262[1].clone();
        if ({ let t5235 = v_k.clone(); Py::Bool({ let t5236 = v_outputs.clone(); if !py::contains(&t5236, &t5235)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5263(mut v_kw: Py) -> R {
    return Ok({ let t5283 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("PCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_virtual_po2(py::getitem(&t5283, &Py::str("PO2"))?, py::getitem(&t5283, &Py::str("T"))?, py::getitem(&t5283, &Py::str("pH"))?, py::getitem(&t5283, &Py::str("PCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5290(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5292: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5263(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5292 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5293 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5321 = (t5293).items()?;
        if t5321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5321[0].clone();
        v_w = t5321[1].clone();
        if ({ let t5294 = v_k.clone(); Py::Bool({ let t5295 = v_outputs.clone(); if !py::contains(&t5295, &t5294)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5322(mut v_kw: Py) -> R {
    return Ok({ let t5341 = f__from_si(Py::dict(vec![(Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_watson_tbw(py::getitem(&t5341, &Py::str("age"))?, py::getitem(&t5341, &Py::str("height"))?, py::getitem(&t5341, &Py::str("weight"))?, py::getitem(&t5341, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5348(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5350: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5322(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5350 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5351 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5379 = (t5351).items()?;
        if t5379.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5379[0].clone();
        v_w = t5379[1].clone();
        if ({ let t5352 = v_k.clone(); Py::Bool({ let t5353 = v_outputs.clone(); if !py::contains(&t5353, &t5352)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_537(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t539: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_517(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t539 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t540 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t568 = (t540).items()?;
        if t568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t568[0].clone();
        v_w = t568[1].clone();
        if ({ let t541 = v_k.clone(); Py::Bool({ let t542 = v_outputs.clone(); if !py::contains(&t542, &t541)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5380(mut v_kw: Py) -> R {
    return Ok({ let t5393 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_weir_rq(py::getitem(&t5393, &Py::str("VO2"))?, py::getitem(&t5393, &Py::str("VCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5400(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5402: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5380(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5402 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5403 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5431 = (t5403).items()?;
        if t5431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5431[0].clone();
        v_w = t5431[1].clone();
        if ({ let t5404 = v_k.clone(); Py::Bool({ let t5405 = v_outputs.clone(); if !py::contains(&t5405, &t5404)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5432(mut v_kw: Py) -> R {
    return Ok({ let t5445 = f__from_si(Py::dict(vec![(Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_womersley(py::getitem(&t5445, &Py::str("r"))?, py::getitem(&t5445, &Py::str("HR"))?, py::getitem(&t5445, &Py::str("rho"))?, py::getitem(&t5445, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5452(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5454: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5432(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5454 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5455 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5483 = (t5455).items()?;
        if t5483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5483[0].clone();
        v_w = t5483[1].clone();
        if ({ let t5456 = v_k.clone(); Py::Bool({ let t5457 = v_outputs.clone(); if !py::contains(&t5457, &t5456)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_569(mut v_kw: Py) -> R {
    return Ok({ let t582 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmi(py::getitem(&t582, &Py::str("weight"))?, py::getitem(&t582, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_589(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t591: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_569(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t591 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t592 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t620 = (t592).items()?;
        if t620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t620[0].clone();
        v_w = t620[1].clone();
        if ({ let t593 = v_k.clone(); Py::Bool({ let t594 = v_outputs.clone(); if !py::contains(&t594, &t593)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_621(mut v_kw: Py) -> R {
    return Ok({ let t644 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_harris_benedict(py::getitem(&t644, &Py::str("weight"))?, py::getitem(&t644, &Py::str("height"))?, py::getitem(&t644, &Py::str("age"))?, py::getitem(&t644, &Py::str("sex"))?, py::getitem(&t644, &Py::str("version"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_651(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t653: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_621(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t653 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t654 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t682 = (t654).items()?;
        if t682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t682[0].clone();
        v_w = t682[1].clone();
        if ({ let t655 = v_k.clone(); Py::Bool({ let t656 = v_outputs.clone(); if !py::contains(&t656, &t655)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_68(mut v_kw: Py) -> R {
    return Ok({ let t96 = f__from_si(Py::dict(vec![(Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_alveolar_gas(py::getitem(&t96, &Py::str("FiO2"))?, py::getitem(&t96, &Py::str("Patm"))?, py::getitem(&t96, &Py::str("PH2O"))?, py::getitem(&t96, &Py::str("PaCO2"))?, py::getitem(&t96, &Py::str("RQ"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_683(mut v_kw: Py) -> R {
    return Ok({ let t705 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_katch_mcardle(py::getitem(&t705, &Py::str("weight"))?, py::getitem(&t705, &Py::str("body_fat"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_712(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t714: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_683(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t714 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t715 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t743 = (t715).items()?;
        if t743.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t743[0].clone();
        v_w = t743[1].clone();
        if ({ let t716 = v_k.clone(); Py::Bool({ let t717 = v_outputs.clone(); if !py::contains(&t717, &t716)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_744(mut v_kw: Py) -> R {
    return Ok({ let t763 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_bmr_mifflin(py::getitem(&t763, &Py::str("weight"))?, py::getitem(&t763, &Py::str("height"))?, py::getitem(&t763, &Py::str("age"))?, py::getitem(&t763, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_770(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t772: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_744(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t772 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t773 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t801 = (t773).items()?;
        if t801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t801[0].clone();
        v_w = t801[1].clone();
        if ({ let t774 = v_k.clone(); Py::Bool({ let t775 = v_outputs.clone(); if !py::contains(&t775, &t774)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_802(mut v_kw: Py) -> R {
    return Ok({ let t815 = f__from_si(Py::dict(vec![(Py::str("density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)]))]), v_kw.clone())?; f_body_fat_density(py::getitem(&t815, &Py::str("density"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_822(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t824: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_802(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t824 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t825 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t853 = (t825).items()?;
        if t853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t853[0].clone();
        v_w = t853[1].clone();
        if ({ let t826 = v_k.clone(); Py::Bool({ let t827 = v_outputs.clone(); if !py::contains(&t827, &t826)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_854(mut v_kw: Py) -> R {
    return Ok({ let t876 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_compartments(py::getitem(&t876, &Py::str("weight"))?, py::method(&t876, "get", &[Py::str("TBW_fraction"), Py::Float(0.6)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_883(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t885: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_854(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t885 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t886 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t914 = (t886).items()?;
        if t914.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t914[0].clone();
        v_w = t914[1].clone();
        if ({ let t887 = v_k.clone(); Py::Bool({ let t888 = v_outputs.clone(); if !py::contains(&t888, &t887)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_915(mut v_kw: Py) -> R {
    return Ok({ let t928 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_body_surface_area(py::getitem(&t928, &Py::str("weight"))?, py::getitem(&t928, &Py::str("height"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_935(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t937: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_915(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t937 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t938 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t966 = (t938).items()?;
        if t966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t966[0].clone();
        v_w = t966[1].clone();
        if ({ let t939 = v_k.clone(); Py::Bool({ let t940 = v_outputs.clone(); if !py::contains(&t940, &t939)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_967(mut v_kw: Py) -> R {
    return Ok({ let t984 = f__from_si(Py::dict(vec![(Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PECO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_bohr_dead_space(py::getitem(&t984, &Py::str("PaCO2"))?, py::getitem(&t984, &Py::str("PECO2"))?, py::getitem(&t984, &Py::str("VT"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_991(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t993: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_967(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t993 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t994 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1022 = (t994).items()?;
        if t1022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1022[0].clone();
        v_w = t1022[1].clone();
        if ({ let t995 = v_k.clone(); Py::Bool({ let t996 = v_outputs.clone(); if !py::contains(&t996, &t995)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
