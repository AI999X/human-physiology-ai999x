//! Every Human Physiology capability's own live figures (src/figures.rs): its worked example gives each published
//! figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its
//! canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use human_physiology_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/human-physiology-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("hp_alveolar_laplace_pressure", &["field"]),
    ("hp_alveolar_ventilation_co2", &["field"]),
    ("hp_bernoulli_pressure_drop", &["dp-mmhg-vs-v1"]),
    ("hp_blood_velocity_area", &["field"]),
    ("hp_bmi", &["field"]),
    ("hp_bmr_katch_mcardle", &["field"]),
    ("hp_body_fat_from_density", &["fat-siri-vs-density"]),
    ("hp_body_surface_area", &["field"]),
    ("hp_cardiac_output", &["field"]),
    ("hp_egfr_ckd_epi_2021", &["field"]),
    ("hp_fev1_fvc_ratio", &["field"]),
    ("hp_filtration_fraction", &["field"]),
    ("hp_fluid_shift", &["osm-final-vs-tbw"]),
    ("hp_gibbs_donnan", &["field"]),
    ("hp_hydrostatic_pressure", &["field"]),
    ("hp_lean_body_mass_boer", &["field"]),
    ("hp_mean_arterial_pressure", &["field"]),
    ("hp_nernst_potential", &["e-vs-cout"]),
    ("hp_ph_henderson_hasselbalch", &["field"]),
    ("hp_plasma_osmolality", &["field"]),
    ("hp_qrs_axis", &["field"]),
    ("hp_qt_correction", &["field"]),
    ("hp_radius_flow_ratio", &["field"]),
    ("hp_stroke_volume_ef", &["field"]),
    ("hp_teichholz_volumes", &["field"]),
    ("hp_vascular_compliance", &["field"]),
    ("hp_vessels_series_parallel", &["bars"]),
    ("hp_weir_energy_expenditure", &["field"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("hp_vessels_series_parallel", "R", "[4, 5, 10]"),
    ("hp_body_fat_from_density", "density", "1080 kg/m^3"),
];

/// The capability refused by the model for these inputs gives no figure (capability, inputs).
const REFUSED: (&str, &[(&str, &str)]) = ("hp_stroke_volume_ef", &[("EDV", "50 mL"), ("ESV", "120 mL")]);

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            i += 1;
            value = cs[s0..i].iter().collect();
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The numbers of a text multiplied by f ("12 V" -> "16.44 V"; "[3, 6]" -> "[4.11, 8.22]"; "8e-05 m^3/s" ->
/// "0.0001096 m^3/s"): a number that follows a letter or "^" (a unit's power) is kept.
fn scale_text(t: &str, f: f64) -> Option<String> {
    let cs: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let mut changed = false;
    let mut i = 0;
    while i < cs.len() {
        let prev = if i == 0 { ' ' } else { cs[i - 1] };
        let starts = cs[i].is_ascii_digit() || (cs[i] == '-' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit());
        if starts && !(prev.is_alphanumeric() || prev == '^' || prev == '.' || prev == '_') {
            let s0 = i;
            i += 1;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            if i + 1 < cs.len() && (cs[i] == 'e' || cs[i] == 'E')
                && (cs[i + 1].is_ascii_digit() || ((cs[i + 1] == '-' || cs[i + 1] == '+') && i + 2 < cs.len())) {
                i += 2;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let tok: String = cs[s0..i].iter().collect();
            match tok.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    changed = true;
                }
                _ => out.push_str(&tok),
            }
        } else {
            out.push(cs[i]);
            i += 1;
        }
    }
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73, 2.0] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    let p = pack();
    let (key, inputs) = REFUSED;
    let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, key, &given, None);
    assert!(!a.solved(), "{key}: expected a refusal");
    assert!(figures(p, key, &given, None, None).is_empty());
}
