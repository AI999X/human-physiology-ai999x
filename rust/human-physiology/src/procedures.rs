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
        "hp_egfr_ckd_epi_2021" => {
            let r = f_egfr_ckd_epi_2021(match v.get("Scr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sex") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_egfr_ckd_epi_2021_cys" => {
            let r = s_1403({ let mut d = vec![]; for k in ["Scr", "Scys", "age", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_elimination_half_life" => {
            let r = s_1464({ let mut d = vec![]; for k in ["CL", "Vd", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fev1_fvc_ratio" => {
            let r = s_1520({ let mut d = vec![]; for k in ["FEV1", "FVC"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1542(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_cardiac_output" => {
            let r = s_1574({ let mut d = vec![]; for k in ["VO2", "CaO2", "CvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_diffusion" => {
            let r = s_1630({ let mut d = vec![]; for k in ["D", "A", "dC", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fick_saturations" => {
            let r = s_1682({ let mut d = vec![]; for k in ["VO2", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_filtration_fraction" => {
            let r = s_1749({ let mut d = vec![]; for k in ["GFR", "RPF"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1771(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fluid_shift" => {
            let r = s_1803({ let mut d = vec![]; for k in ["TBW", "Posm", "V_added", "C_added", "ICF_fraction"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_fractional_excretion" => {
            let r = f_fractional_excretion(match v.get("Ux") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Px") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ucr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pcr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_free_water_clearance" => {
            let r = s_1913({ let mut d = vec![]; for k in ["V", "Uosm", "Posm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1933(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gas_volume_conversion" => {
            let r = s_1965({ let mut d = vec![]; for k in ["V", "T", "Patm", "PH2O"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1996(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ghk_voltage" => {
            let r = s_2028({ let mut d = vec![]; for k in ["PK", "PNa", "PCl", "K_o", "K_i", "Na_o", "Na_i", "Cl_o", "Cl_i", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2055(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_gibbs_donnan" => {
            let r = s_2087({ let mut d = vec![]; for k in ["A", "c", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_glucose_titration" => {
            let r = s_2142({ let mut d = vec![]; for k in ["GFR", "Pglucose", "Tm"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2164(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_heart_rate_reserve" => {
            let r = f_heart_rate_reserve(match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HR_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intensity") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_fit" => {
            let r = s_2254({ let mut d = vec![]; for k in ["P1", "S1", "P2", "S2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_force_velocity" => {
            let r = s_2323({ let mut d = vec![]; for k in ["F0", "a", "b", "v"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2347(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_po2" => {
            let r = s_2379({ let mut d = vec![]; for k in ["SO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2408(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hill_saturation" => {
            let r = s_2440({ let mut d = vec![]; for k in ["PO2", "P50", "n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2462(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_pressure" => {
            let r = s_2494({ let mut d = vec![]; for k in ["h", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2514(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_hydrostatic_weighing" => {
            let r = s_2546({ let mut d = vec![]; for k in ["mass_air", "mass_water", "water_density", "RV", "GI"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2579(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ionic_current" => {
            let r = s_2611({ let mut d = vec![]; for k in ["g", "Vm", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_laplace_wall_tension" => {
            let r = s_2663({ let mut d = vec![]; for k in ["P", "r", "shape", "h"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lean_body_mass_boer" => {
            let r = s_2721({ let mut d = vec![]; for k in ["weight", "height", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2748(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_length_constant" => {
            let r = s_2780({ let mut d = vec![]; for k in ["Rm", "Ri", "d", "x"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2804(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_lung_capacities" => {
            let r = s_2836({ let mut d = vec![]; for k in ["TV", "IRV", "ERV", "RV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_mean_arterial_pressure" => {
            let r = s_2890({ let mut d = vec![]; for k in ["SBP", "DBP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2912(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_charge" => {
            let r = s_2944({ let mut d = vec![]; for k in ["Cm", "dV", "area"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2966(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_membrane_time_constant" => {
            let r = s_2998({ let mut d = vec![]; for k in ["Rm", "Cm", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3022(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_met_energy" => {
            let r = s_3054({ let mut d = vec![]; for k in ["MET", "weight", "duration"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3074(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_minute_ventilation" => {
            let r = s_3106({ let mut d = vec![]; for k in ["VT", "RR", "VD"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_moens_korteweg" => {
            let r = s_3162({ let mut d = vec![]; for k in ["E", "h", "D", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3182(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_muscle_max_power" => {
            let r = s_3214({ let mut d = vec![]; for k in ["F0", "a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3234(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nernst_potential" => {
            let r = s_3266({ let mut d = vec![]; for k in ["z", "Cout", "Cin", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_nerve_conduction_velocity" => {
            let r = s_3327({ let mut d = vec![]; for k in ["distance", "latency_proximal", "latency_distal"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3351(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_osmotic_pressure" => {
            let r = s_3383({ let mut d = vec![]; for k in ["osm", "T"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3406(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_consumption" => {
            let r = s_3438({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "SvO2", "PaO2", "PvO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_content" => {
            let r = s_3505({ let mut d = vec![]; for k in ["Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3537(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_oxygen_delivery" => {
            let r = s_3569({ let mut d = vec![]; for k in ["CO", "Hb", "SaO2", "PaO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3600(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pah_renal_flow" => {
            let r = s_3632({ let mut d = vec![]; for k in ["U_PAH", "V", "P_PAH", "Hct", "E"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3662(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pco2_from_ph" => {
            let r = s_3694({ let mut d = vec![]; for k in ["pH", "HCO3"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_ph_henderson_hasselbalch" => {
            let r = s_3749({ let mut d = vec![]; for k in ["HCO3", "PaCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3769(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_plasma_osmolality" => {
            let r = s_3801({ let mut d = vec![]; for k in ["Na", "glucose", "BUN", "measured"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3825(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_flow" => {
            let r = s_3857({ let mut d = vec![]; for k in ["dP", "eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3877(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_poiseuille_resistance" => {
            let r = s_3909({ let mut d = vec![]; for k in ["eta", "L", "r"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3929(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_pulmonary_vascular_resistance" => {
            let r = s_3961({ let mut d = vec![]; for k in ["mPAP", "PCWP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qp_qs" => {
            let r = f_qp_qs(match v.get("SaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpvO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("SpaO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4041(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qrs_axis" => {
            let r = f_qrs_axis(match v.get("lead_I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("aVF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4088(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_qt_correction" => {
            let r = s_4120({ let mut d = vec![]; for k in ["QT", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4140(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_radius_flow_ratio" => {
            let r = f_radius_flow_ratio(match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_renal_clearance" => {
            let r = s_4216({ let mut d = vec![]; for k in ["U", "V", "P"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4238(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_mechanics" => {
            let r = s_4270({ let mut d = vec![]; for k in ["VT", "Ppeak", "Pplat", "PEEP", "flow"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_respiratory_time_constant" => {
            let r = s_4327({ let mut d = vec![]; for k in ["R", "C", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4351(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_po2" => {
            let r = f_severinghaus_po2(match v.get("SO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4403(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_severinghaus_saturation" => {
            let r = s_4435({ let mut d = vec![]; for k in ["PO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_shunt_fraction" => {
            let r = f_shunt_fraction(match v.get("CcO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CaO2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CvO2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4503(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_solution_osmolarity" => {
            let r = s_4535({ let mut d = vec![]; for k in ["conc", "molar_mass", "i", "phi"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_starling_filtration" => {
            let r = s_4597({ let mut d = vec![]; for k in ["Pc", "Pi", "pic", "pii", "Kf", "sigma"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4628(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_strength_duration" => {
            let r = s_4660({ let mut d = vec![]; for k in ["rheobase", "chronaxie", "t"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4680(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_volume_ef" => {
            let r = s_4712({ let mut d = vec![]; for k in ["EDV", "ESV"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4736(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_stroke_work" => {
            let r = s_4768({ let mut d = vec![]; for k in ["SV", "MAP", "HR"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4790(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_systemic_vascular_resistance" => {
            let r = s_4822({ let mut d = vec![]; for k in ["MAP", "CVP", "CO"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4846(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_teichholz_volumes" => {
            let r = s_4878({ let mut d = vec![]; for k in ["LVIDd", "LVIDs"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4901(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_tubular_handling" => {
            let r = s_4933({ let mut d = vec![]; for k in ["GFR", "Px", "Ux", "V"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vascular_compliance" => {
            let r = s_4988({ let mut d = vec![]; for k in ["dV", "dP"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5008(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessel_reynolds" => {
            let r = s_5040({ let mut d = vec![]; for k in ["rho", "v", "D", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5060(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_vessels_series_parallel" => {
            let r = f_network_resistance(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_virtual_po2" => {
            let r = s_5145({ let mut d = vec![]; for k in ["PO2", "T", "pH", "PCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5172(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_watson_total_body_water" => {
            let r = s_5204({ let mut d = vec![]; for k in ["age", "height", "weight", "sex"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5230(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_weir_energy_expenditure" => {
            let r = s_5262({ let mut d = vec![]; for k in ["VO2", "VCO2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5282(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hp_womersley_number" => {
            let r = s_5314({ let mut d = vec![]; for k in ["r", "HR", "rho", "eta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t5342 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t5342.truthy() { t5342 } else { { let t5341 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t5341.truthy() { t5341 } else { { let t5340 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t5340.truthy() { t5340 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t5343 = v_a.clone(); Py::Bool({ let t5344 = v_b.clone(); if py::eq(&t5343, &t5344) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t5345 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5345.truthy() { t5345 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5349 = py::not(&{ let t5346 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5346.truthy() { t5346 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5349.truthy() { t5349 } else { { let t5347 = py::len(&v_a.clone())?; Py::Bool({ let t5348 = py::len(&v_b.clone())?; if py::ne(&t5347, &t5348) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        v_w = Py::Float(0.0);
        for t5350 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t5351 = (t5350).items()?;
            if t5351.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t5351[0].clone();
            v_y = t5351[1].clone();
            v_w = py::minmax(&[v_w.clone(), f__diff(v_x.clone(), v_y.clone())?], true)?;
        }
        return Ok(v_w.clone());
    }
    if ({ let t5356 = { let t5352 = v_a.clone(); Py::Bool({ let t5353 = Py::None; if py::is(&t5352, &t5353) { true } else { false } }) }; if t5356.truthy() { t5356 } else { { let t5354 = v_b.clone(); Py::Bool({ let t5355 = Py::None; if py::is(&t5354, &t5355) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5357 = v_a.clone(); Py::Bool({ let t5358 = v_b.clone(); if py::is(&t5357, &t5358) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t5360: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5359 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5359[0].clone();
            v_b = t5359[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5360 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t5361 = v_a.clone(); Py::Bool({ let t5362 = v_b.clone(); if py::eq(&t5361, &t5362) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5363 = v_d.clone(); Py::Bool({ let t5364 = v_d.clone(); if py::ne(&t5363, &t5364) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fr(mut v_v: Py, mut v_name: Py, mut v_open_lo: Py, mut v_open_hi: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t4613 = (if (v_open_lo.clone()).truthy() { { let t4605 = v_v.clone(); Py::Bool({ let t4606 = Py::Int(0i64); if py::gt(&t4605, &t4606)? { true } else { false } }) } } else { { let t4607 = v_v.clone(); Py::Bool({ let t4608 = Py::Int(0i64); if py::ge(&t4607, &t4608)? { true } else { false } }) } }); if !t4613.truthy() { t4613 } else { (if (v_open_hi.clone()).truthy() { { let t4609 = v_v.clone(); Py::Bool({ let t4610 = Py::Int(1i64); if py::lt(&t4609, &t4610)? { true } else { false } }) } } else { { let t4611 = v_v.clone(); Py::Bool({ let t4612 = Py::Int(1i64); if py::le(&t4611, &t4612)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
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
    for t5328 in py::iter(&(py::method(&v_conv.clone(), "items", &[])?))? {
        let t5332 = (t5328).items()?;
        if t5332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5332[0].clone();
        let t5333 = (t5332[1].clone()).items()?;
        if t5333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f = t5333[0].clone();
        v_off = t5333[1].clone();
        v_v = py::method(&v_out.clone(), "get", &[v_k.clone()])?;
        if ({ let t5331 = { let t5329 = v_v.clone(); Py::Bool({ let t5330 = Py::None; if !py::is(&t5329, &t5330) { true } else { false } }) }; if !t5331.truthy() { t5331 } else { py::not(&Py::Bool(matches!(v_v.clone(), Py::Str(_) | Py::Bool(_)))) } }).truthy() {
            py::setitem(&v_out.clone(), &v_k.clone(), py::truediv(&py::sub(&v_v.clone(), &v_off.clone())?, &v_f.clone())?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int_ok(mut v_v: Py, mut v_name: Py) -> R {
    v_v = (if ({ let t4543 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4543.truthy() { t4543 } else { { let t4542 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4542.truthy() { t4542 } else { { let t4540 = py::float(&v_v.clone())?; Py::Bool({ let t4541 = py::int(&v_v.clone())?; if py::eq(&t4540, &t4541) { true } else { false } }) } } } } }).truthy() { py::int(&v_v.clone())? } else { Py::None });
    let _ = f__need({ let t4549 = { let t4544 = v_v.clone(); Py::Bool({ let t4545 = Py::None; if !py::is(&t4544, &t4545) { true } else { false } }) }; if !t4549.truthy() { t4549 } else { { let t4546 = Py::Int(1i64); Py::Bool({ let t4547 = v_v.clone(); if py::le(&t4546, &t4547)? { { let t4548 = Py::Int(6i64); if py::le(&t4547, &t4548)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of particles from 1 to 6".to_string()].concat()))?;
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
    let _ = f__need({ let t4938 = v_v.clone(); Py::Bool({ let t4939 = Py::Int(0i64); if py::ge(&t4938, &t4939)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t5316 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5316.truthy() { t5316 } else { { let t5315 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5315.truthy() { t5315 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
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
    let _ = f__need({ let t5317 = v_v.clone(); Py::Bool({ let t5318 = Py::Int(0i64); if py::gt(&t5317, &t5318)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
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
    if ({ let t5323 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5323.truthy() { t5323 } else { { let t5322 = Py::Bool(matches!(v_v.clone(), Py::Str(_))); if t5322.truthy() { t5322 } else { { let t5320 = v_v.clone(); Py::Bool({ let t5321 = Py::None; if py::is(&t5320, &t5321) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        return Ok(f__r12(v_v.clone())?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5324 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5324;
out.push(f__rnd(v_x.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5325 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t5326 = (t5325).items()?;
if t5326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5326[0].clone();
v_x = t5326[1].clone();
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
    let _ = f__need({ let t5210 = v_v.clone(); Py::Bool({ let t5211 = Py::tuple(vec![Py::str("female"), Py::str("male")]); if py::contains(&t5211, &t5210)? { true } else { false } }) }, Py::str("sex must be female or male"))?;
    return Ok({ let t5212 = v_v.clone(); Py::Bool({ let t5213 = Py::str("female"); if py::eq(&t5212, &t5213) { true } else { false } }) });
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
    v_out = { let mut out = vec![]; for t5092 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5092;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) };
    let _ = f__need({ let t5095 = v_lo.clone(); Py::Bool({ let t5096 = py::len(&v_out.clone())?; if py::le(&t5095, &t5096)? { { let t5097 = v_hi.clone(); if py::le(&t5096, &t5097)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " needs between ".to_string(), py::format(&v_lo.clone(), "")?, " and ".to_string(), py::format(&v_hi.clone(), "")?, " entries".to_string()].concat()))?;
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
    let t4993 = (Py::tuple(vec![f__pos(v_dV.clone(), Py::str("dV"))?, f__pos(v_dP.clone(), Py::str("dP"))?])).items()?;
    if t4993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dV = t4993[0].clone();
    v_dP = t4993[1].clone();
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
fn f_egfr_ckd_epi_2021(mut v_Scr: Py, mut v_age: Py, mut v_sex: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_fem: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1355 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1355[0].clone();
    v_age = t1355[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1360 = Py::Int(18i64); Py::Bool({ let t1361 = v_age.clone(); if py::le(&t1360, &t1361)? { { let t1362 = Py::Int(120i64); if py::le(&t1361, &t1362)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1363 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.241))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.302))?]) })).items()?;
    if t1363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1363[0].clone();
    v_a = t1363[1].clone();
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
    let t1408 = (Py::tuple(vec![f__pos(v_Scr.clone(), Py::str("Scr"))?, f__pos(v_Scys.clone(), Py::str("Scys"))?, f__num(v_age.clone(), Py::str("age"))?])).items()?;
    if t1408.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Scr = t1408[0].clone();
    v_Scys = t1408[1].clone();
    v_age = t1408[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t1413 = Py::Int(18i64); Py::Bool({ let t1414 = v_age.clone(); if py::le(&t1413, &t1414)? { { let t1415 = Py::Int(120i64); if py::le(&t1414, &t1415)? { true } else { false } } } else { false } }) }, Py::str("the CKD-EPI equation applies to ages 18 to 120"))?;
    let t1416 = ((if (v_fem.clone()).truthy() { Py::tuple(vec![Py::Float(0.7), py::neg(&Py::Float(0.219))?]) } else { Py::tuple(vec![Py::Float(0.9), py::neg(&Py::Float(0.144))?]) })).items()?;
    if t1416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1416[0].clone();
    v_a = t1416[1].clone();
    let t1417 = (Py::tuple(vec![py::truediv(&v_Scr.clone(), &v_k.clone())?, py::truediv(&v_Scys.clone(), &Py::Float(0.8))?])).items()?;
    if t1417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1417[0].clone();
    v_c = t1417[1].clone();
    v_v = py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(135.0), &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], false)?, &v_a.clone())?)?, &py::pow(&py::minmax(&[v_s.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.544))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], false)?, &py::neg(&Py::Float(0.323))?)?)?, &py::pow(&py::minmax(&[v_c.clone(), Py::Float(1.0)], true)?, &py::neg(&Py::Float(0.778))?)?)?, &py::pow(&Py::Float(0.9961), &v_age.clone())?)?, &(if (v_fem.clone()).truthy() { Py::Float(0.963) } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eGFR"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_elimination_half_life(mut v_CL: Py, mut v_Vd: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t1469 = (Py::tuple(vec![f__pos(v_CL.clone(), Py::str("CL"))?, f__pos(v_Vd.clone(), Py::str("Vd"))?])).items()?;
    if t1469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_CL = t1469[0].clone();
    v_Vd = t1469[1].clone();
    v_k = py::truediv(&v_CL.clone(), &v_Vd.clone())?;
    v_out = Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("half_life"), py::truediv(&py::math1("log", &Py::Float(2.0))?, &v_k.clone())?)]);
    if ({ let t1470 = v_t.clone(); Py::Bool({ let t1471 = Py::None; if !py::is(&t1470, &t1471) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("fraction_remaining"), py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &f__nonneg(v_t.clone(), Py::str("t"))?)?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fev1_fvc(mut v_FEV1: Py, mut v_FVC: Py) -> R {
    let t1525 = (Py::tuple(vec![f__pos(v_FEV1.clone(), Py::str("FEV1"))?, f__pos(v_FVC.clone(), Py::str("FVC"))?])).items()?;
    if t1525.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FEV1 = t1525[0].clone();
    v_FVC = t1525[1].clone();
    let _ = f__need({ let t1526 = v_FEV1.clone(); Py::Bool({ let t1527 = v_FVC.clone(); if py::le(&t1526, &t1527)? { true } else { false } }) }, Py::str("FEV1 cannot exceed FVC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("ratio"), py::truediv(&v_FEV1.clone(), &v_FVC.clone())?), (Py::str("percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_FEV1.clone())?, &v_FVC.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_cardiac_output(mut v_VO2: Py, mut v_CaO2: Py, mut v_CvO2: Py) -> R {
    let mut v_d: Py = Py::None;
    let t1581 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t1581.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1581[0].clone();
    v_CaO2 = t1581[1].clone();
    v_CvO2 = t1581[2].clone();
    let _ = f__need({ let t1582 = v_CvO2.clone(); Py::Bool({ let t1583 = v_CaO2.clone(); if py::lt(&t1582, &t1583)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    v_d = py::sub(&v_CaO2.clone(), &v_CvO2.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&v_d.clone(), &Py::Float(10.0))?)?), (Py::str("avDO2"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_diffusion(mut v_D: Py, mut v_A: Py, mut v_dC: Py, mut v_T: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1635 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_A.clone(), Py::str("A"))?, f__num(v_dC.clone(), Py::str("dC"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t1635.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1635[0].clone();
    v_A = t1635[1].clone();
    v_dC = t1635[2].clone();
    v_T = t1635[3].clone();
    v_x = py::mul(&v_T.clone(), &Py::Float(0.0001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("J"), py::truediv(&py::mul(&py::mul(&v_D.clone(), &v_A.clone())?, &py::mul(&v_dC.clone(), &Py::Float(0.001))?)?, &v_x.clone())?), (Py::str("t_diffusion"), py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&Py::Float(2.0), &v_D.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_saturations(mut v_VO2: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t1689 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t1689.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t1689[0].clone();
    v_Hb = t1689[1].clone();
    v_PaO2 = t1689[2].clone();
    v_PvO2 = t1689[3].clone();
    let t1699 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t1699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t1699[0].clone();
    v_SvO2 = t1699[1].clone();
    let t1700 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t1700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t1700[0].clone();
    v_cv = t1700[1].clone();
    let _ = f__need({ let t1701 = v_cv.clone(); Py::Bool({ let t1702 = v_ca.clone(); if py::lt(&t1701, &t1702)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("CO"), py::truediv(&v_VO2.clone(), &py::mul(&py::sub(&v_ca.clone(), &v_cv.clone())?, &Py::Float(10.0))?)?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_fraction(mut v_GFR: Py, mut v_RPF: Py) -> R {
    let t1754 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__pos(v_RPF.clone(), Py::str("RPF"))?])).items()?;
    if t1754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t1754[0].clone();
    v_RPF = t1754[1].clone();
    let _ = f__need({ let t1755 = v_GFR.clone(); Py::Bool({ let t1756 = v_RPF.clone(); if py::lt(&t1755, &t1756)? { true } else { false } }) }, Py::str("GFR must be smaller than the renal plasma flow"))?;
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
    let t1810 = (Py::tuple(vec![f__pos(v_TBW.clone(), Py::str("TBW"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?, f__pos(v_V_added.clone(), Py::str("V_added"))?, f__nonneg(v_C_added.clone(), Py::str("C_added"))?])).items()?;
    if t1810.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TBW = t1810[0].clone();
    v_Posm = t1810[1].clone();
    v_V = t1810[2].clone();
    v_C = t1810[3].clone();
    v_f = f__fr(v_ICF_fraction.clone(), Py::str("ICF_fraction"), Py::Bool(true), Py::Bool(true))?;
    let t1820 = (Py::tuple(vec![py::mul(&v_f.clone(), &v_TBW.clone())?, py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &v_TBW.clone())?])).items()?;
    if t1820.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_icf0 = t1820[0].clone();
    v_ecf0 = t1820[1].clone();
    v_osm = py::truediv(&py::add(&py::mul(&v_TBW.clone(), &v_Posm.clone())?, &py::mul(&v_V.clone(), &v_C.clone())?)?, &py::add(&v_TBW.clone(), &v_V.clone())?)?;
    v_icf = py::truediv(&py::mul(&v_icf0.clone(), &v_Posm.clone())?, &v_osm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("osm_final"), v_osm.clone()), (Py::str("ICF"), v_icf.clone()), (Py::str("ECF"), py::sub(&py::add(&v_TBW.clone(), &v_V.clone())?, &v_icf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fractional_excretion(mut v_Ux: Py, mut v_Px: Py, mut v_Ucr: Py, mut v_Pcr: Py) -> R {
    let mut v_fe: Py = Py::None;
    let t1873 = (Py::tuple(vec![f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_Px.clone(), Py::str("Px"))?, f__pos(v_Ucr.clone(), Py::str("Ucr"))?, f__pos(v_Pcr.clone(), Py::str("Pcr"))?])).items()?;
    if t1873.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Ux = t1873[0].clone();
    v_Px = t1873[1].clone();
    v_Ucr = t1873[2].clone();
    v_Pcr = t1873[3].clone();
    v_fe = py::truediv(&py::mul(&v_Ux.clone(), &v_Pcr.clone())?, &py::mul(&v_Px.clone(), &v_Ucr.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("FE"), v_fe.clone()), (Py::str("FE_percent"), py::mul(&Py::Float(100.0), &v_fe.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_water_clearance(mut v_V: Py, mut v_Uosm: Py, mut v_Posm: Py) -> R {
    let mut v_cosm: Py = Py::None;
    let t1918 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_Uosm.clone(), Py::str("Uosm"))?, f__pos(v_Posm.clone(), Py::str("Posm"))?])).items()?;
    if t1918.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1918[0].clone();
    v_Uosm = t1918[1].clone();
    v_Posm = t1918[2].clone();
    v_cosm = py::truediv(&py::mul(&v_Uosm.clone(), &v_V.clone())?, &v_Posm.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Cosm"), v_cosm.clone()), (Py::str("CH2O"), py::sub(&v_V.clone(), &v_cosm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_volume_conversion(mut v_V: Py, mut v_T: Py, mut v_Patm: Py, mut v_PH2O: Py) -> R {
    let mut v_tk: Py = Py::None;
    let t1972 = (Py::tuple(vec![f__pos(v_V.clone(), Py::str("V"))?, f__num(v_T.clone(), Py::str("T"))?, f__pos(v_Patm.clone(), Py::str("Patm"))?, f__nonneg(v_PH2O.clone(), Py::str("PH2O"))?])).items()?;
    if t1972.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1972[0].clone();
    v_T = t1972[1].clone();
    v_Patm = t1972[2].clone();
    v_PH2O = t1972[3].clone();
    let _ = f__need({ let t1981 = { let t1973 = py::neg(&Py::Int(50i64))?; Py::Bool({ let t1974 = v_T.clone(); if py::lt(&t1973, &t1974)? { { let t1975 = Py::Int(60i64); if py::lt(&t1974, &t1975)? { true } else { false } } } else { false } }) }; if !t1981.truthy() { t1981 } else { { let t1980 = { let t1976 = v_Patm.clone(); Py::Bool({ let t1977 = v_PH2O.clone(); if py::gt(&t1976, &t1977)? { true } else { false } }) }; if !t1980.truthy() { t1980 } else { { let t1978 = v_Patm.clone(); Py::Bool({ let t1979 = Py::Int(47i64); if py::gt(&t1978, &t1979)? { true } else { false } }) } } } } }, Py::str("temperature -50..60 degC, Patm above the vapour pressures"))?;
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
    let t2035 = (Py::tuple(vec![f__pos(v_PK.clone(), Py::str("PK"))?, f__nonneg(v_PNa.clone(), Py::str("PNa"))?, f__nonneg(v_PCl.clone(), Py::str("PCl"))?])).items()?;
    if t2035.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PK = t2035[0].clone();
    v_PNa = t2035[1].clone();
    v_PCl = t2035[2].clone();
    v_vals = { let mut out = vec![]; for t2036 in py::iter(&(Py::tuple(vec![v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone()])))? { let mut v_x: Py; 
v_x = t2036;
out.push(f__pos(v_x.clone(), Py::str("concentrations"))?); } Py::list(out) };
    let t2037 = (v_vals.clone()).items()?;
    if t2037.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_K_o = t2037[0].clone();
    v_K_i = t2037[1].clone();
    v_Na_o = t2037[2].clone();
    v_Na_i = t2037[3].clone();
    v_Cl_o = t2037[4].clone();
    v_Cl_i = t2037[5].clone();
    v_T = f__num(v_T.clone(), Py::str("T"))?;
    let _ = f__need({ let t2038 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2039 = v_T.clone(); if py::lt(&t2038, &t2039)? { { let t2040 = Py::Int(100i64); if py::lt(&t2039, &t2040)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2092 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("A"))?, f__pos(v_c.clone(), Py::str("c"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t2092.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2092[0].clone();
    v_c = t2092[1].clone();
    v_T = t2092[2].clone();
    let _ = f__need({ let t2093 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t2094 = v_T.clone(); if py::lt(&t2093, &t2094)? { { let t2095 = Py::Int(100i64); if py::lt(&t2094, &t2095)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
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
    let t2149 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Pglucose.clone(), Py::str("Pglucose"))?, f__pos(v_Tm.clone(), Py::str("Tm"))?])).items()?;
    if t2149.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t2149[0].clone();
    v_P = t2149[1].clone();
    v_Tm = t2149[2].clone();
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
    let t2209 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_HR_rest.clone(), Py::str("HR_rest"))?, f__fr(v_intensity.clone(), Py::str("intensity"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2209.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2209[0].clone();
    v_r = t2209[1].clone();
    v_f = t2209[2].clone();
    let _ = f__need({ let t2210 = Py::Int(5i64); Py::Bool({ let t2211 = v_A.clone(); if py::le(&t2210, &t2211)? { { let t2212 = Py::Int(100i64); if py::le(&t2211, &t2212)? { true } else { false } } } else { false } }) }, Py::str("age must be between 5 and 100"))?;
    v_mx = py::sub(&Py::Float(220.0), &v_A.clone())?;
    let _ = f__need({ let t2213 = v_r.clone(); Py::Bool({ let t2214 = v_mx.clone(); if py::lt(&t2213, &t2214)? { true } else { false } }) }, Py::str("the resting heart rate must be below the estimated maximum"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("HRmax"), v_mx.clone()), (Py::str("HRmax_Tanaka"), py::sub(&Py::Float(208.0), &py::mul(&Py::Float(0.7), &v_A.clone())?)?), (Py::str("HRR"), py::sub(&v_mx.clone(), &v_r.clone())?), (Py::str("target"), py::add(&v_r.clone(), &py::mul(&v_f.clone(), &py::sub(&v_mx.clone(), &v_r.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_fit(mut v_P1: Py, mut v_S1: Py, mut v_P2: Py, mut v_S2: Py) -> R {
    let mut v_n: Py = Py::None;
    let t2259 = (Py::tuple(vec![f__pos(v_P1.clone(), Py::str("P1"))?, f__pos(v_P2.clone(), Py::str("P2"))?])).items()?;
    if t2259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P1 = t2259[0].clone();
    v_P2 = t2259[1].clone();
    let t2269 = (Py::tuple(vec![f__fr(v_S1.clone(), Py::str("S1"), Py::Bool(true), Py::Bool(true))?, f__fr(v_S2.clone(), Py::str("S2"), Py::Bool(true), Py::Bool(true))?])).items()?;
    if t2269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S1 = t2269[0].clone();
    v_S2 = t2269[1].clone();
    let _ = f__need({ let t2274 = { let t2270 = v_P1.clone(); Py::Bool({ let t2271 = v_P2.clone(); if py::ne(&t2270, &t2271) { true } else { false } }) }; if !t2274.truthy() { t2274 } else { { let t2272 = v_S1.clone(); Py::Bool({ let t2273 = v_S2.clone(); if py::ne(&t2272, &t2273) { true } else { false } }) } } }, Py::str("the two points must differ"))?;
    let _ = f__need({ let t2275 = py::mul(&py::sub(&v_P2.clone(), &v_P1.clone())?, &py::sub(&v_S2.clone(), &v_S1.clone())?)?; Py::Bool({ let t2276 = Py::Int(0i64); if py::gt(&t2275, &t2276)? { true } else { false } }) }, Py::str("saturation must increase with PO2"))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&py::truediv(&v_S2.clone(), &py::sub(&Py::Int(1i64), &v_S2.clone())?)?, &py::truediv(&v_S1.clone(), &py::sub(&Py::Int(1i64), &v_S1.clone())?)?)?)?, &py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("P50"), py::mul(&v_P1.clone(), &py::pow(&py::truediv(&py::sub(&Py::Int(1i64), &v_S1.clone())?, &v_S1.clone())?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_force_velocity(mut v_F0: Py, mut v_a: Py, mut v_b: Py, mut v_v: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_vmax: Py = Py::None;
    let t2330 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?, f__nonneg(v_v.clone(), Py::str("v"))?])).items()?;
    if t2330.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t2330[0].clone();
    v_a = t2330[1].clone();
    v_b = t2330[2].clone();
    v_v = t2330[3].clone();
    v_vmax = py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?;
    let _ = f__need({ let t2331 = v_v.clone(); Py::Bool({ let t2332 = v_vmax.clone(); if py::le(&t2331, &t2332)? { true } else { false } }) }, Py::str("the shortening velocity cannot exceed vmax = b F0/a"))?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("F"), v_F.clone()), (Py::str("power"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), v_vmax.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_po2(mut v_SO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let t2393 = (Py::tuple(vec![f__fr(v_SO2.clone(), Py::str("SO2"), Py::Bool(true), Py::Bool(true))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2393.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_SO2 = t2393[0].clone();
    v_P50 = t2393[1].clone();
    v_n = t2393[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO2"), py::mul(&v_P50.clone(), &py::pow(&py::truediv(&v_SO2.clone(), &py::sub(&Py::Float(1.0), &v_SO2.clone())?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_saturation(mut v_PO2: Py, mut v_P50: Py, mut v_n: Py) -> R {
    let mut v_x: Py = Py::None;
    let t2447 = (Py::tuple(vec![f__nonneg(v_PO2.clone(), Py::str("PO2"))?, f__pos(v_P50.clone(), Py::str("P50"))?, f__pos(v_n.clone(), Py::str("n"))?])).items()?;
    if t2447.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t2447[0].clone();
    v_P50 = t2447[1].clone();
    v_n = t2447[2].clone();
    v_x = py::pow(&py::truediv(&v_PO2.clone(), &v_P50.clone())?, &v_n.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SO2"), py::truediv(&v_x.clone(), &py::add(&Py::Float(1.0), &v_x.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrostatic_column(mut v_h: Py, mut v_rho: Py) -> R {
    let mut v_p: Py = Py::None;
    let t2499 = (Py::tuple(vec![f__num(v_h.clone(), Py::str("h"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t2499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_h = t2499[0].clone();
    v_rho = t2499[1].clone();
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
    let t2553 = (Py::tuple(vec![f__pos(v_mass_air.clone(), Py::str("mass_air"))?, f__num(v_mass_water.clone(), Py::str("mass_water"))?, f__pos(v_water_density.clone(), Py::str("water_density"))?, f__nonneg(v_RV.clone(), Py::str("RV"))?, f__nonneg(v_GI.clone(), Py::str("GI"))?])).items()?;
    if t2553.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Ma = t2553[0].clone();
    v_Mw = t2553[1].clone();
    v_rho = t2553[2].clone();
    v_RV = t2553[3].clone();
    v_GI = t2553[4].clone();
    let _ = f__need({ let t2559 = { let t2554 = v_Mw.clone(); Py::Bool({ let t2555 = v_Ma.clone(); if py::lt(&t2554, &t2555)? { true } else { false } }) }; if !t2559.truthy() { t2559 } else { { let t2556 = Py::Float(0.95); Py::Bool({ let t2557 = v_rho.clone(); if py::lt(&t2556, &t2557)? { { let t2558 = Py::Float(1.0); if py::le(&t2557, &t2558)? { true } else { false } } } else { false } }) } } }, Py::str("underwater mass below the mass in air; water density 0.95-1.0 g/mL"))?;
    v_V = py::sub(&py::sub(&py::truediv(&py::sub(&v_Ma.clone(), &v_Mw.clone())?, &v_rho.clone())?, &v_RV.clone())?, &v_GI.clone())?;
    let _ = f__need({ let t2560 = v_V.clone(); Py::Bool({ let t2561 = Py::Int(0i64); if py::gt(&t2560, &t2561)? { true } else { false } }) }, Py::str("the computed body volume is not positive"))?;
    v_D = py::truediv(&v_Ma.clone(), &v_V.clone())?;
    let _ = f__need({ let t2562 = Py::Float(0.9); Py::Bool({ let t2563 = v_D.clone(); if py::lt(&t2562, &t2563)? { { let t2564 = Py::Float(1.2); if py::lt(&t2563, &t2564)? { true } else { false } } } else { false } }) }, Py::str("the computed density is outside 0.9-1.2 g/mL"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("volume"), v_V.clone()), (Py::str("density"), v_D.clone()), (Py::str("fat_Siri"), py::sub(&py::truediv(&Py::Float(495.0), &v_D.clone())?, &Py::Float(450.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ionic_current(mut v_g: Py, mut v_Vm: Py, mut v_E: Py) -> R {
    let t2616 = (Py::tuple(vec![f__nonneg(v_g.clone(), Py::str("g"))?, f__num(v_Vm.clone(), Py::str("Vm"))?, f__num(v_E.clone(), Py::str("E"))?])).items()?;
    if t2616.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2616[0].clone();
    v_Vm = t2616[1].clone();
    v_E = t2616[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I"), py::mul(&v_g.clone(), &py::sub(&v_Vm.clone(), &v_E.clone())?)?), (Py::str("driving_force"), py::sub(&v_Vm.clone(), &v_E.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_wall_tension(mut v_P: Py, mut v_r: Py, mut v_shape: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2668 = (Py::tuple(vec![f__pos(v_P.clone(), Py::str("P"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t2668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2668[0].clone();
    v_r = t2668[1].clone();
    let _ = f__need({ let t2669 = v_shape.clone(); Py::Bool({ let t2670 = Py::tuple(vec![Py::str("cylinder"), Py::str("sphere")]); if py::contains(&t2670, &t2669)? { true } else { false } }) }, Py::str("shape must be cylinder or sphere"))?;
    v_T = py::mul(&py::truediv(&py::mul(&py::mul(&v_P.clone(), &c_MMHG()?)?, &v_r.clone())?, &Py::Float(100.0))?, &(if ({ let t2671 = v_shape.clone(); Py::Bool({ let t2672 = Py::str("cylinder"); if py::eq(&t2671, &t2672) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.5) }))?;
    v_out = Py::dict(vec![(Py::str("T"), v_T.clone())]);
    if ({ let t2673 = v_h.clone(); Py::Bool({ let t2674 = Py::None; if !py::is(&t2673, &t2674) { true } else { false } }) }).truthy() {
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
    let t2726 = (Py::tuple(vec![f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_height.clone(), Py::str("height"))?])).items()?;
    if t2726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_W = t2726[0].clone();
    v_H = t2726[1].clone();
    v_fem = f__sex(v_sex.clone())?;
    v_v = (if (v_fem.clone()).truthy() { py::sub(&py::add(&py::mul(&Py::Float(0.252), &v_W.clone())?, &py::mul(&Py::Float(0.473), &v_H.clone())?)?, &Py::Float(48.3))? } else { py::sub(&py::add(&py::mul(&Py::Float(0.407), &v_W.clone())?, &py::mul(&Py::Float(0.267), &v_H.clone())?)?, &Py::Float(19.2))? });
    let _ = f__need({ let t2731 = Py::Int(0i64); Py::Bool({ let t2732 = v_v.clone(); if py::lt(&t2731, &t2732)? { { let t2733 = v_W.clone(); if py::lt(&t2732, &t2733)? { true } else { false } } } else { false } }) }, Py::str("the formula gives no valid lean mass for these values"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("LBM"), v_v.clone()), (Py::str("fat_mass"), py::sub(&v_W.clone(), &v_v.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_length_constant(mut v_Rm: Py, mut v_Ri: Py, mut v_d: Py, mut v_x: Py) -> R {
    let mut v_lam: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t2785 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Ri.clone(), Py::str("Ri"))?, f__pos(v_d.clone(), Py::str("d"))?])).items()?;
    if t2785.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t2785[0].clone();
    v_Ri = t2785[1].clone();
    v_d = t2785[2].clone();
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&v_Rm.clone(), &py::mul(&v_d.clone(), &Py::Float(0.0001))?)?, &py::mul(&Py::Float(4.0), &v_Ri.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("lambda"), py::mul(&v_lam.clone(), &Py::Float(10.0))?)]);
    if ({ let t2786 = v_x.clone(); Py::Bool({ let t2787 = Py::None; if !py::is(&t2786, &t2787) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("attenuation"), py::math1("exp", &py::truediv(&py::neg(&py::truediv(&f__nonneg(v_x.clone(), Py::str("x"))?, &Py::Float(10.0))?)?, &v_lam.clone())?)?)?;
    }
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lung_volumes(mut v_TV: Py, mut v_IRV: Py, mut v_ERV: Py, mut v_RV: Py) -> R {
    let t2843 = (Py::tuple(vec![f__pos(v_TV.clone(), Py::str("TV"))?, f__nonneg(v_IRV.clone(), Py::str("IRV"))?, f__nonneg(v_ERV.clone(), Py::str("ERV"))?, f__pos(v_RV.clone(), Py::str("RV"))?])).items()?;
    if t2843.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_TV = t2843[0].clone();
    v_IRV = t2843[1].clone();
    v_ERV = t2843[2].clone();
    v_RV = t2843[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("IC"), py::add(&v_TV.clone(), &v_IRV.clone())?), (Py::str("FRC"), py::add(&v_ERV.clone(), &v_RV.clone())?), (Py::str("VC"), py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?), (Py::str("TLC"), py::add(&py::add(&py::add(&v_IRV.clone(), &v_TV.clone())?, &v_ERV.clone())?, &v_RV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_arterial_pressure(mut v_SBP: Py, mut v_DBP: Py) -> R {
    let t2895 = (Py::tuple(vec![f__pos(v_SBP.clone(), Py::str("SBP"))?, f__pos(v_DBP.clone(), Py::str("DBP"))?])).items()?;
    if t2895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SBP = t2895[0].clone();
    v_DBP = t2895[1].clone();
    let _ = f__need({ let t2896 = v_SBP.clone(); Py::Bool({ let t2897 = v_DBP.clone(); if py::gt(&t2896, &t2897)? { true } else { false } }) }, Py::str("the systolic pressure must exceed the diastolic"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("MAP"), py::add(&v_DBP.clone(), &py::truediv(&py::sub(&v_SBP.clone(), &v_DBP.clone())?, &Py::Float(3.0))?)?), (Py::str("PP"), py::sub(&v_SBP.clone(), &v_DBP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_charge(mut v_Cm: Py, mut v_dV: Py, mut v_area: Py) -> R {
    let mut v_q: Py = Py::None;
    let t2949 = (Py::tuple(vec![f__pos(v_Cm.clone(), Py::str("Cm"))?, f__num(v_dV.clone(), Py::str("dV"))?, f__pos(v_area.clone(), Py::str("area"))?])).items()?;
    if t2949.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cm = t2949[0].clone();
    v_dV = t2949[1].clone();
    v_area = t2949[2].clone();
    let _ = f__need({ let t2950 = v_dV.clone(); Py::Bool({ let t2951 = Py::Int(0i64); if py::ne(&t2950, &t2951) { true } else { false } }) }, Py::str("the voltage change must not be zero"))?;
    v_q = py::mul(&py::mul(&py::mul(&v_Cm.clone(), &Py::Float(1e-06))?, &py::mul(&v_area.clone(), &Py::Float(1e-08))?)?, &py::mul(&v_dV.clone(), &Py::Float(0.001))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q_pC"), py::mul(&v_q.clone(), &Py::Float(1000000000000.0))?), (Py::str("ions"), py::truediv(&py::abs(&v_q.clone())?, &c_E_CHARGE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_time_constant(mut v_Rm: Py, mut v_Cm: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t3003 = (Py::tuple(vec![f__pos(v_Rm.clone(), Py::str("Rm"))?, f__pos(v_Cm.clone(), Py::str("Cm"))?])).items()?;
    if t3003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rm = t3003[0].clone();
    v_Cm = t3003[1].clone();
    v_tau = py::mul(&py::mul(&v_Rm.clone(), &v_Cm.clone())?, &Py::Float(0.001))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone())]);
    if ({ let t3004 = v_t.clone(); Py::Bool({ let t3005 = Py::None; if !py::is(&t3004, &t3005) { true } else { false } }) }).truthy() {
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
    let t3059 = (Py::tuple(vec![f__pos(v_MET.clone(), Py::str("MET"))?, f__pos(v_weight.clone(), Py::str("weight"))?, f__pos(v_duration.clone(), Py::str("duration"))?])).items()?;
    if t3059.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MET = t3059[0].clone();
    v_W = t3059[1].clone();
    v_m = t3059[2].clone();
    v_vo2 = py::mul(&py::mul(&v_MET.clone(), &Py::Float(3.5))?, &v_W.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), v_vo2.clone()), (Py::str("kcal_per_min"), py::truediv(&v_vo2.clone(), &Py::Float(200.0))?), (Py::str("kcal"), py::mul(&py::truediv(&v_vo2.clone(), &Py::Float(200.0))?, &v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minute_ventilation(mut v_VT: Py, mut v_RR: Py, mut v_VD: Py) -> R {
    let t3113 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__pos(v_RR.clone(), Py::str("RR"))?, f__nonneg(v_VD.clone(), Py::str("VD"))?])).items()?;
    if t3113.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t3113[0].clone();
    v_RR = t3113[1].clone();
    v_VD = t3113[2].clone();
    let _ = f__need({ let t3114 = v_VD.clone(); Py::Bool({ let t3115 = v_VT.clone(); if py::lt(&t3114, &t3115)? { true } else { false } }) }, Py::str("the dead space must be smaller than the tidal volume"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VE"), py::truediv(&py::mul(&v_VT.clone(), &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VA"), py::truediv(&py::mul(&py::sub(&v_VT.clone(), &v_VD.clone())?, &v_RR.clone())?, &Py::Float(1000.0))?), (Py::str("VD_VT"), py::truediv(&v_VD.clone(), &v_VT.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moens_korteweg(mut v_E: Py, mut v_h: Py, mut v_D: Py, mut v_rho: Py) -> R {
    let t3167 = (Py::tuple(vec![f__pos(v_E.clone(), Py::str("E"))?, f__pos(v_h.clone(), Py::str("h"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_rho.clone(), Py::str("rho"))?])).items()?;
    if t3167.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3167[0].clone();
    v_h = t3167[1].clone();
    v_D = t3167[2].clone();
    v_rho = t3167[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("PWV"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&v_E.clone(), &Py::Float(1000.0))?, &py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_muscle_max_power(mut v_F0: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3219 = (Py::tuple(vec![f__pos(v_F0.clone(), Py::str("F0"))?, f__pos(v_a.clone(), Py::str("a"))?, f__pos(v_b.clone(), Py::str("b"))?])).items()?;
    if t3219.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F0 = t3219[0].clone();
    v_a = t3219[1].clone();
    v_b = t3219[2].clone();
    v_v = py::mul(&v_b.clone(), &py::sub(&py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&v_F0.clone(), &v_a.clone())?)?)?, &Py::Float(1.0))?)?;
    v_F = py::truediv(&py::sub(&py::mul(&v_F0.clone(), &v_b.clone())?, &py::mul(&v_a.clone(), &v_v.clone())?)?, &py::add(&v_v.clone(), &v_b.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("v_opt"), v_v.clone()), (Py::str("F_opt"), v_F.clone()), (Py::str("P_max"), py::mul(&v_F.clone(), &v_v.clone())?), (Py::str("vmax"), py::truediv(&py::mul(&v_b.clone(), &v_F0.clone())?, &v_a.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst(mut v_z: Py, mut v_Cout: Py, mut v_Cin: Py, mut v_T: Py) -> R {
    v_z = py::int(&v_z.clone())?;
    let t3271 = (Py::tuple(vec![f__pos(v_Cout.clone(), Py::str("Cout"))?, f__pos(v_Cin.clone(), Py::str("Cin"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3271.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cout = t3271[0].clone();
    v_Cin = t3271[1].clone();
    v_T = t3271[2].clone();
    let _ = f__need({ let t3277 = { let t3272 = v_z.clone(); Py::Bool({ let t3273 = Py::Int(0i64); if py::ne(&t3272, &t3273) { true } else { false } }) }; if !t3277.truthy() { t3277 } else { { let t3274 = py::neg(&Py::Int(3i64))?; Py::Bool({ let t3275 = v_z.clone(); if py::le(&t3274, &t3275)? { { let t3276 = Py::Int(3i64); if py::le(&t3275, &t3276)? { true } else { false } } } else { false } }) } } }, Py::str("the valence must be a nonzero whole number from -3 to 3"))?;
    let _ = f__need({ let t3278 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3279 = v_T.clone(); if py::lt(&t3278, &t3279)? { { let t3280 = Py::Int(100i64); if py::lt(&t3279, &t3280)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("E"), py::mul(&py::truediv(&f__rtf(v_T.clone())?, &v_z.clone())?, &py::math1("log", &py::truediv(&v_Cout.clone(), &v_Cin.clone())?)?)?), (Py::str("RT_F"), f__rtf(v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nerve_conduction_velocity(mut v_distance: Py, mut v_latency_proximal: Py, mut v_latency_distal: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_ld: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t3334 = (Py::tuple(vec![f__pos(v_distance.clone(), Py::str("distance"))?, f__pos(v_latency_proximal.clone(), Py::str("latency_proximal"))?, f__nonneg(v_latency_distal.clone(), Py::str("latency_distal"))?])).items()?;
    if t3334.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t3334[0].clone();
    v_lp = t3334[1].clone();
    v_ld = t3334[2].clone();
    let _ = f__need({ let t3335 = v_lp.clone(); Py::Bool({ let t3336 = v_ld.clone(); if py::gt(&t3335, &t3336)? { true } else { false } }) }, Py::str("the proximal latency must exceed the distal latency"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("NCV"), py::truediv(&v_D.clone(), &py::sub(&v_lp.clone(), &v_ld.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_network_resistance(mut v_R: Py, mut v_arrangement: Py) -> R {
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_R = f__vec(v_R.clone(), Py::str("R"), Py::Int(1i64), Py::Int(10i64))?;
    for t5098 in py::iter(&(v_R.clone()))? {
        v_x = t5098;
        let _ = f__need({ let t5099 = v_x.clone(); Py::Bool({ let t5100 = Py::Int(0i64); if py::gt(&t5099, &t5100)? { true } else { false } }) }, Py::str("every resistance must be positive"))?;
    }
    let _ = f__need({ let t5101 = v_arrangement.clone(); Py::Bool({ let t5102 = Py::tuple(vec![Py::str("series"), Py::str("parallel")]); if py::contains(&t5102, &t5101)? { true } else { false } }) }, Py::str("arrangement must be series or parallel"))?;
    v_tot = (if ({ let t5103 = v_arrangement.clone(); Py::Bool({ let t5104 = Py::str("series"); if py::eq(&t5103, &t5104) { true } else { false } }) }).truthy() { py::sum(&v_R.clone(), None)? } else { py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t5105 in py::iter(&(v_R.clone()))? { let mut v_x: Py; 
v_x = t5105;
out.push(py::truediv(&Py::Float(1.0), &v_x.clone())?); } Py::list(out) }, None)?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("R_total"), v_tot.clone()), (Py::str("G_total"), py::truediv(&Py::Float(1.0), &v_tot.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_osm: Py, mut v_T: Py) -> R {
    let mut v_p: Py = Py::None;
    let t3388 = (Py::tuple(vec![f__nonneg(v_osm.clone(), Py::str("osm"))?, f__num(v_T.clone(), Py::str("T"))?])).items()?;
    if t3388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_osm = t3388[0].clone();
    v_T = t3388[1].clone();
    let _ = f__need({ let t3389 = py::neg(&Py::Int(20i64))?; Py::Bool({ let t3390 = v_T.clone(); if py::lt(&t3389, &t3390)? { { let t3391 = Py::Int(100i64); if py::lt(&t3390, &t3391)? { true } else { false } } } else { false } }) }, Py::str("temperature must be between -20 and 100 degC"))?;
    v_p = py::mul(&py::mul(&v_osm.clone(), &c_R_GAS()?)?, &py::add(&v_T.clone(), &c_T0C()?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pi"), v_p.clone()), (Py::str("pi_mmHg"), py::truediv(&v_p.clone(), &c_MMHG()?)?), (Py::str("pi_atm"), py::truediv(&v_p.clone(), &Py::Float(101325.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_consumption(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_SvO2: Py, mut v_PaO2: Py, mut v_PvO2: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let t3445 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?, f__nonneg(v_PvO2.clone(), Py::str("PvO2"))?])).items()?;
    if t3445.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3445[0].clone();
    v_Hb = t3445[1].clone();
    v_PaO2 = t3445[2].clone();
    v_PvO2 = t3445[3].clone();
    let t3455 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t3455.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SaO2 = t3455[0].clone();
    v_SvO2 = t3455[1].clone();
    let t3456 = (Py::tuple(vec![f__o2c(v_Hb.clone(), v_SaO2.clone(), v_PaO2.clone())?, f__o2c(v_Hb.clone(), v_SvO2.clone(), v_PvO2.clone())?])).items()?;
    if t3456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t3456[0].clone();
    v_cv = t3456[1].clone();
    let _ = f__need({ let t3457 = v_cv.clone(); Py::Bool({ let t3458 = v_ca.clone(); if py::lt(&t3457, &t3458)? { true } else { false } }) }, Py::str("the venous oxygen content must be below the arterial"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("VO2"), py::mul(&py::mul(&v_CO.clone(), &py::sub(&v_ca.clone(), &v_cv.clone())?)?, &Py::Float(10.0))?), (Py::str("O2ER"), py::truediv(&py::sub(&v_ca.clone(), &v_cv.clone())?, &v_ca.clone())?), (Py::str("CaO2"), v_ca.clone()), (Py::str("CvO2"), v_cv.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_content(mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t3521 = (Py::tuple(vec![f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3521.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Hb = t3521[0].clone();
    v_SaO2 = t3521[1].clone();
    v_PaO2 = t3521[2].clone();
    let t3522 = (Py::tuple(vec![py::mul(&py::mul(&c_O2_HB()?, &v_Hb.clone())?, &v_SaO2.clone())?, py::mul(&c_O2_SOL()?, &v_PaO2.clone())?])).items()?;
    if t3522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3522[0].clone();
    v_d = t3522[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("CaO2"), py::add(&v_b.clone(), &v_d.clone())?), (Py::str("bound"), v_b.clone()), (Py::str("dissolved"), v_d.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oxygen_delivery(mut v_CO: Py, mut v_Hb: Py, mut v_SaO2: Py, mut v_PaO2: Py) -> R {
    let mut v_c: Py = Py::None;
    let t3585 = (Py::tuple(vec![f__pos(v_CO.clone(), Py::str("CO"))?, f__pos(v_Hb.clone(), Py::str("Hb"))?, f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__nonneg(v_PaO2.clone(), Py::str("PaO2"))?])).items()?;
    if t3585.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_CO = t3585[0].clone();
    v_Hb = t3585[1].clone();
    v_SaO2 = t3585[2].clone();
    v_PaO2 = t3585[3].clone();
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
    let t3637 = (Py::tuple(vec![f__pos(v_U_PAH.clone(), Py::str("U_PAH"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P_PAH.clone(), Py::str("P_PAH"))?])).items()?;
    if t3637.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t3637[0].clone();
    v_V = t3637[1].clone();
    v_P = t3637[2].clone();
    let t3647 = (Py::tuple(vec![f__fr(v_Hct.clone(), Py::str("Hct"), Py::Bool(false), Py::Bool(true))?, f__fr(v_E.clone(), Py::str("E"), Py::Bool(true), Py::Bool(false))?])).items()?;
    if t3647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Hct = t3647[0].clone();
    v_E = t3647[1].clone();
    v_erpf = py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?;
    v_rpf = py::truediv(&v_erpf.clone(), &v_E.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("eRPF"), v_erpf.clone()), (Py::str("RPF"), v_rpf.clone()), (Py::str("RBF"), py::truediv(&v_rpf.clone(), &py::sub(&Py::Float(1.0), &v_Hct.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pco2_from_ph(mut v_pH: Py, mut v_HCO3: Py) -> R {
    let t3699 = (Py::tuple(vec![f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_HCO3.clone(), Py::str("HCO3"))?])).items()?;
    if t3699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pH = t3699[0].clone();
    v_HCO3 = t3699[1].clone();
    let _ = f__need({ let t3700 = Py::Float(6.5); Py::Bool({ let t3701 = v_pH.clone(); if py::le(&t3700, &t3701)? { { let t3702 = Py::Float(8.0); if py::le(&t3701, &t3702)? { true } else { false } } } else { false } }) }, Py::str("pH must be between 6.5 and 8.0"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PaCO2"), py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &c_PK_CO2()?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_henderson(mut v_HCO3: Py, mut v_PaCO2: Py) -> R {
    let mut v_ph: Py = Py::None;
    let t3754 = (Py::tuple(vec![f__pos(v_HCO3.clone(), Py::str("HCO3"))?, f__pos(v_PaCO2.clone(), Py::str("PaCO2"))?])).items()?;
    if t3754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_HCO3 = t3754[0].clone();
    v_PaCO2 = t3754[1].clone();
    v_ph = py::add(&c_PK_CO2()?, &py::math1("log10", &py::truediv(&v_HCO3.clone(), &py::mul(&c_CO2_SOL()?, &v_PaCO2.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pH"), v_ph.clone()), (Py::str("H"), py::pow(&Py::Float(10.0), &py::sub(&Py::Float(9.0), &v_ph.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_plasma_osmolality(mut v_Na: Py, mut v_glucose: Py, mut v_BUN: Py, mut v_measured: Py) -> R {
    let mut v_calc: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t3808 = (Py::tuple(vec![f__pos(v_Na.clone(), Py::str("Na"))?, f__nonneg(v_glucose.clone(), Py::str("glucose"))?, f__nonneg(v_BUN.clone(), Py::str("BUN"))?])).items()?;
    if t3808.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Na = t3808[0].clone();
    v_glucose = t3808[1].clone();
    v_BUN = t3808[2].clone();
    v_calc = py::add(&py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?, &py::truediv(&v_BUN.clone(), &Py::Float(2.8))?)?;
    v_out = Py::dict(vec![(Py::str("calculated"), v_calc.clone()), (Py::str("effective"), py::add(&py::mul(&Py::Float(2.0), &v_Na.clone())?, &py::truediv(&v_glucose.clone(), &Py::Float(18.0))?)?)]);
    if ({ let t3809 = v_measured.clone(); Py::Bool({ let t3810 = Py::None; if !py::is(&t3809, &t3810) { true } else { false } }) }).truthy() {
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
    let t3862 = (Py::tuple(vec![f__pos(v_dP.clone(), Py::str("dP"))?, f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3862.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_dP = t3862[0].clone();
    v_eta = t3862[1].clone();
    v_L = t3862[2].clone();
    v_r = t3862[3].clone();
    v_Q = py::truediv(&py::mul(&v_dP.clone(), &c_MMHG()?)?, &f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?)?;
    v_a = py::mul(&v_r.clone(), &Py::Float(0.001))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Q"), py::mul(&v_Q.clone(), &Py::Float(1000000.0))?), (Py::str("Q_mL_min"), py::mul(&v_Q.clone(), &Py::Float(60000000.0))?), (Py::str("v_mean"), py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&c_PI()?, &v_a.clone())?, &v_a.clone())?)?, &Py::Float(100.0))?), (Py::str("tau_wall"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_eta.clone())?, &Py::Float(0.001))?, &v_Q.clone())?, &py::mul(&c_PI()?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poiseuille_resistance(mut v_eta: Py, mut v_L: Py, mut v_r: Py) -> R {
    let mut v_R: Py = Py::None;
    let t3914 = (Py::tuple(vec![f__pos(v_eta.clone(), Py::str("eta"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t3914.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_eta = t3914[0].clone();
    v_L = t3914[1].clone();
    v_r = t3914[2].clone();
    v_R = f__pois_R(v_eta.clone(), v_L.clone(), v_r.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("R_mmHg"), py::truediv(&py::truediv(&py::mul(&v_R.clone(), &Py::Float(1e-06))?, &Py::Float(60.0))?, &c_MMHG()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pulmonary_vascular_resistance(mut v_mPAP: Py, mut v_PCWP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t3968 = (Py::tuple(vec![f__num(v_mPAP.clone(), Py::str("mPAP"))?, f__nonneg(v_PCWP.clone(), Py::str("PCWP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t3968.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mPAP = t3968[0].clone();
    v_PCWP = t3968[1].clone();
    v_CO = t3968[2].clone();
    let _ = f__need({ let t3969 = v_mPAP.clone(); Py::Bool({ let t3970 = v_PCWP.clone(); if py::gt(&t3969, &t3970)? { true } else { false } }) }, Py::str("the mean pulmonary artery pressure must exceed the wedge pressure"))?;
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
    let t4028 = (Py::tuple(vec![f__fr(v_SaO2.clone(), Py::str("SaO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SvO2.clone(), Py::str("SvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpvO2.clone(), Py::str("SpvO2"), Py::Bool(false), Py::Bool(false))?, f__fr(v_SpaO2.clone(), Py::str("SpaO2"), Py::Bool(false), Py::Bool(false))?])).items()?;
    if t4028.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Sa = t4028[0].clone();
    v_Sv = t4028[1].clone();
    v_Spv = t4028[2].clone();
    v_Spa = t4028[3].clone();
    let _ = f__need({ let t4033 = { let t4029 = v_Sa.clone(); Py::Bool({ let t4030 = v_Sv.clone(); if py::gt(&t4029, &t4030)? { true } else { false } }) }; if !t4033.truthy() { t4033 } else { { let t4031 = v_Spv.clone(); Py::Bool({ let t4032 = v_Spa.clone(); if py::gt(&t4031, &t4032)? { true } else { false } }) } } }, Py::str("need SaO2 > SvO2 and SpvO2 > SpaO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qp_Qs"), py::truediv(&py::sub(&v_Sa.clone(), &v_Sv.clone())?, &py::sub(&v_Spv.clone(), &v_Spa.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qrs_axis(mut v_lead_I: Py, mut v_aVF: Py) -> R {
    let t4075 = (Py::tuple(vec![f__num(v_lead_I.clone(), Py::str("lead_I"))?, f__num(v_aVF.clone(), Py::str("aVF"))?])).items()?;
    if t4075.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lead_I = t4075[0].clone();
    v_aVF = t4075[1].clone();
    let _ = f__need({ let t4080 = { let t4076 = v_lead_I.clone(); Py::Bool({ let t4077 = Py::Int(0i64); if py::ne(&t4076, &t4077) { true } else { false } }) }; if t4080.truthy() { t4080 } else { { let t4078 = v_aVF.clone(); Py::Bool({ let t4079 = Py::Int(0i64); if py::ne(&t4078, &t4079) { true } else { false } }) } } }, Py::str("at least one net deflection must be nonzero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("axis"), py::math1("degrees", &py::atan2(&v_aVF.clone(), &v_lead_I.clone())?)?), (Py::str("amplitude"), py::hypot(&[v_lead_I.clone(), v_aVF.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qt_correction(mut v_QT: Py, mut v_HR: Py) -> R {
    let mut v_rr: Py = Py::None;
    let t4125 = (Py::tuple(vec![f__pos(v_QT.clone(), Py::str("QT"))?, f__pos(v_HR.clone(), Py::str("HR"))?])).items()?;
    if t4125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_QT = t4125[0].clone();
    v_HR = t4125[1].clone();
    v_rr = py::truediv(&Py::Float(60.0), &v_HR.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RR"), v_rr.clone()), (Py::str("QTc_Bazett"), py::truediv(&v_QT.clone(), &py::math1("sqrt", &v_rr.clone())?)?), (Py::str("QTc_Fridericia"), py::truediv(&v_QT.clone(), &py::pow(&v_rr.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?)?), (Py::str("QTc_Framingham"), py::add(&v_QT.clone(), &py::mul(&Py::Float(154.0), &py::sub(&Py::Float(1.0), &v_rr.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radius_flow_ratio(mut v_r1: Py, mut v_r2: Py) -> R {
    let mut v_q: Py = Py::None;
    let t4176 = (Py::tuple(vec![f__pos(v_r1.clone(), Py::str("r1"))?, f__pos(v_r2.clone(), Py::str("r2"))?])).items()?;
    if t4176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t4176[0].clone();
    v_r2 = t4176[1].clone();
    v_q = py::pow(&py::truediv(&v_r2.clone(), &v_r1.clone())?, &Py::Int(4i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("flow_ratio"), v_q.clone()), (Py::str("resistance_ratio"), py::truediv(&Py::Float(1.0), &v_q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_renal_clearance(mut v_U: Py, mut v_V: Py, mut v_P: Py) -> R {
    let t4223 = (Py::tuple(vec![f__nonneg(v_U.clone(), Py::str("U"))?, f__pos(v_V.clone(), Py::str("V"))?, f__pos(v_P.clone(), Py::str("P"))?])).items()?;
    if t4223.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4223[0].clone();
    v_V = t4223[1].clone();
    v_P = t4223[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("C"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &v_P.clone())?), (Py::str("excretion"), py::truediv(&py::mul(&v_U.clone(), &v_V.clone())?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_mechanics(mut v_VT: Py, mut v_Ppeak: Py, mut v_Pplat: Py, mut v_PEEP: Py, mut v_flow: Py) -> R {
    let t4277 = (Py::tuple(vec![f__pos(v_VT.clone(), Py::str("VT"))?, f__num(v_Ppeak.clone(), Py::str("Ppeak"))?, f__num(v_Pplat.clone(), Py::str("Pplat"))?, f__nonneg(v_PEEP.clone(), Py::str("PEEP"))?, f__pos(v_flow.clone(), Py::str("flow"))?])).items()?;
    if t4277.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_VT = t4277[0].clone();
    v_Ppeak = t4277[1].clone();
    v_Pplat = t4277[2].clone();
    v_PEEP = t4277[3].clone();
    v_flow = t4277[4].clone();
    let _ = f__need({ let t4278 = v_Ppeak.clone(); Py::Bool({ let t4279 = v_Pplat.clone(); if py::ge(&t4278, &t4279)? { { let t4280 = v_PEEP.clone(); if py::gt(&t4279, &t4280)? { true } else { false } } } else { false } }) }, Py::str("need Ppeak >= Pplat > PEEP"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("C_static"), py::truediv(&v_VT.clone(), &py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)?), (Py::str("C_dynamic"), py::truediv(&v_VT.clone(), &py::sub(&v_Ppeak.clone(), &v_PEEP.clone())?)?), (Py::str("R_aw"), py::truediv(&py::sub(&v_Ppeak.clone(), &v_Pplat.clone())?, &py::truediv(&v_flow.clone(), &Py::Float(60.0))?)?), (Py::str("driving_pressure"), py::sub(&v_Pplat.clone(), &v_PEEP.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_respiratory_time_constant(mut v_R: Py, mut v_C: Py, mut v_t: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let t4332 = (Py::tuple(vec![f__pos(v_R.clone(), Py::str("R"))?, f__pos(v_C.clone(), Py::str("C"))?])).items()?;
    if t4332.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t4332[0].clone();
    v_C = t4332[1].clone();
    v_tau = py::truediv(&py::mul(&v_R.clone(), &v_C.clone())?, &Py::Float(1000.0))?;
    v_out = Py::dict(vec![(Py::str("tau"), v_tau.clone()), (Py::str("t_95"), py::mul(&Py::Float(3.0), &v_tau.clone())?)]);
    if ({ let t4333 = v_t.clone(); Py::Bool({ let t4334 = Py::None; if !py::is(&t4333, &t4334) { true } else { false } }) }).truthy() {
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
    let t4394 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?, py::sub(&py::truediv(&py::neg(&v_q.clone())?, &Py::Float(2.0))?, &v_d.clone())?])).items()?;
    if t4394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4394[0].clone();
    v_b = t4394[1].clone();
    v_P = py::add(&py::copysign(&py::pow(&py::abs(&v_a.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_a.clone())?, &py::copysign(&py::pow(&py::abs(&v_b.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_b.clone())?)?;
    for t4395 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t4395;
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
    let t4492 = (Py::tuple(vec![f__pos(v_CcO2.clone(), Py::str("CcO2"))?, f__pos(v_CaO2.clone(), Py::str("CaO2"))?, f__nonneg(v_CvO2.clone(), Py::str("CvO2"))?])).items()?;
    if t4492.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Cc = t4492[0].clone();
    v_Ca = t4492[1].clone();
    v_Cv = t4492[2].clone();
    let _ = f__need({ let t4493 = v_Cc.clone(); Py::Bool({ let t4494 = v_Ca.clone(); if py::ge(&t4493, &t4494)? { { let t4495 = v_Cv.clone(); if py::gt(&t4494, &t4495)? { true } else { false } } } else { false } }) }, Py::str("need CcO2 >= CaO2 > CvO2"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Qs_Qt"), py::truediv(&py::sub(&v_Cc.clone(), &v_Ca.clone())?, &py::sub(&v_Cc.clone(), &v_Cv.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solution_osmolarity(mut v_conc: Py, mut v_molar_mass: Py, mut v_i: Py, mut v_phi: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t4550 = (Py::tuple(vec![f__pos(v_conc.clone(), Py::str("conc"))?, f__pos(v_molar_mass.clone(), Py::str("molar_mass"))?, f__int_ok(v_i.clone(), Py::str("i"))?, f__pos(v_phi.clone(), Py::str("phi"))?])).items()?;
    if t4550.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_conc = t4550[0].clone();
    v_M = t4550[1].clone();
    v_i = t4550[2].clone();
    v_phi = t4550[3].clone();
    v_m = py::mul(&py::truediv(&v_conc.clone(), &v_M.clone())?, &Py::Float(1000.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("molarity"), v_m.clone()), (Py::str("osmolarity"), py::mul(&py::mul(&v_m.clone(), &v_i.clone())?, &v_phi.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_starling_filtration(mut v_Pc: Py, mut v_Pi: Py, mut v_pic: Py, mut v_pii: Py, mut v_Kf: Py, mut v_sigma: Py) -> R {
    let mut v_nfp: Py = Py::None;
    let t4604 = (Py::tuple(vec![f__num(v_Pc.clone(), Py::str("Pc"))?, f__num(v_Pi.clone(), Py::str("Pi"))?, f__nonneg(v_pic.clone(), Py::str("pic"))?, f__nonneg(v_pii.clone(), Py::str("pii"))?, f__pos(v_Kf.clone(), Py::str("Kf"))?])).items()?;
    if t4604.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t4604[0].clone();
    v_Pi = t4604[1].clone();
    v_pic = t4604[2].clone();
    v_pii = t4604[3].clone();
    v_Kf = t4604[4].clone();
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
    let t4665 = (Py::tuple(vec![f__pos(v_rheobase.clone(), Py::str("rheobase"))?, f__pos(v_chronaxie.clone(), Py::str("chronaxie"))?, f__pos(v_t.clone(), Py::str("t"))?])).items()?;
    if t4665.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_I0 = t4665[0].clone();
    v_c = t4665[1].clone();
    v_t = t4665[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("I_lapicque"), py::truediv(&v_I0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_t.clone())?, &py::math1("log", &Py::Float(2.0))?)?, &v_c.clone())?)?)?)?), (Py::str("I_weiss"), py::mul(&v_I0.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_c.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_volume_ef(mut v_EDV: Py, mut v_ESV: Py) -> R {
    let mut v_sv: Py = Py::None;
    let t4719 = (Py::tuple(vec![f__pos(v_EDV.clone(), Py::str("EDV"))?, f__nonneg(v_ESV.clone(), Py::str("ESV"))?])).items()?;
    if t4719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_EDV = t4719[0].clone();
    v_ESV = t4719[1].clone();
    let _ = f__need({ let t4720 = v_ESV.clone(); Py::Bool({ let t4721 = v_EDV.clone(); if py::lt(&t4720, &t4721)? { true } else { false } }) }, Py::str("ESV must be smaller than EDV"))?;
    v_sv = py::sub(&v_EDV.clone(), &v_ESV.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SV"), v_sv.clone()), (Py::str("EF"), py::truediv(&v_sv.clone(), &v_EDV.clone())?), (Py::str("EF_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_sv.clone())?, &v_EDV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stroke_work(mut v_SV: Py, mut v_MAP: Py, mut v_HR: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4773 = (Py::tuple(vec![f__pos(v_SV.clone(), Py::str("SV"))?, f__pos(v_MAP.clone(), Py::str("MAP"))?])).items()?;
    if t4773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_SV = t4773[0].clone();
    v_MAP = t4773[1].clone();
    v_w = py::mul(&py::mul(&py::mul(&v_MAP.clone(), &c_MMHG()?)?, &v_SV.clone())?, &Py::Float(1e-06))?;
    v_out = Py::dict(vec![(Py::str("SW"), v_w.clone()), (Py::str("SW_gm"), py::truediv(&v_w.clone(), &py::mul(&c_G0()?, &Py::Float(0.001))?)?)]);
    if ({ let t4774 = v_HR.clone(); Py::Bool({ let t4775 = Py::None; if !py::is(&t4774, &t4775) { true } else { false } }) }).truthy() {
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
    let t4883 = (Py::tuple(vec![f__pos(v_LVIDd.clone(), Py::str("LVIDd"))?, f__pos(v_LVIDs.clone(), Py::str("LVIDs"))?])).items()?;
    if t4883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_LVIDd = t4883[0].clone();
    v_LVIDs = t4883[1].clone();
    let _ = f__need({ let t4884 = v_LVIDs.clone(); Py::Bool({ let t4885 = v_LVIDd.clone(); if py::lt(&t4884, &t4885)? { true } else { false } }) }, Py::str("the end-systolic diameter must be smaller than the end-diastolic"))?;
    let t4886 = (Py::tuple(vec![f__teich(v_LVIDd.clone())?, f__teich(v_LVIDs.clone())?])).items()?;
    if t4886.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_edv = t4886[0].clone();
    v_esv = t4886[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("EDV"), v_edv.clone()), (Py::str("ESV"), v_esv.clone()), (Py::str("EF"), py::truediv(&py::sub(&v_edv.clone(), &v_esv.clone())?, &v_edv.clone())?), (Py::str("FS"), py::truediv(&py::sub(&v_LVIDd.clone(), &v_LVIDs.clone())?, &v_LVIDd.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tubular_handling(mut v_GFR: Py, mut v_Px: Py, mut v_Ux: Py, mut v_V: Py) -> R {
    let mut v_ex: Py = Py::None;
    let mut v_fl: Py = Py::None;
    let t4940 = (Py::tuple(vec![f__pos(v_GFR.clone(), Py::str("GFR"))?, f__nonneg(v_Px.clone(), Py::str("Px"))?, f__nonneg(v_Ux.clone(), Py::str("Ux"))?, f__pos(v_V.clone(), Py::str("V"))?])).items()?;
    if t4940.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_GFR = t4940[0].clone();
    v_Px = t4940[1].clone();
    v_Ux = t4940[2].clone();
    v_V = t4940[3].clone();
    let t4941 = (Py::tuple(vec![py::truediv(&py::mul(&v_GFR.clone(), &v_Px.clone())?, &Py::Float(100.0))?, py::truediv(&py::mul(&v_Ux.clone(), &v_V.clone())?, &Py::Float(100.0))?])).items()?;
    if t4941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fl = t4941[0].clone();
    v_ex = t4941[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("filtered_load"), v_fl.clone()), (Py::str("excreted"), v_ex.clone()), (Py::str("reabsorbed"), py::sub(&v_fl.clone(), &v_ex.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vascular_resistance(mut v_MAP: Py, mut v_CVP: Py, mut v_CO: Py) -> R {
    let mut v_w: Py = Py::None;
    let t4829 = (Py::tuple(vec![f__num(v_MAP.clone(), Py::str("MAP"))?, f__nonneg(v_CVP.clone(), Py::str("CVP"))?, f__pos(v_CO.clone(), Py::str("CO"))?])).items()?;
    if t4829.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_MAP = t4829[0].clone();
    v_CVP = t4829[1].clone();
    v_CO = t4829[2].clone();
    let _ = f__need({ let t4830 = v_MAP.clone(); Py::Bool({ let t4831 = v_CVP.clone(); if py::gt(&t4830, &t4831)? { true } else { false } }) }, Py::str("the mean arterial pressure must exceed the venous pressure"))?;
    v_w = py::truediv(&py::sub(&v_MAP.clone(), &v_CVP.clone())?, &v_CO.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("SVR"), v_w.clone()), (Py::str("SVR_dyn"), py::mul(&v_w.clone(), &c_WOOD_DYN()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vessel_reynolds(mut v_rho: Py, mut v_v: Py, mut v_D: Py, mut v_eta: Py) -> R {
    let mut v_Re: Py = Py::None;
    let t5045 = (Py::tuple(vec![f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_v.clone(), Py::str("v"))?, f__pos(v_D.clone(), Py::str("D"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5045.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t5045[0].clone();
    v_v = t5045[1].clone();
    v_D = t5045[2].clone();
    v_eta = t5045[3].clone();
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &py::truediv(&v_v.clone(), &Py::Float(100.0))?)?, &py::truediv(&v_D.clone(), &Py::Float(1000.0))?)?, &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Re"), v_Re.clone()), (Py::str("v_critical"), py::mul(&py::truediv(&py::mul(&Py::Float(2000.0), &py::truediv(&v_eta.clone(), &Py::Float(1000.0))?)?, &py::truediv(&py::mul(&v_rho.clone(), &v_D.clone())?, &Py::Float(1000.0))?)?, &Py::Float(100.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_virtual_po2(mut v_PO2: Py, mut v_T: Py, mut v_pH: Py, mut v_PCO2: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let t5150 = (Py::tuple(vec![f__pos(v_PO2.clone(), Py::str("PO2"))?, f__num(v_T.clone(), Py::str("T"))?, f__num(v_pH.clone(), Py::str("pH"))?, f__pos(v_PCO2.clone(), Py::str("PCO2"))?])).items()?;
    if t5150.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_PO2 = t5150[0].clone();
    v_T = t5150[1].clone();
    v_pH = t5150[2].clone();
    v_PCO2 = t5150[3].clone();
    let _ = f__need({ let t5157 = { let t5151 = Py::Int(25i64); Py::Bool({ let t5152 = v_T.clone(); if py::le(&t5151, &t5152)? { { let t5153 = Py::Int(43i64); if py::le(&t5152, &t5153)? { true } else { false } } } else { false } }) }; if !t5157.truthy() { t5157 } else { { let t5154 = Py::Float(6.8); Py::Bool({ let t5155 = v_pH.clone(); if py::le(&t5154, &t5155)? { { let t5156 = Py::Float(7.8); if py::le(&t5155, &t5156)? { true } else { false } } } else { false } }) } } }, Py::str("temperature 25-43 degC and pH 6.8-7.8"))?;
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
    let t5209 = (Py::tuple(vec![f__num(v_age.clone(), Py::str("age"))?, f__pos(v_height.clone(), Py::str("height"))?, f__pos(v_weight.clone(), Py::str("weight"))?])).items()?;
    if t5209.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_age = t5209[0].clone();
    v_height = t5209[1].clone();
    v_weight = t5209[2].clone();
    v_fem = f__sex(v_sex.clone())?;
    let _ = f__need({ let t5214 = v_age.clone(); Py::Bool({ let t5215 = Py::Int(18i64); if py::ge(&t5214, &t5215)? { true } else { false } }) }, Py::str("the Watson equations are for adults"))?;
    v_v = (if (v_fem.clone()).truthy() { py::add(&py::add(&py::neg(&Py::Float(2.097))?, &py::mul(&Py::Float(0.1069), &v_height.clone())?)?, &py::mul(&Py::Float(0.2466), &v_weight.clone())?)? } else { py::add(&py::add(&py::sub(&Py::Float(2.447), &py::mul(&Py::Float(0.09516), &v_age.clone())?)?, &py::mul(&Py::Float(0.1074), &v_height.clone())?)?, &py::mul(&Py::Float(0.3362), &v_weight.clone())?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("TBW"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weir_rq(mut v_VO2: Py, mut v_VCO2: Py) -> R {
    let mut v_ee: Py = Py::None;
    let t5267 = (Py::tuple(vec![f__pos(v_VO2.clone(), Py::str("VO2"))?, f__pos(v_VCO2.clone(), Py::str("VCO2"))?])).items()?;
    if t5267.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_VO2 = t5267[0].clone();
    v_VCO2 = t5267[1].clone();
    v_ee = py::mul(&py::add(&py::truediv(&py::mul(&Py::Float(3.941), &v_VO2.clone())?, &Py::Float(1000.0))?, &py::truediv(&py::mul(&Py::Float(1.106), &v_VCO2.clone())?, &Py::Float(1000.0))?)?, &Py::Float(1440.0))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("RQ"), py::truediv(&v_VCO2.clone(), &v_VO2.clone())?), (Py::str("EE"), v_ee.clone()), (Py::str("EE_kJ"), py::mul(&v_ee.clone(), &Py::Float(4.184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_womersley(mut v_r: Py, mut v_HR: Py, mut v_rho: Py, mut v_eta: Py) -> R {
    let mut v_w: Py = Py::None;
    let t5319 = (Py::tuple(vec![f__pos(v_r.clone(), Py::str("r"))?, f__pos(v_HR.clone(), Py::str("HR"))?, f__pos(v_rho.clone(), Py::str("rho"))?, f__pos(v_eta.clone(), Py::str("eta"))?])).items()?;
    if t5319.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_r = t5319[0].clone();
    v_HR = t5319[1].clone();
    v_rho = t5319[2].clone();
    v_eta = t5319[3].clone();
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
fn s_1371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1373: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1372 = v_inputs.clone(); f_egfr_ckd_epi_2021(py::getitem(&t1372, &Py::str("Scr"))?, py::getitem(&t1372, &Py::str("age"))?, py::getitem(&t1372, &Py::str("sex"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1373 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1374 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1402 = (t1374).items()?;
        if t1402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1402[0].clone();
        v_w = t1402[1].clone();
        if ({ let t1375 = v_k.clone(); Py::Bool({ let t1376 = v_outputs.clone(); if !py::contains(&t1376, &t1375)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1403(mut v_kw: Py) -> R {
    return Ok({ let t1425 = f__from_si(Py::dict(vec![(Py::str("Scys"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_egfr_ckd_epi_2021_cys(py::getitem(&t1425, &Py::str("Scr"))?, py::getitem(&t1425, &Py::str("Scys"))?, py::getitem(&t1425, &Py::str("age"))?, py::getitem(&t1425, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1403(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1463 = (t1435).items()?;
        if t1463.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1463[0].clone();
        v_w = t1463[1].clone();
        if ({ let t1436 = v_k.clone(); Py::Bool({ let t1437 = v_outputs.clone(); if !py::contains(&t1437, &t1436)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1464(mut v_kw: Py) -> R {
    return Ok({ let t1481 = f__from_si(Py::dict(vec![(Py::str("CL"), Py::tuple(vec![Py::Float(2.7777777777777776e-07), Py::Float(0.0)])), (Py::str("Vd"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(3600.0), Py::Float(0.0)]))]), v_kw.clone())?; f_elimination_half_life(py::getitem(&t1481, &Py::str("CL"))?, py::getitem(&t1481, &Py::str("Vd"))?, py::method(&t1481, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1488(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1490: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1464(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1490 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1491 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1519 = (t1491).items()?;
        if t1519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1519[0].clone();
        v_w = t1519[1].clone();
        if ({ let t1492 = v_k.clone(); Py::Bool({ let t1493 = v_outputs.clone(); if !py::contains(&t1493, &t1492)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1520(mut v_kw: Py) -> R {
    return Ok({ let t1535 = f__from_si(Py::dict(vec![(Py::str("FEV1"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("FVC"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fev1_fvc(py::getitem(&t1535, &Py::str("FEV1"))?, py::getitem(&t1535, &Py::str("FVC"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1542(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1544: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1520(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1544 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1545 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1573 = (t1545).items()?;
        if t1573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1573[0].clone();
        v_w = t1573[1].clone();
        if ({ let t1546 = v_k.clone(); Py::Bool({ let t1547 = v_outputs.clone(); if !py::contains(&t1547, &t1546)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
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
fn s_1574(mut v_kw: Py) -> R {
    return Ok({ let t1591 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_cardiac_output(py::getitem(&t1591, &Py::str("VO2"))?, py::getitem(&t1591, &Py::str("CaO2"))?, py::getitem(&t1591, &Py::str("CvO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1600: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1574(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1600 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1601 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1629 = (t1601).items()?;
        if t1629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1629[0].clone();
        v_w = t1629[1].clone();
        if ({ let t1602 = v_k.clone(); Py::Bool({ let t1603 = v_outputs.clone(); if !py::contains(&t1603, &t1602)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1630(mut v_kw: Py) -> R {
    return Ok({ let t1643 = f__from_si(Py::dict(vec![(Py::str("D"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("A"), Py::tuple(vec![Py::Float(0.0001), Py::Float(0.0)])), (Py::str("dC"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_diffusion(py::getitem(&t1643, &Py::str("D"))?, py::getitem(&t1643, &Py::str("A"))?, py::getitem(&t1643, &Py::str("dC"))?, py::getitem(&t1643, &Py::str("T"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1652: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1630(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1652 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1653 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1681 = (t1653).items()?;
        if t1681.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1681[0].clone();
        v_w = t1681[1].clone();
        if ({ let t1654 = v_k.clone(); Py::Bool({ let t1655 = v_outputs.clone(); if !py::contains(&t1655, &t1654)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1682(mut v_kw: Py) -> R {
    return Ok({ let t1710 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_fick_saturations(py::getitem(&t1710, &Py::str("VO2"))?, py::getitem(&t1710, &Py::str("Hb"))?, py::getitem(&t1710, &Py::str("SaO2"))?, py::getitem(&t1710, &Py::str("SvO2"))?, py::getitem(&t1710, &Py::str("PaO2"))?, py::getitem(&t1710, &Py::str("PvO2"))?)? });
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
            v_want = s_1682(v_inputs.clone())?;
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
    return Ok({ let t1764 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("RPF"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_filtration_fraction(py::getitem(&t1764, &Py::str("GFR"))?, py::getitem(&t1764, &Py::str("RPF"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1771(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1773: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1749(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1773 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1774 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1802 = (t1774).items()?;
        if t1802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1802[0].clone();
        v_w = t1802[1].clone();
        if ({ let t1775 = v_k.clone(); Py::Bool({ let t1776 = v_outputs.clone(); if !py::contains(&t1776, &t1775)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1803(mut v_kw: Py) -> R {
    return Ok({ let t1828 = f__from_si(Py::dict(vec![(Py::str("TBW"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("V_added"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_fluid_shift(py::getitem(&t1828, &Py::str("TBW"))?, py::getitem(&t1828, &Py::str("Posm"))?, py::getitem(&t1828, &Py::str("V_added"))?, py::getitem(&t1828, &Py::str("C_added"))?, py::method(&t1828, "get", &[Py::str("ICF_fraction"), py::truediv(&Py::Float(2.0), &Py::Float(3.0))?])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1803(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1837 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1838 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1866 = (t1838).items()?;
        if t1866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1866[0].clone();
        v_w = t1866[1].clone();
        if ({ let t1839 = v_k.clone(); Py::Bool({ let t1840 = v_outputs.clone(); if !py::contains(&t1840, &t1839)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
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
fn s_1881(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1883: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1882 = v_inputs.clone(); f_fractional_excretion(py::getitem(&t1882, &Py::str("Ux"))?, py::getitem(&t1882, &Py::str("Px"))?, py::getitem(&t1882, &Py::str("Ucr"))?, py::getitem(&t1882, &Py::str("Pcr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1883 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1884 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1912 = (t1884).items()?;
        if t1912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1912[0].clone();
        v_w = t1912[1].clone();
        if ({ let t1885 = v_k.clone(); Py::Bool({ let t1886 = v_outputs.clone(); if !py::contains(&t1886, &t1885)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1913(mut v_kw: Py) -> R {
    return Ok({ let t1926 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_free_water_clearance(py::getitem(&t1926, &Py::str("V"))?, py::getitem(&t1926, &Py::str("Uosm"))?, py::getitem(&t1926, &Py::str("Posm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1933(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1935: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1913(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1935 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1936 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1964 = (t1936).items()?;
        if t1964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1964[0].clone();
        v_w = t1964[1].clone();
        if ({ let t1937 = v_k.clone(); Py::Bool({ let t1938 = v_outputs.clone(); if !py::contains(&t1938, &t1937)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1965(mut v_kw: Py) -> R {
    return Ok({ let t1989 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("Patm"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PH2O"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_gas_volume_conversion(py::getitem(&t1989, &Py::str("V"))?, py::getitem(&t1989, &Py::str("T"))?, py::getitem(&t1989, &Py::str("Patm"))?, py::getitem(&t1989, &Py::str("PH2O"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1996(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1998: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_1965(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1998 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1999 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2027 = (t1999).items()?;
        if t2027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2027[0].clone();
        v_w = t2027[1].clone();
        if ({ let t2000 = v_k.clone(); Py::Bool({ let t2001 = v_outputs.clone(); if !py::contains(&t2001, &t2000)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2028(mut v_kw: Py) -> R {
    return Ok({ let t2048 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_ghk_voltage(py::getitem(&t2048, &Py::str("PK"))?, py::getitem(&t2048, &Py::str("PNa"))?, py::getitem(&t2048, &Py::str("PCl"))?, py::getitem(&t2048, &Py::str("K_o"))?, py::getitem(&t2048, &Py::str("K_i"))?, py::getitem(&t2048, &Py::str("Na_o"))?, py::getitem(&t2048, &Py::str("Na_i"))?, py::getitem(&t2048, &Py::str("Cl_o"))?, py::getitem(&t2048, &Py::str("Cl_i"))?, py::method(&t2048, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2055(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2057: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2028(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2057 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2058 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2086 = (t2058).items()?;
        if t2086.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2086[0].clone();
        v_w = t2086[1].clone();
        if ({ let t2059 = v_k.clone(); Py::Bool({ let t2060 = v_outputs.clone(); if !py::contains(&t2060, &t2059)? { true } else { false } }) }).truthy() {
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
fn s_2087(mut v_kw: Py) -> R {
    return Ok({ let t2103 = f__from_si(Py::dict(vec![(Py::str("c"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_gibbs_donnan(py::getitem(&t2103, &Py::str("A"))?, py::getitem(&t2103, &Py::str("c"))?, py::method(&t2103, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2110(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2087(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2112 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2113 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2141 = (t2113).items()?;
        if t2141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2141[0].clone();
        v_w = t2141[1].clone();
        if ({ let t2114 = v_k.clone(); Py::Bool({ let t2115 = v_outputs.clone(); if !py::contains(&t2115, &t2114)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2142(mut v_kw: Py) -> R {
    return Ok({ let t2157 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("Tm"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_glucose_titration(py::getitem(&t2157, &Py::str("GFR"))?, py::getitem(&t2157, &Py::str("Pglucose"))?, py::getitem(&t2157, &Py::str("Tm"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2164(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2166: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2142(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2166 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2167 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2195 = (t2167).items()?;
        if t2195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2195[0].clone();
        v_w = t2195[1].clone();
        if ({ let t2168 = v_k.clone(); Py::Bool({ let t2169 = v_outputs.clone(); if !py::contains(&t2169, &t2168)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2223 = v_inputs.clone(); f_heart_rate_reserve(py::getitem(&t2223, &Py::str("age"))?, py::getitem(&t2223, &Py::str("HR_rest"))?, py::getitem(&t2223, &Py::str("intensity"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2253 = (t2225).items()?;
        if t2253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2253[0].clone();
        v_w = t2253[1].clone();
        if ({ let t2226 = v_k.clone(); Py::Bool({ let t2227 = v_outputs.clone(); if !py::contains(&t2227, &t2226)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2254(mut v_kw: Py) -> R {
    return Ok({ let t2284 = f__from_si(Py::dict(vec![(Py::str("P1"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_fit(py::getitem(&t2284, &Py::str("P1"))?, py::getitem(&t2284, &Py::str("S1"))?, py::getitem(&t2284, &Py::str("P2"))?, py::getitem(&t2284, &Py::str("S2"))?)? });
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
            v_want = s_2254(v_inputs.clone())?;
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
    return Ok({ let t2340 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_force_velocity(py::getitem(&t2340, &Py::str("F0"))?, py::getitem(&t2340, &Py::str("a"))?, py::getitem(&t2340, &Py::str("b"))?, py::getitem(&t2340, &Py::str("v"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2347(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2349: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2323(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2349 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2350 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2378 = (t2350).items()?;
        if t2378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2378[0].clone();
        v_w = t2378[1].clone();
        if ({ let t2351 = v_k.clone(); Py::Bool({ let t2352 = v_outputs.clone(); if !py::contains(&t2352, &t2351)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2379(mut v_kw: Py) -> R {
    return Ok({ let t2401 = f__from_si(Py::dict(vec![(Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_po2(py::getitem(&t2401, &Py::str("SO2"))?, py::getitem(&t2401, &Py::str("P50"))?, py::getitem(&t2401, &Py::str("n"))?)? });
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
fn s_2408(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2379(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2411 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2439 = (t2411).items()?;
        if t2439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2439[0].clone();
        v_w = t2439[1].clone();
        if ({ let t2412 = v_k.clone(); Py::Bool({ let t2413 = v_outputs.clone(); if !py::contains(&t2413, &t2412)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2440(mut v_kw: Py) -> R {
    return Ok({ let t2455 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("P50"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_hill_saturation(py::getitem(&t2455, &Py::str("PO2"))?, py::getitem(&t2455, &Py::str("P50"))?, py::getitem(&t2455, &Py::str("n"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2462(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2464: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2440(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2464 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2465 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2493 = (t2465).items()?;
        if t2493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2493[0].clone();
        v_w = t2493[1].clone();
        if ({ let t2466 = v_k.clone(); Py::Bool({ let t2467 = v_outputs.clone(); if !py::contains(&t2467, &t2466)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2494(mut v_kw: Py) -> R {
    return Ok({ let t2507 = f__from_si(Py::dict(vec![(Py::str("h"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_column(py::getitem(&t2507, &Py::str("h"))?, py::getitem(&t2507, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2514(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2494(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2517 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2545 = (t2517).items()?;
        if t2545.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2545[0].clone();
        v_w = t2545[1].clone();
        if ({ let t2518 = v_k.clone(); Py::Bool({ let t2519 = v_outputs.clone(); if !py::contains(&t2519, &t2518)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2546(mut v_kw: Py) -> R {
    return Ok({ let t2572 = f__from_si(Py::dict(vec![(Py::str("mass_air"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("mass_water"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("water_density"), Py::tuple(vec![Py::Float(1000.0000000000001), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("GI"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_hydrostatic_weighing(py::getitem(&t2572, &Py::str("mass_air"))?, py::getitem(&t2572, &Py::str("mass_water"))?, py::getitem(&t2572, &Py::str("water_density"))?, py::getitem(&t2572, &Py::str("RV"))?, py::method(&t2572, "get", &[Py::str("GI"), Py::Float(0.1)])?)? });
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
            v_want = s_2546(v_inputs.clone())?;
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
    return Ok({ let t2624 = f__from_si(Py::dict(vec![(Py::str("g"), Py::tuple(vec![Py::Float(1e-09), Py::Float(0.0)])), (Py::str("Vm"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("E"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_ionic_current(py::getitem(&t2624, &Py::str("g"))?, py::getitem(&t2624, &Py::str("Vm"))?, py::getitem(&t2624, &Py::str("E"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2631(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2633: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2611(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2633 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2634 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2662 = (t2634).items()?;
        if t2662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2662[0].clone();
        v_w = t2662[1].clone();
        if ({ let t2635 = v_k.clone(); Py::Bool({ let t2636 = v_outputs.clone(); if !py::contains(&t2636, &t2635)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2663(mut v_kw: Py) -> R {
    return Ok({ let t2682 = f__from_si(Py::dict(vec![(Py::str("P"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_laplace_wall_tension(py::getitem(&t2682, &Py::str("P"))?, py::getitem(&t2682, &Py::str("r"))?, py::getitem(&t2682, &Py::str("shape"))?, py::method(&t2682, "get", &[Py::str("h"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2691: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2663(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2691 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2692 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2720 = (t2692).items()?;
        if t2720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2720[0].clone();
        v_w = t2720[1].clone();
        if ({ let t2693 = v_k.clone(); Py::Bool({ let t2694 = v_outputs.clone(); if !py::contains(&t2694, &t2693)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2721(mut v_kw: Py) -> R {
    return Ok({ let t2741 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_lean_body_mass_boer(py::getitem(&t2741, &Py::str("weight"))?, py::getitem(&t2741, &Py::str("height"))?, py::getitem(&t2741, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2748(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2750: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2721(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2750 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2751 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2779 = (t2751).items()?;
        if t2779.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2779[0].clone();
        v_w = t2779[1].clone();
        if ({ let t2752 = v_k.clone(); Py::Bool({ let t2753 = v_outputs.clone(); if !py::contains(&t2753, &t2752)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2780(mut v_kw: Py) -> R {
    return Ok({ let t2797 = f__from_si(Py::dict(vec![(Py::str("d"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("x"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_length_constant(py::getitem(&t2797, &Py::str("Rm"))?, py::getitem(&t2797, &Py::str("Ri"))?, py::getitem(&t2797, &Py::str("d"))?, py::method(&t2797, "get", &[Py::str("x"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2804(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2806: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2780(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2806 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2807 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2835 = (t2807).items()?;
        if t2835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2835[0].clone();
        v_w = t2835[1].clone();
        if ({ let t2808 = v_k.clone(); Py::Bool({ let t2809 = v_outputs.clone(); if !py::contains(&t2809, &t2808)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2836(mut v_kw: Py) -> R {
    return Ok({ let t2851 = f__from_si(Py::dict(vec![(Py::str("TV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("IRV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ERV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("RV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_lung_volumes(py::getitem(&t2851, &Py::str("TV"))?, py::getitem(&t2851, &Py::str("IRV"))?, py::getitem(&t2851, &Py::str("ERV"))?, py::getitem(&t2851, &Py::str("RV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2836(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2861 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2889 = (t2861).items()?;
        if t2889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2889[0].clone();
        v_w = t2889[1].clone();
        if ({ let t2862 = v_k.clone(); Py::Bool({ let t2863 = v_outputs.clone(); if !py::contains(&t2863, &t2862)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2890(mut v_kw: Py) -> R {
    return Ok({ let t2905 = f__from_si(Py::dict(vec![(Py::str("SBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("DBP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_mean_arterial_pressure(py::getitem(&t2905, &Py::str("SBP"))?, py::getitem(&t2905, &Py::str("DBP"))?)? });
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
fn s_2912(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2914: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2890(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2914 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2915 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2943 = (t2915).items()?;
        if t2943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2943[0].clone();
        v_w = t2943[1].clone();
        if ({ let t2916 = v_k.clone(); Py::Bool({ let t2917 = v_outputs.clone(); if !py::contains(&t2917, &t2916)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2944(mut v_kw: Py) -> R {
    return Ok({ let t2959 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("dV"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("area"), Py::tuple(vec![Py::Float(1e-12), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_charge(py::getitem(&t2959, &Py::str("Cm"))?, py::getitem(&t2959, &Py::str("dV"))?, py::getitem(&t2959, &Py::str("area"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2966(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2968: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2944(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2968 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2969 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2997 = (t2969).items()?;
        if t2997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2997[0].clone();
        v_w = t2997[1].clone();
        if ({ let t2970 = v_k.clone(); Py::Bool({ let t2971 = v_outputs.clone(); if !py::contains(&t2971, &t2970)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2998(mut v_kw: Py) -> R {
    return Ok({ let t3015 = f__from_si(Py::dict(vec![(Py::str("Cm"), Py::tuple(vec![Py::Float(0.009999999999999998), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_membrane_time_constant(py::getitem(&t3015, &Py::str("Rm"))?, py::getitem(&t3015, &Py::str("Cm"))?, py::method(&t3015, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3022(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3024: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_2998(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3024 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3025 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3053 = (t3025).items()?;
        if t3053.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3053[0].clone();
        v_w = t3053[1].clone();
        if ({ let t3026 = v_k.clone(); Py::Bool({ let t3027 = v_outputs.clone(); if !py::contains(&t3027, &t3026)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3054(mut v_kw: Py) -> R {
    return Ok({ let t3067 = f__from_si(Py::dict(vec![(Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("duration"), Py::tuple(vec![Py::Float(60.0), Py::Float(0.0)]))]), v_kw.clone())?; f_met_energy(py::getitem(&t3067, &Py::str("MET"))?, py::getitem(&t3067, &Py::str("weight"))?, py::getitem(&t3067, &Py::str("duration"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3074(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3076: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3054(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3076 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3077 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3105 = (t3077).items()?;
        if t3105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3105[0].clone();
        v_w = t3105[1].clone();
        if ({ let t3078 = v_k.clone(); Py::Bool({ let t3079 = v_outputs.clone(); if !py::contains(&t3079, &t3078)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3106(mut v_kw: Py) -> R {
    return Ok({ let t3123 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("VD"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_minute_ventilation(py::getitem(&t3123, &Py::str("VT"))?, py::getitem(&t3123, &Py::str("RR"))?, py::getitem(&t3123, &Py::str("VD"))?)? });
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
fn s_3130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3132: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3106(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3132 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3133 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3161 = (t3133).items()?;
        if t3161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3161[0].clone();
        v_w = t3161[1].clone();
        if ({ let t3134 = v_k.clone(); Py::Bool({ let t3135 = v_outputs.clone(); if !py::contains(&t3135, &t3134)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3162(mut v_kw: Py) -> R {
    return Ok({ let t3175 = f__from_si(Py::dict(vec![(Py::str("E"), Py::tuple(vec![Py::Float(1000.0), Py::Float(0.0)])), (Py::str("h"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_moens_korteweg(py::getitem(&t3175, &Py::str("E"))?, py::getitem(&t3175, &Py::str("h"))?, py::getitem(&t3175, &Py::str("D"))?, py::getitem(&t3175, &Py::str("rho"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3182(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3162(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3185 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3213 = (t3185).items()?;
        if t3213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3213[0].clone();
        v_w = t3213[1].clone();
        if ({ let t3186 = v_k.clone(); Py::Bool({ let t3187 = v_outputs.clone(); if !py::contains(&t3187, &t3186)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3214(mut v_kw: Py) -> R {
    return Ok({ let t3227 = f__from_si(Py::dict(vec![(Py::str("F0"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("a"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("b"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_muscle_max_power(py::getitem(&t3227, &Py::str("F0"))?, py::getitem(&t3227, &Py::str("a"))?, py::getitem(&t3227, &Py::str("b"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3234(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3236: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3214(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3236 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3237 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3265 = (t3237).items()?;
        if t3265.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3265[0].clone();
        v_w = t3265[1].clone();
        if ({ let t3238 = v_k.clone(); Py::Bool({ let t3239 = v_outputs.clone(); if !py::contains(&t3239, &t3238)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3266(mut v_kw: Py) -> R {
    return Ok({ let t3288 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_nernst(py::getitem(&t3288, &Py::str("z"))?, py::getitem(&t3288, &Py::str("Cout"))?, py::getitem(&t3288, &Py::str("Cin"))?, py::method(&t3288, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3297: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3266(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3297 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3298 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3326 = (t3298).items()?;
        if t3326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3326[0].clone();
        v_w = t3326[1].clone();
        if ({ let t3299 = v_k.clone(); Py::Bool({ let t3300 = v_outputs.clone(); if !py::contains(&t3300, &t3299)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3327(mut v_kw: Py) -> R {
    return Ok({ let t3344 = f__from_si(Py::dict(vec![(Py::str("distance"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_proximal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("latency_distal"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_nerve_conduction_velocity(py::getitem(&t3344, &Py::str("distance"))?, py::getitem(&t3344, &Py::str("latency_proximal"))?, py::getitem(&t3344, &Py::str("latency_distal"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3351(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3353: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3327(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3353 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3354 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3382 = (t3354).items()?;
        if t3382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3382[0].clone();
        v_w = t3382[1].clone();
        if ({ let t3355 = v_k.clone(); Py::Bool({ let t3356 = v_outputs.clone(); if !py::contains(&t3356, &t3355)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3383(mut v_kw: Py) -> R {
    return Ok({ let t3399 = f__from_si(Py::dict(vec![(Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)]))]), v_kw.clone())?; f_osmotic_pressure(py::getitem(&t3399, &Py::str("osm"))?, py::method(&t3399, "get", &[Py::str("T"), Py::Float(37.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3406(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3408: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3383(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3408 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3409 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3437 = (t3409).items()?;
        if t3437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3437[0].clone();
        v_w = t3437[1].clone();
        if ({ let t3410 = v_k.clone(); Py::Bool({ let t3411 = v_outputs.clone(); if !py::contains(&t3411, &t3410)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3438(mut v_kw: Py) -> R {
    return Ok({ let t3466 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PvO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_consumption(py::getitem(&t3466, &Py::str("CO"))?, py::getitem(&t3466, &Py::str("Hb"))?, py::getitem(&t3466, &Py::str("SaO2"))?, py::getitem(&t3466, &Py::str("SvO2"))?, py::getitem(&t3466, &Py::str("PaO2"))?, py::getitem(&t3466, &Py::str("PvO2"))?)? });
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
fn s_3473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3438(v_inputs.clone())?;
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
    return Ok({ let t3530 = f__from_si(Py::dict(vec![(Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_content(py::getitem(&t3530, &Py::str("Hb"))?, py::getitem(&t3530, &Py::str("SaO2"))?, py::getitem(&t3530, &Py::str("PaO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3537(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3539: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3505(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3539 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3540 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3568 = (t3540).items()?;
        if t3568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3568[0].clone();
        v_w = t3568[1].clone();
        if ({ let t3541 = v_k.clone(); Py::Bool({ let t3542 = v_outputs.clone(); if !py::contains(&t3542, &t3541)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3569(mut v_kw: Py) -> R {
    return Ok({ let t3593 = f__from_si(Py::dict(vec![(Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)])), (Py::str("PaO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_oxygen_delivery(py::getitem(&t3593, &Py::str("CO"))?, py::getitem(&t3593, &Py::str("Hb"))?, py::getitem(&t3593, &Py::str("SaO2"))?, py::getitem(&t3593, &Py::str("PaO2"))?)? });
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
fn s_3600(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3602: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3569(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3602 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3603 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3631 = (t3603).items()?;
        if t3631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3631[0].clone();
        v_w = t3631[1].clone();
        if ({ let t3604 = v_k.clone(); Py::Bool({ let t3605 = v_outputs.clone(); if !py::contains(&t3605, &t3604)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3632(mut v_kw: Py) -> R {
    return Ok({ let t3655 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_pah_renal_flow(py::getitem(&t3655, &Py::str("U_PAH"))?, py::getitem(&t3655, &Py::str("V"))?, py::getitem(&t3655, &Py::str("P_PAH"))?, py::getitem(&t3655, &Py::str("Hct"))?, py::method(&t3655, "get", &[Py::str("E"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3662(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3664: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3632(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3664 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3665 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3693 = (t3665).items()?;
        if t3693.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3693[0].clone();
        v_w = t3693[1].clone();
        if ({ let t3666 = v_k.clone(); Py::Bool({ let t3667 = v_outputs.clone(); if !py::contains(&t3667, &t3666)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3694(mut v_kw: Py) -> R {
    return Ok({ let t3710 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_pco2_from_ph(py::getitem(&t3710, &Py::str("pH"))?, py::getitem(&t3710, &Py::str("HCO3"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3694(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3748 = (t3720).items()?;
        if t3748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3748[0].clone();
        v_w = t3748[1].clone();
        if ({ let t3721 = v_k.clone(); Py::Bool({ let t3722 = v_outputs.clone(); if !py::contains(&t3722, &t3721)? { true } else { false } }) }).truthy() {
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
fn s_3749(mut v_kw: Py) -> R {
    return Ok({ let t3762 = f__from_si(Py::dict(vec![(Py::str("HCO3"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("PaCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_ph_henderson(py::getitem(&t3762, &Py::str("HCO3"))?, py::getitem(&t3762, &Py::str("PaCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3769(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3771: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3749(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3771 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3772 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3800 = (t3772).items()?;
        if t3800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3800[0].clone();
        v_w = t3800[1].clone();
        if ({ let t3773 = v_k.clone(); Py::Bool({ let t3774 = v_outputs.clone(); if !py::contains(&t3774, &t3773)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3801(mut v_kw: Py) -> R {
    return Ok({ let t3818 = f__from_si(Py::dict(vec![(Py::str("Na"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_plasma_osmolality(py::getitem(&t3818, &Py::str("Na"))?, py::getitem(&t3818, &Py::str("glucose"))?, py::getitem(&t3818, &Py::str("BUN"))?, py::method(&t3818, "get", &[Py::str("measured"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3825(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3827: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3801(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3827 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3828 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3856 = (t3828).items()?;
        if t3856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3856[0].clone();
        v_w = t3856[1].clone();
        if ({ let t3829 = v_k.clone(); Py::Bool({ let t3830 = v_outputs.clone(); if !py::contains(&t3830, &t3829)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3857(mut v_kw: Py) -> R {
    return Ok({ let t3870 = f__from_si(Py::dict(vec![(Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_flow(py::getitem(&t3870, &Py::str("dP"))?, py::getitem(&t3870, &Py::str("eta"))?, py::getitem(&t3870, &Py::str("L"))?, py::getitem(&t3870, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3877(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3879: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3857(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3879 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3880 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3908 = (t3880).items()?;
        if t3908.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3908[0].clone();
        v_w = t3908[1].clone();
        if ({ let t3881 = v_k.clone(); Py::Bool({ let t3882 = v_outputs.clone(); if !py::contains(&t3882, &t3881)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3909(mut v_kw: Py) -> R {
    return Ok({ let t3922 = f__from_si(Py::dict(vec![(Py::str("L"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_poiseuille_resistance(py::getitem(&t3922, &Py::str("eta"))?, py::getitem(&t3922, &Py::str("L"))?, py::getitem(&t3922, &Py::str("r"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3929(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3931: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3909(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3931 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3932 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3960 = (t3932).items()?;
        if t3960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3960[0].clone();
        v_w = t3960[1].clone();
        if ({ let t3933 = v_k.clone(); Py::Bool({ let t3934 = v_outputs.clone(); if !py::contains(&t3934, &t3933)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3961(mut v_kw: Py) -> R {
    return Ok({ let t3978 = f__from_si(Py::dict(vec![(Py::str("mPAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("PCWP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_pulmonary_vascular_resistance(py::getitem(&t3978, &Py::str("mPAP"))?, py::getitem(&t3978, &Py::str("PCWP"))?, py::getitem(&t3978, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3987: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_3961(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3987 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3988 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4016 = (t3988).items()?;
        if t4016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4016[0].clone();
        v_w = t4016[1].clone();
        if ({ let t3989 = v_k.clone(); Py::Bool({ let t3990 = v_outputs.clone(); if !py::contains(&t3990, &t3989)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4041(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4043: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4042 = v_inputs.clone(); f_qp_qs(py::getitem(&t4042, &Py::str("SaO2"))?, py::getitem(&t4042, &Py::str("SvO2"))?, py::getitem(&t4042, &Py::str("SpvO2"))?, py::getitem(&t4042, &Py::str("SpaO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4043 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4044 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4072 = (t4044).items()?;
        if t4072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4072[0].clone();
        v_w = t4072[1].clone();
        if ({ let t4045 = v_k.clone(); Py::Bool({ let t4046 = v_outputs.clone(); if !py::contains(&t4046, &t4045)? { true } else { false } }) }).truthy() {
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
fn s_4088(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4090: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4089 = v_inputs.clone(); f_qrs_axis(py::getitem(&t4089, &Py::str("lead_I"))?, py::getitem(&t4089, &Py::str("aVF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4090 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4091 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4119 = (t4091).items()?;
        if t4119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4119[0].clone();
        v_w = t4119[1].clone();
        if ({ let t4092 = v_k.clone(); Py::Bool({ let t4093 = v_outputs.clone(); if !py::contains(&t4093, &t4092)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4120(mut v_kw: Py) -> R {
    return Ok({ let t4133 = f__from_si(Py::dict(vec![(Py::str("QT"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_qt_correction(py::getitem(&t4133, &Py::str("QT"))?, py::getitem(&t4133, &Py::str("HR"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4140(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4120(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4143 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4171 = (t4143).items()?;
        if t4171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4171[0].clone();
        v_w = t4171[1].clone();
        if ({ let t4144 = v_k.clone(); Py::Bool({ let t4145 = v_outputs.clone(); if !py::contains(&t4145, &t4144)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4186: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4185 = v_inputs.clone(); f_radius_flow_ratio(py::getitem(&t4185, &Py::str("r1"))?, py::getitem(&t4185, &Py::str("r2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4186 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4187 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4215 = (t4187).items()?;
        if t4215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4215[0].clone();
        v_w = t4215[1].clone();
        if ({ let t4188 = v_k.clone(); Py::Bool({ let t4189 = v_outputs.clone(); if !py::contains(&t4189, &t4188)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4216(mut v_kw: Py) -> R {
    return Ok({ let t4231 = f__from_si(Py::dict(vec![(Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_renal_clearance(py::getitem(&t4231, &Py::str("U"))?, py::getitem(&t4231, &Py::str("V"))?, py::getitem(&t4231, &Py::str("P"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4238(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4240: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4216(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4240 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4241 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4269 = (t4241).items()?;
        if t4269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4269[0].clone();
        v_w = t4269[1].clone();
        if ({ let t4242 = v_k.clone(); Py::Bool({ let t4243 = v_outputs.clone(); if !py::contains(&t4243, &t4242)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4270(mut v_kw: Py) -> R {
    return Ok({ let t4288 = f__from_si(Py::dict(vec![(Py::str("VT"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("flow"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_mechanics(py::getitem(&t4288, &Py::str("VT"))?, py::getitem(&t4288, &Py::str("Ppeak"))?, py::getitem(&t4288, &Py::str("Pplat"))?, py::getitem(&t4288, &Py::str("PEEP"))?, py::getitem(&t4288, &Py::str("flow"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4295(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4297: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4270(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4297 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4298 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4326 = (t4298).items()?;
        if t4326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4326[0].clone();
        v_w = t4326[1].clone();
        if ({ let t4299 = v_k.clone(); Py::Bool({ let t4300 = v_outputs.clone(); if !py::contains(&t4300, &t4299)? { true } else { false } }) }).truthy() {
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
fn s_4327(mut v_kw: Py) -> R {
    return Ok({ let t4344 = f__from_si(Py::dict(vec![(Py::str("t"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_respiratory_time_constant(py::getitem(&t4344, &Py::str("R"))?, py::getitem(&t4344, &Py::str("C"))?, py::method(&t4344, "get", &[Py::str("t"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4351(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4353: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4327(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4353 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4354 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4382 = (t4354).items()?;
        if t4382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4382[0].clone();
        v_w = t4382[1].clone();
        if ({ let t4355 = v_k.clone(); Py::Bool({ let t4356 = v_outputs.clone(); if !py::contains(&t4356, &t4355)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4403(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4404 = v_inputs.clone(); f_severinghaus_po2(py::getitem(&t4404, &Py::str("SO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4406 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4434 = (t4406).items()?;
        if t4434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4434[0].clone();
        v_w = t4434[1].clone();
        if ({ let t4407 = v_k.clone(); Py::Bool({ let t4408 = v_outputs.clone(); if !py::contains(&t4408, &t4407)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4435(mut v_kw: Py) -> R {
    return Ok({ let t4447 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_severinghaus_saturation(py::getitem(&t4447, &Py::str("PO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4435(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4456 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4457 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4485 = (t4457).items()?;
        if t4485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4485[0].clone();
        v_w = t4485[1].clone();
        if ({ let t4458 = v_k.clone(); Py::Bool({ let t4459 = v_outputs.clone(); if !py::contains(&t4459, &t4458)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4503(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4504 = v_inputs.clone(); f_shunt_fraction(py::getitem(&t4504, &Py::str("CcO2"))?, py::getitem(&t4504, &Py::str("CaO2"))?, py::getitem(&t4504, &Py::str("CvO2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4506 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4534 = (t4506).items()?;
        if t4534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4534[0].clone();
        v_w = t4534[1].clone();
        if ({ let t4507 = v_k.clone(); Py::Bool({ let t4508 = v_outputs.clone(); if !py::contains(&t4508, &t4507)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4535(mut v_kw: Py) -> R {
    return Ok({ let t4558 = f__from_si(Py::dict(vec![(Py::str("conc"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("molar_mass"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_solution_osmolarity(py::getitem(&t4558, &Py::str("conc"))?, py::getitem(&t4558, &Py::str("molar_mass"))?, py::getitem(&t4558, &Py::str("i"))?, py::method(&t4558, "get", &[Py::str("phi"), Py::Float(1.0)])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4535(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4596 = (t4568).items()?;
        if t4596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4596[0].clone();
        v_w = t4596[1].clone();
        if ({ let t4569 = v_k.clone(); Py::Bool({ let t4570 = v_outputs.clone(); if !py::contains(&t4570, &t4569)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4597(mut v_kw: Py) -> R {
    return Ok({ let t4621 = f__from_si(Py::dict(vec![(Py::str("Pc"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("Pi"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pic"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("pii"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_starling_filtration(py::getitem(&t4621, &Py::str("Pc"))?, py::getitem(&t4621, &Py::str("Pi"))?, py::getitem(&t4621, &Py::str("pic"))?, py::getitem(&t4621, &Py::str("pii"))?, py::getitem(&t4621, &Py::str("Kf"))?, py::method(&t4621, "get", &[Py::str("sigma"), Py::Float(1.0)])?)? });
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
fn s_4628(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4630: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4597(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4630 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4631 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4659 = (t4631).items()?;
        if t4659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4659[0].clone();
        v_w = t4659[1].clone();
        if ({ let t4632 = v_k.clone(); Py::Bool({ let t4633 = v_outputs.clone(); if !py::contains(&t4633, &t4632)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4660(mut v_kw: Py) -> R {
    return Ok({ let t4673 = f__from_si(Py::dict(vec![(Py::str("chronaxie"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("t"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_strength_duration(py::getitem(&t4673, &Py::str("rheobase"))?, py::getitem(&t4673, &Py::str("chronaxie"))?, py::getitem(&t4673, &Py::str("t"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4680(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4682: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4660(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4682 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4683 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4711 = (t4683).items()?;
        if t4711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4711[0].clone();
        v_w = t4711[1].clone();
        if ({ let t4684 = v_k.clone(); Py::Bool({ let t4685 = v_outputs.clone(); if !py::contains(&t4685, &t4684)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4712(mut v_kw: Py) -> R {
    return Ok({ let t4729 = f__from_si(Py::dict(vec![(Py::str("EDV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("ESV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_volume_ef(py::getitem(&t4729, &Py::str("EDV"))?, py::getitem(&t4729, &Py::str("ESV"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4736(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4738: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4712(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4738 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4739 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4767 = (t4739).items()?;
        if t4767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4767[0].clone();
        v_w = t4767[1].clone();
        if ({ let t4740 = v_k.clone(); Py::Bool({ let t4741 = v_outputs.clone(); if !py::contains(&t4741, &t4740)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4768(mut v_kw: Py) -> R {
    return Ok({ let t4783 = f__from_si(Py::dict(vec![(Py::str("SV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_stroke_work(py::getitem(&t4783, &Py::str("SV"))?, py::getitem(&t4783, &Py::str("MAP"))?, py::method(&t4783, "get", &[Py::str("HR"), Py::None])?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4790(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4792: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4768(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4792 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4793 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4821 = (t4793).items()?;
        if t4821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4821[0].clone();
        v_w = t4821[1].clone();
        if ({ let t4794 = v_k.clone(); Py::Bool({ let t4795 = v_outputs.clone(); if !py::contains(&t4795, &t4794)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4822(mut v_kw: Py) -> R {
    return Ok({ let t4839 = f__from_si(Py::dict(vec![(Py::str("MAP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CVP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("CO"), Py::tuple(vec![Py::Float(1.6666666666666667e-05), Py::Float(0.0)]))]), v_kw.clone())?; f_vascular_resistance(py::getitem(&t4839, &Py::str("MAP"))?, py::getitem(&t4839, &Py::str("CVP"))?, py::getitem(&t4839, &Py::str("CO"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4846(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4848: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4822(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4848 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4849 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4877 = (t4849).items()?;
        if t4877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4877[0].clone();
        v_w = t4877[1].clone();
        if ({ let t4850 = v_k.clone(); Py::Bool({ let t4851 = v_outputs.clone(); if !py::contains(&t4851, &t4850)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
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
fn s_4878(mut v_kw: Py) -> R {
    return Ok({ let t4894 = f__from_si(Py::dict(vec![(Py::str("LVIDd"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("LVIDs"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)]))]), v_kw.clone())?; f_teichholz(py::getitem(&t4894, &Py::str("LVIDd"))?, py::getitem(&t4894, &Py::str("LVIDs"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4901(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4903: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4878(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4903 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4904 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4932 = (t4904).items()?;
        if t4932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4932[0].clone();
        v_w = t4932[1].clone();
        if ({ let t4905 = v_k.clone(); Py::Bool({ let t4906 = v_outputs.clone(); if !py::contains(&t4906, &t4905)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4933(mut v_kw: Py) -> R {
    return Ok({ let t4949 = f__from_si(Py::dict(vec![(Py::str("GFR"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("V"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_tubular_handling(py::getitem(&t4949, &Py::str("GFR"))?, py::getitem(&t4949, &Py::str("Px"))?, py::getitem(&t4949, &Py::str("Ux"))?, py::getitem(&t4949, &Py::str("V"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4958: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4933(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4958 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4959 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4987 = (t4959).items()?;
        if t4987.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4987[0].clone();
        v_w = t4987[1].clone();
        if ({ let t4960 = v_k.clone(); Py::Bool({ let t4961 = v_outputs.clone(); if !py::contains(&t4961, &t4960)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4988(mut v_kw: Py) -> R {
    return Ok({ let t5001 = f__from_si(Py::dict(vec![(Py::str("dV"), Py::tuple(vec![Py::Float(1e-06), Py::Float(0.0)])), (Py::str("dP"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_compliance(py::getitem(&t5001, &Py::str("dV"))?, py::getitem(&t5001, &Py::str("dP"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5008(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5010: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_4988(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5010 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5011 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5039 = (t5011).items()?;
        if t5039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5039[0].clone();
        v_w = t5039[1].clone();
        if ({ let t5012 = v_k.clone(); Py::Bool({ let t5013 = v_outputs.clone(); if !py::contains(&t5013, &t5012)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5040(mut v_kw: Py) -> R {
    return Ok({ let t5053 = f__from_si(Py::dict(vec![(Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("v"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("D"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)]))]), v_kw.clone())?; f_vessel_reynolds(py::getitem(&t5053, &Py::str("rho"))?, py::getitem(&t5053, &Py::str("v"))?, py::getitem(&t5053, &Py::str("D"))?, py::getitem(&t5053, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5060(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5062: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5040(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5062 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5063 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5091 = (t5063).items()?;
        if t5091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5091[0].clone();
        v_w = t5091[1].clone();
        if ({ let t5064 = v_k.clone(); Py::Bool({ let t5065 = v_outputs.clone(); if !py::contains(&t5065, &t5064)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5114 = v_inputs.clone(); f_network_resistance(py::getitem(&t5114, &Py::str("R"))?, py::getitem(&t5114, &Py::str("arrangement"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5144 = (t5116).items()?;
        if t5144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5144[0].clone();
        v_w = t5144[1].clone();
        if ({ let t5117 = v_k.clone(); Py::Bool({ let t5118 = v_outputs.clone(); if !py::contains(&t5118, &t5117)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5145(mut v_kw: Py) -> R {
    return Ok({ let t5165 = f__from_si(Py::dict(vec![(Py::str("PO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("T"), Py::tuple(vec![Py::Float(1.0), Py::Float(273.15)])), (Py::str("PCO2"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)]))]), v_kw.clone())?; f_virtual_po2(py::getitem(&t5165, &Py::str("PO2"))?, py::getitem(&t5165, &Py::str("T"))?, py::getitem(&t5165, &Py::str("pH"))?, py::getitem(&t5165, &Py::str("PCO2"))?)? });
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
fn s_5172(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5174: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5145(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5174 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5175 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5203 = (t5175).items()?;
        if t5203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5203[0].clone();
        v_w = t5203[1].clone();
        if ({ let t5176 = v_k.clone(); Py::Bool({ let t5177 = v_outputs.clone(); if !py::contains(&t5177, &t5176)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5204(mut v_kw: Py) -> R {
    return Ok({ let t5223 = f__from_si(Py::dict(vec![(Py::str("height"), Py::tuple(vec![Py::Float(0.01), Py::Float(0.0)])), (Py::str("weight"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_watson_tbw(py::getitem(&t5223, &Py::str("age"))?, py::getitem(&t5223, &Py::str("height"))?, py::getitem(&t5223, &Py::str("weight"))?, py::getitem(&t5223, &Py::str("sex"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5230(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5232: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5204(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5232 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5233 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5261 = (t5233).items()?;
        if t5261.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5261[0].clone();
        v_w = t5261[1].clone();
        if ({ let t5234 = v_k.clone(); Py::Bool({ let t5235 = v_outputs.clone(); if !py::contains(&t5235, &t5234)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5262(mut v_kw: Py) -> R {
    return Ok({ let t5275 = f__from_si(Py::dict(vec![(Py::str("VO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)])), (Py::str("VCO2"), Py::tuple(vec![Py::Float(1.6666666666666667e-08), Py::Float(0.0)]))]), v_kw.clone())?; f_weir_rq(py::getitem(&t5275, &Py::str("VO2"))?, py::getitem(&t5275, &Py::str("VCO2"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5282(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5284: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5262(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5284 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5285 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5313 = (t5285).items()?;
        if t5313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5313[0].clone();
        v_w = t5313[1].clone();
        if ({ let t5286 = v_k.clone(); Py::Bool({ let t5287 = v_outputs.clone(); if !py::contains(&t5287, &t5286)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5314(mut v_kw: Py) -> R {
    return Ok({ let t5327 = f__from_si(Py::dict(vec![(Py::str("r"), Py::tuple(vec![Py::Float(0.001), Py::Float(0.0)])), (Py::str("rho"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]))]), v_kw.clone())?; f_womersley(py::getitem(&t5327, &Py::str("r"))?, py::getitem(&t5327, &Py::str("HR"))?, py::getitem(&t5327, &Py::str("rho"))?, py::getitem(&t5327, &Py::str("eta"))?)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5334(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5336: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = s_5314(v_inputs.clone())?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5336 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5337 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5365 = (t5337).items()?;
        if t5365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5365[0].clone();
        v_w = t5365[1].clone();
        if ({ let t5338 = v_k.clone(); Py::Bool({ let t5339 = v_outputs.clone(); if !py::contains(&t5339, &t5338)? { true } else { false } }) }).truthy() {
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
