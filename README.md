# human-physiology-ai999x — offline, verified human physiology solver with step-by-step answers

[![tests](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/human-physiology-ai999x)](https://pypi.org/project/human-physiology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/human-physiology-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/human-physiology-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/human-physiology-ai999x/)** · **[Guides](https://huggingface.co/AI999X/human-physiology-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/human-physiology-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

human-physiology-ai999x is an offline, deterministic Python solver that answers human physiology questions with verified results from 96 certified capabilities, gives 692 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**human-physiology-ai999x** is a compact, deterministic human physiology engine for Python. It reads a question in plain English (or a typed call), picks a certified human physiology capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified human physiology capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 692 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** cardiovascular, respiratory, renal, nerve and muscle physiology calculations.

## Model overview

| Property | Value |
|---|---|
| Package | `human-physiology-ai999x` 1.0.0 (import `human_physiology_ai999x`) |
| Type | Deterministic neuro-symbolic human physiology engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 96 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 692 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install human-physiology-ai999x
```

```python
import human_physiology_ai999x as human_physiology

human_physiology.ask("cardiac output: HR = 72, SV = 70 mL").show()
# -> SOLVED — CO = 5.04, CO_mL = 5040  (hp_cardiac_output)
human_physiology.ask("alveolar gas equation: FiO2 = 21%, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8").show()
# -> SOLVED — PiO2 = 149.73, PAO2 = 99.73, PAO2_full = 101.83  (hp_alveolar_gas_equation)
human_physiology.ask("Nernst potential: z = 1, Cout = 4, Cin = 140").show()
# -> SOLVED — E = -95.0226, RT_F = 26.7267  (hp_nernst_potential)
human_physiology.solve("hp_poiseuille_flow", dP="10 mmHg", eta=3.5, L="10 cm", r="2 mm").show()
# -> SOLVED — Q = 23.934, Q_mL_min = 1436.04, v_mean = 190.461, tau_wall = 13.3322  (hp_poiseuille_flow)
human_physiology.ask("eGFR CKD-EPI 2021: Scr = 1.0, age = 50, sex = male").show()
# -> SOLVED — eGFR = 91.6915  (hp_egfr_ckd_epi_2021)
human_physiology.ask("Who created you?").message
# -> human-physiology-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `human-physiology-ai999x "<question>"`, `human-physiology-ai999x --capabilities`. The outputs above were produced by running this exact code on human-physiology-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

96 certified procedures. A few of them:

- **Procedures:** ATPS to BTPS and STPD gas volume conversion; Alveolar gas equation; Alveolar ventilation from CO2 output and arterial PCO2; Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio; Arterial oxygen content; Barometric pressure and inspired PO2 at altitude; Basal metabolic rate by the Harris-Benedict equation; Basal metabolic rate by the Katch-McArdle equation; Bernoulli pressure drop across a narrowing; Bicarbonate from pH and PCO2 (Henderson-Hasselbalch); Blood pH by the Henderson-Hasselbalch equation; Body density and fat by hydrostatic weighing; Body fat from body density (Siri and Brozek); Body fluid compartments; Body mass index; Body surface area (DuBois, Mosteller, Haycock, Gehan-George); Calculated and effective plasma osmolality and the osmolal gap; Cardiac index and stroke volume index; Cardiac output by the Fick principle; Cardiac output from heart rate and stroke volume; Charge and number of ions to change the membrane potential; Compliance and elastance; Conduction velocity scaling with fibre diameter; Creatinine clearance by the Cockcroft-Gault equation …

`human_physiology_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified human physiology capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (cardiovascular, respiratory, renal, nerve and muscle physiology calculations); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{human_physiology_ai999x_1_0_0,
  title  = {human-physiology-ai999x: an offline, verified human physiology solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/human-physiology-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/human-physiology-ai999x/discussions or visit https://ai999x.com.

## Support the project

human-physiology-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/human-physiology-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/human-physiology-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/human-physiology-ai999x/tree/main/onnx).
