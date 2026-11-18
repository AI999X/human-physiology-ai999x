# Human Physiology: the desktop app, user guide

**Human Physiology**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Human Physiology covers cardiac output, blood pressure, vascular resistance and Poiseuille flow, lung volumes, ventilation and the alveolar gas equation, oxygen content and binding, acid-base, renal clearance and eGFR, Nernst and Goldman-Hodgkin-Katz potentials, cable properties, body composition and metabolic rate (computed values, not medical advice). Its desktop app holds all 98 calculations in 8 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-human-physiology-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-human-physiology-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-human-physiology-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-human-physiology.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-human-physiology-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-human-physiology-1.6.5-linux-x86_64.tar.gz
./ai999x-human-physiology-1.6.5-linux-x86_64/ai999x-human-physiology
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Human Physiology app, opened on Cardiac output from heart rate and stroke volume with its worked example.](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/hero.webp)

*The Human Physiology app, opened on Cardiac output from heart rate and stroke volume with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Cardiovascular › Cardiac output from heart rate and stroke volume (or type part of its name in the search box). It opens with a worked example filled in: “cardiac output: HR = 72, SV = 70 mL” The answer: CO = 5.04, CO_mL = 5040.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Cardiac output from heart rate and stroke volume: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-1.webp)

*Cardiac output from heart rate and stroke volume: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “cardiac output: HR = 72, SV = 70 mL”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Cardiac output from heart rate and stroke volume
CO = 5.04
CO_mL = 5040
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Alveolar gas equation](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-2.webp)

*Alveolar gas equation*

![Nernst equilibrium potential of an ion](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-3.webp)

*Nernst equilibrium potential of an ion*

![Estimated GFR by the CKD-EPI 2021 creatinine equation](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-4.webp)

*Estimated GFR by the CKD-EPI 2021 creatinine equation*

![Red-cell indices: MCV, MCH and MCHC](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-5.webp)

*Red-cell indices: MCV, MCH and MCHC*

![Body mass index](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-6.webp)

*Body mass index*

![Body fluid compartments](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-7.webp)

*Body fluid compartments*

![Conduction velocity scaling with fibre diameter](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/scene-8.webp)

*Conduction velocity scaling with fibre diameter*

![Cardiac output from heart rate and stroke volume: a chart, enlarged.](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-1.webp)

*Cardiac output from heart rate and stroke volume: a chart, enlarged.*

![Alveolar gas equation: a chart, enlarged.](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/chart-2.webp)

*Alveolar gas equation: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Human Physiology app in the dark theme.](https://www.ai999x.com/?ax_shot=human-physiology-ai999x/hero-dark.webp)

*The Human Physiology app in the dark theme.*

## 7. Every calculation

98 calculations in 8 topics; each has its own entry on the model's page (https://www.ai999x.com/models/human-physiology-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Blood | Red-cell indices: MCV, MCH and MCHC |
| Body composition | Body density and fat by hydrostatic weighing, Body fat from body density (Siri and Brozek), Body mass index, Body surface area (DuBois, Mosteller, Haycock, Gehan-George), Lean body mass by the Boer formula |
| Body fluids | Body fluid compartments, Gibbs-Donnan equilibrium with an impermeant anion, Osmolarity of a solution, Osmotic pressure by van 't Hoff's law, Total body water by the Watson equations, Two-compartment fluid shift after adding a solution (Darrow-Yannet) |
| Cardiovascular | Bernoulli pressure drop across a narrowing, Cardiac index and stroke volume index, Cardiac output by the Fick principle, Cardiac output from heart rate and stroke volume, Compliance and elastance, Effect of a change of vessel radius on flow and resistance, Fick cardiac output from hemoglobin, saturations and PO2, Heart rate from the ECG (R-R interval or grid boxes), Heart-rate corrected QT interval (Bazett, Fridericia, Framingham), Hydrostatic pressure of a blood column, Laplace wall tension and wall stress of a vessel or chamber, Left-ventricular volumes and ejection fraction by the Teichholz formula, Mean arterial pressure and pulse pressure, Mean blood velocity from flow and total cross-sectional area, Mean QRS (electrical) axis from leads I and aVF, Poiseuille flow, mean velocity and wall shear stress in a vessel, Poiseuille resistance of a blood vessel, Pulmonary vascular resistance and transpulmonary gradient, Pulmonary-to-systemic flow ratio Qp/Qs, Pulse wave velocity by the Moens-Korteweg equation, Reynolds number of blood flow in a vessel, Starling forces and capillary filtration, Stroke volume and ejection fraction, Stroke work and cardiac power, Systemic vascular resistance (total peripheral resistance), Total resistance of vascular beds in series or in parallel, Womersley number of pulsatile blood flow |
| Metabolism | Basal metabolic rate by the Harris-Benedict equation, Basal metabolic rate by the Katch-McArdle equation, Energy expenditure of an activity from METs, First-order elimination: rate constant and half-life from clearance and volume, Food energy by the Atwater factors, Maximal heart rate, heart rate reserve and Karvonen target, Respiratory quotient and energy expenditure (Weir equation), Resting energy expenditure by the Mifflin-St Jeor equation |
| Nerve and muscle | Charge and number of ions to change the membrane potential, Conduction velocity scaling with fibre diameter, Hill muscle force-velocity relation, Ionic current and driving force, Length constant of an axon and steady-state attenuation, Maximal power of a muscle from Hill's equation, Membrane potential by the chord conductance equation, Membrane time constant and charging, Nernst equilibrium potential of an ion, Nerve conduction velocity from two stimulation sites, Resting membrane potential by the Goldman-Hodgkin-Katz equation, Threshold current for a pulse width (strength-duration curve) |
| Renal | Calculated and effective plasma osmolality and the osmolal gap, Creatinine clearance by the Cockcroft-Gault equation, Creatinine clearance from a timed urine collection, Estimated GFR by the CKD-EPI 2021 creatinine equation, Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation, Filtered load, excretion and net tubular reabsorption or secretion, Filtration fraction, Fractional excretion of a solute, Glucose titration curve (ideal transport maximum), Osmolar and free water clearance, Renal clearance of a substance (inulin GFR), Renal plasma and blood flow from PAH clearance |
| Respiratory | Alveolar gas equation, Alveolar ventilation from CO2 output and arterial PCO2, Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio, Arterial oxygen content, ATPS to BTPS and STPD gas volume conversion, Barometric pressure and inspired PO2 at altitude, Bicarbonate from pH and PCO2 (Henderson-Hasselbalch), Blood pH by the Henderson-Hasselbalch equation, FEV1/FVC ratio, Fick's law of diffusion across a membrane, Hill coefficient and P50 from two points of the dissociation curve, Laplace pressure of an alveolus, Lung capacities from the four lung volumes, Minute and alveolar ventilation, Oxygen consumption and extraction ratio (reverse Fick), Oxyhemoglobin saturation by the Hill equation, PCO2 from pH and bicarbonate (Henderson-Hasselbalch), Physiological dead space by the Bohr (Enghoff) equation, Physiological shunt fraction (shunt equation), PO2 at a given saturation by the Hill equation, PO2 at a given saturation by the Severinghaus equation, Respiratory time constant and passive exhalation, Saturation by the Severinghaus (1979) dissociation equation, Serum anion gap and albumin-corrected anion gap, Static and dynamic compliance and airway resistance, Systemic oxygen delivery, Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus) |

## 8. Other ways to use it

The same engine runs on the command line (`human-physiology`, its own guide), in Python (`pip install human-physiology-ai999x`, `import human_physiology_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/human-physiology-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/human-physiology-ai999x/
