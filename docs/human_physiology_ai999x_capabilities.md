# human-physiology-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `human_physiology_ai999x.about()` for the same answer.

Every capability human-physiology-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import human_physiology_ai999x

result = human_physiology_ai999x.solve('hp_cardiac_output', HR=72, SV='7e-05 m^3')
result.show()
# CO = 5.04, CO_mL = 5040
```

## By topic

| topic | capabilities |
|---|---|
| Blood / red cells | [Red-cell indices: MCV, MCH and MCHC](#hp_red_cell_indices) |
| Body composition / anthropometry | [Body mass index](#hp_bmi), [Body surface area (DuBois, Mosteller, Haycock, Gehan-George)](#hp_body_surface_area), [Lean body mass by the Boer formula](#hp_lean_body_mass_boer) |
| Body composition / densitometry | [Body fat from body density (Siri and Brozek)](#hp_body_fat_from_density), [Body density and fat by hydrostatic weighing](#hp_hydrostatic_weighing) |
| Body fluids / compartments | [Body fluid compartments](#hp_body_fluid_compartments), [Total body water by the Watson equations](#hp_watson_total_body_water) |
| Body fluids / membrane equilibria | [Gibbs-Donnan equilibrium with an impermeant anion](#hp_gibbs_donnan) |
| Body fluids / osmolarity | [Osmolarity of a solution](#hp_solution_osmolarity), [Osmotic pressure by van 't Hoff's law](#hp_osmotic_pressure), [Two-compartment fluid shift after adding a solution (Darrow-Yannet)](#hp_fluid_shift) |
| Cardiovascular / blood pressure | [Mean arterial pressure and pulse pressure](#hp_mean_arterial_pressure), [Hydrostatic pressure of a blood column](#hp_hydrostatic_pressure) |
| Cardiovascular / cardiac output | [Cardiac output from heart rate and stroke volume](#hp_cardiac_output), [Stroke volume and ejection fraction](#hp_stroke_volume_ef), [Cardiac index and stroke volume index](#hp_cardiac_index), [Cardiac output by the Fick principle](#hp_fick_cardiac_output), [Fick cardiac output from hemoglobin, saturations and PO2](#hp_fick_saturations), [Left-ventricular volumes and ejection fraction by the Teichholz formula](#hp_teichholz_volumes) |
| Cardiovascular / cardiac work | [Stroke work and cardiac power](#hp_stroke_work) |
| Cardiovascular / electrocardiogram | [Heart-rate corrected QT interval (Bazett, Fridericia, Framingham)](#hp_qt_correction), [Mean QRS (electrical) axis from leads I and aVF](#hp_qrs_axis), [Heart rate from the ECG (R-R interval or grid boxes)](#hp_ecg_heart_rate) |
| Cardiovascular / flow and resistance | [Systemic vascular resistance (total peripheral resistance)](#hp_systemic_vascular_resistance), [Pulmonary vascular resistance and transpulmonary gradient](#hp_pulmonary_vascular_resistance), [Poiseuille resistance of a blood vessel](#hp_poiseuille_resistance), [Poiseuille flow, mean velocity and wall shear stress in a vessel](#hp_poiseuille_flow), [Effect of a change of vessel radius on flow and resistance](#hp_radius_flow_ratio), [Total resistance of vascular beds in series or in parallel](#hp_vessels_series_parallel), [Reynolds number of blood flow in a vessel](#hp_vessel_reynolds), [Mean blood velocity from flow and total cross-sectional area](#hp_blood_velocity_area), [Bernoulli pressure drop across a narrowing](#hp_bernoulli_pressure_drop), [Womersley number of pulsatile blood flow](#hp_womersley_number) |
| Cardiovascular / microcirculation | [Starling forces and capillary filtration](#hp_starling_filtration) |
| Cardiovascular / shunts | [Pulmonary-to-systemic flow ratio Qp/Qs](#hp_qp_qs) |
| Cardiovascular / wall mechanics | [Laplace wall tension and wall stress of a vessel or chamber](#hp_laplace_wall_tension), [Compliance and elastance](#hp_vascular_compliance), [Pulse wave velocity by the Moens-Korteweg equation](#hp_moens_korteweg) |
| Metabolism / clearance kinetics | [First-order elimination: rate constant and half-life from clearance and volume](#hp_elimination_half_life) |
| Metabolism / energy balance | [Food energy by the Atwater factors](#hp_atwater_energy) |
| Metabolism / energy expenditure | [Basal metabolic rate by the Harris-Benedict equation](#hp_bmr_harris_benedict), [Resting energy expenditure by the Mifflin-St Jeor equation](#hp_bmr_mifflin_st_jeor), [Basal metabolic rate by the Katch-McArdle equation](#hp_bmr_katch_mcardle), [Energy expenditure of an activity from METs](#hp_met_energy) |
| Metabolism / exercise | [Maximal heart rate, heart rate reserve and Karvonen target](#hp_heart_rate_reserve) |
| Metabolism / indirect calorimetry | [Respiratory quotient and energy expenditure (Weir equation)](#hp_weir_energy_expenditure) |
| Nerve and muscle / cable properties | [Membrane time constant and charging](#hp_membrane_time_constant), [Length constant of an axon and steady-state attenuation](#hp_length_constant) |
| Nerve and muscle / conduction | [Conduction velocity scaling with fibre diameter](#hp_conduction_velocity_scaling), [Nerve conduction velocity from two stimulation sites](#hp_nerve_conduction_velocity) |
| Nerve and muscle / excitability | [Threshold current for a pulse width (strength-duration curve)](#hp_strength_duration) |
| Nerve and muscle / membrane potential | [Nernst equilibrium potential of an ion](#hp_nernst_potential), [Resting membrane potential by the Goldman-Hodgkin-Katz equation](#hp_ghk_voltage), [Membrane potential by the chord conductance equation](#hp_chord_conductance), [Ionic current and driving force](#hp_ionic_current), [Charge and number of ions to change the membrane potential](#hp_membrane_charge) |
| Nerve and muscle / muscle mechanics | [Hill muscle force-velocity relation](#hp_hill_force_velocity), [Maximal power of a muscle from Hill's equation](#hp_muscle_max_power) |
| Renal / body fluids | [Calculated and effective plasma osmolality and the osmolal gap](#hp_plasma_osmolality) |
| Renal / clearance | [Renal clearance of a substance (inulin GFR)](#hp_renal_clearance), [Creatinine clearance from a timed urine collection](#hp_creatinine_clearance_timed), [Filtration fraction](#hp_filtration_fraction) |
| Renal / GFR estimation | [Estimated GFR by the CKD-EPI 2021 creatinine equation](#hp_egfr_ckd_epi_2021), [Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation](#hp_egfr_ckd_epi_2021_cys), [Creatinine clearance by the Cockcroft-Gault equation](#hp_cockcroft_gault) |
| Renal / renal blood flow | [Renal plasma and blood flow from PAH clearance](#hp_pah_renal_flow) |
| Renal / tubular function | [Fractional excretion of a solute](#hp_fractional_excretion), [Filtered load, excretion and net tubular reabsorption or secretion](#hp_tubular_handling), [Glucose titration curve (ideal transport maximum)](#hp_glucose_titration) |
| Renal / water balance | [Osmolar and free water clearance](#hp_free_water_clearance) |
| Respiratory / acid-base | [Blood pH by the Henderson-Hasselbalch equation](#hp_ph_henderson_hasselbalch), [Bicarbonate from pH and PCO2 (Henderson-Hasselbalch)](#hp_bicarbonate_from_ph), [PCO2 from pH and bicarbonate (Henderson-Hasselbalch)](#hp_pco2_from_ph), [Serum anion gap and albumin-corrected anion gap](#hp_anion_gap) |
| Respiratory / environment | [Barometric pressure and inspired PO2 at altitude](#hp_barometric_altitude) |
| Respiratory / gas exchange | [Physiological shunt fraction (shunt equation)](#hp_shunt_fraction), [Alveolar gas equation](#hp_alveolar_gas_equation), [Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio](#hp_aa_gradient), [Fick's law of diffusion across a membrane](#hp_fick_diffusion) |
| Respiratory / gas laws | [ATPS to BTPS and STPD gas volume conversion](#hp_gas_volume_conversion) |
| Respiratory / lung volumes | [Lung capacities from the four lung volumes](#hp_lung_capacities) |
| Respiratory / mechanics | [Static and dynamic compliance and airway resistance](#hp_respiratory_mechanics), [Respiratory time constant and passive exhalation](#hp_respiratory_time_constant), [Laplace pressure of an alveolus](#hp_alveolar_laplace_pressure) |
| Respiratory / oxygen binding | [Oxyhemoglobin saturation by the Hill equation](#hp_hill_saturation), [PO2 at a given saturation by the Hill equation](#hp_hill_po2), [Hill coefficient and P50 from two points of the dissociation curve](#hp_hill_fit), [Saturation by the Severinghaus (1979) dissociation equation](#hp_severinghaus_saturation), [PO2 at a given saturation by the Severinghaus equation](#hp_severinghaus_po2), [Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus)](#hp_virtual_po2) |
| Respiratory / oxygen transport | [Arterial oxygen content](#hp_oxygen_content), [Systemic oxygen delivery](#hp_oxygen_delivery), [Oxygen consumption and extraction ratio (reverse Fick)](#hp_oxygen_consumption) |
| Respiratory / spirometry | [FEV1/FVC ratio](#hp_fev1_fvc_ratio) |
| Respiratory / ventilation | [Minute and alveolar ventilation](#hp_minute_ventilation), [Physiological dead space by the Bohr (Enghoff) equation](#hp_bohr_dead_space), [Alveolar ventilation from CO2 output and arterial PCO2](#hp_alveolar_ventilation_co2) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| hp_cardiac_output | Cardiac output from heart rate and stroke volume | HR, SV | CO, CO_mL |
| hp_stroke_volume_ef | Stroke volume and ejection fraction | EDV, ESV | SV, EF, EF_percent |
| hp_cardiac_index | Cardiac index and stroke volume index | CO, BSA, HR | CI, SVI, SV |
| hp_fick_cardiac_output | Cardiac output by the Fick principle | VO2, CaO2, CvO2 | CO, avDO2 |
| hp_fick_saturations | Fick cardiac output from hemoglobin, saturations and PO2 | VO2, Hb, SaO2, SvO2, PaO2, PvO2 | CO, CaO2, CvO2 |
| hp_mean_arterial_pressure | Mean arterial pressure and pulse pressure | SBP, DBP | MAP, PP |
| hp_systemic_vascular_resistance | Systemic vascular resistance (total peripheral resistance) | MAP, CVP, CO | SVR, SVR_dyn |
| hp_pulmonary_vascular_resistance | Pulmonary vascular resistance and transpulmonary gradient | mPAP, PCWP, CO | PVR, PVR_dyn, TPG |
| hp_poiseuille_resistance | Poiseuille resistance of a blood vessel | eta, L, r | R, R_mmHg |
| hp_poiseuille_flow | Poiseuille flow, mean velocity and wall shear stress in a vessel | dP, eta, L, r | Q, Q_mL_min, v_mean, tau_wall |
| hp_radius_flow_ratio | Effect of a change of vessel radius on flow and resistance | r1, r2 | flow_ratio, resistance_ratio |
| hp_vessels_series_parallel | Total resistance of vascular beds in series or in parallel | R, arrangement | R_total, G_total |
| hp_vessel_reynolds | Reynolds number of blood flow in a vessel | rho, v, D, eta | Re, v_critical |
| hp_laplace_wall_tension | Laplace wall tension and wall stress of a vessel or chamber | P, r, shape, h | T, stress |
| hp_vascular_compliance | Compliance and elastance | dV, dP | C, E |
| hp_blood_velocity_area | Mean blood velocity from flow and total cross-sectional area | Q, A | v |
| hp_bernoulli_pressure_drop | Bernoulli pressure drop across a narrowing | v1, v2, rho | dP, dP_mmHg, dP_simplified |
| hp_stroke_work | Stroke work and cardiac power | SV, MAP, HR | SW, SW_gm, power |
| hp_teichholz_volumes | Left-ventricular volumes and ejection fraction by the Teichholz formula | LVIDd, LVIDs | EDV, ESV, EF, FS |
| hp_qt_correction | Heart-rate corrected QT interval (Bazett, Fridericia, Framingham) | QT, HR | RR, QTc_Bazett, QTc_Fridericia, QTc_Framingham |
| hp_qrs_axis | Mean QRS (electrical) axis from leads I and aVF | lead_I, aVF | axis, amplitude |
| hp_starling_filtration | Starling forces and capillary filtration | Pc, Pi, pic, pii, Kf, sigma | NFP, Jv |
| hp_hydrostatic_pressure | Hydrostatic pressure of a blood column | h, rho | dP, dP_mmHg |
| hp_moens_korteweg | Pulse wave velocity by the Moens-Korteweg equation | E, h, D, rho | PWV |
| hp_womersley_number | Womersley number of pulsatile blood flow | r, HR, rho, eta | alpha, omega |
| hp_qp_qs | Pulmonary-to-systemic flow ratio Qp/Qs | SaO2, SvO2, SpvO2, SpaO2 | Qp_Qs |
| hp_shunt_fraction | Physiological shunt fraction (shunt equation) | CcO2, CaO2, CvO2 | Qs_Qt |
| hp_lung_capacities | Lung capacities from the four lung volumes | TV, IRV, ERV, RV | IC, FRC, VC, TLC |
| hp_minute_ventilation | Minute and alveolar ventilation | VT, RR, VD | VE, VA, VD_VT |
| hp_bohr_dead_space | Physiological dead space by the Bohr (Enghoff) equation | PaCO2, PECO2, VT | VD_VT, VD |
| hp_alveolar_ventilation_co2 | Alveolar ventilation from CO2 output and arterial PCO2 | VCO2, PaCO2 | VA |
| hp_alveolar_gas_equation | Alveolar gas equation | FiO2, Patm, PH2O, PaCO2, RQ | PiO2, PAO2, PAO2_full |
| hp_aa_gradient | Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio | FiO2, Patm, PH2O, PaCO2, PaO2, RQ | PAO2, Aa_gradient, a_A_ratio, PF_ratio |
| hp_oxygen_content | Arterial oxygen content | Hb, SaO2, PaO2 | CaO2, bound, dissolved |
| hp_oxygen_delivery | Systemic oxygen delivery | CO, Hb, SaO2, PaO2 | DO2, CaO2 |
| hp_oxygen_consumption | Oxygen consumption and extraction ratio (reverse Fick) | CO, Hb, SaO2, SvO2, PaO2, PvO2 | VO2, O2ER, CaO2, CvO2 |
| hp_hill_saturation | Oxyhemoglobin saturation by the Hill equation | PO2, P50, n | SO2 |
| hp_hill_po2 | PO2 at a given saturation by the Hill equation | SO2, P50, n | PO2 |
| hp_hill_fit | Hill coefficient and P50 from two points of the dissociation curve | P1, S1, P2, S2 | n, P50 |
| hp_severinghaus_saturation | Saturation by the Severinghaus (1979) dissociation equation | PO2 | SO2 |
| hp_severinghaus_po2 | PO2 at a given saturation by the Severinghaus equation | SO2 | PO2 |
| hp_virtual_po2 | Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus) | PO2, T, pH, PCO2 | PO2_virtual, SO2, factor |
| hp_ph_henderson_hasselbalch | Blood pH by the Henderson-Hasselbalch equation | HCO3, PaCO2 | pH, H |
| hp_bicarbonate_from_ph | Bicarbonate from pH and PCO2 (Henderson-Hasselbalch) | pH, PaCO2 | HCO3 |
| hp_pco2_from_ph | PCO2 from pH and bicarbonate (Henderson-Hasselbalch) | pH, HCO3 | PaCO2 |
| hp_anion_gap | Serum anion gap and albumin-corrected anion gap | Na, Cl, HCO3, albumin | AG, AG_corrected |
| hp_fick_diffusion | Fick's law of diffusion across a membrane | D, A, dC, T | J, t_diffusion |
| hp_respiratory_mechanics | Static and dynamic compliance and airway resistance | VT, Ppeak, Pplat, PEEP, flow | C_static, C_dynamic, R_aw, driving_pressure |
| hp_respiratory_time_constant | Respiratory time constant and passive exhalation | R, C, t | tau, fraction_exhaled, t_95 |
| hp_alveolar_laplace_pressure | Laplace pressure of an alveolus | gamma, r | P, P_cmH2O |
| hp_barometric_altitude | Barometric pressure and inspired PO2 at altitude | h, FiO2 | Patm, PiO2, Patm_kPa |
| hp_gas_volume_conversion | ATPS to BTPS and STPD gas volume conversion | V, T, Patm, PH2O | V_BTPS, V_STPD |
| hp_weir_energy_expenditure | Respiratory quotient and energy expenditure (Weir equation) | VO2, VCO2 | RQ, EE, EE_kJ |
| hp_fev1_fvc_ratio | FEV1/FVC ratio | FEV1, FVC | ratio, percent |
| hp_renal_clearance | Renal clearance of a substance (inulin GFR) | U, V, P | C, excretion |
| hp_creatinine_clearance_timed | Creatinine clearance from a timed urine collection | Ucr, Vurine, hours, Pcr, BSA | urine_flow, CrCl, CrCl_normalized |
| hp_pah_renal_flow | Renal plasma and blood flow from PAH clearance | U_PAH, V, P_PAH, Hct, E | eRPF, RPF, RBF |
| hp_filtration_fraction | Filtration fraction | GFR, RPF | FF, FF_percent |
| hp_free_water_clearance | Osmolar and free water clearance | V, Uosm, Posm | Cosm, CH2O |
| hp_fractional_excretion | Fractional excretion of a solute | Ux, Px, Ucr, Pcr | FE, FE_percent |
| hp_egfr_ckd_epi_2021 | Estimated GFR by the CKD-EPI 2021 creatinine equation | Scr, age, sex | eGFR |
| hp_egfr_ckd_epi_2021_cys | Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation | Scr, Scys, age, sex | eGFR |
| hp_cockcroft_gault | Creatinine clearance by the Cockcroft-Gault equation | age, weight, Scr, sex | CrCl |
| hp_tubular_handling | Filtered load, excretion and net tubular reabsorption or secretion | GFR, Px, Ux, V | filtered_load, excreted, reabsorbed |
| hp_glucose_titration | Glucose titration curve (ideal transport maximum) | GFR, Pglucose, Tm | filtered_load, reabsorbed, excreted, threshold |
| hp_plasma_osmolality | Calculated and effective plasma osmolality and the osmolal gap | Na, glucose, BUN, measured | calculated, effective, osmolal_gap |
| hp_body_fluid_compartments | Body fluid compartments | weight, TBW_fraction | TBW, ICF, ECF, ISF, plasma |
| hp_watson_total_body_water | Total body water by the Watson equations | age, height, weight, sex | TBW |
| hp_solution_osmolarity | Osmolarity of a solution | conc, molar_mass, i, phi | molarity, osmolarity |
| hp_osmotic_pressure | Osmotic pressure by van 't Hoff's law | osm, T | pi, pi_mmHg, pi_atm |
| hp_fluid_shift | Two-compartment fluid shift after adding a solution (Darrow-Yannet) | TBW, Posm, V_added, C_added, ICF_fraction | osm_final, ICF, ECF |
| hp_gibbs_donnan | Gibbs-Donnan equilibrium with an impermeant anion | A, c, T | Na_in, Cl_in, r, V_mV, osmotic_excess |
| hp_nernst_potential | Nernst equilibrium potential of an ion | z, Cout, Cin, T | E, RT_F |
| hp_ghk_voltage | Resting membrane potential by the Goldman-Hodgkin-Katz equation | PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i, T | Vm |
| hp_chord_conductance | Membrane potential by the chord conductance equation | gK, gNa, gCl, EK, ENa, ECl | Vm |
| hp_ionic_current | Ionic current and driving force | g, Vm, E | I, driving_force |
| hp_membrane_time_constant | Membrane time constant and charging | Rm, Cm, t | tau, fraction |
| hp_length_constant | Length constant of an axon and steady-state attenuation | Rm, Ri, d, x | lambda, attenuation |
| hp_conduction_velocity_scaling | Conduction velocity scaling with fibre diameter | v1, d1, d2, fiber | v2, ratio |
| hp_nerve_conduction_velocity | Nerve conduction velocity from two stimulation sites | distance, latency_proximal, latency_distal | NCV |
| hp_membrane_charge | Charge and number of ions to change the membrane potential | Cm, dV, area | Q_pC, ions |
| hp_strength_duration | Threshold current for a pulse width (strength-duration curve) | rheobase, chronaxie, t | I_lapicque, I_weiss |
| hp_hill_force_velocity | Hill muscle force-velocity relation | F0, a, b, v | F, power, vmax |
| hp_muscle_max_power | Maximal power of a muscle from Hill's equation | F0, a, b | v_opt, F_opt, P_max, vmax |
| hp_bmi | Body mass index | weight, height | BMI |
| hp_body_surface_area | Body surface area (DuBois, Mosteller, Haycock, Gehan-George) | weight, height | BSA_DuBois, BSA_Mosteller, BSA_Haycock, BSA_GehanGeorge |
| hp_bmr_harris_benedict | Basal metabolic rate by the Harris-Benedict equation | weight, height, age, sex, version | BMR, BMR_kJ |
| hp_bmr_mifflin_st_jeor | Resting energy expenditure by the Mifflin-St Jeor equation | weight, height, age, sex | REE, REE_kJ |
| hp_bmr_katch_mcardle | Basal metabolic rate by the Katch-McArdle equation | weight, body_fat | LBM, BMR |
| hp_met_energy | Energy expenditure of an activity from METs | MET, weight, duration | VO2, kcal_per_min, kcal |
| hp_atwater_energy | Food energy by the Atwater factors | carbohydrate, fat, protein, alcohol | kcal, kJ |
| hp_body_fat_from_density | Body fat from body density (Siri and Brozek) | density | fat_Siri, fat_Brozek |
| hp_hydrostatic_weighing | Body density and fat by hydrostatic weighing | mass_air, mass_water, water_density, RV, GI | volume, density, fat_Siri |
| hp_lean_body_mass_boer | Lean body mass by the Boer formula | weight, height, sex | LBM, fat_mass |
| hp_heart_rate_reserve | Maximal heart rate, heart rate reserve and Karvonen target | age, HR_rest, intensity | HRmax, HRmax_Tanaka, HRR, target |
| hp_elimination_half_life | First-order elimination: rate constant and half-life from clearance and volume | CL, Vd, t | k, half_life, fraction_remaining |
| hp_red_cell_indices | Red-cell indices: MCV, MCH and MCHC | Hct, Hb, RBC | MCV, MCH, MCHC |
| hp_ecg_heart_rate | Heart rate from the ECG (R-R interval or grid boxes) | RR, small_boxes, large_boxes, speed | HR, RR, RR_ms |

<a id="hp_cardiac_output"></a>

### Cardiac output from heart rate and stroke volume, `hp_cardiac_output`

Cardiac output $CO = HR$ x SV, in L/min and mL/min.

$$
CO = HR \times SV
$$

| input | meaning | condition |
|---|---|---|
| `HR` | heart rate in beats/min |  |
| `SV` | stroke volume (e.g. in mL) |  |

| output | meaning |
|---|---|
| `CO` | cardiac output in L/min |
| `CO_mL` | cardiac output in mL/min |

- **Method:** $CO = HR$ $SV / 1000$
- **Assumes:** $HR > 0$; $SV > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CO (cardiac output in L/min) against HR (heart rate in beats/min) from 36 to 108](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-cardiac-output-curve)

*CO (cardiac output in L/min) against HR (heart rate in beats/min) from 36 to 108. One curve for each value of SV (stroke volume (e.g. in mL)): 7.0000e-05 m^3, 3.5000e-05 m^3, 0.00014 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, HR = 72, CO = 5.04.*

![CO (cardiac output in L/min) over HR (heart rate in beats/min) from 36 to 108 and SV (stroke volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-cardiac-output-field)

*CO (cardiac output in L/min) over HR (heart rate in beats/min) from 36 to 108 and SV (stroke volume (e.g. in mL)) from 3.5000e-05 m^3 to 0.000105 m^3: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 1.26 to dark 11.34); iso-lines at CO = 2, 4, 6, 8, 10. Marked: the worked example, HR = 72, SV = 7.0000e-05 m^3, CO = 5.04.*

```python
human_physiology_ai999x.solve('hp_cardiac_output', HR=72, SV=7e-05)
# -> CO = 5.04, CO_mL = 5040   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_cardiac_output', HR=60, SV=7.999999999999999e-05)
# -> CO = 4.8, CO_mL = 4800   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| cardiac output: HR = 72, SV = 70 mL | SOLVED | CO = 5.04, CO_mL = 5040 |
| cardiac output: HR = 60, SV = 80 mL | SOLVED | CO = 4.8, CO_mL = 4800 |
| cardiac output: SV = 70 mL | NEEDS_INPUT | hp_cardiac_output needs HR (heart rate in beats/min). Example: solve('hp_cardiac_output', HR=72, SV='7e-05 m^3'). |
| cardiac output: HR = 72, SV = 70 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| cardiac output: HR = 0, SV = 70 mL | INVALID_INPUT | Cardiac output from heart rate and stroke volume: HR must be positive. |

<a id="hp_stroke_volume_ef"></a>

### Stroke volume and ejection fraction: `hp_stroke_volume_ef`

Stroke volume SV = EDV - ESV and ejection fraction EF = SV/EDV from end-diastolic and end-systolic volumes.

$$
SV = EDV - ESV,\quad EF = \frac{SV}{EDV}
$$

| input | meaning | condition |
|---|---|---|
| `EDV` | end-diastolic volume (e.g. in mL) |  |
| `ESV` | end-systolic volume (e.g. in mL) |  |

| output | meaning |
|---|---|
| `SV` | stroke volume in mL |
| `EF` | ejection fraction (fraction) |
| `EF_percent` | ejection fraction in % |

- **Method:** SV = EDV - ESV; EF = SV/EDV
- **Assumes:** 0 <= ESV < EDV
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![EF (ejection fraction (fraction)) against EDV (end-diastolic volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-stroke-volume-ef-curve)

*EF (ejection fraction (fraction)) against EDV (end-diastolic volume (e.g. in mL)) from 6.0000e-05 m^3 to 0.00018 m^3. One curve for each value of ESV (end-systolic volume (e.g. in mL)): 5.0000e-05 m^3, 2.5000e-05 m^3, 1.0000e-04 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, EDV = 0.00012 m^3, EF = 0.583333.*

![EF (ejection fraction (fraction)) over EDV (end-diastolic volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-stroke-volume-ef-field)

*EF (ejection fraction (fraction)) over EDV (end-diastolic volume (e.g. in mL)) from 6.0000e-05 m^3 to 0.00018 m^3 and ESV (end-systolic volume (e.g. in mL)) from 2.5000e-05 m^3 to 7.5000e-05 m^3: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.00793651 to dark 0.861111); iso-lines at EF = 0.2, 0.4, 0.6, 0.8; blank cells: inputs the capability refuses. Marked: the worked example, EDV = 0.00012 m^3, ESV = 5.0000e-05 m^3, EF = 0.583333.*

```python
human_physiology_ai999x.solve('hp_stroke_volume_ef', EDV=0.00011999999999999999, ESV=4.9999999999999996e-05)
# -> SV = 70, EF = 0.583333, EF_percent = 58.3333   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_stroke_volume_ef', EDV=0.00015, ESV=8.999999999999999e-05)
# -> SV = 60, EF = 0.4, EF_percent = 40   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| stroke volume and ejection fraction: EDV = 120 mL, ESV = 50 mL | SOLVED | SV = 70, EF = 0.583333, EF_percent = 58.3333 |
| stroke volume and ejection fraction: EDV = 150 mL, ESV = 90 mL | SOLVED | SV = 60, EF = 0.4, EF_percent = 40 |
| stroke volume and ejection fraction: ESV = 50 mL | NEEDS_INPUT | hp_stroke_volume_ef needs EDV (end-diastolic volume (e.g. in mL)). Example: solve('hp_stroke_volume_ef', EDV='0.00012 m^ |
| stroke volume and ejection fraction: EDV = 120 mL, ESV = 50 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| stroke volume and ejection fraction: EDV = 100 mL, ESV = 120 mL | INVALID_INPUT | Stroke volume and ejection fraction: ESV must be smaller than EDV. |

<a id="hp_cardiac_index"></a>

### Cardiac index and stroke volume index: `hp_cardiac_index`

Cardiac index CI = CO/BSA, stroke volume $SV = CO/HR$ and stroke volume index SVI = CI/HR.

$$
CI = \frac{CO}{BSA},\quad SVI = \frac{CI}{HR}
$$

| input | meaning | condition |
|---|---|---|
| `CO` | cardiac output (e.g. in L/min) |  |
| `BSA` | body surface area (e.g. in m^2) |  |
| `HR` | heart rate in beats/min (optional; needed for the stroke volume index) |  |

| output | meaning |
|---|---|
| `CI` | cardiac index in L/min/m^2 |
| `SVI` | stroke volume index in $mL/m^{2}$ |
| `SV` | stroke volume in mL |

- **Method:** CI = CO/BSA; SVI = 1000 CI/HR
- **Assumes:** CO, BSA, $HR > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CI (cardiac index in L/min/m^2) against BSA (body surface area (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-cardiac-index-curve)

*CI (cardiac index in L/min/m^2) against BSA (body surface area (e.g. in m^2)) from 0.9 m^2 to 2.7 m^2. One curve for each value of CO (cardiac output (e.g. in L/min)): 8.4000e-05 m^3/s, 4.2000e-05 m^3/s, 0.000168 m^3/s. Other inputs as in the worked example: HR = 72.0. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, BSA = 1.8 m^2, CI = 2.8.*

```python
human_physiology_ai999x.solve('hp_cardiac_index', CO=8.400000000000001e-05, BSA=1.8, HR=72)
# -> CI = 2.8, SVI = 38.8889, SV = 70   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_cardiac_index', CO=0.0001, BSA=2.0, HR=75)
# -> CI = 3, SVI = 40, SV = 80   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| cardiac index: CO = 5.04 L/min, BSA = 1.8 m^2, HR = 72 | SOLVED | CI = 2.8, SVI = 38.8889, SV = 70 |
| cardiac index: CO = 6 L/min, BSA = 2 m^2, HR = 75 | SOLVED | CI = 3, SVI = 40, SV = 80 |
| cardiac index: BSA = 1.8 m^2, HR = 72 | NEEDS_INPUT | hp_cardiac_index needs CO (cardiac output (e.g. in L/min)). Example: solve('hp_cardiac_index', CO='8.4e-05 m^3/s', BSA=' |
| cardiac index: CO = 5.04 L/min, BSA = 1.8 m^2, HR = 72, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| cardiac index: CO = 5 L/min, BSA = 0 m^2, HR = 70 | INVALID_INPUT | Cardiac index and stroke volume index: BSA must be positive. |

<a id="hp_fick_cardiac_output"></a>

### Cardiac output by the Fick principle: `hp_fick_cardiac_output`

Cardiac output from oxygen consumption and the arteriovenous oxygen content difference: CO = VO2/(CaO2 - CvO2).

$$
CO = \frac{\dot V_{O_2}}{C_aO_2 - C_{\bar v}O_2}
$$

| input | meaning | condition |
|---|---|---|
| `VO2` | oxygen consumption (e.g. in mL/min) |  |
| `CaO2` | arterial oxygen content in mL $O_{2}/dL$ |  |
| `CvO2` | mixed venous oxygen content in mL $O_{2}/dL$ |  |

| output | meaning |
|---|---|
| `CO` | cardiac output in L/min |
| `avDO2` | arteriovenous O2 content difference in mL/dL |

- **Method:** CO = VO2 / (10 (CaO2 - CvO2)) (contents per dL -> per L)
- **Assumes:** $VO2 > 0$; 0 <= CvO2 < CaO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CO (cardiac output in L/min) against CaO2 (arterial oxygen content in mL O2/dL) from 10 to 30](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fick-cardiac-output-curve)

*CO (cardiac output in L/min) against CaO2 (arterial oxygen content in mL O2/dL) from 10 to 30. One curve for each value of VO2 (oxygen consumption (e.g. in mL/min)): 4.1667e-06 m^3/s, 2.0833e-06 m^3/s, 8.3333e-06 m^3/s. Other inputs as in the worked example: CvO2 = 15.0. 23 points per curve computed by human-physiology-ai999x; marked: the worked example, CaO2 = 20, CO = 5.*

```python
human_physiology_ai999x.solve('hp_fick_cardiac_output', VO2=4.166666666666667e-06, CaO2=20, CvO2=15)
# -> CO = 5, avDO2 = 5   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_fick_cardiac_output', VO2=5e-06, CaO2=19.5, CvO2=14.5)
# -> CO = 6, avDO2 = 5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fick cardiac output: VO2 = 250 mL/min, CaO2 = 20, CvO2 = 15 | SOLVED | CO = 5, avDO2 = 5 |
| Fick cardiac output: VO2 = 300 mL/min, CaO2 = 19.5, CvO2 = 14.5 | SOLVED | CO = 6, avDO2 = 5 |
| Fick cardiac output: CaO2 = 20, CvO2 = 15 | NEEDS_INPUT | hp_fick_cardiac_output needs VO2 (oxygen consumption (e.g. in mL/min)). Example: solve('hp_fick_cardiac_output', VO2='4. |
| Fick cardiac output: VO2 = 250 mL/min, CaO2 = 20, CvO2 = 15, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Fick cardiac output: VO2 = 250 mL/min, CaO2 = 15, CvO2 = 20 | INVALID_INPUT | Cardiac output by the Fick principle: the venous oxygen content must be below the arterial. |

<a id="hp_fick_saturations"></a>

### Fick cardiac output from hemoglobin, saturations and PO2, `hp_fick_saturations`

Arterial and mixed venous O2 contents (1.34 Hb $S + 0.003$ PO2) and the Fick cardiac output VO2/(CaO2 - CvO2).

$$
CO = \frac{\dot V_{O_2}}{1.34\,Hb\,(S_aO_2 - S_{\bar v}O_2) + 0.003\,(P_aO_2 - P_{\bar v}O_2)}
$$

| input | meaning | condition |
|---|---|---|
| `VO2` | oxygen consumption (e.g. in mL/min) |  |
| `Hb` | hemoglobin in g/dL |  |
| `SaO2` | arterial saturation (fraction) |  |
| `SvO2` | mixed venous saturation (fraction) |  |
| `PaO2` | arterial PO2 (e.g. in mmHg) |  |
| `PvO2` | mixed venous PO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `CO` | cardiac output in L/min |
| `CaO2` | arterial O2 content in mL/dL |
| `CvO2` | venous O2 content in mL/dL |

- **Method:** $C = 1.34$ Hb $S + 0.003$ PO2; CO = VO2/(10 (CaO2 - CvO2))
- **Assumes:** saturations between 0 and 1; CvO2 < CaO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CO (cardiac output in L/min) against SvO2 (mixed venous saturation (fraction)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fick-saturations-curve)

*CO (cardiac output in L/min) against SvO2 (mixed venous saturation (fraction)) from 0.02 to 0.98. One curve for each value of SaO2 (arterial saturation (fraction)): 0.98, 0.26, 0.74. Other inputs as in the worked example: VO2 = 4.166666666666667e-06 m^3/s, Hb = 15.0, PaO2 = 12665.626804425001 Pa, PvO2 = 5332.8954966 Pa. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, SvO2 = 0.75, CO = 5.22139.*

```python
human_physiology_ai999x.solve('hp_fick_saturations', VO2=4.166666666666667e-06, Hb=15, SaO2=0.98, SvO2=0.75, PaO2=12665.626804425001, PvO2=5332.8954966)
# -> CO = 5.22139, CaO2 = 19.983, CvO2 = 15.195   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fick cardiac output from saturations: VO2 = 250 mL/min, Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 95 mmHg, PvO2 = 40 mmHg | SOLVED | CO = 5.22139, CaO2 = 19.983, CvO2 = 15.195 |
| Fick cardiac output from saturations: Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 95 mmHg, PvO2 = 40 mmHg | NEEDS_INPUT | hp_fick_saturations needs VO2 (oxygen consumption (e.g. in mL/min)). Example: solve('hp_fick_saturations', VO2='4.16667e |
| Fick cardiac output from saturations: VO2 = 250 mL/min, Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 95 mmHg, PvO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Fick cardiac output from saturations: VO2 = 250 mL/min, Hb = 15, SaO2 = 0.75, SvO2 = 0.98, PaO2 = 95 mmHg, PvO2 = 40 mmHg | INVALID_INPUT | Fick cardiac output from hemoglobin, saturations and PO2: the venous oxygen content must be below the arterial. |

<a id="hp_mean_arterial_pressure"></a>

### Mean arterial pressure and pulse pressure: `hp_mean_arterial_pressure`

Pulse pressure PP = SBP - DBP and the estimated mean arterial pressure MAP = DBP + PP/3.

$$
MAP = DBP + \tfrac{1}{3}(SBP - DBP)
$$

| input | meaning | condition |
|---|---|---|
| `SBP` | systolic pressure (e.g. in mmHg) |  |
| `DBP` | diastolic pressure (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `MAP` | mean arterial pressure in mmHg |
| `PP` | pulse pressure in mmHg |

- **Method:** MAP = DBP + (SBP - DBP)/3
- **Assumes:** SBP > DBP > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![MAP (mean arterial pressure in mmHg) against SBP (systolic pressure (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-mean-arterial-pressure-curve)

*MAP (mean arterial pressure in mmHg) against SBP (systolic pressure (e.g. in mmHg)) from 7999.34 Pa to 23998 Pa. One curve for each value of DBP (diastolic pressure (e.g. in mmHg)): 10665.8 Pa, 5332.9 Pa. 25 points per curve computed by human-physiology-ai999x; marked: the worked example, SBP = 15998.7 Pa, MAP = 93.3333.*

![MAP (mean arterial pressure in mmHg) over SBP (systolic pressure (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-mean-arterial-pressure-field)

*MAP (mean arterial pressure in mmHg) over SBP (systolic pressure (e.g. in mmHg)) from 7999.34 Pa to 23998 Pa and DBP (diastolic pressure (e.g. in mmHg)) from 5332.9 Pa to 15998.7 Pa: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 46.6667 to dark 140); iso-lines at MAP = 60, 80, 100, 120; blank cells: inputs the capability refuses. Marked: the worked example, SBP = 15998.7 Pa, DBP = 10665.8 Pa, MAP = 93.3333.*

```python
human_physiology_ai999x.solve('hp_mean_arterial_pressure', SBP=15998.6864898, DBP=10665.7909932)
# -> MAP = 93.3333, PP = 40   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_mean_arterial_pressure', SBP=19998.35811225, DBP=11999.01486735)
# -> MAP = 110, PP = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| mean arterial pressure: SBP = 120 mmHg, DBP = 80 mmHg | SOLVED | MAP = 93.3333, PP = 40 |
| mean arterial pressure: SBP = 150 mmHg, DBP = 90 mmHg | SOLVED | MAP = 110, PP = 60 |
| mean arterial pressure: DBP = 80 mmHg | NEEDS_INPUT | hp_mean_arterial_pressure needs SBP (systolic pressure (e.g. in mmHg)). Example: solve('hp_mean_arterial_pressure', SBP= |
| mean arterial pressure: SBP = 120 mmHg, DBP = 80 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| mean arterial pressure: SBP = 80 mmHg, DBP = 120 mmHg | INVALID_INPUT | Mean arterial pressure and pulse pressure: the systolic pressure must exceed the diastolic. |

<a id="hp_systemic_vascular_resistance"></a>

### Systemic vascular resistance (total peripheral resistance), `hp_systemic_vascular_resistance`

SVR = (MAP - CVP)/CO in Wood units (mmHg min/L) and in dyn s $cm^{-5}$ (exact factor 79.9934; customary 80).

$$
SVR = \frac{MAP - CVP}{CO}
$$

| input | meaning | condition |
|---|---|---|
| `MAP` | mean arterial pressure (e.g. in mmHg) |  |
| `CVP` | central venous pressure (e.g. in mmHg) |  |
| `CO` | cardiac output (e.g. in L/min) |  |

| output | meaning |
|---|---|
| `SVR` | systemic vascular resistance in mmHg min/L (Wood units) |
| `SVR_dyn` | systemic vascular resistance in dyn s $cm^{-5}$ |

- **Method:** SVR = (MAP - CVP)/CO; x 79.9934 for dyn s $cm^{-5}$
- **Assumes:** MAP > CVP >= 0; $CO > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SVR (systemic vascular resistance in mmHg min/L (Wood units)) against CO (cardiac output (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-systemic-vascular-resistance-curve)

*SVR (systemic vascular resistance in mmHg min/L (Wood units)) against CO (cardiac output (e.g. in L/min)) from 4.1667e-05 m^3/s to 0.000125 m^3/s. One curve for each value of MAP (mean arterial pressure (e.g. in mmHg)): 12399 Pa, 6199.49 Pa, 24798 Pa. Other inputs as in the worked example: CVP = 399.96716224500005 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, CO = 8.3333e-05 m^3/s, SVR = 18.*

```python
human_physiology_ai999x.solve('hp_systemic_vascular_resistance', MAP=12398.982029595001, CVP=399.96716224500005, CO=8.333333333333334e-05)
# -> SVR = 18, SVR_dyn = 1439.88   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_systemic_vascular_resistance', MAP=13332.238741500001, CVP=0.0, CO=8.333333333333334e-05)
# -> SVR = 20, SVR_dyn = 1599.87   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| systemic vascular resistance: MAP = 93 mmHg, CVP = 3 mmHg, CO = 5 L/min | SOLVED | SVR = 18, SVR_dyn = 1439.88 |
| systemic vascular resistance: MAP = 100 mmHg, CVP = 0 mmHg, CO = 5 L/min | SOLVED | SVR = 20, SVR_dyn = 1599.87 |
| systemic vascular resistance: CVP = 3 mmHg, CO = 5 L/min | NEEDS_INPUT | hp_systemic_vascular_resistance needs MAP (mean arterial pressure (e.g. in mmHg)). Example: solve('hp_systemic_vascular_ |
| systemic vascular resistance: MAP = 93 mmHg, CVP = 3 mmHg, CO = 5 L/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| systemic vascular resistance: MAP = 93 mmHg, CVP = 3 mmHg, CO = 0 L/min | INVALID_INPUT | Systemic vascular resistance (total peripheral resistance): CO must be positive. |

<a id="hp_pulmonary_vascular_resistance"></a>

### Pulmonary vascular resistance and transpulmonary gradient, `hp_pulmonary_vascular_resistance`

Transpulmonary gradient TPG = mPAP - PCWP and PVR = TPG/CO in Wood units and dyn s $cm^{-5.}$

$$
PVR = \frac{\bar P_{PA} - P_{CW}}{CO}
$$

| input | meaning | condition |
|---|---|---|
| `mPAP` | mean pulmonary artery pressure (e.g. in mmHg) |  |
| `PCWP` | pulmonary capillary wedge pressure (e.g. in mmHg) |  |
| `CO` | cardiac output (e.g. in L/min) |  |

| output | meaning |
|---|---|
| `PVR` | pulmonary vascular resistance in Wood units |
| `PVR_dyn` | pulmonary vascular resistance in dyn s $cm^{-5}$ |
| `TPG` | transpulmonary gradient in mmHg |

- **Method:** PVR = (mPAP - PCWP)/CO
- **Assumes:** mPAP > PCWP >= 0; $CO > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PVR (pulmonary vascular resistance in Wood units) against CO (cardiac output (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-pulmonary-vascular-resistance-curve)

*PVR (pulmonary vascular resistance in Wood units) against CO (cardiac output (e.g. in L/min)) from 4.1667e-05 m^3/s to 0.000125 m^3/s. One curve for each value of mPAP (mean pulmonary artery pressure (e.g. in mmHg)): 1999.84 Pa, 3999.67 Pa. Other inputs as in the worked example: PCWP = 1066.57909932 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, CO = 8.3333e-05 m^3/s, PVR = 1.4.*

```python
human_physiology_ai999x.solve('hp_pulmonary_vascular_resistance', mPAP=1999.835811225, PCWP=1066.57909932, CO=8.333333333333334e-05)
# -> PVR = 1.4, PVR_dyn = 111.991, TPG = 7   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_pulmonary_vascular_resistance', mPAP=5332.8954966, PCWP=1333.22387415, CO=6.666666666666667e-05)
# -> PVR = 7.5, PVR_dyn = 599.951, TPG = 30   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| pulmonary vascular resistance: mPAP = 15 mmHg, PCWP = 8 mmHg, CO = 5 L/min | SOLVED | PVR = 1.4, PVR_dyn = 111.991, TPG = 7 |
| pulmonary vascular resistance: mPAP = 40 mmHg, PCWP = 10 mmHg, CO = 4 L/min | SOLVED | PVR = 7.5, PVR_dyn = 599.951, TPG = 30 |
| pulmonary vascular resistance: PCWP = 8 mmHg, CO = 5 L/min | NEEDS_INPUT | hp_pulmonary_vascular_resistance needs mPAP (mean pulmonary artery pressure (e.g. in mmHg)). Example: solve('hp_pulmonar |
| pulmonary vascular resistance: mPAP = 15 mmHg, PCWP = 8 mmHg, CO = 5 L/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| pulmonary vascular resistance: mPAP = 8 mmHg, PCWP = 15 mmHg, CO = 5 L/min | INVALID_INPUT | Pulmonary vascular resistance and transpulmonary gradient: the mean pulmonary artery pressure must exceed the wedge pres |

<a id="hp_poiseuille_resistance"></a>

### Poiseuille resistance of a blood vessel: `hp_poiseuille_resistance`

Resistance to laminar flow of a cylindrical vessel $R = 8$ eta L/(pi r^4), in Pa $s/m^{3}$ and in mmHg min/mL.

$$
R = \frac{8\eta L}{\pi r^4}
$$

| input | meaning | condition |
|---|---|---|
| `eta` | blood viscosity in cP (mPa s) |  |
| `L` | vessel length (e.g. in cm) |  |
| `r` | vessel radius (e.g. in mm) |  |

| output | meaning |
|---|---|
| `R` | resistance in Pa $s/m^{3}$ |
| `R_mmHg` | resistance in mmHg min/mL |

- **Method:** $R = 8$ eta L/(pi r^4)
- **Assumes:** eta, L, $r > 0$; steady laminar Newtonian flow in a rigid tube
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![R (resistance in Pa s/m^3) against r (vessel radius (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-poiseuille-resistance-curve)

*R (resistance in Pa s/m^3) against r (vessel radius (e.g. in mm)) from 0.001 m to 0.003 m. One curve for each value of eta (blood viscosity in cP (mPa s)): 3.5, 1.75, 7. Other inputs as in the worked example: L = 0.1 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, r = 0.002 m, R = 5.5704e+07.*

```python
human_physiology_ai999x.solve('hp_poiseuille_resistance', eta=3.5, L=0.1, r=0.002)
# -> R = 5.57042e+07, R_mmHg = 0.0069636   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_poiseuille_resistance', eta=4, L=0.05, r=0.0015)
# -> R = 1.00602e+08, R_mmHg = 0.0125762   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Poiseuille resistance: eta = 3.5, L = 10 cm, r = 2 mm | SOLVED | R = 5.57042e+07, R_mmHg = 0.0069636 |
| Poiseuille resistance: eta = 4, L = 5 cm, r = 1.5 mm | SOLVED | R = 1.00602e+08, R_mmHg = 0.0125762 |
| Poiseuille resistance: L = 10 cm, r = 2 mm | NEEDS_INPUT | hp_poiseuille_resistance needs eta (blood viscosity in cP (mPa s)). Example: solve('hp_poiseuille_resistance', eta=3.5,  |
| Poiseuille resistance: eta = 3.5, L = 10 cm, r = 2 mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Poiseuille resistance: eta = 3.5, L = 10 cm, r = 0 mm | INVALID_INPUT | Poiseuille resistance of a blood vessel: r must be positive. |

<a id="hp_poiseuille_flow"></a>

### Poiseuille flow, mean velocity and wall shear stress in a vessel, `hp_poiseuille_flow`

Laminar flow $Q = dP$ pi r^4/(8 eta L) through a vessel, its mean velocity and the wall shear stress 4 eta Q/(pi r^3).

$$
Q = \frac{\pi r^4 \Delta P}{8 \eta L},\quad \tau_w = \frac{4\eta Q}{\pi r^3}
$$

| input | meaning | condition |
|---|---|---|
| `dP` | pressure drop along the vessel (e.g. in mmHg) |  |
| `eta` | blood viscosity in cP (mPa s) |  |
| `L` | vessel length (e.g. in cm) |  |
| `r` | vessel radius (e.g. in mm) |  |

| output | meaning |
|---|---|
| `Q` | flow in mL/s |
| `Q_mL_min` | flow in mL/min |
| `v_mean` | mean velocity in cm/s |
| `tau_wall` | wall shear stress in Pa |

- **Method:** $Q = dP$ pi r^4 /(8 eta L); $tau_w = 4$ eta Q/(pi r^3)
- **Assumes:** all inputs > 0; steady laminar Newtonian flow
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Q (flow in mL/s) against r (vessel radius (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-poiseuille-flow-curve)

*Q (flow in mL/s) against r (vessel radius (e.g. in mm)) from 0.001 m to 0.003 m. One curve for each value of dP (pressure drop along the vessel (e.g. in mmHg)): 1333.22 Pa, 666.612 Pa, 2666.45 Pa. Other inputs as in the worked example: eta = 3.5, L = 0.1 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, r = 0.002 m, Q = 23.934.*

```python
human_physiology_ai999x.solve('hp_poiseuille_flow', dP=1333.22387415, eta=3.5, L=0.1, r=0.002)
# -> Q = 23.934, Q_mL_min = 1436.04, v_mean = 190.461, tau_wall = 13.3322   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_poiseuille_flow', dP=666.611937075, eta=3, L=0.02, r=0.001)
# -> Q = 4.36296, Q_mL_min = 261.778, v_mean = 138.877, tau_wall = 16.6653   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Poiseuille flow: dP = 10 mmHg, eta = 3.5, L = 10 cm, r = 2 mm | SOLVED | Q = 23.934, Q_mL_min = 1436.04, v_mean = 190.461, tau_wall = 13.3322 |
| Poiseuille flow: dP = 5 mmHg, eta = 3, L = 2 cm, r = 1 mm | SOLVED | Q = 4.36296, Q_mL_min = 261.778, v_mean = 138.877, tau_wall = 16.6653 |
| Poiseuille flow: eta = 3.5, L = 10 cm, r = 2 mm | NEEDS_INPUT | hp_poiseuille_flow needs dP (pressure drop along the vessel (e.g. in mmHg)). Example: solve('hp_poiseuille_flow', dP='13 |
| Poiseuille flow: dP = 10 mmHg, eta = 3.5, L = 10 cm, r = 2 mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Poiseuille flow: dP = 10 mmHg, eta = -3.5, L = 10 cm, r = 2 mm | INVALID_INPUT | Poiseuille flow, mean velocity and wall shear stress in a vessel: eta must be positive. |

<a id="hp_radius_flow_ratio"></a>

### Effect of a change of vessel radius on flow and resistance, `hp_radius_flow_ratio`

At the same pressure drop, flow scales as $r^{4}$ and resistance as $1/r^{4}$ (Poiseuille).

$$
\frac{Q_2}{Q_1} = \left(\frac{r_2}{r_1}\right)^4
$$

| input | meaning | condition |
|---|---|---|
| `r1` | initial radius (any unit) |  |
| `r2` | new radius (same unit) |  |

| output | meaning |
|---|---|
| `flow_ratio` | new flow / initial flow |
| `resistance_ratio` | new resistance / initial resistance |

- **Method:** $Q_{2}/Q_{1} = (r_{2}/r_{1})^{4}$
- **Assumes:** r1, $r_{2} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![flow_ratio (new flow / initial flow) against r1 (initial radius (any unit)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-radius-flow-ratio-curve)

*flow_ratio (new flow / initial flow) against r1 (initial radius (any unit)) from 0.5 to 1.5. One curve for each value of r2 (new radius (same unit)): 2, 1, 4. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, r1 = 1, flow_ratio = 16.*

![flow_ratio (new flow / initial flow) over r1 (initial radius (any unit)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-radius-flow-ratio-field)

*flow_ratio (new flow / initial flow) over r1 (initial radius (any unit)) from 0.5 to 1.5 and r2 (new radius (same unit)) from 1 to 3: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.197531 to dark 1296, logarithmic shading); iso-lines at flow_ratio = 1, 10, 100, 1000. Marked: the worked example, r1 = 1, r2 = 2, flow_ratio = 16.*

```python
human_physiology_ai999x.solve('hp_radius_flow_ratio', r1=1, r2=2)
# -> flow_ratio = 16, resistance_ratio = 0.0625   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_radius_flow_ratio', r1=1, r2=0.8)
# -> flow_ratio = 0.4096, resistance_ratio = 2.44141   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| radius change and flow: r1 = 1, r2 = 2 | SOLVED | flow_ratio = 16, resistance_ratio = 0.0625 |
| radius change and flow: r1 = 1, r2 = 0.8 | SOLVED | flow_ratio = 0.4096, resistance_ratio = 2.44141 |
| radius change and flow: r2 = 2 | NEEDS_INPUT | hp_radius_flow_ratio needs r1 (initial radius (any unit)). Example: solve('hp_radius_flow_ratio', r1=1, r2=2). |
| radius change and flow: r1 = 1, r2 = 2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| radius change and flow: r1 = 1, r2 = 0 | INVALID_INPUT | Effect of a change of vessel radius on flow and resistance: r2 must be positive. |

<a id="hp_vessels_series_parallel"></a>

### Total resistance of vascular beds in series or in parallel, `hp_vessels_series_parallel`

Series beds add resistances; parallel beds (organ circulations) add conductances.

$$
R_s = \sum R_i,\quad \frac{1}{R_p} = \sum \frac{1}{R_i}
$$

| input | meaning | condition |
|---|---|---|
| `R` | resistances of the beds (any one unit, e.g. mmHg min/L) |  |
| `arrangement` | series or parallel |  |

| output | meaning |
|---|---|
| `R_total` | total resistance (same unit) |
| `G_total` | total conductance (reciprocal of the resistance unit) |

- **Method:** series: sum R; parallel: 1/sum(1/R)
- **Assumes:** every resistance > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![R_total (total resistance (same unit)) for each option of arrangement (series or parallel), one run of human-physiology-ai999x per option: series: 11, parallel: 1](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-vessels-series-parallel-bars)

*R_total (total resistance (same unit)) for each option of arrangement (series or parallel), one run of human-physiology-ai999x per option: series: 11, parallel: 1. The worked example uses parallel. Other inputs as in the worked example: R = [2, 3, 6].*

```python
human_physiology_ai999x.solve('hp_vessels_series_parallel', R=[2, 3, 6], arrangement='parallel')
# -> R_total = 1, G_total = 1   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_vessels_series_parallel', R=[2, 3, 6], arrangement='series')
# -> R_total = 11, G_total = 0.0909091   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| vessels in series or parallel: R = [2, 3, 6], arrangement = parallel | SOLVED | R_total = 1, G_total = 1 |
| vessels in series or parallel: R = [2, 3, 6], arrangement = series | SOLVED | R_total = 11, G_total = 0.0909091 |
| vessels in series or parallel: arrangement = parallel | NEEDS_INPUT | hp_vessels_series_parallel needs R (resistances of the beds (any one unit, e.g. mmHg min/L)). Example: solve('hp_vessels |
| vessels in series or parallel: R = [2, 3, 6], arrangement = parallel, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| vessels in series or parallel: R = [2, -3], arrangement = series | INVALID_INPUT | Total resistance of vascular beds in series or in parallel: every resistance must be positive. |

<a id="hp_vessel_reynolds"></a>

### Reynolds number of blood flow in a vessel: `hp_vessel_reynolds`

$Re = \rho$ v D/eta and the mean velocity at which Re reaches 2000 (the customary laminar limit).

$$
Re = \frac{\rho v D}{\eta}
$$

| input | meaning | condition |
|---|---|---|
| `rho` | blood density (e.g. in kg/m^3) |  |
| `v` | mean velocity (e.g. in cm/s) |  |
| `D` | vessel diameter (e.g. in mm) |  |
| `eta` | viscosity in cP |  |

| output | meaning |
|---|---|
| `Re` | Reynolds number |
| `v_critical` | mean velocity at the customary laminar-limit Reynolds number in cm/s |

- **Method:** $Re = \rho$ v D / eta
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Re (Reynolds number) against eta (viscosity in cP) from 1](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-vessel-reynolds-curve)

*Re (Reynolds number) against eta (viscosity in cP) from 1.75 to 5.25. One curve for each value of rho (blood density (e.g. in kg/m^3)): 1060 kg/m^3, 530 kg/m^3, 2120 kg/m^3. Other inputs as in the worked example: v = 0.3 m/s, D = 0.025 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, eta = 3.5, Re = 2271.43.*

```python
human_physiology_ai999x.solve('hp_vessel_reynolds', rho=1060.0, v=0.3, D=0.025, eta=3.5)
# -> Re = 2271.43, v_critical = 26.4151   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_vessel_reynolds', rho=1050.0, v=0.05, D=0.004, eta=4)
# -> Re = 52.5, v_critical = 190.476   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| vessel Reynolds number: rho = 1060 kg/m^3, v = 30 cm/s, D = 25 mm, eta = 3.5 | SOLVED | Re = 2271.43, v_critical = 26.4151 |
| vessel Reynolds number: rho = 1050 kg/m^3, v = 5 cm/s, D = 4 mm, eta = 4 | SOLVED | Re = 52.5, v_critical = 190.476 |
| vessel Reynolds number: v = 30 cm/s, D = 25 mm, eta = 3.5 | NEEDS_INPUT | hp_vessel_reynolds needs rho (blood density (e.g. in kg/m^3)). Example: solve('hp_vessel_reynolds', rho='1060 kg/m^3', v |
| vessel Reynolds number: rho = 1060 kg/m^3, v = 30 cm/s, D = 25 mm, eta = 3.5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| vessel Reynolds number: rho = 1060 kg/m^3, v = 30 cm/s, D = 0 mm, eta = 3.5 | INVALID_INPUT | Reynolds number of blood flow in a vessel: D must be positive. |

<a id="hp_laplace_wall_tension"></a>

### Laplace wall tension and wall stress of a vessel or chamber, `hp_laplace_wall_tension`

Law of Laplace: wall tension $T = P$ r (cylinder) or P $r/2$ (sphere) and wall stress T/h.

$$
T = P r \;(\text{cylinder}),\quad T = \tfrac{P r}{2} \;(\text{sphere}),\quad \sigma = T/h
$$

| input | meaning | condition |
|---|---|---|
| `P` | transmural pressure (e.g. in mmHg) |  |
| `r` | radius (e.g. in cm) |  |
| `shape` | cylinder (vessel) or sphere (chamber, alveolus) |  |
| `h` | wall thickness (e.g. in mm) (optional; needed for the wall stress) |  |

| output | meaning |
|---|---|
| `T` | wall tension in N/m |
| `stress` | wall stress in kPa |

- **Method:** $T = P$ r (cylinder), P $r/2$ (sphere); stress = T/h
- **Assumes:** P, r, $h > 0$; thin wall
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![T (wall tension in N/m) against P (transmural pressure (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-laplace-wall-tension-curve)

*T (wall tension in N/m) against P (transmural pressure (e.g. in mmHg)) from 6666.12 Pa to 19998.4 Pa. One curve for each value of r (radius (e.g. in cm)): 0.015 m, 0.0075 m, 0.03 m. Other inputs as in the worked example: shape = cylinder, h = 0.002 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, P = 13332.2 Pa, T = 199.984.*

```python
human_physiology_ai999x.solve('hp_laplace_wall_tension', P=13332.238741500001, r=0.015, shape='cylinder', h=0.002)
# -> T = 199.984, stress = 99.9918   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_laplace_wall_tension', P=2666.4477483, r=0.05, shape='sphere', h=0.001)
# -> T = 66.6612, stress = 66.6612   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_laplace_wall_tension', P=13332.238741500001, r=0.015, shape='cylinder')
# -> T = 199.984   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Laplace wall tension: P = 100 mmHg, r = 1.5 cm, shape = cylinder, h = 2 mm | SOLVED | T = 199.984, stress = 99.9918 |
| Laplace wall tension: P = 20 mmHg, r = 5 cm, shape = sphere, h = 1 mm | SOLVED | T = 66.6612, stress = 66.6612 |
| Laplace wall tension: P = 100 mmHg, r = 1.5 cm, shape = cylinder | SOLVED | T = 199.984 |
| Laplace wall tension: r = 1.5 cm, shape = cylinder, h = 2 mm | NEEDS_INPUT | hp_laplace_wall_tension needs P (transmural pressure (e.g. in mmHg)). Example: solve('hp_laplace_wall_tension', P='13332 |
| Laplace wall tension: P = 100 mmHg, r = 1.5 cm, shape = cylinder, h = 2 mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Laplace wall tension: P = 100 mmHg, r = 0 cm, shape = cylinder, h = 2 mm | INVALID_INPUT | Laplace wall tension and wall stress of a vessel or chamber: r must be positive. |

<a id="hp_vascular_compliance"></a>

### Compliance and elastance: `hp_vascular_compliance`

Compliance $C = dV/dP$ and its inverse, the elastance $E = dP/dV.$

$$
C = \frac{\Delta V}{\Delta P}
$$

| input | meaning | condition |
|---|---|---|
| `dV` | volume change (e.g. in mL) |  |
| `dP` | pressure change (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `C` | compliance in mL/mmHg |
| `E` | elastance in mmHg/mL |

- **Method:** $C = dV/dP$
- **Assumes:** dV, $dP > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![C (compliance in mL/mmHg) against dP (pressure change (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-vascular-compliance-curve)

*C (compliance in mL/mmHg) against dP (pressure change (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa. One curve for each value of dV (volume change (e.g. in mL)): 7.0000e-05 m^3, 3.5000e-05 m^3, 0.00014 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, dP = 5332.9 Pa, C = 1.75.*

![C (compliance in mL/mmHg) over dV (volume change (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-vascular-compliance-field)

*C (compliance in mL/mmHg) over dV (volume change (e.g. in mL)) from 3.5000e-05 m^3 to 0.000105 m^3 and dP (pressure change (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.583333 to dark 5.25); iso-lines at C = 1, 2, 3, 4, 5. Marked: the worked example, dV = 7.0000e-05 m^3, dP = 5332.9 Pa, C = 1.75.*

```python
human_physiology_ai999x.solve('hp_vascular_compliance', dV=7e-05, dP=5332.8954966)
# -> C = 1.75, E = 0.571429   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_vascular_compliance', dV=4.9999999999999996e-05, dP=3333.0596853750003)
# -> C = 2, E = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| vascular compliance: dV = 70 mL, dP = 40 mmHg | SOLVED | C = 1.75, E = 0.571429 |
| vascular compliance: dV = 50 mL, dP = 25 mmHg | SOLVED | C = 2, E = 0.5 |
| vascular compliance: dP = 40 mmHg | NEEDS_INPUT | hp_vascular_compliance needs dV (volume change (e.g. in mL)). Example: solve('hp_vascular_compliance', dV='7e-05 m^3', d |
| vascular compliance: dV = 70 mL, dP = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| vascular compliance: dV = 70 mL, dP = 0 mmHg | INVALID_INPUT | Compliance and elastance: dP must be positive. |

<a id="hp_blood_velocity_area"></a>

### Mean blood velocity from flow and total cross-sectional area, `hp_blood_velocity_area`

Continuity: mean velocity $v = Q/A$ (slow in the capillaries because their total area is large).

$$
v = \frac{Q}{A}
$$

| input | meaning | condition |
|---|---|---|
| `Q` | flow (e.g. in L/min) |  |
| `A` | total cross-sectional area (e.g. in cm^2) |  |

| output | meaning |
|---|---|
| `v` | mean velocity in cm/s |

- **Method:** $v = Q/A$ (L/min -> cm^3/s)
- **Assumes:** Q, $A > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![v (mean velocity in cm/s) against A (total cross-sectional area (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-blood-velocity-area-curve)

*v (mean velocity in cm/s) against A (total cross-sectional area (e.g. in cm^2)) from 0.0002 m^2 to 0.0006 m^2. One curve for each value of Q (flow (e.g. in L/min)): 8.3333e-05 m^3/s, 4.1667e-05 m^3/s, 0.000166667 m^3/s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, A = 0.0004 m^2, v = 20.8333.*

![v (mean velocity in cm/s) over Q (flow (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-blood-velocity-area-field)

*v (mean velocity in cm/s) over Q (flow (e.g. in L/min)) from 4.1667e-05 m^3/s to 0.000125 m^3/s and A (total cross-sectional area (e.g. in cm^2)) from 0.0002 m^2 to 0.0006 m^2: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 6.94444 to dark 62.5); iso-lines at v = 10, 20, 30, 40, 50, 60. Marked: the worked example, Q = 8.3333e-05 m^3/s, A = 0.0004 m^2, v = 20.8333.*

```python
human_physiology_ai999x.solve('hp_blood_velocity_area', Q=8.333333333333334e-05, A=0.0004)
# -> v = 20.8333   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_blood_velocity_area', Q=8.333333333333334e-05, A=0.3)
# -> v = 0.0277778   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| mean blood velocity: Q = 5 L/min, A = 4 cm^2 | SOLVED | v = 20.8333 |
| mean blood velocity: Q = 5 L/min, A = 3000 cm^2 | SOLVED | v = 0.0277778 |
| mean blood velocity: A = 4 cm^2 | NEEDS_INPUT | hp_blood_velocity_area needs Q (flow (e.g. in L/min)). Example: solve('hp_blood_velocity_area', Q='8.33333e-05 m^3/s', A |
| mean blood velocity: Q = 5 L/min, A = 4 cm^2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| mean blood velocity: Q = 5 L/min, A = 0 cm^2 | INVALID_INPUT | Mean blood velocity from flow and total cross-sectional area: A must be positive. |

<a id="hp_bernoulli_pressure_drop"></a>

### Bernoulli pressure drop across a narrowing, `hp_bernoulli_pressure_drop`

Convective pressure drop 0.5 rho $(v_{2}^{2} - v_{1}^{2})$ across a stenosis or valve, and the simplified 4 $v^{2}$ (mmHg, v in m/s).

$$
\Delta P = \tfrac{1}{2}\rho\,(v_2^2 - v_1^2) \approx 4 v_2^2
$$

| input | meaning | condition |
|---|---|---|
| `v1` | upstream velocity (e.g. in m/s) |  |
| `v2` | jet velocity (e.g. in m/s) |  |
| `rho` | blood density (e.g. in kg/m^3) |  |

| output | meaning |
|---|---|
| `dP` | pressure drop in Pa |
| `dP_mmHg` | pressure drop in mmHg |
| `dP_simplified` | pressure drop by the simplified Bernoulli equation in mmHg |

- **Method:** $dP = \rho$ $(v_{2}^{2} - v_{1}^{2})/2$
- **Assumes:** $v_{2} > v_{1}  \geq  0$; inviscid steady flow
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dP_mmHg (pressure drop in mmHg) against v1 (upstream velocity (e](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-bernoulli-pressure-drop-dp-mmhg-vs-v1.svg)

*dP_mmHg (pressure drop in mmHg) against v1 (upstream velocity (e.g. in m/s)), with rho = 1060 kg/m^3, v2 = 4 m/s. 31 points computed by human-physiology-ai999x; marked: v1 = 1, dP_mmHg = 59.6299.*

![dP_mmHg (pressure drop in mmHg) against v1 (upstream velocity (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bernoulli-pressure-drop-curve)

*dP_mmHg (pressure drop in mmHg) against v1 (upstream velocity (e.g. in m/s)) from 0.1 m/s to 10 m/s on a logarithmic axis. One curve for each value of v2 (jet velocity (e.g. in m/s)): 4 m/s, 2 m/s, 8 m/s. Other inputs as in the worked example: rho = 1060.0 kg/m^3. 33 points per curve computed by human-physiology-ai999x; marked: the worked example, v1 = 1 m/s, dP_mmHg = 59.6299.*

```python
human_physiology_ai999x.solve('hp_bernoulli_pressure_drop', v1=1.0, v2=4.0, rho=1060.0)
# -> dP = 7950, dP_mmHg = 59.6299, dP_simplified = 64   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bernoulli_pressure_drop', v1=0.5, v2=3.0, rho=1000.0)
# -> dP = 4375, dP_mmHg = 32.8152, dP_simplified = 36   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bernoulli pressure drop: v1 = 1 m/s, v2 = 4 m/s, rho = 1060 kg/m^3 | SOLVED | dP = 7950, dP_mmHg = 59.6299, dP_simplified = 64 |
| Bernoulli pressure drop: v1 = 0.5 m/s, v2 = 3 m/s, rho = 1000 kg/m^3 | SOLVED | dP = 4375, dP_mmHg = 32.8152, dP_simplified = 36 |
| Bernoulli pressure drop: v2 = 4 m/s, rho = 1060 kg/m^3 | NEEDS_INPUT | hp_bernoulli_pressure_drop needs v1 (upstream velocity (e.g. in m/s)). Example: solve('hp_bernoulli_pressure_drop', v1=' |
| Bernoulli pressure drop: v1 = 1 m/s, v2 = 4 m/s, rho = 1060 kg/m^3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Bernoulli pressure drop: v1 = 4 m/s, v2 = 1 m/s, rho = 1060 kg/m^3 | INVALID_INPUT | Bernoulli pressure drop across a narrowing: the downstream (jet) velocity v2 must exceed v1. |

<a id="hp_stroke_work"></a>

### Stroke work and cardiac power: `hp_stroke_work`

External stroke work SW = MAP x SV (J and g m) and the cardiac power output SW x HR.

$$
SW = \bar P\, SV,\quad \dot W = SW \cdot HR
$$

| input | meaning | condition |
|---|---|---|
| `SV` | stroke volume (e.g. in mL) |  |
| `MAP` | mean arterial (ejection) pressure (e.g. in mmHg) |  |
| `HR` | heart rate in beats/min (optional; needed for the cardiac power) |  |

| output | meaning |
|---|---|
| `SW` | stroke work in J |
| `SW_gm` | stroke work in g m |
| `power` | cardiac power output in W |

- **Method:** SW = MAP SV (pressure-volume work); power = SW $HR/60$
- **Assumes:** SV, MAP, $HR > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SW (stroke work in J) against SV (stroke volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-stroke-work-curve)

*SW (stroke work in J) against SV (stroke volume (e.g. in mL)) from 3.5000e-05 m^3 to 0.000105 m^3. One curve for each value of MAP (mean arterial (ejection) pressure (e.g. in mmHg)): 13332.2 Pa, 6666.12 Pa, 26664.5 Pa. Other inputs as in the worked example: HR = 72.0. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, SV = 7.0000e-05 m^3, SW = 0.933257.*

```python
human_physiology_ai999x.solve('hp_stroke_work', SV=7e-05, MAP=13332.238741500001, HR=72)
# -> SW = 0.933257, SW_gm = 95.1657, power = 1.11991   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_stroke_work', SV=7.999999999999999e-05, MAP=11999.01486735, HR=60)
# -> SW = 0.959921, SW_gm = 97.8847, power = 0.959921   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_stroke_work', SV=7e-05, MAP=13332.238741500001)
# -> SW = 0.933257, SW_gm = 95.1657   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| stroke work: SV = 70 mL, MAP = 100 mmHg, HR = 72 | SOLVED | SW = 0.933257, SW_gm = 95.1657, power = 1.11991 |
| stroke work: SV = 80 mL, MAP = 90 mmHg, HR = 60 | SOLVED | SW = 0.959921, SW_gm = 97.8847, power = 0.959921 |
| stroke work: SV = 70 mL, MAP = 100 mmHg | SOLVED | SW = 0.933257, SW_gm = 95.1657 |
| stroke work: MAP = 100 mmHg, HR = 72 | NEEDS_INPUT | hp_stroke_work needs SV (stroke volume (e.g. in mL)). Example: solve('hp_stroke_work', SV='7e-05 m^3', MAP='13332.2 Pa', |
| stroke work: SV = 70 mL, MAP = 100 mmHg, HR = 72, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| stroke work: SV = 70 mL, MAP = 0 mmHg, HR = 72 | INVALID_INPUT | Stroke work and cardiac power: MAP must be positive. |

<a id="hp_teichholz_volumes"></a>

### Left-ventricular volumes and ejection fraction by the Teichholz formula, `hp_teichholz_volumes`

Teichholz volume $V = 7$ $D^{3}/(2.4 + D)$ from M-mode internal diameters, EF and fractional shortening.

$$
V = \frac{7 D^3}{2.4 + D}
$$

| input | meaning | condition |
|---|---|---|
| `LVIDd` | end-diastolic internal diameter (e.g. in cm) |  |
| `LVIDs` | end-systolic internal diameter (e.g. in cm) |  |

| output | meaning |
|---|---|
| `EDV` | end-diastolic volume in mL |
| `ESV` | end-systolic volume in mL |
| `EF` | ejection fraction |
| `FS` | fractional shortening |

- **Method:** $V = 7$ $D^{3}/(2.4 + D)$ (D in cm, V in mL)
- **Assumes:** 0 < LVIDs < LVIDd
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![EF (ejection fraction) against LVIDd (end-diastolic internal diameter (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-teichholz-volumes-curve)

*EF (ejection fraction) against LVIDd (end-diastolic internal diameter (e.g. in cm)) from 0.025 m to 0.075 m. One curve for each value of LVIDs (end-systolic internal diameter (e.g. in cm)): 0.032 m, 0.016 m. 26 points per curve computed by human-physiology-ai999x; marked: the worked example, LVIDd = 0.05 m, EF = 0.653595.*

![EF (ejection fraction) over LVIDs (end-systolic internal diameter (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-teichholz-volumes-field)

*EF (ejection fraction) over LVIDs (end-systolic internal diameter (e.g. in cm)) from 0.016 m to 0.048 m and LVIDd (end-diastolic internal diameter (e.g. in cm)) from 0.025 m to 0.075 m: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.00143992 to dark 0.97597); iso-lines at EF = 0.2, 0.4, 0.6, 0.8; blank cells: inputs the capability refuses. Marked: the worked example, LVIDs = 0.032 m, LVIDd = 0.05 m, EF = 0.653595.*

```python
human_physiology_ai999x.solve('hp_teichholz_volumes', LVIDd=0.05, LVIDs=0.032)
# -> EDV = 118.243, ESV = 40.96, EF = 0.653595, FS = 0.36   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_teichholz_volumes', LVIDd=0.055, LVIDs=0.045)
# -> EDV = 147.421, ESV = 92.4457, EF = 0.372913, FS = 0.181818   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Teichholz volumes: LVIDd = 5 cm, LVIDs = 3.2 cm | SOLVED | EDV = 118.243, ESV = 40.96, EF = 0.653595, FS = 0.36 |
| Teichholz volumes: LVIDd = 5.5 cm, LVIDs = 4.5 cm | SOLVED | EDV = 147.421, ESV = 92.4457, EF = 0.372913, FS = 0.181818 |
| Teichholz volumes: LVIDs = 3.2 cm | NEEDS_INPUT | hp_teichholz_volumes needs LVIDd (end-diastolic internal diameter (e.g. in cm)). Example: solve('hp_teichholz_volumes',  |
| Teichholz volumes: LVIDd = 5 cm, LVIDs = 3.2 cm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Teichholz volumes: LVIDd = 3 cm, LVIDs = 5 cm | INVALID_INPUT | Left-ventricular volumes and ejection fraction by the Teichholz formula: the end-systolic diameter must be smaller than  |

<a id="hp_qt_correction"></a>

### Heart-rate corrected QT interval (Bazett, Fridericia, Framingham), `hp_qt_correction`

$RR = 60/HR$ and the corrected QT: Bazett QT/sqrt(RR), Fridericia $QT/RR^{1/3}$, Framingham $QT + 154$ $(1 - RR)$.

$$
QT_c = \frac{QT}{\sqrt{RR}} \;(\text{Bazett}),\quad \frac{QT}{RR^{1/3}} \;(\text{Fridericia})
$$

| input | meaning | condition |
|---|---|---|
| `QT` | measured QT interval (e.g. in ms) |  |
| `HR` | heart rate in beats/min |  |

| output | meaning |
|---|---|
| `RR` | RR interval in s |
| `QTc_Bazett` | Bazett QTc in ms |
| `QTc_Fridericia` | Fridericia QTc in ms |
| `QTc_Framingham` | Framingham QTc in ms |

- **Method:** RR in seconds; the three published corrections
- **Assumes:** QT, $HR > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![QTc_Bazett (Bazett QTc in ms) against HR (heart rate in beats/min) from 37](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-qt-correction-curve)

*QTc_Bazett (Bazett QTc in ms) against HR (heart rate in beats/min) from 37.5 to 112.5. One curve for each value of QT (measured QT interval (e.g. in ms)): 0.4 s, 0.2 s, 0.8 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, HR = 75, QTc_Bazett = 447.214.*

![QTc_Bazett (Bazett QTc in ms) over QT (measured QT interval (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-qt-correction-field)

*QTc_Bazett (Bazett QTc in ms) over QT (measured QT interval (e.g. in ms)) from 0.2 s to 0.6 s and HR (heart rate in beats/min) from 37.5 to 112.5: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 158.114 to dark 821.584); iso-lines at QTc_Bazett = 200, 400, 600, 800. Marked: the worked example, QT = 0.4 s, HR = 75, QTc_Bazett = 447.214.*

```python
human_physiology_ai999x.solve('hp_qt_correction', QT=0.4, HR=75)
# -> RR = 0.8, QTc_Bazett = 447.214, QTc_Fridericia = 430.887, QTc_Framingham = 430.8   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_qt_correction', QT=0.36, HR=100)
# -> RR = 0.6, QTc_Bazett = 464.758, QTc_Fridericia = 426.827, QTc_Framingham = 421.6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| corrected QT interval: QT = 400 ms, HR = 75 | SOLVED | RR = 0.8, QTc_Bazett = 447.214, QTc_Fridericia = 430.887, QTc_Framingham = 430.8 |
| corrected QT interval: QT = 360 ms, HR = 100 | SOLVED | RR = 0.6, QTc_Bazett = 464.758, QTc_Fridericia = 426.827, QTc_Framingham = 421.6 |
| corrected QT interval: HR = 75 | NEEDS_INPUT | hp_qt_correction needs QT (measured QT interval (e.g. in ms)). Example: solve('hp_qt_correction', QT='0.4 s', HR=75). |
| corrected QT interval: QT = 400 ms, HR = 75, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| corrected QT interval: QT = 400 ms, HR = 0 | INVALID_INPUT | Heart-rate corrected QT interval (Bazett, Fridericia, Framingham): HR must be positive. |

<a id="hp_qrs_axis"></a>

### Mean QRS (electrical) axis from leads I and aVF, `hp_qrs_axis`

The frontal-plane mean electrical axis atan2(aVF, I) in degrees (hexaxial convention: lead I at 0 deg, aVF $at +90$ deg).

$$
\theta = \operatorname{atan2}(aVF, I)
$$

| input | meaning | condition |
|---|---|---|
| `lead_I` | net QRS deflection in lead I (mm or mV) |  |
| `aVF` | net QRS deflection in aVF (same unit) |  |

| output | meaning |
|---|---|
| `axis` | mean QRS axis in degrees (principal value) |
| `amplitude` | length of the mean vector (same unit) |

- **Method:** axis = atan2(aVF, I)
- **Assumes:** not both zero
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![axis (mean QRS axis in degrees (principal value)) against lead_I (net QRS deflection in lead I (mm or mV)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-qrs-axis-curve)

*axis (mean QRS axis in degrees (principal value)) against lead_I (net QRS deflection in lead I (mm or mV)) from 0.05 to 5 on a logarithmic axis. One curve for each value of aVF (net QRS deflection in aVF (same unit)): -0.5, -0.25, -1. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, lead_I = 0.5, axis = -45.*

![axis (mean QRS axis in degrees (principal value)) over lead_I (net QRS deflection in lead I (mm or mV)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-qrs-axis-field)

*axis (mean QRS axis in degrees (principal value)) over lead_I (net QRS deflection in lead I (mm or mV)) from 0.05 to 5 and aVF (net QRS deflection in aVF (same unit)) from -0.75 to -0.25 (logarithmic horizontal axis): each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light -86.1859 to dark -2.86241); iso-lines at axis = -80, -60, -40, -20. Marked: the worked example, lead_I = 0.5, aVF = -0.5, axis = -45.*

```python
human_physiology_ai999x.solve('hp_qrs_axis', lead_I=0.5, aVF=-0.5)
# -> axis = -45, amplitude = 0.707107   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_qrs_axis', lead_I=-0.2, aVF=0.8)
# -> axis = 104.036, amplitude = 0.824621   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| QRS axis: lead_I = 0.5, aVF = -0.5 | SOLVED | axis = -45, amplitude = 0.707107 |
| QRS axis: lead_I = -0.2, aVF = 0.8 | SOLVED | axis = 104.036, amplitude = 0.824621 |
| QRS axis: aVF = -0.5 | NEEDS_INPUT | hp_qrs_axis needs lead_I (net QRS deflection in lead I (mm or mV)). Example: solve('hp_qrs_axis', lead_I=0.5, aVF=-0.5). |
| QRS axis: lead_I = 0.5, aVF = -0.5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| QRS axis: lead_I = 0, aVF = 0 | INVALID_INPUT | Mean QRS (electrical) axis from leads I and aVF: at least one net deflection must be nonzero. |

<a id="hp_starling_filtration"></a>

### Starling forces and capillary filtration: `hp_starling_filtration`

Net filtration pressure (Pc - Pi) - sigma (pi_c - pi_i) and the filtration rate $Jv = Kf$ x NFP (glomerular or systemic capillaries).

$$
J_v = K_f\left[(P_c - P_i) - \sigma(\pi_c - \pi_i)\right]
$$

| input | meaning | condition |
|---|---|---|
| `Pc` | capillary hydrostatic pressure (e.g. in mmHg) |  |
| `Pi` | interstitial (or Bowman's space) hydrostatic pressure (e.g. in mmHg) |  |
| `pic` | capillary oncotic pressure (e.g. in mmHg) |  |
| `pii` | interstitial oncotic pressure (e.g. in mmHg) |  |
| `Kf` | filtration coefficient in mL/min per mmHg |  |
| `sigma` | reflection coefficient (optional; ideal membrane if omitted) |  |

| output | meaning |
|---|---|
| `NFP` | net filtration pressure in mmHg (negative = absorption) |
| `Jv` | filtration rate in mL/min |

- **Method:** $Jv = Kf$ [(Pc - Pi) - sigma (pi_c - pi_i)]
- **Assumes:** $Kf > 0$; $0  \leq  \sigma  \leq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Jv (filtration rate in mL/min) against Pc (capillary hydrostatic pressure (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-starling-filtration-curve)

*Jv (filtration rate in mL/min) against Pc (capillary hydrostatic pressure (e.g. in mmHg)) from 2333.14 Pa to 6999.43 Pa. One curve for each value of pic (capillary oncotic pressure (e.g. in mmHg)): 3733.03 Pa, 1866.51 Pa, 7466.05 Pa. Other inputs as in the worked example: Pi = 0.0 Pa, pii = 399.96716224500005 Pa, Kf = 0.5. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Pc = 4666.28 Pa, Jv = 5.*

```python
human_physiology_ai999x.solve('hp_starling_filtration', Pc=4666.283559525, Pi=0.0, pic=3733.0268476200004, pii=399.96716224500005, Kf=0.5)
# -> NFP = 10, Jv = 5   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_starling_filtration', Pc=7332.731307825001, Pi=1999.835811225, pic=3999.67162245, pii=0.0, Kf=12.5)
# -> NFP = 10, Jv = 125   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Starling filtration: Pc = 35 mmHg, Pi = 0 mmHg, pic = 28 mmHg, pii = 3 mmHg, Kf = 0.5 | SOLVED | NFP = 10, Jv = 5 |
| Starling filtration: Pc = 55 mmHg, Pi = 15 mmHg, pic = 30 mmHg, pii = 0 mmHg, Kf = 12.5 | SOLVED | NFP = 10, Jv = 125 |
| Starling filtration: Pi = 0 mmHg, pic = 28 mmHg, pii = 3 mmHg, Kf = 0.5 | NEEDS_INPUT | hp_starling_filtration needs Pc (capillary hydrostatic pressure (e.g. in mmHg)). Example: solve('hp_starling_filtration' |
| Starling filtration: Pc = 35 mmHg, Pi = 0 mmHg, pic = 28 mmHg, pii = 3 mmHg, Kf = 0.5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Starling filtration: Pc = 35 mmHg, Pi = 0 mmHg, pic = 28 mmHg, pii = 3 mmHg, Kf = 0 | INVALID_INPUT | Starling forces and capillary filtration: Kf must be positive. |

<a id="hp_hydrostatic_pressure"></a>

### Hydrostatic pressure of a blood column: `hp_hydrostatic_pressure`

Pressure difference rho g h between two points a height h apart in a column of blood (posture effects).

$$
\Delta P = \rho g h
$$

| input | meaning | condition |
|---|---|---|
| `h` | height of the column (e.g. in cm) (positive = the lower point) |  |
| `rho` | blood density (e.g. in kg/m^3) |  |

| output | meaning |
|---|---|
| `dP` | pressure difference in Pa |
| `dP_mmHg` | pressure difference in mmHg |

- **Method:** $dP = \rho$ g h
- **Assumes:** $\rho > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dP_mmHg (pressure difference in mmHg) against h (height of the column (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hydrostatic-pressure-curve)

*dP_mmHg (pressure difference in mmHg) against h (height of the column (e.g. in cm) (positive = the lower point)) from 0.65 m to 1.95 m. One curve for each value of rho (blood density (e.g. in kg/m^3)): 1060 kg/m^3, 530 kg/m^3, 2120 kg/m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, h = 1.3 m, dP_mmHg = 101.36.*

![dP_mmHg (pressure difference in mmHg) over h (height of the column (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hydrostatic-pressure-field)

*dP_mmHg (pressure difference in mmHg) over h (height of the column (e.g. in cm) (positive = the lower point)) from 0.65 m to 1.95 m and rho (blood density (e.g. in kg/m^3)) from 530 kg/m^3 to 1590 kg/m^3: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 25.34 to dark 228.06); iso-lines at dP_mmHg = 50, 100, 150, 200. Marked: the worked example, h = 1.3 m, rho = 1060 kg/m^3, dP_mmHg = 101.36.*

```python
human_physiology_ai999x.solve('hp_hydrostatic_pressure', h=1.3, rho=1060.0)
# -> dP = 13513.6, dP_mmHg = 101.36   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hydrostatic_pressure', h=0.5, rho=1000.0)
# -> dP = 4903.32, dP_mmHg = 36.778   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| hydrostatic pressure of a blood column: h = 130 cm, rho = 1060 kg/m^3 | SOLVED | dP = 13513.6, dP_mmHg = 101.36 |
| hydrostatic pressure of a blood column: h = 50 cm, rho = 1000 kg/m^3 | SOLVED | dP = 4903.32, dP_mmHg = 36.778 |
| hydrostatic pressure of a blood column: rho = 1060 kg/m^3 | NEEDS_INPUT | hp_hydrostatic_pressure needs h (height of the column (e.g. in cm) (positive = the lower point)). Example: solve('hp_hyd |
| hydrostatic pressure of a blood column: h = 130 cm, rho = 1060 kg/m^3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| hydrostatic pressure of a blood column: h = 130 cm, rho = 0 kg/m^3 | INVALID_INPUT | Hydrostatic pressure of a blood column: rho must be positive. |

<a id="hp_moens_korteweg"></a>

### Pulse wave velocity by the Moens-Korteweg equation, `hp_moens_korteweg`

Pulse wave velocity of a thin elastic tube PWV = sqrt(E h/(rho D)).

$$
PWV = \sqrt{\frac{E h}{\rho D}}
$$

| input | meaning | condition |
|---|---|---|
| `E` | wall elastic modulus (e.g. in kPa) |  |
| `h` | wall thickness (e.g. in mm) |  |
| `D` | vessel diameter (e.g. in mm) |  |
| `rho` | blood density (e.g. in kg/m^3) |  |

| output | meaning |
|---|---|
| `PWV` | pulse wave velocity in m/s |

- **Method:** PWV = sqrt(E h /(rho D))
- **Assumes:** all inputs > 0; thin-walled elastic tube
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PWV (pulse wave velocity in m/s) against D (vessel diameter (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-moens-korteweg-curve)

*PWV (pulse wave velocity in m/s) against D (vessel diameter (e.g. in mm)) from 0.0125 m to 0.0375 m. One curve for each value of E (wall elastic modulus (e.g. in kPa)): 500000 Pa, 250000 Pa, 1e+06 Pa. Other inputs as in the worked example: h = 0.0015 m, rho = 1060.0 kg/m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, D = 0.025 m, PWV = 5.31995.*

```python
human_physiology_ai999x.solve('hp_moens_korteweg', E=500000.0, h=0.0015, D=0.025, rho=1060.0)
# -> PWV = 5.31995   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_moens_korteweg', E=1000000.0, h=0.001, D=0.02, rho=1050.0)
# -> PWV = 6.90066   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Moens-Korteweg pulse wave velocity: E = 500 kPa, h = 1.5 mm, D = 25 mm, rho = 1060 kg/m^3 | SOLVED | PWV = 5.31995 |
| Moens-Korteweg pulse wave velocity: E = 1000 kPa, h = 1 mm, D = 20 mm, rho = 1050 kg/m^3 | SOLVED | PWV = 6.90066 |
| Moens-Korteweg pulse wave velocity: h = 1.5 mm, D = 25 mm, rho = 1060 kg/m^3 | NEEDS_INPUT | hp_moens_korteweg needs E (wall elastic modulus (e.g. in kPa)). Example: solve('hp_moens_korteweg', E='500000 Pa', h='0. |
| Moens-Korteweg pulse wave velocity: E = 500 kPa, h = 1.5 mm, D = 25 mm, rho = 1060 kg/m^3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Moens-Korteweg pulse wave velocity: E = 0 kPa, h = 1.5 mm, D = 25 mm, rho = 1060 kg/m^3 | INVALID_INPUT | Pulse wave velocity by the Moens-Korteweg equation: E must be positive. |

<a id="hp_womersley_number"></a>

### Womersley number of pulsatile blood flow: `hp_womersley_number`

$\alpha = r$ sqrt(omega rho/eta) with $\omega = 2$ pi $HR/60$: the ratio of pulsatile inertia to viscous forces.

$$
\alpha = r\sqrt{\frac{\omega\rho}{\eta}}
$$

| input | meaning | condition |
|---|---|---|
| `r` | vessel radius (e.g. in mm) |  |
| `HR` | heart rate in beats/min |  |
| `rho` | blood density (e.g. in kg/m^3) |  |
| `eta` | viscosity in cP |  |

| output | meaning |
|---|---|
| `alpha` | Womersley number |
| `omega` | angular frequency in rad/s |

- **Method:** $\alpha = r$ sqrt(omega rho / eta)
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![alpha (Womersley number) against eta (viscosity in cP) from 1](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-womersley-number-curve)

*alpha (Womersley number) against eta (viscosity in cP) from 1.75 to 5.25. One curve for each value of r (vessel radius (e.g. in mm)): 0.0125 m, 0.00625 m, 0.025 m. Other inputs as in the worked example: HR = 72.0, rho = 1060.0 kg/m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, eta = 3.5, alpha = 18.889.*

```python
human_physiology_ai999x.solve('hp_womersley_number', r=0.0125, HR=72, rho=1060.0, eta=3.5)
# -> alpha = 18.889, omega = 7.53982   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_womersley_number', r=0.002, HR=60, rho=1050.0, eta=4)
# -> alpha = 2.56853, omega = 6.28319   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Womersley number: r = 12.5 mm, HR = 72, rho = 1060 kg/m^3, eta = 3.5 | SOLVED | alpha = 18.889, omega = 7.53982 |
| Womersley number: r = 2 mm, HR = 60, rho = 1050 kg/m^3, eta = 4 | SOLVED | alpha = 2.56853, omega = 6.28319 |
| Womersley number: HR = 72, rho = 1060 kg/m^3, eta = 3.5 | NEEDS_INPUT | hp_womersley_number needs r (vessel radius (e.g. in mm)). Example: solve('hp_womersley_number', r='0.0125 m', HR=72, rho |
| Womersley number: r = 12.5 mm, HR = 72, rho = 1060 kg/m^3, eta = 3.5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Womersley number: r = 12.5 mm, HR = 0, rho = 1060 kg/m^3, eta = 3.5 | INVALID_INPUT | Womersley number of pulsatile blood flow: HR must be positive. |

<a id="hp_qp_qs"></a>

### Pulmonary-to-systemic flow ratio Qp/Qs: `hp_qp_qs`

Qp/Qs = (SaO2 - SvO2)/(SpvO2 - SpaO2) from oxygen saturations (hemoglobin and dissolved O2 cancel or are neglected).

$$
\frac{Q_p}{Q_s} = \frac{S_aO_2 - S_{\bar v}O_2}{S_{pv}O_2 - S_{pa}O_2}
$$

| input | meaning | condition |
|---|---|---|
| `SaO2` | systemic arterial saturation |  |
| `SvO2` | mixed venous saturation |  |
| `SpvO2` | pulmonary venous saturation |  |
| `SpaO2` | pulmonary artery saturation |  |

| output | meaning |
|---|---|
| `Qp_Qs` | pulmonary to systemic flow ratio |

- **Method:** Fick principle applied to both circulations
- **Assumes:** SaO2 > SvO2; SpvO2 > SpaO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Qp_Qs (pulmonary to systemic flow ratio) against SpaO2 (pulmonary artery saturation) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-qp-qs-curve)

*Qp_Qs (pulmonary to systemic flow ratio) against SpaO2 (pulmonary artery saturation) from 0.02 to 0.98. One curve for each value of SvO2 (mixed venous saturation): 0.65, 0.26, 0.74. Other inputs as in the worked example: SaO2 = 0.95, SpvO2 = 0.98. 40 points per curve computed by human-physiology-ai999x; marked: the worked example, SpaO2 = 0.8, Qp_Qs = 1.66667.*

```python
human_physiology_ai999x.solve('hp_qp_qs', SaO2=0.95, SvO2=0.65, SpvO2=0.98, SpaO2=0.8)
# -> Qp_Qs = 1.66667   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_qp_qs', SaO2=0.97, SvO2=0.7, SpvO2=0.98, SpaO2=0.85)
# -> Qp_Qs = 2.07692   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Qp/Qs: SaO2 = 0.95, SvO2 = 0.65, SpvO2 = 0.98, SpaO2 = 0.8 | SOLVED | Qp_Qs = 1.66667 |
| Qp/Qs: SaO2 = 0.97, SvO2 = 0.7, SpvO2 = 0.98, SpaO2 = 0.85 | SOLVED | Qp_Qs = 2.07692 |
| Qp/Qs: SvO2 = 0.65, SpvO2 = 0.98, SpaO2 = 0.8 | NEEDS_INPUT | hp_qp_qs needs SaO2 (systemic arterial saturation). Example: solve('hp_qp_qs', SaO2=0.95, SvO2=0.65, SpvO2=0.98, SpaO2=0 |
| Qp/Qs: SaO2 = 0.95, SvO2 = 0.65, SpvO2 = 0.98, SpaO2 = 0.8, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Qp/Qs: SaO2 = 0.6, SvO2 = 0.65, SpvO2 = 0.98, SpaO2 = 0.8 | INVALID_INPUT | Pulmonary-to-systemic flow ratio Qp/Qs: need SaO2 > SvO2 and SpvO2 > SpaO2. |

<a id="hp_shunt_fraction"></a>

### Physiological shunt fraction (shunt equation), `hp_shunt_fraction`

Qs/Qt = (Cc'O2 - CaO2)/(Cc'O2 - CvO2) from end-capillary, arterial and mixed venous O2 contents.

$$
\frac{Q_s}{Q_t} = \frac{C_{c'}O_2 - C_aO_2}{C_{c'}O_2 - C_{\bar v}O_2}
$$

| input | meaning | condition |
|---|---|---|
| `CcO2` | end-capillary O2 content in mL/dL |  |
| `CaO2` | arterial O2 content in mL/dL |  |
| `CvO2` | mixed venous O2 content in mL/dL |  |

| output | meaning |
|---|---|
| `Qs_Qt` | shunt fraction |

- **Method:** mixing balance of shunted and capillary blood
- **Assumes:** CcO2 >= CaO2 > CvO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Qs_Qt (shunt fraction) against CvO2 (mixed venous O2 content in mL/dL) from 7](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-shunt-fraction-curve)

*Qs_Qt (shunt fraction) against CvO2 (mixed venous O2 content in mL/dL) from 7.5 to 22.5. One curve for each value of CcO2 (end-capillary O2 content in mL/dL): 21, 42. Other inputs as in the worked example: CaO2 = 20.0. 25 points per curve computed by human-physiology-ai999x; marked: the worked example, CvO2 = 15, Qs_Qt = 0.166667.*

```python
human_physiology_ai999x.solve('hp_shunt_fraction', CcO2=21, CaO2=20, CvO2=15)
# -> Qs_Qt = 0.166667   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_shunt_fraction', CcO2=20.5, CaO2=18, CvO2=14)
# -> Qs_Qt = 0.384615   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| shunt fraction: CcO2 = 21, CaO2 = 20, CvO2 = 15 | SOLVED | Qs_Qt = 0.166667 |
| shunt fraction: CcO2 = 20.5, CaO2 = 18, CvO2 = 14 | SOLVED | Qs_Qt = 0.384615 |
| shunt fraction: CaO2 = 20, CvO2 = 15 | NEEDS_INPUT | hp_shunt_fraction needs CcO2 (end-capillary O2 content in mL/dL). Example: solve('hp_shunt_fraction', CcO2=21, CaO2=20,  |
| shunt fraction: CcO2 = 21, CaO2 = 20, CvO2 = 15, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| shunt fraction: CcO2 = 18, CaO2 = 20, CvO2 = 15 | INVALID_INPUT | Physiological shunt fraction (shunt equation): need CcO2 >= CaO2 > CvO2. |

<a id="hp_lung_capacities"></a>

### Lung capacities from the four lung volumes, `hp_lung_capacities`

Inspiratory capacity, functional residual capacity, vital capacity and total lung capacity from TV, IRV, ERV and RV.

$$
TLC = IRV + TV + ERV + RV
$$

| input | meaning | condition |
|---|---|---|
| `TV` | tidal volume (e.g. in mL) |  |
| `IRV` | inspiratory reserve volume (e.g. in mL) |  |
| `ERV` | expiratory reserve volume (e.g. in mL) |  |
| `RV` | residual volume (e.g. in mL) |  |

| output | meaning |
|---|---|
| `IC` | inspiratory capacity in mL |
| `FRC` | functional residual capacity in mL |
| `VC` | vital capacity in mL |
| `TLC` | total lung capacity in mL |

- **Method:** IC = TV + IRV; FRC = ERV + RV; VC = IRV + TV + ERV; TLC = VC + RV
- **Assumes:** TV, $RV > 0$; IRV, ERV >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![TLC (total lung capacity in mL) against IRV (inspiratory reserve volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-lung-capacities-curve)

*TLC (total lung capacity in mL) against IRV (inspiratory reserve volume (e.g. in mL)) from 0.0015 m^3 to 0.0045 m^3. One curve for each value of RV (residual volume (e.g. in mL)): 0.0012 m^3, 0.0006 m^3, 0.0024 m^3. Other inputs as in the worked example: TV = 0.0005 m^3, ERV = 0.0010999999999999998 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, IRV = 0.003 m^3, TLC = 5800.*

```python
human_physiology_ai999x.solve('hp_lung_capacities', TV=0.0005, IRV=0.003, ERV=0.0010999999999999998, RV=0.0012)
# -> IC = 3500, FRC = 2300, VC = 4600, TLC = 5800   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_lung_capacities', TV=0.00039999999999999996, IRV=0.002, ERV=0.0009, RV=0.001)
# -> IC = 2400, FRC = 1900, VC = 3300, TLC = 4300   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| lung capacities: TV = 500 mL, IRV = 3000 mL, ERV = 1100 mL, RV = 1200 mL | SOLVED | IC = 3500, FRC = 2300, VC = 4600, TLC = 5800 |
| lung capacities: TV = 400 mL, IRV = 2000 mL, ERV = 900 mL, RV = 1000 mL | SOLVED | IC = 2400, FRC = 1900, VC = 3300, TLC = 4300 |
| lung capacities: IRV = 3000 mL, ERV = 1100 mL, RV = 1200 mL | NEEDS_INPUT | hp_lung_capacities needs TV (tidal volume (e.g. in mL)). Example: solve('hp_lung_capacities', TV='0.0005 m^3', IRV='0.00 |
| lung capacities: TV = 500 mL, IRV = 3000 mL, ERV = 1100 mL, RV = 1200 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| lung capacities: TV = 0 mL, IRV = 3000 mL, ERV = 1100 mL, RV = 1200 mL | INVALID_INPUT | Lung capacities from the four lung volumes: TV must be positive. |

<a id="hp_minute_ventilation"></a>

### Minute and alveolar ventilation: `hp_minute_ventilation`

Minute ventilation $VE = VT$ x RR, alveolar ventilation $VA = (VT - VD)$ x RR and the dead-space fraction VD/VT.

$$
\dot V_A = (V_T - V_D) f
$$

| input | meaning | condition |
|---|---|---|
| `VT` | tidal volume (e.g. in mL) |  |
| `RR` | respiratory rate in breaths/min |  |
| `VD` | dead space (e.g. in mL) |  |

| output | meaning |
|---|---|
| `VE` | minute ventilation in L/min |
| `VA` | alveolar ventilation in L/min |
| `VD_VT` | dead space fraction |

- **Method:** $VE = VT$ f; $VA = (VT - VD)$ f
- **Assumes:** $0  \leq  VD < VT$; $RR > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![VA (alveolar ventilation in L/min) against VT (tidal volume (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-minute-ventilation-curve)

*VA (alveolar ventilation in L/min) against VT (tidal volume (e.g. in mL)) from 0.00025 m^3 to 0.00075 m^3. One curve for each value of RR (respiratory rate in breaths/min): 12, 6, 24. Other inputs as in the worked example: VD = 0.00015 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, VT = 0.0005 m^3, VA = 4.2.*

```python
human_physiology_ai999x.solve('hp_minute_ventilation', VT=0.0005, RR=12, VD=0.00015)
# -> VE = 6, VA = 4.2, VD_VT = 0.3   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_minute_ventilation', VT=0.0003, RR=30, VD=0.00015)
# -> VE = 9, VA = 4.5, VD_VT = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| minute ventilation: VT = 500 mL, RR = 12, VD = 150 mL | SOLVED | VE = 6, VA = 4.2, VD_VT = 0.3 |
| minute ventilation: VT = 300 mL, RR = 30, VD = 150 mL | SOLVED | VE = 9, VA = 4.5, VD_VT = 0.5 |
| minute ventilation: RR = 12, VD = 150 mL | NEEDS_INPUT | hp_minute_ventilation needs VT (tidal volume (e.g. in mL)). Example: solve('hp_minute_ventilation', VT='0.0005 m^3', RR= |
| minute ventilation: VT = 500 mL, RR = 12, VD = 150 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| minute ventilation: VT = 500 mL, RR = 12, VD = 600 mL | INVALID_INPUT | Minute and alveolar ventilation: the dead space must be smaller than the tidal volume. |

<a id="hp_bohr_dead_space"></a>

### Physiological dead space by the Bohr (Enghoff) equation, `hp_bohr_dead_space`

VD/VT = (PaCO2 - PECO2)/PaCO2 and the dead-space volume, with arterial PCO2 standing for alveolar (Enghoff).

$$
\frac{V_D}{V_T} = \frac{P_aCO_2 - P_ECO_2}{P_aCO_2}
$$

| input | meaning | condition |
|---|---|---|
| `PaCO2` | arterial PCO2 (e.g. in mmHg) |  |
| `PECO2` | mixed expired PCO2 (e.g. in mmHg) |  |
| `VT` | tidal volume (e.g. in mL) |  |

| output | meaning |
|---|---|
| `VD_VT` | dead space fraction |
| `VD` | dead space volume in mL |

- **Method:** Bohr CO2 mass balance
- **Assumes:** 0 <= PECO2 < PaCO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![VD_VT (dead space fraction) against PaCO2 (arterial PCO2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bohr-dead-space-curve)

*VD_VT (dead space fraction) against PaCO2 (arterial PCO2 (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa. One curve for each value of PECO2 (mixed expired PCO2 (e.g. in mmHg)): 3733.03 Pa, 1866.51 Pa. Other inputs as in the worked example: VT = 0.0005 m^3. 24 points per curve computed by human-physiology-ai999x; marked: the worked example, PaCO2 = 5332.9 Pa, VD_VT = 0.3.*

```python
human_physiology_ai999x.solve('hp_bohr_dead_space', PaCO2=5332.8954966, PECO2=3733.0268476200004, VT=0.0005)
# -> VD_VT = 0.3, VD = 150   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bohr_dead_space', PaCO2=6666.119370750001, PECO2=3333.0596853750003, VT=0.00039999999999999996)
# -> VD_VT = 0.5, VD = 200   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bohr dead space: PaCO2 = 40 mmHg, PECO2 = 28 mmHg, VT = 500 mL | SOLVED | VD_VT = 0.3, VD = 150 |
| Bohr dead space: PaCO2 = 50 mmHg, PECO2 = 25 mmHg, VT = 400 mL | SOLVED | VD_VT = 0.5, VD = 200 |
| Bohr dead space: PECO2 = 28 mmHg, VT = 500 mL | NEEDS_INPUT | hp_bohr_dead_space needs PaCO2 (arterial PCO2 (e.g. in mmHg)). Example: solve('hp_bohr_dead_space', PaCO2='5332.9 Pa', P |
| Bohr dead space: PaCO2 = 40 mmHg, PECO2 = 28 mmHg, VT = 500 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Bohr dead space: PaCO2 = 30 mmHg, PECO2 = 40 mmHg, VT = 500 mL | INVALID_INPUT | Physiological dead space by the Bohr (Enghoff) equation: the mixed expired PCO2 must be below the arterial PCO2. |

<a id="hp_alveolar_ventilation_co2"></a>

### Alveolar ventilation from CO2 output and arterial PCO2, `hp_alveolar_ventilation_co2`

Alveolar ventilation equation VA (BTPS) = VCO2 (STPD) x 863/PaCO2, with $K = 760$ x $310.15/273.15$ mmHg.

$$
\dot V_A = \frac{\dot V_{CO_2} \times 0.863}{P_aCO_2}
$$

| input | meaning | condition |
|---|---|---|
| `VCO2` | CO2 output (e.g. in mL/min) (STPD) |  |
| `PaCO2` | arterial (alveolar) PCO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `VA` | alveolar ventilation in L/min (BTPS) |

- **Method:** VA = VCO2 K / PaCO2, $K = 760$ x $310.15/273.15$
- **Assumes:** VCO2, PaCO2 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![VA (alveolar ventilation in L/min (BTPS)) against PaCO2 (arterial (alveolar) PCO2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-alveolar-ventilation-co2-curve)

*VA (alveolar ventilation in L/min (BTPS)) against PaCO2 (arterial (alveolar) PCO2 (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa. One curve for each value of VCO2 (CO2 output (e.g. in mL/min) (STPD)): 3.3333e-06 m^3/s, 1.6667e-06 m^3/s, 6.6667e-06 m^3/s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, PaCO2 = 5332.9 Pa, VA = 4.31474.*

![VA (alveolar ventilation in L/min (BTPS)) over PaCO2 (arterial (alveolar) PCO2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-alveolar-ventilation-co2-field)

*VA (alveolar ventilation in L/min (BTPS)) over PaCO2 (arterial (alveolar) PCO2 (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa and VCO2 (CO2 output (e.g. in mL/min) (STPD)) from 1.6667e-06 m^3/s to 5.0000e-06 m^3/s: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 1.43825 to dark 12.9442); iso-lines at VA = 2, 4, 6, 8, 10, 12. Marked: the worked example, PaCO2 = 5332.9 Pa, VCO2 = 3.3333e-06 m^3/s, VA = 4.31474.*

```python
human_physiology_ai999x.solve('hp_alveolar_ventilation_co2', VCO2=3.3333333333333333e-06, PaCO2=5332.8954966)
# -> VA = 4.31474   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_alveolar_ventilation_co2', VCO2=4.166666666666667e-06, PaCO2=6666.119370750001)
# -> VA = 4.31474   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| alveolar ventilation from CO2 output: VCO2 = 200 mL/min, PaCO2 = 40 mmHg | SOLVED | VA = 4.31474 |
| alveolar ventilation from CO2 output: VCO2 = 250 mL/min, PaCO2 = 50 mmHg | SOLVED | VA = 4.31474 |
| alveolar ventilation from CO2 output: PaCO2 = 40 mmHg | NEEDS_INPUT | hp_alveolar_ventilation_co2 needs VCO2 (CO2 output (e.g. in mL/min) (STPD)). Example: solve('hp_alveolar_ventilation_co2 |
| alveolar ventilation from CO2 output: VCO2 = 200 mL/min, PaCO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| alveolar ventilation from CO2 output: VCO2 = 200 mL/min, PaCO2 = 0 mmHg | INVALID_INPUT | Alveolar ventilation from CO2 output and arterial PCO2: PaCO2 must be positive. |

<a id="hp_alveolar_gas_equation"></a>

### Alveolar gas equation: `hp_alveolar_gas_equation`

Inspired PO2 = FiO2 $(PB - PH2O)$ and alveolar PO2 by the simplified (PiO2 - PaCO2/R) and full (nitrogen-balance) alveolar gas equation.

$$
P_AO_2 = F_iO_2(P_B - P_{H_2O}) - \frac{P_aCO_2}{R}\left[1 - F_iO_2(1 - R)\right]
$$

| input | meaning | condition |
|---|---|---|
| `FiO2` | inspired O2 fraction |  |
| `Patm` | barometric pressure (e.g. in mmHg) |  |
| `PH2O` | water vapour pressure (e.g. in mmHg) |  |
| `PaCO2` | arterial PCO2 (e.g. in mmHg) |  |
| `RQ` | respiratory quotient |  |

| output | meaning |
|---|---|
| `PiO2` | inspired PO2 in mmHg |
| `PAO2` | alveolar PO2, simplified equation, in mmHg |
| `PAO2_full` | alveolar PO2, full equation, in mmHg |

- **Method:** PAO2 = PiO2 - PaCO2/R (+ PaCO2 FiO2 (1 - R)/R)
- **Assumes:** 0 < FiO2 <= 1; Patm > PH2O; $0 < RQ  \leq  1.5$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PAO2 (alveolar PO2, simplified equation, in mmHg) against RQ (respiratory quotient) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-alveolar-gas-equation-curve)

*PAO2 (alveolar PO2, simplified equation, in mmHg) against RQ (respiratory quotient) from 0.4 to 1.2. One curve for each value of Patm (barometric pressure (e.g. in mmHg)): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: FiO2 = 0.21, PH2O = 6266.152208505 Pa, PaCO2 = 5332.8954966 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, RQ = 0.8, PAO2 = 99.73.*

```python
human_physiology_ai999x.solve('hp_alveolar_gas_equation', FiO2=0.21, Patm=101325.01443540001, PH2O=6266.152208505, PaCO2=5332.8954966, RQ=0.8)
# -> PiO2 = 149.73, PAO2 = 99.73, PAO2_full = 101.83   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_alveolar_gas_equation', FiO2=0.5, Patm=93325.67119050001, PH2O=6266.152208505, PaCO2=5999.507433675, RQ=0.8)
# -> PiO2 = 326.5, PAO2 = 270.25, PAO2_full = 275.875   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| alveolar gas equation: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8 | SOLVED | PiO2 = 149.73, PAO2 = 99.73, PAO2_full = 101.83 |
| alveolar gas equation: FiO2 = 0.5, Patm = 700 mmHg, PH2O = 47 mmHg, PaCO2 = 45 mmHg, RQ = 0.8 | SOLVED | PiO2 = 326.5, PAO2 = 270.25, PAO2_full = 275.875 |
| alveolar gas equation: Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8 | NEEDS_INPUT | hp_alveolar_gas_equation needs FiO2 (inspired O2 fraction). Example: solve('hp_alveolar_gas_equation', FiO2=0.21, Patm=' |
| alveolar gas equation: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| alveolar gas equation: FiO2 = 0.21, Patm = 40 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8 | INVALID_INPUT | Alveolar gas equation: the barometric pressure must exceed the water vapour pressure. |

<a id="hp_aa_gradient"></a>

### Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio, `hp_aa_gradient`

PAO2 from the simplified alveolar gas equation, the A-a gradient PAO2 - PaO2, the a/A ratio and PaO2/FiO2.

$$
A\!-\!a = P_AO_2 - P_aO_2
$$

| input | meaning | condition |
|---|---|---|
| `FiO2` | inspired O2 fraction |  |
| `Patm` | barometric pressure (e.g. in mmHg) |  |
| `PH2O` | water vapour pressure (e.g. in mmHg) |  |
| `PaCO2` | arterial PCO2 (e.g. in mmHg) |  |
| `PaO2` | arterial PO2 (e.g. in mmHg) |  |
| `RQ` | respiratory quotient |  |

| output | meaning |
|---|---|
| `PAO2` | alveolar PO2 in mmHg |
| `Aa_gradient` | A-a gradient in mmHg |
| `a_A_ratio` | PaO2/PAO2 |
| `PF_ratio` | PaO2/FiO2 in mmHg |

- **Method:** A-a = FiO2 (PB - PH2O) - PaCO2/R - PaO2
- **Assumes:** 0 < FiO2 <= 1; computed PAO2 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Aa_gradient (A-a gradient in mmHg) against RQ (respiratory quotient) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-aa-gradient-curve)

*Aa_gradient (A-a gradient in mmHg) against RQ (respiratory quotient) from 0.4 to 1.2. One curve for each value of Patm (barometric pressure (e.g. in mmHg)): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: FiO2 = 0.21, PH2O = 6266.152208505 Pa, PaCO2 = 5332.8954966 Pa, PaO2 = 11999.01486735 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, RQ = 0.8, Aa_gradient = 9.73.*

```python
human_physiology_ai999x.solve('hp_aa_gradient', FiO2=0.21, Patm=101325.01443540001, PH2O=6266.152208505, PaCO2=5332.8954966, PaO2=11999.01486735, RQ=0.8)
# -> PAO2 = 99.73, Aa_gradient = 9.73, a_A_ratio = 0.902437, PF_ratio = 428.571   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_aa_gradient', FiO2=0.4, Patm=101325.01443540001, PH2O=6266.152208505, PaCO2=4666.283559525, PaO2=10665.7909932, RQ=0.8)
# -> PAO2 = 241.45, Aa_gradient = 161.45, a_A_ratio = 0.331332, PF_ratio = 200   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A-a gradient: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, PaO2 = 90 mmHg, RQ = 0.8 | SOLVED | PAO2 = 99.73, Aa_gradient = 9.73, a_A_ratio = 0.902437, PF_ratio = 428.571 |
| A-a gradient: FiO2 = 0.4, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 35 mmHg, PaO2 = 80 mmHg, RQ = 0.8 | SOLVED | PAO2 = 241.45, Aa_gradient = 161.45, a_A_ratio = 0.331332, PF_ratio = 200 |
| A-a gradient: Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, PaO2 = 90 mmHg, RQ = 0.8 | NEEDS_INPUT | hp_aa_gradient needs FiO2 (inspired O2 fraction). Example: solve('hp_aa_gradient', FiO2=0.21, Patm='101325 Pa', PH2O='62 |
| A-a gradient: FiO2 = 0.21, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, PaO2 = 90 mmHg, RQ = 0.8, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| A-a gradient: FiO2 = 0, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, PaO2 = 90 mmHg, RQ = 0.8 | INVALID_INPUT | Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio: FiO2 must be a fraction between 0 and 1. |

<a id="hp_oxygen_content"></a>

### Arterial oxygen content: `hp_oxygen_content`

CaO2 = 1.34 x Hb x SaO2 + 0.003 x PaO2 (hemoglobin-bound plus dissolved oxygen).

$$
C_aO_2 = 1.34\,Hb\,S_aO_2 + 0.003\,P_aO_2
$$

| input | meaning | condition |
|---|---|---|
| `Hb` | hemoglobin in g/dL |  |
| `SaO2` | arterial saturation |  |
| `PaO2` | arterial PO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `CaO2` | oxygen content in mL $O_{2}/dL$ |
| `bound` | hemoglobin-bound O2 in mL/dL |
| `dissolved` | dissolved O2 in mL/dL |

- **Method:** 1.34 Hb $S + 0.003$ PO2
- **Assumes:** $Hb > 0$; 0 <= SaO2 <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CaO2 (oxygen content in mL O2/dL) against Hb (hemoglobin in g/dL) from 7](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-oxygen-content-curve)

*CaO2 (oxygen content in mL O2/dL) against Hb (hemoglobin in g/dL) from 7.5 to 22.5. One curve for each value of SaO2 (arterial saturation): 0.98, 0.26, 0.74. Other inputs as in the worked example: PaO2 = 13332.238741500001 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Hb = 15, CaO2 = 19.998.*

```python
human_physiology_ai999x.solve('hp_oxygen_content', Hb=15, SaO2=0.98, PaO2=13332.238741500001)
# -> CaO2 = 19.998, bound = 19.698, dissolved = 0.3   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_oxygen_content', Hb=12, SaO2=0.9, PaO2=7999.3432449)
# -> CaO2 = 14.652, bound = 14.472, dissolved = 0.18   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| oxygen content: Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg | SOLVED | CaO2 = 19.998, bound = 19.698, dissolved = 0.3 |
| oxygen content: Hb = 12, SaO2 = 0.9, PaO2 = 60 mmHg | SOLVED | CaO2 = 14.652, bound = 14.472, dissolved = 0.18 |
| oxygen content: SaO2 = 0.98, PaO2 = 100 mmHg | NEEDS_INPUT | hp_oxygen_content needs Hb (hemoglobin in g/dL). Example: solve('hp_oxygen_content', Hb=15, SaO2=0.98, PaO2='13332.2 Pa' |
| oxygen content: Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| oxygen content: Hb = 15, SaO2 = 120%, PaO2 = 100 mmHg | INVALID_INPUT | Arterial oxygen content: SaO2 must be a fraction between 0 and 1. |

<a id="hp_oxygen_delivery"></a>

### Systemic oxygen delivery: `hp_oxygen_delivery`

$DO2 = CO$ x CaO2 x 10 (mL O2/min), with CaO2 = 1.34 Hb SaO2 + 0.003 PaO2.

$$
D_{O_2} = CO \times C_aO_2 \times 10
$$

| input | meaning | condition |
|---|---|---|
| `CO` | cardiac output (e.g. in L/min) |  |
| `Hb` | hemoglobin in g/dL |  |
| `SaO2` | arterial saturation |  |
| `PaO2` | arterial PO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `DO2` | oxygen delivery in mL O2/min |
| `CaO2` | arterial oxygen content in mL/dL |

- **Method:** $DO2 = 10$ CO CaO2
- **Assumes:** CO, $Hb > 0$; 0 <= SaO2 <= 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![DO2 (oxygen delivery in mL O2/min) against CO (cardiac output (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-oxygen-delivery-curve)

*DO2 (oxygen delivery in mL O2/min) against CO (cardiac output (e.g. in L/min)) from 4.1667e-05 m^3/s to 0.000125 m^3/s. One curve for each value of Hb (hemoglobin in g/dL): 15, 7.5, 30. Other inputs as in the worked example: SaO2 = 0.98, PaO2 = 13332.238741500001 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, CO = 8.3333e-05 m^3/s, DO2 = 999.9.*

```python
human_physiology_ai999x.solve('hp_oxygen_delivery', CO=8.333333333333334e-05, Hb=15, SaO2=0.98, PaO2=13332.238741500001)
# -> DO2 = 999.9, CaO2 = 19.998   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_oxygen_delivery', CO=6.666666666666667e-05, Hb=10, SaO2=1, PaO2=0.0)
# -> DO2 = 536, CaO2 = 13.4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| oxygen delivery: CO = 5 L/min, Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg | SOLVED | DO2 = 999.9, CaO2 = 19.998 |
| oxygen delivery: CO = 4 L/min, Hb = 10, SaO2 = 1, PaO2 = 0 mmHg | SOLVED | DO2 = 536, CaO2 = 13.4 |
| oxygen delivery: Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg | NEEDS_INPUT | hp_oxygen_delivery needs CO (cardiac output (e.g. in L/min)). Example: solve('hp_oxygen_delivery', CO='8.33333e-05 m^3/s |
| oxygen delivery: CO = 5 L/min, Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| oxygen delivery: CO = 0 L/min, Hb = 15, SaO2 = 0.98, PaO2 = 100 mmHg | INVALID_INPUT | Systemic oxygen delivery: CO must be positive. |

<a id="hp_oxygen_consumption"></a>

### Oxygen consumption and extraction ratio (reverse Fick), `hp_oxygen_consumption`

$VO2 = CO$ (CaO2 - CvO2) x 10 and the oxygen extraction ratio (CaO2 - CvO2)/CaO2.

$$
\dot V_{O_2} = CO\,(C_aO_2 - C_{\bar v}O_2)
$$

| input | meaning | condition |
|---|---|---|
| `CO` | cardiac output (e.g. in L/min) |  |
| `Hb` | hemoglobin in g/dL |  |
| `SaO2` | arterial saturation |  |
| `SvO2` | mixed venous saturation |  |
| `PaO2` | arterial PO2 (e.g. in mmHg) |  |
| `PvO2` | mixed venous PO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `VO2` | oxygen consumption in mL/min |
| `O2ER` | oxygen extraction ratio |
| `CaO2` | arterial content in mL/dL |
| `CvO2` | venous content in mL/dL |

- **Method:** $VO2 = 10$ CO (CaO2 - CvO2)
- **Assumes:** CvO2 < CaO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![VO2 (oxygen consumption in mL/min) against CO (cardiac output (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-oxygen-consumption-curve)

*VO2 (oxygen consumption in mL/min) against CO (cardiac output (e.g. in L/min)) from 4.1667e-05 m^3/s to 0.000125 m^3/s. One curve for each value of Hb (hemoglobin in g/dL): 15, 7.5, 30. Other inputs as in the worked example: SaO2 = 0.98, SvO2 = 0.75, PaO2 = 13332.238741500001 Pa, PvO2 = 5332.8954966 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, CO = 8.3333e-05 m^3/s, VO2 = 240.15.*

```python
human_physiology_ai999x.solve('hp_oxygen_consumption', CO=8.333333333333334e-05, Hb=15, SaO2=0.98, SvO2=0.75, PaO2=13332.238741500001, PvO2=5332.8954966)
# -> VO2 = 240.15, O2ER = 0.240174, CaO2 = 19.998, CvO2 = 15.195   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_oxygen_consumption', CO=0.0001, Hb=13, SaO2=0.97, SvO2=0.65, PaO2=11999.01486735, PvO2=4666.283559525)
# -> VO2 = 344.364, O2ER = 0.33432, CaO2 = 17.1674, CvO2 = 11.428   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| oxygen consumption: CO = 5 L/min, Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 100 mmHg, PvO2 = 40 mmHg | SOLVED | VO2 = 240.15, O2ER = 0.240174, CaO2 = 19.998, CvO2 = 15.195 |
| oxygen consumption: CO = 6 L/min, Hb = 13, SaO2 = 0.97, SvO2 = 0.65, PaO2 = 90 mmHg, PvO2 = 35 mmHg | SOLVED | VO2 = 344.364, O2ER = 0.33432, CaO2 = 17.1674, CvO2 = 11.428 |
| oxygen consumption: Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 100 mmHg, PvO2 = 40 mmHg | NEEDS_INPUT | hp_oxygen_consumption needs CO (cardiac output (e.g. in L/min)). Example: solve('hp_oxygen_consumption', CO='8.33333e-05 |
| oxygen consumption: CO = 5 L/min, Hb = 15, SaO2 = 0.98, SvO2 = 0.75, PaO2 = 100 mmHg, PvO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| oxygen consumption: CO = 5 L/min, Hb = 15, SaO2 = 0.6, SvO2 = 0.75, PaO2 = 30 mmHg, PvO2 = 40 mmHg | INVALID_INPUT | Oxygen consumption and extraction ratio (reverse Fick): the venous oxygen content must be below the arterial. |

<a id="hp_hill_saturation"></a>

### Oxyhemoglobin saturation by the Hill equation, `hp_hill_saturation`

$SO2 = PO2^{n}/(PO2^{n} + P50^{n})$ for a Hill coefficient n and half-saturation pressure P50.

$$
S = \frac{P^n}{P^n + P_{50}^n}
$$

| input | meaning | condition |
|---|---|---|
| `PO2` | partial pressure of O2 (e.g. in mmHg) |  |
| `P50` | P50 (e.g. in mmHg) |  |
| `n` | Hill coefficient |  |

| output | meaning |
|---|---|
| `SO2` | fractional saturation |

- **Method:** Hill equation
- **Assumes:** $PO2  \geq  0$; P50, $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SO2 (fractional saturation) against PO2 (partial pressure of O2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hill-saturation-curve)

*SO2 (fractional saturation) against PO2 (partial pressure of O2 (e.g. in mmHg)) from 533.29 Pa to 53329 Pa on a logarithmic axis. One curve for each value of P50 (P50 (e.g. in mmHg)): 3573.04 Pa, 1786.52 Pa, 7146.08 Pa. Other inputs as in the worked example: n = 2.7. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, PO2 = 5332.9 Pa, SO2 = 0.746738.*

```python
human_physiology_ai999x.solve('hp_hill_saturation', PO2=5332.8954966, P50=3573.0399827220003, n=2.7)
# -> SO2 = 0.746738   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hill_saturation', PO2=3573.0399827220003, P50=3573.0399827220003, n=2.7)
# -> SO2 = 0.5   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hill_saturation', PO2=7999.3432449, P50=3599.704460205, n=2.6)
# -> SO2 = 0.88856   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hill saturation: PO2 = 40 mmHg, P50 = 26.8 mmHg, n = 2.7 | SOLVED | SO2 = 0.746738 |
| Hill saturation: PO2 = 26.8 mmHg, P50 = 26.8 mmHg, n = 2.7 | SOLVED | SO2 = 0.5 |
| Hill saturation: PO2 = 60 mmHg, P50 = 27 mmHg, n = 2.6 | SOLVED | SO2 = 0.88856 |
| Hill saturation: P50 = 26.8 mmHg, n = 2.7 | NEEDS_INPUT | hp_hill_saturation needs PO2 (partial pressure of O2 (e.g. in mmHg)). Example: solve('hp_hill_saturation', PO2='5332.9 P |
| Hill saturation: PO2 = 40 mmHg, P50 = 26.8 mmHg, n = 2.7, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Hill saturation: PO2 = 40 mmHg, P50 = 0 mmHg, n = 2.7 | INVALID_INPUT | Oxyhemoglobin saturation by the Hill equation: P50 must be positive. |

<a id="hp_hill_po2"></a>

### PO2 at a given saturation by the Hill equation, `hp_hill_po2`

Inverse Hill equation $PO2 = P50$ (S/(1 - S))^$(1/n)$.

$$
P = P_{50}\left(\frac{S}{1-S}\right)^{1/n}
$$

| input | meaning | condition |
|---|---|---|
| `SO2` | fractional saturation |  |
| `P50` | P50 (e.g. in mmHg) |  |
| `n` | Hill coefficient |  |

| output | meaning |
|---|---|
| `PO2` | partial pressure of O2 in mmHg |

- **Method:** $PO2 = P50$ (S/(1-S))^$(1/n)$
- **Assumes:** $0 < SO2 < 1$; P50, $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PO2 (partial pressure of O2 in mmHg) against n (Hill coefficient) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hill-po2-curve)

*PO2 (partial pressure of O2 in mmHg) against n (Hill coefficient) from 0.27 to 27 on a logarithmic axis. One curve for each value of P50 (P50 (e.g. in mmHg)): 3573.04 Pa, 1786.52 Pa, 7146.08 Pa. Other inputs as in the worked example: SO2 = 0.75. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, n = 2.7, PO2 = 40.2575.*

```python
human_physiology_ai999x.solve('hp_hill_po2', SO2=0.75, P50=3573.0399827220003, n=2.7)
# -> PO2 = 40.2575   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hill_po2', SO2=0.9, P50=3573.0399827220003, n=2.7)
# -> PO2 = 60.4725   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hill PO2: SO2 = 0.75, P50 = 26.8 mmHg, n = 2.7 | SOLVED | PO2 = 40.2575 |
| Hill PO2: SO2 = 0.9, P50 = 26.8 mmHg, n = 2.7 | SOLVED | PO2 = 60.4725 |
| Hill PO2: P50 = 26.8 mmHg, n = 2.7 | NEEDS_INPUT | hp_hill_po2 needs SO2 (fractional saturation). Example: solve('hp_hill_po2', SO2=0.75, P50='3573.04 Pa', n=2.7). |
| Hill PO2: SO2 = 0.75, P50 = 26.8 mmHg, n = 2.7, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Hill PO2: SO2 = 1, P50 = 26.8 mmHg, n = 2.7 | INVALID_INPUT | PO2 at a given saturation by the Hill equation: SO2 must be a fraction between 0 and 1. |

<a id="hp_hill_fit"></a>

### Hill coefficient and P50 from two points of the dissociation curve, `hp_hill_fit`

The Hill coefficient n and P50 of the curve through two measured (PO2, saturation) points (Hill plot slope).

$$
n = \frac{\ln\frac{S_2/(1-S_2)}{S_1/(1-S_1)}}{\ln(P_2/P_1)}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | first PO2 (e.g. in mmHg) |  |
| `S1` | saturation at P1 |  |
| `P2` | second PO2 (e.g. in mmHg) |  |
| `S2` | saturation at P2 |  |

| output | meaning |
|---|---|
| `n` | Hill coefficient |
| `P50` | P50 in mmHg |

- **Method:** slope of log($S/(1-S)$) against log PO2
- **Assumes:** $0 < S < 1$; saturation increases with PO2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P50 (P50 in mmHg) against S2 (saturation at P2) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hill-fit-curve)

*P50 (P50 in mmHg) against S2 (saturation at P2) from 0.02 to 0.98, logarithmic output axis. One curve for each value of S1 (saturation at P1): 0.312124, 0.26, 0.74. Other inputs as in the worked example: P1 = 2666.4477483 Pa, P2 = 7999.3432449 Pa. 28 points per curve computed by human-physiology-ai999x; marked: the worked example, S2 = 0.898078, P50 = 26.8.*

```python
human_physiology_ai999x.solve('hp_hill_fit', P1=2666.4477483, S1=0.312124018611, P2=7999.3432449, S2=0.898077644148)
# -> n = 2.7, P50 = 26.8   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hill fit: P1 = 20 mmHg, S1 = 0.312124018611, P2 = 60 mmHg, S2 = 0.898077644148 | SOLVED | n = 2.7, P50 = 26.8 |
| Hill fit: S1 = 0.312124018611, P2 = 60 mmHg, S2 = 0.898077644148 | NEEDS_INPUT | hp_hill_fit needs P1 (first PO2 (e.g. in mmHg)). Example: solve('hp_hill_fit', P1='2666.45 Pa', S1=0.312124, P2='7999.34 |
| Hill fit: P1 = 20 mmHg, S1 = 0.312124018611, P2 = 60 mmHg, S2 = 0.898077644148, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Hill fit: P1 = 20 mmHg, S1 = 0.9, P2 = 60 mmHg, S2 = 0.3 | INVALID_INPUT | Hill coefficient and P50 from two points of the dissociation curve: saturation must increase with PO2. |

<a id="hp_severinghaus_saturation"></a>

### Saturation by the Severinghaus (1979) dissociation equation, `hp_severinghaus_saturation`

Standard human oxyhemoglobin curve S = 1/(23400/(PO2^3 + 150 PO2) + 1) (37 degC, pH 7.40, PCO2 40 mmHg).

$$
S = \left(\frac{23400}{P^3 + 150P} + 1\right)^{-1}
$$

| input | meaning | condition |
|---|---|---|
| `PO2` | partial pressure of O2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `SO2` | fractional saturation |

- **Method:** Severinghaus equation
- **Assumes:** $PO2 > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SO2 (fractional saturation) against PO2 (partial pressure of O2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-severinghaus-saturation-curve)

*SO2 (fractional saturation) against PO2 (partial pressure of O2 (e.g. in mmHg)) from 533.29 Pa to 53329 Pa on a logarithmic axis. 41 points computed by human-physiology-ai999x; marked: the worked example, PO2 = 5332.9 Pa, SO2 = 0.749465.*

```python
human_physiology_ai999x.solve('hp_severinghaus_saturation', PO2=5332.8954966)
# -> SO2 = 0.749465   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_severinghaus_saturation', PO2=13332.238741500001)
# -> SO2 = 0.977465   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Severinghaus saturation: PO2 = 40 mmHg | SOLVED | SO2 = 0.749465 |
| Severinghaus saturation: PO2 = 100 mmHg | SOLVED | SO2 = 0.977465 |
| Severinghaus saturation:  | NEEDS_INPUT | hp_severinghaus_saturation needs PO2 (partial pressure of O2 (e.g. in mmHg)). Example: solve('hp_severinghaus_saturation |
| Severinghaus saturation: PO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Severinghaus saturation: PO2 = 0 mmHg | INVALID_INPUT | Saturation by the Severinghaus (1979) dissociation equation: PO2 must be positive. |

<a id="hp_severinghaus_po2"></a>

### PO2 at a given saturation by the Severinghaus equation, `hp_severinghaus_po2`

Inverse of the Severinghaus curve: the real root of $P^{3} + 150$ $P = 23400$ $S/(1 - S)$.

$$
P^3 + 150P = \frac{23400\,S}{1-S}
$$

| input | meaning | condition |
|---|---|---|
| `SO2` | fractional saturation |  |

| output | meaning |
|---|---|
| `PO2` | partial pressure of O2 in mmHg |

- **Method:** Cardano root of the cubic, Newton polished
- **Assumes:** $0 < SO2 < 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PO2 (partial pressure of O2 in mmHg) against SO2 (fractional saturation) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-severinghaus-po2-curve)

*PO2 (partial pressure of O2 in mmHg) against SO2 (fractional saturation) from 0.02 to 0.98. 41 points computed by human-physiology-ai999x; marked: the worked example, SO2 = 0.749465, PO2 = 40.*

```python
human_physiology_ai999x.solve('hp_severinghaus_po2', SO2=0.749464668094)
# -> PO2 = 40   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_severinghaus_po2', SO2=0.977465331279)
# -> PO2 = 100   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Severinghaus PO2: SO2 = 0.749464668094 | SOLVED | PO2 = 40 |
| Severinghaus PO2: SO2 = 0.977465331279 | SOLVED | PO2 = 100 |
| Severinghaus PO2:  | NEEDS_INPUT | hp_severinghaus_po2 needs SO2 (fractional saturation). Example: solve('hp_severinghaus_po2', SO2=0.749465). |
| Severinghaus PO2: SO2 = 0.749464668094, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Severinghaus PO2: SO2 = 1 | INVALID_INPUT | PO2 at a given saturation by the Severinghaus equation: SO2 must be a fraction between 0 and 1. |

<a id="hp_virtual_po2"></a>

### Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus), `hp_virtual_po2`

Virtual $PO2 = PO2$ x 10^(0.024 $(37 - T) + 0.40$ $(pH - 7.40) + 0.06$ log10(40/PCO2)) and the saturation from the standard curve (Bohr effect).

$$
P_{virt} = P\cdot 10^{0.024(37-T) + 0.40(pH - 7.40) + 0.06\log_{10}(40/P_{CO_2})}
$$

| input | meaning | condition |
|---|---|---|
| `PO2` | measured PO2 (e.g. in mmHg) |  |
| `T` | temperature (e.g. in degC) |  |
| `pH` | blood pH |  |
| `PCO2` | PCO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `PO2_virtual` | standard-condition (virtual) PO2 in mmHg |
| `SO2` | fractional saturation |
| `factor` | correction factor |

- **Method:** Severinghaus (1979) correction, then the standard curve
- **Assumes:** $25  \leq  T  \leq  43$ degC; $6.8  \leq  pH  \leq  7.8$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SO2 (fractional saturation) against PO2 (measured PO2 (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-virtual-po2-curve)

*SO2 (fractional saturation) against PO2 (measured PO2 (e.g. in mmHg)) from 1066.58 Pa to 106658 Pa on a logarithmic axis. Other inputs as in the worked example: T = 312.15 K, pH = 7.3, PCO2 = 6666.119370750001 Pa. 41 points computed by human-physiology-ai999x; marked: the worked example, PO2 = 10665.8 Pa, SO2 = 0.922228.*

```python
human_physiology_ai999x.solve('hp_virtual_po2', PO2=10665.7909932, T=312.15, pH=7.3, PCO2=6666.119370750001)
# -> PO2_virtual = 64.4578, SO2 = 0.922228, factor = 0.805722   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_virtual_po2', PO2=7999.3432449, T=310.15, pH=7.45, PCO2=3999.67162245)
# -> PO2_virtual = 63.9216, SO2 = 0.920454, factor = 1.06536   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| virtual PO2: PO2 = 80 mmHg, T = 39 degC, pH = 7.3, PCO2 = 50 mmHg | SOLVED | PO2_virtual = 64.4578, SO2 = 0.922228, factor = 0.805722 |
| virtual PO2: PO2 = 60 mmHg, T = 37 degC, pH = 7.45, PCO2 = 30 mmHg | SOLVED | PO2_virtual = 63.9216, SO2 = 0.920454, factor = 1.06536 |
| virtual PO2: T = 39 degC, pH = 7.3, PCO2 = 50 mmHg | NEEDS_INPUT | hp_virtual_po2 needs PO2 (measured PO2 (e.g. in mmHg)). Example: solve('hp_virtual_po2', PO2='10665.8 Pa', T='312.15 K', |
| virtual PO2: PO2 = 80 mmHg, T = 39 degC, pH = 7.3, PCO2 = 50 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| virtual PO2: PO2 = 80 mmHg, T = 50 degC, pH = 7.3, PCO2 = 50 mmHg | INVALID_INPUT | Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus): temperature 25-43 degC and pH 6.8-7.8. |

<a id="hp_ph_henderson_hasselbalch"></a>

### Blood pH by the Henderson-Hasselbalch equation, `hp_ph_henderson_hasselbalch`

$pH = 6.1 + log10$(HCO3/(0.03 PaCO2)) and the hydrogen-ion concentration in nmol/L.

$$
pH = 6.1 + \log_{10}\frac{[HCO_3^-]}{0.03\,P_{CO_2}}
$$

| input | meaning | condition |
|---|---|---|
| `HCO3` | bicarbonate (e.g. in mmol/L) |  |
| `PaCO2` | PCO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `H` | hydrogen-ion concentration in nmol/L |

- **Method:** pK' 6.1, CO2 solubility 0.03 mmol/L per mmHg
- **Assumes:** HCO3, PaCO2 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against HCO3 (bicarbonate (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-ph-henderson-hasselbalch-curve)

*pH (pH) against HCO3 (bicarbonate (e.g. in mmol/L)) from 12 mol/m^3 to 36 mol/m^3. One curve for each value of PaCO2 (PCO2 (e.g. in mmHg)): 5332.9 Pa, 2666.45 Pa, 10665.8 Pa. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, HCO3 = 24 mol/m^3, pH = 7.40103.*

![pH (pH) over HCO3 (bicarbonate (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-ph-henderson-hasselbalch-field)

*pH (pH) over HCO3 (bicarbonate (e.g. in mmol/L)) from 12 mol/m^3 to 36 mol/m^3 and PaCO2 (PCO2 (e.g. in mmHg)) from 2666.45 Pa to 7999.34 Pa: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 6.92391 to dark 7.87815); iso-lines at pH = 7, 7.2, 7.4, 7.6, 7.8. Marked: the worked example, HCO3 = 24 mol/m^3, PaCO2 = 5332.9 Pa, pH = 7.40103.*

```python
human_physiology_ai999x.solve('hp_ph_henderson_hasselbalch', HCO3=24.0, PaCO2=5332.8954966)
# -> pH = 7.40103, H = 39.7164   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ph_henderson_hasselbalch', HCO3=12.0, PaCO2=3333.0596853750003)
# -> pH = 7.30412, H = 49.6455   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| blood pH: HCO3 = 24 mmol/L, PaCO2 = 40 mmHg | SOLVED | pH = 7.40103, H = 39.7164 |
| blood pH: HCO3 = 12 mmol/L, PaCO2 = 25 mmHg | SOLVED | pH = 7.30412, H = 49.6455 |
| blood pH: PaCO2 = 40 mmHg | NEEDS_INPUT | hp_ph_henderson_hasselbalch needs HCO3 (bicarbonate (e.g. in mmol/L)). Example: solve('hp_ph_henderson_hasselbalch', HCO |
| blood pH: HCO3 = 24 mmol/L, PaCO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| blood pH: HCO3 = 0 mmol/L, PaCO2 = 40 mmHg | INVALID_INPUT | Blood pH by the Henderson-Hasselbalch equation: HCO3 must be positive. |

<a id="hp_bicarbonate_from_ph"></a>

### Bicarbonate from pH and PCO2 (Henderson-Hasselbalch), `hp_bicarbonate_from_ph`

HCO3 = 0.03 PaCO2 x $10^{pH - 6.1}$.

$$
[HCO_3^-] = 0.03\,P_{CO_2}\,10^{pH - 6.1}
$$

| input | meaning | condition |
|---|---|---|
| `pH` | pH |  |
| `PaCO2` | PCO2 (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `HCO3` | bicarbonate in mmol/L |

- **Method:** Henderson-Hasselbalch solved for bicarbonate
- **Assumes:** $6.5  \leq  pH  \leq  8.0$; PaCO2 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![HCO3 (bicarbonate in mmol/L) against pH (pH) from 6](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bicarbonate-from-ph-curve)

*HCO3 (bicarbonate in mmol/L) against pH (pH) from 6.66 to 8.14. One curve for each value of PaCO2 (PCO2 (e.g. in mmHg)): 5332.9 Pa, 2666.45 Pa, 10665.8 Pa. 28 points per curve computed by human-physiology-ai999x; marked: the worked example, pH = 7.4, HCO3 = 23.9431.*

```python
human_physiology_ai999x.solve('hp_bicarbonate_from_ph', pH=7.4, PaCO2=5332.8954966)
# -> HCO3 = 23.9431   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bicarbonate_from_ph', pH=7.25, PaCO2=7999.3432449)
# -> HCO3 = 25.4257   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| bicarbonate from pH: pH = 7.4, PaCO2 = 40 mmHg | SOLVED | HCO3 = 23.9431 |
| bicarbonate from pH: pH = 7.25, PaCO2 = 60 mmHg | SOLVED | HCO3 = 25.4257 |
| bicarbonate from pH: PaCO2 = 40 mmHg | NEEDS_INPUT | hp_bicarbonate_from_ph needs pH (pH). Example: solve('hp_bicarbonate_from_ph', pH=7.4, PaCO2='5332.9 Pa'). |
| bicarbonate from pH: pH = 7.4, PaCO2 = 40 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| bicarbonate from pH: pH = 9, PaCO2 = 40 mmHg | INVALID_INPUT | Bicarbonate from pH and PCO2 (Henderson-Hasselbalch): pH must be between 6.5 and 8.0. |

<a id="hp_pco2_from_ph"></a>

### PCO2 from pH and bicarbonate (Henderson-Hasselbalch), `hp_pco2_from_ph`

PaCO2 = HCO3/(0.03 x $10^{pH - 6.1}$).

$$
P_{CO_2} = \frac{[HCO_3^-]}{0.03\cdot 10^{pH - 6.1}}
$$

| input | meaning | condition |
|---|---|---|
| `pH` | pH |  |
| `HCO3` | bicarbonate (e.g. in mmol/L) |  |

| output | meaning |
|---|---|
| `PaCO2` | PCO2 in mmHg |

- **Method:** Henderson-Hasselbalch solved for PCO2
- **Assumes:** $6.5  \leq  pH  \leq  8.0$; HCO3 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PaCO2 (PCO2 in mmHg) against pH (pH) from 6](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-pco2-from-ph-curve)

*PaCO2 (PCO2 in mmHg) against pH (pH) from 6.66 to 8.14. One curve for each value of HCO3 (bicarbonate (e.g. in mmol/L)): 24 mol/m^3, 12 mol/m^3, 48 mol/m^3. 28 points per curve computed by human-physiology-ai999x; marked: the worked example, pH = 7.4, PaCO2 = 40.095.*

```python
human_physiology_ai999x.solve('hp_pco2_from_ph', pH=7.4, HCO3=24.0)
# -> PaCO2 = 40.095   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_pco2_from_ph', pH=7.2, HCO3=15.0)
# -> PaCO2 = 39.7164   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PCO2 from pH: pH = 7.4, HCO3 = 24 mmol/L | SOLVED | PaCO2 = 40.095 |
| PCO2 from pH: pH = 7.2, HCO3 = 15 mmol/L | SOLVED | PaCO2 = 39.7164 |
| PCO2 from pH: HCO3 = 24 mmol/L | NEEDS_INPUT | hp_pco2_from_ph needs pH (pH). Example: solve('hp_pco2_from_ph', pH=7.4, HCO3='24 mol/m^3'). |
| PCO2 from pH: pH = 7.4, HCO3 = 24 mmol/L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| PCO2 from pH: pH = 6, HCO3 = 24 mmol/L | INVALID_INPUT | PCO2 from pH and bicarbonate (Henderson-Hasselbalch): pH must be between 6.5 and 8.0. |

<a id="hp_anion_gap"></a>

### Serum anion gap and albumin-corrected anion gap, `hp_anion_gap`

AG = Na - (Cl + HCO3) and the albumin-corrected value $AG + 2.5$ (4.0 - albumin in g/dL).

$$
AG = [Na^+] - ([Cl^-] + [HCO_3^-])
$$

| input | meaning | condition |
|---|---|---|
| `Na` | sodium (e.g. in mmol/L) |  |
| `Cl` | chloride (e.g. in mmol/L) |  |
| `HCO3` | bicarbonate (e.g. in mmol/L) |  |
| `albumin` | albumin in g/dL (optional; normal albumin if omitted) |  |

| output | meaning |
|---|---|
| `AG` | anion gap in mmol/L |
| `AG_corrected` | albumin-corrected anion gap in mmol/L |

- **Method:** AG = Na - Cl - HCO3; + 2.5 per g/dL of albumin below 4.0
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![AG (anion gap in mmol/L) against Na (sodium (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-anion-gap-curve)

*AG (anion gap in mmol/L) against Na (sodium (e.g. in mmol/L)) from 70 mol/m^3 to 210 mol/m^3. One curve for each value of Cl (chloride (e.g. in mmol/L)): 104 mol/m^3, 52 mol/m^3, 208 mol/m^3. Other inputs as in the worked example: HCO3 = 24.0 mol/m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Na = 140 mol/m^3, AG = 12.*

```python
human_physiology_ai999x.solve('hp_anion_gap', Na=140.0, Cl=104.0, HCO3=24.0)
# -> AG = 12, AG_corrected = 12   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_anion_gap', Na=140.0, Cl=104.0, HCO3=24.0, albumin=2)
# -> AG = 12, AG_corrected = 17   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| anion gap: Na = 140 mmol/L, Cl = 104 mmol/L, HCO3 = 24 mmol/L | SOLVED | AG = 12, AG_corrected = 12 |
| anion gap: Na = 140 mmol/L, Cl = 104 mmol/L, HCO3 = 24 mmol/L, albumin = 2 | SOLVED | AG = 12, AG_corrected = 17 |
| anion gap: Cl = 104 mmol/L, HCO3 = 24 mmol/L | NEEDS_INPUT | hp_anion_gap needs Na (sodium (e.g. in mmol/L)). Example: solve('hp_anion_gap', Na='140 mol/m^3', Cl='104 mol/m^3', HCO3 |
| anion gap: Na = 140 mmol/L, Cl = 104 mmol/L, HCO3 = 24 mmol/L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| anion gap: Na = 0 mmol/L, Cl = 104 mmol/L, HCO3 = 24 mmol/L | INVALID_INPUT | Serum anion gap and albumin-corrected anion gap: Na must be positive. |

<a id="hp_fick_diffusion"></a>

### Fick's law of diffusion across a membrane: `hp_fick_diffusion`

Steady diffusive flux $J = D$ A dC/T across a barrier of thickness T, and the diffusion time $T^{2}/(2D)$.

$$
J = D A \frac{\Delta C}{T},\quad t = \frac{T^2}{2D}
$$

| input | meaning | condition |
|---|---|---|
| `D` | diffusion coefficient (e.g. in cm^2/s) |  |
| `A` | membrane area (e.g. in cm^2) |  |
| `dC` | concentration difference (e.g. in mmol/L) |  |
| `T` | membrane thickness (e.g. in um) |  |

| output | meaning |
|---|---|
| `J` | diffusion rate in mmol/s |
| `t_diffusion` | characteristic diffusion time across the membrane in s |

- **Method:** $J = D$ A dC / T (units converted to cm)
- **Assumes:** D, A, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![J (diffusion rate in mmol/s) against T (membrane thickness (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fick-diffusion-curve)

*J (diffusion rate in mmol/s) against T (membrane thickness (e.g. in um)) from 1.0000e-06 m to 3.0000e-06 m. One curve for each value of D (diffusion coefficient (e.g. in cm^2/s)): 1.5000e-09 m^2/s, 7.5000e-10 m^2/s, 3.0000e-09 m^2/s. Other inputs as in the worked example: A = 0.0002 m^2, dC = 0.5 mol/m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, T = 2.0000e-06 m, J = 7.5000e-05.*

```python
human_physiology_ai999x.solve('hp_fick_diffusion', D=1.5000000000000002e-09, A=0.0002, dC=0.5, T=2e-06)
# -> J = 7.5e-05, t_diffusion = 0.00133333   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_fick_diffusion', D=2e-09, A=0.0001, dC=1.0, T=1e-06)
# -> J = 0.0002, t_diffusion = 0.00025   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Fick diffusion: D = 1.5e-05 cm^2/s, A = 2 cm^2, dC = 0.5 mmol/L, T = 2 um | SOLVED | J = 7.5e-05, t_diffusion = 0.00133333 |
| Fick diffusion: D = 2e-05 cm^2/s, A = 1 cm^2, dC = 1 mmol/L, T = 1 um | SOLVED | J = 0.0002, t_diffusion = 0.00025 |
| Fick diffusion: A = 2 cm^2, dC = 0.5 mmol/L, T = 2 um | NEEDS_INPUT | hp_fick_diffusion needs D (diffusion coefficient (e.g. in cm^2/s)). Example: solve('hp_fick_diffusion', D='1.5e-09 m^2/s |
| Fick diffusion: D = 1.5e-05 cm^2/s, A = 2 cm^2, dC = 0.5 mmol/L, T = 2 um, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Fick diffusion: D = 1.5e-05 cm^2/s, A = 2 cm^2, dC = 0.5 mmol/L, T = 0 um | INVALID_INPUT | Fick's law of diffusion across a membrane: T must be positive. |

<a id="hp_respiratory_mechanics"></a>

### Static and dynamic compliance and airway resistance, `hp_respiratory_mechanics`

Static compliance VT/(Pplat - PEEP), dynamic compliance VT/(Ppeak - PEEP), airway resistance (Ppeak - Pplat)/flow and driving pressure.

$$
C_{st} = \frac{V_T}{P_{plat} - PEEP},\quad R_{aw} = \frac{P_{peak} - P_{plat}}{\dot V}
$$

| input | meaning | condition |
|---|---|---|
| `VT` | tidal volume (e.g. in mL) |  |
| `Ppeak` | peak airway pressure in cmH2O |  |
| `Pplat` | plateau pressure in cmH2O |  |
| `PEEP` | end-expiratory pressure in cmH2O |  |
| `flow` | constant inspiratory flow (e.g. in L/min) |  |

| output | meaning |
|---|---|
| `C_static` | static compliance in mL/cmH2O |
| `C_dynamic` | dynamic compliance in mL/cmH2O |
| `R_aw` | airway resistance in cmH2O s/L |
| `driving_pressure` | driving pressure in cmH2O |

- **Method:** pressure differences over volume and flow
- **Assumes:** Ppeak >= Pplat > PEEP >= 0; constant flow
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![C_static (static compliance in mL/cmH2O) against Pplat (plateau pressure in cmH2O) from 10 to 30](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-respiratory-mechanics-curve)

*C_static (static compliance in mL/cmH2O) against Pplat (plateau pressure in cmH2O) from 10 to 30. One curve for each value of VT (tidal volume (e.g. in mL)): 0.0005 m^3, 0.00025 m^3, 0.001 m^3. Other inputs as in the worked example: Ppeak = 25.0, PEEP = 5.0, flow = 0.001 m^3/s. 23 points per curve computed by human-physiology-ai999x; marked: the worked example, Pplat = 20, C_static = 33.3333.*

```python
human_physiology_ai999x.solve('hp_respiratory_mechanics', VT=0.0005, Ppeak=25, Pplat=20, PEEP=5, flow=0.001)
# -> C_static = 33.3333, C_dynamic = 25, R_aw = 5, driving_pressure = 15   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_respiratory_mechanics', VT=0.00045, Ppeak=30, Pplat=20, PEEP=5, flow=0.0006666666666666668)
# -> C_static = 30, C_dynamic = 18, R_aw = 15, driving_pressure = 15   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| respiratory mechanics: VT = 500 mL, Ppeak = 25, Pplat = 20, PEEP = 5, flow = 60 L/min | SOLVED | C_static = 33.3333, C_dynamic = 25, R_aw = 5, driving_pressure = 15 |
| respiratory mechanics: VT = 450 mL, Ppeak = 30, Pplat = 20, PEEP = 5, flow = 40 L/min | SOLVED | C_static = 30, C_dynamic = 18, R_aw = 15, driving_pressure = 15 |
| respiratory mechanics: Ppeak = 25, Pplat = 20, PEEP = 5, flow = 60 L/min | NEEDS_INPUT | hp_respiratory_mechanics needs VT (tidal volume (e.g. in mL)). Example: solve('hp_respiratory_mechanics', VT='0.0005 m^3 |
| respiratory mechanics: VT = 500 mL, Ppeak = 25, Pplat = 20, PEEP = 5, flow = 60 L/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| respiratory mechanics: VT = 500 mL, Ppeak = 25, Pplat = 4, PEEP = 5, flow = 60 L/min | INVALID_INPUT | Static and dynamic compliance and airway resistance: need Ppeak >= Pplat > PEEP. |

<a id="hp_respiratory_time_constant"></a>

### Respiratory time constant and passive exhalation, `hp_respiratory_time_constant`

$\tau = R$ C, the fraction of the volume exhaled after time t, $1 - e^{-t/\tau}$, and 3 tau (about 95 %).

$$
\tau = R C,\quad V(t) = V_0 e^{-t/\tau}
$$

| input | meaning | condition |
|---|---|---|
| `R` | airway resistance in cmH2O s/L |  |
| `C` | compliance in mL/cmH2O |  |
| `t` | time (e.g. in s) (optional; needed for the fraction exhaled) |  |

| output | meaning |
|---|---|
| `tau` | time constant in s |
| `fraction_exhaled` | fraction exhaled at time t |
| `t_95` | three time constants in s |

- **Method:** single-compartment passive emptying
- **Assumes:** R, $C > 0$; $t  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![tau (time constant in s) against R (airway resistance in cmH2O s/L) from 2](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-respiratory-time-constant-curve)

*tau (time constant in s) against R (airway resistance in cmH2O s/L) from 2.5 to 7.5. One curve for each value of C (compliance in mL/cmH2O): 50, 25, 100. Other inputs as in the worked example: t = 0.5 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, R = 5, tau = 0.25.*

```python
human_physiology_ai999x.solve('hp_respiratory_time_constant', R=5, C=50, t=0.5)
# -> tau = 0.25, t_95 = 0.75, fraction_exhaled = 0.864665   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_respiratory_time_constant', R=10, C=80, t=1.0)
# -> tau = 0.8, t_95 = 2.4, fraction_exhaled = 0.713495   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_respiratory_time_constant', R=5, C=50)
# -> tau = 0.25, t_95 = 0.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| respiratory time constant: R = 5, C = 50, t = 0.5 s | SOLVED | tau = 0.25, t_95 = 0.75, fraction_exhaled = 0.864665 |
| respiratory time constant: R = 10, C = 80, t = 1 s | SOLVED | tau = 0.8, t_95 = 2.4, fraction_exhaled = 0.713495 |
| respiratory time constant: R = 5, C = 50 | SOLVED | tau = 0.25, t_95 = 0.75 |
| respiratory time constant: C = 50, t = 0.5 s | NEEDS_INPUT | hp_respiratory_time_constant needs R (airway resistance in cmH2O s/L). Example: solve('hp_respiratory_time_constant', R= |
| respiratory time constant: R = 5, C = 50, t = 0.5 s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| respiratory time constant: R = 0, C = 50, t = 0.5 s | INVALID_INPUT | Respiratory time constant and passive exhalation: R must be positive. |

<a id="hp_alveolar_laplace_pressure"></a>

### Laplace pressure of an alveolus: `hp_alveolar_laplace_pressure`

Pressure needed to keep a spherical alveolus open, $P = 2$ gamma/r (why surfactant matters).

$$
P = \frac{2\gamma}{r}
$$

| input | meaning | condition |
|---|---|---|
| `gamma` | surface tension (e.g. in mN/m) |  |
| `r` | alveolar radius (e.g. in um) |  |

| output | meaning |
|---|---|
| `P` | collapsing pressure in Pa |
| `P_cmH2O` | collapsing pressure in cmH2O |

- **Method:** $P = 2$ gamma / r
- **Assumes:** gamma, $r > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P_cmH2O (collapsing pressure in cmH2O) against r (alveolar radius (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-alveolar-laplace-pressure-curve)

*P_cmH2O (collapsing pressure in cmH2O) against r (alveolar radius (e.g. in um)) from 5.0000e-05 m to 0.00015 m. One curve for each value of gamma (surface tension (e.g. in mN/m)): 0.025 N/m, 0.0125 N/m, 0.05 N/m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, r = 1.0000e-04 m, P_cmH2O = 5.09858.*

![P_cmH2O (collapsing pressure in cmH2O) over gamma (surface tension (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-alveolar-laplace-pressure-field)

*P_cmH2O (collapsing pressure in cmH2O) over gamma (surface tension (e.g. in mN/m)) from 0.0125 N/m to 0.0375 N/m and r (alveolar radius (e.g. in um)) from 5.0000e-05 m to 0.00015 m: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 1.69953 to dark 15.2957); iso-lines at P_cmH2O = 2.5, 5, 7.5, 10, 12.5. Marked: the worked example, gamma = 0.025 N/m, r = 1.0000e-04 m, P_cmH2O = 5.09858.*

```python
human_physiology_ai999x.solve('hp_alveolar_laplace_pressure', gamma=0.025, r=9.999999999999999e-05)
# -> P = 500, P_cmH2O = 5.09858   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_alveolar_laplace_pressure', gamma=0.005, r=4.9999999999999996e-05)
# -> P = 200, P_cmH2O = 2.03943   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| alveolar Laplace pressure: gamma = 25 mN/m, r = 100 um | SOLVED | P = 500, P_cmH2O = 5.09858 |
| alveolar Laplace pressure: gamma = 5 mN/m, r = 50 um | SOLVED | P = 200, P_cmH2O = 2.03943 |
| alveolar Laplace pressure: r = 100 um | NEEDS_INPUT | hp_alveolar_laplace_pressure needs gamma (surface tension (e.g. in mN/m)). Example: solve('hp_alveolar_laplace_pressure' |
| alveolar Laplace pressure: gamma = 25 mN/m, r = 100 um, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| alveolar Laplace pressure: gamma = 25 mN/m, r = 0 um | INVALID_INPUT | Laplace pressure of an alveolus: r must be positive. |

<a id="hp_barometric_altitude"></a>

### Barometric pressure and inspired PO2 at altitude, `hp_barometric_altitude`

Standard-atmosphere barometric pressure $P = 101325$ (1 - 0.0065 $h/288.15)^(g$ M/(R L)) and the humidified inspired PO2.

$$
P = P_0\left(1 - \frac{L h}{T_0}\right)^{gM/(RL)}
$$

| input | meaning | condition |
|---|---|---|
| `h` | altitude (e.g. in m) |  |
| `FiO2` | inspired oxygen fraction (optional; dry air if omitted) |  |

| output | meaning |
|---|---|
| `Patm` | barometric pressure in mmHg |
| `PiO2` | humidified inspired PO2 in mmHg |
| `Patm_kPa` | barometric pressure in kPa |

- **Method:** ICAO standard troposphere (lapse 6.5 K/km)
- **Assumes:** $0  \leq  h  \leq  11000$ m
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Patm (barometric pressure in mmHg) against h (altitude (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-barometric-altitude-curve)

*Patm (barometric pressure in mmHg) against h (altitude (e.g. in m)) from 300 m to 30000 m on a logarithmic axis. 32 points computed by human-physiology-ai999x; marked: the worked example, h = 3000 m, Patm = 525.861.*

```python
human_physiology_ai999x.solve('hp_barometric_altitude', h=3000.0)
# -> Patm = 525.861, PiO2 = 100.321, Patm_kPa = 70.109   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_barometric_altitude', h=5000.0, FiO2=0.21)
# -> Patm = 405.187, PiO2 = 75.2193, Patm_kPa = 54.0205   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| barometric pressure at altitude: h = 3000 m | SOLVED | Patm = 525.861, PiO2 = 100.321, Patm_kPa = 70.109 |
| barometric pressure at altitude: h = 5000 m, FiO2 = 0.21 | SOLVED | Patm = 405.187, PiO2 = 75.2193, Patm_kPa = 54.0205 |
| barometric pressure at altitude:  | NEEDS_INPUT | hp_barometric_altitude needs h (altitude (e.g. in m)). Example: solve('hp_barometric_altitude', h='3000 m'). |
| barometric pressure at altitude: h = 3000 m, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| barometric pressure at altitude: h = 20000 m | INVALID_INPUT | Barometric pressure and inspired PO2 at altitude: altitude must be between 0 and 11000 m (troposphere). |

<a id="hp_gas_volume_conversion"></a>

### ATPS to BTPS and STPD gas volume conversion, `hp_gas_volume_conversion`

A spirometer volume measured at ambient temperature and pressure, saturated (ATPS), converted to body (BTPS) and standard dry (STPD) conditions.

$$
V_{BTPS} = V\,\frac{310.15}{T}\,\frac{P_B - P_{H_2O}}{P_B - 47}
$$

| input | meaning | condition |
|---|---|---|
| `V` | measured volume (e.g. in L) |  |
| `T` | spirometer temperature (e.g. in degC) |  |
| `Patm` | barometric pressure (e.g. in mmHg) |  |
| `PH2O` | water vapour pressure at T (e.g. in mmHg) |  |

| output | meaning |
|---|---|
| `V_BTPS` | volume at BTPS in L |
| `V_STPD` | volume at STPD in L |

- **Method:** ideal gas law on the dry-gas partial pressures
- **Assumes:** Patm > PH2O; Patm > 47 mmHg
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V_BTPS (volume at BTPS in L) against Patm (barometric pressure (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-gas-volume-conversion-curve)

*V_BTPS (volume at BTPS in L) against Patm (barometric pressure (e.g. in mmHg)) from 10132.5 Pa to 1.01325e+06 Pa on a logarithmic axis. One curve for each value of V (measured volume (e.g. in L)): 0.001 m^3, 0.0005 m^3, 0.002 m^3. Other inputs as in the worked example: T = 295.15 K, PH2O = 2639.7832708170004 Pa. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Patm = 101325 Pa, V_BTPS = 1.09091.*

```python
human_physiology_ai999x.solve('hp_gas_volume_conversion', V=0.001, T=295.15, Patm=101325.01443540001, PH2O=2639.7832708170004)
# -> V_BTPS = 1.09091, V_STPD = 0.901351   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_gas_volume_conversion', V=0.0025, T=298.15, Patm=98658.5666871, PH2O=3173.0728204770003)
# -> V_BTPS = 2.68768, V_STPD = 2.15838   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| gas volume conversion: V = 1 L, T = 22 degC, Patm = 760 mmHg, PH2O = 19.8 mmHg | SOLVED | V_BTPS = 1.09091, V_STPD = 0.901351 |
| gas volume conversion: V = 2.5 L, T = 25 degC, Patm = 740 mmHg, PH2O = 23.8 mmHg | SOLVED | V_BTPS = 2.68768, V_STPD = 2.15838 |
| gas volume conversion: T = 22 degC, Patm = 760 mmHg, PH2O = 19.8 mmHg | NEEDS_INPUT | hp_gas_volume_conversion needs V (measured volume (e.g. in L)). Example: solve('hp_gas_volume_conversion', V='0.001 m^3' |
| gas volume conversion: V = 1 L, T = 22 degC, Patm = 760 mmHg, PH2O = 19.8 mmHg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| gas volume conversion: V = 0 L, T = 22 degC, Patm = 760 mmHg, PH2O = 19.8 mmHg | INVALID_INPUT | ATPS to BTPS and STPD gas volume conversion: V must be positive. |

<a id="hp_weir_energy_expenditure"></a>

### Respiratory quotient and energy expenditure (Weir equation), `hp_weir_energy_expenditure`

RQ = VCO2/VO2 and resting energy expenditure by the abbreviated Weir equation (3.941 $VO2 + 1.106$ VCO2) x 1440 kcal/day (L/min).

$$
EE = (3.941\,\dot V_{O_2} + 1.106\,\dot V_{CO_2})\times 1440
$$

| input | meaning | condition |
|---|---|---|
| `VO2` | oxygen consumption (e.g. in mL/min) |  |
| `VCO2` | CO2 production (e.g. in mL/min) |  |

| output | meaning |
|---|---|
| `RQ` | respiratory quotient |
| `EE` | energy expenditure in kcal/day |
| `EE_kJ` | energy expenditure in kJ/day |

- **Method:** Weir (1949) without urinary nitrogen
- **Assumes:** VO2, VCO2 > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![EE (energy expenditure in kcal/day) against VO2 (oxygen consumption (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-weir-energy-expenditure-curve)

*EE (energy expenditure in kcal/day) against VO2 (oxygen consumption (e.g. in mL/min)) from 2.0833e-06 m^3/s to 6.2500e-06 m^3/s. One curve for each value of VCO2 (CO2 production (e.g. in mL/min)): 3.3333e-06 m^3/s, 1.6667e-06 m^3/s, 6.6667e-06 m^3/s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, VO2 = 4.1667e-06 m^3/s, EE = 1737.29.*

![EE (energy expenditure in kcal/day) over VO2 (oxygen consumption (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-weir-energy-expenditure-field)

*EE (energy expenditure in kcal/day) over VO2 (oxygen consumption (e.g. in mL/min)) from 2.0833e-06 m^3/s to 6.2500e-06 m^3/s and VCO2 (CO2 production (e.g. in mL/min)) from 1.6667e-06 m^3/s to 5.0000e-06 m^3/s: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 868.644 to dark 2605.93); iso-lines at EE = 1000, 1500, 2000, 2500. Marked: the worked example, VO2 = 4.1667e-06 m^3/s, VCO2 = 3.3333e-06 m^3/s, EE = 1737.29.*

```python
human_physiology_ai999x.solve('hp_weir_energy_expenditure', VO2=4.166666666666667e-06, VCO2=3.3333333333333333e-06)
# -> RQ = 0.8, EE = 1737.29, EE_kJ = 7268.81   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_weir_energy_expenditure', VO2=5e-06, VCO2=4.5e-06)
# -> RQ = 0.9, EE = 2132.52, EE_kJ = 8922.48   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Weir energy expenditure: VO2 = 250 mL/min, VCO2 = 200 mL/min | SOLVED | RQ = 0.8, EE = 1737.29, EE_kJ = 7268.81 |
| Weir energy expenditure: VO2 = 300 mL/min, VCO2 = 270 mL/min | SOLVED | RQ = 0.9, EE = 2132.52, EE_kJ = 8922.48 |
| Weir energy expenditure: VCO2 = 200 mL/min | NEEDS_INPUT | hp_weir_energy_expenditure needs VO2 (oxygen consumption (e.g. in mL/min)). Example: solve('hp_weir_energy_expenditure', |
| Weir energy expenditure: VO2 = 250 mL/min, VCO2 = 200 mL/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Weir energy expenditure: VO2 = 0 mL/min, VCO2 = 200 mL/min | INVALID_INPUT | Respiratory quotient and energy expenditure (Weir equation): VO2 must be positive. |

<a id="hp_fev1_fvc_ratio"></a>

### FEV1/FVC ratio: `hp_fev1_fvc_ratio`

The fraction of the forced vital capacity exhaled in the first second.

$$
\frac{FEV_1}{FVC}
$$

| input | meaning | condition |
|---|---|---|
| `FEV1` | forced expiratory volume in the first second (e.g. in L) |  |
| `FVC` | forced vital capacity (e.g. in L) |  |

| output | meaning |
|---|---|
| `ratio` | FEV1/FVC |
| `percent` | FEV1/FVC in % |

- **Method:** FEV1 / FVC
- **Assumes:** 0 < FEV1 <= FVC
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![ratio (FEV1/FVC) against FVC (forced vital capacity (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fev1-fvc-ratio-curve)

*ratio (FEV1/FVC) against FVC (forced vital capacity (e.g. in L)) from 0.002 m^3 to 0.006 m^3. One curve for each value of FEV1 (forced expiratory volume in the first second (e.g. in L)): 0.0032 m^3, 0.0016 m^3. 22 points per curve computed by human-physiology-ai999x; marked: the worked example, FVC = 0.004 m^3, ratio = 0.8.*

![ratio (FEV1/FVC) over FVC (forced vital capacity (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fev1-fvc-ratio-field)

*ratio (FEV1/FVC) over FVC (forced vital capacity (e.g. in L)) from 0.002 m^3 to 0.006 m^3 and FEV1 (forced expiratory volume in the first second (e.g. in L)) from 0.0016 m^3 to 0.0048 m^3: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.266667 to dark 1); iso-lines at ratio = 0.4, 0.6, 0.8; blank cells: inputs the capability refuses. Marked: the worked example, FVC = 0.004 m^3, FEV1 = 0.0032 m^3, ratio = 0.8.*

```python
human_physiology_ai999x.solve('hp_fev1_fvc_ratio', FEV1=0.0032, FVC=0.004)
# -> ratio = 0.8, percent = 80   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_fev1_fvc_ratio', FEV1=0.0015, FVC=0.003)
# -> ratio = 0.5, percent = 50   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| FEV1/FVC ratio: FEV1 = 3.2 L, FVC = 4 L | SOLVED | ratio = 0.8, percent = 80 |
| FEV1/FVC ratio: FEV1 = 1.5 L, FVC = 3 L | SOLVED | ratio = 0.5, percent = 50 |
| FEV1/FVC ratio: FVC = 4 L | NEEDS_INPUT | hp_fev1_fvc_ratio needs FEV1 (forced expiratory volume in the first second (e.g. in L)). Example: solve('hp_fev1_fvc_rat |
| FEV1/FVC ratio: FEV1 = 3.2 L, FVC = 4 L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| FEV1/FVC ratio: FEV1 = 5 L, FVC = 4 L | INVALID_INPUT | FEV1/FVC ratio: FEV1 cannot exceed FVC. |

<a id="hp_renal_clearance"></a>

### Renal clearance of a substance (inulin GFR), `hp_renal_clearance`

Clearance $C = U$ V/P (equal to GFR for inulin) and the urinary excretion rate U V.

$$
C_x = \frac{U_x \dot V}{P_x}
$$

| input | meaning | condition |
|---|---|---|
| `U` | urine concentration in mg/dL |  |
| `V` | urine flow (e.g. in mL/min) |  |
| `P` | plasma concentration in mg/dL |  |

| output | meaning |
|---|---|
| `C` | clearance in mL/min |
| `excretion` | excretion rate in mg/min |

- **Method:** $C = U$ V / P
- **Assumes:** $U  \geq  0$; V, $P > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![C (clearance in mL/min) against P (plasma concentration in mg/dL) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-renal-clearance-curve)

*C (clearance in mL/min) against P (plasma concentration in mg/dL) from 0.02 to 0.98. One curve for each value of U (urine concentration in mg/dL): 30, 15, 60. Other inputs as in the worked example: V = 1.6666666666666667e-08 m^3/s. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, P = 0.25, C = 120.*

```python
human_physiology_ai999x.solve('hp_renal_clearance', U=30, V=1.6666666666666667e-08, P=0.25)
# -> C = 120, excretion = 0.3   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_renal_clearance', U=100, V=2.5e-08, P=1.2)
# -> C = 125, excretion = 1.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| renal clearance: U = 30, V = 1 mL/min, P = 0.25 | SOLVED | C = 120, excretion = 0.3 |
| renal clearance: U = 100, V = 1.5 mL/min, P = 1.2 | SOLVED | C = 125, excretion = 1.5 |
| renal clearance: V = 1 mL/min, P = 0.25 | NEEDS_INPUT | hp_renal_clearance needs U (urine concentration in mg/dL). Example: solve('hp_renal_clearance', U=30, V='1.66667e-08 m^3 |
| renal clearance: U = 30, V = 1 mL/min, P = 0.25, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| renal clearance: U = 30, V = 1 mL/min, P = 0 | INVALID_INPUT | Renal clearance of a substance (inulin GFR): P must be positive. |

<a id="hp_creatinine_clearance_timed"></a>

### Creatinine clearance from a timed urine collection, `hp_creatinine_clearance_timed`

Urine flow from a timed collection, creatinine clearance U V/P and its value normalized to 1.73 $m^{2.}$

$$
C_{Cr} = \frac{U_{Cr}\dot V}{P_{Cr}}
$$

| input | meaning | condition |
|---|---|---|
| `Ucr` | urine creatinine in mg/dL |  |
| `Vurine` | urine volume collected (e.g. in mL) |  |
| `hours` | collection time (e.g. in h) |  |
| `Pcr` | plasma creatinine in mg/dL |  |
| `BSA` | body surface area (e.g. in m^2) (optional; standard adult area if omitted) |  |

| output | meaning |
|---|---|
| `urine_flow` | urine flow in mL/min |
| `CrCl` | creatinine clearance in mL/min |
| `CrCl_normalized` | creatinine clearance in mL/min normalized to standard body surface area |

- **Method:** V = volume/(60 hours); CrCl = U V/P; x 1.73/BSA
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CrCl (creatinine clearance in mL/min) against hours (collection time (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-creatinine-clearance-timed-curve)

*CrCl (creatinine clearance in mL/min) against hours (collection time (e.g. in h)) from 43200 s to 129600 s. One curve for each value of Ucr (urine creatinine in mg/dL): 100, 50, 200. Other inputs as in the worked example: Vurine = 0.0014399999999999999 m^3, Pcr = 1.0. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, hours = 86400 s, CrCl = 100.*

```python
human_physiology_ai999x.solve('hp_creatinine_clearance_timed', Ucr=100, Vurine=0.0014399999999999999, hours=86400.0, Pcr=1)
# -> urine_flow = 1, CrCl = 100, CrCl_normalized = 100   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_creatinine_clearance_timed', Ucr=80, Vurine=0.0018, hours=86400.0, Pcr=1.2, BSA=2.0)
# -> urine_flow = 1.25, CrCl = 83.3333, CrCl_normalized = 72.0833   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| creatinine clearance from a timed urine: Ucr = 100, Vurine = 1440 mL, hours = 24 h, Pcr = 1 | SOLVED | urine_flow = 1, CrCl = 100, CrCl_normalized = 100 |
| creatinine clearance from a timed urine: Ucr = 80, Vurine = 1800 mL, hours = 24 h, Pcr = 1.2, BSA = 2 m^2 | SOLVED | urine_flow = 1.25, CrCl = 83.3333, CrCl_normalized = 72.0833 |
| creatinine clearance from a timed urine: Vurine = 1440 mL, hours = 24 h, Pcr = 1 | NEEDS_INPUT | hp_creatinine_clearance_timed needs Ucr (urine creatinine in mg/dL). Example: solve('hp_creatinine_clearance_timed', Ucr |
| creatinine clearance from a timed urine: Ucr = 100, Vurine = 1440 mL, hours = 24 h, Pcr = 1, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| creatinine clearance from a timed urine: Ucr = 100, Vurine = 1440 mL, hours = 0 h, Pcr = 1 | INVALID_INPUT | Creatinine clearance from a timed urine collection: hours must be positive. |

<a id="hp_pah_renal_flow"></a>

### Renal plasma and blood flow from PAH clearance, `hp_pah_renal_flow`

Effective renal plasma flow = PAH clearance, true RPF = eRPF/E (PAH extraction) and RBF = RPF/(1 - Hct).

$$
RBF = \frac{U_{PAH}\dot V / P_{PAH}}{E_{PAH}(1 - Hct)}
$$

| input | meaning | condition |
|---|---|---|
| `U_PAH` | urine PAH concentration in mg/dL |  |
| `V` | urine flow (e.g. in mL/min) |  |
| `P_PAH` | plasma PAH concentration in mg/dL |  |
| `Hct` | hematocrit (fraction) |  |
| `E` | PAH extraction ratio (optional; complete extraction, the effective flow, if omitted) |  |

| output | meaning |
|---|---|
| `eRPF` | effective renal plasma flow in mL/min |
| `RPF` | renal plasma flow in mL/min |
| `RBF` | renal blood flow in mL/min |

- **Method:** eRPF = U V/P; RPF = eRPF/E; RBF = RPF/(1 - Hct)
- **Assumes:** 0 <= Hct < 1; $0 < E  \leq  1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![RBF (renal blood flow in mL/min) against Hct (hematocrit (fraction)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-pah-renal-flow-curve)

*RBF (renal blood flow in mL/min) against Hct (hematocrit (fraction)) from 0.02 to 0.98. One curve for each value of E (PAH extraction ratio (optional; complete extraction, the effective flow, if omitted)): 0.9, 0.26. Other inputs as in the worked example: U_PAH = 600.0, V = 1.6666666666666667e-08 m^3/s, P_PAH = 1.0. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Hct = 0.45, RBF = 1212.12.*

```python
human_physiology_ai999x.solve('hp_pah_renal_flow', U_PAH=600, V=1.6666666666666667e-08, P_PAH=1, Hct=0.45, E=0.9)
# -> eRPF = 600, RPF = 666.667, RBF = 1212.12   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_pah_renal_flow', U_PAH=650, V=1.6666666666666667e-08, P_PAH=1, Hct=0.35)
# -> eRPF = 650, RPF = 650, RBF = 1000   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| renal plasma flow: U_PAH = 600, V = 1 mL/min, P_PAH = 1, Hct = 0.45, E = 0.9 | SOLVED | eRPF = 600, RPF = 666.667, RBF = 1212.12 |
| renal plasma flow: U_PAH = 650, V = 1 mL/min, P_PAH = 1, Hct = 0.35 | SOLVED | eRPF = 650, RPF = 650, RBF = 1000 |
| renal plasma flow: V = 1 mL/min, P_PAH = 1, Hct = 0.45, E = 0.9 | NEEDS_INPUT | hp_pah_renal_flow needs U_PAH (urine PAH concentration in mg/dL). Example: solve('hp_pah_renal_flow', U_PAH=600, V='1.66 |
| renal plasma flow: U_PAH = 600, V = 1 mL/min, P_PAH = 1, Hct = 0.45, E = 0.9, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| renal plasma flow: U_PAH = 600, V = 1 mL/min, P_PAH = 1, Hct = 1 | INVALID_INPUT | Renal plasma and blood flow from PAH clearance: Hct must be a fraction between 0 and 1. |

<a id="hp_filtration_fraction"></a>

### Filtration fraction: `hp_filtration_fraction`

The fraction of renal plasma flow filtered at the glomerulus, FF = GFR/RPF.

$$
FF = \frac{GFR}{RPF}
$$

| input | meaning | condition |
|---|---|---|
| `GFR` | glomerular filtration rate (e.g. in mL/min) |  |
| `RPF` | renal plasma flow (e.g. in mL/min) |  |

| output | meaning |
|---|---|
| `FF` | filtration fraction |
| `FF_percent` | filtration fraction in % |

- **Method:** FF = GFR / RPF
- **Assumes:** 0 < GFR < RPF
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![FF (filtration fraction) against RPF (renal plasma flow (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-filtration-fraction-curve)

*FF (filtration fraction) against RPF (renal plasma flow (e.g. in mL/min)) from 5.0000e-06 m^3/s to 1.5000e-05 m^3/s. One curve for each value of GFR (glomerular filtration rate (e.g. in mL/min)): 2.0000e-06 m^3/s, 1.0000e-06 m^3/s, 4.0000e-06 m^3/s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, RPF = 1.0000e-05 m^3/s, FF = 0.2.*

![FF (filtration fraction) over GFR (glomerular filtration rate (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-filtration-fraction-field)

*FF (filtration fraction) over GFR (glomerular filtration rate (e.g. in mL/min)) from 1.0000e-06 m^3/s to 3.0000e-06 m^3/s and RPF (renal plasma flow (e.g. in mL/min)) from 5.0000e-06 m^3/s to 1.5000e-05 m^3/s: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.0666667 to dark 0.6); iso-lines at FF = 0.1, 0.2, 0.3, 0.4, 0.5. Marked: the worked example, GFR = 2.0000e-06 m^3/s, RPF = 1.0000e-05 m^3/s, FF = 0.2.*

```python
human_physiology_ai999x.solve('hp_filtration_fraction', GFR=2e-06, RPF=1e-05)
# -> FF = 0.2, FF_percent = 20   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_filtration_fraction', GFR=1.6666666666666667e-06, RPF=6.666666666666667e-06)
# -> FF = 0.25, FF_percent = 25   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| filtration fraction: GFR = 120 mL/min, RPF = 600 mL/min | SOLVED | FF = 0.2, FF_percent = 20 |
| filtration fraction: GFR = 100 mL/min, RPF = 400 mL/min | SOLVED | FF = 0.25, FF_percent = 25 |
| filtration fraction: RPF = 600 mL/min | NEEDS_INPUT | hp_filtration_fraction needs GFR (glomerular filtration rate (e.g. in mL/min)). Example: solve('hp_filtration_fraction', |
| filtration fraction: GFR = 120 mL/min, RPF = 600 mL/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| filtration fraction: GFR = 700 mL/min, RPF = 600 mL/min | INVALID_INPUT | Filtration fraction: GFR must be smaller than the renal plasma flow. |

<a id="hp_free_water_clearance"></a>

### Osmolar and free water clearance: `hp_free_water_clearance`

Osmolar clearance Cosm = Uosm V/Posm and free water clearance CH2O = V - Cosm (negative when the urine is concentrated).

$$
C_{H_2O} = \dot V - \frac{U_{osm}\dot V}{P_{osm}}
$$

| input | meaning | condition |
|---|---|---|
| `V` | urine flow (e.g. in mL/min) |  |
| `Uosm` | urine osmolality in mOsm/kg |  |
| `Posm` | plasma osmolality in mOsm/kg |  |

| output | meaning |
|---|---|
| `Cosm` | osmolar clearance in mL/min |
| `CH2O` | free water clearance in mL/min |

- **Method:** $CH2O = V$ (1 - Uosm/Posm)
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CH2O (free water clearance in mL/min) against Posm (plasma osmolality in mOsm/kg) from 30 to 3000 on a logarithmic axis](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-free-water-clearance-curve)

*CH2O (free water clearance in mL/min) against Posm (plasma osmolality in mOsm/kg) from 30 to 3000 on a logarithmic axis. One curve for each value of Uosm (urine osmolality in mOsm/kg): 150, 75, 300. Other inputs as in the worked example: V = 3.3333333333333334e-08 m^3/s. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Posm = 300, CH2O = 1.*

```python
human_physiology_ai999x.solve('hp_free_water_clearance', V=3.3333333333333334e-08, Uosm=150, Posm=300)
# -> Cosm = 1, CH2O = 1   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_free_water_clearance', V=1.6666666666666667e-08, Uosm=900, Posm=300)
# -> Cosm = 3, CH2O = -2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| free water clearance: V = 2 mL/min, Uosm = 150, Posm = 300 | SOLVED | Cosm = 1, CH2O = 1 |
| free water clearance: V = 1 mL/min, Uosm = 900, Posm = 300 | SOLVED | Cosm = 3, CH2O = -2 |
| free water clearance: Uosm = 150, Posm = 300 | NEEDS_INPUT | hp_free_water_clearance needs V (urine flow (e.g. in mL/min)). Example: solve('hp_free_water_clearance', V='3.33333e-08  |
| free water clearance: V = 2 mL/min, Uosm = 150, Posm = 300, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| free water clearance: V = 2 mL/min, Uosm = 150, Posm = 0 | INVALID_INPUT | Osmolar and free water clearance: Posm must be positive. |

<a id="hp_fractional_excretion"></a>

### Fractional excretion of a solute: `hp_fractional_excretion`

FE = (Ux Pcr)/(Px Ucr): the fraction of the filtered load excreted, with creatinine clearance as GFR (e.g. FENa).

$$
FE_x = \frac{U_x P_{Cr}}{P_x U_{Cr}}
$$

| input | meaning | condition |
|---|---|---|
| `Ux` | urine concentration of the solute (any unit) |  |
| `Px` | plasma concentration of the solute (same unit) |  |
| `Ucr` | urine creatinine (any unit) |  |
| `Pcr` | plasma creatinine (same unit) |  |

| output | meaning |
|---|---|
| `FE` | fractional excretion |
| `FE_percent` | fractional excretion in % |

- **Method:** $FE = Ux$ Pcr / (Px Ucr)
- **Assumes:** Px, Ucr, Pcr > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![FE_percent (fractional excretion in %) against Px (plasma concentration of the solute (same unit)) from 70 to 210](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fractional-excretion-curve)

*FE_percent (fractional excretion in %) against Px (plasma concentration of the solute (same unit)) from 70 to 210. One curve for each value of Ux (urine concentration of the solute (any unit)): 40, 20, 80. Other inputs as in the worked example: Ucr = 100.0, Pcr = 1.0. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Px = 140, FE_percent = 0.285714.*

```python
human_physiology_ai999x.solve('hp_fractional_excretion', Ux=40, Px=140, Ucr=100, Pcr=1)
# -> FE = 0.00285714, FE_percent = 0.285714   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_fractional_excretion', Ux=2, Px=140, Ucr=120, Pcr=1.5)
# -> FE = 0.000178571, FE_percent = 0.0178571   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| fractional excretion: Ux = 40, Px = 140, Ucr = 100, Pcr = 1 | SOLVED | FE = 0.00285714, FE_percent = 0.285714 |
| fractional excretion: Ux = 2, Px = 140, Ucr = 120, Pcr = 1.5 | SOLVED | FE = 0.000178571, FE_percent = 0.0178571 |
| fractional excretion: Px = 140, Ucr = 100, Pcr = 1 | NEEDS_INPUT | hp_fractional_excretion needs Ux (urine concentration of the solute (any unit)). Example: solve('hp_fractional_excretion |
| fractional excretion: Ux = 40, Px = 140, Ucr = 100, Pcr = 1, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| fractional excretion: Ux = 40, Px = 0, Ucr = 100, Pcr = 1 | INVALID_INPUT | Fractional excretion of a solute: Px must be positive. |

<a id="hp_egfr_ckd_epi_2021"></a>

### Estimated GFR by the CKD-EPI 2021 creatinine equation, `hp_egfr_ckd_epi_2021`

eGFR = 142 min(Scr/k, 1)^a max(Scr/k, 1)^-1.200 0.9938^age (x 1.012 female), the race-free 2021 CKD-EPI creatinine equation (computed value only).

$$
eGFR = 142\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-1.200}\,0.9938^{age}\,[1.012]
$$

| input | meaning | condition |
|---|---|---|
| `Scr` | serum creatinine in mg/dL |  |
| `age` | age in years |  |
| `sex` | sex for the equation's coefficients |  |

| output | meaning |
|---|---|
| `eGFR` | estimated GFR in mL/min normalized to standard body surface area |

- **Method:** CKD-EPI 2021: $k = 0.7/0.9$, $a = -0.241/-0.302$ (female/male)
- **Assumes:** Scr > 0; 18 <= age <= 120
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![eGFR (estimated GFR in mL/min normalized to standard body surface area) against Scr (serum creatinine in mg/dL) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-egfr-ckd-epi-2021-curve)

*eGFR (estimated GFR in mL/min normalized to standard body surface area) against Scr (serum creatinine in mg/dL) from 0.1 to 10 on a logarithmic axis. One curve for each value of age (age in years): 50, 25, 100. Other inputs as in the worked example: sex = male. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Scr = 1, eGFR = 91.6915.*

![eGFR (estimated GFR in mL/min normalized to standard body surface area) over Scr (serum creatinine in mg/dL) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-egfr-ckd-epi-2021-field)

*eGFR (estimated GFR in mL/min normalized to standard body surface area) over Scr (serum creatinine in mg/dL) from 0.1 to 10 and age (age in years) from 25 to 75 (logarithmic horizontal axis): each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 4.95226 to dark 236.018); iso-lines at eGFR = 50, 100, 150, 200. Other inputs as in the worked example: sex = male. Marked: the worked example, Scr = 1, age = 50, eGFR = 91.6915.*

```python
human_physiology_ai999x.solve('hp_egfr_ckd_epi_2021', Scr=1, age=50, sex='male')
# -> eGFR = 91.6915   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_egfr_ckd_epi_2021', Scr=0.8, age=65, sex='female')
# -> eGFR = 81.7171   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_egfr_ckd_epi_2021', Scr=0.6, age=30, sex='female')
# -> eGFR = 123.758   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| eGFR CKD-EPI 2021: Scr = 1, age = 50, sex = male | SOLVED | eGFR = 91.6915 |
| eGFR CKD-EPI 2021: Scr = 0.8, age = 65, sex = female | SOLVED | eGFR = 81.7171 |
| eGFR CKD-EPI 2021: Scr = 0.6, age = 30, sex = female | SOLVED | eGFR = 123.758 |
| eGFR CKD-EPI 2021: age = 50, sex = male | NEEDS_INPUT | hp_egfr_ckd_epi_2021 needs Scr (serum creatinine in mg/dL). Example: solve('hp_egfr_ckd_epi_2021', Scr=1, age=50, sex='m |
| eGFR CKD-EPI 2021: Scr = 1, age = 50, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| eGFR CKD-EPI 2021: Scr = 1, age = 10, sex = male | INVALID_INPUT | Estimated GFR by the CKD-EPI 2021 creatinine equation: the CKD-EPI equation applies to ages 18 to 120. |

<a id="hp_egfr_ckd_epi_2021_cys"></a>

### Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation, `hp_egfr_ckd_epi_2021_cys`

The race-free 2021 CKD-EPI equation from serum creatinine and cystatin C (computed value only).

$$
eGFR = 135\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-0.544}\min(S_{cys}/0.8,1)^{-0.323}\max(S_{cys}/0.8,1)^{-0.778}\,0.9961^{age}
$$

| input | meaning | condition |
|---|---|---|
| `Scr` | serum creatinine in mg/dL |  |
| `Scys` | serum cystatin C (e.g. in mg/L) |  |
| `age` | age in years |  |
| `sex` | sex for the equation's coefficients |  |

| output | meaning |
|---|---|
| `eGFR` | estimated GFR in mL/min normalized to standard body surface area |

- **Method:** 135 min(Scr/k,1)^a max(Scr/k,1)^-0.544 min(Scys/0.8,1)^-0.323 max(Scys/0.8,1)^-0.778 0.9961^age (x 0.963 female)
- **Assumes:** Scr, Scys > 0; 18 <= age <= 120
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![eGFR (estimated GFR in mL/min normalized to standard body surface area) against Scys (serum cystatin C (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-egfr-ckd-epi-2021-cys-curve)

*eGFR (estimated GFR in mL/min normalized to standard body surface area) against Scys (serum cystatin C (e.g. in mg/L)) from 0.0001 kg/m^3 to 0.01 kg/m^3 on a logarithmic axis. One curve for each value of Scr (serum creatinine in mg/dL): 1, 0.5, 2. Other inputs as in the worked example: age = 50.0, sex = male. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Scys = 0.001 kg/m^3, eGFR = 88.144.*

```python
human_physiology_ai999x.solve('hp_egfr_ckd_epi_2021_cys', Scr=1, Scys=0.001, age=50, sex='male')
# -> eGFR = 88.144   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_egfr_ckd_epi_2021_cys', Scr=0.8, Scys=0.0009000000000000001, age=65, sex='female')
# -> eGFR = 85.5671   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| eGFR creatinine-cystatin C: Scr = 1, Scys = 1 mg/L, age = 50, sex = male | SOLVED | eGFR = 88.144 |
| eGFR creatinine-cystatin C: Scr = 0.8, Scys = 0.9 mg/L, age = 65, sex = female | SOLVED | eGFR = 85.5671 |
| eGFR creatinine-cystatin C: Scys = 1 mg/L, age = 50, sex = male | NEEDS_INPUT | hp_egfr_ckd_epi_2021_cys needs Scr (serum creatinine in mg/dL). Example: solve('hp_egfr_ckd_epi_2021_cys', Scr=1, Scys=' |
| eGFR creatinine-cystatin C: Scr = 1, Scys = 1 mg/L, age = 50, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| eGFR creatinine-cystatin C: Scr = 1, Scys = 0 mg/L, age = 50, sex = male | INVALID_INPUT | Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation: Scys must be positive. |

<a id="hp_cockcroft_gault"></a>

### Creatinine clearance by the Cockcroft-Gault equation, `hp_cockcroft_gault`

Estimated creatinine clearance (140 - age) x weight/(72 x Scr), x 0.85 for female (computed value only).

$$
C_{Cr} = \frac{(140 - age)\,W}{72\,S_{cr}}\;[\times 0.85]
$$

| input | meaning | condition |
|---|---|---|
| `age` | age in years |  |
| `weight` | body weight (e.g. in kg) |  |
| `Scr` | serum creatinine in mg/dL |  |
| `sex` | sex for the equation's coefficient |  |

| output | meaning |
|---|---|
| `CrCl` | estimated creatinine clearance in mL/min |

- **Method:** Cockcroft-Gault (1976)
- **Assumes:** 18 <= age < 140; weight, Scr > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CrCl (estimated creatinine clearance in mL/min) against Scr (serum creatinine in mg/dL) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-cockcroft-gault-curve)

*CrCl (estimated creatinine clearance in mL/min) against Scr (serum creatinine in mg/dL) from 0.5 to 1.5. One curve for each value of weight (body weight (e.g. in kg)): 72 kg, 36 kg, 144 kg. Other inputs as in the worked example: age = 60.0, sex = male. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Scr = 1, CrCl = 80.*

```python
human_physiology_ai999x.solve('hp_cockcroft_gault', age=60, weight=72.0, Scr=1, sex='male')
# -> CrCl = 80   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_cockcroft_gault', age=70, weight=60.0, Scr=1.2, sex='female')
# -> CrCl = 41.3194   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cockcroft-Gault creatinine clearance: age = 60, weight = 72 kg, Scr = 1, sex = male | SOLVED | CrCl = 80 |
| Cockcroft-Gault creatinine clearance: age = 70, weight = 60 kg, Scr = 1.2, sex = female | SOLVED | CrCl = 41.3194 |
| Cockcroft-Gault creatinine clearance: weight = 72 kg, Scr = 1, sex = male | NEEDS_INPUT | hp_cockcroft_gault needs age (age in years). Example: solve('hp_cockcroft_gault', age=60, weight='72 kg', Scr=1, sex='ma |
| Cockcroft-Gault creatinine clearance: age = 60, weight = 72 kg, Scr = 1, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Cockcroft-Gault creatinine clearance: age = 60, weight = 72 kg, Scr = 0, sex = male | INVALID_INPUT | Creatinine clearance by the Cockcroft-Gault equation: Scr must be positive. |

<a id="hp_tubular_handling"></a>

### Filtered load, excretion and net tubular reabsorption or secretion, `hp_tubular_handling`

Filtered load GFR x Px, excretion rate Ux x V and net reabsorption (positive) or secretion (negative) as their difference.

$$
R_x = GFR\cdot P_x - U_x \dot V
$$

| input | meaning | condition |
|---|---|---|
| `GFR` | GFR (e.g. in mL/min) |  |
| `Px` | plasma concentration in mg/dL |  |
| `Ux` | urine concentration in mg/dL |  |
| `V` | urine flow (e.g. in mL/min) |  |

| output | meaning |
|---|---|
| `filtered_load` | filtered load in mg/min |
| `excreted` | excretion rate in mg/min |
| `reabsorbed` | net reabsorption in mg/min (negative = net secretion) |

- **Method:** filtered - excreted (mg/dL x mL/min / 100 = mg/min)
- **Assumes:** GFR, $V > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![reabsorbed (net reabsorption in mg/min (negative = net secretion)) against GFR (GFR (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-tubular-handling-curve)

*reabsorbed (net reabsorption in mg/min (negative = net secretion)) against GFR (GFR (e.g. in mL/min)) from 1.0000e-06 m^3/s to 3.0000e-06 m^3/s. One curve for each value of Px (plasma concentration in mg/dL): 100, 50, 200. Other inputs as in the worked example: Ux = 0.0, V = 1.6666666666666667e-08 m^3/s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, GFR = 2.0000e-06 m^3/s, reabsorbed = 120.*

```python
human_physiology_ai999x.solve('hp_tubular_handling', GFR=2e-06, Px=100, Ux=0, V=1.6666666666666667e-08)
# -> filtered_load = 120, excreted = 0, reabsorbed = 120   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_tubular_handling', GFR=2.0833333333333334e-06, Px=15, Ux=600, V=1.6666666666666667e-08)
# -> filtered_load = 18.75, excreted = 6, reabsorbed = 12.75   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| tubular reabsorption: GFR = 120 mL/min, Px = 100, Ux = 0, V = 1 mL/min | SOLVED | filtered_load = 120, excreted = 0, reabsorbed = 120 |
| tubular reabsorption: GFR = 125 mL/min, Px = 15, Ux = 600, V = 1 mL/min | SOLVED | filtered_load = 18.75, excreted = 6, reabsorbed = 12.75 |
| tubular reabsorption: Px = 100, Ux = 0, V = 1 mL/min | NEEDS_INPUT | hp_tubular_handling needs GFR (GFR (e.g. in mL/min)). Example: solve('hp_tubular_handling', GFR='2e-06 m^3/s', Px=100, U |
| tubular reabsorption: GFR = 120 mL/min, Px = 100, Ux = 0, V = 1 mL/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| tubular reabsorption: GFR = 0 mL/min, Px = 100, Ux = 0, V = 1 mL/min | INVALID_INPUT | Filtered load, excretion and net tubular reabsorption or secretion: GFR must be positive. |

<a id="hp_glucose_titration"></a>

### Glucose titration curve (ideal transport maximum), `hp_glucose_titration`

Filtered, reabsorbed (at most Tm) and excreted glucose and the plasma threshold Tm/GFR, ignoring splay.

$$
E = \max(0,\; GFR\cdot P_G - T_m)
$$

| input | meaning | condition |
|---|---|---|
| `GFR` | GFR (e.g. in mL/min) |  |
| `Pglucose` | plasma glucose in mg/dL |  |
| `Tm` | transport maximum (e.g. in mg/min) |  |

| output | meaning |
|---|---|
| `filtered_load` | filtered load in mg/min |
| `reabsorbed` | reabsorbed in mg/min |
| `excreted` | excreted in mg/min |
| `threshold` | plasma threshold in mg/dL |

- **Method:** reabsorbed = min(filtered, Tm)
- **Assumes:** GFR, $Tm > 0$; Pglucose >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![excreted (excreted in mg/min) against Tm (transport maximum (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-glucose-titration-curve)

*excreted (excreted in mg/min) against Tm (transport maximum (e.g. in mg/min)) from 6.2500e-07 kg/s to 6.2500e-05 kg/s on a logarithmic axis. One curve for each value of Pglucose (plasma glucose in mg/dL): 400, 200, 800. Other inputs as in the worked example: GFR = 2.0833333333333334e-06 m^3/s. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, Tm = 6.2500e-06 kg/s, excreted = 125.*

```python
human_physiology_ai999x.solve('hp_glucose_titration', GFR=2.0833333333333334e-06, Pglucose=400, Tm=6.25e-06)
# -> filtered_load = 500, reabsorbed = 375, excreted = 125, threshold = 300   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_glucose_titration', GFR=2e-06, Pglucose=100, Tm=6.25e-06)
# -> filtered_load = 120, reabsorbed = 120, excreted = 0, threshold = 312.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| glucose titration: GFR = 125 mL/min, Pglucose = 400, Tm = 375 mg/min | SOLVED | filtered_load = 500, reabsorbed = 375, excreted = 125, threshold = 300 |
| glucose titration: GFR = 120 mL/min, Pglucose = 100, Tm = 375 mg/min | SOLVED | filtered_load = 120, reabsorbed = 120, excreted = 0, threshold = 312.5 |
| glucose titration: Pglucose = 400, Tm = 375 mg/min | NEEDS_INPUT | hp_glucose_titration needs GFR (GFR (e.g. in mL/min)). Example: solve('hp_glucose_titration', GFR='2.08333e-06 m^3/s', P |
| glucose titration: GFR = 125 mL/min, Pglucose = 400, Tm = 375 mg/min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| glucose titration: GFR = 125 mL/min, Pglucose = 400, Tm = 0 mg/min | INVALID_INPUT | Glucose titration curve (ideal transport maximum): Tm must be positive. |

<a id="hp_plasma_osmolality"></a>

### Calculated and effective plasma osmolality and the osmolal gap, `hp_plasma_osmolality`

Calculated osmolality 2 Na + glucose/18 + BUN/2.8, effective osmolality (tonicity) 2 Na + glucose/18 and, with a measured value, the osmolal gap.

$$
P_{osm} = 2[Na^+] + \frac{glucose}{18} + \frac{BUN}{2.8}
$$

| input | meaning | condition |
|---|---|---|
| `Na` | sodium (e.g. in mmol/L) |  |
| `glucose` | glucose in mg/dL |  |
| `BUN` | blood urea nitrogen in mg/dL |  |
| `measured` | measured osmolality in mOsm/kg (optional) |  |

| output | meaning |
|---|---|
| `calculated` | calculated osmolality in mOsm/kg |
| `effective` | effective osmolality in mOsm/kg |
| `osmolal_gap` | measured - calculated in mOsm/kg |

- **Method:** published divisors 18 (glucose) and 2.8 (urea nitrogen)
- **Assumes:** $Na > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![calculated (calculated osmolality in mOsm/kg) against Na (sodium (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-plasma-osmolality-curve)

*calculated (calculated osmolality in mOsm/kg) against Na (sodium (e.g. in mmol/L)) from 70 mol/m^3 to 210 mol/m^3. Other inputs as in the worked example: glucose = 90.0, BUN = 14.0. 31 points computed by human-physiology-ai999x; marked: the worked example, Na = 140 mol/m^3, calculated = 290.*

![calculated (calculated osmolality in mOsm/kg) over Na (sodium (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-plasma-osmolality-field)

*calculated (calculated osmolality in mOsm/kg) over Na (sodium (e.g. in mmol/L)) from 70 mol/m^3 to 210 mol/m^3 and glucose (glucose in mg/dL) from 45 to 135: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 147.5 to dark 432.5); iso-lines at calculated = 150, 200, 250, 300, 350, 400. Other inputs as in the worked example: BUN = 14.0. Marked: the worked example, Na = 140 mol/m^3, glucose = 90, calculated = 290.*

```python
human_physiology_ai999x.solve('hp_plasma_osmolality', Na=140.0, glucose=90, BUN=14)
# -> calculated = 290, effective = 285   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_plasma_osmolality', Na=135.0, glucose=180, BUN=28, measured=300)
# -> calculated = 290, effective = 280, osmolal_gap = 10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| plasma osmolality: Na = 140 mmol/L, glucose = 90, BUN = 14 | SOLVED | calculated = 290, effective = 285 |
| plasma osmolality: Na = 135 mmol/L, glucose = 180, BUN = 28, measured = 300 | SOLVED | calculated = 290, effective = 280, osmolal_gap = 10 |
| plasma osmolality: glucose = 90, BUN = 14 | NEEDS_INPUT | hp_plasma_osmolality needs Na (sodium (e.g. in mmol/L)). Example: solve('hp_plasma_osmolality', Na='140 mol/m^3', glucos |
| plasma osmolality: Na = 140 mmol/L, glucose = 90, BUN = 14, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| plasma osmolality: Na = 0 mmol/L, glucose = 90, BUN = 14 | INVALID_INPUT | Calculated and effective plasma osmolality and the osmolal gap: Na must be positive. |

<a id="hp_body_fluid_compartments"></a>

### Body fluid compartments: `hp_body_fluid_compartments`

Total body water (60 % of weight by default), intracellular $2/3$, extracellular $1/3$, interstitial $3/4$ and plasma $1/4$ of the ECF.

$$
TBW = 0.6\,W,\; ICF = \tfrac23 TBW,\; ECF = \tfrac13 TBW
$$

| input | meaning | condition |
|---|---|---|
| `weight` | body weight (e.g. in kg) |  |
| `TBW_fraction` | total body water fraction of weight (optional; the textbook adult fraction if omitted) |  |

| output | meaning |
|---|---|
| `TBW` | total body water in L |
| `ICF` | intracellular fluid in L |
| `ECF` | extracellular fluid in L |
| `ISF` | interstitial fluid in L |
| `plasma` | plasma volume in L |

- **Method:** $60-40-20$ rule: ICF $2/3$, ECF $1/3$ of TBW; plasma $1/4$ of ECF
- **Assumes:** weight > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![TBW (total body water in L) against weight (body weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-body-fluid-compartments-curve)

*TBW (total body water in L) against weight (body weight (e.g. in kg)) from 35 kg to 105 kg. 31 points computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, TBW = 42.*

```python
human_physiology_ai999x.solve('hp_body_fluid_compartments', weight=70.0)
# -> TBW = 42, ICF = 28, ECF = 14, ISF = 10.5, plasma = 3.5   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_body_fluid_compartments', weight=60.0, TBW_fraction=0.5)
# -> TBW = 30, ICF = 20, ECF = 10, ISF = 7.5, plasma = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| body fluid compartments: weight = 70 kg | SOLVED | TBW = 42, ICF = 28, ECF = 14, ISF = 10.5, plasma = 3.5 |
| body fluid compartments: weight = 60 kg, TBW_fraction = 0.5 | SOLVED | TBW = 30, ICF = 20, ECF = 10, ISF = 7.5, plasma = 2.5 |
| body fluid compartments:  | NEEDS_INPUT | hp_body_fluid_compartments needs weight (body weight (e.g. in kg)). Example: solve('hp_body_fluid_compartments', weight= |
| body fluid compartments: weight = 70 kg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| body fluid compartments: weight = 0 kg | INVALID_INPUT | Body fluid compartments: weight must be positive. |

<a id="hp_watson_total_body_water"></a>

### Total body water by the Watson equations: `hp_watson_total_body_water`

Watson (1980): male $2.447 - 0.09516$ age + 0.1074 $H + 0.3362$ W; female -2.097 + 0.1069 $H + 0.2466$ W (litres).

$$
TBW_{m} = 2.447 - 0.09516\,A + 0.1074\,H + 0.3362\,W
$$

| input | meaning | condition |
|---|---|---|
| `age` | age in years |  |
| `height` | height (e.g. in cm) |  |
| `weight` | weight (e.g. in kg) |  |
| `sex` | sex for the equation |  |

| output | meaning |
|---|---|
| `TBW` | total body water in L |

- **Method:** Watson anthropometric regression
- **Assumes:** adult; height, weight > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![TBW (total body water in L) against height (height (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-watson-total-body-water-curve)

*TBW (total body water in L) against height (height (e.g. in cm)) from 0.875 m to 2.625 m. One curve for each value of weight (weight (e.g. in kg)): 70 kg, 35 kg, 140 kg. Other inputs as in the worked example: age = 40.0, sex = male. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, height = 1.75 m, TBW = 40.9696.*

```python
human_physiology_ai999x.solve('hp_watson_total_body_water', age=40, height=1.75, weight=70.0, sex='male')
# -> TBW = 40.9696   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_watson_total_body_water', age=40, height=1.6500000000000001, weight=60.0, sex='female')
# -> TBW = 30.3375   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Watson total body water: age = 40, height = 175 cm, weight = 70 kg, sex = male | SOLVED | TBW = 40.9696 |
| Watson total body water: age = 40, height = 165 cm, weight = 60 kg, sex = female | SOLVED | TBW = 30.3375 |
| Watson total body water: height = 175 cm, weight = 70 kg, sex = male | NEEDS_INPUT | hp_watson_total_body_water needs age (age in years). Example: solve('hp_watson_total_body_water', age=40, height='1.75 m |
| Watson total body water: age = 40, height = 175 cm, weight = 70 kg, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Watson total body water: age = 10, height = 140 cm, weight = 35 kg, sex = male | INVALID_INPUT | Total body water by the Watson equations: the Watson equations are for adults. |

<a id="hp_solution_osmolarity"></a>

### Osmolarity of a solution: `hp_solution_osmolarity`

Molarity from a mass concentration and the osmolarity (particles per formula unit, times the osmotic coefficient).

$$
\text{osmolarity} = \phi\, i\, \frac{c}{M}
$$

| input | meaning | condition |
|---|---|---|
| `conc` | mass concentration (e.g. in g/L) |  |
| `molar_mass` | molar mass (e.g. in g/mol) |  |
| `i` | number of particles per formula unit (dissociation) |  |
| `phi` | osmotic coefficient (optional; ideal solution if omitted) |  |

| output | meaning |
|---|---|
| `molarity` | molarity in mmol/L |
| `osmolarity` | osmolarity in mOsm/L |

- **Method:** mOsm/L = 1000 c/M x i x phi
- **Assumes:** conc, molar mass > 0; $1  \leq  i  \leq  6$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![osmolarity (osmolarity in mOsm/L) against molar_mass (molar mass (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-solution-osmolarity-curve)

*osmolarity (osmolarity in mOsm/L) against molar_mass (molar mass (e.g. in g/mol)) from 0.02922 kg/mol to 0.08766 kg/mol. One curve for each value of conc (mass concentration (e.g. in g/L)): 9 kg/m^3, 4.5 kg/m^3, 18 kg/m^3. Other inputs as in the worked example: i = 2. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, molar_mass = 0.05844 kg/mol, osmolarity = 308.008.*

```python
human_physiology_ai999x.solve('hp_solution_osmolarity', conc=9.0, molar_mass=0.05844, i=2)
# -> molarity = 154.004, osmolarity = 308.008   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_solution_osmolarity', conc=50.0, molar_mass=0.180156, i=1)
# -> molarity = 277.537, osmolarity = 277.537   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| solution osmolarity: conc = 9 g/L, molar_mass = 58.44 g/mol, i = 2 | SOLVED | molarity = 154.004, osmolarity = 308.008 |
| solution osmolarity: conc = 50 g/L, molar_mass = 180.156 g/mol, i = 1 | SOLVED | molarity = 277.537, osmolarity = 277.537 |
| solution osmolarity: molar_mass = 58.44 g/mol, i = 2 | NEEDS_INPUT | hp_solution_osmolarity needs conc (mass concentration (e.g. in g/L)). Example: solve('hp_solution_osmolarity', conc='9 k |
| solution osmolarity: conc = 9 g/L, molar_mass = 58.44 g/mol, i = 2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| solution osmolarity: conc = 9 g/L, molar_mass = 0 g/mol, i = 2 | INVALID_INPUT | Osmolarity of a solution: molar_mass must be positive. |

<a id="hp_osmotic_pressure"></a>

### Osmotic pressure by van 't Hoff's law: `hp_osmotic_pressure`

$pi = c$ R T for an osmolarity c, in Pa, mmHg and atm (about 19.3 mmHg per mOsm/L at 37 degC).

$$
\pi = c R T
$$

| input | meaning | condition |
|---|---|---|
| `osm` | osmolarity (or osmolarity difference) in mOsm/L |  |
| `T` | temperature (e.g. in degC) (optional; body temperature if omitted) |  |

| output | meaning |
|---|---|
| `pi` | osmotic pressure in Pa |
| `pi_mmHg` | osmotic pressure in mmHg |
| `pi_atm` | osmotic pressure in atm |

- **Method:** $pi = c$ R T (ideal dilute solution)
- **Assumes:** osm >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pi_mmHg (osmotic pressure in mmHg) against osm (osmolarity (or osmolarity difference) in mOsm/L) from 150 to 450](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-osmotic-pressure-curve)

*pi_mmHg (osmotic pressure in mmHg) against osm (osmolarity (or osmolarity difference) in mOsm/L) from 150 to 450. 31 points computed by human-physiology-ai999x; marked: the worked example, osm = 300, pi_mmHg = 5802.62.*

```python
human_physiology_ai999x.solve('hp_osmotic_pressure', osm=300)
# -> pi = 773619, pi_mmHg = 5802.62, pi_atm = 7.63503   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_osmotic_pressure', osm=10, T=298.15)
# -> pi = 24789.6, pi_mmHg = 185.937, pi_atm = 0.244654   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| osmotic pressure: osm = 300 | SOLVED | pi = 773619, pi_mmHg = 5802.62, pi_atm = 7.63503 |
| osmotic pressure: osm = 10, T = 25 degC | SOLVED | pi = 24789.6, pi_mmHg = 185.937, pi_atm = 0.244654 |
| osmotic pressure:  | NEEDS_INPUT | hp_osmotic_pressure needs osm (osmolarity (or osmolarity difference) in mOsm/L). Example: solve('hp_osmotic_pressure', o |
| osmotic pressure: osm = 300, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| osmotic pressure: osm = -5 | INVALID_INPUT | Osmotic pressure by van 't Hoff's law: osm must be zero or positive. |

<a id="hp_fluid_shift"></a>

### Two-compartment fluid shift after adding a solution (Darrow-Yannet), `hp_fluid_shift`

New osmolality and intracellular and extracellular volumes at osmotic equilibrium after a volume of solution enters the ECF.

$$
P_{osm}' = \frac{TBW\,P_{osm} + V C}{TBW + V},\quad ICF' = \frac{ICF\,P_{osm}}{P_{osm}'}
$$

| input | meaning | condition |
|---|---|---|
| `TBW` | initial total body water (e.g. in L) |  |
| `Posm` | initial osmolality in mOsm/kg |  |
| `V_added` | volume added to the ECF (e.g. in L) |  |
| `C_added` | osmolarity of the added solution in mOsm/L |  |
| `ICF_fraction` | initial intracellular fraction of total body water (optional; the textbook two thirds if omitted) |  |

| output | meaning |
|---|---|
| `osm_final` | final osmolality in mOsm/kg |
| `ICF` | final intracellular volume in L |
| `ECF` | final extracellular volume in L |

- **Method:** solute conservation in each compartment, equal osmolality, water conservation
- **Assumes:** TBW, Posm, V_added > 0; C_added >= 0; solute stays in the ECF, water moves freely
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![osm_final (final osmolality in mOsm/kg) against TBW (initial total body water (e](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-fluid-shift-osm-final-vs-tbw.svg)

*osm_final (final osmolality in mOsm/kg) against TBW (initial total body water (e.g. in L)), with C_added = 0, Posm = 290, V_added = 0.002 m^3. 31 points computed by human-physiology-ai999x; marked: TBW = 0.042, osm_final = 276.818.*

![osm_final (final osmolality in mOsm/kg) against TBW (initial total body water (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-fluid-shift-curve)

*osm_final (final osmolality in mOsm/kg) against TBW (initial total body water (e.g. in L)) from 0.0042 m^3 to 0.42 m^3 on a logarithmic axis. One curve for each value of Posm (initial osmolality in mOsm/kg): 290, 145, 580. Other inputs as in the worked example: V_added = 0.002 m^3, C_added = 0.0. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, TBW = 0.042 m^3, osm_final = 276.818.*

```python
human_physiology_ai999x.solve('hp_fluid_shift', TBW=0.042, Posm=290, V_added=0.002, C_added=0)
# -> osm_final = 276.818, ICF = 29.3333, ECF = 14.6667   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_fluid_shift', TBW=0.042, Posm=290, V_added=0.001, C_added=600)
# -> osm_final = 297.209, ICF = 27.3208, ECF = 15.6792   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| fluid shift: TBW = 42 L, Posm = 290, V_added = 2 L, C_added = 0 | SOLVED | osm_final = 276.818, ICF = 29.3333, ECF = 14.6667 |
| fluid shift: TBW = 42 L, Posm = 290, V_added = 1 L, C_added = 600 | SOLVED | osm_final = 297.209, ICF = 27.3208, ECF = 15.6792 |
| fluid shift: Posm = 290, V_added = 2 L, C_added = 0 | NEEDS_INPUT | hp_fluid_shift needs TBW (initial total body water (e.g. in L)). Example: solve('hp_fluid_shift', TBW='0.042 m^3', Posm= |
| fluid shift: TBW = 42 L, Posm = 290, V_added = 2 L, C_added = 0, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| fluid shift: TBW = 42 L, Posm = 290, V_added = 0 L, C_added = 0 | INVALID_INPUT | Two-compartment fluid shift after adding a solution (Darrow-Yannet): V_added must be positive. |

<a id="hp_gibbs_donnan"></a>

### Gibbs-Donnan equilibrium with an impermeant anion, `hp_gibbs_donnan`

Equilibrium NaCl distribution across a membrane when one side holds impermeant anions (charge concentration A) facing a large bath of NaCl c: concentrations, Donnan ratio, potential and osmotic excess.

$$
[Na^+]_i[Cl^-]_i = [Na^+]_o[Cl^-]_o,\quad [Na^+]_i = [Cl^-]_i + A
$$

| input | meaning | condition |
|---|---|---|
| `A` | impermeant anion charge concentration inside in mEq/L |  |
| `c` | bath NaCl concentration (e.g. in mmol/L) |  |
| `T` | temperature (e.g. in degC) (optional; body temperature if omitted) |  |

| output | meaning |
|---|---|
| `Na_in` | Na+ inside in mmol/L |
| `Cl_in` | Cl- inside in mmol/L |
| `r` | Donnan ratio Na_in/Na_out = Cl_out/Cl_in |
| `V_mV` | membrane potential inside relative to the bath in mV |
| `osmotic_excess` | extra diffusible ions inside in mmol/L |

- **Method:** Na_in $Cl_in = c^{2}$ with $Na_in = Cl_in + A$ (quadratic); V = -(RT/F) ln(Na_in/c)
- **Assumes:** A, $c > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Na_in (Na+ inside in mmol/L) against A (impermeant anion charge concentration inside in mEq/L) from 2 to 200 on a logarithmic axis](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-gibbs-donnan-curve)

*Na_in (Na+ inside in mmol/L) against A (impermeant anion charge concentration inside in mEq/L) from 2 to 200 on a logarithmic axis. One curve for each value of c (bath NaCl concentration (e.g. in mmol/L)): 150 mol/m^3, 75 mol/m^3, 300 mol/m^3. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, A = 20, Na_in = 160.333.*

![Na_in (Na+ inside in mmol/L) over c (bath NaCl concentration (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-gibbs-donnan-field)

*Na_in (Na+ inside in mmol/L) over c (bath NaCl concentration (e.g. in mmol/L)) from 75 mol/m^3 to 225 mol/m^3 and A (impermeant anion charge concentration inside in mEq/L) from 2 to 200 (logarithmic vertical axis): each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 76.0067 to dark 346.221); iso-lines at Na_in = 100, 150, 200, 250, 300. Marked: the worked example, c = 150 mol/m^3, A = 20, Na_in = 160.333.*

```python
human_physiology_ai999x.solve('hp_gibbs_donnan', A=20, c=150.0)
# -> Na_in = 160.333, Cl_in = 140.333, r = 1.06889, V_mV = -1.78046, osmotic_excess = 0.665928   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_gibbs_donnan', A=10, c=100.0, T=298.15)
# -> Na_in = 105.125, Cl_in = 95.1249, r = 1.05125, V_mV = -1.28409, osmotic_excess = 0.249844   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gibbs-Donnan equilibrium: A = 20, c = 150 mmol/L | SOLVED | Na_in = 160.333, Cl_in = 140.333, r = 1.06889, V_mV = -1.78046, osmotic_excess = 0.665928 |
| Gibbs-Donnan equilibrium: A = 10, c = 100 mmol/L, T = 25 degC | SOLVED | Na_in = 105.125, Cl_in = 95.1249, r = 1.05125, V_mV = -1.28409, osmotic_excess = 0.249844 |
| Gibbs-Donnan equilibrium: c = 150 mmol/L | NEEDS_INPUT | hp_gibbs_donnan needs A (impermeant anion charge concentration inside in mEq/L). Example: solve('hp_gibbs_donnan', A=20, |
| Gibbs-Donnan equilibrium: A = 20, c = 150 mmol/L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Gibbs-Donnan equilibrium: A = 0, c = 150 mmol/L | INVALID_INPUT | Gibbs-Donnan equilibrium with an impermeant anion: A must be positive. |

<a id="hp_nernst_potential"></a>

### Nernst equilibrium potential of an ion: `hp_nernst_potential`

$E = (RT/zF)$ ln(Cout/Cin) in mV for an ion of valence z.

$$
E = \frac{RT}{zF}\ln\frac{[C]_o}{[C]_i}
$$

| input | meaning | condition |
|---|---|---|
| `z` | ion valence (charge number, negative for anions) |  |
| `Cout` | extracellular concentration in mM |  |
| `Cin` | intracellular concentration in mM |  |
| `T` | temperature (e.g. in degC) (optional; body temperature if omitted) |  |

| output | meaning |
|---|---|
| `E` | equilibrium potential in mV |
| `RT_F` | RT/F in mV |

- **Method:** Nernst equation
- **Assumes:** z a nonzero whole number; concentrations > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![E (equilibrium potential in mV) against Cout (extracellular concentration in mM), with Cin = 140, z = 1](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-nernst-potential-e-vs-cout.svg)

*E (equilibrium potential in mV) against Cout (extracellular concentration in mM), with Cin = 140, z = 1. 31 points computed by human-physiology-ai999x; marked: Cout = 4, E = -95.0226.*

![E (equilibrium potential in mV) against Cout (extracellular concentration in mM) from 2 to 6](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-nernst-potential-curve)

*E (equilibrium potential in mV) against Cout (extracellular concentration in mM) from 2 to 6. One curve for each value of Cin (intracellular concentration in mM): 140, 70, 280. Other inputs as in the worked example: z = 1. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Cout = 4, E = -95.0226.*

```python
human_physiology_ai999x.solve('hp_nernst_potential', z=1, Cout=4, Cin=140)
# -> E = -95.0226, RT_F = 26.7267   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_nernst_potential', z=1, Cout=145, Cin=12)
# -> E = 66.5982, RT_F = 26.7267   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_nernst_potential', z=2, Cout=2.5, Cin=0.0001)
# -> E = 135.326, RT_F = 26.7267   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Nernst potential: z = 1, Cout = 4, Cin = 140 | SOLVED | E = -95.0226, RT_F = 26.7267 |
| Nernst potential: z = 1, Cout = 145, Cin = 12 | SOLVED | E = 66.5982, RT_F = 26.7267 |
| Nernst potential: z = 2, Cout = 2.5, Cin = 0.0001 | SOLVED | E = 135.326, RT_F = 26.7267 |
| Nernst potential: Cout = 4, Cin = 140 | NEEDS_INPUT | hp_nernst_potential needs z (ion valence (charge number, negative for anions)). Example: solve('hp_nernst_potential', z= |
| Nernst potential: z = 1, Cout = 4, Cin = 140, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Nernst potential: z = 0, Cout = 4, Cin = 140 | INVALID_INPUT | Nernst equilibrium potential of an ion: the valence must be a nonzero whole number from -3 to 3. |

<a id="hp_ghk_voltage"></a>

### Resting membrane potential by the Goldman-Hodgkin-Katz equation, `hp_ghk_voltage`

$Vm = (RT/F)$ ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o)) with relative permeabilities.

$$
V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}
$$

| input | meaning | condition |
|---|---|---|
| `PK` | relative K+ permeability |  |
| `PNa` | relative Na+ permeability |  |
| `PCl` | relative Cl- permeability |  |
| `K_o` | extracellular K+ in mM |  |
| `K_i` | intracellular K+ in mM |  |
| `Na_o` | extracellular Na+ in mM |  |
| `Na_i` | intracellular Na+ in mM |  |
| `Cl_o` | extracellular Cl- in mM |  |
| `Cl_i` | intracellular Cl- in mM |  |
| `T` | temperature (e.g. in degC) (optional; body temperature if omitted) |  |

| output | meaning |
|---|---|
| `Vm` | membrane potential in mV |

- **Method:** GHK voltage equation (chloride terms inverted)
- **Assumes:** $PK > 0$; PNa, PCl >= 0; concentrations > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vm (membrane potential in mV) against K_o (extracellular K+ in mM) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-ghk-voltage-curve)

*Vm (membrane potential in mV) against K_o (extracellular K+ in mM) from 0.4 to 40 on a logarithmic axis. One curve for each value of K_i (intracellular K+ in mM): 140, 70, 280. Other inputs as in the worked example: PK = 1.0, PNa = 0.04, PCl = 0.45, Na_o = 145.0, Na_i = 12.0, Cl_o = 116.0, Cl_i = 4.2. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, K_o = 4, Vm = -74.896.*

```python
human_physiology_ai999x.solve('hp_ghk_voltage', PK=1, PNa=0.04, PCl=0.45, K_o=4, K_i=140, Na_o=145, Na_i=12, Cl_o=116, Cl_i=4.2)
# -> Vm = -74.896   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ghk_voltage', PK=1, PNa=0.05, PCl=0, K_o=5, K_i=150, Na_o=145, Na_i=15, Cl_o=110, Cl_i=10)
# -> Vm = -67.0865   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| GHK voltage: PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4, K_i = 140, Na_o = 145, Na_i = 12, Cl_o = 116, Cl_i = 4.2 | SOLVED | Vm = -74.896 |
| GHK voltage: PK = 1, PNa = 0.05, PCl = 0, K_o = 5, K_i = 150, Na_o = 145, Na_i = 15, Cl_o = 110, Cl_i = 10 | SOLVED | Vm = -67.0865 |
| GHK voltage: PNa = 0.04, PCl = 0.45, K_o = 4, K_i = 140, Na_o = 145, Na_i = 12, Cl_o = 116, Cl_i = 4.2 | NEEDS_INPUT | hp_ghk_voltage needs PK (relative K+ permeability). Example: solve('hp_ghk_voltage', PK=1, PNa=0.04, PCl=0.45, K_o=4, K_ |
| GHK voltage: PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4, K_i = 140, Na_o = 145, Na_i = 12, Cl_o = 116, Cl_i = 4.2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| GHK voltage: PK = 0, PNa = 0.04, PCl = 0.45, K_o = 4, K_i = 140, Na_o = 145, Na_i = 12, Cl_o = 116, Cl_i = 4.2 | INVALID_INPUT | Resting membrane potential by the Goldman-Hodgkin-Katz equation: PK must be positive. |

<a id="hp_chord_conductance"></a>

### Membrane potential by the chord conductance equation, `hp_chord_conductance`

Steady potential at which the ohmic ionic currents cancel: Vm = (gK EK + gNa ENa + gCl ECl)/(gK + gNa + gCl).

$$
V_m = \frac{g_K E_K + g_{Na}E_{Na} + g_{Cl}E_{Cl}}{g_K + g_{Na} + g_{Cl}}
$$

| input | meaning | condition |
|---|---|---|
| `gK` | K+ conductance (any unit) |  |
| `gNa` | Na+ conductance (same unit) |  |
| `gCl` | Cl- conductance (same unit) |  |
| `EK` | K+ equilibrium potential (e.g. in mV) |  |
| `ENa` | Na+ equilibrium potential (e.g. in mV) |  |
| `ECl` | Cl- equilibrium potential (e.g. in mV) |  |

| output | meaning |
|---|---|
| `Vm` | membrane potential in mV |

- **Method:** conductance-weighted mean of the equilibrium potentials
- **Assumes:** conductances >= 0, not all zero
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vm (membrane potential in mV) against gNa (Na+ conductance (same unit)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-chord-conductance-curve)

*Vm (membrane potential in mV) against gNa (Na+ conductance (same unit)) from 0.005 to 0.5 on a logarithmic axis. One curve for each value of EK (K+ equilibrium potential (e.g. in mV)): -0.095 V, -0.0475 V, -0.19 V. Other inputs as in the worked example: gK = 1.0, gCl = 0.3, ENa = 0.067 V, ECl = -0.09 V. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, gNa = 0.05, Vm = -87.8889.*

```python
human_physiology_ai999x.solve('hp_chord_conductance', gK=1, gNa=0.05, gCl=0.3, EK=-0.095, ENa=0.067, ECl=-0.09)
# -> Vm = -87.8889   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_chord_conductance', gK=10, gNa=1, gCl=0, EK=-0.09, ENa=0.06, ECl=-0.07)
# -> Vm = -76.3636   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| chord conductance: gK = 1, gNa = 0.05, gCl = 0.3, EK = -95 mV, ENa = 67 mV, ECl = -90 mV | SOLVED | Vm = -87.8889 |
| chord conductance: gK = 10, gNa = 1, gCl = 0, EK = -90 mV, ENa = 60 mV, ECl = -70 mV | SOLVED | Vm = -76.3636 |
| chord conductance: gNa = 0.05, gCl = 0.3, EK = -95 mV, ENa = 67 mV, ECl = -90 mV | NEEDS_INPUT | hp_chord_conductance needs gK (K+ conductance (any unit)). Example: solve('hp_chord_conductance', gK=1, gNa=0.05, gCl=0. |
| chord conductance: gK = 1, gNa = 0.05, gCl = 0.3, EK = -95 mV, ENa = 67 mV, ECl = -90 mV, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| chord conductance: gK = 0, gNa = 0, gCl = 0, EK = -95 mV, ENa = 67 mV, ECl = -90 mV | INVALID_INPUT | Membrane potential by the chord conductance equation: at least one conductance must be positive. |

<a id="hp_ionic_current"></a>

### Ionic current and driving force: `hp_ionic_current`

Ohmic ionic current $I = g$ (Vm - E) (positive = outward) and the driving force Vm - E.

$$
I = g\,(V_m - E)
$$

| input | meaning | condition |
|---|---|---|
| `g` | conductance (e.g. in nS) |  |
| `Vm` | membrane potential (e.g. in mV) |  |
| `E` | equilibrium potential of the ion (e.g. in mV) |  |

| output | meaning |
|---|---|
| `I` | current in pA (positive outward) |
| `driving_force` | driving force in mV |

- **Method:** $I = g$ (Vm - E); nS x $mV = pA$
- **Assumes:** $g  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![I (current in pA (positive outward)) against g (conductance (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-ionic-current-curve)

*I (current in pA (positive outward)) against g (conductance (e.g. in nS)) from 5.0000e-09 S to 1.5000e-08 S. One curve for each value of E (equilibrium potential of the ion (e.g. in mV)): -0.095 V, -0.0475 V, -0.19 V. Other inputs as in the worked example: Vm = -0.07 V. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, g = 1.0000e-08 S, I = 250.*

```python
human_physiology_ai999x.solve('hp_ionic_current', g=1e-08, Vm=-0.07, E=-0.095)
# -> I = 250, driving_force = 25   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ionic_current', g=2e-09, Vm=-0.07, E=0.06)
# -> I = -260, driving_force = -130   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ionic current: g = 10 nS, Vm = -70 mV, E = -95 mV | SOLVED | I = 250, driving_force = 25 |
| ionic current: g = 2 nS, Vm = -70 mV, E = 60 mV | SOLVED | I = -260, driving_force = -130 |
| ionic current: Vm = -70 mV, E = -95 mV | NEEDS_INPUT | hp_ionic_current needs g (conductance (e.g. in nS)). Example: solve('hp_ionic_current', g='1e-08 S', Vm='-0.07 V', E='-0 |
| ionic current: g = 10 nS, Vm = -70 mV, E = -95 mV, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| ionic current: g = -1 nS, Vm = -70 mV, E = -95 mV | INVALID_INPUT | Ionic current and driving force: g must be zero or positive. |

<a id="hp_membrane_time_constant"></a>

### Membrane time constant and charging: `hp_membrane_time_constant`

$tau_m = Rm$ Cm (specific membrane resistance times capacitance) and the fraction of the final voltage reached after t.

$$
\tau_m = R_m C_m
$$

| input | meaning | condition |
|---|---|---|
| `Rm` | specific membrane resistance in ohm $cm^{2}$ |  |
| `Cm` | specific membrane capacitance (e.g. in uF/cm^2) |  |
| `t` | time (e.g. in ms) (optional; needed for the fraction charged) |  |

| output | meaning |
|---|---|
| `tau` | membrane time constant in ms |
| `fraction` | fraction of the final voltage at time t |

- **Method:** $\tau = Rm$ Cm; V(t)/$V_{inf} = $1 - e$^{\frac{-t}{\tau}}$
- **Assumes:** Rm, $Cm > 0$; $t  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![tau (membrane time constant in ms) against Rm (specific membrane resistance in ohm cm^2) from 10000 to 30000](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-membrane-time-constant-curve)

*tau (membrane time constant in ms) against Rm (specific membrane resistance in ohm cm^2) from 10000 to 30000. One curve for each value of Cm (specific membrane capacitance (e.g. in uF/cm^2)): 0.01 F/m^2, 0.005 F/m^2, 0.02 F/m^2. Other inputs as in the worked example: t = 0.01 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Rm = 20000, tau = 20.*

```python
human_physiology_ai999x.solve('hp_membrane_time_constant', Rm=20000, Cm=0.009999999999999998, t=0.01)
# -> tau = 20, fraction = 0.393469   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_membrane_time_constant', Rm=5000, Cm=0.019999999999999997, t=0.005)
# -> tau = 10, fraction = 0.393469   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_membrane_time_constant', Rm=20000, Cm=0.009999999999999998)
# -> tau = 20   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| membrane time constant: Rm = 20000, Cm = 1 uF/cm^2, t = 10 ms | SOLVED | tau = 20, fraction = 0.393469 |
| membrane time constant: Rm = 5000, Cm = 2 uF/cm^2, t = 5 ms | SOLVED | tau = 10, fraction = 0.393469 |
| membrane time constant: Rm = 20000, Cm = 1 uF/cm^2 | SOLVED | tau = 20 |
| membrane time constant: Cm = 1 uF/cm^2, t = 10 ms | NEEDS_INPUT | hp_membrane_time_constant needs Rm (specific membrane resistance in ohm cm^2). Example: solve('hp_membrane_time_constant |
| membrane time constant: Rm = 20000, Cm = 1 uF/cm^2, t = 10 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| membrane time constant: Rm = 0, Cm = 1 uF/cm^2, t = 10 ms | INVALID_INPUT | Membrane time constant and charging: Rm must be positive. |

<a id="hp_length_constant"></a>

### Length constant of an axon and steady-state attenuation, `hp_length_constant`

$\lambda = sqrt$(Rm d/(4 Ri)) of a passive fibre and the steady voltage fraction $e^{-x/\lambda}$ at distance x.

$$
\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0 e^{-x/\lambda}
$$

| input | meaning | condition |
|---|---|---|
| `Rm` | specific membrane resistance in ohm $cm^{2}$ |  |
| `Ri` | axoplasm resistivity in ohm cm |  |
| `d` | fibre diameter (e.g. in um) |  |
| `x` | distance from the injection site (e.g. in mm) (optional; needed for the attenuation) |  |

| output | meaning |
|---|---|
| `lambda` | length constant in mm |
| `attenuation` | steady-state fraction of the voltage remaining at distance x |

- **Method:** $\lambda = sqrt$(rm/ri)
- **Assumes:** Rm, Ri, $d > 0$; $x  \geq  0$; semi-infinite passive cable
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![lambda (length constant in mm) against Ri (axoplasm resistivity in ohm cm) from 50 to 150](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-length-constant-curve)

*lambda (length constant in mm) against Ri (axoplasm resistivity in ohm cm) from 50 to 150. One curve for each value of Rm (specific membrane resistance in ohm cm^2): 20000, 10000, 40000. Other inputs as in the worked example: d = 9.999999999999999e-06 m, x = 0.001 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Ri = 100, lambda = 2.23607.*

```python
human_physiology_ai999x.solve('hp_length_constant', Rm=20000, Ri=100, d=9.999999999999999e-06, x=0.001)
# -> lambda = 2.23607, attenuation = 0.639407   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_length_constant', Rm=5000, Ri=200, d=2e-06, x=0.0005)
# -> lambda = 0.353553, attenuation = 0.243117   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_length_constant', Rm=20000, Ri=100, d=9.999999999999999e-06)
# -> lambda = 2.23607   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| length constant: Rm = 20000, Ri = 100, d = 10 um, x = 1 mm | SOLVED | lambda = 2.23607, attenuation = 0.639407 |
| length constant: Rm = 5000, Ri = 200, d = 2 um, x = 0.5 mm | SOLVED | lambda = 0.353553, attenuation = 0.243117 |
| length constant: Rm = 20000, Ri = 100, d = 10 um | SOLVED | lambda = 2.23607 |
| length constant: Ri = 100, d = 10 um, x = 1 mm | NEEDS_INPUT | hp_length_constant needs Rm (specific membrane resistance in ohm cm^2). Example: solve('hp_length_constant', Rm=20000, R |
| length constant: Rm = 20000, Ri = 100, d = 10 um, x = 1 mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| length constant: Rm = 20000, Ri = 0, d = 10 um, x = 1 mm | INVALID_INPUT | Length constant of an axon and steady-state attenuation: Ri must be positive. |

<a id="hp_conduction_velocity_scaling"></a>

### Conduction velocity scaling with fibre diameter, `hp_conduction_velocity_scaling`

Conduction velocity is proportional to diameter for myelinated fibres and to its square root for unmyelinated fibres.

$$
v \propto d \;(\text{myelinated}),\quad v \propto \sqrt{d}\;(\text{unmyelinated})
$$

| input | meaning | condition |
|---|---|---|
| `v1` | known conduction velocity (e.g. in m/s) |  |
| `d1` | diameter of that fibre (e.g. in um) |  |
| `d2` | diameter of the other fibre (e.g. in um) |  |
| `fiber` | fibre type |  |

| output | meaning |
|---|---|
| `v2` | conduction velocity of the other fibre in m/s |
| `ratio` | ratio of the new to the known velocity |

- **Method:** v ~ d (myelinated), v ~ sqrt(d) (unmyelinated, cable theory)
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![v2 (conduction velocity of the other fibre in m/s) against d1 (diameter of that fibre (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-conduction-velocity-scaling-curve)

*v2 (conduction velocity of the other fibre in m/s) against d1 (diameter of that fibre (e.g. in um)) from 5.0000e-06 m to 1.5000e-05 m. One curve for each value of v1 (known conduction velocity (e.g. in m/s)): 60 m/s, 30 m/s, 120 m/s. Other inputs as in the worked example: d2 = 4.9999999999999996e-06 m, fiber = myelinated. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, d1 = 1.0000e-05 m, v2 = 30.*

```python
human_physiology_ai999x.solve('hp_conduction_velocity_scaling', v1=60.0, d1=9.999999999999999e-06, d2=4.9999999999999996e-06, fiber='myelinated')
# -> v2 = 30, ratio = 0.5   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_conduction_velocity_scaling', v1=1.0, d1=1e-06, d2=4e-06, fiber='unmyelinated')
# -> v2 = 2, ratio = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| conduction velocity scaling: v1 = 60 m/s, d1 = 10 um, d2 = 5 um, fiber = myelinated | SOLVED | v2 = 30, ratio = 0.5 |
| conduction velocity scaling: v1 = 1 m/s, d1 = 1 um, d2 = 4 um, fiber = unmyelinated | SOLVED | v2 = 2, ratio = 2 |
| conduction velocity scaling: d1 = 10 um, d2 = 5 um, fiber = myelinated | NEEDS_INPUT | hp_conduction_velocity_scaling needs v1 (known conduction velocity (e.g. in m/s)). Example: solve('hp_conduction_velocit |
| conduction velocity scaling: v1 = 60 m/s, d1 = 10 um, d2 = 5 um, fiber = myelinated, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| conduction velocity scaling: v1 = 60 m/s, d1 = 0 um, d2 = 5 um, fiber = myelinated | INVALID_INPUT | Conduction velocity scaling with fibre diameter: d1 must be positive. |

<a id="hp_nerve_conduction_velocity"></a>

### Nerve conduction velocity from two stimulation sites, `hp_nerve_conduction_velocity`

NCV = distance between stimulation sites / difference of the response latencies $(mm/ms = m/s)$.

$$
NCV = \frac{d}{t_p - t_d}
$$

| input | meaning | condition |
|---|---|---|
| `distance` | distance between stimulation sites (e.g. in mm) |  |
| `latency_proximal` | latency from the proximal site (e.g. in ms) |  |
| `latency_distal` | latency from the distal site (e.g. in ms) |  |

| output | meaning |
|---|---|
| `NCV` | conduction velocity in m/s |

- **Method:** NCV = d / (t_p - t_d)
- **Assumes:** latency_proximal > latency_distal
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![NCV (conduction velocity in m/s) against latency_proximal (latency from the proximal site (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-nerve-conduction-velocity-curve)

*NCV (conduction velocity in m/s) against latency_proximal (latency from the proximal site (e.g. in ms)) from 0.00425 s to 0.01275 s. One curve for each value of distance (distance between stimulation sites (e.g. in mm)): 0.25 m, 0.125 m, 0.5 m. Other inputs as in the worked example: latency_distal = 0.0035 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, latency_proximal = 0.0085 s, NCV = 50.*

```python
human_physiology_ai999x.solve('hp_nerve_conduction_velocity', distance=0.25, latency_proximal=0.0085, latency_distal=0.0035)
# -> NCV = 50   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_nerve_conduction_velocity', distance=0.3, latency_proximal=0.009000000000000001, latency_distal=0.004)
# -> NCV = 60   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| nerve conduction velocity: distance = 250 mm, latency_proximal = 8.5 ms, latency_distal = 3.5 ms | SOLVED | NCV = 50 |
| nerve conduction velocity: distance = 300 mm, latency_proximal = 9 ms, latency_distal = 4 ms | SOLVED | NCV = 60 |
| nerve conduction velocity: latency_proximal = 8.5 ms, latency_distal = 3.5 ms | NEEDS_INPUT | hp_nerve_conduction_velocity needs distance (distance between stimulation sites (e.g. in mm)). Example: solve('hp_nerve_ |
| nerve conduction velocity: distance = 250 mm, latency_proximal = 8.5 ms, latency_distal = 3.5 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| nerve conduction velocity: distance = 250 mm, latency_proximal = 3.5 ms, latency_distal = 8.5 ms | INVALID_INPUT | Nerve conduction velocity from two stimulation sites: the proximal latency must exceed the distal latency. |

<a id="hp_membrane_charge"></a>

### Charge and number of ions to change the membrane potential, `hp_membrane_charge`

$Q = Cm$ A dV and the number of monovalent ions it represents (why few ions cross in an action potential).

$$
Q = C_m A \Delta V,\quad N = \frac{|Q|}{e}
$$

| input | meaning | condition |
|---|---|---|
| `Cm` | specific membrane capacitance (e.g. in uF/cm^2) |  |
| `dV` | voltage change (e.g. in mV) |  |
| `area` | membrane area (e.g. in um^2) |  |

| output | meaning |
|---|---|
| `Q_pC` | charge in pC |
| `ions` | number of monovalent ions |

- **Method:** $Q = C$ A dV; ions = |Q|/e
- **Assumes:** Cm, area > 0; dV not 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![ions (number of monovalent ions) against Cm (specific membrane capacitance (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-membrane-charge-curve)

*ions (number of monovalent ions) against Cm (specific membrane capacitance (e.g. in uF/cm^2)) from 0.005 F/m^2 to 0.015 F/m^2. One curve for each value of dV (voltage change (e.g. in mV)): 0.1 V, 0.05 V, 0.2 V. Other inputs as in the worked example: area = 1e-09 m^2. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, Cm = 0.01 F/m^2, ions = 6.24151e+06.*

```python
human_physiology_ai999x.solve('hp_membrane_charge', Cm=0.009999999999999998, dV=0.1, area=1e-09)
# -> Q_pC = 1, ions = 6.24151e+06   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_membrane_charge', Cm=0.009, dV=0.08, area=5e-10)
# -> Q_pC = 0.36, ions = 2.24694e+06   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| membrane charge: Cm = 1 uF/cm^2, dV = 100 mV, area = 1000 um^2 | SOLVED | Q_pC = 1, ions = 6.24151e+06 |
| membrane charge: Cm = 0.9 uF/cm^2, dV = 80 mV, area = 500 um^2 | SOLVED | Q_pC = 0.36, ions = 2.24694e+06 |
| membrane charge: dV = 100 mV, area = 1000 um^2 | NEEDS_INPUT | hp_membrane_charge needs Cm (specific membrane capacitance (e.g. in uF/cm^2)). Example: solve('hp_membrane_charge', Cm=' |
| membrane charge: Cm = 1 uF/cm^2, dV = 100 mV, area = 1000 um^2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| membrane charge: Cm = 1 uF/cm^2, dV = 100 mV, area = 0 um^2 | INVALID_INPUT | Charge and number of ions to change the membrane potential: area must be positive. |

<a id="hp_strength_duration"></a>

### Threshold current for a pulse width (strength-duration curve), `hp_strength_duration`

Threshold current for a pulse of width t from rheobase and chronaxie: Lapicque (exponential RC) and Weiss (linear charge) forms.

$$
I = \frac{I_{rh}}{1 - 2^{-t/c}}\;(\text{Lapicque}),\quad I = I_{rh}\left(1 + \frac{c}{t}\right)\;(\text{Weiss})
$$

| input | meaning | condition |
|---|---|---|
| `rheobase` | rheobase current (any unit, e.g. mA) |  |
| `chronaxie` | chronaxie (e.g. in ms) |  |
| `t` | pulse width (e.g. in ms) |  |

| output | meaning |
|---|---|
| `I_lapicque` | threshold current by the Lapicque (exponential) law, in the unit of the rheobase |
| `I_weiss` | threshold current by the Weiss (linear) law, in the unit of the rheobase |

- **Method:** Lapicque: tau = chronaxie/ln 2; Weiss: linear charge law
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![I_lapicque (threshold current by the Lapicque (exponential) law, in the unit of the rheobase) against t (pulse width (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-strength-duration-curve)

*I_lapicque (threshold current by the Lapicque (exponential) law, in the unit of the rheobase) against t (pulse width (e.g. in ms)) from 5.0000e-05 s to 0.005 s on a logarithmic axis. One curve for each value of rheobase (rheobase current (any unit, e.g. mA)): 1, 0.5, 2. Other inputs as in the worked example: chronaxie = 0.0005 s. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, t = 0.0005 s, I_lapicque = 2.*

```python
human_physiology_ai999x.solve('hp_strength_duration', rheobase=1, chronaxie=0.0005, t=0.0005)
# -> I_lapicque = 2, I_weiss = 2   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_strength_duration', rheobase=1, chronaxie=0.0005, t=0.0001)
# -> I_lapicque = 7.72502, I_weiss = 6   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_strength_duration', rheobase=0.4, chronaxie=0.0003, t=0.001)
# -> I_lapicque = 0.444056, I_weiss = 0.52   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| strength-duration threshold: rheobase = 1, chronaxie = 0.5 ms, t = 0.5 ms | SOLVED | I_lapicque = 2, I_weiss = 2 |
| strength-duration threshold: rheobase = 1, chronaxie = 0.5 ms, t = 0.1 ms | SOLVED | I_lapicque = 7.72502, I_weiss = 6 |
| strength-duration threshold: rheobase = 0.4, chronaxie = 0.3 ms, t = 1 ms | SOLVED | I_lapicque = 0.444056, I_weiss = 0.52 |
| strength-duration threshold: chronaxie = 0.5 ms, t = 0.5 ms | NEEDS_INPUT | hp_strength_duration needs rheobase (rheobase current (any unit, e.g. mA)). Example: solve('hp_strength_duration', rheob |
| strength-duration threshold: rheobase = 1, chronaxie = 0.5 ms, t = 0.5 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| strength-duration threshold: rheobase = 1, chronaxie = 0 ms, t = 0.5 ms | INVALID_INPUT | Threshold current for a pulse width (strength-duration curve): chronaxie must be positive. |

<a id="hp_hill_force_velocity"></a>

### Hill muscle force-velocity relation: `hp_hill_force_velocity`

Hill's equation (F + a)$(v + b) = (F_{0} + a)$ b: force at shortening velocity v, the power F v and vmax = b $F_{0}/a.$

$$
(F + a)(v + b) = (F_0 + a)\,b
$$

| input | meaning | condition |
|---|---|---|
| `F0` | isometric force (e.g. in N) |  |
| `a` | Hill constant a (e.g. in N) |  |
| `b` | Hill constant b (e.g. in m/s) |  |
| `v` | shortening velocity (e.g. in m/s) |  |

| output | meaning |
|---|---|
| `F` | force in N |
| `power` | power in W |
| `vmax` | maximal shortening velocity in m/s |

- **Method:** F = (F0 b - a v)/(v + b)
- **Assumes:** F0, a, $b > 0$; 0 <= v <= vmax
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![F (force in N) against b (Hill constant b (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hill-force-velocity-curve)

*F (force in N) against b (Hill constant b (e.g. in m/s)) from 0.03 m/s to 3 m/s on a logarithmic axis. One curve for each value of F0 (isometric force (e.g. in N)): 100 N, 50 N, 200 N. Other inputs as in the worked example: a = 25.0 N, v = 0.1 m/s. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, b = 0.3 m/s, F = 68.75.*

```python
human_physiology_ai999x.solve('hp_hill_force_velocity', F0=100.0, a=25.0, b=0.3, v=0.1)
# -> F = 68.75, power = 6.875, vmax = 1.2   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hill_force_velocity', F0=400.0, a=80.0, b=0.5, v=0.3)
# -> F = 220, power = 66, vmax = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hill force-velocity: F0 = 100 N, a = 25 N, b = 0.3 m/s, v = 0.1 m/s | SOLVED | F = 68.75, power = 6.875, vmax = 1.2 |
| Hill force-velocity: F0 = 400 N, a = 80 N, b = 0.5 m/s, v = 0.3 m/s | SOLVED | F = 220, power = 66, vmax = 2.5 |
| Hill force-velocity: a = 25 N, b = 0.3 m/s, v = 0.1 m/s | NEEDS_INPUT | hp_hill_force_velocity needs F0 (isometric force (e.g. in N)). Example: solve('hp_hill_force_velocity', F0='100 N', a='2 |
| Hill force-velocity: F0 = 100 N, a = 25 N, b = 0.3 m/s, v = 0.1 m/s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Hill force-velocity: F0 = 100 N, a = 25 N, b = 0.3 m/s, v = 5 m/s | INVALID_INPUT | Hill muscle force-velocity relation: the shortening velocity cannot exceed vmax = b F0/a. |

<a id="hp_muscle_max_power"></a>

### Maximal power of a muscle from Hill's equation, `hp_muscle_max_power`

The shortening velocity of maximal power v_opt = b (sqrt(1 + F0/a) - 1), the force there and the maximal power.

$$
v_{opt} = b\left(\sqrt{1 + F_0/a} - 1\right)
$$

| input | meaning | condition |
|---|---|---|
| `F0` | isometric force (e.g. in N) |  |
| `a` | Hill constant a (e.g. in N) |  |
| `b` | Hill constant b (e.g. in m/s) |  |

| output | meaning |
|---|---|
| `v_opt` | velocity of maximal power in m/s |
| `F_opt` | force at maximal power in N |
| `P_max` | maximal power in W |
| `vmax` | maximal shortening velocity in m/s |

- **Method:** $dP/dv = 0$ for $P = v$ (F0 b - a v)/(v + b)
- **Assumes:** F0, a, $b > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P_max (maximal power in W) against F0 (isometric force (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-muscle-max-power-curve)

*P_max (maximal power in W) against F0 (isometric force (e.g. in N)) from 10 N to 1000 N on a logarithmic axis. One curve for each value of b (Hill constant b (e.g. in m/s)): 0.3 m/s, 0.15 m/s, 0.6 m/s. Other inputs as in the worked example: a = 25.0 N. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, F0 = 100 N, P_max = 11.459.*

```python
human_physiology_ai999x.solve('hp_muscle_max_power', F0=100.0, a=25.0, b=0.3)
# -> v_opt = 0.37082, F_opt = 30.9017, P_max = 11.459, vmax = 1.2   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_muscle_max_power', F0=400.0, a=80.0, b=0.5)
# -> v_opt = 0.724745, F_opt = 115.959, P_max = 84.0408, vmax = 2.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| muscle maximal power: F0 = 100 N, a = 25 N, b = 0.3 m/s | SOLVED | v_opt = 0.37082, F_opt = 30.9017, P_max = 11.459, vmax = 1.2 |
| muscle maximal power: F0 = 400 N, a = 80 N, b = 0.5 m/s | SOLVED | v_opt = 0.724745, F_opt = 115.959, P_max = 84.0408, vmax = 2.5 |
| muscle maximal power: a = 25 N, b = 0.3 m/s | NEEDS_INPUT | hp_muscle_max_power needs F0 (isometric force (e.g. in N)). Example: solve('hp_muscle_max_power', F0='100 N', a='25 N',  |
| muscle maximal power: F0 = 100 N, a = 25 N, b = 0.3 m/s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| muscle maximal power: F0 = 100 N, a = 0 N, b = 0.3 m/s | INVALID_INPUT | Maximal power of a muscle from Hill's equation: a must be positive. |

<a id="hp_bmi"></a>

### Body mass index: `hp_bmi`

BMI = weight/height^2 $(kg/m^{2})$.

$$
BMI = \frac{W}{H^2}
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `height` | height (e.g. in cm) |  |

| output | meaning |
|---|---|
| `BMI` | body mass index in $kg/m^{2}$ |

- **Method:** BMI = W / H^2 (H in m)
- **Assumes:** weight, height > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BMI (body mass index in kg/m^2) against height (height (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmi-curve)

*BMI (body mass index in kg/m^2) against height (height (e.g. in cm)) from 0.875 m to 2.625 m. One curve for each value of weight (weight (e.g. in kg)): 70 kg, 35 kg, 140 kg. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, height = 1.75 m, BMI = 22.8571.*

![BMI (body mass index in kg/m^2) over height (height (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmi-field)

*BMI (body mass index in kg/m^2) over height (height (e.g. in cm)) from 0.875 m to 2.625 m and weight (weight (e.g. in kg)) from 35 kg to 105 kg: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 5.07937 to dark 137.143); iso-lines at BMI = 25, 50, 75, 100, 125. Marked: the worked example, height = 1.75 m, weight = 70 kg, BMI = 22.8571.*

```python
human_physiology_ai999x.solve('hp_bmi', weight=70.0, height=1.75)
# -> BMI = 22.8571   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmi', weight=95.0, height=1.82)
# -> BMI = 28.6801   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| body mass index: weight = 70 kg, height = 175 cm | SOLVED | BMI = 22.8571 |
| body mass index: weight = 95 kg, height = 182 cm | SOLVED | BMI = 28.6801 |
| body mass index: height = 175 cm | NEEDS_INPUT | hp_bmi needs weight (weight (e.g. in kg)). Example: solve('hp_bmi', weight='70 kg', height='1.75 m'). |
| body mass index: weight = 70 kg, height = 175 cm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| body mass index: weight = 70 kg, height = 0 cm | INVALID_INPUT | Body mass index: height must be positive. |

<a id="hp_body_surface_area"></a>

### Body surface area (DuBois, Mosteller, Haycock, Gehan-George), `hp_body_surface_area`

Body surface area by the DuBois (1916), Mosteller (1987), Haycock (1978) and Gehan-George (1970) formulas.

$$
BSA_{Mosteller} = \sqrt{\frac{H\,W}{3600}},\quad BSA_{DuBois} = 0.007184\,W^{0.425}H^{0.725}
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `height` | height (e.g. in cm) |  |

| output | meaning |
|---|---|
| `BSA_DuBois` | DuBois BSA in $m^{2}$ |
| `BSA_Mosteller` | Mosteller BSA in $m^{2}$ |
| `BSA_Haycock` | Haycock BSA in $m^{2}$ |
| `BSA_GehanGeorge` | Gehan-George BSA in $m^{2}$ |

- **Method:** published power-law formulas
- **Assumes:** weight, height > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BSA_Mosteller (Mosteller BSA in m^2) against weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-body-surface-area-curve)

*BSA_Mosteller (Mosteller BSA in m^2) against weight (weight (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of height (height (e.g. in cm)): 1.75 m, 0.875 m, 3.5 m. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, BSA_Mosteller = 1.84466.*

![BSA_Mosteller (Mosteller BSA in m^2) over weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-body-surface-area-field)

*BSA_Mosteller (Mosteller BSA in m^2) over weight (weight (e.g. in kg)) from 35 kg to 105 kg and height (height (e.g. in cm)) from 0.875 m to 2.625 m: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 0.922331 to dark 2.76699); iso-lines at BSA_Mosteller = 1, 1.5, 2, 2.5. Marked: the worked example, weight = 70 kg, height = 1.75 m, BSA_Mosteller = 1.84466.*

```python
human_physiology_ai999x.solve('hp_body_surface_area', weight=70.0, height=1.75)
# -> BSA_DuBois = 1.84814, BSA_Mosteller = 1.84466, BSA_Haycock = 1.84678, BSA_GehanGeorge = 1.85385   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_body_surface_area', weight=55.0, height=1.6)
# -> BSA_DuBois = 1.56317, BSA_Mosteller = 1.56347, BSA_Haycock = 1.56553, BSA_GehanGeorge = 1.57667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| body surface area: weight = 70 kg, height = 175 cm | SOLVED | BSA_DuBois = 1.84814, BSA_Mosteller = 1.84466, BSA_Haycock = 1.84678, BSA_GehanGeorge = 1.85385 |
| body surface area: weight = 55 kg, height = 160 cm | SOLVED | BSA_DuBois = 1.56317, BSA_Mosteller = 1.56347, BSA_Haycock = 1.56553, BSA_GehanGeorge = 1.57667 |
| body surface area: height = 175 cm | NEEDS_INPUT | hp_body_surface_area needs weight (weight (e.g. in kg)). Example: solve('hp_body_surface_area', weight='70 kg', height=' |
| body surface area: weight = 70 kg, height = 175 cm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| body surface area: weight = 0 kg, height = 175 cm | INVALID_INPUT | Body surface area (DuBois, Mosteller, Haycock, Gehan-George): weight must be positive. |

<a id="hp_bmr_harris_benedict"></a>

### Basal metabolic rate by the Harris-Benedict equation, `hp_bmr_harris_benedict`

Harris-Benedict BMR, original (1919) or revised by Roza and Shizgal (1984), in kcal/day and kJ/day (computed value).

$$
BMR_{m} = 66.473 + 13.7516\,W + 5.0033\,H - 6.755\,A
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `height` | height (e.g. in cm) |  |
| `age` | age in years |  |
| `sex` | sex for the coefficients |  |
| `version` | original equation or the revised (Roza-Shizgal) equation |  |

| output | meaning |
|---|---|
| `BMR` | basal metabolic rate in kcal/day |
| `BMR_kJ` | basal metabolic rate in kJ/day |

- **Method:** published linear coefficients
- **Assumes:** adult; weight, height > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BMR (basal metabolic rate in kcal/day) against weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmr-harris-benedict-curve)

*BMR (basal metabolic rate in kcal/day) against weight (weight (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of height (height (e.g. in cm)): 1.75 m, 0.875 m, 3.5 m. Other inputs as in the worked example: age = 30.0, sex = male, version = original. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, BMR = 1702.01.*

```python
human_physiology_ai999x.solve('hp_bmr_harris_benedict', weight=70.0, height=1.75, age=30, sex='male', version='original')
# -> BMR = 1702.01, BMR_kJ = 7121.22   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmr_harris_benedict', weight=70.0, height=1.75, age=30, sex='male', version='revised')
# -> BMR = 1695.67, BMR_kJ = 7094.67   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmr_harris_benedict', weight=60.0, height=1.6500000000000001, age=40, sex='female', version='original')
# -> BMR = 1347.06, BMR_kJ = 5636.1   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmr_harris_benedict', weight=60.0, height=1.6500000000000001, age=40, sex='female', version='revised')
# -> BMR = 1340.38, BMR_kJ = 5608.16   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Harris-Benedict BMR: weight = 70 kg, height = 175 cm, age = 30, sex = male, version = original | SOLVED | BMR = 1702.01, BMR_kJ = 7121.22 |
| Harris-Benedict BMR: weight = 70 kg, height = 175 cm, age = 30, sex = male, version = revised | SOLVED | BMR = 1695.67, BMR_kJ = 7094.67 |
| Harris-Benedict BMR: weight = 60 kg, height = 165 cm, age = 40, sex = female, version = original | SOLVED | BMR = 1347.06, BMR_kJ = 5636.1 |
| Harris-Benedict BMR: weight = 60 kg, height = 165 cm, age = 40, sex = female, version = revised | SOLVED | BMR = 1340.38, BMR_kJ = 5608.16 |
| Harris-Benedict BMR: height = 175 cm, age = 30, sex = male, version = original | NEEDS_INPUT | hp_bmr_harris_benedict needs weight (weight (e.g. in kg)). Example: solve('hp_bmr_harris_benedict', weight='70 kg', heig |
| Harris-Benedict BMR: weight = 70 kg, height = 175 cm, age = 30, sex = male, version = original, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Harris-Benedict BMR: weight = 70 kg, height = 175 cm, age = 10, sex = male, version = original | INVALID_INPUT | Basal metabolic rate by the Harris-Benedict equation: the equations are for adults. |

<a id="hp_bmr_mifflin_st_jeor"></a>

### Resting energy expenditure by the Mifflin-St Jeor equation, `hp_bmr_mifflin_st_jeor`

Mifflin-St Jeor (1990): 10 $W + 6.25$ $H - 5$ $A + 5$ (male) $or - 161$ (female), kcal/day (computed value).

$$
REE = 10W + 6.25H - 5A + s
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `height` | height (e.g. in cm) |  |
| `age` | age in years |  |
| `sex` | sex for the constant |  |

| output | meaning |
|---|---|
| `REE` | resting energy expenditure in kcal/day |
| `REE_kJ` | resting energy expenditure in kJ/day |

- **Method:** Mifflin-St Jeor
- **Assumes:** adult; weight, height > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![REE (resting energy expenditure in kcal/day) against weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmr-mifflin-st-jeor-curve)

*REE (resting energy expenditure in kcal/day) against weight (weight (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of height (height (e.g. in cm)): 1.75 m, 0.875 m, 3.5 m. Other inputs as in the worked example: age = 30.0, sex = male. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, REE = 1648.75.*

```python
human_physiology_ai999x.solve('hp_bmr_mifflin_st_jeor', weight=70.0, height=1.75, age=30, sex='male')
# -> REE = 1648.75, REE_kJ = 6898.37   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmr_mifflin_st_jeor', weight=60.0, height=1.6500000000000001, age=40, sex='female')
# -> REE = 1270.25, REE_kJ = 5314.73   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Mifflin-St Jeor: weight = 70 kg, height = 175 cm, age = 30, sex = male | SOLVED | REE = 1648.75, REE_kJ = 6898.37 |
| Mifflin-St Jeor: weight = 60 kg, height = 165 cm, age = 40, sex = female | SOLVED | REE = 1270.25, REE_kJ = 5314.73 |
| Mifflin-St Jeor: height = 175 cm, age = 30, sex = male | NEEDS_INPUT | hp_bmr_mifflin_st_jeor needs weight (weight (e.g. in kg)). Example: solve('hp_bmr_mifflin_st_jeor', weight='70 kg', heig |
| Mifflin-St Jeor: weight = 70 kg, height = 175 cm, age = 30, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Mifflin-St Jeor: weight = 0 kg, height = 175 cm, age = 30, sex = male | INVALID_INPUT | Resting energy expenditure by the Mifflin-St Jeor equation: weight must be positive. |

<a id="hp_bmr_katch_mcardle"></a>

### Basal metabolic rate by the Katch-McArdle equation, `hp_bmr_katch_mcardle`

Lean body mass W (1 - body fat) and BMR = 370 + 21.6 LBM in kcal/day (computed value).

$$
BMR = 370 + 21.6\,LBM
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `body_fat` | body fat fraction |  |

| output | meaning |
|---|---|
| `LBM` | lean body mass in kg |
| `BMR` | basal metabolic rate in kcal/day |

- **Method:** Katch-McArdle
- **Assumes:** weight > 0; 0 <= body fat < 1
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BMR (basal metabolic rate in kcal/day) against weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmr-katch-mcardle-curve)

*BMR (basal metabolic rate in kcal/day) against weight (weight (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of body_fat (body fat fraction): 0.2, 0.26, 0.74. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, BMR = 1579.6.*

![BMR (basal metabolic rate in kcal/day) over weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-bmr-katch-mcardle-field)

*BMR (basal metabolic rate in kcal/day) over weight (weight (e.g. in kg)) from 35 kg to 105 kg and body_fat (body fat fraction) from 0.02 to 0.98: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 385.12 to dark 2592.64); iso-lines at BMR = 500, 1000, 1500, 2000, 2500. Marked: the worked example, weight = 70 kg, body_fat = 0.2, BMR = 1579.6.*

```python
human_physiology_ai999x.solve('hp_bmr_katch_mcardle', weight=70.0, body_fat=0.2)
# -> LBM = 56, BMR = 1579.6   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_bmr_katch_mcardle', weight=80.0, body_fat=0.25)
# -> LBM = 60, BMR = 1666   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Katch-McArdle BMR: weight = 70 kg, body_fat = 0.2 | SOLVED | LBM = 56, BMR = 1579.6 |
| Katch-McArdle BMR: weight = 80 kg, body_fat = 0.25 | SOLVED | LBM = 60, BMR = 1666 |
| Katch-McArdle BMR: body_fat = 0.2 | NEEDS_INPUT | hp_bmr_katch_mcardle needs weight (weight (e.g. in kg)). Example: solve('hp_bmr_katch_mcardle', weight='70 kg', body_fat |
| Katch-McArdle BMR: weight = 70 kg, body_fat = 0.2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Katch-McArdle BMR: weight = 70 kg, body_fat = 1 | INVALID_INPUT | Basal metabolic rate by the Katch-McArdle equation: body_fat must be a fraction between 0 and 1. |

<a id="hp_met_energy"></a>

### Energy expenditure of an activity from METs, `hp_met_energy`

1 MET = 3.5 mL O2/kg/min; oxygen uptake MET x 3.5 x W and energy MET x 3.5 x $W/200$ kcal/min.

$$
\text{kcal/min} = \frac{MET \times 3.5 \times W}{200}
$$

| input | meaning | condition |
|---|---|---|
| `MET` | activity intensity in METs |  |
| `weight` | weight (e.g. in kg) |  |
| `duration` | duration (e.g. in min) |  |

| output | meaning |
|---|---|
| `VO2` | oxygen uptake in mL/min |
| `kcal_per_min` | energy in kcal/min |
| `kcal` | total energy in kcal |

- **Method:** kcal/min = MET x 3.5 x $W / 200$ (about 5 kcal per L O2)
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![kcal (total energy in kcal) against MET (activity intensity in METs) from 4 to 12](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-met-energy-curve)

*kcal (total energy in kcal) against MET (activity intensity in METs) from 4 to 12. One curve for each value of weight (weight (e.g. in kg)): 70 kg, 35 kg, 140 kg. Other inputs as in the worked example: duration = 1800.0 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, MET = 8, kcal = 294.*

```python
human_physiology_ai999x.solve('hp_met_energy', MET=8, weight=70.0, duration=1800.0)
# -> VO2 = 1960, kcal_per_min = 9.8, kcal = 294   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_met_energy', MET=3.5, weight=60.0, duration=3600.0)
# -> VO2 = 735, kcal_per_min = 3.675, kcal = 220.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| MET energy expenditure: MET = 8, weight = 70 kg, duration = 30 min | SOLVED | VO2 = 1960, kcal_per_min = 9.8, kcal = 294 |
| MET energy expenditure: MET = 3.5, weight = 60 kg, duration = 60 min | SOLVED | VO2 = 735, kcal_per_min = 3.675, kcal = 220.5 |
| MET energy expenditure: weight = 70 kg, duration = 30 min | NEEDS_INPUT | hp_met_energy needs MET (activity intensity in METs). Example: solve('hp_met_energy', MET=8, weight='70 kg', duration='1 |
| MET energy expenditure: MET = 8, weight = 70 kg, duration = 30 min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| MET energy expenditure: MET = 0, weight = 70 kg, duration = 30 min | INVALID_INPUT | Energy expenditure of an activity from METs: MET must be positive. |

<a id="hp_atwater_energy"></a>

### Food energy by the Atwater factors: `hp_atwater_energy`

Energy from macronutrients with the general Atwater factors 4 (carbohydrate), 9 (fat), 4 (protein) and 7 (alcohol) kcal/g.

$$
E = 4C + 9F + 4P + 7A
$$

| input | meaning | condition |
|---|---|---|
| `carbohydrate` | carbohydrate (e.g. in g) |  |
| `fat` | fat (e.g. in g) |  |
| `protein` | protein (e.g. in g) |  |
| `alcohol` | alcohol (e.g. in g) (optional; none if omitted) |  |

| output | meaning |
|---|---|
| `kcal` | energy in kcal |
| `kJ` | energy in kJ |

- **Method:** $4/9/4/7$ kcal per g; 1 kcal = 4.184 kJ
- **Assumes:** amounts >= 0, not all zero
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![kcal (energy in kcal) against carbohydrate (carbohydrate (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-atwater-energy-curve)

*kcal (energy in kcal) against carbohydrate (carbohydrate (e.g. in g)) from 0.15 kg to 0.45 kg. One curve for each value of fat (fat (e.g. in g)): 0.07 kg, 0.035 kg, 0.14 kg. Other inputs as in the worked example: protein = 0.08 kg. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, carbohydrate = 0.3 kg, kcal = 2150.*

```python
human_physiology_ai999x.solve('hp_atwater_energy', carbohydrate=0.3, fat=0.07, protein=0.08)
# -> kcal = 2150, kJ = 8995.6   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_atwater_energy', carbohydrate=0.3, fat=0.07, protein=0.08, alcohol=0.02)
# -> kcal = 2290, kJ = 9581.36   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Atwater energy: carbohydrate = 300 g, fat = 70 g, protein = 80 g | SOLVED | kcal = 2150, kJ = 8995.6 |
| Atwater energy: carbohydrate = 300 g, fat = 70 g, protein = 80 g, alcohol = 20 g | SOLVED | kcal = 2290, kJ = 9581.36 |
| Atwater energy: fat = 70 g, protein = 80 g | NEEDS_INPUT | hp_atwater_energy needs carbohydrate (carbohydrate (e.g. in g)). Example: solve('hp_atwater_energy', carbohydrate='0.3 k |
| Atwater energy: carbohydrate = 300 g, fat = 70 g, protein = 80 g, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Atwater energy: carbohydrate = 0 g, fat = 0 g, protein = 0 g | INVALID_INPUT | Food energy by the Atwater factors: at least one amount must be positive. |

<a id="hp_body_fat_from_density"></a>

### Body fat from body density (Siri and Brozek), `hp_body_fat_from_density`

Percent body fat by Siri $(495/D - 450)$ and Brozek $(457/D - 414.2)$ from whole-body density.

$$
\%BF = \frac{495}{D} - 450
$$

| input | meaning | condition |
|---|---|---|
| `density` | body density (e.g. in g/mL) |  |

| output | meaning |
|---|---|
| `fat_Siri` | body fat in % (Siri) |
| `fat_Brozek` | body fat in % (Brozek) |

- **Method:** two-compartment models
- **Assumes:** 0.9 < density < 1.2
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![fat_Siri (body fat in % (Siri)) against density (body density (e](https://huggingface.co/AI999X/human-physiology-ai999x/resolve/main/figures/hp-body-fat-from-density-fat-siri-vs-density.svg)

*fat_Siri (body fat in % (Siri)) against density (body density (e.g. in g/mL)). 9 points computed by human-physiology-ai999x; marked: density = 1050, fat_Siri = 21.4286.*

![fat_Siri (body fat in % (Siri)) against density (body density (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-body-fat-from-density-curve)

*fat_Siri (body fat in % (Siri)) against density (body density (e.g. in g/mL)) from 945 kg/m^3 to 1155 kg/m^3. 31 points computed by human-physiology-ai999x; marked: the worked example, density = 1050 kg/m^3, fat_Siri = 21.4286.*

```python
human_physiology_ai999x.solve('hp_body_fat_from_density', density=1050.0000000000002)
# -> fat_Siri = 21.4286, fat_Brozek = 21.0381   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_body_fat_from_density', density=1070.0000000000002)
# -> fat_Siri = 12.6168, fat_Brozek = 12.9028   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| body fat from density: density = 1.05 g/mL | SOLVED | fat_Siri = 21.4286, fat_Brozek = 21.0381 |
| body fat from density: density = 1.07 g/mL | SOLVED | fat_Siri = 12.6168, fat_Brozek = 12.9028 |
| body fat from density:  | NEEDS_INPUT | hp_body_fat_from_density needs density (body density (e.g. in g/mL)). Example: solve('hp_body_fat_from_density', density |
| body fat from density: density = 1.05 g/mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| body fat from density: density = 2 g/mL | INVALID_INPUT | Body fat from body density (Siri and Brozek): body density must be between 0.9 and 1.2 g/mL. |

<a id="hp_hydrostatic_weighing"></a>

### Body density and fat by hydrostatic weighing, `hp_hydrostatic_weighing`

Body volume (Ma - Mw)/rho_w - RV - GI gas, body density and Siri percent fat.

$$
D_b = \frac{M_a}{\frac{M_a - M_w}{\rho_w} - RV - V_{GI}}
$$

| input | meaning | condition |
|---|---|---|
| `mass_air` | body mass in air (e.g. in kg) |  |
| `mass_water` | underwater mass (e.g. in kg) |  |
| `water_density` | water density (e.g. in g/mL) |  |
| `RV` | residual lung volume (e.g. in L) |  |
| `GI` | gastrointestinal gas (e.g. in L) (optional; the customary estimate if omitted) |  |

| output | meaning |
|---|---|
| `volume` | body volume in L |
| `density` | body density in g/mL |
| `fat_Siri` | body fat in % (Siri) |

- **Method:** Archimedes' principle, then Siri
- **Assumes:** mass_water < mass_air; computed density $0.9-1.2$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![fat_Siri (body fat in % (Siri)) against mass_air (body mass in air (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-hydrostatic-weighing-curve)

*fat_Siri (body fat in % (Siri)) against mass_air (body mass in air (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of mass_water (underwater mass (e.g. in kg)): 3 kg, 1.5 kg, 6 kg. Other inputs as in the worked example: water_density = 995.7000000000002 kg/m^3, RV = 0.0012 m^3. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, mass_air = 70 kg, fat_Siri = 16.6389.*

```python
human_physiology_ai999x.solve('hp_hydrostatic_weighing', mass_air=70.0, mass_water=3.0, water_density=995.7000000000002, RV=0.0012)
# -> volume = 65.9893, density = 1.06078, fat_Siri = 16.6389   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_hydrostatic_weighing', mass_air=80.0, mass_water=3.5, water_density=997.0000000000001, RV=0.0015)
# -> volume = 75.1302, density = 1.06482, fat_Siri = 14.8681   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| hydrostatic weighing: mass_air = 70 kg, mass_water = 3 kg, water_density = 0.9957 g/mL, RV = 1.2 L | SOLVED | volume = 65.9893, density = 1.06078, fat_Siri = 16.6389 |
| hydrostatic weighing: mass_air = 80 kg, mass_water = 3.5 kg, water_density = 0.997 g/mL, RV = 1.5 L | SOLVED | volume = 75.1302, density = 1.06482, fat_Siri = 14.8681 |
| hydrostatic weighing: mass_water = 3 kg, water_density = 0.9957 g/mL, RV = 1.2 L | NEEDS_INPUT | hp_hydrostatic_weighing needs mass_air (body mass in air (e.g. in kg)). Example: solve('hp_hydrostatic_weighing', mass_a |
| hydrostatic weighing: mass_air = 70 kg, mass_water = 3 kg, water_density = 0.9957 g/mL, RV = 1.2 L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| hydrostatic weighing: mass_air = 70 kg, mass_water = 80 kg, water_density = 0.9957 g/mL, RV = 1.2 L | INVALID_INPUT | Body density and fat by hydrostatic weighing: underwater mass below the mass in air; water density 0.95-1.0 g/mL. |

<a id="hp_lean_body_mass_boer"></a>

### Lean body mass by the Boer formula: `hp_lean_body_mass_boer`

Boer (1984): male 0.407 $W + 0.267$ $H - 19.2$, female 0.252 $W + 0.473$ $H - 48.3$ (kg), and the remaining fat mass.

$$
LBM_m = 0.407\,W + 0.267\,H - 19.2
$$

| input | meaning | condition |
|---|---|---|
| `weight` | weight (e.g. in kg) |  |
| `height` | height (e.g. in cm) |  |
| `sex` | sex for the formula |  |

| output | meaning |
|---|---|
| `LBM` | lean body mass in kg |
| `fat_mass` | weight - LBM in kg |

- **Method:** Boer anthropometric formula
- **Assumes:** 0 < LBM < weight
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![LBM (lean body mass in kg) against weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-lean-body-mass-boer-curve)

*LBM (lean body mass in kg) against weight (weight (e.g. in kg)) from 35 kg to 105 kg. One curve for each value of height (height (e.g. in cm)): 1.75 m, 0.875 m. Other inputs as in the worked example: sex = male. 26 points per curve computed by human-physiology-ai999x; marked: the worked example, weight = 70 kg, LBM = 56.015.*

![LBM (lean body mass in kg) over weight (weight (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-lean-body-mass-boer-field)

*LBM (lean body mass in kg) over weight (weight (e.g. in kg)) from 35 kg to 105 kg and height (height (e.g. in cm)) from 0.875 m to 2.625 m: each of the 21 × 15 cells is one run of human-physiology-ai999x, shaded by its value (light 18.4075 to dark 93.6225); iso-lines at LBM = 20, 40, 60, 80; blank cells: inputs the capability refuses. Other inputs as in the worked example: sex = male. Marked: the worked example, weight = 70 kg, height = 1.75 m, LBM = 56.015.*

```python
human_physiology_ai999x.solve('hp_lean_body_mass_boer', weight=70.0, height=1.75, sex='male')
# -> LBM = 56.015, fat_mass = 13.985   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_lean_body_mass_boer', weight=60.0, height=1.6500000000000001, sex='female')
# -> LBM = 44.865, fat_mass = 15.135   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Boer lean body mass: weight = 70 kg, height = 175 cm, sex = male | SOLVED | LBM = 56.015, fat_mass = 13.985 |
| Boer lean body mass: weight = 60 kg, height = 165 cm, sex = female | SOLVED | LBM = 44.865, fat_mass = 15.135 |
| Boer lean body mass: height = 175 cm, sex = male | NEEDS_INPUT | hp_lean_body_mass_boer needs weight (weight (e.g. in kg)). Example: solve('hp_lean_body_mass_boer', weight='70 kg', heig |
| Boer lean body mass: weight = 70 kg, height = 175 cm, sex = male, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Boer lean body mass: weight = 0 kg, height = 175 cm, sex = male | INVALID_INPUT | Lean body mass by the Boer formula: weight must be positive. |

<a id="hp_heart_rate_reserve"></a>

### Maximal heart rate, heart rate reserve and Karvonen target, `hp_heart_rate_reserve`

Estimated maximal heart rate (220 - age; Tanaka $208 - 0.7$ age), the reserve HRmax - HRrest and the Karvonen target HRrest + intensity x reserve.

$$
HR_{target} = HR_{rest} + I\,(HR_{max} - HR_{rest})
$$

| input | meaning | condition |
|---|---|---|
| `age` | age in years |  |
| `HR_rest` | resting heart rate in beats/min |  |
| `intensity` | exercise intensity (fraction of reserve) |  |

| output | meaning |
|---|---|
| `HRmax` | age-predicted maximal heart rate (Fox formula) in beats/min |
| `HRmax_Tanaka` | age-predicted maximal heart rate (Tanaka formula) in beats/min |
| `HRR` | heart rate reserve in beats/min |
| `target` | Karvonen target heart rate in beats/min |

- **Method:** Karvonen with HRmax = 220 - age
- **Assumes:** 5 <= age <= 100; HR_rest below HRmax
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![target (Karvonen target heart rate in beats/min) against intensity (exercise intensity (fraction of reserve)) from 0](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-heart-rate-reserve-curve)

*target (Karvonen target heart rate in beats/min) against intensity (exercise intensity (fraction of reserve)) from 0.02 to 0.98. One curve for each value of age (age in years): 40, 20, 80. Other inputs as in the worked example: HR_rest = 60.0. 41 points per curve computed by human-physiology-ai999x; marked: the worked example, intensity = 0.7, target = 144.*

```python
human_physiology_ai999x.solve('hp_heart_rate_reserve', age=40, HR_rest=60, intensity=0.7)
# -> HRmax = 180, HRmax_Tanaka = 180, HRR = 120, target = 144   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_heart_rate_reserve', age=55, HR_rest=65, intensity=0.6)
# -> HRmax = 165, HRmax_Tanaka = 169.5, HRR = 100, target = 125   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Karvonen target heart rate: age = 40, HR_rest = 60, intensity = 0.7 | SOLVED | HRmax = 180, HRmax_Tanaka = 180, HRR = 120, target = 144 |
| Karvonen target heart rate: age = 55, HR_rest = 65, intensity = 0.6 | SOLVED | HRmax = 165, HRmax_Tanaka = 169.5, HRR = 100, target = 125 |
| Karvonen target heart rate: HR_rest = 60, intensity = 0.7 | NEEDS_INPUT | hp_heart_rate_reserve needs age (age in years). Example: solve('hp_heart_rate_reserve', age=40, HR_rest=60, intensity=0. |
| Karvonen target heart rate: age = 40, HR_rest = 60, intensity = 0.7, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| Karvonen target heart rate: age = 40, HR_rest = 60, intensity = 150% | INVALID_INPUT | Maximal heart rate, heart rate reserve and Karvonen target: intensity must be a fraction between 0 and 1. |

<a id="hp_elimination_half_life"></a>

### First-order elimination: rate constant and half-life from clearance and volume, `hp_elimination_half_life`

Physiology of a one-compartment first-order process: $k = CL/Vd$, half-life ln $2/k$ and the fraction remaining after t (not a dosing recommendation).

$$
t_{1/2} = \frac{\ln 2\, V_d}{CL}
$$

| input | meaning | condition |
|---|---|---|
| `CL` | clearance (e.g. in L/h) |  |
| `Vd` | volume of distribution (e.g. in L) |  |
| `t` | elapsed time (e.g. in h) (optional) |  |

| output | meaning |
|---|---|
| `k` | elimination rate constant per hour |
| `half_life` | half-life in h |
| `fraction_remaining` | fraction remaining after t |

- **Method:** $k = CL/Vd$; $t_{1}/2 = ln$ $2 / k$; $C/C_{0} = exp$(-k t)
- **Assumes:** CL, $Vd > 0$; $t  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![half_life (half-life in h) against CL (clearance (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-elimination-half-life-curve)

*half_life (half-life in h) against CL (clearance (e.g. in L/h)) from 6.9444e-07 m^3/s to 2.0833e-06 m^3/s. One curve for each value of Vd (volume of distribution (e.g. in L)): 0.05 m^3, 0.025 m^3, 0.1 m^3. Other inputs as in the worked example: t = 43200.0 s. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, CL = 1.3889e-06 m^3/s, half_life = 6.93147.*

```python
human_physiology_ai999x.solve('hp_elimination_half_life', CL=1.3888888888888887e-06, Vd=0.05, t=43200.0)
# -> k = 0.1, half_life = 6.93147, fraction_remaining = 0.301194   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_elimination_half_life', CL=8.333333333333333e-07, Vd=0.04, t=86400.0)
# -> k = 0.075, half_life = 9.24196, fraction_remaining = 0.165299   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_elimination_half_life', CL=1.3888888888888887e-06, Vd=0.05)
# -> k = 0.1, half_life = 6.93147   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| elimination half-life: CL = 5 L/h, Vd = 50 L, t = 12 h | SOLVED | k = 0.1, half_life = 6.93147, fraction_remaining = 0.301194 |
| elimination half-life: CL = 3 L/h, Vd = 40 L, t = 24 h | SOLVED | k = 0.075, half_life = 9.24196, fraction_remaining = 0.165299 |
| elimination half-life: CL = 5 L/h, Vd = 50 L | SOLVED | k = 0.1, half_life = 6.93147 |
| elimination half-life: Vd = 50 L, t = 12 h | NEEDS_INPUT | hp_elimination_half_life needs CL (clearance (e.g. in L/h)). Example: solve('hp_elimination_half_life', CL='1.38889e-06  |
| elimination half-life: CL = 5 L/h, Vd = 50 L, t = 12 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| elimination half-life: CL = 0 L/h, Vd = 50 L, t = 12 h | INVALID_INPUT | First-order elimination: rate constant and half-life from clearance and volume: CL must be positive. |

<a id="hp_red_cell_indices"></a>

### Red-cell indices: MCV, MCH and MCHC, `hp_red_cell_indices`

Mean corpuscular volume MCV = Hct/RBC, mean corpuscular hemoglobin MCH = Hb/RBC and mean corpuscular hemoglobin concentration MCHC = Hb/Hct, from the hematocrit, hemoglobin and red-cell count.

$$
MCV = \frac{Hct}{RBC},\quad MCH = \frac{Hb}{RBC},\quad MCHC = \frac{Hb}{Hct}
$$

| input | meaning | condition |
|---|---|---|
| `Hct` | hematocrit (fraction) |  |
| `Hb` | hemoglobin in g/dL |  |
| `RBC` | red-cell count in $10^{12}$ cells/L (= $10^{6}$ cells/uL) |  |

| output | meaning |
|---|---|
| `MCV` | mean corpuscular volume in fL |
| `MCH` | mean corpuscular hemoglobin in pg |
| `MCHC` | mean corpuscular hemoglobin concentration in g/dL |

- **Method:** MCV = Hct x 1000 / RBC; MCH = Hb x 10 / RBC; MCHC = Hb / Hct
- **Assumes:** 0 < Hct < 1; Hb, RBC > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![MCV (mean corpuscular volume in fL) against RBC (red-cell count in 10^12 cells/L (= 10^6 cells/uL)) from 2](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-red-cell-indices-curve)

*MCV (mean corpuscular volume in fL) against RBC (red-cell count in 10^12 cells/L (= 10^6 cells/uL)) from 2.5 to 7.5. One curve for each value of Hct (hematocrit (fraction)): 0.45, 0.26, 0.74. Other inputs as in the worked example: Hb = 15.0. 31 points per curve computed by human-physiology-ai999x; marked: the worked example, RBC = 5, MCV = 90.*

```python
human_physiology_ai999x.solve('hp_red_cell_indices', Hct=0.45, Hb=15, RBC=5)
# -> MCV = 90, MCH = 30, MCHC = 33.3333   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_red_cell_indices', Hct=0.3, Hb=8, RBC=4)
# -> MCV = 75, MCH = 20, MCHC = 26.6667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| red cell indices: Hct = 0.45, Hb = 15, RBC = 5 | SOLVED | MCV = 90, MCH = 30, MCHC = 33.3333 |
| red cell indices: Hct = 0.3, Hb = 8, RBC = 4 | SOLVED | MCV = 75, MCH = 20, MCHC = 26.6667 |
| red cell indices: Hb = 15, RBC = 5 | NEEDS_INPUT | hp_red_cell_indices needs Hct (hematocrit (fraction)). Example: solve('hp_red_cell_indices', Hct=0.45, Hb=15, RBC=5). |
| red cell indices: Hct = 0.45, Hb = 15, RBC = 5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: human_physiology_ai999x.templates |
| red cell indices: Hct = 45%, Hb = 15, RBC = 5 | SOLVED | MCV = 90, MCH = 30, MCHC = 33.3333 |
| red cell indices: Hct = 0.45, Hb = 15, RBC = 0 | INVALID_INPUT | Red-cell indices: MCV, MCH and MCHC: RBC must be positive. |

<a id="hp_ecg_heart_rate"></a>

### Heart rate from the ECG (R-R interval or grid boxes), `hp_ecg_heart_rate`

The heart rate of a regular rhythm from the R-R interval, or from the number of small (1 mm) or large (5 mm) boxes between two R waves at the paper speed (25 mm/s unless stated: 1500/small boxes, 300/large boxes).

$$
HR = \frac{60}{RR},\quad RR = \frac{n\,\ell_{box}}{v_{paper}}
$$

| input | meaning | condition |
|---|---|---|
| `RR` | R-R interval (e.g. in s) (give one of RR, small_boxes, large_boxes) |  |
| `small_boxes` | number of small (1 mm) boxes between two R waves |  |
| `large_boxes` | number of large (5 mm) boxes between two R waves |  |
| `speed` | paper speed in mm/s (default 25) |  |

| output | meaning |
|---|---|
| `HR` | heart rate in beats/min |
| `RR` | R-R interval in s |
| `RR_ms` | R-R interval in ms |

- **Method:** $HR = 60 / RR$; RR = boxes x box length / paper speed (1500/small boxes, 300/large boxes at 25 mm/s)
- **Assumes:** exactly one of RR, small boxes, large boxes; regular rhythm; $20  \leq  HR  \leq  300$ beats/min; 1 small box = 1 mm, 1 large box = 5 mm, paper speed 25 mm/s unless stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![HR (heart rate in beats/min) against RR (R-R interval (e](https://www.ai999x.com/?ax_fig=human-physiology-ai999x/hp-ecg-heart-rate-curve)

*HR (heart rate in beats/min) against RR (R-R interval (e.g. in s) (give one of RR, small_boxes, large_boxes)) from 0.4 s to 1.2 s. 31 points computed by human-physiology-ai999x; marked: the worked example, RR = 0.8 s, HR = 75.*

```python
human_physiology_ai999x.solve('hp_ecg_heart_rate', RR=0.8)
# -> HR = 75, RR = 0.8, RR_ms = 800   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ecg_heart_rate', large_boxes=4)
# -> HR = 75, RR = 0.8, RR_ms = 800   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ecg_heart_rate', small_boxes=15, speed=25)
# -> HR = 100, RR = 0.6, RR_ms = 600   (SOLVED)
```

```python
human_physiology_ai999x.solve('hp_ecg_heart_rate', small_boxes=20, speed=50)
# -> HR = 150, RR = 0.4, RR_ms = 400   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| heart rate from the ECG: RR = 0.8 s | SOLVED | HR = 75, RR = 0.8, RR_ms = 800 |
| heart rate from the ECG: large_boxes = 4 | SOLVED | HR = 75, RR = 0.8, RR_ms = 800 |
| heart rate from the ECG: small_boxes = 15, speed = 25 | SOLVED | HR = 100, RR = 0.6, RR_ms = 600 |
| heart rate from the ECG: small_boxes = 20, speed = 50 | SOLVED | HR = 150, RR = 0.4, RR_ms = 400 |
| ECG heart rate: RR = 800 ms | SOLVED | HR = 75, RR = 0.8, RR_ms = 800 |
| heart rate from the ECG: RR = 0.8 s, large_boxes = 4 | INVALID_INPUT | Heart rate from the ECG (R-R interval or grid boxes): give exactly one of the R-R interval RR, the number of small boxes |
| heart rate from the ECG: large_boxes = 0.5 | INVALID_INPUT | Heart rate from the ECG (R-R interval or grid boxes): the heart rate must lie between 20 and 300 beats/min. |
