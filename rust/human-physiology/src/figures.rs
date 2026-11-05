//! Human Physiology's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: value maps over two inputs with iso-lines, one output against one input, bars over the options of a
//! choice (docs/FIGURE_PARITY.md).

use crate::figs_common::{self as c, FieldSpec};
use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;

/// The field maps: capability, output shown, input across, input up, logarithmic (across, up), title.
const FIELDS: &[(&str, &str, &str, &str, (bool, bool), &str)] = &[
    ("hp_alveolar_laplace_pressure", "P_cmH2O", "gamma", "r", (false, false),
     "Laplace pressure of an alveolus over surface tension and radius"),
    ("hp_alveolar_ventilation_co2", "VA", "PaCO2", "VCO2", (false, false),
     "Alveolar ventilation over arterial PCO2 and CO2 output"),
    ("hp_blood_velocity_area", "v", "Q", "A", (false, false), "Mean blood velocity over flow and cross-sectional area"),
    ("hp_bmi", "BMI", "height", "weight", (false, false), "Body mass index over height and weight"),
    ("hp_bmr_katch_mcardle", "BMR", "weight", "body_fat", (false, false),
     "Basal metabolic rate (Katch-McArdle) over weight and body fat"),
    ("hp_body_surface_area", "BSA_Mosteller", "weight", "height", (false, false),
     "Body surface area (Mosteller) over weight and height"),
    ("hp_cardiac_output", "CO", "HR", "SV", (false, false), "Cardiac output over heart rate and stroke volume"),
    ("hp_egfr_ckd_epi_2021", "eGFR", "Scr", "age", (true, false),
     "Estimated GFR (CKD-EPI 2021) over serum creatinine and age"),
    ("hp_fev1_fvc_ratio", "ratio", "FVC", "FEV1", (false, false), "FEV1/FVC ratio over the two volumes"),
    ("hp_filtration_fraction", "FF", "GFR", "RPF", (false, false),
     "Filtration fraction over filtration rate and renal plasma flow"),
    ("hp_gibbs_donnan", "Na_in", "c", "A", (false, true),
     "Gibbs-Donnan equilibrium: Na+ inside over bath NaCl and impermeant anion"),
    ("hp_hydrostatic_pressure", "dP_mmHg", "h", "rho", (false, false),
     "Hydrostatic pressure of a blood column over height and density"),
    ("hp_lean_body_mass_boer", "LBM", "weight", "height", (false, false), "Lean body mass (Boer) over weight and height"),
    ("hp_mean_arterial_pressure", "MAP", "SBP", "DBP", (false, false),
     "Mean arterial pressure over systolic and diastolic pressure"),
    ("hp_ph_henderson_hasselbalch", "pH", "HCO3", "PaCO2", (false, false),
     "Blood pH (Henderson-Hasselbalch) over bicarbonate and PCO2"),
    ("hp_plasma_osmolality", "calculated", "Na", "glucose", (false, false),
     "Calculated plasma osmolality over sodium and glucose"),
    ("hp_qrs_axis", "axis", "lead_I", "aVF", (true, false), "Mean QRS axis over the deflections in leads I and aVF"),
    ("hp_qt_correction", "QTc_Bazett", "QT", "HR", (false, false), "Corrected QT interval (Bazett) over QT and heart rate"),
    ("hp_radius_flow_ratio", "flow_ratio", "r1", "r2", (false, false),
     "Flow ratio after a change of vessel radius over the two radii"),
    ("hp_stroke_volume_ef", "EF", "EDV", "ESV", (false, false),
     "Ejection fraction over end-diastolic and end-systolic volume"),
    ("hp_teichholz_volumes", "EF", "LVIDs", "LVIDd", (false, false),
     "Ejection fraction (Teichholz) over the two internal diameters"),
    ("hp_vascular_compliance", "C", "dV", "dP", (false, false), "Compliance over volume and pressure change"),
    ("hp_weir_energy_expenditure", "EE", "VO2", "VCO2", (false, false),
     "Energy expenditure (Weir) over O2 consumption and CO2 production"),
];

/// The sweeps: capability, kind, output, input, title.
const SWEEPS: &[(&str, &str, &str, &str, &str)] = &[
    ("hp_bernoulli_pressure_drop", "dp-mmhg-vs-v1", "dP_mmHg", "v1", "Bernoulli pressure drop against upstream velocity"),
    ("hp_body_fat_from_density", "fat-siri-vs-density", "fat_Siri", "density", "Body fat (Siri) against body density"),
    ("hp_fluid_shift", "osm-final-vs-tbw", "osm_final", "TBW", "Final osmolality against total body water"),
    ("hp_nernst_potential", "e-vs-cout", "E", "Cout", "Nernst potential against the outside concentration"),
];

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v: Vec<Option<Figure>> = vec![];
    if let Some(f) = FIELDS.iter().find(|f| f.0 == s.key) {
        v.push(c::field_map(s, &FieldSpec { z: f.1, x: f.2, y: f.3, log: f.4, title: f.5, zunit: "" }));
    }
    if let Some(w) = SWEEPS.iter().find(|w| w.0 == s.key) {
        v.push(c::sweep_around(s, w.1, w.4, w.2, w.3, false));
    }
    if s.key == "hp_vessels_series_parallel" {
        v.push(c::option_bars(s, "bars", "Total resistance of vascular beds in series or in parallel", "R_total",
                              "arrangement"));
    }
    v.into_iter().flatten().collect()
}
