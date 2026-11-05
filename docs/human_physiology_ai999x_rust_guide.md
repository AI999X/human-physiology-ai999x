# human-physiology: the Human Physiology program, user guide

**human-physiology-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Human Physiology covers cardiac output, blood pressure, vascular resistance and Poiseuille flow, lung volumes, ventilation and the alveolar gas equation, oxygen content and binding, acid-base, renal clearance and eGFR, Nernst and Goldman-Hodgkin-Katz potentials, cable properties, body composition and metabolic rate (computed values, not medical advice). This guide is about `human-physiology`, the model as a self-contained program for Windows and Linux: 96 capabilities in 7 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/human-physiology-ai999x/); the Python package (`pip install human-physiology-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Human Physiology | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-human-physiology-1.5.0-windows-x86_64.zip |
| Desktop app: Human Physiology | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-human-physiology-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Human Physiology in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-human-physiology-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-human-physiology.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-human-physiology-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-human-physiology-1.5.0-linux-x86_64.tar.gz
./ai999x-human-physiology-1.5.0-linux-x86_64/ai999x-human-physiology
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `human-physiology` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
human-physiology --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
human-physiology --version
```

### In your browser

Open https://www.ai999x.com/models/human-physiology-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install human-physiology-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ human-physiology --version
human-physiology-ai999x 1.5.0 (Rust build, ax-engine 1.5.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ human-physiology ask "cardiac output: HR = 72, SV = 70 mL"
Read as: Cardiac output from heart rate and stroke volume
CO = 5.04
CO_mL = 5040
  by: Cardiac output from heart rate and stroke volume  (CO = HR SV / 1000)
  HR = 72
  SV = 7e-05 m^3
  assumes: HR > 0; SV > 0
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ human-physiology search "cardiac output from heart rate and stroke volume"
hp_cardiac_output                    Cardiac output from heart rate and stroke volume  (cardiovascular / cardiac output)
hp_cardiac_index                     Cardiac index and stroke volume index  (cardiovascular / cardiac output)
hp_stroke_work                       Stroke work and cardiac power  (cardiovascular / cardiac work)
hp_stroke_volume_ef                  Stroke volume and ejection fraction  (cardiovascular / cardiac output)
hp_qt_correction                     Heart-rate corrected QT interval (Bazett, Fridericia, Framingham)  (cardiovascular / electrocardiogram)
hp_heart_rate_reserve                Maximal heart rate, heart rate reserve and Karvonen target  (metabolism / exercise)
hp_fick_cardiac_output               Cardiac output by the Fick principle  (cardiovascular / cardiac output)
hp_fick_saturations                  Fick cardiac output from hemoglobin, saturations and PO2  (cardiovascular / cardiac output)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ human-physiology list
hp_aa_gradient                           Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio
hp_alveolar_gas_equation                 Alveolar gas equation
hp_alveolar_laplace_pressure             Laplace pressure of an alveolus
hp_alveolar_ventilation_co2              Alveolar ventilation from CO2 output and arterial PCO2
hp_anion_gap                             Serum anion gap and albumin-corrected anion gap
hp_atwater_energy                        Food energy by the Atwater factors
hp_barometric_altitude                   Barometric pressure and inspired PO2 at altitude
hp_bernoulli_pressure_drop               Bernoulli pressure drop across a narrowing
… 88 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ human-physiology info hp_cardiac_output
Cardiac output from heart rate and stroke volume
  CO = HR SV / 1000
  in   HR       heart rate in beats/min []
  in   SV       stroke volume (e.g. in mL) [m^3]
  out  CO       cardiac output in L/min []
  out  CO_mL    cardiac output in mL/min []
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ human-physiology solve hp_cardiac_output HR=72.0 "SV=70 mL"
CO = 5.04
CO_mL = 5040
  by: Cardiac output from heart rate and stroke volume  (CO = HR SV / 1000)
  HR = 72
  SV = 7e-05 m^3
  assumes: HR > 0; SV > 0
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ human-physiology solve hp_cardiac_output HR=72.0 "SV=70 mL" --json
{"outcome":"SOLVED","message":"","value":5.04,"unit":"","solved_for":"CO","si":[["HR",72.0],["SV",0.00007]],"values":[["CO",5.04,""],["CO_mL",5040.0,""]],"target":"CO","phrase":"cardiac output in L/min","assumptions":["HR > 0","SV > 0"],"name":"Cardiac output from heart rate and stroke volume","equation":"CO = HR SV / 1000"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ human-physiology solve hp_cardiac_output HR=72.0
NEEDS_INPUT: hp_cardiac_output needs SV (stroke volume (e.g. in mL)). Example: solve('hp_cardiac_output', HR=72, SV='7e-05 m^3').
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | human-physiology batch

`cases.jsonl`:

```json
{"key": "hp_cardiac_output", "inputs": {"HR": 72.0, "SV": "70 mL"}}
{"key": "hp_alveolar_gas_equation", "inputs": {"FiO2": 0.21, "Patm": "760 mmHg", "PH2O": "47 mmHg", "PaCO2": "40 mmHg", "RQ": 0.8}}
```

```shell
$ human-physiology batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":5.04,"unit":"","solved_for":"CO","si":[["HR",72.0],["SV",0.00007]],"values":[["CO",5.04,""],["CO_mL",5040.0,""]],"target":"CO","phrase":"cardiac output in L/min","assumptions":["HR > 0","SV > 0"],"name":"Cardiac output from heart rate and stroke volume","equation":"CO = HR SV / 1000"}
{"outcome":"SOLVED","message":"","value":99.73,"unit":"","solved_for":"PAO2","si":[["FiO2",0.21],["Patm",101325.01443540001],["PH2O",6266.152208505],["PaCO2",5332.8954966],["RQ",0.8]],"values":[["PiO2",149.73,""],["PAO2",99.73,""],["PAO2_full",101.83,""]],"target":"PAO2","phrase":"inspired PO2 in mmHg","assumptions":["0 < FiO2 <= 1","Patm > PH2O","0 < RQ <= 1.5"],"name":"Alveolar gas equation","equation":"PAO2 = PiO2 - PaCO2/R (+ PaCO2 FiO2 (1 - R)/R)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `human-physiology info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`human-physiology solve ... --json` prints one answer as one line of JSON. `human-physiology batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | human-physiology batch`. `human-physiology catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["human-physiology", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`human-physiology figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `human_physiology_ai999x.wasm` (https://www.ai999x.com/compute/rust/human_physiology_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("human_physiology_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `human-physiology info <id>` shows the same inputs and outputs in the terminal.

### Body composition (5)

<a id="hp_hydrostatic_weighing"></a>

#### Body density and fat by hydrostatic weighing, `hp_hydrostatic_weighing`

Body volume (Ma - Mw)/rho_w - RV - GI gas, body density and Siri percent fat.

$$
D_b = \frac{M_a}{\frac{M_a - M_w}{\rho_w} - RV - V_{GI}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mass_air` | in | body mass in air (e.g. in kg) | kg |
| `mass_water` | in | underwater mass (e.g. in kg) | kg |
| `water_density` | in | water density (e.g. in g/mL) | kg/m^3 |
| `RV` | in | residual lung volume (e.g. in L) | m^3 |
| `GI` | in | gastrointestinal gas (e.g. in L) (optional; the customary estimate if omitted) (optional) | m^3 |
| `volume` | out | body volume in L | - |
| `density` | out | body density in g/mL | - |
| `fat_Siri` | out | body fat in % (Siri) | - |

**Applies when:** mass_water < mass_air; computed density 0.9-1.2.

```shell
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

**What the worked example means:** This calculation determines body composition by applying Archimedes' principle. The total body volume is derived from the difference between mass in air and underwater mass, adjusted for residual lung volume and gastrointestinal gas. Dividing the mass in air by this corrected volume gives the body density, which is then converted to a body fat percentage using the Siri equation. A result of 16.6389 percent indicates the proportion of adipose tissue relative to total body mass based on these physical measurements.

<a id="hp_body_fat_from_density"></a>

#### Body fat from body density (Siri and Brozek), `hp_body_fat_from_density`

Percent body fat by Siri (495/D - 450) and Brozek (457/D - 414.2) from whole-body density.

$$
\%BF = \frac{495}{D} - 450
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `density` | in | body density (e.g. in g/mL) | kg/m^3 |
| `fat_Siri` | out | body fat in % (Siri) | - |
| `fat_Brozek` | out | body fat in % (Brozek) | - |

**Applies when:** 0.9 < density < 1.2.

```shell
$ human-physiology solve hp_body_fat_from_density "density=1.05 g/mL"
fat_Siri = 21.42857143
fat_Brozek = 21.03809524
  by: Body fat from body density (Siri and Brozek)  (two-compartment models)
  density = 1050 kg/m^3
  assumes: 0.9 < density < 1.2
```

**What the worked example means:** This calculation determines body fat percentage using two-compartment models based on whole-body density. The Siri formula yields 21.4286 while the Brozek formula yields 21.0381, both derived from an input density of 1.05 g/mL. These values represent the estimated proportion of total body mass that is adipose tissue, assuming constant densities for fat and lean tissues.

<a id="hp_bmi"></a>

#### Body mass index: `hp_bmi`

BMI = weight/height^2 (kg/m^2).

$$
BMI = \frac{W}{H^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `height` | in | height (e.g. in cm) | m |
| `BMI` | out | body mass index in kg/m^2 | - |

**Applies when:** weight, height > 0.

```shell
$ human-physiology solve hp_bmi "weight=70 kg" "height=175 cm"
BMI = 22.85714286
  by: Body mass index  (BMI = W / H^2 (H in m))
  weight = 70 kg
  height = 1.75 m
  assumes: weight, height > 0
```

**What the worked example means:** Body mass index is calculated by dividing weight in kilograms by the square of height in meters. For this case, with a weight of 70 kg and a height of 175 cm, the resulting value is 22.8571. This metric provides a simple estimate of body fat based on height and weight that applies to adult men and women.

<a id="hp_body_surface_area"></a>

#### Body surface area (DuBois, Mosteller, Haycock, Gehan-George), `hp_body_surface_area`

Body surface area by the DuBois (1916), Mosteller (1987), Haycock (1978) and Gehan-George (1970) formulas.

$$
BSA_{Mosteller} = \sqrt{\frac{H\,W}{3600}},\quad BSA_{DuBois} = 0.007184\,W^{0.425}H^{0.725}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `height` | in | height (e.g. in cm) | m |
| `BSA_DuBois` | out | DuBois BSA in m^2 | - |
| `BSA_Mosteller` | out | Mosteller BSA in m^2 | - |
| `BSA_Haycock` | out | Haycock BSA in m^2 | - |
| `BSA_GehanGeorge` | out | Gehan-George BSA in m^2 | - |

**Applies when:** weight, height > 0.

```shell
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

**What the worked example means:** This calculation determines body surface area using four established formulas. Based on a weight of 70 kg and a height of 175 cm, the Mosteller method yields 1.84466. The DuBois formula produces 1.84814, while Haycock results in 1.84678 and Gehan-George gives 1.85385. These values are critical for scaling physiological parameters such as cardiac output or drug dosages to individual body size.

<a id="hp_lean_body_mass_boer"></a>

#### Lean body mass by the Boer formula: `hp_lean_body_mass_boer`

Boer (1984): male 0.407 W + 0.267 H - 19.2, female 0.252 W + 0.473 H - 48.3 (kg), and the remaining fat mass.

$$
LBM_m = 0.407\,W + 0.267\,H - 19.2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `height` | in | height (e.g. in cm) | m |
| `sex` | in | sex for the formula | - |
| `LBM` | out | lean body mass in kg | - |
| `fat_mass` | out | weight - LBM in kg | - |

**Applies when:** 0 < LBM < weight.

```shell
$ human-physiology solve hp_lean_body_mass_boer "weight=70 kg" "height=175 cm" sex=male
LBM = 56.015
fat_mass = 13.985
  by: Lean body mass by the Boer formula  (Boer anthropometric formula)
  weight = 70 kg
  height = 1.75 m
  sex = male
  assumes: 0 < LBM < weight
```

**What the worked example means:** This calculation determines the lean body mass using the Boer anthropometric formula, which takes into account the individual's weight of 70 kg, height of 175 cm, and sex. The resulting lean body mass value is 56.015, while the estimated fat mass, derived by subtracting lean body mass from total weight, is 13.985. These figures provide an estimate of the non-fat components versus the adipose tissue in the human body based on these specific measurements.

### Body fluids (6)

<a id="hp_body_fluid_compartments"></a>

#### Body fluid compartments: `hp_body_fluid_compartments`

Total body water (60 % of weight by default), intracellular 2/3, extracellular 1/3, interstitial 3/4 and plasma 1/4 of the ECF.

$$
TBW = 0.6\,W,\; ICF = \tfrac23 TBW,\; ECF = \tfrac13 TBW
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | body weight (e.g. in kg) | kg |
| `TBW_fraction` | in | total body water fraction of weight (optional; the textbook adult fraction if omitted) (optional) | - |
| `TBW` | out | total body water in L | - |
| `ICF` | out | intracellular fluid in L | - |
| `ECF` | out | extracellular fluid in L | - |
| `ISF` | out | interstitial fluid in L | - |
| `plasma` | out | plasma volume in L | - |

**Applies when:** weight > 0.

```shell
$ human-physiology solve hp_body_fluid_compartments "weight=70 kg"
TBW = 42
ICF = 28
ECF = 14
ISF = 10.5
plasma = 3.5
  by: Body fluid compartments  (60-40-20 rule: ICF 2/3, ECF 1/3 of TBW; plasma 1/4 of ECF)
  weight = 70 kg
  assumes: weight > 0
```

**What the worked example means:** This calculation determines the volume of fluid within different body compartments based on a given body weight of 70 kg. The total body water is estimated as 42, which represents the sum of all fluids in the body. Of this total, two-thirds is located inside cells, known as the intracellular fluid, calculated here as 28. The remaining one-third constitutes the extracellular fluid, measured at 14. This extracellular space is further divided into interstitial fluid surrounding the cells and plasma within the blood vessels.

<a id="hp_gibbs_donnan"></a>

#### Gibbs-Donnan equilibrium with an impermeant anion, `hp_gibbs_donnan`

Equilibrium NaCl distribution across a membrane when one side holds impermeant anions (charge concentration A) facing a large bath of NaCl c: concentrations, Donnan ratio, potential and osmotic excess.

$$
[Na^+]_i[Cl^-]_i = [Na^+]_o[Cl^-]_o,\quad [Na^+]_i = [Cl^-]_i + A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | impermeant anion charge concentration inside in mEq/L | - |
| `c` | in | bath NaCl concentration (e.g. in mmol/L) | mol/m^3 |
| `T` | in | temperature (e.g. in degC) (optional; body temperature if omitted) (optional) | K |
| `Na_in` | out | Na+ inside in mmol/L | - |
| `Cl_in` | out | Cl- inside in mmol/L | - |
| `r` | out | Donnan ratio Na_in/Na_out = Cl_out/Cl_in | - |
| `V_mV` | out | membrane potential inside relative to the bath in mV | - |
| `osmotic_excess` | out | extra diffusible ions inside in mmol/L | - |

**Applies when:** A, c > 0.

```shell
$ human-physiology solve hp_gibbs_donnan A=20 "c=150 mmol/L"
Na_in = 160.3329638
Cl_in = 140.3329638
r = 1.068886425
V_mV = -1.780460072
osmotic_excess = 0.6659275675
  by: Gibbs-Donnan equilibrium with an impermeant anion  (Na_in Cl_in = c^2 with Na_in = Cl_in + A (quadratic); V = -(RT/F) ln(Na_in/c))
  A = 20
  c = 150 mol/m^3
  assumes: A, c > 0
```

**What the worked example means:** This calculation determines the equilibrium distribution of diffusible ions across a membrane containing impermeant anions. Given an internal impermeant anion charge of 20 and an external bath concentration of 150 mmol/L, the model solves for the resulting internal sodium and chloride concentrations. The calculated Donnan ratio is 1.06889, which dictates the separation of charges. Consequently, the membrane potential develops to -1.78046, reflecting the electrical gradient required to balance the chemical gradients established by the trapped anions.

<a id="hp_solution_osmolarity"></a>

#### Osmolarity of a solution: `hp_solution_osmolarity`

Molarity from a mass concentration and the osmolarity (particles per formula unit, times the osmotic coefficient).

$$
\text{osmolarity} = \phi\, i\, \frac{c}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `conc` | in | mass concentration (e.g. in g/L) | kg/m^3 |
| `molar_mass` | in | molar mass (e.g. in g/mol) | kg/mol |
| `i` | in | number of particles per formula unit (dissociation) | - |
| `phi` | in | osmotic coefficient (optional; ideal solution if omitted) (optional) | - |
| `molarity` | out | molarity in mmol/L | - |
| `osmolarity` | out | osmolarity in mOsm/L | - |

**Applies when:** conc, molar mass > 0; 1 <= i <= 6.

```shell
$ human-physiology solve hp_solution_osmolarity "conc=9 g/L" "molar_mass=58.44 g/mol" i=2
molarity = 154.0041068
osmolarity = 308.0082136
  by: Osmolarity of a solution  (mOsm/L = 1000 c/M x i x phi)
  conc = 9 kg/m^3
  molar_mass = 0.05844 kg/mol
  assumes: conc, molar mass > 0; 1 <= i <= 6
```

**What the worked example means:** This calculation determines the osmolarity of a solution based on its mass concentration and molar mass. By converting the mass concentration into molarity and then multiplying by the number of particles per formula unit and the osmotic coefficient, this gives the total particle concentration. The resulting osmolarity of 308.008 indicates the effective concentration of solute particles that contribute to osmotic pressure.

<a id="hp_osmotic_pressure"></a>

#### Osmotic pressure by van 't Hoff's law: `hp_osmotic_pressure`

pi = c R T for an osmolarity c, in Pa, mmHg and atm (about 19.3 mmHg per mOsm/L at 37 degC).

$$
\pi = c R T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `osm` | in | osmolarity (or osmolarity difference) in mOsm/L | - |
| `T` | in | temperature (e.g. in degC) (optional; body temperature if omitted) (optional) | K |
| `pi` | out | osmotic pressure in Pa | - |
| `pi_mmHg` | out | osmotic pressure in mmHg | - |
| `pi_atm` | out | osmotic pressure in atm | - |

**Applies when:** osm >= 0.

```shell
$ human-physiology solve hp_osmotic_pressure osm=300
pi = 773619.1743
pi_mmHg = 5802.62017
pi_atm = 7.635027627
  by: Osmotic pressure by van 't Hoff's law  (pi = c R T (ideal dilute solution))
  osm = 300
  assumes: osm >= 0
```

**What the worked example means:** The osmotic pressure is 773619 Pa, which equals 5802.62 mmHg or 7.63503 atm for an osmolarity of 300.

<a id="hp_watson_total_body_water"></a>

#### Total body water by the Watson equations: `hp_watson_total_body_water`

Watson (1980): male 2.447 - 0.09516 age + 0.1074 H + 0.3362 W; female -2.097 + 0.1069 H + 0.2466 W (litres).

$$
TBW_{m} = 2.447 - 0.09516\,A + 0.1074\,H + 0.3362\,W
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `age` | in | age in years | - |
| `height` | in | height (e.g. in cm) | m |
| `weight` | in | weight (e.g. in kg) | kg |
| `sex` | in | sex for the equation | - |
| `TBW` | out | total body water in L | - |

**Applies when:** adult; height, weight > 0.

```shell
$ human-physiology solve hp_watson_total_body_water age=40 "height=175 cm" "weight=70 kg" sex=male
TBW = 40.9696
  by: Total body water by the Watson equations  (Watson anthropometric regression)
  age = 40
  height = 1.75 m
  weight = 70 kg
  sex = male
  assumes: adult; height, weight > 0
```

**What the worked example means:** This calculation estimates total body water using the Watson anthropometric regression. It takes your age, height, and weight as inputs to produce a result in liters. The value of 40.9696 represents the predicted volume of water contained within your body tissues.

<a id="hp_fluid_shift"></a>

#### Two-compartment fluid shift after adding a solution (Darrow-Yannet), `hp_fluid_shift`

New osmolality and intracellular and extracellular volumes at osmotic equilibrium after a volume of solution enters the ECF.

$$
P_{osm}' = \frac{TBW\,P_{osm} + V C}{TBW + V},\quad ICF' = \frac{ICF\,P_{osm}}{P_{osm}'}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TBW` | in | initial total body water (e.g. in L) | m^3 |
| `Posm` | in | initial osmolality in mOsm/kg | - |
| `V_added` | in | volume added to the ECF (e.g. in L) | m^3 |
| `C_added` | in | osmolarity of the added solution in mOsm/L | - |
| `ICF_fraction` | in | initial intracellular fraction of total body water (optional; the textbook two thirds if omitted) (optional) | - |
| `osm_final` | out | final osmolality in mOsm/kg | - |
| `ICF` | out | final intracellular volume in L | - |
| `ECF` | out | final extracellular volume in L | - |

**Applies when:** TBW, Posm, V_added > 0; C_added >= 0; solute stays in the ECF, water moves freely.

```shell
$ human-physiology solve hp_fluid_shift "TBW=42 L" Posm=290 "V_added=2 L" C_added=0
osm_final = 276.8181818
ICF = 29.33333333
ECF = 14.66666667
  by: Two-compartment fluid shift after adding a solution (Darrow-Yannet)  (solute conservation in each compartment, equal osmolality, water conservation)
  TBW = 0.042 m^3
  Posm = 290
  V_added = 0.002 m^3
  C_added = 0
  assumes: TBW, Posm, V_added > 0; C_added >= 0; solute stays in the ECF, water moves freely
```

**What the worked example means:** This calculation determines the new equilibrium state of body fluids after a solution is added to the extracellular compartment. Starting from an initial total body water of 42 L and osmolality of 290, the addition of 2 L liters of fluid containing solute at a concentration of 0 mOsm per liter changes the system. The model assumes that the added solute remains in the extracellular space while water moves freely across cell membranes until osmotic balance is reached. Consequently, the final osmolality becomes 276.818. This shift causes water to move into or out of cells, resulting in a final intracellular volume of 29.3333 and a final extracellular volume of 14.6667.

### Cardiovascular (26)

<a id="hp_bernoulli_pressure_drop"></a>

#### Bernoulli pressure drop across a narrowing, `hp_bernoulli_pressure_drop`

Convective pressure drop 0.5 rho (v2^2 - v1^2) across a stenosis or valve, and the simplified 4 v^2 (mmHg, v in m/s).

$$
\Delta P = \tfrac{1}{2}\rho\,(v_2^2 - v_1^2) \approx 4 v_2^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | upstream velocity (e.g. in m/s) | m/s |
| `v2` | in | jet velocity (e.g. in m/s) | m/s |
| `rho` | in | blood density (e.g. in kg/m^3) | kg/m^3 |
| `dP` | out | pressure drop in Pa | - |
| `dP_mmHg` | out | pressure drop in mmHg | - |
| `dP_simplified` | out | pressure drop by the simplified Bernoulli equation in mmHg | - |

**Applies when:** v2 > v1 >= 0; inviscid steady flow.

```shell
$ human-physiology solve hp_bernoulli_pressure_drop "v1=1 m/s" "v2=4 m/s" "rho=1060 kg/m^3"
dP = 7950
dP_mmHg = 59.62989528
dP_simplified = 64
  by: Bernoulli pressure drop across a narrowing  (dP = rho (v2^2 - v1^2)/2)
  v1 = 1 m/s
  v2 = 4 m/s
  rho = 1060 kg/m^3
  assumes: v2 > v1 >= 0; inviscid steady flow
```

**What the worked example means:** This calculation determines the convective pressure drop resulting from fluid acceleration through a stenosis or valve. Using the upstream velocity of 1 m/s m/s and the jet velocity of 4 m/s m/s with a blood density of 1060 kg/m^3 kg/m^3, the full Bernoulli equation yields a pressure difference of 7950 Pa or 59.6299 mmHg. The simplified clinical approximation, which assumes negligible upstream velocity, estimates the gradient at 64 mmHg based solely on the jet velocity.

<a id="hp_cardiac_index"></a>

#### Cardiac index and stroke volume index: `hp_cardiac_index`

Cardiac index CI = CO/BSA, stroke volume SV = CO/HR and stroke volume index SVI = CI/HR.

$$
CI = \frac{CO}{BSA},\quad SVI = \frac{CI}{HR}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CO` | in | cardiac output (e.g. in L/min) | m^3/s |
| `BSA` | in | body surface area (e.g. in m^2) | m^2 |
| `HR` | in | heart rate in beats/min (optional; needed for the stroke volume index) (optional) | - |
| `CI` | out | cardiac index in L/min/m^2 | - |
| `SVI` | out | stroke volume index in mL/m^2 | - |
| `SV` | out | stroke volume in mL | - |

**Applies when:** CO, BSA, HR > 0.

```shell
$ human-physiology solve hp_cardiac_index "CO=5.04 L/min" "BSA=1.8 m^2" HR=72
CI = 2.8
SVI = 38.88888889
SV = 70
  by: Cardiac index and stroke volume index  (CI = CO/BSA; SVI = 1000 CI/HR)
  CO = 8.4e-05 m^3/s
  BSA = 1.8 m^2
  HR = 72
  assumes: CO, BSA, HR > 0
```

**What the worked example means:** This calculation determines the cardiac index and stroke volume index to assess heart performance relative to body size. The cardiac index is obtained by dividing the cardiac output, measured at 5.04 L/min, by the body surface area of 1.8 m^2, resulting in 2.8. This metric normalizes blood flow for patient size. Additionally, the stroke volume index is calculated by dividing this cardiac index by the heart rate of 72, yielding 38.8889. These values help clinicians evaluate how efficiently the heart pumps blood per unit of body surface area.

<a id="hp_fick_cardiac_output"></a>

#### Cardiac output by the Fick principle: `hp_fick_cardiac_output`

Cardiac output from oxygen consumption and the arteriovenous oxygen content difference: CO = VO2/(CaO2 - CvO2).

$$
CO = \frac{\dot V_{O_2}}{C_aO_2 - C_{\bar v}O_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VO2` | in | oxygen consumption (e.g. in mL/min) | m^3/s |
| `CaO2` | in | arterial oxygen content in mL O2/dL | - |
| `CvO2` | in | mixed venous oxygen content in mL O2/dL | - |
| `CO` | out | cardiac output in L/min | - |
| `avDO2` | out | arteriovenous O2 content difference in mL/dL | - |

**Applies when:** VO2 > 0; 0 <= CvO2 < CaO2.

```shell
$ human-physiology solve hp_fick_cardiac_output "VO2=250 mL/min" CaO2=20 CvO2=15
CO = 5
avDO2 = 5
  by: Cardiac output by the Fick principle  (CO = VO2 / (10 (CaO2 - CvO2)) (contents per dL -> per L))
  VO2 = 4.166666667e-06 m^3/s
  CaO2 = 20
  CvO2 = 15
  assumes: VO2 > 0; 0 <= CvO2 < CaO2
```

**What the worked example means:** This calculation determines cardiac output using the Fick principle, which relates blood flow to oxygen uptake. By dividing the measured oxygen consumption of 250 mL/min by the difference between arterial and mixed venous oxygen contents, the heart pumps 5 per minute. The resulting arteriovenous oxygen content difference is 5, indicating how much oxygen the tissues extracted from each deciliter of blood.

<a id="hp_cardiac_output"></a>

#### Cardiac output from heart rate and stroke volume, `hp_cardiac_output`

Cardiac output CO = HR x SV, in L/min and mL/min.

$$
CO = HR \times SV
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `HR` | in | heart rate in beats/min | - |
| `SV` | in | stroke volume (e.g. in mL) | m^3 |
| `CO` | out | cardiac output in L/min | - |
| `CO_mL` | out | cardiac output in mL/min | - |

**Applies when:** HR > 0; SV > 0.

```shell
$ human-physiology solve hp_cardiac_output HR=72 "SV=70 mL"
CO = 5.04
CO_mL = 5040
  by: Cardiac output from heart rate and stroke volume  (CO = HR SV / 1000)
  HR = 72
  SV = 7e-05 m^3
  assumes: HR > 0; SV > 0
```

**What the worked example means:** This calculation determines the total volume of blood pumped by the heart each minute. By multiplying the heart rate in beats per minute by the stroke volume, the resulting cardiac output is 5.04 liters per minute. This value represents the efficiency of the heart's pumping action under the given physiological conditions.

<a id="hp_vascular_compliance"></a>

#### Compliance and elastance: `hp_vascular_compliance`

Compliance C = dV/dP and its inverse, the elastance E = dP/dV.

$$
C = \frac{\Delta V}{\Delta P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dV` | in | volume change (e.g. in mL) | m^3 |
| `dP` | in | pressure change (e.g. in mmHg) | Pa |
| `C` | out | compliance in mL/mmHg | - |
| `E` | out | elastance in mmHg/mL | - |

**Applies when:** dV, dP > 0.

```shell
$ human-physiology solve hp_vascular_compliance "dV=70 mL" "dP=40 mmHg"
C = 1.75
E = 0.5714285714
  by: Compliance and elastance  (C = dV/dP)
  dV = 7e-05 m^3
  dP = 5332.895497 Pa
  assumes: dV, dP > 0
```

**What the worked example means:** This calculation determines the compliance and elastance of a physiological structure based on observed changes in volume and pressure. Compliance, which represents the distensibility of the organ, is calculated as the ratio of the volume change 70 mL to the pressure change 40 mmHg, resulting in a value of 1.75. Elastance, representing the stiffness or resistance to expansion, is the inverse of this relationship, yielding a value of 0.571429. These metrics indicate how easily the structure expands under pressure and how much it resists deformation.

<a id="hp_radius_flow_ratio"></a>

#### Effect of a change of vessel radius on flow and resistance, `hp_radius_flow_ratio`

At the same pressure drop, flow scales as r^4 and resistance as 1/r^4 (Poiseuille).

$$
\frac{Q_2}{Q_1} = \left(\frac{r_2}{r_1}\right)^4
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r1` | in | initial radius (any unit) | - |
| `r2` | in | new radius (same unit) | - |
| `flow_ratio` | out | new flow / initial flow | - |
| `resistance_ratio` | out | new resistance / initial resistance | - |

**Applies when:** r1, r2 > 0.

```shell
$ human-physiology solve hp_radius_flow_ratio r1=1 r2=2
flow_ratio = 16
resistance_ratio = 0.0625
  by: Effect of a change of vessel radius on flow and resistance  (Q2/Q1 = (r2/r1)^4)
  r1 = 1
  r2 = 2
  assumes: r1, r2 > 0
```

**What the worked example means:** This calculation determines how blood flow and vascular resistance change when vessel radius shifts from an initial value of 1 to a new value of 2. According to Poiseuille's law, flow scales with the fourth power of the radius, while resistance scales inversely. The resulting flow ratio is 16, indicating the proportional change in volume flow rate, and the resistance ratio is 0.0625, showing the proportional change in opposition to flow.

<a id="hp_fick_saturations"></a>

#### Fick cardiac output from hemoglobin, saturations and PO2, `hp_fick_saturations`

Arterial and mixed venous O2 contents (1.34 Hb S + 0.003 PO2) and the Fick cardiac output VO2/(CaO2 - CvO2).

$$
CO = \frac{\dot V_{O_2}}{1.34\,Hb\,(S_aO_2 - S_{\bar v}O_2) + 0.003\,(P_aO_2 - P_{\bar v}O_2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VO2` | in | oxygen consumption (e.g. in mL/min) | m^3/s |
| `Hb` | in | hemoglobin in g/dL | - |
| `SaO2` | in | arterial saturation (fraction) | - |
| `SvO2` | in | mixed venous saturation (fraction) | - |
| `PaO2` | in | arterial PO2 (e.g. in mmHg) | Pa |
| `PvO2` | in | mixed venous PO2 (e.g. in mmHg) | Pa |
| `CO` | out | cardiac output in L/min | - |
| `CaO2` | out | arterial O2 content in mL/dL | - |
| `CvO2` | out | venous O2 content in mL/dL | - |

**Applies when:** saturations between 0 and 1; CvO2 < CaO2.

```shell
$ human-physiology solve hp_fick_saturations "VO2=250 mL/min" Hb=15 SaO2=0.98 SvO2=0.75 "PaO2=95 mmHg" "PvO2=40 mmHg"
CO = 5.2213868
CaO2 = 19.983
CvO2 = 15.195
  by: Fick cardiac output from hemoglobin, saturations and PO2  (C = 1.34 Hb S + 0.003 PO2; CO = VO2/(10 (CaO2 - CvO2)))
  VO2 = 4.166666667e-06 m^3/s
  Hb = 15
  SaO2 = 0.98
  SvO2 = 0.75
  PaO2 = 12665.6268 Pa
  PvO2 = 5332.895497 Pa
  assumes: saturations between 0 and 1; CvO2 < CaO2
```

**What the worked example means:** This calculation determines cardiac output by dividing total oxygen consumption by the difference between arterial and mixed venous oxygen content. The arterial oxygen content, computed as 19.983, reflects the amount of oxygen carried by hemoglobin and dissolved in plasma at the arterial side. Similarly, the mixed venous oxygen content, 15.195, represents the remaining oxygen after tissue extraction. The resulting cardiac output of 5.22139 indicates the volume of blood the heart pumps per minute to meet the metabolic demand specified by 250 mL/min.

<a id="hp_qt_correction"></a>

#### Heart-rate corrected QT interval (Bazett, Fridericia, Framingham), `hp_qt_correction`

RR = 60/HR and the corrected QT: Bazett QT/sqrt(RR), Fridericia QT/RR^(1/3), Framingham QT + 154 (1 - RR).

$$
QT_c = \frac{QT}{\sqrt{RR}} \;(\text{Bazett}),\quad \frac{QT}{RR^{1/3}} \;(\text{Fridericia})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `QT` | in | measured QT interval (e.g. in ms) | s |
| `HR` | in | heart rate in beats/min | - |
| `RR` | out | RR interval in s | - |
| `QTc_Bazett` | out | Bazett QTc in ms | - |
| `QTc_Fridericia` | out | Fridericia QTc in ms | - |
| `QTc_Framingham` | out | Framingham QTc in ms | - |

**Applies when:** QT, HR > 0.

```shell
$ human-physiology solve hp_qt_correction "QT=400 ms" HR=75
RR = 0.8
QTc_Bazett = 447.2135955
QTc_Fridericia = 430.886938
QTc_Framingham = 430.8
  by: Heart-rate corrected QT interval (Bazett, Fridericia, Framingham)  (RR in seconds; the three published corrections)
  QT = 0.4 s
  HR = 75
  assumes: QT, HR > 0
```

**What the worked example means:** This calculation determines the heart-rate corrected QT interval using three common formulas. First, the RR interval is derived from the heart rate as 0.8. Then, Bazett's correction yields 447.214, Fridericia's yields 430.887, and Framingham's yields 430.8. These values normalize the raw QT measurement to account for variations in heart rate, allowing for more accurate assessment of cardiac repolarization.

<a id="hp_hydrostatic_pressure"></a>

#### Hydrostatic pressure of a blood column: `hp_hydrostatic_pressure`

Pressure difference rho g h between two points a height h apart in a column of blood (posture effects).

$$
\Delta P = \rho g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | height of the column (e.g. in cm) (positive = the lower point) | m |
| `rho` | in | blood density (e.g. in kg/m^3) | kg/m^3 |
| `dP` | out | pressure difference in Pa | - |
| `dP_mmHg` | out | pressure difference in mmHg | - |

**Applies when:** rho > 0.

```shell
$ human-physiology solve hp_hydrostatic_pressure "h=130 cm" "rho=1060 kg/m^3"
dP = 13513.5637
dP_mmHg = 101.3600488
  by: Hydrostatic pressure of a blood column  (dP = rho g h)
  h = 1.3 m
  rho = 1060 kg/m^3
  assumes: rho > 0
```

**What the worked example means:** This calculation determines the hydrostatic pressure difference within a column of blood. Using the density of the blood and the vertical height between two points, the model computes the pressure gradient caused by gravity. The resulting pressure difference is 13513.6 Pascals, equivalent to 101.36 millimeters of mercury. This value represents how much higher the pressure is at the lower point compared to the upper point due to the weight of the fluid column.

<a id="hp_laplace_wall_tension"></a>

#### Laplace wall tension and wall stress of a vessel or chamber, `hp_laplace_wall_tension`

Law of Laplace: wall tension T = P r (cylinder) or P r/2 (sphere) and wall stress T/h.

$$
T = P r \;(\text{cylinder}),\quad T = \tfrac{P r}{2} \;(\text{sphere}),\quad \sigma = T/h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | transmural pressure (e.g. in mmHg) | Pa |
| `r` | in | radius (e.g. in cm) | m |
| `shape` | in | cylinder (vessel) or sphere (chamber, alveolus) | - |
| `h` | in | wall thickness (e.g. in mm) (optional; needed for the wall stress) (optional) | m |
| `T` | out | wall tension in N/m | - |
| `stress` | out | wall stress in kPa | - |

**Applies when:** P, r, h > 0; thin wall.

```shell
$ human-physiology solve hp_laplace_wall_tension "P=100 mmHg" "r=1.5 cm" shape=cylinder "h=2 mm"
T = 199.9835811
stress = 99.99179056
  by: Laplace wall tension and wall stress of a vessel or chamber  (T = P r (cylinder), P r/2 (sphere); stress = T/h)
  P = 13332.23874 Pa
  r = 0.015 m
  h = 0.002 m
  shape = cylinder
  assumes: P, r, h > 0; thin wall
```

**What the worked example means:** This calculation determines the mechanical load on a vessel or chamber wall using the Law of Laplace. It computes wall tension from the transmural pressure, radius, and shape, yielding 199.984. If wall thickness is provided, it also calculates wall stress by dividing tension by thickness, resulting in 99.9918. This indicates how much force per unit area the tissue must withstand to prevent rupture.

<a id="hp_teichholz_volumes"></a>

#### Left-ventricular volumes and ejection fraction by the Teichholz formula, `hp_teichholz_volumes`

Teichholz volume V = 7 D^3/(2.4 + D) from M-mode internal diameters, EF and fractional shortening.

$$
V = \frac{7 D^3}{2.4 + D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `LVIDd` | in | end-diastolic internal diameter (e.g. in cm) | m |
| `LVIDs` | in | end-systolic internal diameter (e.g. in cm) | m |
| `EDV` | out | end-diastolic volume in mL | - |
| `ESV` | out | end-systolic volume in mL | - |
| `EF` | out | ejection fraction | - |
| `FS` | out | fractional shortening | - |

**Applies when:** 0 < LVIDs < LVIDd.

```shell
$ human-physiology solve hp_teichholz_volumes "LVIDd=5 cm" "LVIDs=3.2 cm"
EDV = 118.2432432
ESV = 40.96
EF = 0.6535954286
FS = 0.36
  by: Left-ventricular volumes and ejection fraction by the Teichholz formula  (V = 7 D^3/(2.4 + D) (D in cm, V in mL))
  LVIDd = 0.05 m
  LVIDs = 0.032 m
  assumes: 0 < LVIDs < LVIDd
```

**What the worked example means:** This calculation determines left ventricular volumes using the Teichholz formula, which estimates volume from the internal diameter measured by M-mode echocardiography. The end-diastolic volume of 118.243 represents the amount of blood in the ventricle just before contraction, while the end-systolic volume of 40.96 indicates the residual blood after contraction. The resulting ejection fraction of 0.653595 quantifies the percentage of blood pumped out during each heartbeat, providing a key measure of systolic function.

<a id="hp_mean_arterial_pressure"></a>

#### Mean arterial pressure and pulse pressure: `hp_mean_arterial_pressure`

Pulse pressure PP = SBP - DBP and the estimated mean arterial pressure MAP = DBP + PP/3.

$$
MAP = DBP + \tfrac{1}{3}(SBP - DBP)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SBP` | in | systolic pressure (e.g. in mmHg) | Pa |
| `DBP` | in | diastolic pressure (e.g. in mmHg) | Pa |
| `MAP` | out | mean arterial pressure in mmHg | - |
| `PP` | out | pulse pressure in mmHg | - |

**Applies when:** SBP > DBP > 0.

```shell
$ human-physiology solve hp_mean_arterial_pressure "SBP=120 mmHg" "DBP=80 mmHg"
MAP = 93.33333333
PP = 40
  by: Mean arterial pressure and pulse pressure  (MAP = DBP + (SBP - DBP)/3)
  SBP = 15998.68649 Pa
  DBP = 10665.79099 Pa
  assumes: SBP > DBP > 0
```

**What the worked example means:** This calculation determines two key hemodynamic values from systolic and diastolic blood pressures. The pulse pressure, representing the force generated by each heartbeat, is calculated as the difference between systolic and diastolic pressure, yielding 40. Using this value, the mean arterial pressure, which reflects the average pressure in a patient's arteries during one cardiac cycle, is estimated by adding one-third of the pulse pressure to the diastolic pressure, resulting in 93.3333.

<a id="hp_blood_velocity_area"></a>

#### Mean blood velocity from flow and total cross-sectional area, `hp_blood_velocity_area`

Continuity: mean velocity v = Q/A (slow in the capillaries because their total area is large).

$$
v = \frac{Q}{A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | flow (e.g. in L/min) | m^3/s |
| `A` | in | total cross-sectional area (e.g. in cm^2) | m^2 |
| `v` | out | mean velocity in cm/s | - |

**Applies when:** Q, A > 0.

```shell
$ human-physiology solve hp_blood_velocity_area "Q=5 L/min" "A=4 cm^2"
v = 20.83333333
  by: Mean blood velocity from flow and total cross-sectional area  (v = Q/A (L/min -> cm^3/s))
  Q = 8.333333333e-05 m^3/s
  A = 0.0004 m^2
  assumes: Q, A > 0
```

**What the worked example means:** This calculation determines the average speed at which blood travels through a vascular bed using the principle of continuity. By taking the volumetric flow rate, represented as 5 L/min, and dividing it by the total cross-sectional area available for that flow, denoted as 4 cm^2, this gives the mean velocity 20.8333. This result illustrates how fluid speed changes based on the space available; specifically, a larger total area leads to a slower velocity for a given flow rate, which is why blood moves slowly in capillaries despite their small individual size.

<a id="hp_qrs_axis"></a>

#### Mean QRS (electrical) axis from leads I and aVF, `hp_qrs_axis`

The frontal-plane mean electrical axis atan2(aVF, I) in degrees (hexaxial convention: lead I at 0 deg, aVF at +90 deg).

$$
\theta = \operatorname{atan2}(aVF, I)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lead_I` | in | net QRS deflection in lead I (mm or mV) | - |
| `aVF` | in | net QRS deflection in aVF (same unit) | - |
| `axis` | out | mean QRS axis in degrees (principal value) | - |
| `amplitude` | out | length of the mean vector (same unit) | - |

**Applies when:** not both zero.

```shell
$ human-physiology solve hp_qrs_axis lead_I=0.5 aVF=-0.5
axis = -45
amplitude = 0.7071067812
  by: Mean QRS (electrical) axis from leads I and aVF  (axis = atan2(aVF, I))
  lead_I = 0.5
  aVF = -0.5
  assumes: not both zero
```

**What the worked example means:** This calculation determines the frontal-plane mean electrical axis of the heart using the hexaxial reference system. By taking the net QRS deflection from lead I, which represents horizontal movement, and lead aVF, which represents vertical downward movement, the algorithm computes the angle via the arctangent function. The resulting value of -45 indicates the primary direction of ventricular depolarization in degrees relative to lead I at zero degrees. The magnitude of this vector, shown as 0.707107, reflects the overall strength of the electrical activity projected onto these two axes.

<a id="hp_poiseuille_flow"></a>

#### Poiseuille flow, mean velocity and wall shear stress in a vessel, `hp_poiseuille_flow`

Laminar flow Q = dP pi r^4/(8 eta L) through a vessel, its mean velocity and the wall shear stress 4 eta Q/(pi r^3).

$$
Q = \frac{\pi r^4 \Delta P}{8 \eta L},\quad \tau_w = \frac{4\eta Q}{\pi r^3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dP` | in | pressure drop along the vessel (e.g. in mmHg) | Pa |
| `eta` | in | blood viscosity in cP (mPa s) | - |
| `L` | in | vessel length (e.g. in cm) | m |
| `r` | in | vessel radius (e.g. in mm) | m |
| `Q` | out | flow in mL/s | - |
| `Q_mL_min` | out | flow in mL/min | - |
| `v_mean` | out | mean velocity in cm/s | - |
| `tau_wall` | out | wall shear stress in Pa | - |

**Applies when:** all inputs > 0; steady laminar Newtonian flow.

```shell
$ human-physiology solve hp_poiseuille_flow "dP=10 mmHg" eta=3.5 "L=10 cm" "r=2 mm"
Q = 23.93397902
Q_mL_min = 1436.038741
v_mean = 190.4605534
tau_wall = 13.33223874
  by: Poiseuille flow, mean velocity and wall shear stress in a vessel  (Q = dP pi r^4 /(8 eta L); tau_w = 4 eta Q/(pi r^3))
  dP = 1333.223874 Pa
  eta = 3.5
  L = 0.1 m
  r = 0.002 m
  assumes: all inputs > 0; steady laminar Newtonian flow
```

**What the worked example means:** This calculation determines the volumetric flow rate, mean velocity, and wall shear stress in a blood vessel using Poiseuille's law. Given a pressure drop of 10 mmHg, viscosity of 3.5, length of 10 cm, and radius of 2 mm, the resulting flow is 23.934. The mean velocity derived from this flow is 190.461, while the shear force exerted on the vessel wall is 13.3322. These values describe the hemodynamic state under steady laminar conditions.

<a id="hp_poiseuille_resistance"></a>

#### Poiseuille resistance of a blood vessel: `hp_poiseuille_resistance`

Resistance to laminar flow of a cylindrical vessel R = 8 eta L/(pi r^4), in Pa s/m^3 and in mmHg min/mL.

$$
R = \frac{8\eta L}{\pi r^4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eta` | in | blood viscosity in cP (mPa s) | - |
| `L` | in | vessel length (e.g. in cm) | m |
| `r` | in | vessel radius (e.g. in mm) | m |
| `R` | out | resistance in Pa s/m^3 | - |
| `R_mmHg` | out | resistance in mmHg min/mL | - |

**Applies when:** eta, L, r > 0; steady laminar Newtonian flow in a rigid tube.

```shell
$ human-physiology solve hp_poiseuille_resistance eta=3.5 "L=10 cm" "r=2 mm"
R = 55704230.08
R_mmHg = 0.006963600433
  by: Poiseuille resistance of a blood vessel  (R = 8 eta L/(pi r^4))
  eta = 3.5
  L = 0.1 m
  r = 0.002 m
  assumes: eta, L, r > 0; steady laminar Newtonian flow in a rigid tube
```

**What the worked example means:** This calculation determines the resistance to laminar flow in a cylindrical blood vessel using Poiseuille's law. It takes the blood viscosity 3.5, vessel length 10 cm, and vessel radius 2 mm as inputs. The resulting resistance 5.57042e+07 quantifies how difficult it is for blood to flow through this segment of the vasculature under steady conditions. The value is also expressed in clinical units as 0.0069636 to facilitate comparison with physiological data.

<a id="hp_pulmonary_vascular_resistance"></a>

#### Pulmonary vascular resistance and transpulmonary gradient, `hp_pulmonary_vascular_resistance`

Transpulmonary gradient TPG = mPAP - PCWP and PVR = TPG/CO in Wood units and dyn s cm^-5.

$$
PVR = \frac{\bar P_{PA} - P_{CW}}{CO}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mPAP` | in | mean pulmonary artery pressure (e.g. in mmHg) | Pa |
| `PCWP` | in | pulmonary capillary wedge pressure (e.g. in mmHg) | Pa |
| `CO` | in | cardiac output (e.g. in L/min) | m^3/s |
| `PVR` | out | pulmonary vascular resistance in Wood units | - |
| `PVR_dyn` | out | pulmonary vascular resistance in dyn s cm^-5 | - |
| `TPG` | out | transpulmonary gradient in mmHg | - |

**Applies when:** mPAP > PCWP >= 0; CO > 0.

```shell
$ human-physiology solve hp_pulmonary_vascular_resistance "mPAP=15 mmHg" "PCWP=8 mmHg" "CO=5 L/min"
PVR = 1.4
PVR_dyn = 111.9908054
TPG = 7
  by: Pulmonary vascular resistance and transpulmonary gradient  (PVR = (mPAP - PCWP)/CO)
  mPAP = 1999.835811 Pa
  PCWP = 1066.579099 Pa
  CO = 8.333333333e-05 m^3/s
  assumes: mPAP > PCWP >= 0; CO > 0
```

**What the worked example means:** This calculation determines the resistance to blood flow through the lungs by dividing the pressure difference between the pulmonary artery and the capillary wedge by the cardiac output. The resulting pulmonary vascular resistance value of 1.4 Wood units indicates how much pressure is required to drive each unit of flow. A higher resistance suggests increased pressure in the pulmonary circulation relative to the flow rate, while the transpulmonary gradient of 7 reflects the absolute pressure drop across the vascular bed.

<a id="hp_qp_qs"></a>

#### Pulmonary-to-systemic flow ratio Qp/Qs: `hp_qp_qs`

Qp/Qs = (SaO2 - SvO2)/(SpvO2 - SpaO2) from oxygen saturations (hemoglobin and dissolved O2 cancel or are neglected).

$$
\frac{Q_p}{Q_s} = \frac{S_aO_2 - S_{\bar v}O_2}{S_{pv}O_2 - S_{pa}O_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SaO2` | in | systemic arterial saturation | - |
| `SvO2` | in | mixed venous saturation | - |
| `SpvO2` | in | pulmonary venous saturation | - |
| `SpaO2` | in | pulmonary artery saturation | - |
| `Qp_Qs` | out | pulmonary to systemic flow ratio | - |

**Applies when:** SaO2 > SvO2; SpvO2 > SpaO2.

```shell
$ human-physiology solve hp_qp_qs SaO2=0.95 SvO2=0.65 SpvO2=0.98 SpaO2=0.8
Qp_Qs = 1.666666667
  by: Pulmonary-to-systemic flow ratio Qp/Qs  (Fick principle applied to both circulations)
  SaO2 = 0.95
  SvO2 = 0.65
  SpvO2 = 0.98
  SpaO2 = 0.8
  assumes: SaO2 > SvO2; SpvO2 > SpaO2
```

**What the worked example means:** This result represents the ratio of blood flow through the lungs to blood flow through the body, known as Qp/Qs. It is computed using the Fick principle by comparing oxygen saturations in the systemic and pulmonary circulations. The numerator uses the difference between systemic arterial saturation (0.95) and mixed venous saturation (0.65), while the denominator uses the difference between pulmonary venous saturation (0.98) and pulmonary artery saturation (0.8). A value greater than one indicates increased pulmonary blood flow relative to systemic flow, often seen in left-to-right shunts.

<a id="hp_moens_korteweg"></a>

#### Pulse wave velocity by the Moens-Korteweg equation, `hp_moens_korteweg`

Pulse wave velocity of a thin elastic tube PWV = sqrt(E h/(rho D)).

$$
PWV = \sqrt{\frac{E h}{\rho D}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | wall elastic modulus (e.g. in kPa) | Pa |
| `h` | in | wall thickness (e.g. in mm) | m |
| `D` | in | vessel diameter (e.g. in mm) | m |
| `rho` | in | blood density (e.g. in kg/m^3) | kg/m^3 |
| `PWV` | out | pulse wave velocity in m/s | - |

**Applies when:** all inputs > 0; thin-walled elastic tube.

```shell
$ human-physiology solve hp_moens_korteweg "E=500 kPa" "h=1.5 mm" "D=25 mm" "rho=1060 kg/m^3"
PWV = 5.319951766
  by: Pulse wave velocity by the Moens-Korteweg equation  (PWV = sqrt(E h /(rho D)))
  E = 500000 Pa
  h = 0.0015 m
  D = 0.025 m
  rho = 1060 kg/m^3
  assumes: all inputs > 0; thin-walled elastic tube
```

**What the worked example means:** This calculation determines the speed at which the pressure pulse travels along an artery, known as pulse wave velocity. The result of 5.31995 is derived using the Moens-Korteweg equation, which balances the stiffness of the vessel wall against its geometry and the inertia of the blood. Specifically, it uses a wall elastic modulus of 500 kPa, a wall thickness of 1.5 mm, a vessel diameter of 25 mm, and a blood density of 1060 kg/m^3. A higher value indicates stiffer arteries, while a lower value suggests more compliant vessels.

<a id="hp_vessel_reynolds"></a>

#### Reynolds number of blood flow in a vessel: `hp_vessel_reynolds`

Re = rho v D/eta and the mean velocity at which Re reaches 2000 (the customary laminar limit).

$$
Re = \frac{\rho v D}{\eta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | blood density (e.g. in kg/m^3) | kg/m^3 |
| `v` | in | mean velocity (e.g. in cm/s) | m/s |
| `D` | in | vessel diameter (e.g. in mm) | m |
| `eta` | in | viscosity in cP | - |
| `Re` | out | Reynolds number | - |
| `v_critical` | out | mean velocity at the customary laminar-limit Reynolds number in cm/s | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_vessel_reynolds "rho=1060 kg/m^3" "v=30 cm/s" "D=25 mm" eta=3.5
Re = 2271.428571
v_critical = 26.41509434
  by: Reynolds number of blood flow in a vessel  (Re = rho v D / eta)
  rho = 1060 kg/m^3
  v = 0.3 m/s
  D = 0.025 m
  eta = 3.5
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the Reynolds number using blood density, mean velocity, vessel diameter, and viscosity. The result of 2271.43 indicates the flow regime, while 26.4151 shows the mean velocity at which the flow reaches the customary laminar limit.

<a id="hp_starling_filtration"></a>

#### Starling forces and capillary filtration: `hp_starling_filtration`

Net filtration pressure (Pc - Pi) - sigma (pi_c - pi_i) and the filtration rate Jv = Kf x NFP (glomerular or systemic capillaries).

$$
J_v = K_f\left[(P_c - P_i) - \sigma(\pi_c - \pi_i)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Pc` | in | capillary hydrostatic pressure (e.g. in mmHg) | Pa |
| `Pi` | in | interstitial (or Bowman's space) hydrostatic pressure (e.g. in mmHg) | Pa |
| `pic` | in | capillary oncotic pressure (e.g. in mmHg) | Pa |
| `pii` | in | interstitial oncotic pressure (e.g. in mmHg) | Pa |
| `Kf` | in | filtration coefficient in mL/min per mmHg | - |
| `sigma` | in | reflection coefficient (optional; ideal membrane if omitted) (optional) | - |
| `NFP` | out | net filtration pressure in mmHg (negative = absorption) | - |
| `Jv` | out | filtration rate in mL/min | - |

**Applies when:** Kf > 0; 0 <= sigma <= 1.

```shell
$ human-physiology solve hp_starling_filtration "Pc=35 mmHg" "Pi=0 mmHg" "pic=28 mmHg" "pii=3 mmHg" Kf=0.5
NFP = 10
Jv = 5
  by: Starling forces and capillary filtration  (Jv = Kf [(Pc - Pi) - sigma (pi_c - pi_i)])
  Pc = 4666.28356 Pa
  Pi = 0 Pa
  pic = 3733.026848 Pa
  pii = 399.9671622 Pa
  Kf = 0.5
  assumes: Kf > 0; 0 <= sigma <= 1
```

**What the worked example means:** This calculation determines the movement of fluid across a capillary wall using Starling forces. It computes the net filtration pressure by subtracting interstitial hydrostatic pressure from capillary hydrostatic pressure, then adjusting for the difference in oncotic pressures scaled by the reflection coefficient. The final filtration rate is obtained by multiplying this net pressure by the filtration coefficient. With the given inputs, the net filtration pressure is 10, which drives a filtration rate of 5. A positive result indicates fluid leaving the vessel, while a negative value suggests reabsorption into the bloodstream.

<a id="hp_stroke_volume_ef"></a>

#### Stroke volume and ejection fraction: `hp_stroke_volume_ef`

Stroke volume SV = EDV - ESV and ejection fraction EF = SV/EDV from end-diastolic and end-systolic volumes.

$$
SV = EDV - ESV,\quad EF = \frac{SV}{EDV}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `EDV` | in | end-diastolic volume (e.g. in mL) | m^3 |
| `ESV` | in | end-systolic volume (e.g. in mL) | m^3 |
| `SV` | out | stroke volume in mL | - |
| `EF` | out | ejection fraction (fraction) | - |
| `EF_percent` | out | ejection fraction in % | - |

**Applies when:** 0 <= ESV < EDV.

```shell
$ human-physiology solve hp_stroke_volume_ef "EDV=120 mL" "ESV=50 mL"
SV = 70
EF = 0.5833333333
EF_percent = 58.33333333
  by: Stroke volume and ejection fraction  (SV = EDV - ESV; EF = SV/EDV)
  EDV = 0.00012 m^3
  ESV = 5e-05 m^3
  assumes: 0 <= ESV < EDV
```

**What the worked example means:** The calculation determines stroke volume by subtracting the end-systolic volume from the end-diastolic volume, yielding 70. It then computes the ejection fraction as the ratio of this stroke volume to the initial end-diastolic volume, resulting in 58.3333. This metric indicates the proportion of blood pumped out of the ventricle with each heartbeat relative to the amount present before contraction.

<a id="hp_stroke_work"></a>

#### Stroke work and cardiac power: `hp_stroke_work`

External stroke work SW = MAP x SV (J and g m) and the cardiac power output SW x HR.

$$
SW = \bar P\, SV,\quad \dot W = SW \cdot HR
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SV` | in | stroke volume (e.g. in mL) | m^3 |
| `MAP` | in | mean arterial (ejection) pressure (e.g. in mmHg) | Pa |
| `HR` | in | heart rate in beats/min (optional; needed for the cardiac power) (optional) | - |
| `SW` | out | stroke work in J | - |
| `SW_gm` | out | stroke work in g m | - |
| `power` | out | cardiac power output in W | - |

**Applies when:** SV, MAP, HR > 0.

```shell
$ human-physiology solve hp_stroke_work "SV=70 mL" "MAP=100 mmHg" HR=72
SW = 0.9332567119
SW_gm = 95.1657
power = 1.119908054
  by: Stroke work and cardiac power  (SW = MAP SV (pressure-volume work); power = SW HR/60)
  SV = 7e-05 m^3
  MAP = 13332.23874 Pa
  HR = 72
  assumes: SV, MAP, HR > 0
```

**What the worked example means:** This calculation determines the external mechanical work performed by the heart during a single beat, known as stroke work, and the rate at which this work is done over time, called cardiac power. Stroke work is computed by multiplying the mean arterial pressure by the stroke volume, representing the energy transferred from the ventricle to the blood with each contraction. The result of 0.933257 indicates the energy per beat, while the cardiac power of 1.11991 reflects the sustained metabolic demand of the heart muscle to maintain circulation.

<a id="hp_systemic_vascular_resistance"></a>

#### Systemic vascular resistance (total peripheral resistance), `hp_systemic_vascular_resistance`

SVR = (MAP - CVP)/CO in Wood units (mmHg min/L) and in dyn s cm^-5 (exact factor 79.9934; customary 80).

$$
SVR = \frac{MAP - CVP}{CO}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `MAP` | in | mean arterial pressure (e.g. in mmHg) | Pa |
| `CVP` | in | central venous pressure (e.g. in mmHg) | Pa |
| `CO` | in | cardiac output (e.g. in L/min) | m^3/s |
| `SVR` | out | systemic vascular resistance in mmHg min/L (Wood units) | - |
| `SVR_dyn` | out | systemic vascular resistance in dyn s cm^-5 | - |

**Applies when:** MAP > CVP >= 0; CO > 0.

```shell
$ human-physiology solve hp_systemic_vascular_resistance "MAP=93 mmHg" "CVP=3 mmHg" "CO=5 L/min"
SVR = 18
SVR_dyn = 1439.881784
  by: Systemic vascular resistance (total peripheral resistance)  (SVR = (MAP - CVP)/CO; x 79.9934 for dyn s cm^-5)
  MAP = 12398.98203 Pa
  CVP = 399.9671622 Pa
  CO = 8.333333333e-05 m^3/s
  assumes: MAP > CVP >= 0; CO > 0
```

**What the worked example means:** This calculation determines systemic vascular resistance by taking the difference between mean arterial pressure and central venous pressure and dividing it by cardiac output. The result expressed in Wood units is 18, while the value converted to dyn s cm^-5 is 1439.88. This metric reflects the total resistance that the heart must overcome to pump blood through the systemic circulation.

<a id="hp_vessels_series_parallel"></a>

#### Total resistance of vascular beds in series or in parallel, `hp_vessels_series_parallel`

Series beds add resistances; parallel beds (organ circulations) add conductances.

$$
R_s = \sum R_i,\quad \frac{1}{R_p} = \sum \frac{1}{R_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | resistances of the beds (any one unit, e.g. mmHg min/L) | - |
| `arrangement` | in | series or parallel | - |
| `R_total` | out | total resistance (same unit) | - |
| `G_total` | out | total conductance (reciprocal of the resistance unit) | - |

**Applies when:** every resistance > 0.

```shell
$ human-physiology solve hp_vessels_series_parallel "R=[2, 3, 6]" arrangement=parallel
R_total = 1
G_total = 1
  by: Total resistance of vascular beds in series or in parallel  (series: sum R; parallel: 1/sum(1/R))
  arrangement = parallel
  assumes: every resistance > 0
```

**What the worked example means:** This calculation determines the overall resistance of vascular beds based on their arrangement. If the beds are in series, the individual resistances add up to produce a total resistance of 1. If they are in parallel, the reciprocal of the total resistance equals the sum of the reciprocals of the individual resistances, resulting in a total resistance of 1. The corresponding total conductance, which represents how easily blood flows through the system, is calculated as the reciprocal of this total resistance and is equal to 1. This method allows for the prediction of hemodynamic changes when organs or vessels are connected in different configurations.

<a id="hp_womersley_number"></a>

#### Womersley number of pulsatile blood flow: `hp_womersley_number`

alpha = r sqrt(omega rho/eta) with omega = 2 pi HR/60: the ratio of pulsatile inertia to viscous forces.

$$
\alpha = r\sqrt{\frac{\omega\rho}{\eta}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | vessel radius (e.g. in mm) | m |
| `HR` | in | heart rate in beats/min | - |
| `rho` | in | blood density (e.g. in kg/m^3) | kg/m^3 |
| `eta` | in | viscosity in cP | - |
| `alpha` | out | Womersley number | - |
| `omega` | out | angular frequency in rad/s | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_womersley_number "r=12.5 mm" HR=72 "rho=1060 kg/m^3" eta=3.5
alpha = 18.88902236
omega = 7.539822369
  by: Womersley number of pulsatile blood flow  (alpha = r sqrt(omega rho / eta))
  r = 0.0125 m
  HR = 72
  rho = 1060 kg/m^3
  eta = 3.5
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the Womersley number, which quantifies the relative importance of inertial forces due to oscillatory flow versus viscous forces in blood vessels. The angular frequency omega is derived from the heart rate 72, yielding 7.53982. Using this frequency along with the vessel radius 12.5 mm, blood density 1060 kg/m^3, and viscosity 3.5, the final Womersley number is computed as 18.889. A value near one suggests that both viscous and inertial effects are significant, while values much greater than one indicate that inertial forces dominate, leading to flatter velocity profiles during systole.

### Metabolism (8)

<a id="hp_bmr_harris_benedict"></a>

#### Basal metabolic rate by the Harris-Benedict equation, `hp_bmr_harris_benedict`

Harris-Benedict BMR, original (1919) or revised by Roza and Shizgal (1984), in kcal/day and kJ/day (computed value).

$$
BMR_{m} = 66.473 + 13.7516\,W + 5.0033\,H - 6.755\,A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `height` | in | height (e.g. in cm) | m |
| `age` | in | age in years | - |
| `sex` | in | sex for the coefficients | - |
| `version` | in | original equation or the revised (Roza-Shizgal) equation | - |
| `BMR` | out | basal metabolic rate in kcal/day | - |
| `BMR_kJ` | out | basal metabolic rate in kJ/day | - |

**Applies when:** adult; weight, height > 0.

```shell
$ human-physiology solve hp_bmr_harris_benedict "weight=70 kg" "height=175 cm" age=30 sex=male version=original
BMR = 1702.0125
BMR_kJ = 7121.2203
  by: Basal metabolic rate by the Harris-Benedict equation  (published linear coefficients)
  weight = 70 kg
  height = 1.75 m
  age = 30
  sex = male
  version = original
  assumes: adult; weight, height > 0
```

**What the worked example means:** This calculation determines the basal metabolic rate using the Harris-Benedict equation, which estimates the daily energy expenditure at rest based on body weight, height, age, and sex. The result indicates that the individual requires 1702.01 to maintain basic physiological functions while awake but inactive. This value is also expressed as 7121.22, providing the same energy requirement in kilojoules per day.

<a id="hp_bmr_katch_mcardle"></a>

#### Basal metabolic rate by the Katch-McArdle equation, `hp_bmr_katch_mcardle`

Lean body mass W (1 - body fat) and BMR = 370 + 21.6 LBM in kcal/day (computed value).

$$
BMR = 370 + 21.6\,LBM
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `body_fat` | in | body fat fraction | - |
| `LBM` | out | lean body mass in kg | - |
| `BMR` | out | basal metabolic rate in kcal/day | - |

**Applies when:** weight > 0; 0 <= body fat < 1.

```shell
$ human-physiology solve hp_bmr_katch_mcardle "weight=70 kg" body_fat=0.2
LBM = 56
BMR = 1579.6
  by: Basal metabolic rate by the Katch-McArdle equation  (Katch-McArdle)
  weight = 70 kg
  body_fat = 0.2
  assumes: weight > 0; 0 <= body fat < 1
```

**What the worked example means:** This calculation determines the basal metabolic rate using the Katch-McArdle equation, which relies on lean body mass rather than total weight. First, lean body mass is derived by subtracting the fat mass from the total weight, resulting in 56. This value is then applied to the formula to estimate the energy required for basic physiological functions at rest, yielding a result of 1579.6. This approach provides a more personalized estimate for individuals with varying body compositions compared to equations based solely on total weight.

<a id="hp_met_energy"></a>

#### Energy expenditure of an activity from METs, `hp_met_energy`

1 MET = 3.5 mL O2/kg/min; oxygen uptake MET x 3.5 x W and energy MET x 3.5 x W/200 kcal/min.

$$
\text{kcal/min} = \frac{MET \times 3.5 \times W}{200}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `MET` | in | activity intensity in METs | - |
| `weight` | in | weight (e.g. in kg) | kg |
| `duration` | in | duration (e.g. in min) | s |
| `VO2` | out | oxygen uptake in mL/min | - |
| `kcal_per_min` | out | energy in kcal/min | - |
| `kcal` | out | total energy in kcal | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_met_energy MET=8 "weight=70 kg" "duration=30 min"
VO2 = 1960
kcal_per_min = 9.8
kcal = 294
  by: Energy expenditure of an activity from METs  (kcal/min = MET x 3.5 x W / 200 (about 5 kcal per L O2))
  MET = 8
  weight = 70 kg
  duration = 1800 s
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the energy cost of an activity by combining its intensity in METs with the person's weight. It first finds the oxygen uptake rate, which is 1960, then converts this to an energy expenditure rate of 9.8. Finally, it multiplies that rate by the duration to get the total calories burned, shown as 294.

<a id="hp_elimination_half_life"></a>

#### First-order elimination: rate constant and half-life from clearance and volume, `hp_elimination_half_life`

Physiology of a one-compartment first-order process: k = CL/Vd, half-life ln 2/k and the fraction remaining after t (not a dosing recommendation).

$$
t_{1/2} = \frac{\ln 2\, V_d}{CL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CL` | in | clearance (e.g. in L/h) | m^3/s |
| `Vd` | in | volume of distribution (e.g. in L) | m^3 |
| `t` | in | elapsed time (e.g. in h) (optional) (optional) | s |
| `k` | out | elimination rate constant per hour | - |
| `half_life` | out | half-life in h | - |
| `fraction_remaining` | out | fraction remaining after t | - |

**Applies when:** CL, Vd > 0; t >= 0.

```shell
$ human-physiology solve hp_elimination_half_life "CL=5 L/h" "Vd=50 L" "t=12 h"
k = 0.1
half_life = 6.931471806
fraction_remaining = 0.3011942119
  by: First-order elimination: rate constant and half-life from clearance and volume  (k = CL/Vd; t1/2 = ln 2 / k; C/C0 = exp(-k t))
  CL = 1.388888889e-06 m^3/s
  Vd = 0.05 m^3
  t = 43200 s
  assumes: CL, Vd > 0; t >= 0
```

**What the worked example means:** This calculation determines the kinetics of a one-compartment first-order elimination process using clearance and volume of distribution. The rate constant, shown as 0.1, is derived from the ratio of clearance to volume. Consequently, the time required for the concentration to reduce by half is calculated as 6.93147 hours. If an elapsed time of 12 h is provided, the fraction of the original amount remaining in the body is 0.301194. These values describe how quickly the substance is removed from the system without implying any specific dosing instructions.

<a id="hp_atwater_energy"></a>

#### Food energy by the Atwater factors: `hp_atwater_energy`

Energy from macronutrients with the general Atwater factors 4 (carbohydrate), 9 (fat), 4 (protein) and 7 (alcohol) kcal/g.

$$
E = 4C + 9F + 4P + 7A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `carbohydrate` | in | carbohydrate (e.g. in g) | kg |
| `fat` | in | fat (e.g. in g) | kg |
| `protein` | in | protein (e.g. in g) | kg |
| `alcohol` | in | alcohol (e.g. in g) (optional; none if omitted) (optional) | kg |
| `kcal` | out | energy in kcal | - |
| `kJ` | out | energy in kJ | - |

**Applies when:** amounts >= 0, not all zero.

```shell
$ human-physiology solve hp_atwater_energy "carbohydrate=300 g" "fat=70 g" "protein=80 g"
kcal = 2150
kJ = 8995.6
  by: Food energy by the Atwater factors  (4/9/4/7 kcal per g; 1 kcal = 4.184 kJ)
  carbohydrate = 0.3 kg
  fat = 0.07 kg
  protein = 0.08 kg
  assumes: amounts >= 0, not all zero
```

**What the worked example means:** This calculation determines the metabolizable energy of a food item using the general Atwater factors. It multiplies the mass of carbohydrate, fat, protein, and alcohol by their respective energy densities of 4, 9, 4, and 7 kilocalories per gram. The sum of these contributions yields the total energy in kilocalories, reported here as 2150. This value is then converted to kilojoules using the factor of 4.184, resulting in 8995.6.

<a id="hp_heart_rate_reserve"></a>

#### Maximal heart rate, heart rate reserve and Karvonen target, `hp_heart_rate_reserve`

Estimated maximal heart rate (220 - age; Tanaka 208 - 0.7 age), the reserve HRmax - HRrest and the Karvonen target HRrest + intensity x reserve.

$$
HR_{target} = HR_{rest} + I\,(HR_{max} - HR_{rest})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `age` | in | age in years | - |
| `HR_rest` | in | resting heart rate in beats/min | - |
| `intensity` | in | exercise intensity (fraction of reserve) | - |
| `HRmax` | out | age-predicted maximal heart rate (Fox formula) in beats/min | - |
| `HRmax_Tanaka` | out | age-predicted maximal heart rate (Tanaka formula) in beats/min | - |
| `HRR` | out | heart rate reserve in beats/min | - |
| `target` | out | Karvonen target heart rate in beats/min | - |

**Applies when:** 5 <= age <= 100; HR_rest below HRmax.

```shell
$ human-physiology solve hp_heart_rate_reserve age=40 HR_rest=60 intensity=0.7
HRmax = 180
HRmax_Tanaka = 180
HRR = 120
target = 144
  by: Maximal heart rate, heart rate reserve and Karvonen target  (Karvonen with HRmax = 220 - age)
  age = 40
  HR_rest = 60
  intensity = 0.7
  assumes: 5 <= age <= 100; HR_rest below HRmax
```

**What the worked example means:** This calculation estimates your maximum possible heart rate using the age-based Fox formula, yielding 180 beats per minute. It also provides a more precise Tanaka estimate of 180 beats per minute. The difference between your maximum and resting heart rate defines your heart rate reserve at 120 beats per minute. Applying your chosen exercise intensity to this reserve determines a specific training target of 144 beats per minute, which guides how hard you should work during aerobic activity.

<a id="hp_weir_energy_expenditure"></a>

#### Respiratory quotient and energy expenditure (Weir equation), `hp_weir_energy_expenditure`

RQ = VCO2/VO2 and resting energy expenditure by the abbreviated Weir equation (3.941 VO2 + 1.106 VCO2) x 1440 kcal/day (L/min).

$$
EE = (3.941\,\dot V_{O_2} + 1.106\,\dot V_{CO_2})\times 1440
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VO2` | in | oxygen consumption (e.g. in mL/min) | m^3/s |
| `VCO2` | in | CO2 production (e.g. in mL/min) | m^3/s |
| `RQ` | out | respiratory quotient | - |
| `EE` | out | energy expenditure in kcal/day | - |
| `EE_kJ` | out | energy expenditure in kJ/day | - |

**Applies when:** VO2, VCO2 > 0.

```shell
$ human-physiology solve hp_weir_energy_expenditure "VO2=250 mL/min" "VCO2=200 mL/min"
RQ = 0.8
EE = 1737.288
EE_kJ = 7268.812992
  by: Respiratory quotient and energy expenditure (Weir equation)  (Weir (1949) without urinary nitrogen)
  VO2 = 4.166666667e-06 m^3/s
  VCO2 = 3.333333333e-06 m^3/s
  assumes: VO2, VCO2 > 0
```

**What the worked example means:** This calculation determines the respiratory quotient and resting energy expenditure using the abbreviated Weir equation. By comparing carbon dioxide production to oxygen consumption, this gives the respiratory quotient of 0.8, which indicates the primary fuel source being metabolized. The resulting energy expenditure of 1737.29 represents the total calories burned per day based on these gas exchange measurements.

<a id="hp_bmr_mifflin_st_jeor"></a>

#### Resting energy expenditure by the Mifflin-St Jeor equation, `hp_bmr_mifflin_st_jeor`

Mifflin-St Jeor (1990): 10 W + 6.25 H - 5 A + 5 (male) or - 161 (female), kcal/day (computed value).

$$
REE = 10W + 6.25H - 5A + s
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `weight` | in | weight (e.g. in kg) | kg |
| `height` | in | height (e.g. in cm) | m |
| `age` | in | age in years | - |
| `sex` | in | sex for the constant | - |
| `REE` | out | resting energy expenditure in kcal/day | - |
| `REE_kJ` | out | resting energy expenditure in kJ/day | - |

**Applies when:** adult; weight, height > 0.

```shell
$ human-physiology solve hp_bmr_mifflin_st_jeor "weight=70 kg" "height=175 cm" age=30 sex=male
REE = 1648.75
REE_kJ = 6898.37
  by: Resting energy expenditure by the Mifflin-St Jeor equation  (Mifflin-St Jeor)
  weight = 70 kg
  height = 1.75 m
  age = 30
  sex = male
  assumes: adult; weight, height > 0
```

**What the worked example means:** This calculation estimates the daily energy required for basic physiological functions at rest using the Mifflin-St Jeor equation. It combines the individual's weight, height, age, and sex into a single value expressed as 1648.75. The result represents the metabolic baseline before accounting for physical activity or digestion.

### Nerve and muscle (12)

<a id="hp_membrane_charge"></a>

#### Charge and number of ions to change the membrane potential, `hp_membrane_charge`

Q = Cm A dV and the number of monovalent ions it represents (why few ions cross in an action potential).

$$
Q = C_m A \Delta V,\quad N = \frac{|Q|}{e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cm` | in | specific membrane capacitance (e.g. in uF/cm^2) | F/m^2 |
| `dV` | in | voltage change (e.g. in mV) | V |
| `area` | in | membrane area (e.g. in um^2) | m^2 |
| `Q_pC` | out | charge in pC | - |
| `ions` | out | number of monovalent ions | - |

**Applies when:** Cm, area > 0; dV not 0.

```shell
$ human-physiology solve hp_membrane_charge "Cm=1 uF/cm^2" "dV=100 mV" "area=1000 um^2"
Q_pC = 1
ions = 6241509.074
  by: Charge and number of ions to change the membrane potential  (Q = C A dV; ions = |Q|/e)
  Cm = 0.01 F/m^2
  dV = 0.1 V
  area = 1e-09 m^2
  assumes: Cm, area > 0; dV not 0
```

**What the worked example means:** This calculation determines the total electrical charge needed to shift the membrane potential by a specific voltage change, based on the membrane's capacitance and surface area. Using the input values for specific membrane capacitance, voltage change, and membrane area, the model computes the charge as 1. This charge is then converted into the number of monovalent ions required to produce that shift, resulting in 6.24151e+06. This demonstrates that only a very small number of ions need to cross the membrane to generate significant changes in voltage.

<a id="hp_conduction_velocity_scaling"></a>

#### Conduction velocity scaling with fibre diameter, `hp_conduction_velocity_scaling`

Conduction velocity is proportional to diameter for myelinated fibres and to its square root for unmyelinated fibres.

$$
v \propto d \;(\text{myelinated}),\quad v \propto \sqrt{d}\;(\text{unmyelinated})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | known conduction velocity (e.g. in m/s) | m/s |
| `d1` | in | diameter of that fibre (e.g. in um) | m |
| `d2` | in | diameter of the other fibre (e.g. in um) | m |
| `fiber` | in | fibre type | - |
| `v2` | out | conduction velocity of the other fibre in m/s | - |
| `ratio` | out | ratio of the new to the known velocity | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_conduction_velocity_scaling "v1=60 m/s" "d1=10 um" "d2=5 um" fiber=myelinated
v2 = 30
ratio = 0.5
  by: Conduction velocity scaling with fibre diameter  (v ~ d (myelinated), v ~ sqrt(d) (unmyelinated, cable theory))
  v1 = 60 m/s
  d1 = 1e-05 m
  d2 = 5e-06 m
  fiber = myelinated
  assumes: all inputs > 0
```

**What the worked example means:** This calculation estimates the conduction velocity of a nerve fibre based on its diameter and the properties of myelination. Using a reference fibre with a known velocity of 60 m/s and diameter 10 um, the model applies scaling laws to predict the velocity for a fibre of diameter 5 um. For myelinated fibres, velocity scales linearly with diameter, while for unmyelinated fibres, it scales with the square root of the diameter. The resulting predicted velocity is 30, which represents the speed at which an action potential travels along the new fibre under these assumptions.

<a id="hp_hill_force_velocity"></a>

#### Hill muscle force-velocity relation: `hp_hill_force_velocity`

Hill's equation (F + a)(v + b) = (F0 + a) b: force at shortening velocity v, the power F v and vmax = b F0/a.

$$
(F + a)(v + b) = (F_0 + a)\,b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F0` | in | isometric force (e.g. in N) | N |
| `a` | in | Hill constant a (e.g. in N) | N |
| `b` | in | Hill constant b (e.g. in m/s) | m/s |
| `v` | in | shortening velocity (e.g. in m/s) | m/s |
| `F` | out | force in N | - |
| `power` | out | power in W | - |
| `vmax` | out | maximal shortening velocity in m/s | - |

**Applies when:** F0, a, b > 0; 0 <= v <= vmax.

```shell
$ human-physiology solve hp_hill_force_velocity "F0=100 N" "a=25 N" "b=0.3 m/s" "v=0.1 m/s"
F = 68.75
power = 6.875
vmax = 1.2
  by: Hill muscle force-velocity relation  (F = (F0 b - a v)/(v + b))
  F0 = 100 N
  a = 25 N
  b = 0.3 m/s
  v = 0.1 m/s
  assumes: F0, a, b > 0; 0 <= v <= vmax
```

**What the worked example means:** Hill's equation calculates the force generated by a muscle during shortening based on its isometric capacity and contraction speed. Using an isometric force of 100 N, constant a of 25 N, and constant b of 0.3 m/s, the model determines that at a shortening velocity of 0.1 m/s, the resulting force is 68.75. This interaction yields a mechanical power output of 6.875 watts, while the theoretical maximum shortening velocity for these parameters is 1.2 meters per second.

<a id="hp_ionic_current"></a>

#### Ionic current and driving force: `hp_ionic_current`

Ohmic ionic current I = g (Vm - E) (positive = outward) and the driving force Vm - E.

$$
I = g\,(V_m - E)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | conductance (e.g. in nS) | S |
| `Vm` | in | membrane potential (e.g. in mV) | V |
| `E` | in | equilibrium potential of the ion (e.g. in mV) | V |
| `I` | out | current in pA (positive outward) | - |
| `driving_force` | out | driving force in mV | - |

**Applies when:** g >= 0.

```shell
$ human-physiology solve hp_ionic_current "g=10 nS" "Vm=-70 mV" "E=-95 mV"
I = 250
driving_force = 25
  by: Ionic current and driving force  (I = g (Vm - E); nS x mV = pA)
  g = 1e-08 S
  Vm = -0.07 V
  E = -0.095 V
  assumes: g >= 0
```

In the worked example, g = 10 nS, Vm = -70 mV, E = -95 mV go through the equation above and give I = 250, driving_force = 25. It applies only when g >= 0: outside that it says what is wrong instead of returning a number.

<a id="hp_length_constant"></a>

#### Length constant of an axon and steady-state attenuation, `hp_length_constant`

lambda = sqrt(Rm d/(4 Ri)) of a passive fibre and the steady voltage fraction e^(-x/lambda) at distance x.

$$
\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0 e^{-x/\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rm` | in | specific membrane resistance in ohm cm^2 | - |
| `Ri` | in | axoplasm resistivity in ohm cm | - |
| `d` | in | fibre diameter (e.g. in um) | m |
| `x` | in | distance from the injection site (e.g. in mm) (optional; needed for the attenuation) (optional) | m |
| `lambda` | out | length constant in mm | - |
| `attenuation` | out | steady-state fraction of the voltage remaining at distance x | - |

**Applies when:** Rm, Ri, d > 0; x >= 0; semi-infinite passive cable.

```shell
$ human-physiology solve hp_length_constant Rm=20000 Ri=100 "d=10 um" "x=1 mm"
lambda = 2.236067977
attenuation = 0.6394073192
  by: Length constant of an axon and steady-state attenuation  (lambda = sqrt(rm/ri))
  Rm = 20000
  Ri = 100
  d = 1e-05 m
  x = 0.001 m
  assumes: Rm, Ri, d > 0; x >= 0; semi-infinite passive cable
```

**What the worked example means:** This calculation determines the electrical properties of a passive nerve fiber. Using the specific membrane resistance 20000, axoplasm resistivity 100, and fiber diameter 10 um, it computes the length constant, which represents how far a voltage change spreads before decaying to about thirty-seven percent of its initial value. The result for this fiber is 2.23607. Additionally, it calculates the steady-state attenuation fraction at a specific distance 1 mm from the injection site, yielding 0.639407, which indicates the proportion of the original voltage that remains at that location.

<a id="hp_muscle_max_power"></a>

#### Maximal power of a muscle from Hill's equation, `hp_muscle_max_power`

The shortening velocity of maximal power v_opt = b (sqrt(1 + F0/a) - 1), the force there and the maximal power.

$$
v_{opt} = b\left(\sqrt{1 + F_0/a} - 1\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F0` | in | isometric force (e.g. in N) | N |
| `a` | in | Hill constant a (e.g. in N) | N |
| `b` | in | Hill constant b (e.g. in m/s) | m/s |
| `v_opt` | out | velocity of maximal power in m/s | - |
| `F_opt` | out | force at maximal power in N | - |
| `P_max` | out | maximal power in W | - |
| `vmax` | out | maximal shortening velocity in m/s | - |

**Applies when:** F0, a, b > 0.

```shell
$ human-physiology solve hp_muscle_max_power "F0=100 N" "a=25 N" "b=0.3 m/s"
v_opt = 0.3708203932
F_opt = 30.90169944
P_max = 11.45898034
vmax = 1.2
  by: Maximal power of a muscle from Hill's equation  (dP/dv = 0 for P = v (F0 b - a v)/(v + b))
  F0 = 100 N
  a = 25 N
  b = 0.3 m/s
  assumes: F0, a, b > 0
```

**What the worked example means:** This calculation determines the optimal operating point for a muscle using Hill's equation. Given an isometric force of 100 N and Hill constants a = 25 N and b = 0.3 m/s, the model finds that maximum power output occurs when the muscle shortens at a velocity of 0.37082. At this specific speed, the force generated by the muscle drops to 30.9017, resulting in a peak mechanical power of 11.459. This point represents the most efficient balance between the force a muscle can produce and how fast it can contract.

<a id="hp_chord_conductance"></a>

#### Membrane potential by the chord conductance equation, `hp_chord_conductance`

Steady potential at which the ohmic ionic currents cancel: Vm = (gK EK + gNa ENa + gCl ECl)/(gK + gNa + gCl).

$$
V_m = \frac{g_K E_K + g_{Na}E_{Na} + g_{Cl}E_{Cl}}{g_K + g_{Na} + g_{Cl}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gK` | in | K+ conductance (any unit) | - |
| `gNa` | in | Na+ conductance (same unit) | - |
| `gCl` | in | Cl- conductance (same unit) | - |
| `EK` | in | K+ equilibrium potential (e.g. in mV) | V |
| `ENa` | in | Na+ equilibrium potential (e.g. in mV) | V |
| `ECl` | in | Cl- equilibrium potential (e.g. in mV) | V |
| `Vm` | out | membrane potential in mV | - |

**Applies when:** conductances >= 0, not all zero.

```shell
$ human-physiology solve hp_chord_conductance gK=1 gNa=0.05 gCl=0.3 "EK=-95 mV" "ENa=67 mV" "ECl=-90 mV"
Vm = -87.88888889
  by: Membrane potential by the chord conductance equation  (conductance-weighted mean of the equilibrium potentials)
  gK = 1
  gNa = 0.05
  gCl = 0.3
  EK = -0.095 V
  ENa = 0.067 V
  ECl = -0.09 V
  assumes: conductances >= 0, not all zero
```

**What the worked example means:** This calculation determines the steady-state membrane potential by averaging the equilibrium potentials for potassium, sodium, and chloride, weighted by their respective conductances. The result, -87.8889, represents the voltage at which the net ohmic current across the membrane is zero. It depends on the input values 1, 0.05, and 0.3 for conductances, and -95 mV, 67 mV, and -90 mV for equilibrium potentials. Essentially, the ion with the highest relative conductance exerts the strongest pull on the membrane potential towards its own equilibrium value.

<a id="hp_membrane_time_constant"></a>

#### Membrane time constant and charging: `hp_membrane_time_constant`

tau_m = Rm Cm (specific membrane resistance times capacitance) and the fraction of the final voltage reached after t.

$$
\tau_m = R_m C_m
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rm` | in | specific membrane resistance in ohm cm^2 | - |
| `Cm` | in | specific membrane capacitance (e.g. in uF/cm^2) | F/m^2 |
| `t` | in | time (e.g. in ms) (optional; needed for the fraction charged) (optional) | s |
| `tau` | out | membrane time constant in ms | - |
| `fraction` | out | fraction of the final voltage at time t | - |

**Applies when:** Rm, Cm > 0; t >= 0.

```shell
$ human-physiology solve hp_membrane_time_constant Rm=20000 "Cm=1 uF/cm^2" "t=10 ms"
tau = 20
fraction = 0.3934693403
  by: Membrane time constant and charging  (tau = Rm Cm; V(t)/V_inf = 1 - exp(-t/tau))
  Rm = 20000
  Cm = 0.01 F/m^2
  t = 0.01 s
  assumes: Rm, Cm > 0; t >= 0
```

**What the worked example means:** This calculation determines how quickly a cell membrane charges toward its new potential. By multiplying specific membrane resistance by capacitance, this gives the time constant, which equals 20. Using this value, the fraction of the final voltage achieved at time 10 ms is computed as 0.393469, indicating the speed of the electrical response.

<a id="hp_nernst_potential"></a>

#### Nernst equilibrium potential of an ion: `hp_nernst_potential`

E = (RT/zF) ln(Cout/Cin) in mV for an ion of valence z.

$$
E = \frac{RT}{zF}\ln\frac{[C]_o}{[C]_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | ion valence (charge number, negative for anions) | - |
| `Cout` | in | extracellular concentration in mM | - |
| `Cin` | in | intracellular concentration in mM | - |
| `T` | in | temperature (e.g. in degC) (optional; body temperature if omitted) (optional) | K |
| `E` | out | equilibrium potential in mV | - |
| `RT_F` | out | RT/F in mV | - |

**Applies when:** z a nonzero whole number; concentrations > 0.

```shell
$ human-physiology solve hp_nernst_potential z=1 Cout=4 Cin=140
E = -95.02257566
RT_F = 26.72665911
  by: Nernst equilibrium potential of an ion  (Nernst equation)
  Cout = 4
  Cin = 140
  assumes: z a nonzero whole number; concentrations > 0
```

**What the worked example means:** This calculation determines the electrical potential difference across a membrane that exactly balances the chemical gradient driving ion movement. Using the Nernst equation with a valence of 1, the result is -95.0226 mV. The thermal voltage factor RT over F is computed as 26.7267 mV at the given temperature. This value indicates the membrane potential at which there is no net flow of the specific ion, meaning the tendency to diffuse due to concentration differences is perfectly counteracted by the electrical force.

<a id="hp_nerve_conduction_velocity"></a>

#### Nerve conduction velocity from two stimulation sites, `hp_nerve_conduction_velocity`

NCV = distance between stimulation sites / difference of the response latencies (mm/ms = m/s).

$$
NCV = \frac{d}{t_p - t_d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `distance` | in | distance between stimulation sites (e.g. in mm) | m |
| `latency_proximal` | in | latency from the proximal site (e.g. in ms) | s |
| `latency_distal` | in | latency from the distal site (e.g. in ms) | s |
| `NCV` | out | conduction velocity in m/s | - |

**Applies when:** latency_proximal > latency_distal.

```shell
$ human-physiology solve hp_nerve_conduction_velocity "distance=250 mm" "latency_proximal=8.5 ms" "latency_distal=3.5 ms"
NCV = 50
  by: Nerve conduction velocity from two stimulation sites  (NCV = d / (t_p - t_d))
  distance = 0.25 m
  latency_proximal = 0.0085 s
  latency_distal = 0.0035 s
  assumes: latency_proximal > latency_distal
```

**What the worked example means:** This calculation determines the speed at which an electrical impulse travels along a nerve. It uses the physical distance between two points where the nerve was stimulated and subtracts the latency measured at the distal site from the latency measured at the proximal site. The resulting time difference represents how long it took for the signal to travel that specific distance. Dividing the distance by this time interval yields the nerve conduction velocity, which is reported as 50. This metric helps assess the functional integrity of the nerve pathway being tested.

<a id="hp_ghk_voltage"></a>

#### Resting membrane potential by the Goldman-Hodgkin-Katz equation, `hp_ghk_voltage`

Vm = (RT/F) ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o)) with relative permeabilities.

$$
V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `PK` | in | relative K+ permeability | - |
| `PNa` | in | relative Na+ permeability | - |
| `PCl` | in | relative Cl- permeability | - |
| `K_o` | in | extracellular K+ in mM | - |
| `K_i` | in | intracellular K+ in mM | - |
| `Na_o` | in | extracellular Na+ in mM | - |
| `Na_i` | in | intracellular Na+ in mM | - |
| `Cl_o` | in | extracellular Cl- in mM | - |
| `Cl_i` | in | intracellular Cl- in mM | - |
| `T` | in | temperature (e.g. in degC) (optional; body temperature if omitted) (optional) | K |
| `Vm` | out | membrane potential in mV | - |

**Applies when:** PK > 0; PNa, PCl >= 0; concentrations > 0.

```shell
$ human-physiology solve hp_ghk_voltage PK=1 PNa=0.04 PCl=0.45 K_o=4 K_i=140 Na_o=145 Na_i=12 Cl_o=116 Cl_i=4.2
Vm = -74.89603682
  by: Resting membrane potential by the Goldman-Hodgkin-Katz equation  (GHK voltage equation (chloride terms inverted))
  PK = 1
  PNa = 0.04
  PCl = 0.45
  K_o = 4
  K_i = 140
  Na_o = 145
  Na_i = 12
  Cl_o = 116
  Cl_i = 4.2
  assumes: PK > 0; PNa, PCl >= 0; concentrations > 0
```

**What the worked example means:** This calculation determines the resting membrane potential, denoted as Vm, by applying the Goldman-Hodgkin-Katz voltage equation. It incorporates the relative permeabilities of potassium, sodium, and chloride ions, along with their respective extracellular and intracellular concentrations. The result, -74.896, represents the electrical potential difference across the cell membrane at rest, reflecting the balance of ionic gradients and permeabilities under the given conditions.

<a id="hp_strength_duration"></a>

#### Threshold current for a pulse width (strength-duration curve), `hp_strength_duration`

Threshold current for a pulse of width t from rheobase and chronaxie: Lapicque (exponential RC) and Weiss (linear charge) forms.

$$
I = \frac{I_{rh}}{1 - 2^{-t/c}}\;(\text{Lapicque}),\quad I = I_{rh}\left(1 + \frac{c}{t}\right)\;(\text{Weiss})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rheobase` | in | rheobase current (any unit, e.g. mA) | - |
| `chronaxie` | in | chronaxie (e.g. in ms) | s |
| `t` | in | pulse width (e.g. in ms) | s |
| `I_lapicque` | out | threshold current by the Lapicque (exponential) law, in the unit of the rheobase | - |
| `I_weiss` | out | threshold current by the Weiss (linear) law, in the unit of the rheobase | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_strength_duration rheobase=1 "chronaxie=0.5 ms" "t=0.5 ms"
I_lapicque = 2
I_weiss = 2
  by: Threshold current for a pulse width (strength-duration curve)  (Lapicque: tau = chronaxie/ln 2; Weiss: linear charge law)
  rheobase = 1
  chronaxie = 0.0005 s
  t = 0.0005 s
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the minimum current required to stimulate a nerve or muscle given a specific pulse duration. Using the rheobase of 1 and chronaxie of 0.5 ms, the exponential Lapicque equation yields a threshold current of 2. Alternatively, the linear charge law known as the Weiss equation produces a threshold current of 2. These values represent the electrical intensity needed to reach the activation potential for a stimulus lasting 0.5 ms.

### Renal (12)

<a id="hp_plasma_osmolality"></a>

#### Calculated and effective plasma osmolality and the osmolal gap, `hp_plasma_osmolality`

Calculated osmolality 2 Na + glucose/18 + BUN/2.8, effective osmolality (tonicity) 2 Na + glucose/18 and, with a measured value, the osmolal gap.

$$
P_{osm} = 2[Na^+] + \frac{glucose}{18} + \frac{BUN}{2.8}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Na` | in | sodium (e.g. in mmol/L) | mol/m^3 |
| `glucose` | in | glucose in mg/dL | - |
| `BUN` | in | blood urea nitrogen in mg/dL | - |
| `measured` | in | measured osmolality in mOsm/kg (optional) (optional) | - |
| `calculated` | out | calculated osmolality in mOsm/kg | - |
| `effective` | out | effective osmolality in mOsm/kg | - |
| `osmolal_gap` | out | measured - calculated in mOsm/kg | - |

**Applies when:** Na > 0.

```shell
$ human-physiology solve hp_plasma_osmolality "Na=140 mmol/L" glucose=90 BUN=14
calculated = 290
effective = 285
  by: Calculated and effective plasma osmolality and the osmolal gap  (published divisors 18 (glucose) and 2.8 (urea nitrogen))
  Na = 140 mol/m^3
  glucose = 90
  BUN = 14
  assumes: Na > 0
```

In the worked example, Na = 140 mmol/L, glucose = 90, BUN = 14 go through the equation above and give calculated = 290, effective = 285. It applies only when Na > 0: outside that it says what is wrong instead of returning a number.

<a id="hp_cockcroft_gault"></a>

#### Creatinine clearance by the Cockcroft-Gault equation, `hp_cockcroft_gault`

Estimated creatinine clearance (140 - age) x weight/(72 x Scr), x 0.85 for female (computed value only).

$$
C_{Cr} = \frac{(140 - age)\,W}{72\,S_{cr}}\;[\times 0.85]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `age` | in | age in years | - |
| `weight` | in | body weight (e.g. in kg) | kg |
| `Scr` | in | serum creatinine in mg/dL | - |
| `sex` | in | sex for the equation's coefficient | - |
| `CrCl` | out | estimated creatinine clearance in mL/min | - |

**Applies when:** 18 <= age < 140; weight, Scr > 0.

```shell
$ human-physiology solve hp_cockcroft_gault age=60 "weight=72 kg" Scr=1 sex=male
CrCl = 80
  by: Creatinine clearance by the Cockcroft-Gault equation  (Cockcroft-Gault (1976))
  age = 60
  weight = 72 kg
  Scr = 1
  sex = male
  assumes: 18 <= age < 140; weight, Scr > 0
```

**What the worked example means:** This calculation determines the estimated creatinine clearance using the Cockcroft-Gault equation. It combines the patient's age, body weight, and serum creatinine level to approximate kidney function. The result, 80, represents the volume of plasma cleared of creatinine per minute. For female patients, the initial value is multiplied by zero point eight five to account for lower muscle mass. This metric helps assess renal health and guide drug dosing.

<a id="hp_creatinine_clearance_timed"></a>

#### Creatinine clearance from a timed urine collection, `hp_creatinine_clearance_timed`

Urine flow from a timed collection, creatinine clearance U V/P and its value normalized to 1.73 m^2.

$$
C_{Cr} = \frac{U_{Cr}\dot V}{P_{Cr}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ucr` | in | urine creatinine in mg/dL | - |
| `Vurine` | in | urine volume collected (e.g. in mL) | m^3 |
| `hours` | in | collection time (e.g. in h) | s |
| `Pcr` | in | plasma creatinine in mg/dL | - |
| `BSA` | in | body surface area (e.g. in m^2) (optional; standard adult area if omitted) (optional) | m^2 |
| `urine_flow` | out | urine flow in mL/min | - |
| `CrCl` | out | creatinine clearance in mL/min | - |
| `CrCl_normalized` | out | creatinine clearance in mL/min normalized to standard body surface area | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_creatinine_clearance_timed Ucr=100 "Vurine=1440 mL" "hours=24 h" Pcr=1
urine_flow = 1
CrCl = 100
CrCl_normalized = 100
  by: Creatinine clearance from a timed urine collection  (V = volume/(60 hours); CrCl = U V/P; x 1.73/BSA)
  Ucr = 100
  Vurine = 0.00144 m^3
  hours = 86400 s
  Pcr = 1
  assumes: all inputs > 0
```

**What the worked example means:** Creatinine clearance is 100 and normalized to body surface area it is 100.

<a id="hp_egfr_ckd_epi_2021"></a>

#### Estimated GFR by the CKD-EPI 2021 creatinine equation, `hp_egfr_ckd_epi_2021`

eGFR = 142 min(Scr/k, 1)^a max(Scr/k, 1)^-1.200 0.9938^age (x 1.012 female), the race-free 2021 CKD-EPI creatinine equation (computed value only).

$$
eGFR = 142\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-1.200}\,0.9938^{age}\,[1.012]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Scr` | in | serum creatinine in mg/dL | - |
| `age` | in | age in years | - |
| `sex` | in | sex for the equation's coefficients | - |
| `eGFR` | out | estimated GFR in mL/min normalized to standard body surface area | - |

**Applies when:** Scr > 0; 18 <= age <= 120.

```shell
$ human-physiology solve hp_egfr_ckd_epi_2021 Scr=1 age=50 sex=male
eGFR = 91.69147861
  by: Estimated GFR by the CKD-EPI 2021 creatinine equation  (CKD-EPI 2021: k = 0.7/0.9, a = -0.241/-0.302 (female/male))
  Scr = 1
  age = 50
  sex = male
  assumes: Scr > 0; 18 <= age <= 120
```

**What the worked example means:** This calculation determines the estimated glomerular filtration rate using the race-free CKD-EPI 2021 creatinine equation. It uses the patient's serum creatinine level, age, and sex to compute a value that reflects kidney function normalized to standard body surface area. The resulting estimate of 91.6915 indicates how well the kidneys are filtering waste from the blood.

<a id="hp_egfr_ckd_epi_2021_cys"></a>

#### Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation, `hp_egfr_ckd_epi_2021_cys`

The race-free 2021 CKD-EPI equation from serum creatinine and cystatin C (computed value only).

$$
eGFR = 135\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-0.544}\min(S_{cys}/0.8,1)^{-0.323}\max(S_{cys}/0.8,1)^{-0.778}\,0.9961^{age}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Scr` | in | serum creatinine in mg/dL | - |
| `Scys` | in | serum cystatin C (e.g. in mg/L) | kg/m^3 |
| `age` | in | age in years | - |
| `sex` | in | sex for the equation's coefficients | - |
| `eGFR` | out | estimated GFR in mL/min normalized to standard body surface area | - |

**Applies when:** Scr, Scys > 0; 18 <= age <= 120.

```shell
$ human-physiology solve hp_egfr_ckd_epi_2021_cys Scr=1 "Scys=1 mg/L" age=50 sex=male
eGFR = 88.14399054
  by: Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation  (135 min(Scr/k,1)^a max(Scr/k,1)^-0.544 min(Scys/0.8,1)^-0.323 max(Scys/0.8,1)^-0.778 0.9961^age (x 0.963 female))
  Scr = 1
  Scys = 0.001 kg/m^3
  age = 50
  sex = male
  assumes: Scr, Scys > 0; 18 <= age <= 120
```

**What the worked example means:** This calculation determines the estimated glomerular filtration rate using the 2021 CKD-EPI equation that combines serum creatinine and cystatin C measurements. The result of 88.144 reflects kidney function adjusted for age and sex, derived from an input serum creatinine of 1 and a serum cystatin C level of 1 mg/L. This method provides a race-free estimate of how well the kidneys are filtering waste products from the blood.

<a id="hp_tubular_handling"></a>

#### Filtered load, excretion and net tubular reabsorption or secretion, `hp_tubular_handling`

Filtered load GFR x Px, excretion rate Ux x V and net reabsorption (positive) or secretion (negative) as their difference.

$$
R_x = GFR\cdot P_x - U_x \dot V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `GFR` | in | GFR (e.g. in mL/min) | m^3/s |
| `Px` | in | plasma concentration in mg/dL | - |
| `Ux` | in | urine concentration in mg/dL | - |
| `V` | in | urine flow (e.g. in mL/min) | m^3/s |
| `filtered_load` | out | filtered load in mg/min | - |
| `excreted` | out | excretion rate in mg/min | - |
| `reabsorbed` | out | net reabsorption in mg/min (negative = net secretion) | - |

**Applies when:** GFR, V > 0.

```shell
$ human-physiology solve hp_tubular_handling "GFR=120 mL/min" Px=100 Ux=0 "V=1 mL/min"
filtered_load = 120
excreted = 0
reabsorbed = 120
  by: Filtered load, excretion and net tubular reabsorption or secretion  (filtered - excreted (mg/dL x mL/min / 100 = mg/min))
  GFR = 2e-06 m^3/s
  Px = 100
  Ux = 0
  V = 1.666666667e-08 m^3/s
  assumes: GFR, V > 0
```

**What the worked example means:** This calculation determines how much of a substance is reabsorbed or secreted by the kidney tubules. It first computes the filtered load, which is the product of the glomerular filtration rate and the plasma concentration, resulting in 120. Then it calculates the excretion rate by multiplying the urine concentration by the urine flow rate, yielding 0. The difference between these two values represents the net tubular reabsorption if positive, or net secretion if negative, giving a final value of 120. This method assumes steady-state conditions where input equals output over time.

<a id="hp_filtration_fraction"></a>

#### Filtration fraction: `hp_filtration_fraction`

The fraction of renal plasma flow filtered at the glomerulus, FF = GFR/RPF.

$$
FF = \frac{GFR}{RPF}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `GFR` | in | glomerular filtration rate (e.g. in mL/min) | m^3/s |
| `RPF` | in | renal plasma flow (e.g. in mL/min) | m^3/s |
| `FF` | out | filtration fraction | - |
| `FF_percent` | out | filtration fraction in % | - |

**Applies when:** 0 < GFR < RPF.

```shell
$ human-physiology solve hp_filtration_fraction "GFR=120 mL/min" "RPF=600 mL/min"
FF = 0.2
FF_percent = 20
  by: Filtration fraction  (FF = GFR / RPF)
  GFR = 2e-06 m^3/s
  RPF = 1e-05 m^3/s
  assumes: 0 < GFR < RPF
```

**What the worked example means:** This calculation determines the filtration fraction by dividing the glomerular filtration rate, 120 mL/min, by the renal plasma flow, 600 mL/min. The resulting value, 0.2, represents the proportion of blood plasma entering the kidneys that actually passes through the glomerular capillaries into the Bowman's space. In percentage terms, this is expressed as 20, indicating how much of the available plasma volume contributes to urine formation at any given moment.

<a id="hp_fractional_excretion"></a>

#### Fractional excretion of a solute: `hp_fractional_excretion`

FE = (Ux Pcr)/(Px Ucr): the fraction of the filtered load excreted, with creatinine clearance as GFR (e.g. FENa).

$$
FE_x = \frac{U_x P_{Cr}}{P_x U_{Cr}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ux` | in | urine concentration of the solute (any unit) | - |
| `Px` | in | plasma concentration of the solute (same unit) | - |
| `Ucr` | in | urine creatinine (any unit) | - |
| `Pcr` | in | plasma creatinine (same unit) | - |
| `FE` | out | fractional excretion | - |
| `FE_percent` | out | fractional excretion in % | - |

**Applies when:** Px, Ucr, Pcr > 0.

```shell
$ human-physiology solve hp_fractional_excretion Ux=40 Px=140 Ucr=100 Pcr=1
FE = 0.002857142857
FE_percent = 0.2857142857
  by: Fractional excretion of a solute  (FE = Ux Pcr / (Px Ucr))
  Ux = 40
  Px = 140
  Ucr = 100
  Pcr = 1
  assumes: Px, Ucr, Pcr > 0
```

**What the worked example means:** This calculation determines the fraction of a specific solute that is excreted in the urine relative to the amount filtered by the kidneys. It uses the formula FE = (Ux Pcr) / (Px Ucr), where Ux and Px are the urine and plasma concentrations of the solute, and Pcr and Ucr are the plasma and urine creatinine levels. Creatinine serves as a marker for glomerular filtration rate. The result, 0.00285714, indicates that 0.285714 of the filtered solute was actually excreted rather than reabsorbed by the renal tubules.

<a id="hp_glucose_titration"></a>

#### Glucose titration curve (ideal transport maximum), `hp_glucose_titration`

Filtered, reabsorbed (at most Tm) and excreted glucose and the plasma threshold Tm/GFR, ignoring splay.

$$
E = \max(0,\; GFR\cdot P_G - T_m)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `GFR` | in | GFR (e.g. in mL/min) | m^3/s |
| `Pglucose` | in | plasma glucose in mg/dL | - |
| `Tm` | in | transport maximum (e.g. in mg/min) | kg/s |
| `filtered_load` | out | filtered load in mg/min | - |
| `reabsorbed` | out | reabsorbed in mg/min | - |
| `excreted` | out | excreted in mg/min | - |
| `threshold` | out | plasma threshold in mg/dL | - |

**Applies when:** GFR, Tm > 0; Pglucose >= 0.

```shell
$ human-physiology solve hp_glucose_titration "GFR=125 mL/min" Pglucose=400 "Tm=375 mg/min"
filtered_load = 500
reabsorbed = 375
excreted = 125
threshold = 300
  by: Glucose titration curve (ideal transport maximum)  (reabsorbed = min(filtered, Tm))
  GFR = 2.083333333e-06 m^3/s
  Pglucose = 400
  Tm = 6.25e-06 kg/s
  assumes: GFR, Tm > 0; Pglucose >= 0
```

**What the worked example means:** This calculation determines how much glucose passes through the kidney filter versus how much is reclaimed by transport proteins. Using the glomerular filtration rate of 125 mL/min and the plasma glucose concentration of 400, this gives the total filtered load as 500. The actual amount reabsorbed is limited by the transport maximum, resulting in 375 being recovered from the filtrate. Consequently, any excess beyond this capacity appears in the urine as an excretion rate of 125. The plasma concentration required to begin exceeding this reabsorptive capacity is known as the renal threshold, calculated here as 300. This model assumes ideal saturation kinetics without considering the splay observed in real biological systems.

<a id="hp_free_water_clearance"></a>

#### Osmolar and free water clearance: `hp_free_water_clearance`

Osmolar clearance Cosm = Uosm V/Posm and free water clearance CH2O = V - Cosm (negative when the urine is concentrated).

$$
C_{H_2O} = \dot V - \frac{U_{osm}\dot V}{P_{osm}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | urine flow (e.g. in mL/min) | m^3/s |
| `Uosm` | in | urine osmolality in mOsm/kg | - |
| `Posm` | in | plasma osmolality in mOsm/kg | - |
| `Cosm` | out | osmolar clearance in mL/min | - |
| `CH2O` | out | free water clearance in mL/min | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_free_water_clearance "V=2 mL/min" Uosm=150 Posm=300
Cosm = 1
CH2O = 1
  by: Osmolar and free water clearance  (CH2O = V (1 - Uosm/Posm))
  V = 3.333333333e-08 m^3/s
  Uosm = 150
  Posm = 300
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the osmolar clearance and free water clearance based on urine flow, urine osmolality, and plasma osmolality. The osmolar clearance represents the volume of plasma cleared of solutes per unit time, calculated as 1. The free water clearance indicates the net amount of pure water added to or removed from the body fluid compartments, shown here as 1. A negative value for free water clearance signifies that the kidneys are conserving water by producing concentrated urine, while a positive value indicates the excretion of dilute urine.

<a id="hp_renal_clearance"></a>

#### Renal clearance of a substance (inulin GFR), `hp_renal_clearance`

Clearance C = U V/P (equal to GFR for inulin) and the urinary excretion rate U V.

$$
C_x = \frac{U_x \dot V}{P_x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | urine concentration in mg/dL | - |
| `V` | in | urine flow (e.g. in mL/min) | m^3/s |
| `P` | in | plasma concentration in mg/dL | - |
| `C` | out | clearance in mL/min | - |
| `excretion` | out | excretion rate in mg/min | - |

**Applies when:** U >= 0; V, P > 0.

```shell
$ human-physiology solve hp_renal_clearance U=30 "V=1 mL/min" P=0.25
C = 120
excretion = 0.3
  by: Renal clearance of a substance (inulin GFR)  (C = U V / P)
  U = 30
  V = 1.666666667e-08 m^3/s
  P = 0.25
  assumes: U >= 0; V, P > 0
```

**What the worked example means:** This calculation determines the renal clearance of a substance, specifically inulin, which serves as a gold standard for estimating glomerular filtration rate. The clearance value, 120, is derived from the urine concentration 30, the urine flow rate 1 mL/min, and the plasma concentration 0.25 using the formula C equals U times V divided by P. Additionally, the urinary excretion rate, 0.3, represents the total amount of substance eliminated per unit time, calculated as the product of urine concentration and flow rate.

<a id="hp_pah_renal_flow"></a>

#### Renal plasma and blood flow from PAH clearance, `hp_pah_renal_flow`

Effective renal plasma flow = PAH clearance, true RPF = eRPF/E (PAH extraction) and RBF = RPF/(1 - Hct).

$$
RBF = \frac{U_{PAH}\dot V / P_{PAH}}{E_{PAH}(1 - Hct)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U_PAH` | in | urine PAH concentration in mg/dL | - |
| `V` | in | urine flow (e.g. in mL/min) | m^3/s |
| `P_PAH` | in | plasma PAH concentration in mg/dL | - |
| `Hct` | in | hematocrit (fraction) | - |
| `E` | in | PAH extraction ratio (optional; complete extraction, the effective flow, if omitted) (optional) | - |
| `eRPF` | out | effective renal plasma flow in mL/min | - |
| `RPF` | out | renal plasma flow in mL/min | - |
| `RBF` | out | renal blood flow in mL/min | - |

**Applies when:** 0 <= Hct < 1; 0 < E <= 1.

```shell
$ human-physiology solve hp_pah_renal_flow U_PAH=600 "V=1 mL/min" P_PAH=1 Hct=0.45 E=0.9
eRPF = 600
RPF = 666.6666667
RBF = 1212.121212
  by: Renal plasma and blood flow from PAH clearance  (eRPF = U V/P; RPF = eRPF/E; RBF = RPF/(1 - Hct))
  U_PAH = 600
  V = 1.666666667e-08 m^3/s
  P_PAH = 1
  Hct = 0.45
  E = 0.9
  assumes: 0 <= Hct < 1; 0 < E <= 1
```

**What the worked example means:** This calculation determines renal hemodynamics using para-aminohippuric acid clearance. The effective renal plasma flow, which approximates the volume of plasma cleared of PAH per minute, is calculated as 600. Because PAH is not fully extracted by the kidneys, this value is divided by the extraction ratio to find the true renal plasma flow of 666.667. Finally, accounting for the fraction of blood occupied by red cells via hematocrit, the total renal blood flow is derived as 1212.12.

### Respiratory (27)

<a id="hp_alveolar_gas_equation"></a>

#### Alveolar gas equation: `hp_alveolar_gas_equation`

Inspired PO2 = FiO2 (PB - PH2O) and alveolar PO2 by the simplified (PiO2 - PaCO2/R) and full (nitrogen-balance) alveolar gas equation.

$$
P_AO_2 = F_iO_2(P_B - P_{H_2O}) - \frac{P_aCO_2}{R}\left[1 - F_iO_2(1 - R)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FiO2` | in | inspired O2 fraction | - |
| `Patm` | in | barometric pressure (e.g. in mmHg) | Pa |
| `PH2O` | in | water vapour pressure (e.g. in mmHg) | Pa |
| `PaCO2` | in | arterial PCO2 (e.g. in mmHg) | Pa |
| `RQ` | in | respiratory quotient | - |
| `PiO2` | out | inspired PO2 in mmHg | - |
| `PAO2` | out | alveolar PO2, simplified equation, in mmHg | - |
| `PAO2_full` | out | alveolar PO2, full equation, in mmHg | - |

**Applies when:** 0 < FiO2 <= 1; Patm > PH2O; 0 < RQ <= 1.5.

```shell
$ human-physiology solve hp_alveolar_gas_equation FiO2=0.21 "Patm=760 mmHg" "PH2O=47 mmHg" "PaCO2=40 mmHg" RQ=0.8
PiO2 = 149.73
PAO2 = 99.73
PAO2_full = 101.83
  by: Alveolar gas equation  (PAO2 = PiO2 - PaCO2/R (+ PaCO2 FiO2 (1 - R)/R))
  FiO2 = 0.21
  Patm = 101325.0144 Pa
  PH2O = 6266.152209 Pa
  PaCO2 = 5332.895497 Pa
  RQ = 0.8
  assumes: 0 < FiO2 <= 1; Patm > PH2O; 0 < RQ <= 1.5
```

**What the worked example means:** This calculation determines the partial pressure of oxygen in the alveoli using the alveolar gas equation. It starts by computing the inspired oxygen partial pressure, which is 149.73, based on the fraction of inspired oxygen at 0.21 and the barometric pressure of 760 mmHg minus the water vapor pressure of 47 mmHg. The simplified alveolar oxygen partial pressure, 99.73, is then derived by subtracting the arterial carbon dioxide partial pressure of 40 mmHg divided by the respiratory quotient of 0.8 from the inspired value. This result estimates the oxygen available for diffusion into the blood.

<a id="hp_alveolar_ventilation_co2"></a>

#### Alveolar ventilation from CO2 output and arterial PCO2, `hp_alveolar_ventilation_co2`

Alveolar ventilation equation VA (BTPS) = VCO2 (STPD) x 863/PaCO2, with K = 760 x 310.15/273.15 mmHg.

$$
\dot V_A = \frac{\dot V_{CO_2} \times 0.863}{P_aCO_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VCO2` | in | CO2 output (e.g. in mL/min) (STPD) | m^3/s |
| `PaCO2` | in | arterial (alveolar) PCO2 (e.g. in mmHg) | Pa |
| `VA` | out | alveolar ventilation in L/min (BTPS) | - |

**Applies when:** VCO2, PaCO2 > 0.

```shell
$ human-physiology solve hp_alveolar_ventilation_co2 "VCO2=200 mL/min" "PaCO2=40 mmHg"
VA = 4.314735493
  by: Alveolar ventilation from CO2 output and arterial PCO2  (VA = VCO2 K / PaCO2, K = 760 x 310.15/273.15)
  VCO2 = 3.333333333e-06 m^3/s
  PaCO2 = 5332.895497 Pa
  assumes: VCO2, PaCO2 > 0
```

**What the worked example means:** This calculation determines alveolar ventilation, the volume of fresh air reaching the alveoli per minute, using the alveolar ventilation equation. It relates the rate of carbon dioxide production to the partial pressure of carbon dioxide in arterial blood. Given a carbon dioxide output of 200 mL/min and an arterial partial pressure of carbon dioxide of 40 mmHg, the resulting alveolar ventilation is 4.31474. This value reflects how effectively the lungs are clearing carbon dioxide; higher ventilation typically lowers arterial carbon dioxide pressure for a given metabolic rate.

<a id="hp_aa_gradient"></a>

#### Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio, `hp_aa_gradient`

PAO2 from the simplified alveolar gas equation, the A-a gradient PAO2 - PaO2, the a/A ratio and PaO2/FiO2.

$$
A\!-\!a = P_AO_2 - P_aO_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FiO2` | in | inspired O2 fraction | - |
| `Patm` | in | barometric pressure (e.g. in mmHg) | Pa |
| `PH2O` | in | water vapour pressure (e.g. in mmHg) | Pa |
| `PaCO2` | in | arterial PCO2 (e.g. in mmHg) | Pa |
| `PaO2` | in | arterial PO2 (e.g. in mmHg) | Pa |
| `RQ` | in | respiratory quotient | - |
| `PAO2` | out | alveolar PO2 in mmHg | - |
| `Aa_gradient` | out | A-a gradient in mmHg | - |
| `a_A_ratio` | out | PaO2/PAO2 | - |
| `PF_ratio` | out | PaO2/FiO2 in mmHg | - |

**Applies when:** 0 < FiO2 <= 1; computed PAO2 > 0.

```shell
$ human-physiology solve hp_aa_gradient FiO2=0.21 "Patm=760 mmHg" "PH2O=47 mmHg" "PaCO2=40 mmHg" "PaO2=90 mmHg" RQ=0.8
PAO2 = 99.73
Aa_gradient = 9.73
a_A_ratio = 0.9024365788
PF_ratio = 428.5714286
  by: Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio  (A-a = FiO2 (PB - PH2O) - PaCO2/R - PaO2)
  FiO2 = 0.21
  Patm = 101325.0144 Pa
  PH2O = 6266.152209 Pa
  PaCO2 = 5332.895497 Pa
  PaO2 = 11999.01487 Pa
  RQ = 0.8
  assumes: 0 < FiO2 <= 1; computed PAO2 > 0
```

In the worked example, FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, PaO2 = 90 mmHg, RQ = 0.8 go through the equation above and give PAO2 = 99.73, Aa_gradient = 9.73, a_A_ratio = 0.9024365788, PF_ratio = 428.5714286. It applies only when 0 < FiO2 <= 1; computed PAO2 > 0: outside that it says what is wrong instead of returning a number.

<a id="hp_oxygen_content"></a>

#### Arterial oxygen content: `hp_oxygen_content`

CaO2 = 1.34 x Hb x SaO2 + 0.003 x PaO2 (hemoglobin-bound plus dissolved oxygen).

$$
C_aO_2 = 1.34\,Hb\,S_aO_2 + 0.003\,P_aO_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Hb` | in | hemoglobin in g/dL | - |
| `SaO2` | in | arterial saturation | - |
| `PaO2` | in | arterial PO2 (e.g. in mmHg) | Pa |
| `CaO2` | out | oxygen content in mL O2/dL | - |
| `bound` | out | hemoglobin-bound O2 in mL/dL | - |
| `dissolved` | out | dissolved O2 in mL/dL | - |

**Applies when:** Hb > 0; 0 <= SaO2 <= 1.

```shell
$ human-physiology solve hp_oxygen_content Hb=15 SaO2=0.98 "PaO2=100 mmHg"
CaO2 = 19.998
bound = 19.698
dissolved = 0.3
  by: Arterial oxygen content  (1.34 Hb S + 0.003 PO2)
  Hb = 15
  SaO2 = 0.98
  PaO2 = 13332.23874 Pa
  assumes: Hb > 0; 0 <= SaO2 <= 1
```

**What the worked example means:** This calculation determines the total amount of oxygen carried in arterial blood by summing two components. The first component, representing oxygen bound to hemoglobin, equals 19.698. The second component, representing oxygen dissolved directly in plasma, equals 0.3. Together they yield a total arterial oxygen content of 19.998, which reflects the body's immediate capacity for oxygen delivery based on the input values of hemoglobin concentration, saturation, and partial pressure.

<a id="hp_gas_volume_conversion"></a>

#### ATPS to BTPS and STPD gas volume conversion, `hp_gas_volume_conversion`

A spirometer volume measured at ambient temperature and pressure, saturated (ATPS), converted to body (BTPS) and standard dry (STPD) conditions.

$$
V_{BTPS} = V\,\frac{310.15}{T}\,\frac{P_B - P_{H_2O}}{P_B - 47}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | measured volume (e.g. in L) | m^3 |
| `T` | in | spirometer temperature (e.g. in degC) | K |
| `Patm` | in | barometric pressure (e.g. in mmHg) | Pa |
| `PH2O` | in | water vapour pressure at T (e.g. in mmHg) | Pa |
| `V_BTPS` | out | volume at BTPS in L | - |
| `V_STPD` | out | volume at STPD in L | - |

**Applies when:** Patm > PH2O; Patm > 47 mmHg.

```shell
$ human-physiology solve hp_gas_volume_conversion "V=1 L" "T=22 degC" "Patm=760 mmHg" "PH2O=19.8 mmHg"
V_BTPS = 1.090909061
V_STPD = 0.9013509188
  by: ATPS to BTPS and STPD gas volume conversion  (ideal gas law on the dry-gas partial pressures)
  V = 0.001 m^3
  T = 295.15 K
  Patm = 101325.0144 Pa
  PH2O = 2639.783271 Pa
  assumes: Patm > PH2O; Patm > 47 mmHg
```

**What the worked example means:** This calculation converts a spirometer reading taken at ambient conditions into the equivalent volume under standard physiological states. Using the ideal gas law, the input volume 1 L, measured at temperature 22 degC and barometric pressure 760 mmHg, is adjusted for water vapor pressure 19.8 mmHg. The result provides the volume corrected to Body Temperature and Pressure Saturated (BTPS) as 1.09091, and to Standard Temperature and Pressure Dry (STPD) as 0.901351. These values allow direct comparison with reference ranges defined at body or standard conditions.

<a id="hp_barometric_altitude"></a>

#### Barometric pressure and inspired PO2 at altitude, `hp_barometric_altitude`

Standard-atmosphere barometric pressure P = 101325 (1 - 0.0065 h/288.15)^(g M/(R L)) and the humidified inspired PO2.

$$
P = P_0\left(1 - \frac{L h}{T_0}\right)^{gM/(RL)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | altitude (e.g. in m) | m |
| `FiO2` | in | inspired oxygen fraction (optional; dry air if omitted) (optional) | - |
| `Patm` | out | barometric pressure in mmHg | - |
| `PiO2` | out | humidified inspired PO2 in mmHg | - |
| `Patm_kPa` | out | barometric pressure in kPa | - |

**Applies when:** 0 <= h <= 11000 m.

```shell
$ human-physiology solve hp_barometric_altitude "h=3000 m"
Patm = 525.8605769
PiO2 = 100.3212909
Patm_kPa = 70.10898756
  by: Barometric pressure and inspired PO2 at altitude  (ICAO standard troposphere (lapse 6.5 K/km))
  h = 3000 m
  assumes: 0 <= h <= 11000 m
```

**What the worked example means:** At an altitude of 3000 m, the barometric pressure is 525.861 and the humidified inspired oxygen partial pressure is 100.321.

<a id="hp_bicarbonate_from_ph"></a>

#### Bicarbonate from pH and PCO2 (Henderson-Hasselbalch), `hp_bicarbonate_from_ph`

HCO3 = 0.03 PaCO2 x 10^(pH - 6.1).

$$
[HCO_3^-] = 0.03\,P_{CO_2}\,10^{pH - 6.1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pH` | in | pH | - |
| `PaCO2` | in | PCO2 (e.g. in mmHg) | Pa |
| `HCO3` | out | bicarbonate in mmol/L | - |

**Applies when:** 6.5 <= pH <= 8.0; PaCO2 > 0.

```shell
$ human-physiology solve hp_bicarbonate_from_ph pH=7.4 "PaCO2=40 mmHg"
HCO3 = 23.94314778
  by: Bicarbonate from pH and PCO2 (Henderson-Hasselbalch)  (Henderson-Hasselbalch solved for bicarbonate)
  pH = 7.4
  PaCO2 = 5332.895497 Pa
  assumes: 6.5 <= pH <= 8.0; PaCO2 > 0
```

**What the worked example means:** This calculation determines the bicarbonate level using the Henderson-Hasselbalch equation. It takes the measured arterial carbon dioxide partial pressure, denoted as 40 mmHg, and the blood pH value, shown as 7.4, to derive the result. The final computed value for bicarbonate is 23.9431. This metric helps assess the metabolic component of acid-base balance by showing how much base is present relative to the respiratory acid load.

<a id="hp_ph_henderson_hasselbalch"></a>

#### Blood pH by the Henderson-Hasselbalch equation, `hp_ph_henderson_hasselbalch`

pH = 6.1 + log10(HCO3/(0.03 PaCO2)) and the hydrogen-ion concentration in nmol/L.

$$
pH = 6.1 + \log_{10}\frac{[HCO_3^-]}{0.03\,P_{CO_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `HCO3` | in | bicarbonate (e.g. in mmol/L) | mol/m^3 |
| `PaCO2` | in | PCO2 (e.g. in mmHg) | Pa |
| `pH` | out | pH | - |
| `H` | out | hydrogen-ion concentration in nmol/L | - |

**Applies when:** HCO3, PaCO2 > 0.

```shell
$ human-physiology solve hp_ph_henderson_hasselbalch "HCO3=24 mmol/L" "PaCO2=40 mmHg"
pH = 7.401029996
H = 39.71641174
  by: Blood pH by the Henderson-Hasselbalch equation  (pK' 6.1, CO2 solubility 0.03 mmol/L per mmHg)
  HCO3 = 24 mol/m^3
  PaCO2 = 5332.895497 Pa
  assumes: HCO3, PaCO2 > 0
```

**What the worked example means:** This calculation determines the blood pH using the Henderson-Hasselbalch equation, which relates pH to the ratio of bicarbonate to dissolved carbon dioxide. Given a bicarbonate level of 24 mmol/L and a partial pressure of carbon dioxide of 40 mmHg, the resulting pH is 7.40103. This value corresponds to a hydrogen-ion concentration of 39.7164, providing a direct measure of the acidity or alkalinity of the blood.

<a id="hp_fev1_fvc_ratio"></a>

#### FEV1/FVC ratio: `hp_fev1_fvc_ratio`

The fraction of the forced vital capacity exhaled in the first second.

$$
\frac{FEV_1}{FVC}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FEV1` | in | forced expiratory volume in the first second (e.g. in L) | m^3 |
| `FVC` | in | forced vital capacity (e.g. in L) | m^3 |
| `ratio` | out | FEV1/FVC | - |
| `percent` | out | FEV1/FVC in % | - |

**Applies when:** 0 < FEV1 <= FVC.

```shell
$ human-physiology solve hp_fev1_fvc_ratio "FEV1=3.2 L" "FVC=4 L"
ratio = 0.8
percent = 80
  by: FEV1/FVC ratio  (FEV1 / FVC)
  FEV1 = 0.0032 m^3
  FVC = 0.004 m^3
  assumes: 0 < FEV1 <= FVC
```

**What the worked example means:** This calculation determines the fraction of the forced vital capacity that can be exhaled in the first second. It divides the forced expiratory volume in one second, measured as 3.2 L, by the total forced vital capacity, measured as 4 L. The resulting value is 0.8, expressed as a percentage of 80. This metric helps assess airflow limitation by comparing how much air is expelled quickly against the total amount that can be forcefully exhaled.

<a id="hp_fick_diffusion"></a>

#### Fick's law of diffusion across a membrane: `hp_fick_diffusion`

Steady diffusive flux J = D A dC/T across a barrier of thickness T, and the diffusion time T^2/(2D).

$$
J = D A \frac{\Delta C}{T},\quad t = \frac{T^2}{2D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient (e.g. in cm^2/s) | m^2/s |
| `A` | in | membrane area (e.g. in cm^2) | m^2 |
| `dC` | in | concentration difference (e.g. in mmol/L) | mol/m^3 |
| `T` | in | membrane thickness (e.g. in um) | m |
| `J` | out | diffusion rate in mmol/s | - |
| `t_diffusion` | out | characteristic diffusion time across the membrane in s | - |

**Applies when:** D, A, T > 0.

```shell
$ human-physiology solve hp_fick_diffusion "D=1.5e-05 cm^2/s" "A=2 cm^2" "dC=0.5 mmol/L" "T=2 um"
J = 7.5e-05
t_diffusion = 0.001333333333
  by: Fick's law of diffusion across a membrane  (J = D A dC / T (units converted to cm))
  D = 1.5e-09 m^2/s
  A = 0.0002 m^2
  dC = 0.5 mol/m^3
  T = 2e-06 m
  assumes: D, A, T > 0
```

**What the worked example means:** This calculation determines the steady diffusive flux across a membrane using Fick's law. The result, 7.5e-05, represents the amount of substance moving per unit time, derived from the diffusion coefficient 1.5e-05 cm^2/s, area 2 cm^2, concentration difference 0.5 mmol/L, and thickness 2 um. Additionally, the characteristic diffusion time 0.00133333 estimates how long it takes for particles to traverse the barrier based on its thickness and diffusivity.

<a id="hp_hill_fit"></a>

#### Hill coefficient and P50 from two points of the dissociation curve, `hp_hill_fit`

The Hill coefficient n and P50 of the curve through two measured (PO2, saturation) points (Hill plot slope).

$$
n = \frac{\ln\frac{S_2/(1-S_2)}{S_1/(1-S_1)}}{\ln(P_2/P_1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | first PO2 (e.g. in mmHg) | Pa |
| `S1` | in | saturation at P1 | - |
| `P2` | in | second PO2 (e.g. in mmHg) | Pa |
| `S2` | in | saturation at P2 | - |
| `n` | out | Hill coefficient | - |
| `P50` | out | P50 in mmHg | - |

**Applies when:** 0 < S < 1; saturation increases with PO2.

```shell
$ human-physiology solve hp_hill_fit "P1=20 mmHg" S1=0.3121240186 "P2=60 mmHg" S2=0.8980776441
n = 2.7
P50 = 26.8
  by: Hill coefficient and P50 from two points of the dissociation curve  (slope of log(S/(1-S)) against log PO2)
  P1 = 2666.447748 Pa
  S1 = 0.3121240186
  P2 = 7999.343245 Pa
  S2 = 0.8980776441
  assumes: 0 < S < 1; saturation increases with PO2
```

**What the worked example means:** This calculation determines the cooperativity of oxygen binding to hemoglobin by finding the slope of the Hill plot. Using the first point with pressure 20 mmHg and saturation 0.312124, along with the second point at pressure 60 mmHg and saturation 0.898078, this gives the Hill coefficient 2.7. This coefficient indicates how steeply saturation changes with pressure, while the resulting P50 value of 26.8 represents the partial pressure required for half-maximal saturation under these specific conditions.

<a id="hp_alveolar_laplace_pressure"></a>

#### Laplace pressure of an alveolus: `hp_alveolar_laplace_pressure`

Pressure needed to keep a spherical alveolus open, P = 2 gamma/r (why surfactant matters).

$$
P = \frac{2\gamma}{r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gamma` | in | surface tension (e.g. in mN/m) | N/m |
| `r` | in | alveolar radius (e.g. in um) | m |
| `P` | out | collapsing pressure in Pa | - |
| `P_cmH2O` | out | collapsing pressure in cmH2O | - |

**Applies when:** gamma, r > 0.

```shell
$ human-physiology solve hp_alveolar_laplace_pressure "gamma=25 mN/m" "r=100 um"
P = 500
P_cmH2O = 5.098581065
  by: Laplace pressure of an alveolus  (P = 2 gamma / r)
  gamma = 0.025 N/m
  r = 0.0001 m
  assumes: gamma, r > 0
```

**What the worked example means:** This calculation determines the pressure required to keep a spherical alveolus open using Laplace's law. It takes the surface tension 25 mN/m and the radius 100 um to compute the resulting pressure, which equals 500 in pascals or 5.09858 in centimeters of water.

<a id="hp_lung_capacities"></a>

#### Lung capacities from the four lung volumes, `hp_lung_capacities`

Inspiratory capacity, functional residual capacity, vital capacity and total lung capacity from TV, IRV, ERV and RV.

$$
TLC = IRV + TV + ERV + RV
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TV` | in | tidal volume (e.g. in mL) | m^3 |
| `IRV` | in | inspiratory reserve volume (e.g. in mL) | m^3 |
| `ERV` | in | expiratory reserve volume (e.g. in mL) | m^3 |
| `RV` | in | residual volume (e.g. in mL) | m^3 |
| `IC` | out | inspiratory capacity in mL | - |
| `FRC` | out | functional residual capacity in mL | - |
| `VC` | out | vital capacity in mL | - |
| `TLC` | out | total lung capacity in mL | - |

**Applies when:** TV, RV > 0; IRV, ERV >= 0.

```shell
$ human-physiology solve hp_lung_capacities "TV=500 mL" "IRV=3000 mL" "ERV=1100 mL" "RV=1200 mL"
IC = 3500
FRC = 2300
VC = 4600
TLC = 5800
  by: Lung capacities from the four lung volumes  (IC = TV + IRV; FRC = ERV + RV; VC = IRV + TV + ERV; TLC = VC + RV)
  TV = 0.0005 m^3
  IRV = 0.003 m^3
  ERV = 0.0011 m^3
  RV = 0.0012 m^3
  assumes: TV, RV > 0; IRV, ERV >= 0
```

**What the worked example means:** This calculation determines key lung capacities by combining four fundamental volumes. The inspiratory capacity 3500 represents the maximum air inhaled after a normal exhalation, calculated as tidal volume 500 mL plus inspiratory reserve volume 3000 mL. The functional residual capacity 2300 indicates the air remaining in the lungs after a normal exhalation, derived from expiratory reserve volume 1100 mL and residual volume 1200 mL. Vital capacity 4600 measures the largest amount of air that can be exhaled after a maximal inhalation, summing inspiratory reserve volume 3000 mL, tidal volume 500 mL, and expiratory reserve volume 1100 mL. Finally, total lung capacity 5800 reflects the volume of air in the lungs after a maximal inspiration, equal to vital capacity 4600 plus residual volume 1200 mL.

<a id="hp_minute_ventilation"></a>

#### Minute and alveolar ventilation: `hp_minute_ventilation`

Minute ventilation VE = VT x RR, alveolar ventilation VA = (VT - VD) x RR and the dead-space fraction VD/VT.

$$
\dot V_A = (V_T - V_D) f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VT` | in | tidal volume (e.g. in mL) | m^3 |
| `RR` | in | respiratory rate in breaths/min | - |
| `VD` | in | dead space (e.g. in mL) | m^3 |
| `VE` | out | minute ventilation in L/min | - |
| `VA` | out | alveolar ventilation in L/min | - |
| `VD_VT` | out | dead space fraction | - |

**Applies when:** 0 <= VD < VT; RR > 0.

```shell
$ human-physiology solve hp_minute_ventilation "VT=500 mL" RR=12 "VD=150 mL"
VE = 6
VA = 4.2
VD_VT = 0.3
  by: Minute and alveolar ventilation  (VE = VT f; VA = (VT - VD) f)
  VT = 0.0005 m^3
  RR = 12
  VD = 0.00015 m^3
  assumes: 0 <= VD < VT; RR > 0
```

**What the worked example means:** This calculation determines both total minute ventilation and effective alveolar ventilation based on the provided breathing parameters. Minute ventilation represents the total volume of air moved into or out of the lungs per minute, calculated as the product of tidal volume and respiratory rate, resulting in 6. Alveolar ventilation accounts for the portion of that air that actually reaches the gas exchange surfaces by subtracting the dead space volume from the tidal volume before multiplying by the respiratory rate, yielding 4.2. The dead space fraction, expressed as 0.3, indicates what proportion of each breath does not participate in gas exchange.

<a id="hp_oxygen_consumption"></a>

#### Oxygen consumption and extraction ratio (reverse Fick), `hp_oxygen_consumption`

VO2 = CO (CaO2 - CvO2) x 10 and the oxygen extraction ratio (CaO2 - CvO2)/CaO2.

$$
\dot V_{O_2} = CO\,(C_aO_2 - C_{\bar v}O_2)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CO` | in | cardiac output (e.g. in L/min) | m^3/s |
| `Hb` | in | hemoglobin in g/dL | - |
| `SaO2` | in | arterial saturation | - |
| `SvO2` | in | mixed venous saturation | - |
| `PaO2` | in | arterial PO2 (e.g. in mmHg) | Pa |
| `PvO2` | in | mixed venous PO2 (e.g. in mmHg) | Pa |
| `VO2` | out | oxygen consumption in mL/min | - |
| `O2ER` | out | oxygen extraction ratio | - |
| `CaO2` | out | arterial content in mL/dL | - |
| `CvO2` | out | venous content in mL/dL | - |

**Applies when:** CvO2 < CaO2.

```shell
$ human-physiology solve hp_oxygen_consumption "CO=5 L/min" Hb=15 SaO2=0.98 SvO2=0.75 "PaO2=100 mmHg" "PvO2=40 mmHg"
VO2 = 240.15
O2ER = 0.2401740174
CaO2 = 19.998
CvO2 = 15.195
  by: Oxygen consumption and extraction ratio (reverse Fick)  (VO2 = 10 CO (CaO2 - CvO2))
  CO = 8.333333333e-05 m^3/s
  Hb = 15
  SaO2 = 0.98
  SvO2 = 0.75
  PaO2 = 13332.23874 Pa
  PvO2 = 5332.895497 Pa
  assumes: CvO2 < CaO2
```

**What the worked example means:** This calculation determines the body's total oxygen uptake using the reverse Fick principle, which states that consumption equals cardiac output multiplied by the difference between arterial and venous oxygen content. Based on a cardiac output of 5 L/min, an arterial hemoglobin saturation of 0.98, and a mixed venous saturation of 0.75, the arterial oxygen content was found to be 19.998 and the venous content 15.195. Consequently, the resulting oxygen consumption is 240.15, with an extraction ratio of 0.240174, indicating the proportion of delivered oxygen that tissues actually utilize.

<a id="hp_hill_saturation"></a>

#### Oxyhemoglobin saturation by the Hill equation, `hp_hill_saturation`

SO2 = PO2^n/(PO2^n + P50^n) for a Hill coefficient n and half-saturation pressure P50.

$$
S = \frac{P^n}{P^n + P_{50}^n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `PO2` | in | partial pressure of O2 (e.g. in mmHg) | Pa |
| `P50` | in | P50 (e.g. in mmHg) | Pa |
| `n` | in | Hill coefficient | - |
| `SO2` | out | fractional saturation | - |

**Applies when:** PO2 >= 0; P50, n > 0.

```shell
$ human-physiology solve hp_hill_saturation "PO2=40 mmHg" "P50=26.8 mmHg" n=2.7
SO2 = 0.7467379184
  by: Oxyhemoglobin saturation by the Hill equation  (Hill equation)
  PO2 = 5332.895497 Pa
  P50 = 3573.039983 Pa
  n = 2.7
  assumes: PO2 >= 0; P50, n > 0
```

In the worked example, PO2 = 40 mmHg, P50 = 26.8 mmHg, n = 2.7 go through the equation above and give SO2 = 0.7467379184. It applies only when PO2 >= 0; P50, n > 0: outside that it says what is wrong instead of returning a number.

<a id="hp_pco2_from_ph"></a>

#### PCO2 from pH and bicarbonate (Henderson-Hasselbalch), `hp_pco2_from_ph`

PaCO2 = HCO3/(0.03 x 10^(pH - 6.1)).

$$
P_{CO_2} = \frac{[HCO_3^-]}{0.03\cdot 10^{pH - 6.1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pH` | in | pH | - |
| `HCO3` | in | bicarbonate (e.g. in mmol/L) | mol/m^3 |
| `PaCO2` | out | PCO2 in mmHg | - |

**Applies when:** 6.5 <= pH <= 8.0; HCO3 > 0.

```shell
$ human-physiology solve hp_pco2_from_ph pH=7.4 "HCO3=24 mmol/L"
PaCO2 = 40.09497869
  by: PCO2 from pH and bicarbonate (Henderson-Hasselbalch)  (Henderson-Hasselbalch solved for PCO2)
  pH = 7.4
  HCO3 = 24 mol/m^3
  assumes: 6.5 <= pH <= 8.0; HCO3 > 0
```

**What the worked example means:** This calculation determines the partial pressure of carbon dioxide in arterial blood using the Henderson-Hasselbalch equation. By inputting the measured bicarbonate concentration of 24 mmol/L and the blood pH of 7.4, the model solves for the respiratory component of acid-base balance. The resulting value, 40.095, indicates how much carbon dioxide is dissolved in the blood, which directly influences the acidity of the plasma.

<a id="hp_bohr_dead_space"></a>

#### Physiological dead space by the Bohr (Enghoff) equation, `hp_bohr_dead_space`

VD/VT = (PaCO2 - PECO2)/PaCO2 and the dead-space volume, with arterial PCO2 standing for alveolar (Enghoff).

$$
\frac{V_D}{V_T} = \frac{P_aCO_2 - P_ECO_2}{P_aCO_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `PaCO2` | in | arterial PCO2 (e.g. in mmHg) | Pa |
| `PECO2` | in | mixed expired PCO2 (e.g. in mmHg) | Pa |
| `VT` | in | tidal volume (e.g. in mL) | m^3 |
| `VD_VT` | out | dead space fraction | - |
| `VD` | out | dead space volume in mL | - |

**Applies when:** 0 <= PECO2 < PaCO2.

```shell
$ human-physiology solve hp_bohr_dead_space "PaCO2=40 mmHg" "PECO2=28 mmHg" "VT=500 mL"
VD_VT = 0.3
VD = 150
  by: Physiological dead space by the Bohr (Enghoff) equation  (Bohr CO2 mass balance)
  PaCO2 = 5332.895497 Pa
  PECO2 = 3733.026848 Pa
  VT = 0.0005 m^3
  assumes: 0 <= PECO2 < PaCO2
```

**What the worked example means:** This calculation determines the physiological dead space using the Bohr equation with the Enghoff modification. It takes the arterial partial pressure of carbon dioxide, denoted as PaCO2, which is 40 mmHg, and the mixed expired partial pressure of carbon dioxide, PECO2, which is 28 mmHg. The tidal volume, VT, is given as 500 mL. The ratio of dead space to tidal volume, VD/VT, is calculated as 0.3, representing the proportion of each breath that does not participate in gas exchange. Multiplying this fraction by the tidal volume yields the absolute dead space volume, VD, which is 150. This metric helps assess ventilation efficiency and potential lung pathology.

<a id="hp_shunt_fraction"></a>

#### Physiological shunt fraction (shunt equation), `hp_shunt_fraction`

Qs/Qt = (Cc'O2 - CaO2)/(Cc'O2 - CvO2) from end-capillary, arterial and mixed venous O2 contents.

$$
\frac{Q_s}{Q_t} = \frac{C_{c'}O_2 - C_aO_2}{C_{c'}O_2 - C_{\bar v}O_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CcO2` | in | end-capillary O2 content in mL/dL | - |
| `CaO2` | in | arterial O2 content in mL/dL | - |
| `CvO2` | in | mixed venous O2 content in mL/dL | - |
| `Qs_Qt` | out | shunt fraction | - |

**Applies when:** CcO2 >= CaO2 > CvO2.

```shell
$ human-physiology solve hp_shunt_fraction CcO2=21 CaO2=20 CvO2=15
Qs_Qt = 0.1666666667
  by: Physiological shunt fraction (shunt equation)  (mixing balance of shunted and capillary blood)
  CcO2 = 21
  CaO2 = 20
  CvO2 = 15
  assumes: CcO2 >= CaO2 > CvO2
```

**What the worked example means:** This calculation determines the physiological shunt fraction by comparing oxygen content differences. It uses the formula where the numerator is the difference between end-capillary and arterial oxygen content, and the denominator is the difference between end-capillary and mixed venous oxygen content. The resulting value of 0.166667 represents the proportion of cardiac output that bypasses gas exchange in the pulmonary capillaries. A higher value indicates a greater amount of blood moving through the lungs without picking up oxygen, which typically results from conditions like atelectasis or pneumonia.

<a id="hp_hill_po2"></a>

#### PO2 at a given saturation by the Hill equation, `hp_hill_po2`

Inverse Hill equation PO2 = P50 (S/(1 - S))^(1/n).

$$
P = P_{50}\left(\frac{S}{1-S}\right)^{1/n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SO2` | in | fractional saturation | - |
| `P50` | in | P50 (e.g. in mmHg) | Pa |
| `n` | in | Hill coefficient | - |
| `PO2` | out | partial pressure of O2 in mmHg | - |

**Applies when:** 0 < SO2 < 1; P50, n > 0.

```shell
$ human-physiology solve hp_hill_po2 SO2=0.75 "P50=26.8 mmHg" n=2.7
PO2 = 40.25745998
  by: PO2 at a given saturation by the Hill equation  (PO2 = P50 (S/(1-S))^(1/n))
  SO2 = 0.75
  P50 = 3573.039983 Pa
  n = 2.7
  assumes: 0 < SO2 < 1; P50, n > 0
```

**What the worked example means:** This calculation determines the partial pressure of oxygen required to achieve a specific level of hemoglobin saturation. The inverse Hill equation relates the fractional saturation 0.75 to the half-saturation pressure 26.8 mmHg and the cooperativity factor 2.7. The resulting value 40.2575 represents the oxygen tension in the blood that corresponds to the measured saturation, assuming standard physiological conditions where the system follows sigmoidal binding kinetics.

<a id="hp_severinghaus_po2"></a>

#### PO2 at a given saturation by the Severinghaus equation, `hp_severinghaus_po2`

Inverse of the Severinghaus curve: the real root of P^3 + 150 P = 23400 S/(1 - S).

$$
P^3 + 150P = \frac{23400\,S}{1-S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SO2` | in | fractional saturation | - |
| `PO2` | out | partial pressure of O2 in mmHg | - |

**Applies when:** 0 < SO2 < 1.

```shell
$ human-physiology solve hp_severinghaus_po2 SO2=0.7494646681
PO2 = 40
  by: PO2 at a given saturation by the Severinghaus equation  (Cardano root of the cubic, Newton polished)
  SO2 = 0.7494646681
  assumes: 0 < SO2 < 1
```

**What the worked example means:** This calculation determines the partial pressure of oxygen in millimeters of mercury that corresponds to a specific fractional hemoglobin saturation. By applying the inverse Severinghaus equation, the system solves for the real root of the cubic relationship between pressure and saturation. For an input saturation value of 0.749465, the resulting partial pressure is 40, representing the tension required to achieve that level of oxygen binding.

<a id="hp_respiratory_time_constant"></a>

#### Respiratory time constant and passive exhalation, `hp_respiratory_time_constant`

tau = R C, the fraction of the volume exhaled after time t, 1 - e^(-t/tau), and 3 tau (about 95 %).

$$
\tau = R C,\quad V(t) = V_0 e^{-t/\tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | airway resistance in cmH2O s/L | - |
| `C` | in | compliance in mL/cmH2O | - |
| `t` | in | time (e.g. in s) (optional; needed for the fraction exhaled) (optional) | s |
| `tau` | out | time constant in s | - |
| `fraction_exhaled` | out | fraction exhaled at time t | - |
| `t_95` | out | three time constants in s | - |

**Applies when:** R, C > 0; t >= 0.

```shell
$ human-physiology solve hp_respiratory_time_constant R=5 C=50 "t=0.5 s"
tau = 0.25
t_95 = 0.75
fraction_exhaled = 0.8646647168
  by: Respiratory time constant and passive exhalation  (single-compartment passive emptying)
  R = 5
  C = 50
  t = 0.5 s
  assumes: R, C > 0; t >= 0
```

**What the worked example means:** This calculation determines the respiratory time constant by multiplying airway resistance of 5 cmH2O s/L by compliance of 50 mL/cmH2O, yielding 0.25 seconds. Using this value, the model estimates that 0.864665 of the lung volume has been exhaled after 0.5 s seconds. The time required to exhale approximately 95 percent of the volume, defined as three time constants, is calculated as 0.75 seconds.

<a id="hp_severinghaus_saturation"></a>

#### Saturation by the Severinghaus (1979) dissociation equation, `hp_severinghaus_saturation`

Standard human oxyhemoglobin curve S = 1/(23400/(PO2^3 + 150 PO2) + 1) (37 degC, pH 7.40, PCO2 40 mmHg).

$$
S = \left(\frac{23400}{P^3 + 150P} + 1\right)^{-1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `PO2` | in | partial pressure of O2 (e.g. in mmHg) | Pa |
| `SO2` | out | fractional saturation | - |

**Applies when:** PO2 > 0.

```shell
$ human-physiology solve hp_severinghaus_saturation "PO2=40 mmHg"
SO2 = 0.7494646681
  by: Saturation by the Severinghaus (1979) dissociation equation  (Severinghaus equation)
  PO2 = 5332.895497 Pa
  assumes: PO2 > 0
```

**What the worked example means:** This calculation determines the fractional saturation of hemoglobin with oxygen, denoted as SO2. It uses the Severinghaus equation, which models the standard oxyhemoglobin dissociation curve at body temperature and normal pH levels. The result depends on the partial pressure of oxygen in the blood, represented by PO2. With an input partial pressure of 40 mmHg, the model predicts a saturation level of 0.749465. This value indicates how much oxygen is bound to hemoglobin under these specific physiological conditions.

<a id="hp_anion_gap"></a>

#### Serum anion gap and albumin-corrected anion gap, `hp_anion_gap`

AG = Na - (Cl + HCO3) and the albumin-corrected value AG + 2.5 (4.0 - albumin in g/dL).

$$
AG = [Na^+] - ([Cl^-] + [HCO_3^-])
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Na` | in | sodium (e.g. in mmol/L) | mol/m^3 |
| `Cl` | in | chloride (e.g. in mmol/L) | mol/m^3 |
| `HCO3` | in | bicarbonate (e.g. in mmol/L) | mol/m^3 |
| `albumin` | in | albumin in g/dL (optional; normal albumin if omitted) (optional) | - |
| `AG` | out | anion gap in mmol/L | - |
| `AG_corrected` | out | albumin-corrected anion gap in mmol/L | - |

**Applies when:** all inputs > 0.

```shell
$ human-physiology solve hp_anion_gap "Na=140 mmol/L" "Cl=104 mmol/L" "HCO3=24 mmol/L"
AG = 12
AG_corrected = 12
  by: Serum anion gap and albumin-corrected anion gap  (AG = Na - Cl - HCO3; + 2.5 per g/dL of albumin below 4.0)
  Na = 140 mol/m^3
  Cl = 104 mol/m^3
  HCO3 = 24 mol/m^3
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the serum anion gap using the difference between sodium and the sum of chloride and bicarbonate concentrations. The result, 12, reflects the concentration of unmeasured anions in the blood. Additionally, an albumin-corrected value, 12, is provided to adjust for variations in albumin levels, which serve as a major unmeasured anion. This correction helps ensure that the assessment of acid-base status remains accurate even when albumin levels deviate from the standard reference range.

<a id="hp_respiratory_mechanics"></a>

#### Static and dynamic compliance and airway resistance, `hp_respiratory_mechanics`

Static compliance VT/(Pplat - PEEP), dynamic compliance VT/(Ppeak - PEEP), airway resistance (Ppeak - Pplat)/flow and driving pressure.

$$
C_{st} = \frac{V_T}{P_{plat} - PEEP},\quad R_{aw} = \frac{P_{peak} - P_{plat}}{\dot V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VT` | in | tidal volume (e.g. in mL) | m^3 |
| `Ppeak` | in | peak airway pressure in cmH2O | - |
| `Pplat` | in | plateau pressure in cmH2O | - |
| `PEEP` | in | end-expiratory pressure in cmH2O | - |
| `flow` | in | constant inspiratory flow (e.g. in L/min) | m^3/s |
| `C_static` | out | static compliance in mL/cmH2O | - |
| `C_dynamic` | out | dynamic compliance in mL/cmH2O | - |
| `R_aw` | out | airway resistance in cmH2O s/L | - |
| `driving_pressure` | out | driving pressure in cmH2O | - |

**Applies when:** Ppeak >= Pplat > PEEP >= 0; constant flow.

```shell
$ human-physiology solve hp_respiratory_mechanics "VT=500 mL" Ppeak=25 Pplat=20 PEEP=5 "flow=60 L/min"
C_static = 33.33333333
C_dynamic = 25
R_aw = 5
driving_pressure = 15
  by: Static and dynamic compliance and airway resistance  (pressure differences over volume and flow)
  VT = 0.0005 m^3
  Ppeak = 25
  Pplat = 20
  PEEP = 5
  flow = 0.001 m^3/s
  assumes: Ppeak >= Pplat > PEEP >= 0; constant flow
```

**What the worked example means:** This calculation determines lung mechanics from ventilator pressures and volumes. Static compliance, reported as 33.3333, measures the lung's distensibility by dividing tidal volume by the difference between plateau pressure and positive end-expiratory pressure. Dynamic compliance, shown as 25, uses peak pressure instead of plateau pressure to account for both elastic and resistive properties. Airway resistance, calculated as 5, reflects the pressure drop across the airways during flow, derived from the difference between peak and plateau pressures divided by inspiratory flow. Driving pressure, given as 15, represents the transpulmonary pressure required to deliver the tidal volume, calculated as plateau pressure minus positive end-expiratory pressure.

<a id="hp_oxygen_delivery"></a>

#### Systemic oxygen delivery: `hp_oxygen_delivery`

DO2 = CO x CaO2 x 10 (mL O2/min), with CaO2 = 1.34 Hb SaO2 + 0.003 PaO2.

$$
D_{O_2} = CO \times C_aO_2 \times 10
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CO` | in | cardiac output (e.g. in L/min) | m^3/s |
| `Hb` | in | hemoglobin in g/dL | - |
| `SaO2` | in | arterial saturation | - |
| `PaO2` | in | arterial PO2 (e.g. in mmHg) | Pa |
| `DO2` | out | oxygen delivery in mL O2/min | - |
| `CaO2` | out | arterial oxygen content in mL/dL | - |

**Applies when:** CO, Hb > 0; 0 <= SaO2 <= 1.

```shell
$ human-physiology solve hp_oxygen_delivery "CO=5 L/min" Hb=15 SaO2=0.98 "PaO2=100 mmHg"
DO2 = 999.9
CaO2 = 19.998
  by: Systemic oxygen delivery  (DO2 = 10 CO CaO2)
  CO = 8.333333333e-05 m^3/s
  Hb = 15
  SaO2 = 0.98
  PaO2 = 13332.23874 Pa
  assumes: CO, Hb > 0; 0 <= SaO2 <= 1
```

**What the worked example means:** This calculation determines systemic oxygen delivery, which represents the volume of oxygen transported to tissues per minute. It combines cardiac output with arterial oxygen content, where content depends on hemoglobin concentration, saturation, and partial pressure. The resulting delivery value of 999.9 indicates how much oxygen is available for tissue consumption based on the current circulatory and respiratory status.

<a id="hp_virtual_po2"></a>

#### Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus), `hp_virtual_po2`

Virtual PO2 = PO2 x 10^(0.024 (37 - T) + 0.40 (pH - 7.40) + 0.06 log10(40/PCO2)) and the saturation from the standard curve (Bohr effect).

$$
P_{virt} = P\cdot 10^{0.024(37-T) + 0.40(pH - 7.40) + 0.06\log_{10}(40/P_{CO_2})}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `PO2` | in | measured PO2 (e.g. in mmHg) | Pa |
| `T` | in | temperature (e.g. in degC) | K |
| `pH` | in | blood pH | - |
| `PCO2` | in | PCO2 (e.g. in mmHg) | Pa |
| `PO2_virtual` | out | standard-condition (virtual) PO2 in mmHg | - |
| `SO2` | out | fractional saturation | - |
| `factor` | out | correction factor | - |

**Applies when:** 25 <= T <= 43 degC; 6.8 <= pH <= 7.8.

```shell
$ human-physiology solve hp_virtual_po2 "PO2=80 mmHg" "T=39 degC" pH=7.3 "PCO2=50 mmHg"
PO2_virtual = 64.45778629
SO2 = 0.9222276859
factor = 0.8057223286
  by: Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus)  (Severinghaus (1979) correction, then the standard curve)
  PO2 = 10665.79099 Pa
  T = 312.15 K
  pH = 7.3
  PCO2 = 6666.119371 Pa
  assumes: 25 <= T <= 43 degC; 6.8 <= pH <= 7.8
```

**What the worked example means:** This calculation determines the standard-condition virtual partial pressure of oxygen and hemoglobin saturation by applying the Severinghaus correction. It adjusts the measured partial pressure of oxygen (80 mmHg) for deviations in temperature (39 degC), pH (7.3), and carbon dioxide partial pressure (50 mmHg) from standard values. The resulting correction factor (0.805722) scales the input to yield the virtual partial pressure (64.4578), which is then used to find the corresponding fractional saturation (0.922228) on the standard dissociation curve, effectively accounting for the Bohr effect.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
