# human-physiology-ai999x — offline, verified human physiology solver with step-by-step answers

[![tests](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/human-physiology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/human-physiology-ai999x)](https://pypi.org/project/human-physiology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/human-physiology-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/human-physiology-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/human-physiology-ai999x/)** · **[Guides](https://huggingface.co/AI999X/human-physiology-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/human-physiology-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Life Sciences** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

human-physiology-ai999x is an offline, deterministic Python solver that answers human physiology questions with verified results from 96 certified capabilities, gives 692 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**human-physiology-ai999x** is a compact, deterministic human physiology engine for Python. It reads a question in plain English (or a typed call), picks a certified human physiology capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** human-physiology-ai999x comes as the AI999X desktop app and as the command-line program `human-physiology` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Human Physiology desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-human-physiology-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-human-physiology-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`human-physiology`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install human-physiology-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Human Physiology** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Human Physiology among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-human-physiology-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-human-physiology-1.4.0-linux-x86_64.tar.gz
./ai999x-human-physiology-1.4.0-linux-x86_64/ai999x-human-physiology
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
human-physiology --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
human-physiology --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ human-physiology ask "cardiac output: HR = 72, SV = 70 mL"
Read as: Cardiac output from heart rate and stroke volume
CO = 5.04
CO_mL = 5040
  by: Cardiac output from heart rate and stroke volume  (CO = HR SV / 1000)
  HR = 72
  SV = 7e-05 m^3
  assumes: HR > 0; SV > 0
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ human-physiology info hp_cardiac_output
Cardiac output from heart rate and stroke volume
  CO = HR SV / 1000
  in   HR       heart rate in beats/min []
  in   SV       stroke volume (e.g. in mL) [m^3]
  out  CO       cardiac output in L/min []
  out  CO_mL    cardiac output in mL/min []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ human-physiology solve hp_cardiac_output HR=72.0 "SV=70 mL"
CO = 5.04
CO_mL = 5040
  by: Cardiac output from heart rate and stroke volume  (CO = HR SV / 1000)
  HR = 72
  SV = 7e-05 m^3
  assumes: HR > 0; SV > 0
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | human-physiology batch

`cases.jsonl`:

```json
{"key": "hp_cardiac_output", "inputs": {"HR": 72.0, "SV": "70 mL"}}
{"key": "hp_alveolar_gas_equation", "inputs": {"FiO2": 0.21, "Patm": "760 mmHg", "PH2O": "47 mmHg", "PaCO2": "40 mmHg", "RQ": 0.8}}
```

```console
$ human-physiology batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":5.04,"unit":"","solved_for":"CO","si":[["HR",72.0],["SV",0.00007]],"values":[["CO",5.04,""],["CO_mL",5040.0,""]],"target":"CO","phrase":"cardiac output in L/min","assumptions":["HR > 0","SV > 0"],"name":"Cardiac output from heart rate and stroke volume","equation":"CO = HR SV / 1000"}
{"outcome":"SOLVED","message":"","value":99.73,"unit":"","solved_for":"PAO2","si":[["FiO2",0.21],["Patm",101325.01443540001],["PH2O",6266.152208505],["PaCO2",5332.8954966],["RQ",0.8]],"values":[["PiO2",149.73,""],["PAO2",99.73,""],["PAO2_full",101.83,""]],"target":"PAO2","phrase":"inspired PO2 in mmHg","assumptions":["0 < FiO2 <= 1","Patm > PH2O","0 < RQ <= 1.5"],"name":"Alveolar gas equation","equation":"PAO2 = PiO2 - PaCO2/R (+ PaCO2 FiO2 (1 - R)/R)"}
```

### In your browser

Open [Human Physiology on ai999x.com](https://www.ai999x.com/models/human-physiology-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `human_physiology_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install human-physiology-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Body density and fat by hydrostatic weighing

Body volume (Ma - Mw)/rho_w - RV - GI gas, body density and Siri percent fat.

Command line:

```console
$ human-physiology solve hp_hydrostatic_weighing "mass_air=70 kg" "mass_water=3 kg" "water_density=0.9957 g/mL" "RV=1.2 L"
volume = 65.98934418
density = 1.060777325
fat_Siri = 16.63893384
  by: Body density and fat by hydrostatic weighing  (Archimedes' principle, then Siri)
  mass_air = 70 kg
  mass_water = 3 kg
  water_density = 995.7 kg/m^3
  RV = 0.0012 m^3
  assumes: mass_water < mass_air; computed density 0.9-1.2
```

Python:

```python
import human_physiology_ai999x as m

a = m.solve("hp_hydrostatic_weighing", mass_air="70 kg", mass_water="3 kg", water_density="0.9957 g/mL", RV="1.2 L")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_hydrostatic_weighing) and press **Compute in your browser**.

### Body fat from body density (Siri and Brozek)

Percent body fat by Siri (495/D - 450) and Brozek (457/D - 414.2) from whole-body density.

Command line:

```console
$ human-physiology solve hp_body_fat_from_density "density=1.05 g/mL"
fat_Siri = 21.42857143
fat_Brozek = 21.03809524
  by: Body fat from body density (Siri and Brozek)  (two-compartment models)
  density = 1050 kg/m^3
  assumes: 0.9 < density < 1.2
```

Python:

```python
import human_physiology_ai999x as m

a = m.solve("hp_body_fat_from_density", density="1.05 g/mL")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_body_fat_from_density) and press **Compute in your browser**.

### Body mass index

BMI = weight/height^2 (kg/m^2).

Command line:

```console
$ human-physiology solve hp_bmi "weight=70 kg" "height=175 cm"
BMI = 22.85714286
  by: Body mass index  (BMI = W / H^2 (H in m))
  weight = 70 kg
  height = 1.75 m
  assumes: weight, height > 0
```

Python:

```python
import human_physiology_ai999x as m

a = m.solve("hp_bmi", weight="70 kg", height="175 cm")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_bmi) and press **Compute in your browser**.

### Body surface area (DuBois, Mosteller, Haycock, Gehan-George)

Body surface area by the DuBois (1916), Mosteller (1987), Haycock (1978) and Gehan-George (1970) formulas.

Command line:

```console
$ human-physiology solve hp_body_surface_area "weight=70 kg" "height=175 cm"
BSA_DuBois = 1.848143018
BSA_Mosteller = 1.844661968
BSA_Haycock = 1.846776349
BSA_GehanGeorge = 1.853853298
  by: Body surface area (DuBois, Mosteller, Haycock, Gehan-George)  (published power-law formulas)
  weight = 70 kg
  height = 1.75 m
  assumes: weight, height > 0
```

Python:

```python
import human_physiology_ai999x as m

a = m.solve("hp_body_surface_area", weight="70 kg", height="175 cm")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/human-physiology-ai999x/#cap-hp_body_surface_area) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/human-physiology-ai999x/blob/main/docs/human_physiology_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/human-physiology-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified human physiology capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 692 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `human-physiology` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** cardiovascular, respiratory, renal, nerve and muscle physiology calculations.

## Model overview

| Property | Value |
|---|---|
| Package | `human-physiology-ai999x` 1.1.0 (import `human_physiology_ai999x`) |
| Type | Deterministic neuro-symbolic human physiology engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 96 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 692 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `human-physiology-ai999x "<question>"`; for the native program use `human-physiology` (above). The outputs above were produced by running this exact code on human-physiology-ai999x 1.1.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `human-physiology-ai999x`; the answer is the one it returned, with the steps it took.

**Cardiac output from heart rate and stroke volume** · _Cardiovascular_

> cardiac output: HR = 72, SV = 70 mL

→ **CO = 5.04, CO_mL = 5040**

<details><summary>Steps</summary>

- Cardiac output from heart rate and stroke volume: HR = 72, SV = 70 mL → CO = 5.04, CO_mL = 5040

</details>

**Alveolar gas equation** · _Respiratory_

> alveolar gas equation: FiO2 = 21%, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8

→ **PiO2 = 149.73, PAO2 = 99.73, PAO2_full = 101.83**

<details><summary>Steps</summary>

- Alveolar gas equation: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8 → PiO2 = 149.73, PAO2 = 99.73, PAO2_full = 101.83

</details>

**Nernst equilibrium potential of an ion** · _Nerve and muscle_

> Nernst potential: z = 1, Cout = 4, Cin = 140

→ **E = -95.02257566, RT_F = 26.72665911**

<details><summary>Steps</summary>

- Nernst equilibrium potential of an ion: z = 1, Cout = 4, Cin = 140 → E = -95.02257566, RT_F = 26.72665911

</details>

**Poiseuille flow, mean velocity and wall shear stress in a vessel** · _Cardiovascular_

`m.solve("hp_poiseuille_flow", dP="10 mmHg", eta=3.5, L="10 cm", r="2 mm")`

→ **Q = 23.93397902, Q_mL_min = 1436.038741, v_mean = 190.4605534, tau_wall = 13.33223874**

<details><summary>Steps</summary>

- Poiseuille flow, mean velocity and wall shear stress in a vessel: dP = 10 mmHg, eta = 3.5, L = 10 cm, r = 2 mm → Q = 23.93397902, Q_mL_min = 1436.038741, v_mean = 190.4605534, tau_wall = 13.33223874

</details>

**Estimated GFR by the CKD-EPI 2021 creatinine equation** · _Renal_

> eGFR CKD-EPI 2021: Scr = 1.0, age = 50, sex = male

→ **eGFR = 91.69147861**

<details><summary>Steps</summary>

- Estimated GFR by the CKD-EPI 2021 creatinine equation: Scr = 1, age = 50, sex = male → eGFR = 91.69147861

</details>

**Body mass index** · _Body composition_

> body mass index: weight = 70 kg, height = 175 cm

→ **BMI = 22.85714286**

<details><summary>Steps</summary>

- Body mass index: weight = 70 kg, height = 175 cm → BMI = 22.85714286

</details>

## Figures

Every point was computed by `human-physiology-ai999x`.

**Nernst equilibrium potential of an ion**

![Nernst equilibrium potential of an ion — computed by human-physiology-ai999x](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-nernst-potential-e-vs-cout.svg)

_E (equilibrium potential in mV) against Cout (extracellular concentration in mM), with Cin = 140, z = 1. 31 points computed by human-physiology-ai999x; marked: Cout = 4, E = -95.0226._

**Body fat from body density (Siri and Brozek)**

![Body fat from body density (Siri and Brozek) — computed by human-physiology-ai999x](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-body-fat-from-density-fat-siri-vs-density.svg)

_fat_Siri (body fat in % (Siri)) against density (body density (e.g. in g/mL)). 9 points computed by human-physiology-ai999x; marked: density = 1050, fat_Siri = 21.4286._

**Two-compartment fluid shift after adding a solution (Darrow-Yannet)**

![Two-compartment fluid shift after adding a solution (Darrow-Yannet) — computed by human-physiology-ai999x](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-fluid-shift-osm-final-vs-tbw.svg)

_osm_final (final osmolality in mOsm/kg) against TBW (initial total body water (e.g. in L)), with C_added = 0, Posm = 290, V_added = 0.002 m^3. 31 points computed by human-physiology-ai999x; marked: TBW = 0.042, osm_final = 276.818._

**Bernoulli pressure drop across a narrowing**

![Bernoulli pressure drop across a narrowing — computed by human-physiology-ai999x](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-bernoulli-pressure-drop-dp-mmhg-vs-v1.svg)

_dP_mmHg (pressure drop in mmHg) against v1 (upstream velocity (e.g. in m/s)), with rho = 1060 kg/m^3, v2 = 4 m/s. 31 points computed by human-physiology-ai999x; marked: v1 = 1, dP_mmHg = 59.6299._

## Capabilities

96 certified procedures. A few of them:

- **Procedures:** ATPS to BTPS and STPD gas volume conversion; Alveolar gas equation; Alveolar ventilation from CO2 output and arterial PCO2; Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio; Arterial oxygen content; Barometric pressure and inspired PO2 at altitude; Basal metabolic rate by the Harris-Benedict equation; Basal metabolic rate by the Katch-McArdle equation; Bernoulli pressure drop across a narrowing; Bicarbonate from pH and PCO2 (Henderson-Hasselbalch); Blood pH by the Henderson-Hasselbalch equation; Body density and fat by hydrostatic weighing; Body fat from body density (Siri and Brozek); Body fluid compartments; Body mass index; Body surface area (DuBois, Mosteller, Haycock, Gehan-George); Calculated and effective plasma osmolality and the osmolal gap; Cardiac index and stroke volume index; Cardiac output by the Fick principle; Cardiac output from heart rate and stroke volume; Charge and number of ions to change the membrane potential; Compliance and elastance; Conduction velocity scaling with fibre diameter; Creatinine clearance by the Cockcroft-Gault equation …

`human_physiology_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified human physiology capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.
- **Not medical advice.** It computes textbook quantities from the numbers you give it; it does not diagnose, treat or recommend doses, and its results are no substitute for a qualified clinician.

## Limitations

- It answers what its certified capabilities cover (cardiovascular, respiratory, renal, nerve and muscle physiology calculations); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{human_physiology_ai999x_1_1_0,
  title  = {human-physiology-ai999x: an offline, verified human physiology solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
