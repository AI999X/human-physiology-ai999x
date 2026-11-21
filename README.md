# Human Physiology (human-physiology-ai999x)

[![tests](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/human-physiology-ai999x)](https://pypi.org/project/human-physiology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/human-physiology-ai999x)

Human Physiology covers cardiac output, blood pressure, vascular resistance and Poiseuille flow, lung volumes, ventilation and the alveolar gas equation, oxygen content and binding, acid-base, renal clearance and eGFR, Nernst and Goldman-Hodgkin-Katz potentials, cable properties, body composition and metabolic rate (computed values, not medical advice): 98 calculations in 8 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Human Physiology desktop app: Cardiac output from heart rate and stroke volume, with its worked example and live scene](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/hero.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-human-physiology-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-human-physiology-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `human-physiology` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install human-physiology-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Human Physiology desktop app: every scene and chart is drawn from the computed answer.

[![Human Physiology desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/clip.mp4)*

| | |
|---|---|
| [![Cardiac output from heart rate and stroke volume: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-1.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_cardiac_output)<br>Cardiac output from heart rate and stroke volume | [![Alveolar gas equation: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-2.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_alveolar_gas_equation)<br>Alveolar gas equation |
| [![Nernst equilibrium potential of an ion: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-3.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_nernst_potential)<br>Nernst equilibrium potential of an ion | [![Estimated GFR by the CKD-EPI 2021 creatinine equation: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-4.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_egfr_ckd_epi_2021)<br>Estimated GFR by the CKD-EPI 2021 creatinine equation |
| [![Red-cell indices: MCV, MCH and MCHC: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-5.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_red_cell_indices)<br>Red-cell indices: MCV, MCH and MCHC | [![Body mass index: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-6.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_bmi)<br>Body mass index |
| [![Body fluid compartments: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-7.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_body_fluid_compartments)<br>Body fluid compartments | [![Conduction velocity scaling with fibre diameter: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-8.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_conduction_velocity_scaling)<br>Conduction velocity scaling with fibre diameter |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Cardiac output from heart rate and stroke volume: a chart in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-1.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_cardiac_output)<br>Cardiac output from heart rate and stroke volume | [![Alveolar gas equation: a chart in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-2.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_alveolar_gas_equation)<br>Alveolar gas equation | [![Nernst equilibrium potential of an ion: a chart in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-3.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_nernst_potential)<br>Nernst equilibrium potential of an ion | [![Estimated GFR by the CKD-EPI 2021 creatinine equation: a chart in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-4.webp)](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_egfr_ckd_epi_2021)<br>Estimated GFR by the CKD-EPI 2021 creatinine equation |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import human_physiology_ai999x as human_physiology

a = human_physiology.ask(
    "red cell indices: Hct = 0.45, Hb = 15, RBC = 5"
)
print(a.outcome, a.values)
# SOLVED {'MCV': 90.0, 'MCH': 30.0, 'MCHC': 33.3333333333}

a = human_physiology.ask(
    "Bernoulli pressure drop: v1 = 1 m/s, v2 = 4 m/s, rho = 1060 "
    "kg/m^3"
)
print(a.outcome, a.values)
# SOLVED {'dP': 7950.0, 'dP_mmHg': 59.6298952797, 'dP_simplified': 64.0}

a = human_physiology.ask(
    "Harris-Benedict BMR: weight = 70 kg, height = 175 cm, age = 30, "
    "sex = male, version = original"
)
print(a.outcome, a.values)
# SOLVED {'BMR': 1702.0125, 'BMR_kJ': 7121.2203}

a = human_physiology.ask(
    "membrane charge: Cm = 1 uF/cm^2, dV = 100 mV, area = 1000 um^2"
)
print(a.outcome, a.values)
# SOLVED {'Q_pC': 1.0, 'ions': 6241509.07446}

a = human_physiology.ask(
    "plasma osmolality: Na = 140 mmol/L, glucose = 90, BUN = 14"
)
print(a.outcome, a.values)
# SOLVED {'calculated': 290.0, 'effective': 285.0}

a = human_physiology.ask(
    "alveolar gas equation: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 "
    "mmHg, PaCO2 = 40 mmHg, RQ = 0.8"
)
print(a.outcome, a.values)
# SOLVED {'PiO2': 149.73, 'PAO2': 99.73, 'PAO2_full': 101.83}

a = human_physiology.ask("body fat from density: density = 1.05 g/mL")
print(a.outcome, a.values)
# SOLVED {'fat_Siri': 21.4285714286, 'fat_Brozek': 21.0380952381}

a = human_physiology.ask(
    "cardiac index: CO = 5.04 L/min, BSA = 1.8 m^2, HR = 72"
)
print(a.outcome, a.values)
# SOLVED {'CI': 2.8, 'SVI': 38.8888888889, 'SV': 70.0}

a = human_physiology.ask(
    "Katch-McArdle BMR: weight = 70 kg, body_fat = 0.2"
)
print(a.outcome, a.values)
# SOLVED {'LBM': 56.0, 'BMR': 1579.6}

a = human_physiology.ask(
    "conduction velocity scaling: v1 = 60 m/s, d1 = 10 um, d2 = 5 um, "
    "fiber = myelinated"
)
print(a.outcome, a.values)
# SOLVED {'v2': 30.0, 'ratio': 0.5}

a = human_physiology.ask(
    "Cockcroft-Gault creatinine clearance: age = 60, weight = 72 kg, "
    "Scr = 1, sex = male"
)
print(a.outcome, a.values)
# SOLVED {'CrCl': 80.0}

a = human_physiology.ask(
    "alveolar ventilation from CO2 output: VCO2 = 200 mL/min, PaCO2 = "
    "40 mmHg"
)
print(a.outcome, a.values)
# SOLVED {'VA': 4.31473549332}

a = human_physiology.ask(
    "body mass index: weight = 70 kg, height = 175 cm"
)
print(a.outcome, a.values)
# SOLVED {'BMI': 22.8571428571}

a = human_physiology.ask(
    "solution osmolarity: conc = 9 g/L, molar_mass = 58.44 g/mol, i = "
    "2"
)
print(a.outcome, a.values)
# SOLVED {'molarity': 154.004106776, 'osmolarity': 308.008213552}

a = human_physiology.ask(
    "Fick cardiac output: VO2 = 250 mL/min, CaO2 = 20, CvO2 = 15"
)
print(a.outcome, a.values)
# SOLVED {'CO': 5.0, 'avDO2': 5.0}
```

## Command line: 15 examples

Each command below was run with `human-physiology` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ human-physiology solve hp_red_cell_indices Hct=0.45 Hb=15 RBC=5
MCV = 90
MCH = 30
MCHC = 33.33333333

$ human-physiology solve hp_hydrostatic_weighing "mass_air=70 kg" "mass_water=3 kg" "water_density=0.9957 g/mL" "RV=1.2 L"
volume = 65.98934418
density = 1.060777325
fat_Siri = 16.63893384

$ human-physiology solve hp_body_fluid_compartments "weight=70 kg"
TBW = 42
ICF = 28
ECF = 14
ISF = 10.5
plasma = 3.5

$ human-physiology solve hp_bernoulli_pressure_drop "v1=1 m/s" "v2=4 m/s" "rho=1060 kg/m^3"
dP = 7950
dP_mmHg = 59.62989528
dP_simplified = 64

$ human-physiology solve hp_bmr_harris_benedict "weight=70 kg" "height=175 cm" age=30 sex=male version=original
BMR = 1702.0125
BMR_kJ = 7121.2203

$ human-physiology solve hp_membrane_charge "Cm=1 uF/cm^2" "dV=100 mV" "area=1000 um^2"
Q_pC = 1
ions = 6241509.074

$ human-physiology solve hp_plasma_osmolality "Na=140 mmol/L" glucose=90 BUN=14
calculated = 290
effective = 285

$ human-physiology solve hp_alveolar_gas_equation FiO2=0.21 "Patm=760 mmHg" "PH2O=47 mmHg" "PaCO2=40 mmHg" RQ=0.8
PiO2 = 149.73
PAO2 = 99.73
PAO2_full = 101.83

$ human-physiology solve hp_body_fat_from_density "density=1.05 g/mL"
fat_Siri = 21.42857143
fat_Brozek = 21.03809524

$ human-physiology solve hp_gibbs_donnan A=20 "c=150 mmol/L"
Na_in = 160.3329638
Cl_in = 140.3329638
r = 1.068886425
V_mV = -1.780460072
osmotic_excess = 0.6659275675

$ human-physiology solve hp_cardiac_index "CO=5.04 L/min" "BSA=1.8 m^2" HR=72
CI = 2.8
SVI = 38.88888889
SV = 70

$ human-physiology solve hp_bmr_katch_mcardle "weight=70 kg" body_fat=0.2
LBM = 56
BMR = 1579.6

$ human-physiology solve hp_conduction_velocity_scaling "v1=60 m/s" "d1=10 um" "d2=5 um" fiber=myelinated
v2 = 30
ratio = 0.5

$ human-physiology solve hp_cockcroft_gault age=60 "weight=72 kg" Scr=1 sex=male
CrCl = 80

$ human-physiology solve hp_alveolar_ventilation_co2 "VCO2=200 mL/min" "PaCO2=40 mmHg"
VA = 4.314735493
```

## Rust library: 15 examples

The crate `human-physiology-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `human_physiology_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/human-physiology/examples/readme.rs`: `cd rust && cargo run --release --example readme -p human-physiology-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&human_physiology_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "hp_red_cell_indices", "inputs": {"Hct": "0.45", "Hb": "15", "RBC": "5"}}));
// SOLVED
// MCV = 90.0
// MCH = 30.0
// MCHC = 33.3333333333

show(json!({"key": "hp_hydrostatic_weighing", "inputs": {"mass_air": "70 kg", "mass_water": "3 kg", "water_density": "0.9957 g/mL", "RV": "1.2 L"}}));
// SOLVED
// volume = 65.98934418
// density = 1.06077732503
// fat_Siri = 16.6389338441

show(json!({"key": "hp_body_fluid_compartments", "inputs": {"weight": "70 kg"}}));
// SOLVED
// TBW = 42.0
// ICF = 28.0
// ECF = 14.0
// ISF = 10.5
// plasma = 3.5

show(json!({"key": "hp_bernoulli_pressure_drop", "inputs": {"v1": "1 m/s", "v2": "4 m/s", "rho": "1060 kg/m^3"}}));
// SOLVED
// dP = 7950.0
// dP_mmHg = 59.6298952797
// dP_simplified = 64.0

show(json!({"key": "hp_bmr_harris_benedict", "inputs": {"weight": "70 kg", "height": "175 cm", "age": "30", "sex": "male", "version": "original"}}));
// SOLVED
// BMR = 1702.0125
// BMR_kJ = 7121.2203

show(json!({"key": "hp_membrane_charge", "inputs": {"Cm": "1 uF/cm^2", "dV": "100 mV", "area": "1000 um^2"}}));
// SOLVED
// Q_pC = 1.0
// ions = 6241509.07446

show(json!({"key": "hp_plasma_osmolality", "inputs": {"Na": "140 mmol/L", "glucose": "90", "BUN": "14"}}));
// SOLVED
// calculated = 290.0
// effective = 285.0

show(json!({"key": "hp_alveolar_gas_equation", "inputs": {"FiO2": "0.21", "Patm": "760 mmHg", "PH2O": "47 mmHg", "PaCO2": "40 mmHg", "RQ": "0.8"}}));
// SOLVED
// PiO2 = 149.73
// PAO2 = 99.73
// PAO2_full = 101.83

show(json!({"key": "hp_body_fat_from_density", "inputs": {"density": "1.05 g/mL"}}));
// SOLVED
// fat_Siri = 21.4285714286
// fat_Brozek = 21.0380952381

show(json!({"key": "hp_gibbs_donnan", "inputs": {"A": "20", "c": "150 mmol/L"}}));
// SOLVED
// Na_in = 160.332963784
// Cl_in = 140.332963784
// r = 1.06888642522
// V_mV = -1.78046007183
// osmotic_excess = 0.665927567458

show(json!({"key": "hp_cardiac_index", "inputs": {"CO": "5.04 L/min", "BSA": "1.8 m^2", "HR": "72"}}));
// SOLVED
// CI = 2.8
// SVI = 38.8888888889
// SV = 70.0

show(json!({"key": "hp_bmr_katch_mcardle", "inputs": {"weight": "70 kg", "body_fat": "0.2"}}));
// SOLVED
// LBM = 56.0
// BMR = 1579.6

show(json!({"key": "hp_conduction_velocity_scaling", "inputs": {"v1": "60 m/s", "d1": "10 um", "d2": "5 um", "fiber": "myelinated"}}));
// SOLVED
// v2 = 30.0
// ratio = 0.5

show(json!({"key": "hp_cockcroft_gault", "inputs": {"age": "60", "weight": "72 kg", "Scr": "1", "sex": "male"}}));
// SOLVED
// CrCl = 80.0

show(json!({"key": "hp_alveolar_ventilation_co2", "inputs": {"VCO2": "200 mL/min", "PaCO2": "40 mmHg"}}));
// SOLVED
// VA = 4.31473549332
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Human Physiology on ai999x.com](https://www.ai999x.com/models/human-physiology-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/human-physiology-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/human-physiology-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `human-physiology` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/human-physiology`) and its data (`rust/models/human_physiology/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p human-physiology-ai999x
./target/release/human-physiology --help
cargo test -p human-physiology-ai999x
```
