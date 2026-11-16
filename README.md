# Human Physiology (human-physiology-ai999x)

[![tests](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/human-physiology-ai999x)](https://pypi.org/project/human-physiology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/human-physiology-ai999x)

Human Physiology covers cardiac output, blood pressure, vascular resistance and Poiseuille flow, lung volumes, ventilation and the alveolar gas equation, oxygen content and binding, acid-base, renal clearance and eGFR, Nernst and Goldman-Hodgkin-Katz potentials, cable properties, body composition and metabolic rate (computed values, not medical advice): 98 calculations in 8 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Human Physiology desktop app: Bernoulli pressure drop across a narrowing, with its worked example and live scene](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/hero.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `human-physiology` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#try) |
| **Python** 3.10+ | `pip install human-physiology-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Human Physiology desktop app: Bernoulli pressure drop across a narrowing, with its worked example and live scene](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/hero-dark.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

[![ATPS to BTPS and STPD gas volume conversion: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/see-1.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

*ATPS to BTPS and STPD gas volume conversion*

[![Charge and number of ions to change the membrane potential: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/see-2.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

*Charge and number of ions to change the membrane potential*

[![Calculated and effective plasma osmolality and the osmolal gap: the live scene in the Human Physiology desktop app](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/see-3.webp)](https://www.ai999x.com/models/human-physiology-ai999x/)

*Calculated and effective plasma osmolality and the osmolal gap*

Every calculation, with its formula, a worked example and its figures: **[Human Physiology on ai999x.com](https://www.ai999x.com/models/human-physiology-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/human-physiology-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/human-physiology-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `human-physiology` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/human-physiology`) and its data (`rust/models/human_physiology/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p human-physiology-ai999x
./target/release/human-physiology --help
cargo test -p human-physiology-ai999x
```
