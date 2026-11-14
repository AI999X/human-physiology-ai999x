"""Human physiology procedures: deterministic calculations of standard textbook physiology quantities with typed
inputs and several outputs (domain code, loaded by the pack; enabled only once tools/certify_procedures.py has
certified them).

CALCULATION ONLY. These procedures compute textbook physiology quantities from stated values. They are NOT medical
advice: they give no diagnosis, no treatment and no dosing recommendation, and must not be used for clinical
decisions. Clinical-style equations (CKD-EPI 2021, Cockcroft-Gault, body surface area, basal metabolic rate) are
evaluated with their published coefficients, the version is named, and every output is a computed value.

Topics: cardiovascular (cardiac output, stroke volume and ejection fraction, cardiac and stroke index, the Fick
principle, mean arterial and pulse pressure, systemic and pulmonary vascular resistance, Poiseuille resistance and
flow, vessels in series and parallel, Reynolds and Womersley numbers in vessels, Laplace wall tension, compliance,
mean velocity and cross-sectional area, Bernoulli pressure drop, stroke work, Teichholz volumes, QT correction,
heart rate from the ECG (R-R interval or grid boxes), the mean QRS axis, Starling filtration, hydrostatic pressure, Moens-Korteweg pulse wave velocity, Qp/Qs and the
physiological shunt); respiratory (lung volumes and capacities, minute and alveolar ventilation, the Bohr dead space,
alveolar ventilation from CO2 output, the alveolar gas equation, the A-a gradient, oxygen content, delivery,
consumption and extraction, the Hill equation and its two-point fit, the Severinghaus oxygen dissociation curve and
its temperature/pH/PCO2 correction, Henderson-Hasselbalch for blood, the anion gap, Fick's law of diffusion,
respiratory mechanics and the time constant, the Laplace pressure of an alveolus, barometric pressure with altitude,
ATPS/BTPS/STPD conversions, the respiratory quotient and the Weir equation, FEV1/FVC); renal (clearance, creatinine
clearance from a timed urine, PAH plasma and blood flow, filtration fraction, free water clearance, fractional
excretion, CKD-EPI 2021 creatinine and creatinine-cystatin C eGFR, Cockcroft-Gault, tubular handling, the glucose
titration curve, plasma and effective osmolality); blood (red-cell indices MCV, MCH, MCHC); body fluids (compartments, the Watson total body water,
osmolarity of a solution, van 't Hoff osmotic pressure, the two-compartment fluid shift, the Gibbs-Donnan
equilibrium); nerve and muscle (Nernst, Goldman-Hodgkin-Katz, chord conductance, ionic current, membrane time and
length constants, conduction velocity scaling and measurement, membrane charge, strength-duration curves, the Hill
force-velocity relation and maximal power); body composition and metabolism (BMI, body surface area, Harris-Benedict,
Mifflin-St Jeor and Katch-McArdle basal metabolic rate, MET energy expenditure, Atwater energy, body fat from density
and hydrostatic weighing, Boer lean body mass, heart rate reserve, first-order elimination half-life).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the one documented question form it parses
('<phrase>: name = value, ...'), worked examples, an INDEPENDENT reference computation (used only by certification;
scipy, sympy and mpmath are imported inside the reference functions, never shipped) and a deliberately wrong variant
that the checks must reject. The shipped code (run, verify, parse) is plain Python (math, re).

How the references differ from the shipped code: Poiseuille resistance, flow and wall shear from quadrature (scipy
quad) of the parabolic velocity profile and a sympy derivative; Laplace tension of cylinders, spheres and alveoli from
quadrature of the pressure force over the wall; hydrostatic pressure and Bernoulli drops from quadrature of the
differential balances; stroke work as the quadrature of P dV; barometric pressure by ODE integration (scipy DOP853) of
the hydrostatic equation through the standard troposphere; time constants (RC membrane, lung emptying, first-order
elimination) by ODE integration; strength-duration thresholds by brentq on the integrated RC response; gas volume
conversions and alveolar ventilation through mole counts of the ideal gas law in SI; the full alveolar gas equation
from a sympy solve of the nitrogen balance; Henderson-Hasselbalch by brentq on the mass-action law; the Hill two-point
fit by scipy least_squares; the Severinghaus inverse by a sympy cubic solve; GHK and chord-conductance potentials by
brentq on zero net current; the Nernst potential by brentq on the electrochemical potential; the length constant from
per-unit-length resistances and a matrix exponential (scipy expm) of the cable equation; maximal muscle power by
brentq on the sympy derivative of the power; vessels in series and parallel by a nodal (Laplacian) linear solve; the
fluid shift by scipy fsolve of the equilibrium system; the Gibbs-Donnan equilibrium by brentq on electroneutrality;
Qp/Qs and shunt fractions by linear solves of the oxygen mass balances; the QRS axis by scipy least_squares on the
lead projections. Weaker references (a published formula or rule evaluated through a sympy solve of the same
defining relation) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- inputs are plain numbers in the customary physiology unit stated in each input's meaning (mmHg, mL, L/min,
  mL/dL, g/dL, mg/dL, mmol/L = mM, cm, kg, years, beats/min, ms, degC); fractions (saturations, FiO2, hematocrit,
  ejection fraction, intensity, body fat) are written as 0.21 or 21% (a bare 21 is refused);
- 1 mmHg = 133.322387415 Pa; 1 cmH2O = 98.0665 Pa; g = 9.80665 m/s^2; R = 8.314462618 J/(mol K);
  F = NA e = 96485.33212... C/mol; e = 1.602176634e-19 C; body temperature 37 degC = 310.15 K; 0 degC = 273.15 K;
- oxygen: 1.34 mL O2 per g of hemoglobin and 0.003 mL O2/dL per mmHg dissolved; water vapour 47 mmHg at 37 degC
  unless stated; CO2 solubility 0.03 mmol/L per mmHg and pK' 6.1;
- vascular resistance in Wood units (mmHg min/L) and in dyn s cm^-5 with the exact factor
  1 mmHg min/L = 79.9934... dyn s cm^-5 (the customary rounded factor is 80);
- MAP = DBP + (SBP - DBP)/3; alveolar ventilation from CO2 uses K = 760 x 310.15/273.15 mmHg (STPD to BTPS);
- CKD-EPI 2021 (race-free) creatinine and creatinine-cystatin C equations with their published coefficients;
  Cockcroft-Gault with 0.85 for female; Harris-Benedict original (1919) or revised (Roza-Shizgal 1984);
  Mifflin-St Jeor (1990); body surface area DuBois (1916), Mosteller (1987), Haycock (1978), Gehan-George (1970);
- every real output is rounded to 12 significant digits (identical answers on every platform and Python version).

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`cardiac output: HR = 72, SV = 70 mL`). A physical input (a pressure, volume, flow, length, mass, time,
temperature, voltage, concentration in mmol/L ...) is written with a unit the shared reader knows (any unit of its
kind: 70 mL or 0.07 L); the runtime passes it in SI and the procedure converts it to the customary unit of the
formula (_PHYS). Inputs whose customary unit the shared reader does not know (mg/dL, g/dL, cP, cmH2O, mM, mOsm/L,
beats/min, years) are plain numbers in the unit of their meaning. Choices are words (`sex = female`). Any other text,
an unknown name, a name given twice, a physical value without its unit or a unit on a plain number is refused,
never guessed.
"""
from __future__ import annotations

import math
import re

INF = float("inf")
PI = math.pi
MMHG = 133.322387415            # Pa
CMH2O = 98.0665                 # Pa
G0 = 9.80665                    # m/s^2
R_GAS = 8.314462618             # J/(mol K)
NA = 6.02214076e23
E_CHARGE = 1.602176634e-19      # C
FARADAY = NA * E_CHARGE         # C/mol
T0C = 273.15
T_BODY = 310.15
O2_HB = 1.34                    # mL O2 per g Hb
O2_SOL = 0.003                  # mL O2 / dL / mmHg
CO2_SOL = 0.03                  # mmol/L / mmHg
PK_CO2 = 6.1
WOOD_DYN = MMHG * 60.0 / 1e-3 / 1e5   # 1 mmHg min/L in dyn s cm^-5
K_ALV = 760.0 * T_BODY / T0C          # mmHg, STPD -> BTPS factor of the alveolar ventilation equation


# ================================================================================================ checks
def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _num(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite number")
    return float(v)


def _pos(v, name):
    v = _num(v, name)
    _need(v > 0, f"{name} must be positive")
    return v


def _nonneg(v, name):
    v = _num(v, name)
    _need(v >= 0, f"{name} must be zero or positive")
    return v


def _fr(v, name, open_lo=False, open_hi=False):
    """A fraction between 0 and 1 (end points allowed unless stated)."""
    v = _num(v, name)
    _need((v > 0 if open_lo else v >= 0) and (v < 1 if open_hi else v <= 1), f"{name} must be a fraction between 0 and 1")
    return v


def _vec(v, name, lo=1, hi=12):
    _need(isinstance(v, (list, tuple)), f"{name} must be a list of numbers")
    out = [_num(x, name) for x in v]
    _need(lo <= len(out) <= hi, f"{name} needs between {lo} and {hi} entries")
    return out


def _sex(v):
    _need(v in ("female", "male"), "sex must be female or male")
    return v == "female"


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


# ================================================================================================ rounding
def _r12(x):
    """12 significant digits (identical on every platform); -0 becomes 0."""
    return float(f"{x:.12g}") + 0.0


def _rnd(v):
    if isinstance(v, bool) or isinstance(v, str) or v is None:
        return v
    if isinstance(v, int):
        return v
    if isinstance(v, float):
        return _r12(v)
    if isinstance(v, (list, tuple)):
        return [_rnd(x) for x in v]
    if isinstance(v, dict):
        return {k: _rnd(x) for k, x in v.items()}
    return v


# ================================================================================================ residuals
def _diff(a, b):
    if isinstance(a, str) or isinstance(b, str) or isinstance(a, bool) or isinstance(b, bool):
        return 0.0 if a == b else INF
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return INF
        w = 0.0
        for x, y in zip(a, b):
            w = max(w, _diff(x, y))
        return w
    if a is None or b is None:
        return 0.0 if a is b else INF
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return INF
    if a == b:
        return 0.0
    d = abs(a - b)
    if d != d:
        return INF
    return d / max(abs(a), abs(b))


def _recheck(run):
    """Residual of a deterministic calculation: the outputs recomputed from the inputs."""
    def verify(inputs, outputs):
        try:
            want = run(**inputs)
        except Exception:
            return INF
        worst = 0.0
        for k, w in want.items():
            if k not in outputs:
                return INF
            worst = max(worst, _diff(outputs[k], w))
        return worst
    return verify


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_PCT_WORDS = ("%", "percent", "per cent")

# Physical inputs (a unit the shared unit reader knows): the input gets the pack kind, the runtime reads the value with
# any unit of that kind and passes it in SI, and the procedure converts it back to the customary physiology unit named
# in the input's meaning before the formula runs. unit -> (kind, SI value of one unit, SI offset)
_PHYS = {
    "mmHg": ("pressure", MMHG, 0.0), "kPa": ("pressure", 1000.0, 0.0),
    "mL": ("volume", 1e-6, 0.0), "L": ("volume", 1e-3, 0.0),
    "mL/min": ("flow_rate", 1e-6 / 60.0, 0.0), "L/min": ("flow_rate", 1e-3 / 60.0, 0.0), "L/h": ("flow_rate", 1e-3 / 3600.0, 0.0),
    "m": ("length", 1.0, 0.0), "cm": ("length", 0.01, 0.0), "mm": ("length", 0.001, 0.0), "um": ("length", 1e-6, 0.0),
    "kg": ("mass", 1.0, 0.0), "g": ("mass", 0.001, 0.0),
    "m^2": ("area", 1.0, 0.0), "cm^2": ("area", 1e-4, 0.0), "um^2": ("area", 1e-12, 0.0),
    "m/s": ("speed", 1.0, 0.0), "cm/s": ("speed", 0.01, 0.0),
    "s": ("time", 1.0, 0.0), "ms": ("time", 0.001, 0.0), "min": ("time", 60.0, 0.0), "h": ("time", 3600.0, 0.0),
    "degC": ("temperature", 1.0, T0C),
    "mV": ("voltage", 0.001, 0.0), "N": ("force", 1.0, 0.0), "nS": ("electrical_conductance", 1e-9, 0.0),
    "kg/m^3": ("density", 1.0, 0.0), "g/mL": ("density", 1000.0000000000001, 0.0), "g/L": ("mass_concentration", 1.0, 0.0),
    "mg/L": ("mass_concentration", 0.001, 0.0), "mmol/L": ("molar_concentration", 1.0, 0.0), "g/mol": ("molar_mass", 0.001, 0.0), "mg/min": ("mass_flow_rate", 1e-6 / 60.0, 0.0),
    "cm^2/s": ("diffusion_coefficient", 1e-4, 0.0), "mN/m": ("surface_tension", 0.001, 0.0),
    "uF/cm^2": ("specific_membrane_capacitance", 0.009999999999999998, 0.0),
}
_UNIT_IN = re.compile(r" in ([^ ()]+)(?= \(|$)")


def _phys_unit(spec):
    """The customary unit of a plain physical input (named in its meaning as '... in <unit>'), or None."""
    if spec.get("type") or spec.get("pct") or spec.get("kind", "number") != "number":
        return None
    m = _UNIT_IN.search(spec["meaning"])
    return m.group(1) if m and m.group(1) in _PHYS else None


def _from_si(conv, kw):
    """SI values of the physical inputs converted to the customary units the formulas use."""
    out = dict(kw)
    for k, (f, off) in conv.items():
        v = out.get(k)
        if v is not None and not isinstance(v, (str, bool)):
            out[k] = (v - off) / f
    return out


def _to_si(conv, kw):
    out = dict(kw)
    for k, (f, off) in conv.items():
        v = out.get(k)
        if v is not None and not isinstance(v, (str, bool)):
            out[k] = v * f + off
    return out
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]\[]*\])\s*$")


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs):
    """The one documented question form, '<phrase>: name = value, ...'.

    Reading guard: the capability phrase must open the question and be followed by a colon (wording that names
    another capability never reaches this reader); the rest is split into 'name = value' pairs and any other text,
    unknown name, repeated name or unit after a value refuses the whole question (a stray number is never ignored);
    a fraction input accepts 20% or 0.2 but refuses a bare 20; conventions are inputs, never assumed."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    by = {}
    names = {}
    for i in inputs:
        by[i["symbol"]] = i
        names[i["symbol"]] = i["symbol"]
    for i in inputs:
        names.setdefault(i["symbol"].lower(), i["symbol"])

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "choice":
                    return None
                out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                continue
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            if t in ("vector", "choice"):
                return None
            num, tail = ms.group(2), ms.group(3)
            pct = tail if tail in _PCT_WORDS else ""
            if spec.get("kind", "number") != "number":           # a physical input: a value with its unit, read in SI
                if not tail or pct:                               # by the runtime; a bare number is refused
                    return None
                out[key] = f"{num} {tail}"
                continue
            if tail and not pct:                                  # a unit on a plain number is refused
                return None
            if t == "integer":
                if pct:
                    return None
                out[key] = num
                continue
            if pct:
                if not spec.get("pct"):
                    return None
                out[key] = float(num) / 100.0
                continue
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _w(v):
    """A number as a user writes it."""
    v = float(v)
    if v == int(v) and abs(v) < 1e15:
        return str(int(v))
    return repr(v)


def _written(spec, v):
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_w(x) for x in v) + "]"
    if t == "choice":
        return str(v)
    if t == "integer":
        return str(int(v))
    return _w(v)


# ================================================================================================ registry
def _n(sym, meaning, required=True, pct=False):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    return d


def _int(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _v(sym, meaning, lo=1, hi=10, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _ch(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, meaning):
    return {"symbol": sym, "kind": "number", "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them.

    Physical inputs (a meaning ending 'in <unit>' with a unit of _PHYS) get their pack kind: the runtime passes them
    in SI and run, reference and wrong variant receive them converted back to the customary unit. The formulas, the
    worked examples and random inputs are written in customary units; examples and random inputs are converted to SI
    here, exactly as the runtime converts a user's written value."""
    conv, shown_unit = {}, {}
    for i in inputs:
        u = _phys_unit(i)
        if u is not None:
            kind, f, off = _PHYS[u]
            i["kind"] = kind
            i["meaning"] = i["meaning"].replace(f" in {u}", f" (e.g. in {u})", 1)
            conv[i["symbol"]] = (f, off)
            shown_unit[i["symbol"]] = u
    customary_run = run
    if conv:
        run = (lambda fn: lambda **kw: fn(**_from_si(conv, kw)))(customary_run)
        reference = (lambda fn: lambda **kw: fn(**_from_si(conv, kw)))(reference)
        wrong = (lambda fn: lambda **kw: fn(**_from_si(conv, kw)))(wrong)
        rand = (lambda fn: lambda rng: _to_si(conv, fn(rng)))(rand)
    parse = _reader(phrases, inputs)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" + (f" {shown_unit[k]}" if k in shown_unit else "")
                                               for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    for q in extra_q:
        if "inputs" in q:
            questions.append({"q": ask_text(q["inputs"]), "outcome": q["outcome"]})
        else:
            questions.append(q)
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates,
        "examples": [{"inputs": _to_si(conv, e["inputs"]), "outputs": e["outputs"]} for e in examples], "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs):
    return {"inputs": inputs, "outputs": outputs}


def _bad(inputs, outcome="INVALID_INPUT"):
    return {"inputs": inputs, "outcome": outcome}


def _u(rng, a, b, nd=1):
    """A uniform random value written with nd decimals (as a user writes it)."""
    return round(rng.uniform(a, b), nd)


def _drop(rng, d, *optional):
    """Random inputs with each named optional input left out half the time (a question asking only for an output that
    does not need it states no value for it)."""
    for k in optional:
        if rng.random() < 0.5:
            d.pop(k, None)
    return d


# ================================================================================================ reference helpers
_SXC: dict = {}


def _sx(eq, unknown, **vals):
    """Reference helper: solve the defining relation `eq` (an expression equal to zero) for `unknown` with sympy and
    evaluate it at the given values in 30-digit arithmetic. Inequalities in a branch are passed as values."""
    import sympy as sp
    key = (eq, unknown, tuple(sorted(vals)))
    if key not in _SXC:
        syms = {k: sp.Symbol(k) for k in list(vals) + [unknown]}
        expr = sp.sympify(eq, locals=syms)
        sol = sp.solve(expr, syms[unknown])
        _SXC[key] = (syms, sol)
    syms, sol = _SXC[key]
    out = []
    for s in sol:
        v = s.subs({syms[k]: sp.Float(repr(float(x)), 30) for k, x in vals.items()}).evalf(30)
        if v.is_real:
            out.append(float(v))
    if len(out) != 1:
        raise RuntimeError(f"expected one real solution, got {out}")
    return out[0]


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _ivp(fun, t_end, y0, rtol=1e-13):
    from scipy.integrate import solve_ivp
    atol = [1e-20 * max(1.0, abs(v)) for v in y0]
    sol = solve_ivp(fun, (0.0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol)
    if not sol.success:
        raise RuntimeError(sol.message)
    return [float(v) for v in sol.y[:, -1]]


# ================================================================================================ cardiovascular
def cardiac_output(HR, SV):
    HR, SV = _pos(HR, "HR"), _pos(SV, "SV")
    return _rnd({"CO": HR * SV / 1000.0, "CO_mL": HR * SV})


_add("hp_cardiac_output", name="Cardiac output from heart rate and stroke volume", topic="cardiovascular / cardiac output",
     purpose="Cardiac output CO = HR x SV, in L/min and mL/min.",
     inputs=[_n("HR", "heart rate in beats/min"), _n("SV", "stroke volume in mL")],
     outputs=[_o("CO", "cardiac output in L/min"), _o("CO_mL", "cardiac output in mL/min")],
     primary="CO", conditions=["HR > 0", "SV > 0"], method="CO = HR SV / 1000", latex=r"CO = HR \times SV",
     run=cardiac_output,
     reference=lambda HR, SV: {"CO": _sx("CO*1000 - HR*SV", "CO", HR=HR, SV=SV), "CO_mL": _sx("C - HR*SV", "C", HR=HR, SV=SV)},
     wrong=lambda HR, SV: {"CO": HR * SV / 100.0, "CO_mL": HR * SV * 10},
     rand=lambda rng: {"HR": _u(rng, 40, 180, 0), "SV": _u(rng, 30, 130, 0)},
     phrases=["cardiac output"],
     examples=[_ex({"HR": 72, "SV": 70}, {"CO": 5.04, "CO_mL": 5040}), _ex({"HR": 60, "SV": 80}, {"CO": 4.8})],
     extra_q=[_bad({"HR": 0, "SV": 70})], level="beginner", weak="the defining product re-solved with sympy")


def stroke_volume_ef(EDV, ESV):
    EDV, ESV = _pos(EDV, "EDV"), _nonneg(ESV, "ESV")
    _need(ESV < EDV, "ESV must be smaller than EDV")
    sv = EDV - ESV
    return _rnd({"SV": sv, "EF": sv / EDV, "EF_percent": 100.0 * sv / EDV})


_add("hp_stroke_volume_ef", name="Stroke volume and ejection fraction", topic="cardiovascular / cardiac output",
     purpose="Stroke volume SV = EDV - ESV and ejection fraction EF = SV/EDV from end-diastolic and end-systolic volumes.",
     inputs=[_n("EDV", "end-diastolic volume in mL"), _n("ESV", "end-systolic volume in mL")],
     outputs=[_o("SV", "stroke volume in mL"), _o("EF", "ejection fraction (fraction)"), _o("EF_percent", "ejection fraction in %")],
     primary="EF", conditions=["0 <= ESV < EDV"], method="SV = EDV - ESV; EF = SV/EDV",
     latex=r"SV = EDV - ESV,\quad EF = \frac{SV}{EDV}", run=stroke_volume_ef,
     reference=lambda EDV, ESV: (lambda sv: {"SV": sv, "EF": _sx("EF*EDV - SV", "EF", EDV=EDV, SV=sv),
                                             "EF_percent": _sx("P*EDV - 100*SV", "P", EDV=EDV, SV=sv)})(_sx("S + ESV - EDV", "S", EDV=EDV, ESV=ESV)),
     wrong=lambda EDV, ESV: {"SV": EDV - ESV, "EF": (EDV - ESV) / ESV, "EF_percent": 100 * (EDV - ESV) / ESV},
     rand=lambda rng: (lambda e: {"EDV": e, "ESV": _u(rng, 0.2 * e, 0.8 * e, 0)})(_u(rng, 80, 220, 0)),
     phrases=["stroke volume and ejection fraction", "ejection fraction"],
     examples=[_ex({"EDV": 120, "ESV": 50}, {"SV": 70, "EF": 0.583333333333, "EF_percent": 58.3333333333}),
               _ex({"EDV": 150, "ESV": 90}, {"SV": 60, "EF": 0.4})],
     extra_q=[_bad({"EDV": 100, "ESV": 120})], level="beginner", weak="the definitions re-solved with sympy")


def cardiac_index(CO, BSA, HR=None):
    CO, BSA = _pos(CO, "CO"), _pos(BSA, "BSA")
    ci = CO / BSA
    out = {"CI": ci}
    if HR is not None:
        HR = _pos(HR, "HR")
        out["SVI"] = ci * 1000.0 / HR
        out["SV"] = CO * 1000.0 / HR
    return _rnd(out)


def ref_cardiac_index(CO, BSA, HR=None):
    out = {"CI": _sx("C*BSA - CO", "C", CO=CO, BSA=BSA)}
    if HR is not None:
        out["SVI"] = _sx("S*BSA*HR - 1000*CO", "S", CO=CO, BSA=BSA, HR=HR)
        out["SV"] = _sx("S*HR - 1000*CO", "S", CO=CO, HR=HR)
    return out


_add("hp_cardiac_index", name="Cardiac index and stroke volume index", topic="cardiovascular / cardiac output",
     purpose="Cardiac index CI = CO/BSA, stroke volume SV = CO/HR and stroke volume index SVI = CI/HR.",
     inputs=[_n("CO", "cardiac output in L/min"), _n("BSA", "body surface area in m^2"),
             _n("HR", "heart rate in beats/min (optional; needed for the stroke volume index)", required=False)],
     outputs=[_o("CI", "cardiac index in L/min/m^2"), _o("SVI", "stroke volume index in mL/m^2"), _o("SV", "stroke volume in mL")],
     primary="CI", conditions=["CO, BSA, HR > 0"], method="CI = CO/BSA; SVI = 1000 CI/HR", latex=r"CI = \frac{CO}{BSA},\quad SVI = \frac{CI}{HR}",
     run=cardiac_index, reference=ref_cardiac_index,
     wrong=lambda CO, BSA, HR=None: dict({"CI": CO * BSA}, **({"SVI": CO * BSA * 1000 / HR, "SV": CO * 1000 / HR} if HR is not None else {})),
     rand=lambda rng: _drop(rng, {"CO": _u(rng, 3, 9, 2), "BSA": _u(rng, 1.4, 2.4, 2), "HR": _u(rng, 50, 120, 0)}, "HR"),
     phrases=["cardiac index"],
     examples=[_ex({"CO": 5.04, "BSA": 1.8, "HR": 72}, {"CI": 2.8, "SVI": 38.8888888889, "SV": 70}),
               _ex({"CO": 6, "BSA": 2, "HR": 75}, {"CI": 3, "SVI": 40})],
     extra_q=[_bad({"CO": 5, "BSA": 0, "HR": 70})], level="beginner", weak="the definitions re-solved with sympy")


def fick_cardiac_output(VO2, CaO2, CvO2):
    VO2, CaO2, CvO2 = _pos(VO2, "VO2"), _pos(CaO2, "CaO2"), _nonneg(CvO2, "CvO2")
    _need(CvO2 < CaO2, "the venous oxygen content must be below the arterial")
    d = CaO2 - CvO2
    return _rnd({"CO": VO2 / (d * 10.0), "avDO2": d})


def ref_fick_co(VO2, CaO2, CvO2):
    # oxygen mass balance across the body: uptake = flow x (arterial - venous) content, contents in mL O2 per L
    from scipy.linalg import solve
    import numpy as np
    A = np.array([[CaO2 * 10.0 - CvO2 * 10.0, 0.0], [0.0, 1.0]])
    x = solve(A, np.array([VO2, CaO2 - CvO2]))
    return {"CO": float(x[0]), "avDO2": float(x[1])}      # mL/min over mL/L = L/min


_add("hp_fick_cardiac_output", name="Cardiac output by the Fick principle", topic="cardiovascular / cardiac output",
     purpose="Cardiac output from oxygen consumption and the arteriovenous oxygen content difference: CO = VO2/(CaO2 - CvO2).",
     inputs=[_n("VO2", "oxygen consumption in mL/min"), _n("CaO2", "arterial oxygen content in mL O2/dL"),
             _n("CvO2", "mixed venous oxygen content in mL O2/dL")],
     outputs=[_o("CO", "cardiac output in L/min"), _o("avDO2", "arteriovenous O2 content difference in mL/dL")],
     primary="CO", conditions=["VO2 > 0", "0 <= CvO2 < CaO2"], method="CO = VO2 / (10 (CaO2 - CvO2)) (contents per dL -> per L)",
     latex=r"CO = \frac{\dot V_{O_2}}{C_aO_2 - C_{\bar v}O_2}", run=fick_cardiac_output, reference=ref_fick_co,
     wrong=lambda VO2, CaO2, CvO2: {"CO": VO2 / (CaO2 - CvO2) / 100.0, "avDO2": CaO2 - CvO2},
     rand=lambda rng: (lambda ca: {"VO2": _u(rng, 150, 400, 0), "CaO2": ca, "CvO2": _u(rng, 0.5 * ca, 0.85 * ca, 1)})(_u(rng, 14, 22, 1)),
     phrases=["Fick cardiac output", "cardiac output by the Fick principle"],
     examples=[_ex({"VO2": 250, "CaO2": 20, "CvO2": 15}, {"CO": 5, "avDO2": 5}), _ex({"VO2": 300, "CaO2": 19.5, "CvO2": 14.5}, {"CO": 6})],
     extra_q=[_bad({"VO2": 250, "CaO2": 15, "CvO2": 20})], level="intermediate", weak="the mass balance solved as a linear system (same relation)")


def _o2c(Hb, S, P):
    return O2_HB * Hb * S + O2_SOL * P


def fick_saturations(VO2, Hb, SaO2, SvO2, PaO2, PvO2):
    VO2, Hb, PaO2, PvO2 = _pos(VO2, "VO2"), _pos(Hb, "Hb"), _nonneg(PaO2, "PaO2"), _nonneg(PvO2, "PvO2")
    SaO2, SvO2 = _fr(SaO2, "SaO2"), _fr(SvO2, "SvO2")
    ca, cv = _o2c(Hb, SaO2, PaO2), _o2c(Hb, SvO2, PvO2)
    _need(cv < ca, "the venous oxygen content must be below the arterial")
    return _rnd({"CO": VO2 / ((ca - cv) * 10.0), "CaO2": ca, "CvO2": cv})


def ref_fick_sat(VO2, Hb, SaO2, SvO2, PaO2, PvO2):
    ca = _sx("C - 134*Hb*S/100 - 3*P/1000", "C", Hb=Hb, S=SaO2, P=PaO2)
    cv = _sx("C - 134*Hb*S/100 - 3*P/1000", "C", Hb=Hb, S=SvO2, P=PvO2)
    return {"CO": _sx("Q*10*(A - B) - V", "Q", A=ca, B=cv, V=VO2), "CaO2": ca, "CvO2": cv}


_add("hp_fick_saturations", name="Fick cardiac output from hemoglobin, saturations and PO2",
     topic="cardiovascular / cardiac output",
     purpose="Arterial and mixed venous O2 contents (1.34 Hb S + 0.003 PO2) and the Fick cardiac output VO2/(CaO2 - CvO2).",
     inputs=[_n("VO2", "oxygen consumption in mL/min"), _n("Hb", "hemoglobin in g/dL"), _n("SaO2", "arterial saturation (fraction)", pct=True),
             _n("SvO2", "mixed venous saturation (fraction)", pct=True), _n("PaO2", "arterial PO2 in mmHg"), _n("PvO2", "mixed venous PO2 in mmHg")],
     outputs=[_o("CO", "cardiac output in L/min"), _o("CaO2", "arterial O2 content in mL/dL"), _o("CvO2", "venous O2 content in mL/dL")],
     primary="CO", conditions=["saturations between 0 and 1", "CvO2 < CaO2"], method="C = 1.34 Hb S + 0.003 PO2; CO = VO2/(10 (CaO2 - CvO2))",
     latex=r"CO = \frac{\dot V_{O_2}}{1.34\,Hb\,(S_aO_2 - S_{\bar v}O_2) + 0.003\,(P_aO_2 - P_{\bar v}O_2)}",
     run=fick_saturations, reference=ref_fick_sat,
     wrong=lambda VO2, Hb, SaO2, SvO2, PaO2, PvO2: {"CO": VO2 / ((1.34 * Hb * (SaO2 - SvO2)) * 10.0) * 0.9, "CaO2": 1.34 * Hb * SaO2, "CvO2": 1.34 * Hb * SvO2},
     rand=lambda rng: {"VO2": _u(rng, 150, 400, 0), "Hb": _u(rng, 9, 17, 1), "SaO2": _u(rng, 0.9, 1.0, 2), "SvO2": _u(rng, 0.55, 0.8, 2),
                       "PaO2": _u(rng, 70, 110, 0), "PvO2": _u(rng, 30, 45, 0)},
     phrases=["Fick cardiac output from saturations"],
     examples=[_ex({"VO2": 250, "Hb": 15, "SaO2": 0.98, "SvO2": 0.75, "PaO2": 95, "PvO2": 40}, {"CO": 5.22138680033, "CaO2": 19.983, "CvO2": 15.195})],
     extra_q=[_bad({"VO2": 250, "Hb": 15, "SaO2": 0.75, "SvO2": 0.98, "PaO2": 95, "PvO2": 40})], level="advanced",
     weak="the published content and Fick relations re-solved with sympy")


def mean_arterial_pressure(SBP, DBP):
    SBP, DBP = _pos(SBP, "SBP"), _pos(DBP, "DBP")
    _need(SBP > DBP, "the systolic pressure must exceed the diastolic")
    return _rnd({"MAP": DBP + (SBP - DBP) / 3.0, "PP": SBP - DBP})


def ref_map(SBP, DBP):
    # time average of an idealized pressure wave at systolic pressure for one third of the cycle
    m = _quad(lambda t: SBP if t < 1.0 / 3.0 else DBP, 0.0, 1.0 / 3.0) + _quad(lambda t: DBP, 1.0 / 3.0, 1.0)
    return {"MAP": m, "PP": _sx("P - S + D", "P", S=SBP, D=DBP)}


_add("hp_mean_arterial_pressure", name="Mean arterial pressure and pulse pressure", topic="cardiovascular / blood pressure",
     purpose="Pulse pressure PP = SBP - DBP and the estimated mean arterial pressure MAP = DBP + PP/3.",
     inputs=[_n("SBP", "systolic pressure in mmHg"), _n("DBP", "diastolic pressure in mmHg")],
     outputs=[_o("MAP", "mean arterial pressure in mmHg"), _o("PP", "pulse pressure in mmHg")],
     primary="MAP", conditions=["SBP > DBP > 0"], method="MAP = DBP + (SBP - DBP)/3", latex=r"MAP = DBP + \tfrac{1}{3}(SBP - DBP)",
     run=mean_arterial_pressure, reference=ref_map,
     wrong=lambda SBP, DBP: {"MAP": (SBP + DBP) / 2.0, "PP": SBP - DBP},
     rand=lambda rng: (lambda d: {"DBP": d, "SBP": d + _u(rng, 20, 80, 0)})(_u(rng, 50, 100, 0)),
     phrases=["mean arterial pressure"],
     examples=[_ex({"SBP": 120, "DBP": 80}, {"MAP": 93.3333333333, "PP": 40}), _ex({"SBP": 150, "DBP": 90}, {"MAP": 110, "PP": 60})],
     extra_q=[_bad({"SBP": 80, "DBP": 120})], level="beginner", weak="the one-third rule restated as the average of an idealized wave")


def vascular_resistance(MAP, CVP, CO):
    MAP, CVP, CO = _num(MAP, "MAP"), _nonneg(CVP, "CVP"), _pos(CO, "CO")
    _need(MAP > CVP, "the mean arterial pressure must exceed the venous pressure")
    w = (MAP - CVP) / CO
    return _rnd({"SVR": w, "SVR_dyn": w * WOOD_DYN})


def ref_vascular_resistance(MAP, CVP, CO):
    # SI route: pressure drop in Pa over flow in m^3/s, then to dyn s cm^-5 (1 dyn = 1e-5 N, 1 cm^5 = 1e-10 m^5)
    r_si = (MAP - CVP) * 133.322387415 / (CO * 1e-3 / 60.0)
    return {"SVR": r_si / 133.322387415 * 1e-3 / 60.0, "SVR_dyn": r_si * 1e-5}


_add("hp_systemic_vascular_resistance", name="Systemic vascular resistance (total peripheral resistance)",
     topic="cardiovascular / flow and resistance",
     purpose="SVR = (MAP - CVP)/CO in Wood units (mmHg min/L) and in dyn s cm^-5 (exact factor 79.9934; customary 80).",
     inputs=[_n("MAP", "mean arterial pressure in mmHg"), _n("CVP", "central venous pressure in mmHg"), _n("CO", "cardiac output in L/min")],
     outputs=[_o("SVR", "systemic vascular resistance in mmHg min/L (Wood units)"), _o("SVR_dyn", "systemic vascular resistance in dyn s cm^-5")],
     primary="SVR", conditions=["MAP > CVP >= 0", "CO > 0"], method="SVR = (MAP - CVP)/CO; x 79.9934 for dyn s cm^-5",
     latex=r"SVR = \frac{MAP - CVP}{CO}", run=vascular_resistance, reference=ref_vascular_resistance,
     wrong=lambda MAP, CVP, CO: {"SVR": MAP / CO, "SVR_dyn": MAP / CO * WOOD_DYN},
     rand=lambda rng: {"MAP": _u(rng, 60, 130, 0), "CVP": _u(rng, 1, 15, 0), "CO": _u(rng, 2.5, 9, 1)},
     phrases=["systemic vascular resistance", "total peripheral resistance"],
     examples=[_ex({"MAP": 93, "CVP": 3, "CO": 5}, {"SVR": 18, "SVR_dyn": 1439.88178408}), _ex({"MAP": 100, "CVP": 0, "CO": 5}, {"SVR": 20, "SVR_dyn": 1599.86864898})],
     extra_q=[_bad({"MAP": 93, "CVP": 3, "CO": 0})], level="intermediate")


def pulmonary_vascular_resistance(mPAP, PCWP, CO):
    mPAP, PCWP, CO = _num(mPAP, "mPAP"), _nonneg(PCWP, "PCWP"), _pos(CO, "CO")
    _need(mPAP > PCWP, "the mean pulmonary artery pressure must exceed the wedge pressure")
    w = (mPAP - PCWP) / CO
    return _rnd({"PVR": w, "PVR_dyn": w * WOOD_DYN, "TPG": mPAP - PCWP})


def ref_pvr(mPAP, PCWP, CO):
    r_si = (mPAP - PCWP) * 133.322387415 / (CO * 1e-3 / 60.0)
    return {"PVR": r_si / 133.322387415 * 1e-3 / 60.0, "PVR_dyn": r_si * 1e-5, "TPG": _sx("T - A + B", "T", A=mPAP, B=PCWP)}


_add("hp_pulmonary_vascular_resistance", name="Pulmonary vascular resistance and transpulmonary gradient",
     topic="cardiovascular / flow and resistance",
     purpose="Transpulmonary gradient TPG = mPAP - PCWP and PVR = TPG/CO in Wood units and dyn s cm^-5.",
     inputs=[_n("mPAP", "mean pulmonary artery pressure in mmHg"), _n("PCWP", "pulmonary capillary wedge pressure in mmHg"),
             _n("CO", "cardiac output in L/min")],
     outputs=[_o("PVR", "pulmonary vascular resistance in Wood units"), _o("PVR_dyn", "pulmonary vascular resistance in dyn s cm^-5"),
              _o("TPG", "transpulmonary gradient in mmHg")],
     primary="PVR", conditions=["mPAP > PCWP >= 0", "CO > 0"], method="PVR = (mPAP - PCWP)/CO",
     latex=r"PVR = \frac{\bar P_{PA} - P_{CW}}{CO}", run=pulmonary_vascular_resistance, reference=ref_pvr,
     wrong=lambda mPAP, PCWP, CO: {"PVR": mPAP / CO, "PVR_dyn": mPAP / CO * WOOD_DYN, "TPG": mPAP},
     rand=lambda rng: {"mPAP": _u(rng, 12, 50, 0), "PCWP": _u(rng, 4, 11, 0), "CO": _u(rng, 2.5, 8, 1)},
     phrases=["pulmonary vascular resistance"],
     examples=[_ex({"mPAP": 15, "PCWP": 8, "CO": 5}, {"PVR": 1.4, "TPG": 7}), _ex({"mPAP": 40, "PCWP": 10, "CO": 4}, {"PVR": 7.5})],
     extra_q=[_bad({"mPAP": 8, "PCWP": 15, "CO": 5})], level="intermediate")


def _pois_R(eta, L, r):
    """Poiseuille resistance in SI from viscosity in cP (mPa s), length in cm and radius in mm."""
    return 8.0 * (eta * 1e-3) * (L * 1e-2) / (PI * (r * 1e-3) ** 4)


def poiseuille_resistance(eta, L, r):
    eta, L, r = _pos(eta, "eta"), _pos(L, "L"), _pos(r, "r")
    R = _pois_R(eta, L, r)
    return _rnd({"R": R, "R_mmHg": R * 1e-6 / 60.0 / MMHG})


def _ref_pois_Q(dP, eta, L, r):
    """Flow (m^3/s) by quadrature of the parabolic velocity profile u = dP (r^2 - s^2) / (4 eta L)."""
    e, l, a = eta * 1e-3, L * 1e-2, r * 1e-3
    return _quad(lambda s: 2 * math.pi * s * dP * (a * a - s * s) / (4 * e * l), 0.0, a)


def ref_poiseuille_resistance(eta, L, r):
    R = 1.0 / _ref_pois_Q(1.0, eta, L, r)
    return {"R": R, "R_mmHg": R * 1e-6 / 60.0 / 133.322387415}


_add("hp_poiseuille_resistance", name="Poiseuille resistance of a blood vessel", topic="cardiovascular / flow and resistance",
     purpose="Resistance to laminar flow of a cylindrical vessel R = 8 eta L/(pi r^4), in Pa s/m^3 and in mmHg min/mL.",
     inputs=[_n("eta", "blood viscosity in cP (mPa s)"), _n("L", "vessel length in cm"), _n("r", "vessel radius in mm")],
     outputs=[_o("R", "resistance in Pa s/m^3"), _o("R_mmHg", "resistance in mmHg min/mL")],
     primary="R", conditions=["eta, L, r > 0", "steady laminar Newtonian flow in a rigid tube"], method="R = 8 eta L/(pi r^4)",
     latex=r"R = \frac{8\eta L}{\pi r^4}", run=poiseuille_resistance, reference=ref_poiseuille_resistance,
     wrong=lambda eta, L, r: {"R": 8 * eta * 1e-3 * L * 1e-2 / (PI * (r * 1e-3) ** 3), "R_mmHg": 0.0},
     rand=lambda rng: {"eta": _u(rng, 2.5, 5, 1), "L": _u(rng, 0.5, 30, 1), "r": _u(rng, 0.5, 12, 1)},
     phrases=["Poiseuille resistance", "vessel resistance"],
     examples=[_ex({"eta": 3.5, "L": 10, "r": 2}, {"R": 55704230.0822, "R_mmHg": 0.00696360043278}),
               _ex({"eta": 4, "L": 5, "r": 1.5}, {"R": 100601643.041, "R_mmHg": 0.0125762378186})],
     extra_q=[_bad({"eta": 3.5, "L": 10, "r": 0})], level="intermediate")


def poiseuille_flow(dP, eta, L, r):
    dP, eta, L, r = _pos(dP, "dP"), _pos(eta, "eta"), _pos(L, "L"), _pos(r, "r")
    Q = dP * MMHG / _pois_R(eta, L, r)
    a = r * 1e-3
    return _rnd({"Q": Q * 1e6, "Q_mL_min": Q * 6e7, "v_mean": Q / (PI * a * a) * 100.0,
                 "tau_wall": 4.0 * eta * 1e-3 * Q / (PI * a ** 3)})


def ref_poiseuille_flow(dP, eta, L, r):
    import sympy as sp
    p = dP * 133.322387415
    Q = _ref_pois_Q(p, eta, L, r)
    s = sp.Symbol("s")
    e, l, a = sp.Float(eta, 30) * sp.Rational(1, 1000), sp.Float(L, 30) / 100, sp.Float(r, 30) / 1000
    u = sp.Float(p, 30) * (a ** 2 - s ** 2) / (4 * e * l)
    tau = float(-(e * sp.diff(u, s)).subs(s, a))
    return {"Q": Q * 1e6, "Q_mL_min": Q * 6e7, "v_mean": Q / (math.pi * (r * 1e-3) ** 2) * 100.0, "tau_wall": tau}


_add("hp_poiseuille_flow", name="Poiseuille flow, mean velocity and wall shear stress in a vessel",
     topic="cardiovascular / flow and resistance",
     purpose="Laminar flow Q = dP pi r^4/(8 eta L) through a vessel, its mean velocity and the wall shear stress 4 eta Q/(pi r^3).",
     inputs=[_n("dP", "pressure drop along the vessel in mmHg"), _n("eta", "blood viscosity in cP (mPa s)"),
             _n("L", "vessel length in cm"), _n("r", "vessel radius in mm")],
     outputs=[_o("Q", "flow in mL/s"), _o("Q_mL_min", "flow in mL/min"), _o("v_mean", "mean velocity in cm/s"), _o("tau_wall", "wall shear stress in Pa")],
     primary="Q", conditions=["all inputs > 0", "steady laminar Newtonian flow"], method="Q = dP pi r^4 /(8 eta L); tau_w = 4 eta Q/(pi r^3)",
     latex=r"Q = \frac{\pi r^4 \Delta P}{8 \eta L},\quad \tau_w = \frac{4\eta Q}{\pi r^3}", run=poiseuille_flow, reference=ref_poiseuille_flow,
     wrong=lambda dP, eta, L, r: (lambda q: {"Q": q * 1e6, "Q_mL_min": q * 6e7, "v_mean": q / (PI * (r * 1e-3) ** 2) * 100, "tau_wall": 0.0})(dP * MMHG * PI * (r * 1e-3) ** 4 / (4 * eta * 1e-3 * L * 1e-2)),
     rand=lambda rng: {"dP": _u(rng, 1, 40, 1), "eta": _u(rng, 2.5, 5, 1), "L": _u(rng, 1, 30, 1), "r": _u(rng, 0.5, 5, 1)},
     phrases=["Poiseuille flow", "flow through a vessel"],
     examples=[_ex({"dP": 10, "eta": 3.5, "L": 10, "r": 2}, {"Q": 23.9339790207, "Q_mL_min": 1436.03874124, "v_mean": 190.46055345, "tau_wall": 13.3322387415}),
               _ex({"dP": 5, "eta": 3, "L": 2, "r": 1}, {"Q": 4.36296492565})],
     extra_q=[_bad({"dP": 10, "eta": -3.5, "L": 10, "r": 2})], level="intermediate")


def radius_flow_ratio(r1, r2):
    r1, r2 = _pos(r1, "r1"), _pos(r2, "r2")
    q = (r2 / r1) ** 4
    return _rnd({"flow_ratio": q, "resistance_ratio": 1.0 / q})


def ref_radius_flow_ratio(r1, r2):
    q = _ref_pois_Q(1.0, 1.0, 1.0, r2) / _ref_pois_Q(1.0, 1.0, 1.0, r1)
    return {"flow_ratio": q, "resistance_ratio": 1.0 / q}


_add("hp_radius_flow_ratio", name="Effect of a change of vessel radius on flow and resistance", topic="cardiovascular / flow and resistance",
     purpose="At the same pressure drop, flow scales as r^4 and resistance as 1/r^4 (Poiseuille).",
     inputs=[_n("r1", "initial radius (any unit)"), _n("r2", "new radius (same unit)")],
     outputs=[_o("flow_ratio", "new flow / initial flow"), _o("resistance_ratio", "new resistance / initial resistance")],
     primary="flow_ratio", conditions=["r1, r2 > 0"], method="Q2/Q1 = (r2/r1)^4", latex=r"\frac{Q_2}{Q_1} = \left(\frac{r_2}{r_1}\right)^4",
     run=radius_flow_ratio, reference=ref_radius_flow_ratio,
     wrong=lambda r1, r2: {"flow_ratio": (r2 / r1) ** 2, "resistance_ratio": (r1 / r2) ** 2},
     rand=lambda rng: {"r1": _u(rng, 0.5, 5, 2), "r2": _u(rng, 0.5, 5, 2)},
     phrases=["radius change and flow", "vessel radius change"],
     examples=[_ex({"r1": 1, "r2": 2}, {"flow_ratio": 16, "resistance_ratio": 0.0625}), _ex({"r1": 1, "r2": 0.8}, {"flow_ratio": 0.4096, "resistance_ratio": 2.44140625})],
     extra_q=[_bad({"r1": 1, "r2": 0})], level="beginner")


def network_resistance(R, arrangement):
    R = _vec(R, "R", 1, 10)
    for x in R:
        _need(x > 0, "every resistance must be positive")
    _need(arrangement in ("series", "parallel"), "arrangement must be series or parallel")
    tot = sum(R) if arrangement == "series" else 1.0 / sum(1.0 / x for x in R)
    return _rnd({"R_total": tot, "G_total": 1.0 / tot})


def ref_network_resistance(R, arrangement):
    # nodal analysis: unit current injected at node 0, node n grounded; R_total = voltage at node 0
    import numpy as np
    n = len(R)
    if arrangement == "series":
        nodes, edges = n + 1, [(i, i + 1, R[i]) for i in range(n)]
    else:
        nodes, edges = 2, [(0, 1, x) for x in R]
    Lp = np.zeros((nodes, nodes))
    for a, b, x in edges:
        g = 1.0 / x
        Lp[a, a] += g
        Lp[b, b] += g
        Lp[a, b] -= g
        Lp[b, a] -= g
    rhs = np.zeros(nodes - 1)
    rhs[0] = 1.0
    v = np.linalg.solve(Lp[:-1, :-1], rhs)
    return {"R_total": float(v[0]), "G_total": 1.0 / float(v[0])}


_add("hp_vessels_series_parallel", name="Total resistance of vascular beds in series or in parallel", topic="cardiovascular / flow and resistance",
     purpose="Series beds add resistances; parallel beds (organ circulations) add conductances.",
     inputs=[_v("R", "resistances of the beds (any one unit, e.g. mmHg min/L)"), _ch("arrangement", ["series", "parallel"], "series or parallel")],
     outputs=[_o("R_total", "total resistance (same unit)"), _o("G_total", "total conductance (reciprocal of the resistance unit)")],
     primary="R_total", conditions=["every resistance > 0"], method="series: sum R; parallel: 1/sum(1/R)",
     latex=r"R_s = \sum R_i,\quad \frac{1}{R_p} = \sum \frac{1}{R_i}", run=network_resistance, reference=ref_network_resistance,
     wrong=lambda R, arrangement: {"R_total": sum(R) if arrangement == "parallel" else 1 / sum(1 / x for x in R), "G_total": 0.0},
     rand=lambda rng: {"R": [_u(rng, 1, 50, 1) for _ in range(rng.randint(2, 6))], "arrangement": rng.choice(["series", "parallel"])},
     phrases=["vessels in series or parallel", "vascular resistance network"],
     examples=[_ex({"R": [2, 3, 6], "arrangement": "parallel"}, {"R_total": 1, "G_total": 1}), _ex({"R": [2, 3, 6], "arrangement": "series"}, {"R_total": 11})],
     extra_q=[_bad({"R": [2, -3], "arrangement": "series"})], level="intermediate")


def vessel_reynolds(rho, v, D, eta):
    rho, v, D, eta = _pos(rho, "rho"), _pos(v, "v"), _pos(D, "D"), _pos(eta, "eta")
    Re = rho * (v / 100.0) * (D / 1000.0) / (eta / 1000.0)
    return _rnd({"Re": Re, "v_critical": 2000.0 * (eta / 1000.0) / (rho * D / 1000.0) * 100.0})


_add("hp_vessel_reynolds", name="Reynolds number of blood flow in a vessel", topic="cardiovascular / flow and resistance",
     purpose="Re = rho v D/eta and the mean velocity at which Re reaches 2000 (the customary laminar limit).",
     inputs=[_n("rho", "blood density in kg/m^3"), _n("v", "mean velocity in cm/s"), _n("D", "vessel diameter in mm"), _n("eta", "viscosity in cP")],
     outputs=[_o("Re", "Reynolds number"), _o("v_critical", "mean velocity at the customary laminar-limit Reynolds number in cm/s")],
     primary="Re", conditions=["all inputs > 0"], method="Re = rho v D / eta", latex=r"Re = \frac{\rho v D}{\eta}",
     run=vessel_reynolds,
     reference=lambda rho, v, D, eta: {"Re": _sx("Re*e - p*v*d", "Re", p=rho, v=v / 100, d=D / 1000, e=eta / 1000),
                                       "v_critical": 100 * _sx("2000*e - p*w*d", "w", p=rho, d=D / 1000, e=eta / 1000)},
     wrong=lambda rho, v, D, eta: {"Re": rho * v / 100 * D / 2000 / (eta / 1000), "v_critical": 4000 * eta / 1000 / (rho * D / 1000) * 100},
     rand=lambda rng: {"rho": _u(rng, 1040, 1070, 0), "v": _u(rng, 1, 60, 1), "D": _u(rng, 1, 30, 1), "eta": _u(rng, 2.5, 5, 1)},
     phrases=["vessel Reynolds number", "Reynolds number of blood flow"],
     examples=[_ex({"rho": 1060, "v": 30, "D": 25, "eta": 3.5}, {"Re": 2271.42857143, "v_critical": 26.4150943396}),
               _ex({"rho": 1050, "v": 5, "D": 4, "eta": 4}, {"Re": 52.5, "v_critical": 190.476190476})],
     extra_q=[_bad({"rho": 1060, "v": 30, "D": 0, "eta": 3.5})], level="intermediate", weak="the definition re-solved with sympy")


def laplace_wall_tension(P, r, shape, h=None):
    P, r = _pos(P, "P"), _pos(r, "r")
    _need(shape in ("cylinder", "sphere"), "shape must be cylinder or sphere")
    T = P * MMHG * r / 100.0 * (1.0 if shape == "cylinder" else 0.5)
    out = {"T": T}
    if h is not None:
        out["stress"] = T / (_pos(h, "h") / 1000.0) / 1000.0
    return _rnd(out)


def ref_laplace(P, r, shape, h=None):
    p, a = P * 133.322387415, r / 100.0
    if shape == "cylinder":
        # half a cylinder of unit length: the pressure force normal to the cut, balanced by two wall lines
        f = _quad(lambda t: p * a * math.sin(t), 0.0, math.pi)
        T = f / 2.0
    else:
        # hemisphere: pressure force on the projected disc, balanced along the rim of length 2 pi a
        f = _quad(lambda t: p * 2 * math.pi * a * a * math.sin(t) * math.cos(t), 0.0, math.pi / 2)
        T = f / (2 * math.pi * a)
    return {"T": T, "stress": T / (h / 1000.0) / 1000.0} if h is not None else {"T": T}


_add("hp_laplace_wall_tension", name="Laplace wall tension and wall stress of a vessel or chamber", topic="cardiovascular / wall mechanics",
     purpose="Law of Laplace: wall tension T = P r (cylinder) or P r/2 (sphere) and wall stress T/h.",
     inputs=[_n("P", "transmural pressure in mmHg"), _n("r", "radius in cm"),
             _ch("shape", ["cylinder", "sphere"], "cylinder (vessel) or sphere (chamber, alveolus)"),
             _n("h", "wall thickness in mm (optional; needed for the wall stress)", required=False)],
     outputs=[_o("T", "wall tension in N/m"), _o("stress", "wall stress in kPa")],
     primary="T", conditions=["P, r, h > 0", "thin wall"], method="T = P r (cylinder), P r/2 (sphere); stress = T/h",
     latex=r"T = P r \;(\text{cylinder}),\quad T = \tfrac{P r}{2} \;(\text{sphere}),\quad \sigma = T/h",
     run=laplace_wall_tension, reference=ref_laplace,
     wrong=lambda P, r, shape, h=None: (lambda T: dict({"T": T}, **({"stress": T / (h / 1000) / 1000} if h is not None else {})))(P * MMHG * r / 100 * (0.5 if shape == "cylinder" else 1.0)),
     rand=lambda rng: _drop(rng, {"P": _u(rng, 5, 200, 0), "r": _u(rng, 0.1, 5, 2), "shape": rng.choice(["cylinder", "sphere"]), "h": _u(rng, 0.2, 15, 1)}, "h"),
     phrases=["Laplace wall tension", "law of Laplace"],
     examples=[_ex({"P": 100, "r": 1.5, "shape": "cylinder", "h": 2}, {"T": 199.983581122, "stress": 99.9917905612}),
               _ex({"P": 20, "r": 5, "shape": "sphere", "h": 1}, {"T": 66.6611937075, "stress": 66.6611937075}),
               _ex({"P": 100, "r": 1.5, "shape": "cylinder"}, {"T": 199.983581122})],
     extra_q=[_bad({"P": 100, "r": 0, "shape": "cylinder", "h": 2})], level="intermediate")


def compliance(dV, dP):
    dV, dP = _pos(dV, "dV"), _pos(dP, "dP")
    return _rnd({"C": dV / dP, "E": dP / dV})


_add("hp_vascular_compliance", name="Compliance and elastance", topic="cardiovascular / wall mechanics",
     purpose="Compliance C = dV/dP and its inverse, the elastance E = dP/dV.",
     inputs=[_n("dV", "volume change in mL"), _n("dP", "pressure change in mmHg")],
     outputs=[_o("C", "compliance in mL/mmHg"), _o("E", "elastance in mmHg/mL")],
     primary="C", conditions=["dV, dP > 0"], method="C = dV/dP", latex=r"C = \frac{\Delta V}{\Delta P}",
     run=compliance, reference=lambda dV, dP: {"C": _sx("C*P - V", "C", V=dV, P=dP), "E": _sx("E*V - P", "E", V=dV, P=dP)},
     wrong=lambda dV, dP: {"C": dP / dV, "E": dV / dP},
     rand=lambda rng: {"dV": _u(rng, 5, 150, 0), "dP": _u(rng, 2, 80, 0)},
     phrases=["vascular compliance", "compliance"],
     examples=[_ex({"dV": 70, "dP": 40}, {"C": 1.75, "E": 0.571428571429}), _ex({"dV": 50, "dP": 25}, {"C": 2, "E": 0.5})],
     extra_q=[_bad({"dV": 70, "dP": 0})], level="beginner", weak="the definition re-solved with sympy")


def blood_velocity_area(Q, A):
    Q, A = _pos(Q, "Q"), _pos(A, "A")
    return _rnd({"v": Q * 1000.0 / 60.0 / A})


_add("hp_blood_velocity_area", name="Mean blood velocity from flow and total cross-sectional area", topic="cardiovascular / flow and resistance",
     purpose="Continuity: mean velocity v = Q/A (slow in the capillaries because their total area is large).",
     inputs=[_n("Q", "flow in L/min"), _n("A", "total cross-sectional area in cm^2")],
     outputs=[_o("v", "mean velocity in cm/s")],
     primary="v", conditions=["Q, A > 0"], method="v = Q/A (L/min -> cm^3/s)", latex=r"v = \frac{Q}{A}",
     run=blood_velocity_area, reference=lambda Q, A: {"v": _sx("v*A*60 - 1000*Q", "v", Q=Q, A=A)},
     wrong=lambda Q, A: {"v": Q * 1000 / A},
     rand=lambda rng: {"Q": _u(rng, 2, 10, 1), "A": _u(rng, 2, 4000, 1)},
     phrases=["mean blood velocity", "blood velocity from flow and area"],
     examples=[_ex({"Q": 5, "A": 4}, {"v": 20.8333333333}), _ex({"Q": 5, "A": 3000}, {"v": 0.0277777777778})],
     extra_q=[_bad({"Q": 5, "A": 0})], level="beginner", weak="continuity re-solved with sympy")


def bernoulli_drop(v1, v2, rho):
    v1, v2, rho = _nonneg(v1, "v1"), _pos(v2, "v2"), _pos(rho, "rho")
    _need(v2 > v1, "the downstream (jet) velocity v2 must exceed v1")
    dp = 0.5 * rho * (v2 * v2 - v1 * v1)
    return _rnd({"dP": dp, "dP_mmHg": dp / MMHG, "dP_simplified": 4.0 * v2 * v2})


def ref_bernoulli(v1, v2, rho):
    dp = _quad(lambda v: rho * v, v1, v2)      # Euler's equation along a streamline: dp = -rho v dv
    return {"dP": dp, "dP_mmHg": dp / 133.322387415, "dP_simplified": _sx("P - 4*v**2", "P", v=v2)}


_add("hp_bernoulli_pressure_drop", name="Bernoulli pressure drop across a narrowing", topic="cardiovascular / flow and resistance",
     purpose="Convective pressure drop 0.5 rho (v2^2 - v1^2) across a stenosis or valve, and the simplified 4 v^2 (mmHg, v in m/s).",
     inputs=[_n("v1", "upstream velocity in m/s"), _n("v2", "jet velocity in m/s"), _n("rho", "blood density in kg/m^3")],
     outputs=[_o("dP", "pressure drop in Pa"), _o("dP_mmHg", "pressure drop in mmHg"), _o("dP_simplified", "pressure drop by the simplified Bernoulli equation in mmHg")],
     primary="dP_mmHg", conditions=["v2 > v1 >= 0", "inviscid steady flow"], method="dP = rho (v2^2 - v1^2)/2",
     latex=r"\Delta P = \tfrac{1}{2}\rho\,(v_2^2 - v_1^2) \approx 4 v_2^2", run=bernoulli_drop, reference=ref_bernoulli,
     wrong=lambda v1, v2, rho: (lambda d: {"dP": d, "dP_mmHg": d / MMHG, "dP_simplified": 4 * v2 * v2})(rho * (v2 - v1) ** 2),
     rand=lambda rng: (lambda a: {"v1": a, "v2": round(a + rng.uniform(0.5, 5), 2), "rho": _u(rng, 1040, 1070, 0)})(_u(rng, 0, 1.5, 2)),
     phrases=["Bernoulli pressure drop"],
     examples=[_ex({"v1": 1, "v2": 4, "rho": 1060}, {"dP": 7950, "dP_mmHg": 59.6298952797, "dP_simplified": 64}),
               _ex({"v1": 0.5, "v2": 3, "rho": 1000}, {"dP": 4375, "dP_mmHg": 32.8151939432})],
     extra_q=[_bad({"v1": 4, "v2": 1, "rho": 1060})], level="intermediate")


def stroke_work(SV, MAP, HR=None):
    SV, MAP = _pos(SV, "SV"), _pos(MAP, "MAP")
    w = MAP * MMHG * SV * 1e-6
    out = {"SW": w, "SW_gm": w / (G0 * 1e-3)}
    if HR is not None:
        out["power"] = w * _pos(HR, "HR") / 60.0
    return _rnd(out)


def ref_stroke_work(SV, MAP, HR=None):
    w = _quad(lambda v: MAP * 133.322387415, 0.0, SV * 1e-6)      # integral of P dV at the mean ejection pressure
    out = {"SW": w, "SW_gm": w / 0.00980665}
    if HR is not None:
        out["power"] = _sx("W*60 - w*h", "W", w=w, h=HR)
    return out


_add("hp_stroke_work", name="Stroke work and cardiac power", topic="cardiovascular / cardiac work",
     purpose="External stroke work SW = MAP x SV (J and g m) and the cardiac power output SW x HR.",
     inputs=[_n("SV", "stroke volume in mL"), _n("MAP", "mean arterial (ejection) pressure in mmHg"),
             _n("HR", "heart rate in beats/min (optional; needed for the cardiac power)", required=False)],
     outputs=[_o("SW", "stroke work in J"), _o("SW_gm", "stroke work in g m"), _o("power", "cardiac power output in W")],
     primary="SW", conditions=["SV, MAP, HR > 0"], method="SW = MAP SV (pressure-volume work); power = SW HR/60",
     latex=r"SW = \bar P\, SV,\quad \dot W = SW \cdot HR", run=stroke_work, reference=ref_stroke_work,
     wrong=lambda SV, MAP, HR=None: (lambda w: dict({"SW": w, "SW_gm": w / (G0 * 1e-3)}, **({"power": w * HR} if HR is not None else {})))(MAP * SV * 1e-6 * 1000),
     rand=lambda rng: _drop(rng, {"SV": _u(rng, 30, 120, 0), "MAP": _u(rng, 60, 130, 0), "HR": _u(rng, 50, 150, 0)}, "HR"),
     phrases=["stroke work", "cardiac power"],
     examples=[_ex({"SV": 70, "MAP": 100, "HR": 72}, {"SW": 0.933256711905, "power": 1.11990805429}),
               _ex({"SV": 80, "MAP": 90, "HR": 60}, {"SW": 0.959921189388, "power": 0.959921189388}),
               _ex({"SV": 70, "MAP": 100}, {"SW": 0.933256711905})],
     extra_q=[_bad({"SV": 70, "MAP": 0, "HR": 72})], level="intermediate")


def _teich(D):
    return 7.0 / (2.4 + D) * D ** 3


def teichholz(LVIDd, LVIDs):
    LVIDd, LVIDs = _pos(LVIDd, "LVIDd"), _pos(LVIDs, "LVIDs")
    _need(LVIDs < LVIDd, "the end-systolic diameter must be smaller than the end-diastolic")
    edv, esv = _teich(LVIDd), _teich(LVIDs)
    return _rnd({"EDV": edv, "ESV": esv, "EF": (edv - esv) / edv, "FS": (LVIDd - LVIDs) / LVIDd})


def ref_teichholz(LVIDd, LVIDs):
    edv = _sx("V*(24/10 + D) - 7*D**3", "V", D=LVIDd)
    esv = _sx("V*(24/10 + D) - 7*D**3", "V", D=LVIDs)
    return {"EDV": edv, "ESV": esv, "EF": 1 - esv / edv, "FS": 1 - LVIDs / LVIDd}


_add("hp_teichholz_volumes", name="Left-ventricular volumes and ejection fraction by the Teichholz formula",
     topic="cardiovascular / cardiac output",
     purpose="Teichholz volume V = 7 D^3/(2.4 + D) from M-mode internal diameters, EF and fractional shortening.",
     inputs=[_n("LVIDd", "end-diastolic internal diameter in cm"), _n("LVIDs", "end-systolic internal diameter in cm")],
     outputs=[_o("EDV", "end-diastolic volume in mL"), _o("ESV", "end-systolic volume in mL"), _o("EF", "ejection fraction"),
              _o("FS", "fractional shortening")],
     primary="EF", conditions=["0 < LVIDs < LVIDd"], method="V = 7 D^3/(2.4 + D) (D in cm, V in mL)",
     latex=r"V = \frac{7 D^3}{2.4 + D}", run=teichholz, reference=ref_teichholz,
     wrong=lambda LVIDd, LVIDs: {"EDV": LVIDd ** 3, "ESV": LVIDs ** 3, "EF": 1 - (LVIDs / LVIDd) ** 3, "FS": (LVIDd - LVIDs) / LVIDd},
     rand=lambda rng: (lambda d: {"LVIDd": d, "LVIDs": round(d * rng.uniform(0.5, 0.85), 2)})(_u(rng, 3.8, 7, 2)),
     phrases=["Teichholz volumes", "Teichholz ejection fraction"],
     examples=[_ex({"LVIDd": 5, "LVIDs": 3.2}, {"EDV": 118.243243243, "ESV": 40.96, "EF": 0.653595428571, "FS": 0.36}),
               _ex({"LVIDd": 5.5, "LVIDs": 4.5}, {"EDV": 147.420886076, "EF": 0.372913468134})],
     extra_q=[_bad({"LVIDd": 3, "LVIDs": 5})], level="advanced", weak="the published formula re-solved with sympy")


def qt_correction(QT, HR):
    QT, HR = _pos(QT, "QT"), _pos(HR, "HR")
    rr = 60.0 / HR
    return _rnd({"RR": rr, "QTc_Bazett": QT / math.sqrt(rr), "QTc_Fridericia": QT / rr ** (1.0 / 3.0),
                 "QTc_Framingham": QT + 154.0 * (1.0 - rr)})


def ref_qt(QT, HR):
    import mpmath as mp
    mp.mp.dps = 30
    rr = mp.mpf(60) / mp.mpf(repr(float(HR)))
    q = mp.mpf(repr(float(QT)))
    return {"RR": float(rr), "QTc_Bazett": float(q * rr ** mp.mpf(-0.5)), "QTc_Fridericia": float(q * mp.cbrt(1 / rr)),
            "QTc_Framingham": float(q + 154 * (1 - rr))}


_add("hp_qt_correction", name="Heart-rate corrected QT interval (Bazett, Fridericia, Framingham)", topic="cardiovascular / electrocardiogram",
     purpose="RR = 60/HR and the corrected QT: Bazett QT/sqrt(RR), Fridericia QT/RR^(1/3), Framingham QT + 154 (1 - RR).",
     inputs=[_n("QT", "measured QT interval in ms"), _n("HR", "heart rate in beats/min")],
     outputs=[_o("RR", "RR interval in s"), _o("QTc_Bazett", "Bazett QTc in ms"), _o("QTc_Fridericia", "Fridericia QTc in ms"),
              _o("QTc_Framingham", "Framingham QTc in ms")],
     primary="QTc_Bazett", conditions=["QT, HR > 0"], method="RR in seconds; the three published corrections",
     latex=r"QT_c = \frac{QT}{\sqrt{RR}} \;(\text{Bazett}),\quad \frac{QT}{RR^{1/3}} \;(\text{Fridericia})",
     run=qt_correction, reference=ref_qt,
     wrong=lambda QT, HR: {"RR": HR / 60, "QTc_Bazett": QT / math.sqrt(HR / 60), "QTc_Fridericia": QT / (HR / 60) ** (1 / 3), "QTc_Framingham": QT},
     rand=lambda rng: {"QT": _u(rng, 300, 500, 0), "HR": _u(rng, 45, 130, 0)},
     phrases=["corrected QT interval", "QT correction"],
     examples=[_ex({"QT": 400, "HR": 75}, {"RR": 0.8, "QTc_Bazett": 447.2135955, "QTc_Fridericia": 430.886938006, "QTc_Framingham": 430.8}),
               _ex({"QT": 360, "HR": 100}, {"QTc_Bazett": 464.758001545, "QTc_Fridericia": 426.827196539})],
     extra_q=[_bad({"QT": 400, "HR": 0})], level="intermediate", weak="the published corrections evaluated in 30-digit mpmath")


def qrs_axis(lead_I, aVF):
    lead_I, aVF = _num(lead_I, "lead_I"), _num(aVF, "aVF")
    _need(lead_I != 0 or aVF != 0, "at least one net deflection must be nonzero")
    return _rnd({"axis": math.degrees(math.atan2(aVF, lead_I)), "amplitude": math.hypot(lead_I, aVF)})


def ref_qrs_axis(lead_I, aVF):
    # fit a frontal-plane vector (A, theta) whose projections on lead I (0 deg) and aVF (+90 deg) are the deflections
    from scipy.optimize import least_squares
    a0 = math.hypot(lead_I, aVF)
    best = None
    for start in range(-180, 180, 45):
        r = least_squares(lambda x: [x[0] * math.cos(math.radians(x[1])) - lead_I, x[0] * math.sin(math.radians(x[1])) - aVF],
                          [a0, start + 22.5], xtol=1e-15, ftol=1e-15, gtol=1e-15)
        if r.x[0] > 0 and (best is None or r.cost < best.cost):
            best = r
    th = (best.x[1] + 180.0) % 360.0 - 180.0
    return {"axis": th, "amplitude": float(best.x[0])}


_add("hp_qrs_axis", name="Mean QRS (electrical) axis from leads I and aVF", topic="cardiovascular / electrocardiogram",
     purpose="The frontal-plane mean electrical axis atan2(aVF, I) in degrees (hexaxial convention: lead I at 0 deg, aVF at +90 deg).",
     inputs=[_n("lead_I", "net QRS deflection in lead I (mm or mV)"), _n("aVF", "net QRS deflection in aVF (same unit)")],
     outputs=[_o("axis", "mean QRS axis in degrees (principal value)"), _o("amplitude", "length of the mean vector (same unit)")],
     primary="axis", conditions=["not both zero"], method="axis = atan2(aVF, I)", latex=r"\theta = \operatorname{atan2}(aVF, I)",
     run=qrs_axis, reference=ref_qrs_axis,
     wrong=lambda lead_I, aVF: {"axis": math.degrees(math.atan(aVF / lead_I)) if lead_I else 90.0, "amplitude": abs(lead_I) + abs(aVF)},
     rand=lambda rng: {"lead_I": round(rng.choice([-1, 1]) * rng.uniform(0.2, 15), 1), "aVF": round(rng.choice([-1, 1]) * rng.uniform(0.2, 15), 1)},
     phrases=["QRS axis", "mean electrical axis"],
     examples=[_ex({"lead_I": 0.5, "aVF": -0.5}, {"axis": -45}), _ex({"lead_I": -0.2, "aVF": 0.8}, {"axis": 104.036243468, "amplitude": 0.824621125124})],
     extra_q=[_bad({"lead_I": 0, "aVF": 0})], level="intermediate")


def starling_filtration(Pc, Pi, pic, pii, Kf, sigma=1.0):
    Pc, Pi, pic, pii, Kf = _num(Pc, "Pc"), _num(Pi, "Pi"), _nonneg(pic, "pic"), _nonneg(pii, "pii"), _pos(Kf, "Kf")
    sigma = _fr(sigma, "sigma")
    nfp = (Pc - Pi) - sigma * (pic - pii)
    return _rnd({"NFP": nfp, "Jv": Kf * nfp})


def ref_starling(Pc, Pi, pic, pii, Kf, sigma=1.0):
    n = _sx("N - (Pc - Pi) + s*(a - b)", "N", Pc=Pc, Pi=Pi, s=sigma, a=pic, b=pii)
    return {"NFP": n, "Jv": _sx("J - K*N", "J", K=Kf, N=n)}


_add("hp_starling_filtration", name="Starling forces and capillary filtration", topic="cardiovascular / microcirculation",
     purpose="Net filtration pressure (Pc - Pi) - sigma (pi_c - pi_i) and the filtration rate Jv = Kf x NFP (glomerular or systemic capillaries).",
     inputs=[_n("Pc", "capillary hydrostatic pressure in mmHg"), _n("Pi", "interstitial (or Bowman's space) hydrostatic pressure in mmHg"),
             _n("pic", "capillary oncotic pressure in mmHg"), _n("pii", "interstitial oncotic pressure in mmHg"),
             _n("Kf", "filtration coefficient in mL/min per mmHg"), _n("sigma", "reflection coefficient (optional; ideal membrane if omitted)", required=False, pct=True)],
     outputs=[_o("NFP", "net filtration pressure in mmHg (negative = absorption)"), _o("Jv", "filtration rate in mL/min")],
     primary="Jv", conditions=["Kf > 0", "0 <= sigma <= 1"], method="Jv = Kf [(Pc - Pi) - sigma (pi_c - pi_i)]",
     latex=r"J_v = K_f\left[(P_c - P_i) - \sigma(\pi_c - \pi_i)\right]", run=starling_filtration, reference=ref_starling,
     wrong=lambda Pc, Pi, pic, pii, Kf, sigma=1.0: {"NFP": (Pc - Pi) + sigma * (pic - pii), "Jv": Kf * ((Pc - Pi) + sigma * (pic - pii))},
     rand=lambda rng: {"Pc": _u(rng, 10, 60, 0), "Pi": _u(rng, -5, 15, 0), "pic": _u(rng, 20, 35, 0), "pii": _u(rng, 0, 8, 0),
                       "Kf": _u(rng, 0.1, 15, 1), "sigma": _u(rng, 0.7, 1.0, 2)},
     phrases=["Starling filtration", "net filtration pressure"],
     examples=[_ex({"Pc": 35, "Pi": 0, "pic": 28, "pii": 3, "Kf": 0.5}, {"NFP": 10, "Jv": 5}),
               _ex({"Pc": 55, "Pi": 15, "pic": 30, "pii": 0, "Kf": 12.5}, {"NFP": 10, "Jv": 125})],
     extra_q=[_bad({"Pc": 35, "Pi": 0, "pic": 28, "pii": 3, "Kf": 0})], level="intermediate", weak="the defining relation re-solved with sympy")


def hydrostatic_column(h, rho):
    h, rho = _num(h, "h"), _pos(rho, "rho")
    p = rho * G0 * h / 100.0
    return _rnd({"dP": p, "dP_mmHg": p / MMHG})


def ref_hydrostatic(h, rho):
    p = _quad(lambda z: rho * 9.80665, 0.0, h / 100.0) if h >= 0 else -_quad(lambda z: rho * 9.80665, 0.0, -h / 100.0)
    return {"dP": p, "dP_mmHg": p / 133.322387415}


_add("hp_hydrostatic_pressure", name="Hydrostatic pressure of a blood column", topic="cardiovascular / blood pressure",
     purpose="Pressure difference rho g h between two points a height h apart in a column of blood (posture effects).",
     inputs=[_n("h", "height of the column in cm (positive = the lower point)"), _n("rho", "blood density in kg/m^3")],
     outputs=[_o("dP", "pressure difference in Pa"), _o("dP_mmHg", "pressure difference in mmHg")],
     primary="dP_mmHg", conditions=["rho > 0"], method="dP = rho g h", latex=r"\Delta P = \rho g h",
     run=hydrostatic_column, reference=ref_hydrostatic,
     wrong=lambda h, rho: {"dP": rho * h / 100, "dP_mmHg": rho * h / 100 / MMHG},
     rand=lambda rng: {"h": _u(rng, 1, 180, 0), "rho": _u(rng, 1000, 1070, 0)},
     phrases=["hydrostatic pressure of a blood column", "hydrostatic pressure"],
     examples=[_ex({"h": 130, "rho": 1060}, {"dP": 13513.5637, "dP_mmHg": 101.360048841}), _ex({"h": 50, "rho": 1000}, {"dP_mmHg": 36.7779567638})],
     extra_q=[_bad({"h": 130, "rho": 0})], level="beginner")


def moens_korteweg(E, h, D, rho):
    E, h, D, rho = _pos(E, "E"), _pos(h, "h"), _pos(D, "D"), _pos(rho, "rho")
    return _rnd({"PWV": math.sqrt(E * 1000.0 * (h / 1000.0) / (rho * D / 1000.0))})


_add("hp_moens_korteweg", name="Pulse wave velocity by the Moens-Korteweg equation", topic="cardiovascular / wall mechanics",
     purpose="Pulse wave velocity of a thin elastic tube PWV = sqrt(E h/(rho D)).",
     inputs=[_n("E", "wall elastic modulus in kPa"), _n("h", "wall thickness in mm"), _n("D", "vessel diameter in mm"), _n("rho", "blood density in kg/m^3")],
     outputs=[_o("PWV", "pulse wave velocity in m/s")],
     primary="PWV", conditions=["all inputs > 0", "thin-walled elastic tube"], method="PWV = sqrt(E h /(rho D))",
     latex=r"PWV = \sqrt{\frac{E h}{\rho D}}", run=moens_korteweg,
     reference=lambda E, h, D, rho: {"PWV": math.sqrt(_sx("s*p*d - e*t", "s", e=E * 1000, t=h / 1000, p=rho, d=D / 1000))},
     wrong=lambda E, h, D, rho: {"PWV": math.sqrt(E * 1000 * h / 1000 / (2 * rho * D / 1000))},
     rand=lambda rng: {"E": _u(rng, 100, 3000, 0), "h": _u(rng, 0.3, 3, 2), "D": _u(rng, 3, 30, 1), "rho": _u(rng, 1040, 1070, 0)},
     phrases=["Moens-Korteweg pulse wave velocity", "pulse wave velocity"],
     examples=[_ex({"E": 500, "h": 1.5, "D": 25, "rho": 1060}, {"PWV": 5.31995176599}), _ex({"E": 1000, "h": 1, "D": 20, "rho": 1050}, {"PWV": 6.90065559342})],
     extra_q=[_bad({"E": 0, "h": 1.5, "D": 25, "rho": 1060})], level="advanced", weak="the published equation re-solved with sympy")


def womersley(r, HR, rho, eta):
    r, HR, rho, eta = _pos(r, "r"), _pos(HR, "HR"), _pos(rho, "rho"), _pos(eta, "eta")
    w = 2.0 * PI * HR / 60.0
    return _rnd({"alpha": (r / 1000.0) * math.sqrt(w * rho / (eta / 1000.0)), "omega": w})


_add("hp_womersley_number", name="Womersley number of pulsatile blood flow", topic="cardiovascular / flow and resistance",
     purpose="alpha = r sqrt(omega rho/eta) with omega = 2 pi HR/60: the ratio of pulsatile inertia to viscous forces.",
     inputs=[_n("r", "vessel radius in mm"), _n("HR", "heart rate in beats/min"), _n("rho", "blood density in kg/m^3"), _n("eta", "viscosity in cP")],
     outputs=[_o("alpha", "Womersley number"), _o("omega", "angular frequency in rad/s")],
     primary="alpha", conditions=["all inputs > 0"], method="alpha = r sqrt(omega rho / eta)", latex=r"\alpha = r\sqrt{\frac{\omega\rho}{\eta}}",
     run=womersley,
     reference=lambda r, HR, rho, eta: (lambda w: {"alpha": math.sqrt(_sx("a2*e - R**2*w*p", "a2", R=r / 1000, w=w, p=rho, e=eta / 1000)), "omega": w})(_sx("w*60 - 2*pi*h", "w", h=HR)),
     wrong=lambda r, HR, rho, eta: {"alpha": (r / 1000) * math.sqrt(HR / 60 * rho / (eta / 1000)), "omega": HR / 60},
     rand=lambda rng: {"r": _u(rng, 0.5, 15, 1), "HR": _u(rng, 45, 150, 0), "rho": _u(rng, 1040, 1070, 0), "eta": _u(rng, 2.5, 5, 1)},
     phrases=["Womersley number"],
     examples=[_ex({"r": 12.5, "HR": 72, "rho": 1060, "eta": 3.5}, {"alpha": 18.8890223584}), _ex({"r": 2, "HR": 60, "rho": 1050, "eta": 4}, {"alpha": 2.56852965187})],
     extra_q=[_bad({"r": 12.5, "HR": 0, "rho": 1060, "eta": 3.5})], level="expert", weak="the definition re-solved with sympy")


def qp_qs(SaO2, SvO2, SpvO2, SpaO2):
    Sa, Sv, Spv, Spa = (_fr(SaO2, "SaO2"), _fr(SvO2, "SvO2"), _fr(SpvO2, "SpvO2"), _fr(SpaO2, "SpaO2"))
    _need(Sa > Sv and Spv > Spa, "need SaO2 > SvO2 and SpvO2 > SpaO2")
    return _rnd({"Qp_Qs": (Sa - Sv) / (Spv - Spa)})


def ref_qp_qs(SaO2, SvO2, SpvO2, SpaO2):
    # oxygen uptake in the lungs equals oxygen delivered to the tissues: Qp (Spv - Spa) = Qs (Sa - Sv), with Qs = 1
    import numpy as np
    A = np.array([[SpvO2 - SpaO2, -(SaO2 - SvO2)], [0.0, 1.0]])
    x = np.linalg.solve(A, np.array([0.0, 1.0]))
    return {"Qp_Qs": float(x[0] / x[1])}


_add("hp_qp_qs", name="Pulmonary-to-systemic flow ratio Qp/Qs", topic="cardiovascular / shunts",
     purpose="Qp/Qs = (SaO2 - SvO2)/(SpvO2 - SpaO2) from oxygen saturations (hemoglobin and dissolved O2 cancel or are neglected).",
     inputs=[_n("SaO2", "systemic arterial saturation", pct=True), _n("SvO2", "mixed venous saturation", pct=True),
             _n("SpvO2", "pulmonary venous saturation", pct=True), _n("SpaO2", "pulmonary artery saturation", pct=True)],
     outputs=[_o("Qp_Qs", "pulmonary to systemic flow ratio")],
     primary="Qp_Qs", conditions=["SaO2 > SvO2", "SpvO2 > SpaO2"], method="Fick principle applied to both circulations",
     latex=r"\frac{Q_p}{Q_s} = \frac{S_aO_2 - S_{\bar v}O_2}{S_{pv}O_2 - S_{pa}O_2}", run=qp_qs, reference=ref_qp_qs,
     wrong=lambda SaO2, SvO2, SpvO2, SpaO2: {"Qp_Qs": (SpvO2 - SpaO2) / (SaO2 - SvO2)},
     rand=lambda rng: (lambda sv: {"SvO2": sv, "SpaO2": round(sv + rng.uniform(0.03, 0.15), 2), "SaO2": _u(rng, 0.88, 0.97, 2), "SpvO2": _u(rng, 0.98, 1.0, 2)})(_u(rng, 0.55, 0.75, 2)),
     phrases=["Qp/Qs", "pulmonary to systemic flow ratio"],
     examples=[_ex({"SaO2": 0.95, "SvO2": 0.65, "SpvO2": 0.98, "SpaO2": 0.8}, {"Qp_Qs": 1.66666666667}),
               _ex({"SaO2": 0.97, "SvO2": 0.7, "SpvO2": 0.98, "SpaO2": 0.85}, {"Qp_Qs": 2.07692307692})],
     extra_q=[_bad({"SaO2": 0.6, "SvO2": 0.65, "SpvO2": 0.98, "SpaO2": 0.8})], level="advanced")


def shunt_fraction(CcO2, CaO2, CvO2):
    Cc, Ca, Cv = _pos(CcO2, "CcO2"), _pos(CaO2, "CaO2"), _nonneg(CvO2, "CvO2")
    _need(Cc >= Ca > Cv, "need CcO2 >= CaO2 > CvO2")
    return _rnd({"Qs_Qt": (Cc - Ca) / (Cc - Cv)})


def ref_shunt(CcO2, CaO2, CvO2):
    # mixing: arterial blood = (1 - f) end-capillary blood + f mixed venous blood
    import numpy as np
    A = np.array([[CvO2 - CcO2]])
    f = np.linalg.solve(A, np.array([CaO2 - CcO2]))
    return {"Qs_Qt": float(f[0])}


_add("hp_shunt_fraction", name="Physiological shunt fraction (shunt equation)", topic="respiratory / gas exchange",
     purpose="Qs/Qt = (Cc'O2 - CaO2)/(Cc'O2 - CvO2) from end-capillary, arterial and mixed venous O2 contents.",
     inputs=[_n("CcO2", "end-capillary O2 content in mL/dL"), _n("CaO2", "arterial O2 content in mL/dL"), _n("CvO2", "mixed venous O2 content in mL/dL")],
     outputs=[_o("Qs_Qt", "shunt fraction")],
     primary="Qs_Qt", conditions=["CcO2 >= CaO2 > CvO2"], method="mixing balance of shunted and capillary blood",
     latex=r"\frac{Q_s}{Q_t} = \frac{C_{c'}O_2 - C_aO_2}{C_{c'}O_2 - C_{\bar v}O_2}", run=shunt_fraction, reference=ref_shunt,
     wrong=lambda CcO2, CaO2, CvO2: {"Qs_Qt": (CcO2 - CaO2) / (CaO2 - CvO2)},
     rand=lambda rng: (lambda cc: {"CcO2": cc, "CaO2": round(cc - rng.uniform(0.2, 3), 2), "CvO2": round(cc - rng.uniform(4, 8), 2)})(_u(rng, 17, 22, 2)),
     phrases=["shunt fraction", "shunt equation"],
     examples=[_ex({"CcO2": 21, "CaO2": 20, "CvO2": 15}, {"Qs_Qt": 0.166666666667}), _ex({"CcO2": 20.5, "CaO2": 18, "CvO2": 14}, {"Qs_Qt": 0.384615384615})],
     extra_q=[_bad({"CcO2": 18, "CaO2": 20, "CvO2": 15})], level="advanced")


# ================================================================================================ respiratory
def lung_volumes(TV, IRV, ERV, RV):
    TV, IRV, ERV, RV = _pos(TV, "TV"), _nonneg(IRV, "IRV"), _nonneg(ERV, "ERV"), _pos(RV, "RV")
    return _rnd({"IC": TV + IRV, "FRC": ERV + RV, "VC": IRV + TV + ERV, "TLC": IRV + TV + ERV + RV})


def ref_lung_volumes(TV, IRV, ERV, RV):
    import numpy as np
    M = np.array([[1, 1, 0, 0], [0, 0, 1, 1], [1, 1, 1, 0], [1, 1, 1, 1]], dtype=float)
    v = M @ np.array([TV, IRV, ERV, RV], dtype=float)
    return {"IC": float(v[0]), "FRC": float(v[1]), "VC": float(v[2]), "TLC": float(v[3])}


_add("hp_lung_capacities", name="Lung capacities from the four lung volumes", topic="respiratory / lung volumes",
     purpose="Inspiratory capacity, functional residual capacity, vital capacity and total lung capacity from TV, IRV, ERV and RV.",
     inputs=[_n("TV", "tidal volume in mL"), _n("IRV", "inspiratory reserve volume in mL"), _n("ERV", "expiratory reserve volume in mL"),
             _n("RV", "residual volume in mL")],
     outputs=[_o("IC", "inspiratory capacity in mL"), _o("FRC", "functional residual capacity in mL"), _o("VC", "vital capacity in mL"),
              _o("TLC", "total lung capacity in mL")],
     primary="TLC", conditions=["TV, RV > 0", "IRV, ERV >= 0"], method="IC = TV + IRV; FRC = ERV + RV; VC = IRV + TV + ERV; TLC = VC + RV",
     latex=r"TLC = IRV + TV + ERV + RV", run=lung_volumes, reference=ref_lung_volumes,
     wrong=lambda TV, IRV, ERV, RV: {"IC": TV + IRV, "FRC": ERV, "VC": IRV + TV + ERV + RV, "TLC": IRV + ERV + RV},
     rand=lambda rng: {"TV": _u(rng, 300, 800, 0), "IRV": _u(rng, 1500, 3500, 0), "ERV": _u(rng, 600, 1500, 0), "RV": _u(rng, 800, 2000, 0)},
     phrases=["lung capacities", "lung volumes and capacities"],
     examples=[_ex({"TV": 500, "IRV": 3000, "ERV": 1100, "RV": 1200}, {"IC": 3500, "FRC": 2300, "VC": 4600, "TLC": 5800}),
               _ex({"TV": 400, "IRV": 2000, "ERV": 900, "RV": 1000}, {"TLC": 4300})],
     extra_q=[_bad({"TV": 0, "IRV": 3000, "ERV": 1100, "RV": 1200})], level="beginner", weak="the sums evaluated as a matrix product")


def minute_ventilation(VT, RR, VD):
    VT, RR, VD = _pos(VT, "VT"), _pos(RR, "RR"), _nonneg(VD, "VD")
    _need(VD < VT, "the dead space must be smaller than the tidal volume")
    return _rnd({"VE": VT * RR / 1000.0, "VA": (VT - VD) * RR / 1000.0, "VD_VT": VD / VT})


_add("hp_minute_ventilation", name="Minute and alveolar ventilation", topic="respiratory / ventilation",
     purpose="Minute ventilation VE = VT x RR, alveolar ventilation VA = (VT - VD) x RR and the dead-space fraction VD/VT.",
     inputs=[_n("VT", "tidal volume in mL"), _n("RR", "respiratory rate in breaths/min"), _n("VD", "dead space in mL")],
     outputs=[_o("VE", "minute ventilation in L/min"), _o("VA", "alveolar ventilation in L/min"), _o("VD_VT", "dead space fraction")],
     primary="VA", conditions=["0 <= VD < VT", "RR > 0"], method="VE = VT f; VA = (VT - VD) f", latex=r"\dot V_A = (V_T - V_D) f",
     run=minute_ventilation,
     reference=lambda VT, RR, VD: {"VE": _sx("E*1000 - T*f", "E", T=VT, f=RR), "VA": _sx("A*1000 - (T - D)*f", "A", T=VT, D=VD, f=RR),
                                   "VD_VT": _sx("x*T - D", "x", T=VT, D=VD)},
     wrong=lambda VT, RR, VD: {"VE": VT * RR / 1000, "VA": VT * RR / 1000 - VD / 1000, "VD_VT": VD / VT},
     rand=lambda rng: (lambda t: {"VT": t, "RR": _u(rng, 6, 35, 0), "VD": _u(rng, 0.15 * t, 0.6 * t, 0)})(_u(rng, 250, 900, 0)),
     phrases=["minute ventilation", "alveolar ventilation"],
     examples=[_ex({"VT": 500, "RR": 12, "VD": 150}, {"VE": 6, "VA": 4.2, "VD_VT": 0.3}), _ex({"VT": 300, "RR": 30, "VD": 150}, {"VE": 9, "VA": 4.5})],
     extra_q=[_bad({"VT": 500, "RR": 12, "VD": 600})], level="beginner", weak="the definitions re-solved with sympy")


def bohr_dead_space(PaCO2, PECO2, VT):
    PaCO2, PECO2, VT = _pos(PaCO2, "PaCO2"), _nonneg(PECO2, "PECO2"), _pos(VT, "VT")
    _need(PECO2 < PaCO2, "the mixed expired PCO2 must be below the arterial PCO2")
    f = (PaCO2 - PECO2) / PaCO2
    return _rnd({"VD_VT": f, "VD": f * VT})


def ref_bohr(PaCO2, PECO2, VT):
    # CO2 balance: all expired CO2 comes from alveolar gas, VT FE = (VT - VD) FA (inspired CO2 = 0)
    vd = _sx("T*E - (T - D)*A", "D", T=VT, E=PECO2, A=PaCO2)
    return {"VD_VT": vd / VT, "VD": vd}


_add("hp_bohr_dead_space", name="Physiological dead space by the Bohr (Enghoff) equation", topic="respiratory / ventilation",
     purpose="VD/VT = (PaCO2 - PECO2)/PaCO2 and the dead-space volume, with arterial PCO2 standing for alveolar (Enghoff).",
     inputs=[_n("PaCO2", "arterial PCO2 in mmHg"), _n("PECO2", "mixed expired PCO2 in mmHg"), _n("VT", "tidal volume in mL")],
     outputs=[_o("VD_VT", "dead space fraction"), _o("VD", "dead space volume in mL")],
     primary="VD_VT", conditions=["0 <= PECO2 < PaCO2"], method="Bohr CO2 mass balance", latex=r"\frac{V_D}{V_T} = \frac{P_aCO_2 - P_ECO_2}{P_aCO_2}",
     run=bohr_dead_space, reference=ref_bohr,
     wrong=lambda PaCO2, PECO2, VT: {"VD_VT": (PaCO2 - PECO2) / PECO2, "VD": (PaCO2 - PECO2) / PECO2 * VT},
     rand=lambda rng: (lambda a: {"PaCO2": a, "PECO2": round(a * rng.uniform(0.4, 0.85), 1), "VT": _u(rng, 300, 800, 0)})(_u(rng, 30, 60, 1)),
     phrases=["Bohr dead space", "physiological dead space"],
     examples=[_ex({"PaCO2": 40, "PECO2": 28, "VT": 500}, {"VD_VT": 0.3, "VD": 150}), _ex({"PaCO2": 50, "PECO2": 25, "VT": 400}, {"VD_VT": 0.5, "VD": 200})],
     extra_q=[_bad({"PaCO2": 30, "PECO2": 40, "VT": 500})], level="intermediate", weak="the CO2 balance re-solved with sympy")


def alveolar_ventilation_co2(VCO2, PaCO2):
    VCO2, PaCO2 = _pos(VCO2, "VCO2"), _pos(PaCO2, "PaCO2")
    return _rnd({"VA": VCO2 * K_ALV / PaCO2 / 1000.0})


def ref_va_co2(VCO2, PaCO2):
    # mole count of the CO2 output (STPD: 760 mmHg, 273.15 K), then the alveolar volume at 310.15 K carrying it at PACO2
    mm = 133.322387415
    n = 760 * mm * VCO2 * 1e-6 / (8.314462618 * 273.15)
    return {"VA": n * 8.314462618 * 310.15 / (PaCO2 * mm) * 1000.0}


_add("hp_alveolar_ventilation_co2", name="Alveolar ventilation from CO2 output and arterial PCO2", topic="respiratory / ventilation",
     purpose="Alveolar ventilation equation VA (BTPS) = VCO2 (STPD) x 863/PaCO2, with K = 760 x 310.15/273.15 mmHg.",
     inputs=[_n("VCO2", "CO2 output in mL/min (STPD)"), _n("PaCO2", "arterial (alveolar) PCO2 in mmHg")],
     outputs=[_o("VA", "alveolar ventilation in L/min (BTPS)")],
     primary="VA", conditions=["VCO2, PaCO2 > 0"], method="VA = VCO2 K / PaCO2, K = 760 x 310.15/273.15",
     latex=r"\dot V_A = \frac{\dot V_{CO_2} \times 0.863}{P_aCO_2}", run=alveolar_ventilation_co2, reference=ref_va_co2,
     wrong=lambda VCO2, PaCO2: {"VA": VCO2 / PaCO2 * 0.760},
     rand=lambda rng: {"VCO2": _u(rng, 120, 400, 0), "PaCO2": _u(rng, 25, 70, 1)},
     phrases=["alveolar ventilation from CO2 output", "alveolar ventilation equation"],
     examples=[_ex({"VCO2": 200, "PaCO2": 40}, {"VA": 4.31473549332}), _ex({"VCO2": 250, "PaCO2": 50}, {"VA": 4.31473549332})],
     extra_q=[_bad({"VCO2": 200, "PaCO2": 0})], level="intermediate")


def alveolar_gas(FiO2, Patm, PH2O, PaCO2, RQ):
    FiO2 = _fr(FiO2, "FiO2", open_lo=True)
    Patm, PH2O, PaCO2, RQ = _pos(Patm, "Patm"), _nonneg(PH2O, "PH2O"), _nonneg(PaCO2, "PaCO2"), _pos(RQ, "RQ")
    _need(Patm > PH2O, "the barometric pressure must exceed the water vapour pressure")
    _need(RQ <= 1.5, "RQ must be at most 1.5")
    pio2 = FiO2 * (Patm - PH2O)
    simp = pio2 - PaCO2 / RQ
    return _rnd({"PiO2": pio2, "PAO2": simp, "PAO2_full": simp + PaCO2 * FiO2 * (1.0 - RQ) / RQ})


def ref_alveolar_gas(FiO2, Patm, PH2O, PaCO2, RQ):
    # nitrogen balance (N2 neither taken up nor given off), per unit alveolar (dry) ventilation
    import sympy as sp
    VI, FAO2 = sp.symbols("VI FAO2")
    f = lambda x: sp.Float(repr(float(x)), 30)
    dry = f(Patm) - f(PH2O)
    FACO2 = f(PaCO2) / dry
    FAN2 = 1 - FAO2 - FACO2
    eqs = [VI * (1 - f(FiO2)) - FAN2, (VI * f(FiO2) - FAO2) * f(RQ) - FACO2]
    s = sp.solve(eqs, [VI, FAO2], dict=True)[0]
    full = float(s[FAO2] * dry)
    pio2 = float(f(FiO2) * dry)
    return {"PiO2": pio2, "PAO2": _sx("A - I + C/R", "A", I=pio2, C=PaCO2, R=RQ), "PAO2_full": full}


_add("hp_alveolar_gas_equation", name="Alveolar gas equation", topic="respiratory / gas exchange",
     purpose="Inspired PO2 = FiO2 (PB - PH2O) and alveolar PO2 by the simplified (PiO2 - PaCO2/R) and full (nitrogen-balance) alveolar gas equation.",
     inputs=[_n("FiO2", "inspired O2 fraction", pct=True), _n("Patm", "barometric pressure in mmHg"), _n("PH2O", "water vapour pressure in mmHg"),
             _n("PaCO2", "arterial PCO2 in mmHg"), _n("RQ", "respiratory quotient")],
     outputs=[_o("PiO2", "inspired PO2 in mmHg"), _o("PAO2", "alveolar PO2, simplified equation, in mmHg"),
              _o("PAO2_full", "alveolar PO2, full equation, in mmHg")],
     primary="PAO2", conditions=["0 < FiO2 <= 1", "Patm > PH2O", "0 < RQ <= 1.5"], method="PAO2 = PiO2 - PaCO2/R (+ PaCO2 FiO2 (1 - R)/R)",
     latex=r"P_AO_2 = F_iO_2(P_B - P_{H_2O}) - \frac{P_aCO_2}{R}\left[1 - F_iO_2(1 - R)\right]",
     run=alveolar_gas, reference=ref_alveolar_gas,
     wrong=lambda FiO2, Patm, PH2O, PaCO2, RQ: {"PiO2": FiO2 * Patm, "PAO2": FiO2 * Patm - PaCO2 / RQ, "PAO2_full": FiO2 * Patm - PaCO2 * RQ},
     rand=lambda rng: {"FiO2": _u(rng, 0.21, 1.0, 2), "Patm": _u(rng, 500, 770, 0), "PH2O": 47, "PaCO2": _u(rng, 25, 60, 0), "RQ": _u(rng, 0.7, 1.0, 2)},
     phrases=["alveolar gas equation", "alveolar oxygen pressure"],
     examples=[_ex({"FiO2": 0.21, "Patm": 760, "PH2O": 47, "PaCO2": 40, "RQ": 0.8}, {"PiO2": 149.73, "PAO2": 99.73, "PAO2_full": 101.83}),
               _ex({"FiO2": 0.5, "Patm": 700, "PH2O": 47, "PaCO2": 45, "RQ": 0.8}, {"PiO2": 326.5, "PAO2": 270.25, "PAO2_full": 275.875})],
     extra_q=[_bad({"FiO2": 0.21, "Patm": 40, "PH2O": 47, "PaCO2": 40, "RQ": 0.8})], level="intermediate")


def aa_gradient(FiO2, Patm, PH2O, PaCO2, PaO2, RQ):
    FiO2 = _fr(FiO2, "FiO2", open_lo=True)
    Patm, PH2O, PaCO2, PaO2, RQ = _pos(Patm, "Patm"), _nonneg(PH2O, "PH2O"), _nonneg(PaCO2, "PaCO2"), _pos(PaO2, "PaO2"), _pos(RQ, "RQ")
    _need(Patm > PH2O, "the barometric pressure must exceed the water vapour pressure")
    pa = FiO2 * (Patm - PH2O) - PaCO2 / RQ
    _need(pa > 0, "the computed alveolar PO2 is not positive")
    # arterial PO2 cannot exceed the alveolar PO2 it equilibrates with: a negative gradient means the inputs disagree
    # (review 2026-11-14: a sweep of FiO2 gave A-a gradients of -80 mmHg)
    _need(PaO2 <= pa, "the arterial PO2 exceeds the computed alveolar PO2: check FiO2, PaCO2 or the respiratory quotient")
    return _rnd({"PAO2": pa, "Aa_gradient": pa - PaO2, "a_A_ratio": PaO2 / pa, "PF_ratio": PaO2 / FiO2})


def ref_aa(FiO2, Patm, PH2O, PaCO2, PaO2, RQ):
    pa = _sx("A - F*(B - W) + C/R", "A", F=FiO2, B=Patm, W=PH2O, C=PaCO2, R=RQ)
    return {"PAO2": pa, "Aa_gradient": _sx("g - A + a", "g", A=pa, a=PaO2), "a_A_ratio": _sx("x*A - a", "x", A=pa, a=PaO2),
            "PF_ratio": _sx("x*F - a", "x", F=FiO2, a=PaO2)}


_add("hp_aa_gradient", name="Alveolar-arterial oxygen gradient, a/A ratio and P/F ratio", topic="respiratory / gas exchange",
     purpose="PAO2 from the simplified alveolar gas equation, the A-a gradient PAO2 - PaO2, the a/A ratio and PaO2/FiO2.",
     inputs=[_n("FiO2", "inspired O2 fraction", pct=True), _n("Patm", "barometric pressure in mmHg"), _n("PH2O", "water vapour pressure in mmHg"),
             _n("PaCO2", "arterial PCO2 in mmHg"), _n("PaO2", "arterial PO2 in mmHg"), _n("RQ", "respiratory quotient")],
     outputs=[_o("PAO2", "alveolar PO2 in mmHg"), _o("Aa_gradient", "A-a gradient in mmHg"), _o("a_A_ratio", "PaO2/PAO2"),
              _o("PF_ratio", "PaO2/FiO2 in mmHg")],
     primary="Aa_gradient", conditions=["0 < FiO2 <= 1", "computed PAO2 > 0", "PaO2 <= computed PAO2"], method="A-a = FiO2 (PB - PH2O) - PaCO2/R - PaO2",
     latex=r"A\!-\!a = P_AO_2 - P_aO_2", run=aa_gradient, reference=ref_aa,
     wrong=lambda FiO2, Patm, PH2O, PaCO2, PaO2, RQ: (lambda pa: {"PAO2": pa, "Aa_gradient": pa - PaO2, "a_A_ratio": PaO2 / pa, "PF_ratio": PaO2 * FiO2})(FiO2 * Patm - PaCO2 / RQ),
     rand=lambda rng: (lambda f: {"FiO2": f, "Patm": 760, "PH2O": 47, "PaCO2": _u(rng, 30, 50, 0),
                                  "PaO2": round(rng.uniform(40, 0.9 * (f * 713 - 65)), 0), "RQ": 0.8})(_u(rng, 0.21, 1.0, 2)),
     phrases=["A-a gradient", "alveolar-arterial gradient"],
     examples=[_ex({"FiO2": 0.21, "Patm": 760, "PH2O": 47, "PaCO2": 40, "PaO2": 90, "RQ": 0.8}, {"PAO2": 99.73, "Aa_gradient": 9.73, "a_A_ratio": 0.902436578763, "PF_ratio": 428.571428571}),
               _ex({"FiO2": 0.4, "Patm": 760, "PH2O": 47, "PaCO2": 35, "PaO2": 80, "RQ": 0.8}, {"PAO2": 241.45, "Aa_gradient": 161.45, "a_A_ratio": 0.331331538621})],
     extra_q=[_bad({"FiO2": 0, "Patm": 760, "PH2O": 47, "PaCO2": 40, "PaO2": 90, "RQ": 0.8})], level="intermediate",
     weak="the simplified equation and ratios re-solved with sympy")


def oxygen_content(Hb, SaO2, PaO2):
    Hb, SaO2, PaO2 = _pos(Hb, "Hb"), _fr(SaO2, "SaO2"), _nonneg(PaO2, "PaO2")
    b, d = O2_HB * Hb * SaO2, O2_SOL * PaO2
    return _rnd({"CaO2": b + d, "bound": b, "dissolved": d})


_add("hp_oxygen_content", name="Arterial oxygen content", topic="respiratory / oxygen transport",
     purpose="CaO2 = 1.34 x Hb x SaO2 + 0.003 x PaO2 (hemoglobin-bound plus dissolved oxygen).",
     inputs=[_n("Hb", "hemoglobin in g/dL"), _n("SaO2", "arterial saturation", pct=True), _n("PaO2", "arterial PO2 in mmHg")],
     outputs=[_o("CaO2", "oxygen content in mL O2/dL"), _o("bound", "hemoglobin-bound O2 in mL/dL"), _o("dissolved", "dissolved O2 in mL/dL")],
     primary="CaO2", conditions=["Hb > 0", "0 <= SaO2 <= 1"], method="1.34 Hb S + 0.003 PO2", latex=r"C_aO_2 = 1.34\,Hb\,S_aO_2 + 0.003\,P_aO_2",
     run=oxygen_content,
     reference=lambda Hb, SaO2, PaO2: {"CaO2": _sx("C - 134*H*S/100 - 3*P/1000", "C", H=Hb, S=SaO2, P=PaO2),
                                       "bound": _sx("b - 134*H*S/100", "b", H=Hb, S=SaO2), "dissolved": _sx("d - 3*P/1000", "d", P=PaO2)},
     wrong=lambda Hb, SaO2, PaO2: {"CaO2": 1.34 * Hb * SaO2 + 0.03 * PaO2, "bound": 1.34 * Hb * SaO2, "dissolved": 0.03 * PaO2},
     rand=lambda rng: {"Hb": _u(rng, 7, 18, 1), "SaO2": _u(rng, 0.7, 1.0, 2), "PaO2": _u(rng, 40, 500, 0)},
     phrases=["oxygen content", "arterial oxygen content"],
     examples=[_ex({"Hb": 15, "SaO2": 0.98, "PaO2": 100}, {"CaO2": 19.998, "bound": 19.698, "dissolved": 0.3}),
               _ex({"Hb": 12, "SaO2": 0.9, "PaO2": 60}, {"CaO2": 14.652})],
     extra_q=[{"q": "oxygen content: Hb = 15, SaO2 = 120%, PaO2 = 100 mmHg", "outcome": "INVALID_INPUT"}], level="beginner",
     weak="the published content relation re-solved with sympy")


def oxygen_delivery(CO, Hb, SaO2, PaO2):
    CO, Hb, SaO2, PaO2 = _pos(CO, "CO"), _pos(Hb, "Hb"), _fr(SaO2, "SaO2"), _nonneg(PaO2, "PaO2")
    c = _o2c(Hb, SaO2, PaO2)
    return _rnd({"DO2": CO * c * 10.0, "CaO2": c})


_add("hp_oxygen_delivery", name="Systemic oxygen delivery", topic="respiratory / oxygen transport",
     purpose="DO2 = CO x CaO2 x 10 (mL O2/min), with CaO2 = 1.34 Hb SaO2 + 0.003 PaO2.",
     inputs=[_n("CO", "cardiac output in L/min"), _n("Hb", "hemoglobin in g/dL"), _n("SaO2", "arterial saturation", pct=True), _n("PaO2", "arterial PO2 in mmHg")],
     outputs=[_o("DO2", "oxygen delivery in mL O2/min"), _o("CaO2", "arterial oxygen content in mL/dL")],
     primary="DO2", conditions=["CO, Hb > 0", "0 <= SaO2 <= 1"], method="DO2 = 10 CO CaO2", latex=r"D_{O_2} = CO \times C_aO_2 \times 10",
     run=oxygen_delivery,
     reference=lambda CO, Hb, SaO2, PaO2: (lambda c: {"DO2": _sx("D - 10*Q*c", "D", Q=CO, c=c), "CaO2": c})(_sx("C - 134*H*S/100 - 3*P/1000", "C", H=Hb, S=SaO2, P=PaO2)),
     wrong=lambda CO, Hb, SaO2, PaO2: {"DO2": CO * (1.34 * Hb * SaO2 + 0.003 * PaO2), "CaO2": 1.34 * Hb * SaO2 + 0.003 * PaO2},
     rand=lambda rng: {"CO": _u(rng, 2.5, 9, 1), "Hb": _u(rng, 7, 18, 1), "SaO2": _u(rng, 0.8, 1.0, 2), "PaO2": _u(rng, 50, 300, 0)},
     phrases=["oxygen delivery"],
     examples=[_ex({"CO": 5, "Hb": 15, "SaO2": 0.98, "PaO2": 100}, {"DO2": 999.9, "CaO2": 19.998}), _ex({"CO": 4, "Hb": 10, "SaO2": 1, "PaO2": 0}, {"DO2": 536})],
     extra_q=[_bad({"CO": 0, "Hb": 15, "SaO2": 0.98, "PaO2": 100})], level="intermediate", weak="the published relations re-solved with sympy")


def oxygen_consumption(CO, Hb, SaO2, SvO2, PaO2, PvO2):
    CO, Hb, PaO2, PvO2 = _pos(CO, "CO"), _pos(Hb, "Hb"), _nonneg(PaO2, "PaO2"), _nonneg(PvO2, "PvO2")
    SaO2, SvO2 = _fr(SaO2, "SaO2"), _fr(SvO2, "SvO2")
    ca, cv = _o2c(Hb, SaO2, PaO2), _o2c(Hb, SvO2, PvO2)
    _need(cv < ca, "the venous oxygen content must be below the arterial")
    return _rnd({"VO2": CO * (ca - cv) * 10.0, "O2ER": (ca - cv) / ca, "CaO2": ca, "CvO2": cv})


def ref_oxygen_consumption(CO, Hb, SaO2, SvO2, PaO2, PvO2):
    ca = _sx("C - 134*H*S/100 - 3*P/1000", "C", H=Hb, S=SaO2, P=PaO2)
    cv = _sx("C - 134*H*S/100 - 3*P/1000", "C", H=Hb, S=SvO2, P=PvO2)
    return {"VO2": _sx("V - 10*Q*(a - b)", "V", Q=CO, a=ca, b=cv), "O2ER": _sx("E*a - (a - b)", "E", a=ca, b=cv), "CaO2": ca, "CvO2": cv}


_add("hp_oxygen_consumption", name="Oxygen consumption and extraction ratio (reverse Fick)", topic="respiratory / oxygen transport",
     purpose="VO2 = CO (CaO2 - CvO2) x 10 and the oxygen extraction ratio (CaO2 - CvO2)/CaO2.",
     inputs=[_n("CO", "cardiac output in L/min"), _n("Hb", "hemoglobin in g/dL"), _n("SaO2", "arterial saturation", pct=True),
             _n("SvO2", "mixed venous saturation", pct=True), _n("PaO2", "arterial PO2 in mmHg"), _n("PvO2", "mixed venous PO2 in mmHg")],
     outputs=[_o("VO2", "oxygen consumption in mL/min"), _o("O2ER", "oxygen extraction ratio"), _o("CaO2", "arterial content in mL/dL"),
              _o("CvO2", "venous content in mL/dL")],
     primary="VO2", conditions=["CvO2 < CaO2"], method="VO2 = 10 CO (CaO2 - CvO2)", latex=r"\dot V_{O_2} = CO\,(C_aO_2 - C_{\bar v}O_2)",
     run=oxygen_consumption, reference=ref_oxygen_consumption,
     wrong=lambda CO, Hb, SaO2, SvO2, PaO2, PvO2: (lambda ca, cv: {"VO2": CO * (ca - cv), "O2ER": (ca - cv) / cv, "CaO2": ca, "CvO2": cv})(_o2c(Hb, SaO2, PaO2), _o2c(Hb, SvO2, PvO2)),
     rand=lambda rng: {"CO": _u(rng, 2.5, 9, 1), "Hb": _u(rng, 8, 17, 1), "SaO2": _u(rng, 0.9, 1.0, 2), "SvO2": _u(rng, 0.5, 0.8, 2),
                       "PaO2": _u(rng, 70, 120, 0), "PvO2": _u(rng, 28, 45, 0)},
     phrases=["oxygen consumption", "oxygen extraction ratio"],
     examples=[_ex({"CO": 5, "Hb": 15, "SaO2": 0.98, "SvO2": 0.75, "PaO2": 100, "PvO2": 40}, {"VO2": 240.15, "O2ER": 0.240174017402}),
               _ex({"CO": 6, "Hb": 13, "SaO2": 0.97, "SvO2": 0.65, "PaO2": 90, "PvO2": 35}, {"VO2": 344.364, "O2ER": 0.334319698964})],
     extra_q=[_bad({"CO": 5, "Hb": 15, "SaO2": 0.6, "SvO2": 0.75, "PaO2": 30, "PvO2": 40})], level="intermediate",
     weak="the published relations re-solved with sympy")


def hill_saturation(PO2, P50, n):
    PO2, P50, n = _nonneg(PO2, "PO2"), _pos(P50, "P50"), _pos(n, "n")
    x = (PO2 / P50) ** n
    return _rnd({"SO2": x / (1.0 + x)})


def ref_hill_sat(PO2, P50, n):
    from scipy.special import expit
    if PO2 == 0:
        return {"SO2": 0.0}
    return {"SO2": float(expit(n * (math.log(PO2) - math.log(P50))))}


_add("hp_hill_saturation", name="Oxyhemoglobin saturation by the Hill equation", topic="respiratory / oxygen binding",
     purpose="SO2 = PO2^n/(PO2^n + P50^n) for a Hill coefficient n and half-saturation pressure P50.",
     inputs=[_n("PO2", "partial pressure of O2 in mmHg"), _n("P50", "P50 in mmHg"), _n("n", "Hill coefficient")],
     outputs=[_o("SO2", "fractional saturation")],
     primary="SO2", conditions=["PO2 >= 0", "P50, n > 0"], method="Hill equation", latex=r"S = \frac{P^n}{P^n + P_{50}^n}",
     run=hill_saturation, reference=ref_hill_sat,
     wrong=lambda PO2, P50, n: {"SO2": PO2 / (PO2 + P50)},
     rand=lambda rng: {"PO2": _u(rng, 10, 100, 1), "P50": _u(rng, 20, 35, 1), "n": _u(rng, 2.0, 3.0, 2)},
     phrases=["Hill saturation", "oxyhemoglobin saturation by the Hill equation"],
     examples=[_ex({"PO2": 40, "P50": 26.8, "n": 2.7}, {"SO2": 0.746737918393}), _ex({"PO2": 26.8, "P50": 26.8, "n": 2.7}, {"SO2": 0.5}),
               _ex({"PO2": 60, "P50": 27, "n": 2.6}, {"SO2": 0.8885604124})],
     extra_q=[_bad({"PO2": 40, "P50": 0, "n": 2.7})], level="intermediate", weak="the same logistic form evaluated with scipy expit")


def hill_po2(SO2, P50, n):
    SO2, P50, n = _fr(SO2, "SO2", True, True), _pos(P50, "P50"), _pos(n, "n")
    return _rnd({"PO2": P50 * (SO2 / (1.0 - SO2)) ** (1.0 / n)})


def ref_hill_po2(SO2, P50, n):
    return {"PO2": _brentq(lambda p: p ** n / (p ** n + P50 ** n) - SO2, 1e-9 * P50, 1e6 * P50)}


_add("hp_hill_po2", name="PO2 at a given saturation by the Hill equation", topic="respiratory / oxygen binding",
     purpose="Inverse Hill equation PO2 = P50 (S/(1 - S))^(1/n).",
     inputs=[_n("SO2", "fractional saturation", pct=True), _n("P50", "P50 in mmHg"), _n("n", "Hill coefficient")],
     outputs=[_o("PO2", "partial pressure of O2 in mmHg")],
     primary="PO2", conditions=["0 < SO2 < 1", "P50, n > 0"], method="PO2 = P50 (S/(1-S))^(1/n)", latex=r"P = P_{50}\left(\frac{S}{1-S}\right)^{1/n}",
     run=hill_po2, reference=ref_hill_po2,
     wrong=lambda SO2, P50, n: {"PO2": P50 * (SO2 / (1 - SO2)) ** n},
     rand=lambda rng: {"SO2": _u(rng, 0.1, 0.97, 2), "P50": _u(rng, 20, 35, 1), "n": _u(rng, 2.0, 3.0, 2)},
     phrases=["Hill PO2", "PO2 at a given saturation"],
     examples=[_ex({"SO2": 0.75, "P50": 26.8, "n": 2.7}, {"PO2": 40.2574599782}), _ex({"SO2": 0.9, "P50": 26.8, "n": 2.7}, {"PO2": 60.4725031305})],
     extra_q=[_bad({"SO2": 1, "P50": 26.8, "n": 2.7})], level="intermediate")


def hill_fit(P1, S1, P2, S2):
    P1, P2 = _pos(P1, "P1"), _pos(P2, "P2")
    S1, S2 = _fr(S1, "S1", True, True), _fr(S2, "S2", True, True)
    _need(P1 != P2 and S1 != S2, "the two points must differ")
    _need((P2 - P1) * (S2 - S1) > 0, "saturation must increase with PO2")
    n = math.log((S2 / (1 - S2)) / (S1 / (1 - S1))) / math.log(P2 / P1)
    return _rnd({"n": n, "P50": P1 * ((1 - S1) / S1) ** (1.0 / n)})


def ref_hill_fit(P1, S1, P2, S2):
    from scipy.optimize import least_squares
    r = least_squares(lambda x: [P1 ** x[0] / (P1 ** x[0] + x[1] ** x[0]) - S1, P2 ** x[0] / (P2 ** x[0] + x[1] ** x[0]) - S2],
                      [2.5, math.sqrt(P1 * P2)], xtol=1e-15, ftol=1e-15, gtol=1e-15, x_scale=[1.0, math.sqrt(P1 * P2)])
    return {"n": float(r.x[0]), "P50": float(r.x[1])}


_add("hp_hill_fit", name="Hill coefficient and P50 from two points of the dissociation curve", topic="respiratory / oxygen binding",
     purpose="The Hill coefficient n and P50 of the curve through two measured (PO2, saturation) points (Hill plot slope).",
     inputs=[_n("P1", "first PO2 in mmHg"), _n("S1", "saturation at P1", pct=True), _n("P2", "second PO2 in mmHg"), _n("S2", "saturation at P2", pct=True)],
     outputs=[_o("n", "Hill coefficient"), _o("P50", "P50 in mmHg")],
     primary="P50", conditions=["0 < S < 1", "saturation increases with PO2"], method="slope of log(S/(1-S)) against log PO2",
     latex=r"n = \frac{\ln\frac{S_2/(1-S_2)}{S_1/(1-S_1)}}{\ln(P_2/P_1)}", run=hill_fit, reference=ref_hill_fit,
     wrong=lambda P1, S1, P2, S2: (lambda n: {"n": n, "P50": P1 * ((1 - S1) / S1) ** n})(math.log((S2 / (1 - S2)) / (S1 / (1 - S1))) / math.log(P2 / P1)),
     rand=lambda rng: (lambda n, p50, a, b: {"P1": a, "S1": round(a ** n / (a ** n + p50 ** n), 4), "P2": b, "S2": round(b ** n / (b ** n + p50 ** n), 4)})(
         rng.uniform(2.2, 3.0), rng.uniform(22, 32), _u(rng, 12, 25, 0), _u(rng, 35, 70, 0)),
     phrases=["Hill fit", "Hill coefficient and P50"],
     examples=[_ex({"P1": 20, "S1": 0.312124018611, "P2": 60, "S2": 0.898077644148}, {"n": 2.7, "P50": 26.8})],
     extra_q=[_bad({"P1": 20, "S1": 0.9, "P2": 60, "S2": 0.3})], level="expert")


def _sev(P):
    return 1.0 / (23400.0 / (P ** 3 + 150.0 * P) + 1.0)


def severinghaus_saturation(PO2):
    PO2 = _pos(PO2, "PO2")
    return _rnd({"SO2": _sev(PO2)})


_add("hp_severinghaus_saturation", name="Saturation by the Severinghaus (1979) dissociation equation", topic="respiratory / oxygen binding",
     purpose="Standard human oxyhemoglobin curve S = 1/(23400/(PO2^3 + 150 PO2) + 1) (37 degC, pH 7.40, PCO2 40 mmHg).",
     inputs=[_n("PO2", "partial pressure of O2 in mmHg")],
     outputs=[_o("SO2", "fractional saturation")],
     primary="SO2", conditions=["PO2 > 0"], method="Severinghaus equation", latex=r"S = \left(\frac{23400}{P^3 + 150P} + 1\right)^{-1}",
     run=severinghaus_saturation, reference=lambda PO2: {"SO2": _sx("S*(23400 + P**3 + 150*P) - (P**3 + 150*P)", "S", P=PO2)},
     wrong=lambda PO2: {"SO2": 1 / (23400 / (PO2 ** 3 + 150 * PO2 * PO2) + 1)},
     rand=lambda rng: {"PO2": _u(rng, 10, 150, 1)},
     phrases=["Severinghaus saturation"],
     examples=[_ex({"PO2": 40}, {"SO2": 0.749464668094}), _ex({"PO2": 100}, {"SO2": 0.977465331279})],
     extra_q=[_bad({"PO2": 0})], level="advanced", weak="the published equation re-solved with sympy")


def severinghaus_po2(SO2):
    SO2 = _fr(SO2, "SO2", True, True)
    K = 23400.0 * SO2 / (1.0 - SO2)
    # P^3 + 150 P - K = 0: one real root (Cardano), polished by Newton steps
    q = -K
    d = math.sqrt(q * q / 4.0 + 150.0 ** 3 / 27.0)
    a, b = -q / 2.0 + d, -q / 2.0 - d
    P = math.copysign(abs(a) ** (1.0 / 3.0), a) + math.copysign(abs(b) ** (1.0 / 3.0), b)
    for _ in range(3):
        P -= (P ** 3 + 150.0 * P - K) / (3 * P * P + 150.0)
    return _rnd({"PO2": P})


def ref_sev_po2(SO2):
    import sympy as sp
    P = sp.Symbol("P", real=True)
    s = sp.Float(repr(float(SO2)), 40)
    roots = [r for r in sp.Poly(P ** 3 + 150 * P - 23400 * s / (1 - s), P).nroots(n=30) if abs(sp.im(r)) < 1e-20]
    return {"PO2": float(sp.re(roots[0]))}


_add("hp_severinghaus_po2", name="PO2 at a given saturation by the Severinghaus equation", topic="respiratory / oxygen binding",
     purpose="Inverse of the Severinghaus curve: the real root of P^3 + 150 P = 23400 S/(1 - S).",
     inputs=[_n("SO2", "fractional saturation", pct=True)],
     outputs=[_o("PO2", "partial pressure of O2 in mmHg")],
     primary="PO2", conditions=["0 < SO2 < 1"], method="Cardano root of the cubic, Newton polished", latex=r"P^3 + 150P = \frac{23400\,S}{1-S}",
     run=severinghaus_po2, reference=ref_sev_po2,
     wrong=lambda SO2: {"PO2": (23400 * SO2 / (1 - SO2)) ** (1 / 3)},
     rand=lambda rng: {"SO2": _u(rng, 0.2, 0.99, 3)},
     phrases=["Severinghaus PO2"],
     examples=[_ex({"SO2": 0.749464668094}, {"PO2": 40}), _ex({"SO2": 0.977465331279}, {"PO2": 100})],
     extra_q=[_bad({"SO2": 1})], level="advanced")


def virtual_po2(PO2, T, pH, PCO2):
    PO2, T, pH, PCO2 = _pos(PO2, "PO2"), _num(T, "T"), _num(pH, "pH"), _pos(PCO2, "PCO2")
    _need(25 <= T <= 43 and 6.8 <= pH <= 7.8, "temperature 25-43 degC and pH 6.8-7.8")
    f = 10.0 ** (0.024 * (37.0 - T) + 0.40 * (pH - 7.40) + 0.06 * math.log10(40.0 / PCO2))
    pv = PO2 * f
    return _rnd({"PO2_virtual": pv, "SO2": _sev(pv), "factor": f})


def ref_virtual_po2(PO2, T, pH, PCO2):
    import mpmath as mp
    mp.mp.dps = 30
    f = mp.power(10, mp.mpf("0.024") * (37 - mp.mpf(repr(T))) + mp.mpf("0.40") * (mp.mpf(repr(pH)) - mp.mpf("7.40")) + mp.mpf("0.06") * mp.log10(40 / mp.mpf(repr(PCO2))))
    pv = mp.mpf(repr(PO2)) * f
    return {"PO2_virtual": float(pv), "SO2": float((pv ** 3 + 150 * pv) / (pv ** 3 + 150 * pv + 23400)), "factor": float(f)}


_add("hp_virtual_po2", name="Temperature, pH and PCO2 correction of the dissociation curve (Severinghaus)", topic="respiratory / oxygen binding",
     purpose="Virtual PO2 = PO2 x 10^(0.024 (37 - T) + 0.40 (pH - 7.40) + 0.06 log10(40/PCO2)) and the saturation from the standard curve (Bohr effect).",
     inputs=[_n("PO2", "measured PO2 in mmHg"), _n("T", "temperature in degC"), _n("pH", "blood pH"), _n("PCO2", "PCO2 in mmHg")],
     outputs=[_o("PO2_virtual", "standard-condition (virtual) PO2 in mmHg"), _o("SO2", "fractional saturation"), _o("factor", "correction factor")],
     primary="SO2", conditions=["25 <= T <= 43 degC", "6.8 <= pH <= 7.8"], method="Severinghaus (1979) correction, then the standard curve",
     latex=r"P_{virt} = P\cdot 10^{0.024(37-T) + 0.40(pH - 7.40) + 0.06\log_{10}(40/P_{CO_2})}", run=virtual_po2, reference=ref_virtual_po2,
     wrong=lambda PO2, T, pH, PCO2: (lambda f: {"PO2_virtual": PO2 / f, "SO2": _sev(PO2 / f), "factor": 1 / f})(10 ** (0.024 * (37 - T) + 0.40 * (pH - 7.40) + 0.06 * math.log10(40 / PCO2))),
     rand=lambda rng: {"PO2": _u(rng, 20, 120, 0), "T": _u(rng, 30, 41, 1), "pH": _u(rng, 7.0, 7.6, 2), "PCO2": _u(rng, 20, 70, 0)},
     phrases=["virtual PO2", "corrected oxygen saturation"],
     examples=[_ex({"PO2": 80, "T": 39, "pH": 7.3, "PCO2": 50}, {"PO2_virtual": 64.4577862898}), _ex({"PO2": 60, "T": 37, "pH": 7.45, "PCO2": 30}, {"PO2_virtual": 63.921590795})],
     extra_q=[_bad({"PO2": 80, "T": 50, "pH": 7.3, "PCO2": 50})], level="expert", weak="the published correction evaluated in 30-digit mpmath")


def ph_henderson(HCO3, PaCO2):
    HCO3, PaCO2 = _pos(HCO3, "HCO3"), _pos(PaCO2, "PaCO2")
    ph = PK_CO2 + math.log10(HCO3 / (CO2_SOL * PaCO2))
    return _rnd({"pH": ph, "H": 10.0 ** (9.0 - ph)})


def ref_ph(HCO3, PaCO2):
    # mass action: [H+][HCO3-] = K' [CO2]; [H+] in mol/L, both other concentrations in mmol/L
    Ka = 10.0 ** -6.1
    h = _brentq(lambda x: x * 1e9 * HCO3 - Ka * 1e9 * 0.03 * PaCO2, 1e-12, 1e-3)
    return {"pH": -math.log10(h), "H": h * 1e9}


_add("hp_ph_henderson_hasselbalch", name="Blood pH by the Henderson-Hasselbalch equation", topic="respiratory / acid-base",
     purpose="pH = 6.1 + log10(HCO3/(0.03 PaCO2)) and the hydrogen-ion concentration in nmol/L.",
     inputs=[_n("HCO3", "bicarbonate in mmol/L"), _n("PaCO2", "PCO2 in mmHg")],
     outputs=[_o("pH", "pH"), _o("H", "hydrogen-ion concentration in nmol/L")],
     primary="pH", conditions=["HCO3, PaCO2 > 0"], method="pK' 6.1, CO2 solubility 0.03 mmol/L per mmHg",
     latex=r"pH = 6.1 + \log_{10}\frac{[HCO_3^-]}{0.03\,P_{CO_2}}", run=ph_henderson, reference=ref_ph,
     wrong=lambda HCO3, PaCO2: (lambda p: {"pH": p, "H": 10 ** (9 - p)})(6.1 + math.log10(0.03 * PaCO2 / HCO3)),
     rand=lambda rng: {"HCO3": _u(rng, 8, 45, 1), "PaCO2": _u(rng, 20, 80, 0)},
     phrases=["blood pH", "Henderson-Hasselbalch pH"],
     examples=[_ex({"HCO3": 24, "PaCO2": 40}, {"pH": 7.40102999566, "H": 39.7164117362}), _ex({"HCO3": 12, "PaCO2": 25}, {"pH": 7.30411998266, "H": 49.6455146703})],
     extra_q=[_bad({"HCO3": 0, "PaCO2": 40})], level="intermediate")


def bicarbonate_from_ph(pH, PaCO2):
    pH, PaCO2 = _num(pH, "pH"), _pos(PaCO2, "PaCO2")
    _need(6.5 <= pH <= 8.0, "pH must be between 6.5 and 8.0")
    return _rnd({"HCO3": CO2_SOL * PaCO2 * 10.0 ** (pH - PK_CO2)})


_add("hp_bicarbonate_from_ph", name="Bicarbonate from pH and PCO2 (Henderson-Hasselbalch)", topic="respiratory / acid-base",
     purpose="HCO3 = 0.03 PaCO2 x 10^(pH - 6.1).",
     inputs=[_n("pH", "pH"), _n("PaCO2", "PCO2 in mmHg")], outputs=[_o("HCO3", "bicarbonate in mmol/L")],
     primary="HCO3", conditions=["6.5 <= pH <= 8.0", "PaCO2 > 0"], method="Henderson-Hasselbalch solved for bicarbonate",
     latex=r"[HCO_3^-] = 0.03\,P_{CO_2}\,10^{pH - 6.1}", run=bicarbonate_from_ph,
     reference=lambda pH, PaCO2: {"HCO3": _brentq(lambda b: 6.1 + math.log10(b / (0.03 * PaCO2)) - pH, 1e-6, 1e4)},
     wrong=lambda pH, PaCO2: {"HCO3": PaCO2 * 10 ** (pH - 6.1)},
     rand=lambda rng: {"pH": _u(rng, 7.0, 7.7, 2), "PaCO2": _u(rng, 20, 80, 0)},
     phrases=["bicarbonate from pH"],
     examples=[_ex({"pH": 7.4, "PaCO2": 40}, {"HCO3": 23.9431477796}), _ex({"pH": 7.25, "PaCO2": 60}, {"HCO3": 25.4256758032})],
     extra_q=[_bad({"pH": 9, "PaCO2": 40})], level="intermediate", weak="the same equation re-solved by brentq")


def pco2_from_ph(pH, HCO3):
    pH, HCO3 = _num(pH, "pH"), _pos(HCO3, "HCO3")
    _need(6.5 <= pH <= 8.0, "pH must be between 6.5 and 8.0")
    return _rnd({"PaCO2": HCO3 / (CO2_SOL * 10.0 ** (pH - PK_CO2))})


_add("hp_pco2_from_ph", name="PCO2 from pH and bicarbonate (Henderson-Hasselbalch)", topic="respiratory / acid-base",
     purpose="PaCO2 = HCO3/(0.03 x 10^(pH - 6.1)).",
     inputs=[_n("pH", "pH"), _n("HCO3", "bicarbonate in mmol/L")], outputs=[_o("PaCO2", "PCO2 in mmHg")],
     primary="PaCO2", conditions=["6.5 <= pH <= 8.0", "HCO3 > 0"], method="Henderson-Hasselbalch solved for PCO2",
     latex=r"P_{CO_2} = \frac{[HCO_3^-]}{0.03\cdot 10^{pH - 6.1}}", run=pco2_from_ph,
     reference=lambda pH, HCO3: {"PaCO2": _brentq(lambda p: 6.1 + math.log10(HCO3 / (0.03 * p)) - pH, 1e-6, 1e5)},
     wrong=lambda pH, HCO3: {"PaCO2": HCO3 * 0.03 * 10 ** (pH - 6.1)},
     rand=lambda rng: {"pH": _u(rng, 7.0, 7.7, 2), "HCO3": _u(rng, 8, 45, 1)},
     phrases=["PCO2 from pH"],
     examples=[_ex({"pH": 7.4, "HCO3": 24}, {"PaCO2": 40.0949786902}), _ex({"pH": 7.2, "HCO3": 15}, {"PaCO2": 39.7164117362})],
     extra_q=[_bad({"pH": 6, "HCO3": 24})], level="intermediate", weak="the same equation re-solved by brentq")


def anion_gap(Na, Cl, HCO3, albumin=4.0):
    Na, Cl, HCO3, albumin = _pos(Na, "Na"), _pos(Cl, "Cl"), _pos(HCO3, "HCO3"), _pos(albumin, "albumin")
    ag = Na - (Cl + HCO3)
    return _rnd({"AG": ag, "AG_corrected": ag + 2.5 * (4.0 - albumin)})


_add("hp_anion_gap", name="Serum anion gap and albumin-corrected anion gap", topic="respiratory / acid-base",
     purpose="AG = Na - (Cl + HCO3) and the albumin-corrected value AG + 2.5 (4.0 - albumin in g/dL).",
     inputs=[_n("Na", "sodium in mmol/L"), _n("Cl", "chloride in mmol/L"), _n("HCO3", "bicarbonate in mmol/L"),
             _n("albumin", "albumin in g/dL (optional; normal albumin if omitted)", required=False)],
     outputs=[_o("AG", "anion gap in mmol/L"), _o("AG_corrected", "albumin-corrected anion gap in mmol/L")],
     primary="AG", conditions=["all inputs > 0"], method="AG = Na - Cl - HCO3; + 2.5 per g/dL of albumin below 4.0",
     latex=r"AG = [Na^+] - ([Cl^-] + [HCO_3^-])", run=anion_gap,
     reference=lambda Na, Cl, HCO3, albumin=4.0: (lambda g: {"AG": g, "AG_corrected": _sx("c - g - 5*(4 - a)/2", "c", g=g, a=albumin)})(_sx("g - N + C + B", "g", N=Na, C=Cl, B=HCO3)),
     wrong=lambda Na, Cl, HCO3, albumin=4.0: {"AG": Na - Cl, "AG_corrected": Na - Cl + 2.5 * (4 - albumin)},
     rand=lambda rng: {"Na": _u(rng, 125, 155, 0), "Cl": _u(rng, 85, 115, 0), "HCO3": _u(rng, 8, 32, 0), "albumin": _u(rng, 1.5, 5, 1)},
     phrases=["anion gap"],
     examples=[_ex({"Na": 140, "Cl": 104, "HCO3": 24}, {"AG": 12, "AG_corrected": 12}), _ex({"Na": 140, "Cl": 104, "HCO3": 24, "albumin": 2}, {"AG": 12, "AG_corrected": 17})],
     extra_q=[_bad({"Na": 0, "Cl": 104, "HCO3": 24})], level="intermediate", weak="the definition re-solved with sympy")


def fick_diffusion(D, A, dC, T):
    D, A, dC, T = _pos(D, "D"), _pos(A, "A"), _num(dC, "dC"), _pos(T, "T")
    x = T * 1e-4                      # um -> cm
    return _rnd({"J": D * A * (dC * 1e-3) / x, "t_diffusion": x * x / (2.0 * D)})


def ref_fick_diffusion(D, A, dC, T):
    import sympy as sp
    z = sp.Symbol("z")
    Cf = sp.Function("C")
    x = sp.Float(repr(T), 30) * sp.Float("1e-4", 30)
    c0 = sp.Float(repr(dC), 30) * sp.Float("1e-3", 30)
    sol = sp.dsolve(sp.Eq(Cf(z).diff(z, 2), 0), Cf(z), ics={Cf(0): c0, Cf(x): 0}).rhs
    J = float(-sp.Float(repr(D), 30) * sp.Float(repr(A), 30) * sp.diff(sol, z))
    # mean-square displacement of 1-D diffusion from a point: <x^2> = 2 D t
    t = _brentq(lambda tt: _quad(lambda s: s * s * math.exp(-s * s / (4 * D * tt)) / math.sqrt(4 * math.pi * D * tt), -50 * math.sqrt(D * tt), 50 * math.sqrt(D * tt)) - (T * 1e-4) ** 2,
                (T * 1e-4) ** 2 / (2 * D) * 0.5, (T * 1e-4) ** 2 / (2 * D) * 2)
    return {"J": J, "t_diffusion": t}


_add("hp_fick_diffusion", name="Fick's law of diffusion across a membrane", topic="respiratory / gas exchange",
     purpose="Steady diffusive flux J = D A dC/T across a barrier of thickness T, and the diffusion time T^2/(2D).",
     inputs=[_n("D", "diffusion coefficient in cm^2/s"), _n("A", "membrane area in cm^2"), _n("dC", "concentration difference in mmol/L"),
             _n("T", "membrane thickness in um")],
     outputs=[_o("J", "diffusion rate in mmol/s"), _o("t_diffusion", "characteristic diffusion time across the membrane in s")],
     primary="J", conditions=["D, A, T > 0"], method="J = D A dC / T (units converted to cm)", latex=r"J = D A \frac{\Delta C}{T},\quad t = \frac{T^2}{2D}",
     run=fick_diffusion, reference=ref_fick_diffusion,
     wrong=lambda D, A, dC, T: {"J": D * A * dC * 1e-3 * T * 1e-4, "t_diffusion": (T * 1e-4) ** 2 / D},
     rand=lambda rng: {"D": float(f"{_u(rng, 0.5, 3, 2)}e-5"), "A": _u(rng, 0.1, 10, 2), "dC": _u(rng, 0.05, 5, 2), "T": _u(rng, 0.3, 20, 1)},
     phrases=["Fick diffusion", "Fick's law of diffusion"],
     examples=[_ex({"D": 1.5e-5, "A": 2, "dC": 0.5, "T": 2}, {"J": 7.5e-05, "t_diffusion": 0.00133333333333}),
               _ex({"D": 2e-5, "A": 1, "dC": 1, "T": 1}, {"J": 0.0002, "t_diffusion": 0.00025})],
     extra_q=[_bad({"D": 1.5e-5, "A": 2, "dC": 0.5, "T": 0})], level="intermediate")


def respiratory_mechanics(VT, Ppeak, Pplat, PEEP, flow):
    VT, Ppeak, Pplat, PEEP, flow = _pos(VT, "VT"), _num(Ppeak, "Ppeak"), _num(Pplat, "Pplat"), _nonneg(PEEP, "PEEP"), _pos(flow, "flow")
    _need(Ppeak >= Pplat > PEEP, "need Ppeak >= Pplat > PEEP")
    return _rnd({"C_static": VT / (Pplat - PEEP), "C_dynamic": VT / (Ppeak - PEEP), "R_aw": (Ppeak - Pplat) / (flow / 60.0),
                 "driving_pressure": Pplat - PEEP})


def ref_resp_mech(VT, Ppeak, Pplat, PEEP, flow):
    return {"C_static": _sx("C*(p - e) - V", "C", V=VT, p=Pplat, e=PEEP), "C_dynamic": _sx("C*(p - e) - V", "C", V=VT, p=Ppeak, e=PEEP),
            "R_aw": _sx("R*f/60 - (a - b)", "R", a=Ppeak, b=Pplat, f=flow), "driving_pressure": _sx("d - p + e", "d", p=Pplat, e=PEEP)}


_add("hp_respiratory_mechanics", name="Static and dynamic compliance and airway resistance", topic="respiratory / mechanics",
     purpose="Static compliance VT/(Pplat - PEEP), dynamic compliance VT/(Ppeak - PEEP), airway resistance (Ppeak - Pplat)/flow and driving pressure.",
     inputs=[_n("VT", "tidal volume in mL"), _n("Ppeak", "peak airway pressure in cmH2O"), _n("Pplat", "plateau pressure in cmH2O"),
             _n("PEEP", "end-expiratory pressure in cmH2O"), _n("flow", "constant inspiratory flow in L/min")],
     outputs=[_o("C_static", "static compliance in mL/cmH2O"), _o("C_dynamic", "dynamic compliance in mL/cmH2O"),
              _o("R_aw", "airway resistance in cmH2O s/L"), _o("driving_pressure", "driving pressure in cmH2O")],
     primary="C_static", conditions=["Ppeak >= Pplat > PEEP >= 0", "constant flow"], method="pressure differences over volume and flow",
     latex=r"C_{st} = \frac{V_T}{P_{plat} - PEEP},\quad R_{aw} = \frac{P_{peak} - P_{plat}}{\dot V}", run=respiratory_mechanics, reference=ref_resp_mech,
     wrong=lambda VT, Ppeak, Pplat, PEEP, flow: {"C_static": VT / Pplat, "C_dynamic": VT / Ppeak, "R_aw": (Ppeak - Pplat) / flow, "driving_pressure": Pplat},
     rand=lambda rng: (lambda pe, pl: {"VT": _u(rng, 300, 700, 0), "PEEP": pe, "Pplat": pl, "Ppeak": round(pl + rng.uniform(1, 15), 0), "flow": _u(rng, 30, 80, 0)})(_u(rng, 0, 12, 0), _u(rng, 14, 30, 0)),
     phrases=["respiratory mechanics", "static compliance and airway resistance"],
     examples=[_ex({"VT": 500, "Ppeak": 25, "Pplat": 20, "PEEP": 5, "flow": 60}, {"C_static": 33.3333333333, "C_dynamic": 25, "R_aw": 5, "driving_pressure": 15}),
               _ex({"VT": 450, "Ppeak": 30, "Pplat": 20, "PEEP": 5, "flow": 40}, {"C_static": 30, "R_aw": 15})],
     extra_q=[_bad({"VT": 500, "Ppeak": 25, "Pplat": 4, "PEEP": 5, "flow": 60})], level="intermediate", weak="the definitions re-solved with sympy")


def respiratory_time_constant(R, C, t=None):
    R, C = _pos(R, "R"), _pos(C, "C")
    tau = R * C / 1000.0
    out = {"tau": tau, "t_95": 3.0 * tau}
    if t is not None:
        out["fraction_exhaled"] = -math.expm1(-_nonneg(t, "t") / tau)
    return _rnd(out)


def ref_resp_tau(R, C, t=None):
    # passive exhalation: R dV/dt = -V/C (V in L, C in L/cmH2O), from V = 1
    c = C / 1000.0
    tau = _brentq(lambda s: _ivp(lambda u, y: [-y[0] / (R * c)], s, [1.0])[0] - math.exp(-1.0), 1e-3 * R * c, 10 * R * c)
    out = {"tau": tau, "t_95": 3 * R * c}
    if t is not None:
        out["fraction_exhaled"] = 1 - (_ivp(lambda s, y: [-y[0] / (R * c)], t, [1.0])[0] if t > 0 else 1.0)
    return out


_add("hp_respiratory_time_constant", name="Respiratory time constant and passive exhalation", topic="respiratory / mechanics",
     purpose="tau = R C, the fraction of the volume exhaled after time t, 1 - e^(-t/tau), and 3 tau (about 95 %).",
     inputs=[_n("R", "airway resistance in cmH2O s/L"), _n("C", "compliance in mL/cmH2O"),
             _n("t", "time in s (optional; needed for the fraction exhaled)", required=False)],
     outputs=[_o("tau", "time constant in s"), _o("fraction_exhaled", "fraction exhaled at time t"), _o("t_95", "three time constants in s")],
     primary="tau", conditions=["R, C > 0", "t >= 0"], method="single-compartment passive emptying", latex=r"\tau = R C,\quad V(t) = V_0 e^{-t/\tau}",
     run=respiratory_time_constant, reference=ref_resp_tau,
     wrong=lambda R, C, t=None: (lambda tau: dict({"tau": tau, "t_95": 3 * tau}, **({"fraction_exhaled": math.exp(-t / tau)} if t is not None else {})))(R / C),
     rand=lambda rng: _drop(rng, {"R": _u(rng, 2, 30, 1), "C": _u(rng, 15, 100, 0), "t": _u(rng, 0.1, 3, 2)}, "t"),
     phrases=["respiratory time constant"],
     examples=[_ex({"R": 5, "C": 50, "t": 0.5}, {"tau": 0.25, "fraction_exhaled": 0.864664716763, "t_95": 0.75}),
               _ex({"R": 10, "C": 80, "t": 1}, {"tau": 0.8, "fraction_exhaled": 0.71349520314}),
               _ex({"R": 5, "C": 50}, {"tau": 0.25, "t_95": 0.75})],
     extra_q=[_bad({"R": 0, "C": 50, "t": 0.5})], level="intermediate")


def alveolar_laplace(gamma, r):
    gamma, r = _pos(gamma, "gamma"), _pos(r, "r")
    p = 2.0 * gamma * 1e-3 / (r * 1e-6)
    return _rnd({"P": p, "P_cmH2O": p / CMH2O})


def ref_alveolar_laplace(gamma, r):
    g, a = gamma * 1e-3, r * 1e-6
    # hemisphere: surface tension along the rim balances the pressure force on the projected disc
    p = _brentq(lambda P: _quad(lambda t: P * 2 * math.pi * a * a * math.sin(t) * math.cos(t), 0.0, math.pi / 2) - g * 2 * math.pi * a, 1e-6, 1e9)
    return {"P": p, "P_cmH2O": p / 98.0665}


_add("hp_alveolar_laplace_pressure", name="Laplace pressure of an alveolus", topic="respiratory / mechanics",
     purpose="Pressure needed to keep a spherical alveolus open, P = 2 gamma/r (why surfactant matters).",
     inputs=[_n("gamma", "surface tension in mN/m"), _n("r", "alveolar radius in um")],
     outputs=[_o("P", "collapsing pressure in Pa"), _o("P_cmH2O", "collapsing pressure in cmH2O")],
     primary="P_cmH2O", conditions=["gamma, r > 0"], method="P = 2 gamma / r", latex=r"P = \frac{2\gamma}{r}",
     run=alveolar_laplace, reference=ref_alveolar_laplace,
     wrong=lambda gamma, r: {"P": gamma * 1e-3 / (r * 1e-6), "P_cmH2O": gamma * 1e-3 / (r * 1e-6) / CMH2O},
     rand=lambda rng: {"gamma": _u(rng, 1, 70, 1), "r": _u(rng, 20, 200, 0)},
     phrases=["alveolar Laplace pressure", "Laplace pressure of an alveolus"],
     examples=[_ex({"gamma": 25, "r": 100}, {"P": 500, "P_cmH2O": 5.09858106489}), _ex({"gamma": 5, "r": 50}, {"P": 200, "P_cmH2O": 2.03943242596})],
     extra_q=[_bad({"gamma": 25, "r": 0})], level="intermediate")


_ISA_EXP = G0 * 0.0289644 / (R_GAS * 0.0065)


def barometric_altitude(h, FiO2=0.2095):
    h = _num(h, "h")
    FiO2 = _fr(FiO2, "FiO2", open_lo=True)
    _need(0 <= h <= 11000, "altitude must be between 0 and 11000 m (troposphere)")
    p = 101325.0 * (1.0 - 0.0065 * h / 288.15) ** _ISA_EXP
    pm = p / MMHG
    return _rnd({"Patm": pm, "PiO2": FiO2 * (pm - 47.0), "Patm_kPa": p / 1000.0})


def ref_barometric(h, FiO2=0.2095):
    M, g, R = 0.0289644, 9.80665, 8.314462618
    p = _ivp(lambda z, y: [-y[0] * M * g / (R * (288.15 - 0.0065 * z))], h, [101325.0])[0] if h > 0 else 101325.0
    pm = p / 133.322387415
    return {"Patm": pm, "PiO2": FiO2 * (pm - 47), "Patm_kPa": p / 1000}


_add("hp_barometric_altitude", name="Barometric pressure and inspired PO2 at altitude", topic="respiratory / environment",
     purpose="Standard-atmosphere barometric pressure P = 101325 (1 - 0.0065 h/288.15)^(g M/(R L)) and the humidified inspired PO2.",
     inputs=[_n("h", "altitude in m"), _n("FiO2", "inspired oxygen fraction (optional; dry air if omitted)", required=False, pct=True)],
     outputs=[_o("Patm", "barometric pressure in mmHg"), _o("PiO2", "humidified inspired PO2 in mmHg"), _o("Patm_kPa", "barometric pressure in kPa")],
     primary="Patm", conditions=["0 <= h <= 11000 m"], method="ICAO standard troposphere (lapse 6.5 K/km)",
     latex=r"P = P_0\left(1 - \frac{L h}{T_0}\right)^{gM/(RL)}", run=barometric_altitude, reference=ref_barometric,
     wrong=lambda h, FiO2=0.2095: (lambda p: {"Patm": p / MMHG, "PiO2": FiO2 * p / MMHG, "Patm_kPa": p / 1000})(101325 * math.exp(-h / 8000)),
     rand=lambda rng: {"h": _u(rng, 100, 9000, 0), "FiO2": 0.2095},
     phrases=["barometric pressure at altitude", "pressure at altitude"],
     examples=[_ex({"h": 3000}, {"Patm": 525.860576939, "PiO2": 100.321290869}), _ex({"h": 5000, "FiO2": 0.21}, {"Patm": 405.186975495, "PiO2": 75.219264854})],
     extra_q=[_bad({"h": 20000})], level="intermediate")


def gas_volume_conversion(V, T, Patm, PH2O):
    V, T, Patm, PH2O = _pos(V, "V"), _num(T, "T"), _pos(Patm, "Patm"), _nonneg(PH2O, "PH2O")
    _need(-50 < T < 60 and Patm > PH2O and Patm > 47, "temperature -50..60 degC, Patm above the vapour pressures")
    tk = T + T0C
    return _rnd({"V_BTPS": V * (T_BODY / tk) * (Patm - PH2O) / (Patm - 47.0), "V_STPD": V * (T0C / tk) * (Patm - PH2O) / 760.0})


def ref_gas_conv(V, T, Patm, PH2O):
    mm, R = 133.322387415, 8.314462618
    n = (Patm - PH2O) * mm * V * 1e-3 / (R * (T + 273.15))            # moles of dry gas measured at ATPS
    return {"V_BTPS": n * R * 310.15 / ((Patm - 47) * mm) * 1e3, "V_STPD": n * R * 273.15 / (760 * mm) * 1e3}


_add("hp_gas_volume_conversion", name="ATPS to BTPS and STPD gas volume conversion", topic="respiratory / gas laws",
     purpose="A spirometer volume measured at ambient temperature and pressure, saturated (ATPS), converted to body (BTPS) and standard dry (STPD) conditions.",
     inputs=[_n("V", "measured volume in L"), _n("T", "spirometer temperature in degC"), _n("Patm", "barometric pressure in mmHg"),
             _n("PH2O", "water vapour pressure at T in mmHg")],
     outputs=[_o("V_BTPS", "volume at BTPS in L"), _o("V_STPD", "volume at STPD in L")],
     primary="V_BTPS", conditions=["Patm > PH2O", "Patm > 47 mmHg"], method="ideal gas law on the dry-gas partial pressures",
     latex=r"V_{BTPS} = V\,\frac{310.15}{T}\,\frac{P_B - P_{H_2O}}{P_B - 47}", run=gas_volume_conversion, reference=ref_gas_conv,
     wrong=lambda V, T, Patm, PH2O: {"V_BTPS": V * (310.15 / (T + 273.15)), "V_STPD": V * 273.15 / (T + 273.15)},
     rand=lambda rng: {"V": _u(rng, 0.5, 6, 2), "T": _u(rng, 18, 30, 1), "Patm": _u(rng, 600, 780, 0), "PH2O": _u(rng, 15, 32, 1)},
     phrases=["gas volume conversion", "BTPS conversion"],
     examples=[_ex({"V": 1, "T": 22, "Patm": 760, "PH2O": 19.8}, {"V_BTPS": 1.09090906067, "V_STPD": 0.901350918801}),
               _ex({"V": 2.5, "T": 25, "Patm": 740, "PH2O": 23.8}, {"V_BTPS": 2.68768311272, "V_STPD": 2.1583761044})],
     extra_q=[_bad({"V": 0, "T": 22, "Patm": 760, "PH2O": 19.8})], level="intermediate")


def weir_rq(VO2, VCO2):
    VO2, VCO2 = _pos(VO2, "VO2"), _pos(VCO2, "VCO2")
    ee = (3.941 * VO2 / 1000.0 + 1.106 * VCO2 / 1000.0) * 1440.0
    return _rnd({"RQ": VCO2 / VO2, "EE": ee, "EE_kJ": ee * 4.184})


_add("hp_weir_energy_expenditure", name="Respiratory quotient and energy expenditure (Weir equation)", topic="metabolism / indirect calorimetry",
     purpose="RQ = VCO2/VO2 and resting energy expenditure by the abbreviated Weir equation (3.941 VO2 + 1.106 VCO2) x 1440 kcal/day (L/min).",
     inputs=[_n("VO2", "oxygen consumption in mL/min"), _n("VCO2", "CO2 production in mL/min")],
     outputs=[_o("RQ", "respiratory quotient"), _o("EE", "energy expenditure in kcal/day"), _o("EE_kJ", "energy expenditure in kJ/day")],
     primary="EE", conditions=["VO2, VCO2 > 0"], method="Weir (1949) without urinary nitrogen", latex=r"EE = (3.941\,\dot V_{O_2} + 1.106\,\dot V_{CO_2})\times 1440",
     run=weir_rq,
     reference=lambda VO2, VCO2: (lambda e: {"RQ": _sx("r*a - b", "r", a=VO2, b=VCO2), "EE": e, "EE_kJ": e * 4.184})(_sx("E - (3941*a/1000 + 1106*b/1000)*1440/1000", "E", a=VO2, b=VCO2)),
     wrong=lambda VO2, VCO2: {"RQ": VO2 / VCO2, "EE": (3.941 * VCO2 + 1.106 * VO2) * 1.44, "EE_kJ": 0.0},
     rand=lambda rng: (lambda o: {"VO2": o, "VCO2": round(o * rng.uniform(0.7, 1.0), 0)})(_u(rng, 150, 400, 0)),
     phrases=["Weir energy expenditure", "respiratory quotient"],
     examples=[_ex({"VO2": 250, "VCO2": 200}, {"RQ": 0.8, "EE": 1737.288}), _ex({"VO2": 300, "VCO2": 270}, {"RQ": 0.9, "EE": 2132.5248})],
     extra_q=[_bad({"VO2": 0, "VCO2": 200})], level="intermediate", weak="the published coefficients re-solved with sympy")


def fev1_fvc(FEV1, FVC):
    FEV1, FVC = _pos(FEV1, "FEV1"), _pos(FVC, "FVC")
    _need(FEV1 <= FVC, "FEV1 cannot exceed FVC")
    return _rnd({"ratio": FEV1 / FVC, "percent": 100.0 * FEV1 / FVC})


_add("hp_fev1_fvc_ratio", name="FEV1/FVC ratio", topic="respiratory / spirometry",
     purpose="The fraction of the forced vital capacity exhaled in the first second.",
     inputs=[_n("FEV1", "forced expiratory volume in the first second in L"), _n("FVC", "forced vital capacity in L")],
     outputs=[_o("ratio", "FEV1/FVC"), _o("percent", "FEV1/FVC in %")],
     primary="ratio", conditions=["0 < FEV1 <= FVC"], method="FEV1 / FVC", latex=r"\frac{FEV_1}{FVC}",
     run=fev1_fvc, reference=lambda FEV1, FVC: {"ratio": _sx("r*F - E", "r", E=FEV1, F=FVC), "percent": _sx("p*F - 100*E", "p", E=FEV1, F=FVC)},
     wrong=lambda FEV1, FVC: {"ratio": FVC / FEV1, "percent": 100 * FVC / FEV1},
     rand=lambda rng: (lambda f: {"FVC": f, "FEV1": round(f * rng.uniform(0.4, 0.95), 2)})(_u(rng, 1.5, 6, 2)),
     phrases=["FEV1/FVC ratio", "FEV1/FVC"],
     examples=[_ex({"FEV1": 3.2, "FVC": 4}, {"ratio": 0.8, "percent": 80}), _ex({"FEV1": 1.5, "FVC": 3}, {"ratio": 0.5})],
     extra_q=[_bad({"FEV1": 5, "FVC": 4})], level="beginner", weak="the ratio re-solved with sympy")


# ================================================================================================ renal
def renal_clearance(U, V, P):
    U, V, P = _nonneg(U, "U"), _pos(V, "V"), _pos(P, "P")
    return _rnd({"C": U * V / P, "excretion": U * V / 100.0})


_add("hp_renal_clearance", name="Renal clearance of a substance (inulin GFR)", topic="renal / clearance",
     purpose="Clearance C = U V/P (equal to GFR for inulin) and the urinary excretion rate U V.",
     inputs=[_n("U", "urine concentration in mg/dL"), _n("V", "urine flow in mL/min"), _n("P", "plasma concentration in mg/dL")],
     outputs=[_o("C", "clearance in mL/min"), _o("excretion", "excretion rate in mg/min")],
     primary="C", conditions=["U >= 0", "V, P > 0"], method="C = U V / P", latex=r"C_x = \frac{U_x \dot V}{P_x}",
     run=renal_clearance,
     reference=lambda U, V, P: {"C": _sx("C*P - U*V", "C", U=U, V=V, P=P), "excretion": _sx("E*100 - U*V", "E", U=U, V=V)},
     wrong=lambda U, V, P: {"C": U * P / V, "excretion": U * V},
     rand=lambda rng: {"U": _u(rng, 1, 200, 1), "V": _u(rng, 0.3, 5, 2), "P": _u(rng, 0.1, 5, 2)},
     phrases=["renal clearance", "inulin clearance"],
     examples=[_ex({"U": 30, "V": 1, "P": 0.25}, {"C": 120, "excretion": 0.3}), _ex({"U": 100, "V": 1.5, "P": 1.2}, {"C": 125})],
     extra_q=[_bad({"U": 30, "V": 1, "P": 0})], level="beginner", weak="the definition re-solved with sympy")


def creatinine_clearance_timed(Ucr, Vurine, hours, Pcr, BSA=1.73):
    Ucr, Vurine, hours, Pcr, BSA = _pos(Ucr, "Ucr"), _pos(Vurine, "Vurine"), _pos(hours, "hours"), _pos(Pcr, "Pcr"), _pos(BSA, "BSA")
    v = Vurine / (hours * 60.0)
    c = Ucr * v / Pcr
    return _rnd({"urine_flow": v, "CrCl": c, "CrCl_normalized": c * 1.73 / BSA})


def ref_crcl_timed(Ucr, Vurine, hours, Pcr, BSA=1.73):
    v = _sx("f*h*60 - W", "f", W=Vurine, h=hours)
    c = _sx("C*P - U*f", "C", U=Ucr, f=v, P=Pcr)
    return {"urine_flow": v, "CrCl": c, "CrCl_normalized": _sx("N*B - C*173/100", "N", C=c, B=BSA)}


_add("hp_creatinine_clearance_timed", name="Creatinine clearance from a timed urine collection", topic="renal / clearance",
     purpose="Urine flow from a timed collection, creatinine clearance U V/P and its value normalized to 1.73 m^2.",
     inputs=[_n("Ucr", "urine creatinine in mg/dL"), _n("Vurine", "urine volume collected in mL"), _n("hours", "collection time in h"),
             _n("Pcr", "plasma creatinine in mg/dL"), _n("BSA", "body surface area in m^2 (optional; standard adult area if omitted)", required=False)],
     outputs=[_o("urine_flow", "urine flow in mL/min"), _o("CrCl", "creatinine clearance in mL/min"),
              _o("CrCl_normalized", "creatinine clearance in mL/min normalized to standard body surface area")],
     primary="CrCl", conditions=["all inputs > 0"], method="V = volume/(60 hours); CrCl = U V/P; x 1.73/BSA",
     latex=r"C_{Cr} = \frac{U_{Cr}\dot V}{P_{Cr}}", run=creatinine_clearance_timed, reference=ref_crcl_timed,
     wrong=lambda Ucr, Vurine, hours, Pcr, BSA=1.73: (lambda v: {"urine_flow": v, "CrCl": Ucr * v / Pcr, "CrCl_normalized": Ucr * v / Pcr * BSA / 1.73})(Vurine / hours),
     rand=lambda rng: {"Ucr": _u(rng, 30, 250, 0), "Vurine": _u(rng, 600, 3000, 0), "hours": rng.choice([12, 24]), "Pcr": _u(rng, 0.5, 3, 2), "BSA": _u(rng, 1.4, 2.4, 2)},
     phrases=["creatinine clearance from a timed urine", "timed creatinine clearance"],
     examples=[_ex({"Ucr": 100, "Vurine": 1440, "hours": 24, "Pcr": 1}, {"urine_flow": 1, "CrCl": 100, "CrCl_normalized": 100}),
               _ex({"Ucr": 80, "Vurine": 1800, "hours": 24, "Pcr": 1.2, "BSA": 2}, {"urine_flow": 1.25, "CrCl": 83.3333333333, "CrCl_normalized": 72.0833333333})],
     extra_q=[_bad({"Ucr": 100, "Vurine": 1440, "hours": 0, "Pcr": 1})], level="intermediate", weak="the definitions re-solved with sympy")


def pah_renal_flow(U_PAH, V, P_PAH, Hct, E=1.0):
    U, V, P = _pos(U_PAH, "U_PAH"), _pos(V, "V"), _pos(P_PAH, "P_PAH")
    Hct, E = _fr(Hct, "Hct", open_hi=True), _fr(E, "E", open_lo=True)
    erpf = U * V / P
    rpf = erpf / E
    return _rnd({"eRPF": erpf, "RPF": rpf, "RBF": rpf / (1.0 - Hct)})


def ref_pah(U_PAH, V, P_PAH, Hct, E=1.0):
    e = _sx("C*P - U*V", "C", U=U_PAH, V=V, P=P_PAH)
    r = _sx("R*E - e", "R", e=e, E=E)
    return {"eRPF": e, "RPF": r, "RBF": _sx("B*(1 - H) - R", "B", R=r, H=Hct)}


_add("hp_pah_renal_flow", name="Renal plasma and blood flow from PAH clearance", topic="renal / renal blood flow",
     purpose="Effective renal plasma flow = PAH clearance, true RPF = eRPF/E (PAH extraction) and RBF = RPF/(1 - Hct).",
     inputs=[_n("U_PAH", "urine PAH concentration in mg/dL"), _n("V", "urine flow in mL/min"), _n("P_PAH", "plasma PAH concentration in mg/dL"),
             _n("Hct", "hematocrit (fraction)", pct=True), _n("E", "PAH extraction ratio (optional; complete extraction, the effective flow, if omitted)", required=False, pct=True)],
     outputs=[_o("eRPF", "effective renal plasma flow in mL/min"), _o("RPF", "renal plasma flow in mL/min"), _o("RBF", "renal blood flow in mL/min")],
     primary="RBF", conditions=["0 <= Hct < 1", "0 < E <= 1"], method="eRPF = U V/P; RPF = eRPF/E; RBF = RPF/(1 - Hct)",
     latex=r"RBF = \frac{U_{PAH}\dot V / P_{PAH}}{E_{PAH}(1 - Hct)}", run=pah_renal_flow, reference=ref_pah,
     wrong=lambda U_PAH, V, P_PAH, Hct, E=1.0: (lambda r: {"eRPF": U_PAH * V / P_PAH, "RPF": r, "RBF": r * (1 - Hct)})(U_PAH * V / P_PAH * E),
     rand=lambda rng: {"U_PAH": _u(rng, 200, 900, 0), "V": _u(rng, 0.5, 3, 2), "P_PAH": _u(rng, 0.5, 2, 2), "Hct": _u(rng, 0.3, 0.55, 2), "E": _u(rng, 0.8, 1.0, 2)},
     phrases=["renal plasma flow", "PAH clearance"],
     examples=[_ex({"U_PAH": 600, "V": 1, "P_PAH": 1, "Hct": 0.45, "E": 0.9}, {"eRPF": 600, "RPF": 666.666666667, "RBF": 1212.12121212}),
               _ex({"U_PAH": 650, "V": 1, "P_PAH": 1, "Hct": 0.35}, {"eRPF": 650, "RBF": 1000})],
     extra_q=[_bad({"U_PAH": 600, "V": 1, "P_PAH": 1, "Hct": 1})], level="intermediate", weak="the definitions re-solved with sympy")


def filtration_fraction(GFR, RPF):
    GFR, RPF = _pos(GFR, "GFR"), _pos(RPF, "RPF")
    _need(GFR < RPF, "GFR must be smaller than the renal plasma flow")
    return _rnd({"FF": GFR / RPF, "FF_percent": 100.0 * GFR / RPF})


_add("hp_filtration_fraction", name="Filtration fraction", topic="renal / clearance",
     purpose="The fraction of renal plasma flow filtered at the glomerulus, FF = GFR/RPF.",
     inputs=[_n("GFR", "glomerular filtration rate in mL/min"), _n("RPF", "renal plasma flow in mL/min")],
     outputs=[_o("FF", "filtration fraction"), _o("FF_percent", "filtration fraction in %")],
     primary="FF", conditions=["0 < GFR < RPF"], method="FF = GFR / RPF", latex=r"FF = \frac{GFR}{RPF}",
     run=filtration_fraction, reference=lambda GFR, RPF: {"FF": _sx("f*R - G", "f", G=GFR, R=RPF), "FF_percent": _sx("p*R - 100*G", "p", G=GFR, R=RPF)},
     wrong=lambda GFR, RPF: {"FF": GFR / (RPF + GFR), "FF_percent": 100 * GFR / (RPF + GFR)},
     rand=lambda rng: (lambda r: {"RPF": r, "GFR": round(r * rng.uniform(0.1, 0.35), 0)})(_u(rng, 300, 900, 0)),
     phrases=["filtration fraction"],
     examples=[_ex({"GFR": 120, "RPF": 600}, {"FF": 0.2, "FF_percent": 20}), _ex({"GFR": 100, "RPF": 400}, {"FF": 0.25})],
     extra_q=[_bad({"GFR": 700, "RPF": 600})], level="beginner", weak="the ratio re-solved with sympy")


def free_water_clearance(V, Uosm, Posm):
    V, Uosm, Posm = _pos(V, "V"), _pos(Uosm, "Uosm"), _pos(Posm, "Posm")
    cosm = Uosm * V / Posm
    return _rnd({"Cosm": cosm, "CH2O": V - cosm})


_add("hp_free_water_clearance", name="Osmolar and free water clearance", topic="renal / water balance",
     purpose="Osmolar clearance Cosm = Uosm V/Posm and free water clearance CH2O = V - Cosm (negative when the urine is concentrated).",
     inputs=[_n("V", "urine flow in mL/min"), _n("Uosm", "urine osmolality in mOsm/kg"), _n("Posm", "plasma osmolality in mOsm/kg")],
     outputs=[_o("Cosm", "osmolar clearance in mL/min"), _o("CH2O", "free water clearance in mL/min")],
     primary="CH2O", conditions=["all inputs > 0"], method="CH2O = V (1 - Uosm/Posm)", latex=r"C_{H_2O} = \dot V - \frac{U_{osm}\dot V}{P_{osm}}",
     run=free_water_clearance,
     reference=lambda V, Uosm, Posm: (lambda c: {"Cosm": c, "CH2O": _sx("W - V + c", "W", V=V, c=c)})(_sx("c*P - U*V", "c", U=Uosm, V=V, P=Posm)),
     wrong=lambda V, Uosm, Posm: {"Cosm": Uosm * V / Posm, "CH2O": Uosm * V / Posm - V},
     rand=lambda rng: {"V": _u(rng, 0.3, 15, 2), "Uosm": _u(rng, 50, 1200, 0), "Posm": _u(rng, 270, 310, 0)},
     phrases=["free water clearance"],
     examples=[_ex({"V": 2, "Uosm": 150, "Posm": 300}, {"Cosm": 1, "CH2O": 1}), _ex({"V": 1, "Uosm": 900, "Posm": 300}, {"Cosm": 3, "CH2O": -2})],
     extra_q=[_bad({"V": 2, "Uosm": 150, "Posm": 0})], level="intermediate", weak="the definitions re-solved with sympy")


def fractional_excretion(Ux, Px, Ucr, Pcr):
    Ux, Px, Ucr, Pcr = _nonneg(Ux, "Ux"), _pos(Px, "Px"), _pos(Ucr, "Ucr"), _pos(Pcr, "Pcr")
    fe = Ux * Pcr / (Px * Ucr)
    return _rnd({"FE": fe, "FE_percent": 100.0 * fe})


def ref_fe(Ux, Px, Ucr, Pcr):
    # excreted / filtered load, with GFR = creatinine clearance for an arbitrary urine flow (it cancels)
    v = 1.7
    gfr = Ucr * v / Pcr
    fe = (Ux * v) / (gfr * Px)
    return {"FE": fe, "FE_percent": 100 * fe}


_add("hp_fractional_excretion", name="Fractional excretion of a solute", topic="renal / tubular function",
     purpose="FE = (Ux Pcr)/(Px Ucr): the fraction of the filtered load excreted, with creatinine clearance as GFR (e.g. FENa).",
     inputs=[_n("Ux", "urine concentration of the solute (any unit)"), _n("Px", "plasma concentration of the solute (same unit)"),
             _n("Ucr", "urine creatinine (any unit)"), _n("Pcr", "plasma creatinine (same unit)")],
     outputs=[_o("FE", "fractional excretion"), _o("FE_percent", "fractional excretion in %")],
     primary="FE_percent", conditions=["Px, Ucr, Pcr > 0"], method="FE = Ux Pcr / (Px Ucr)", latex=r"FE_x = \frac{U_x P_{Cr}}{P_x U_{Cr}}",
     run=fractional_excretion, reference=ref_fe,
     wrong=lambda Ux, Px, Ucr, Pcr: {"FE": Ux * Ucr / (Px * Pcr), "FE_percent": 100 * Ux * Ucr / (Px * Pcr)},
     rand=lambda rng: {"Ux": _u(rng, 5, 150, 0), "Px": _u(rng, 130, 150, 0), "Ucr": _u(rng, 30, 200, 0), "Pcr": _u(rng, 0.5, 4, 2)},
     phrases=["fractional excretion"],
     examples=[_ex({"Ux": 40, "Px": 140, "Ucr": 100, "Pcr": 1}, {"FE": 0.00285714285714, "FE_percent": 0.285714285714}),
               _ex({"Ux": 2, "Px": 140, "Ucr": 120, "Pcr": 1.5}, {"FE": 0.000178571428571})],
     extra_q=[_bad({"Ux": 40, "Px": 0, "Ucr": 100, "Pcr": 1})], level="intermediate")


def egfr_ckd_epi_2021(Scr, age, sex):
    Scr, age = _pos(Scr, "Scr"), _num(age, "age")
    fem = _sex(sex)
    _need(18 <= age <= 120, "the CKD-EPI equation applies to ages 18 to 120")
    k, a = (0.7, -0.241) if fem else (0.9, -0.302)
    s = Scr / k
    v = 142.0 * min(s, 1.0) ** a * max(s, 1.0) ** -1.200 * 0.9938 ** age * (1.012 if fem else 1.0)
    return _rnd({"eGFR": v})


def ref_egfr_2021(Scr, age, sex):
    import mpmath as mp
    mp.mp.dps = 30
    fem = sex == "female"
    k, a = (mp.mpf("0.7"), mp.mpf("-0.241")) if fem else (mp.mpf("0.9"), mp.mpf("-0.302"))
    s = mp.mpf(repr(Scr)) / k
    low = mp.power(s, a) if s < 1 else mp.mpf(1)
    high = mp.power(s, mp.mpf("-1.200")) if s > 1 else mp.mpf(1)
    v = 142 * low * high * mp.power(mp.mpf("0.9938"), mp.mpf(repr(age))) * (mp.mpf("1.012") if fem else 1)
    return {"eGFR": float(v)}


_add("hp_egfr_ckd_epi_2021", name="Estimated GFR by the CKD-EPI 2021 creatinine equation", topic="renal / GFR estimation",
     purpose="eGFR = 142 min(Scr/k, 1)^a max(Scr/k, 1)^-1.200 0.9938^age (x 1.012 female), the race-free 2021 CKD-EPI creatinine equation (computed value only).",
     inputs=[_n("Scr", "serum creatinine in mg/dL"), _n("age", "age in years"), _ch("sex", ["female", "male"], "sex for the equation's coefficients")],
     outputs=[_o("eGFR", "estimated GFR in mL/min normalized to standard body surface area")],
     primary="eGFR", conditions=["Scr > 0", "18 <= age <= 120"], method="CKD-EPI 2021: k = 0.7/0.9, a = -0.241/-0.302 (female/male)",
     latex=r"eGFR = 142\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-1.200}\,0.9938^{age}\,[1.012]",
     run=egfr_ckd_epi_2021, reference=ref_egfr_2021,
     wrong=lambda Scr, age, sex: {"eGFR": 142 * (Scr / 0.9) ** -1.2 * 0.9938 ** (age / 10)},
     rand=lambda rng: {"Scr": _u(rng, 0.4, 5, 2), "age": _u(rng, 18, 90, 0), "sex": rng.choice(["female", "male"])},
     phrases=["eGFR CKD-EPI 2021", "CKD-EPI 2021 eGFR"],
     examples=[_ex({"Scr": 1, "age": 50, "sex": "male"}, {"eGFR": 91.6914786098}), _ex({"Scr": 0.8, "age": 65, "sex": "female"}, {"eGFR": 81.717115291}),
               _ex({"Scr": 0.6, "age": 30, "sex": "female"}, {"eGFR": 123.757864122})],
     extra_q=[_bad({"Scr": 1, "age": 10, "sex": "male"})], level="advanced", weak="the published equation evaluated in 30-digit mpmath")


def egfr_ckd_epi_2021_cys(Scr, Scys, age, sex):
    Scr, Scys, age = _pos(Scr, "Scr"), _pos(Scys, "Scys"), _num(age, "age")
    fem = _sex(sex)
    _need(18 <= age <= 120, "the CKD-EPI equation applies to ages 18 to 120")
    k, a = (0.7, -0.219) if fem else (0.9, -0.144)
    s, c = Scr / k, Scys / 0.8
    v = (135.0 * min(s, 1.0) ** a * max(s, 1.0) ** -0.544 * min(c, 1.0) ** -0.323 * max(c, 1.0) ** -0.778
         * 0.9961 ** age * (0.963 if fem else 1.0))
    return _rnd({"eGFR": v})


def ref_egfr_cys(Scr, Scys, age, sex):
    import mpmath as mp
    mp.mp.dps = 30
    fem = sex == "female"
    k, a = (mp.mpf("0.7"), mp.mpf("-0.219")) if fem else (mp.mpf("0.9"), mp.mpf("-0.144"))
    s, c = mp.mpf(repr(Scr)) / k, mp.mpf(repr(Scys)) / mp.mpf("0.8")
    p = lambda x, lo, hi: mp.power(x, lo) if x < 1 else mp.power(x, hi)
    v = 135 * p(s, a, mp.mpf("-0.544")) * p(c, mp.mpf("-0.323"), mp.mpf("-0.778")) * mp.power(mp.mpf("0.9961"), mp.mpf(repr(age)))
    return {"eGFR": float(v * (mp.mpf("0.963") if fem else 1))}


_add("hp_egfr_ckd_epi_2021_cys", name="Estimated GFR by the CKD-EPI 2021 creatinine-cystatin C equation", topic="renal / GFR estimation",
     purpose="The race-free 2021 CKD-EPI equation from serum creatinine and cystatin C (computed value only).",
     inputs=[_n("Scr", "serum creatinine in mg/dL"), _n("Scys", "serum cystatin C in mg/L"), _n("age", "age in years"),
             _ch("sex", ["female", "male"], "sex for the equation's coefficients")],
     outputs=[_o("eGFR", "estimated GFR in mL/min normalized to standard body surface area")],
     primary="eGFR", conditions=["Scr, Scys > 0", "18 <= age <= 120"],
     method="135 min(Scr/k,1)^a max(Scr/k,1)^-0.544 min(Scys/0.8,1)^-0.323 max(Scys/0.8,1)^-0.778 0.9961^age (x 0.963 female)",
     latex=r"eGFR = 135\,\min(S_{cr}/\kappa,1)^{\alpha}\max(S_{cr}/\kappa,1)^{-0.544}\min(S_{cys}/0.8,1)^{-0.323}\max(S_{cys}/0.8,1)^{-0.778}\,0.9961^{age}",
     run=egfr_ckd_epi_2021_cys, reference=ref_egfr_cys,
     wrong=lambda Scr, Scys, age, sex: {"eGFR": 135 * (Scr / 0.9) ** -0.544 * (Scys / 0.8) ** -0.778 * 0.9961 ** (age / 10)},
     rand=lambda rng: {"Scr": _u(rng, 0.4, 4, 2), "Scys": _u(rng, 0.5, 3, 2), "age": _u(rng, 18, 90, 0), "sex": rng.choice(["female", "male"])},
     phrases=["eGFR creatinine-cystatin C", "CKD-EPI 2021 creatinine-cystatin C eGFR"],
     examples=[_ex({"Scr": 1, "Scys": 1, "age": 50, "sex": "male"}, {"eGFR": 88.1439905436}), _ex({"Scr": 0.8, "Scys": 0.9, "age": 65, "sex": "female"}, {"eGFR": 85.5671132147})],
     extra_q=[_bad({"Scr": 1, "Scys": 0, "age": 50, "sex": "male"})], level="expert", weak="the published equation evaluated in 30-digit mpmath")


def cockcroft_gault(age, weight, Scr, sex):
    age, weight, Scr = _num(age, "age"), _pos(weight, "weight"), _pos(Scr, "Scr")
    fem = _sex(sex)
    _need(18 <= age < 140, "age must be at least 18 and below 140")
    return _rnd({"CrCl": (140.0 - age) * weight / (72.0 * Scr) * (0.85 if fem else 1.0)})


_add("hp_cockcroft_gault", name="Creatinine clearance by the Cockcroft-Gault equation", topic="renal / GFR estimation",
     purpose="Estimated creatinine clearance (140 - age) x weight/(72 x Scr), x 0.85 for female (computed value only).",
     inputs=[_n("age", "age in years"), _n("weight", "body weight in kg"), _n("Scr", "serum creatinine in mg/dL"),
             _ch("sex", ["female", "male"], "sex for the equation's coefficient")],
     outputs=[_o("CrCl", "estimated creatinine clearance in mL/min")],
     primary="CrCl", conditions=["18 <= age < 140", "weight, Scr > 0"], method="Cockcroft-Gault (1976)",
     latex=r"C_{Cr} = \frac{(140 - age)\,W}{72\,S_{cr}}\;[\times 0.85]", run=cockcroft_gault,
     reference=lambda age, weight, Scr, sex: {"CrCl": _sx("C*72*S - (140 - a)*w*f", "C", a=age, w=weight, S=Scr, f=0.85 if sex == "female" else 1.0)},
     wrong=lambda age, weight, Scr, sex: {"CrCl": (140 - age) * weight / (72 * Scr) * (1.0 if sex == "female" else 0.85)},
     rand=lambda rng: {"age": _u(rng, 18, 95, 0), "weight": _u(rng, 40, 130, 1), "Scr": _u(rng, 0.5, 4, 2), "sex": rng.choice(["female", "male"])},
     phrases=["Cockcroft-Gault creatinine clearance", "Cockcroft-Gault"],
     examples=[_ex({"age": 60, "weight": 72, "Scr": 1, "sex": "male"}, {"CrCl": 80}), _ex({"age": 70, "weight": 60, "Scr": 1.2, "sex": "female"}, {"CrCl": 41.3194444444})],
     extra_q=[_bad({"age": 60, "weight": 72, "Scr": 0, "sex": "male"})], level="intermediate", weak="the published equation re-solved with sympy")


def tubular_handling(GFR, Px, Ux, V):
    GFR, Px, Ux, V = _pos(GFR, "GFR"), _nonneg(Px, "Px"), _nonneg(Ux, "Ux"), _pos(V, "V")
    fl, ex = GFR * Px / 100.0, Ux * V / 100.0
    return _rnd({"filtered_load": fl, "excreted": ex, "reabsorbed": fl - ex})


_add("hp_tubular_handling", name="Filtered load, excretion and net tubular reabsorption or secretion", topic="renal / tubular function",
     purpose="Filtered load GFR x Px, excretion rate Ux x V and net reabsorption (positive) or secretion (negative) as their difference.",
     inputs=[_n("GFR", "GFR in mL/min"), _n("Px", "plasma concentration in mg/dL"), _n("Ux", "urine concentration in mg/dL"), _n("V", "urine flow in mL/min")],
     outputs=[_o("filtered_load", "filtered load in mg/min"), _o("excreted", "excretion rate in mg/min"),
              _o("reabsorbed", "net reabsorption in mg/min (negative = net secretion)")],
     primary="reabsorbed", conditions=["GFR, V > 0"], method="filtered - excreted (mg/dL x mL/min / 100 = mg/min)",
     latex=r"R_x = GFR\cdot P_x - U_x \dot V", run=tubular_handling,
     reference=lambda GFR, Px, Ux, V: (lambda f, e: {"filtered_load": f, "excreted": e, "reabsorbed": _sx("r - f + e", "r", f=f, e=e)})(
         _sx("F*100 - G*P", "F", G=GFR, P=Px), _sx("E*100 - U*V", "E", U=Ux, V=V)),
     wrong=lambda GFR, Px, Ux, V: {"filtered_load": GFR * Px / 100, "excreted": Ux * V / 100, "reabsorbed": Ux * V / 100 - GFR * Px / 100},
     rand=lambda rng: {"GFR": _u(rng, 30, 150, 0), "Px": _u(rng, 1, 300, 1), "Ux": _u(rng, 0, 1500, 0), "V": _u(rng, 0.3, 4, 2)},
     phrases=["tubular reabsorption", "filtered load"],
     examples=[_ex({"GFR": 120, "Px": 100, "Ux": 0, "V": 1}, {"filtered_load": 120, "excreted": 0, "reabsorbed": 120}),
               _ex({"GFR": 125, "Px": 15, "Ux": 600, "V": 1}, {"filtered_load": 18.75, "excreted": 6, "reabsorbed": 12.75})],
     extra_q=[_bad({"GFR": 0, "Px": 100, "Ux": 0, "V": 1})], level="intermediate", weak="the mass balance re-solved with sympy")


def glucose_titration(GFR, Pglucose, Tm):
    GFR, P, Tm = _pos(GFR, "GFR"), _nonneg(Pglucose, "Pglucose"), _pos(Tm, "Tm")
    fl = GFR * P / 100.0
    re_ = min(fl, Tm)
    return _rnd({"filtered_load": fl, "reabsorbed": re_, "excreted": fl - re_, "threshold": Tm / GFR * 100.0})


def ref_glucose_titration(GFR, Pglucose, Tm):
    fl = GFR * Pglucose / 100.0
    thr = _brentq(lambda p: GFR * p / 100.0 - Tm, 0.0, 1e7)
    ex = 0.0 if Pglucose <= thr else _quad(lambda p: GFR / 100.0, thr, Pglucose)     # excretion rises with slope GFR above threshold
    return {"filtered_load": fl, "reabsorbed": fl - ex, "excreted": ex, "threshold": thr}


_add("hp_glucose_titration", name="Glucose titration curve (ideal transport maximum)", topic="renal / tubular function",
     purpose="Filtered, reabsorbed (at most Tm) and excreted glucose and the plasma threshold Tm/GFR, ignoring splay.",
     inputs=[_n("GFR", "GFR in mL/min"), _n("Pglucose", "plasma glucose in mg/dL"), _n("Tm", "transport maximum in mg/min")],
     outputs=[_o("filtered_load", "filtered load in mg/min"), _o("reabsorbed", "reabsorbed in mg/min"), _o("excreted", "excreted in mg/min"),
              _o("threshold", "plasma threshold in mg/dL")],
     primary="excreted", conditions=["GFR, Tm > 0", "Pglucose >= 0"], method="reabsorbed = min(filtered, Tm)",
     latex=r"E = \max(0,\; GFR\cdot P_G - T_m)", run=glucose_titration, reference=ref_glucose_titration,
     wrong=lambda GFR, Pglucose, Tm: (lambda fl: {"filtered_load": fl, "reabsorbed": Tm, "excreted": fl - Tm, "threshold": Tm * GFR / 100})(GFR * Pglucose / 100),
     rand=lambda rng: {"GFR": _u(rng, 60, 150, 0), "Pglucose": _u(rng, 50, 800, 0), "Tm": _u(rng, 250, 450, 0)},
     phrases=["glucose titration", "glucose transport maximum"],
     examples=[_ex({"GFR": 125, "Pglucose": 400, "Tm": 375}, {"filtered_load": 500, "reabsorbed": 375, "excreted": 125, "threshold": 300}),
               _ex({"GFR": 120, "Pglucose": 100, "Tm": 375}, {"filtered_load": 120, "reabsorbed": 120, "excreted": 0, "threshold": 312.5})],
     extra_q=[_bad({"GFR": 125, "Pglucose": 400, "Tm": 0})], level="advanced")


def plasma_osmolality(Na, glucose, BUN, measured=None):
    Na, glucose, BUN = _pos(Na, "Na"), _nonneg(glucose, "glucose"), _nonneg(BUN, "BUN")
    calc = 2.0 * Na + glucose / 18.0 + BUN / 2.8
    out = {"calculated": calc, "effective": 2.0 * Na + glucose / 18.0}
    if measured is not None:
        out["osmolal_gap"] = _pos(measured, "measured") - calc
    return _rnd(out)


def ref_plasma_osm(Na, glucose, BUN, measured=None):
    c = _sx("c - 2*N - g/18 - b*10/28", "c", N=Na, g=glucose, b=BUN)
    out = {"calculated": c, "effective": _sx("e - 2*N - g/18", "e", N=Na, g=glucose)}
    if measured is not None:
        out["osmolal_gap"] = measured - c
    return out


_add("hp_plasma_osmolality", name="Calculated and effective plasma osmolality and the osmolal gap", topic="renal / body fluids",
     purpose="Calculated osmolality 2 Na + glucose/18 + BUN/2.8, effective osmolality (tonicity) 2 Na + glucose/18 and, with a measured value, the osmolal gap.",
     inputs=[_n("Na", "sodium in mmol/L"), _n("glucose", "glucose in mg/dL"), _n("BUN", "blood urea nitrogen in mg/dL"),
             _n("measured", "measured osmolality in mOsm/kg (optional)", required=False)],
     outputs=[_o("calculated", "calculated osmolality in mOsm/kg"), _o("effective", "effective osmolality in mOsm/kg"),
              _o("osmolal_gap", "measured - calculated in mOsm/kg")],
     primary="calculated", conditions=["Na > 0"], method="published divisors 18 (glucose) and 2.8 (urea nitrogen)",
     latex=r"P_{osm} = 2[Na^+] + \frac{glucose}{18} + \frac{BUN}{2.8}", run=plasma_osmolality, reference=ref_plasma_osm,
     wrong=lambda Na, glucose, BUN, measured=None: (lambda c: dict({"calculated": c, "effective": Na + glucose / 18}, **({"osmolal_gap": measured - c} if measured is not None else {})))(Na + glucose / 18 + BUN / 2.8),
     rand=lambda rng: _drop(rng, {"Na": _u(rng, 120, 160, 0), "glucose": _u(rng, 60, 600, 0), "BUN": _u(rng, 5, 80, 0), "measured": _u(rng, 270, 340, 0)}, "measured"),
     phrases=["plasma osmolality", "calculated osmolality"],
     examples=[_ex({"Na": 140, "glucose": 90, "BUN": 14}, {"calculated": 290, "effective": 285}),
               _ex({"Na": 135, "glucose": 180, "BUN": 28, "measured": 300}, {"calculated": 290, "effective": 280, "osmolal_gap": 10})],
     extra_q=[_bad({"Na": 0, "glucose": 90, "BUN": 14})], level="intermediate", weak="the published formula re-solved with sympy")


# ================================================================================================ body fluids
def fluid_compartments(weight, TBW_fraction=0.6):
    weight, f = _pos(weight, "weight"), _fr(TBW_fraction, "TBW_fraction", open_lo=True)
    tbw = f * weight
    ecf = tbw / 3.0
    return _rnd({"TBW": tbw, "ICF": 2.0 * tbw / 3.0, "ECF": ecf, "ISF": 0.75 * ecf, "plasma": 0.25 * ecf})


def ref_fluid_comp(weight, TBW_fraction=0.6):
    import numpy as np
    t = TBW_fraction * weight
    # ICF + ECF = TBW, ICF = 2 ECF, ISF + plasma = ECF, ISF = 3 plasma
    A = np.array([[1, 1, 0, 0], [1, -2, 0, 0], [0, 1, -1, -1], [0, 0, 1, -3]], dtype=float)
    x = np.linalg.solve(A, np.array([t, 0, 0, 0]))
    return {"TBW": t, "ICF": float(x[0]), "ECF": float(x[1]), "ISF": float(x[2]), "plasma": float(x[3])}


_add("hp_body_fluid_compartments", name="Body fluid compartments", topic="body fluids / compartments",
     purpose="Total body water (60 % of weight by default), intracellular 2/3, extracellular 1/3, interstitial 3/4 and plasma 1/4 of the ECF.",
     inputs=[_n("weight", "body weight in kg"), _n("TBW_fraction", "total body water fraction of weight (optional; the textbook adult fraction if omitted)", required=False, pct=True)],
     outputs=[_o("TBW", "total body water in L"), _o("ICF", "intracellular fluid in L"), _o("ECF", "extracellular fluid in L"),
              _o("ISF", "interstitial fluid in L"), _o("plasma", "plasma volume in L")],
     primary="TBW", conditions=["weight > 0"], method="60-40-20 rule: ICF 2/3, ECF 1/3 of TBW; plasma 1/4 of ECF",
     latex=r"TBW = 0.6\,W,\; ICF = \tfrac23 TBW,\; ECF = \tfrac13 TBW", run=fluid_compartments, reference=ref_fluid_comp,
     wrong=lambda weight, TBW_fraction=0.6: (lambda t: {"TBW": t, "ICF": t / 3, "ECF": 2 * t / 3, "ISF": t / 2, "plasma": t / 6})(TBW_fraction * weight),
     rand=lambda rng: {"weight": _u(rng, 3, 150, 1), "TBW_fraction": _u(rng, 0.45, 0.75, 2)},
     phrases=["body fluid compartments"],
     examples=[_ex({"weight": 70}, {"TBW": 42, "ICF": 28, "ECF": 14, "ISF": 10.5, "plasma": 3.5}), _ex({"weight": 60, "TBW_fraction": 0.5}, {"TBW": 30, "ECF": 10})],
     extra_q=[_bad({"weight": 0})], level="beginner", weak="the textbook proportions solved as a linear system")


def watson_tbw(age, height, weight, sex):
    age, height, weight = _num(age, "age"), _pos(height, "height"), _pos(weight, "weight")
    fem = _sex(sex)
    _need(age >= 18, "the Watson equations are for adults")
    v = -2.097 + 0.1069 * height + 0.2466 * weight if fem else 2.447 - 0.09516 * age + 0.1074 * height + 0.3362 * weight
    return _rnd({"TBW": v})


_add("hp_watson_total_body_water", name="Total body water by the Watson equations", topic="body fluids / compartments",
     purpose="Watson (1980): male 2.447 - 0.09516 age + 0.1074 H + 0.3362 W; female -2.097 + 0.1069 H + 0.2466 W (litres).",
     inputs=[_n("age", "age in years"), _n("height", "height in cm"), _n("weight", "weight in kg"), _ch("sex", ["female", "male"], "sex for the equation")],
     outputs=[_o("TBW", "total body water in L")],
     primary="TBW", conditions=["adult", "height, weight > 0"], method="Watson anthropometric regression", latex=r"TBW_{m} = 2.447 - 0.09516\,A + 0.1074\,H + 0.3362\,W",
     run=watson_tbw,
     reference=lambda age, height, weight, sex: {"TBW": _sx("T + 2097/1000 - 1069*H/10000 - 1233*W/5000", "T", H=height, W=weight) if sex == "female"
                                                 else _sx("T - 2447/1000 + 9516*A/100000 - 1074*H/10000 - 1681*W/5000", "T", A=age, H=height, W=weight)},
     wrong=lambda age, height, weight, sex: {"TBW": 2.447 - 0.09516 * age + 0.1074 * height + 0.3362 * weight if sex == "female" else -2.097 + 0.1069 * height + 0.2466 * weight},
     rand=lambda rng: {"age": _u(rng, 18, 90, 0), "height": _u(rng, 145, 200, 0), "weight": _u(rng, 40, 130, 1), "sex": rng.choice(["female", "male"])},
     phrases=["Watson total body water", "total body water"],
     examples=[_ex({"age": 40, "height": 175, "weight": 70, "sex": "male"}, {"TBW": 40.9696}), _ex({"age": 40, "height": 165, "weight": 60, "sex": "female"}, {"TBW": 30.3375})],
     extra_q=[_bad({"age": 10, "height": 140, "weight": 35, "sex": "male"})], level="advanced", weak="the published regression re-solved with sympy")


def solution_osmolarity(conc, molar_mass, i, phi=1.0):
    conc, M, i, phi = _pos(conc, "conc"), _pos(molar_mass, "molar_mass"), _int_ok(i, "i"), _pos(phi, "phi")
    m = conc / M * 1000.0
    return _rnd({"molarity": m, "osmolarity": m * i * phi})


def _int_ok(v, name):
    v = int(v) if isinstance(v, (int, float)) and not isinstance(v, bool) and float(v) == int(v) else None
    _need(v is not None and 1 <= v <= 6, f"{name} must be a whole number of particles from 1 to 6")
    return v


_add("hp_solution_osmolarity", name="Osmolarity of a solution", topic="body fluids / osmolarity",
     purpose="Molarity from a mass concentration and the osmolarity (particles per formula unit, times the osmotic coefficient).",
     inputs=[_n("conc", "mass concentration in g/L"), _n("molar_mass", "molar mass in g/mol"), _int("i", "number of particles per formula unit (dissociation)", 1, 6),
             _n("phi", "osmotic coefficient (optional; ideal solution if omitted)", required=False)],
     outputs=[_o("molarity", "molarity in mmol/L"), _o("osmolarity", "osmolarity in mOsm/L")],
     primary="osmolarity", conditions=["conc, molar mass > 0", "1 <= i <= 6"], method="mOsm/L = 1000 c/M x i x phi",
     latex=r"\text{osmolarity} = \phi\, i\, \frac{c}{M}", run=solution_osmolarity,
     reference=lambda conc, molar_mass, i, phi=1.0: (lambda m: {"molarity": m, "osmolarity": _sx("o - m*i*f", "o", m=m, i=i, f=phi)})(_sx("m*M - 1000*c", "m", c=conc, M=molar_mass)),
     wrong=lambda conc, molar_mass, i, phi=1.0: {"molarity": conc / molar_mass * 1000, "osmolarity": conc / molar_mass * 1000 * phi},
     rand=lambda rng: {"conc": _u(rng, 1, 100, 1), "molar_mass": rng.choice([58.44, 180.156, 74.55, 110.98, 84.01]), "i": rng.choice([2, 3]), "phi": _u(rng, 0.85, 1.0, 2)},
     phrases=["solution osmolarity", "osmolarity of a solution"],
     examples=[_ex({"conc": 9, "molar_mass": 58.44, "i": 2}, {"molarity": 154.004106776, "osmolarity": 308.008213552}),
               _ex({"conc": 50, "molar_mass": 180.156, "i": 1}, {"osmolarity": 277.537245498})],
     extra_q=[_bad({"conc": 9, "molar_mass": 0, "i": 2})], level="beginner", weak="the definition re-solved with sympy")


def osmotic_pressure(osm, T=37.0):
    osm, T = _nonneg(osm, "osm"), _num(T, "T")
    _need(-20 < T < 100, "temperature must be between -20 and 100 degC")
    p = osm * R_GAS * (T + T0C)              # mOsm/L = mol/m^3
    return _rnd({"pi": p, "pi_mmHg": p / MMHG, "pi_atm": p / 101325.0})


def ref_osmotic(osm, T=37.0):
    from scipy import constants as C
    p = osm * C.R * (T + 273.15)
    return {"pi": p, "pi_mmHg": p / 133.322387415, "pi_atm": p / C.atm}     # conventional mmHg (scipy's mmHg is the torr)


_add("hp_osmotic_pressure", name="Osmotic pressure by van 't Hoff's law", topic="body fluids / osmolarity",
     purpose="pi = c R T for an osmolarity c, in Pa, mmHg and atm (about 19.3 mmHg per mOsm/L at 37 degC).",
     inputs=[_n("osm", "osmolarity (or osmolarity difference) in mOsm/L"), _n("T", "temperature in degC (optional; body temperature if omitted)", required=False)],
     outputs=[_o("pi", "osmotic pressure in Pa"), _o("pi_mmHg", "osmotic pressure in mmHg"), _o("pi_atm", "osmotic pressure in atm")],
     primary="pi_mmHg", conditions=["osm >= 0"], method="pi = c R T (ideal dilute solution)", latex=r"\pi = c R T",
     run=osmotic_pressure, reference=ref_osmotic,
     wrong=lambda osm, T=37.0: (lambda p: {"pi": p, "pi_mmHg": p / MMHG, "pi_atm": p / 101325})(osm * R_GAS * T),
     rand=lambda rng: {"osm": _u(rng, 1, 600, 0), "T": _u(rng, 20, 40, 1)},
     phrases=["osmotic pressure"],
     examples=[_ex({"osm": 300}, {"pi": 773619.174292, "pi_mmHg": 5802.62016974, "pi_atm": 7.63502762686}), _ex({"osm": 10, "T": 25}, {"pi_mmHg": 185.937041604})],
     extra_q=[_bad({"osm": -5})], level="intermediate", weak="the same law with scipy's constants")


def fluid_shift(TBW, Posm, V_added, C_added, ICF_fraction=2.0 / 3.0):
    TBW, Posm, V, C = _pos(TBW, "TBW"), _pos(Posm, "Posm"), _pos(V_added, "V_added"), _nonneg(C_added, "C_added")
    f = _fr(ICF_fraction, "ICF_fraction", True, True)
    icf0, ecf0 = f * TBW, (1.0 - f) * TBW
    osm = (TBW * Posm + V * C) / (TBW + V)
    icf = icf0 * Posm / osm
    return _rnd({"osm_final": osm, "ICF": icf, "ECF": TBW + V - icf})


def ref_fluid_shift(TBW, Posm, V_added, C_added, ICF_fraction=2.0 / 3.0):
    icf0, ecf0 = ICF_fraction * TBW, (1 - ICF_fraction) * TBW
    # osmotic equilibrium: ICF solute fixed, ECF solute + added solute, total water conserved; root in osmolality
    g = lambda o: icf0 * Posm / o + (ecf0 * Posm + V_added * C_added) / o - (TBW + V_added)
    o = _brentq(g, 1e-6, 1e6)
    icf = icf0 * Posm / o
    ecf = (ecf0 * Posm + V_added * C_added) / o
    return {"osm_final": o, "ICF": icf, "ECF": ecf}


_add("hp_fluid_shift", name="Two-compartment fluid shift after adding a solution (Darrow-Yannet)", topic="body fluids / osmolarity",
     purpose="New osmolality and intracellular and extracellular volumes at osmotic equilibrium after a volume of solution enters the ECF.",
     inputs=[_n("TBW", "initial total body water in L"), _n("Posm", "initial osmolality in mOsm/kg"), _n("V_added", "volume added to the ECF in L"),
             _n("C_added", "osmolarity of the added solution in mOsm/L"),
             _n("ICF_fraction", "initial intracellular fraction of total body water (optional; the textbook two thirds if omitted)", required=False, pct=True)],
     outputs=[_o("osm_final", "final osmolality in mOsm/kg"), _o("ICF", "final intracellular volume in L"), _o("ECF", "final extracellular volume in L")],
     primary="osm_final", conditions=["TBW, Posm, V_added > 0", "C_added >= 0", "solute stays in the ECF, water moves freely"],
     method="solute conservation in each compartment, equal osmolality, water conservation",
     latex=r"P_{osm}' = \frac{TBW\,P_{osm} + V C}{TBW + V},\quad ICF' = \frac{ICF\,P_{osm}}{P_{osm}'}", run=fluid_shift, reference=ref_fluid_shift,
     wrong=lambda TBW, Posm, V_added, C_added, ICF_fraction=2.0 / 3.0: {"osm_final": Posm, "ICF": ICF_fraction * TBW, "ECF": (1 - ICF_fraction) * TBW + V_added},
     rand=lambda rng: {"TBW": _u(rng, 25, 55, 1), "Posm": _u(rng, 270, 310, 0), "V_added": _u(rng, 0.5, 4, 2), "C_added": rng.choice([0, 154, 308, 513, 1026]),
                       "ICF_fraction": _u(rng, 0.6, 0.7, 2)},
     phrases=["fluid shift", "two-compartment fluid shift"],
     examples=[_ex({"TBW": 42, "Posm": 290, "V_added": 2, "C_added": 0}, {"osm_final": 276.818181818, "ICF": 29.3333333333, "ECF": 14.6666666667}),
               _ex({"TBW": 42, "Posm": 290, "V_added": 1, "C_added": 600}, {"osm_final": 297.209302326, "ICF": 27.3208137715, "ECF": 15.6791862285})],
     extra_q=[_bad({"TBW": 42, "Posm": 290, "V_added": 0, "C_added": 0})], level="expert")


def gibbs_donnan(A, c, T=37.0):
    A, c, T = _pos(A, "A"), _pos(c, "c"), _num(T, "T")
    _need(-20 < T < 100, "temperature must be between -20 and 100 degC")
    cl = (-A + math.sqrt(A * A + 4.0 * c * c)) / 2.0
    na = cl + A
    return _rnd({"Na_in": na, "Cl_in": cl, "r": na / c, "V_mV": -R_GAS * (T + T0C) / FARADAY * math.log(na / c) * 1000.0,
                 "osmotic_excess": na + cl - 2.0 * c})


def ref_donnan(A, c, T=37.0):
    cl = _brentq(lambda x: x * (x + A) - c * c, 0.0, c)                    # Na_in Cl_in = Na_out Cl_out
    na = cl + A                                                           # electroneutrality inside
    v = 8.314462618 * (T + 273.15) / (6.02214076e23 * 1.602176634e-19) * math.log(cl / c) * 1000   # chloride Nernst potential
    return {"Na_in": na, "Cl_in": cl, "r": c / cl, "V_mV": v, "osmotic_excess": na + cl - 2 * c}


_add("hp_gibbs_donnan", name="Gibbs-Donnan equilibrium with an impermeant anion", topic="body fluids / membrane equilibria",
     purpose="Equilibrium NaCl distribution across a membrane when one side holds impermeant anions (charge concentration A) facing a large bath of NaCl c: concentrations, Donnan ratio, potential and osmotic excess.",
     inputs=[_n("A", "impermeant anion charge concentration inside in mEq/L"), _n("c", "bath NaCl concentration in mmol/L"),
             _n("T", "temperature in degC (optional; body temperature if omitted)", required=False)],
     outputs=[_o("Na_in", "Na+ inside in mmol/L"), _o("Cl_in", "Cl- inside in mmol/L"), _o("r", "Donnan ratio Na_in/Na_out = Cl_out/Cl_in"),
              _o("V_mV", "membrane potential inside relative to the bath in mV"), _o("osmotic_excess", "extra diffusible ions inside in mmol/L")],
     primary="Na_in", conditions=["A, c > 0"], method="Na_in Cl_in = c^2 with Na_in = Cl_in + A (quadratic); V = -(RT/F) ln(Na_in/c)",
     latex=r"[Na^+]_i[Cl^-]_i = [Na^+]_o[Cl^-]_o,\quad [Na^+]_i = [Cl^-]_i + A", run=gibbs_donnan, reference=ref_donnan,
     wrong=lambda A, c, T=37.0: (lambda cl: {"Na_in": cl + A, "Cl_in": cl, "r": (cl + A) / c, "V_mV": 0.0, "osmotic_excess": A})(c - A / 2),
     rand=lambda rng: {"A": _u(rng, 2, 60, 1), "c": _u(rng, 50, 200, 0), "T": _u(rng, 20, 40, 1)},
     phrases=["Gibbs-Donnan equilibrium", "Donnan equilibrium"],
     examples=[_ex({"A": 20, "c": 150}, {"Na_in": 160.332963784, "Cl_in": 140.332963784, "r": 1.06888642522, "V_mV": -1.78046007183}),
               _ex({"A": 10, "c": 100, "T": 25}, {"Na_in": 105.124921973, "V_mV": -1.28409429524})],
     extra_q=[_bad({"A": 0, "c": 150})], level="expert")


# ================================================================================================ nerve and muscle
def _rtf(T):
    return R_GAS * (T + T0C) / FARADAY * 1000.0      # mV


def nernst(z, Cout, Cin, T=37.0):
    z = int(z)
    Cout, Cin, T = _pos(Cout, "Cout"), _pos(Cin, "Cin"), _num(T, "T")
    _need(z != 0 and -3 <= z <= 3, "the valence must be a nonzero whole number from -3 to 3")
    _need(-20 < T < 100, "temperature must be between -20 and 100 degC")
    return _rnd({"E": _rtf(T) / z * math.log(Cout / Cin), "RT_F": _rtf(T)})


def ref_nernst(z, Cout, Cin, T=37.0):
    # equilibrium: electrochemical potential equal on both sides, RT ln(Cin) + zF E = RT ln(Cout)
    from scipy import constants as C
    RT = C.R * (T + 273.15)
    F = C.physical_constants["Faraday constant"][0]
    e = _brentq(lambda E: RT * math.log(Cin) + int(z) * F * E - RT * math.log(Cout), -2.0, 2.0)
    return {"E": e * 1000.0, "RT_F": RT / F * 1000.0}


_add("hp_nernst_potential", name="Nernst equilibrium potential of an ion", topic="nerve and muscle / membrane potential",
     purpose="E = (RT/zF) ln(Cout/Cin) in mV for an ion of valence z.",
     inputs=[_int("z", "ion valence (charge number, negative for anions)", -3, 3), _n("Cout", "extracellular concentration in mM"),
             _n("Cin", "intracellular concentration in mM"), _n("T", "temperature in degC (optional; body temperature if omitted)", required=False)],
     outputs=[_o("E", "equilibrium potential in mV"), _o("RT_F", "RT/F in mV")],
     primary="E", conditions=["z a nonzero whole number", "concentrations > 0"], method="Nernst equation", latex=r"E = \frac{RT}{zF}\ln\frac{[C]_o}{[C]_i}",
     run=nernst, reference=ref_nernst,
     wrong=lambda z, Cout, Cin, T=37.0: {"E": _rtf(T) / int(z) * math.log(Cin / Cout), "RT_F": _rtf(T)},
     rand=lambda rng: {"z": rng.choice([1, 1, 2, -1]), "Cout": _u(rng, 1, 150, 1), "Cin": _u(rng, 1, 150, 1), "T": _u(rng, 20, 40, 1)},
     phrases=["Nernst potential", "equilibrium potential"],
     examples=[_ex({"z": 1, "Cout": 4, "Cin": 140}, {"E": -95.0225756641}), _ex({"z": 1, "Cout": 145, "Cin": 12}, {"E": 66.598213271}),
               _ex({"z": 2, "Cout": 2.5, "Cin": 0.0001}, {"E": 135.325508733})],
     extra_q=[_bad({"z": 0, "Cout": 4, "Cin": 140})], level="intermediate", weak="the equilibrium condition re-solved by brentq with scipy's constants")


def ghk_voltage(PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i, T=37.0):
    PK, PNa, PCl = _pos(PK, "PK"), _nonneg(PNa, "PNa"), _nonneg(PCl, "PCl")
    vals = [_pos(x, "concentrations") for x in (K_o, K_i, Na_o, Na_i, Cl_o, Cl_i)]
    K_o, K_i, Na_o, Na_i, Cl_o, Cl_i = vals
    T = _num(T, "T")
    _need(-20 < T < 100, "temperature must be between -20 and 100 degC")
    num = PK * K_o + PNa * Na_o + PCl * Cl_i
    den = PK * K_i + PNa * Na_i + PCl * Cl_o
    return _rnd({"Vm": _rtf(T) * math.log(num / den)})


def ref_ghk(PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i, T=37.0):
    # zero net GHK current of the three ions (constant-field flux equation)
    rtf = 8.314462618 * (T + 273.15) / (6.02214076e23 * 1.602176634e-19)

    def I(P, z, ci, co, V):
        # electric current density P z^2 (F V/RT)(ci - co e^{-u})/(1 - e^{-u}), u = z F V/RT (constant factors dropped)
        u = z * V / rtf
        if abs(u) < 1e-12:
            return P * z * (ci - co)
        return P * z * u * (ci - co * math.exp(-u)) / (-math.expm1(-u))
    tot = lambda V: I(PK, 1, K_i, K_o, V) + I(PNa, 1, Na_i, Na_o, V) + I(PCl, -1, Cl_i, Cl_o, V)
    return {"Vm": _brentq(tot, -0.3, 0.3) * 1000.0}


_add("hp_ghk_voltage", name="Resting membrane potential by the Goldman-Hodgkin-Katz equation", topic="nerve and muscle / membrane potential",
     purpose="Vm = (RT/F) ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o)) with relative permeabilities.",
     inputs=[_n("PK", "relative K+ permeability"), _n("PNa", "relative Na+ permeability"), _n("PCl", "relative Cl- permeability"),
             _n("K_o", "extracellular K+ in mM"), _n("K_i", "intracellular K+ in mM"), _n("Na_o", "extracellular Na+ in mM"),
             _n("Na_i", "intracellular Na+ in mM"), _n("Cl_o", "extracellular Cl- in mM"), _n("Cl_i", "intracellular Cl- in mM"),
             _n("T", "temperature in degC (optional; body temperature if omitted)", required=False)],
     outputs=[_o("Vm", "membrane potential in mV")],
     primary="Vm", conditions=["PK > 0", "PNa, PCl >= 0", "concentrations > 0"], method="GHK voltage equation (chloride terms inverted)",
     latex=r"V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}",
     run=ghk_voltage, reference=ref_ghk,
     wrong=lambda PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i, T=37.0: {"Vm": _rtf(T) * math.log((PK * K_o + PNa * Na_o + PCl * Cl_o) / (PK * K_i + PNa * Na_i + PCl * Cl_i))},
     rand=lambda rng: {"PK": 1, "PNa": _u(rng, 0.01, 0.2, 3), "PCl": _u(rng, 0.05, 1.0, 2), "K_o": _u(rng, 2, 10, 1), "K_i": _u(rng, 120, 160, 0),
                       "Na_o": _u(rng, 130, 150, 0), "Na_i": _u(rng, 5, 20, 1), "Cl_o": _u(rng, 100, 125, 0), "Cl_i": _u(rng, 4, 30, 1), "T": _u(rng, 20, 40, 1)},
     phrases=["GHK voltage", "Goldman-Hodgkin-Katz potential"],
     examples=[_ex({"PK": 1, "PNa": 0.04, "PCl": 0.45, "K_o": 4, "K_i": 140, "Na_o": 145, "Na_i": 12, "Cl_o": 116, "Cl_i": 4.2}, {"Vm": -74.8960368175}),
               _ex({"PK": 1, "PNa": 0.05, "PCl": 0, "K_o": 5, "K_i": 150, "Na_o": 145, "Na_i": 15, "Cl_o": 110, "Cl_i": 10}, {"Vm": -67.0865041475})],
     extra_q=[_bad({"PK": 0, "PNa": 0.04, "PCl": 0.45, "K_o": 4, "K_i": 140, "Na_o": 145, "Na_i": 12, "Cl_o": 116, "Cl_i": 4.2})], level="advanced")


def chord_conductance(gK, gNa, gCl, EK, ENa, ECl):
    gK, gNa, gCl = _nonneg(gK, "gK"), _nonneg(gNa, "gNa"), _nonneg(gCl, "gCl")
    EK, ENa, ECl = _num(EK, "EK"), _num(ENa, "ENa"), _num(ECl, "ECl")
    g = gK + gNa + gCl
    _need(g > 0, "at least one conductance must be positive")
    return _rnd({"Vm": (gK * EK + gNa * ENa + gCl * ECl) / g})


def ref_chord(gK, gNa, gCl, EK, ENa, ECl):
    lo, hi = min(EK, ENa, ECl) - 1.0, max(EK, ENa, ECl) + 1.0
    return {"Vm": _brentq(lambda V: gK * (V - EK) + gNa * (V - ENa) + gCl * (V - ECl), lo, hi)}


_add("hp_chord_conductance", name="Membrane potential by the chord conductance equation", topic="nerve and muscle / membrane potential",
     purpose="Steady potential at which the ohmic ionic currents cancel: Vm = (gK EK + gNa ENa + gCl ECl)/(gK + gNa + gCl).",
     inputs=[_n("gK", "K+ conductance (any unit)"), _n("gNa", "Na+ conductance (same unit)"), _n("gCl", "Cl- conductance (same unit)"),
             _n("EK", "K+ equilibrium potential in mV"), _n("ENa", "Na+ equilibrium potential in mV"), _n("ECl", "Cl- equilibrium potential in mV")],
     outputs=[_o("Vm", "membrane potential in mV")],
     primary="Vm", conditions=["conductances >= 0, not all zero"], method="conductance-weighted mean of the equilibrium potentials",
     latex=r"V_m = \frac{g_K E_K + g_{Na}E_{Na} + g_{Cl}E_{Cl}}{g_K + g_{Na} + g_{Cl}}", run=chord_conductance, reference=ref_chord,
     wrong=lambda gK, gNa, gCl, EK, ENa, ECl: {"Vm": (EK + ENa + ECl) / 3},
     rand=lambda rng: {"gK": _u(rng, 0.5, 20, 2), "gNa": _u(rng, 0.01, 5, 2), "gCl": _u(rng, 0, 10, 2), "EK": _u(rng, -100, -80, 0),
                       "ENa": _u(rng, 50, 70, 0), "ECl": _u(rng, -90, -60, 0)},
     phrases=["chord conductance", "chord conductance potential"],
     examples=[_ex({"gK": 1, "gNa": 0.05, "gCl": 0.3, "EK": -95, "ENa": 67, "ECl": -90}, {"Vm": -87.8888888889}),
               _ex({"gK": 10, "gNa": 1, "gCl": 0, "EK": -90, "ENa": 60, "ECl": -70}, {"Vm": -76.3636363636})],
     extra_q=[_bad({"gK": 0, "gNa": 0, "gCl": 0, "EK": -95, "ENa": 67, "ECl": -90})], level="intermediate")


def ionic_current(g, Vm, E):
    g, Vm, E = _nonneg(g, "g"), _num(Vm, "Vm"), _num(E, "E")
    return _rnd({"I": g * (Vm - E), "driving_force": Vm - E})


_add("hp_ionic_current", name="Ionic current and driving force", topic="nerve and muscle / membrane potential",
     purpose="Ohmic ionic current I = g (Vm - E) (positive = outward) and the driving force Vm - E.",
     inputs=[_n("g", "conductance in nS"), _n("Vm", "membrane potential in mV"), _n("E", "equilibrium potential of the ion in mV")],
     outputs=[_o("I", "current in pA (positive outward)"), _o("driving_force", "driving force in mV")],
     primary="I", conditions=["g >= 0"], method="I = g (Vm - E); nS x mV = pA", latex=r"I = g\,(V_m - E)",
     run=ionic_current, reference=lambda g, Vm, E: {"I": _sx("I - g*(V - e)", "I", g=g, V=Vm, e=E), "driving_force": _sx("d - V + e", "d", V=Vm, e=E)},
     wrong=lambda g, Vm, E: {"I": g * (E - Vm), "driving_force": E - Vm},
     rand=lambda rng: {"g": _u(rng, 0.5, 50, 1), "Vm": _u(rng, -90, 40, 0), "E": rng.choice([-95, -90, -70, 60, 67, 120])},
     phrases=["ionic current", "driving force"],
     examples=[_ex({"g": 10, "Vm": -70, "E": -95}, {"I": 250, "driving_force": 25}), _ex({"g": 2, "Vm": -70, "E": 60}, {"I": -260, "driving_force": -130})],
     extra_q=[_bad({"g": -1, "Vm": -70, "E": -95})], level="beginner", weak="the ohmic relation re-solved with sympy")


def membrane_time_constant(Rm, Cm, t=None):
    Rm, Cm = _pos(Rm, "Rm"), _pos(Cm, "Cm")
    tau = Rm * Cm * 1e-3                       # ohm cm^2 x uF/cm^2 = us; in ms
    out = {"tau": tau}
    if t is not None:
        out["fraction"] = -math.expm1(-_nonneg(t, "t") / tau)
    return _rnd(out)


def ref_membrane_tau(Rm, Cm, t=None):
    # RC membrane charged by a current step: C dV/dt = (V_inf - V)/R per unit area, in SI
    R, C = Rm * 1e-4, Cm * 1e-6 * 1e4       # ohm m^2, F/m^2
    out = {"tau": R * C * 1e3}
    if t is not None:
        out["fraction"] = _ivp(lambda s, y: [(1.0 - y[0]) / (R * C)], t * 1e-3, [0.0])[0] if t > 0 else 0.0
    return out


_add("hp_membrane_time_constant", name="Membrane time constant and charging", topic="nerve and muscle / cable properties",
     purpose="tau_m = Rm Cm (specific membrane resistance times capacitance) and the fraction of the final voltage reached after t.",
     inputs=[_n("Rm", "specific membrane resistance in ohm cm^2"), _n("Cm", "specific membrane capacitance in uF/cm^2"),
             _n("t", "time in ms (optional; needed for the fraction charged)", required=False)],
     outputs=[_o("tau", "membrane time constant in ms"), _o("fraction", "fraction of the final voltage at time t")],
     primary="tau", conditions=["Rm, Cm > 0", "t >= 0"], method="tau = Rm Cm; V(t)/V_inf = 1 - exp(-t/tau)", latex=r"\tau_m = R_m C_m",
     run=membrane_time_constant, reference=ref_membrane_tau,
     wrong=lambda Rm, Cm, t=None: (lambda tau: dict({"tau": tau}, **({"fraction": math.exp(-t / tau)} if t is not None else {})))(Rm * Cm * 1e-6),
     rand=lambda rng: _drop(rng, {"Rm": _u(rng, 1000, 50000, 0), "Cm": _u(rng, 0.5, 2, 2), "t": _u(rng, 0.5, 40, 1)}, "t"),
     phrases=["membrane time constant"],
     examples=[_ex({"Rm": 20000, "Cm": 1, "t": 10}, {"tau": 20, "fraction": 0.393469340287}), _ex({"Rm": 5000, "Cm": 2, "t": 5}, {"tau": 10, "fraction": 0.393469340287}),
               _ex({"Rm": 20000, "Cm": 1}, {"tau": 20})],
     extra_q=[_bad({"Rm": 0, "Cm": 1, "t": 10})], level="intermediate")


def length_constant(Rm, Ri, d, x=None):
    Rm, Ri, d = _pos(Rm, "Rm"), _pos(Ri, "Ri"), _pos(d, "d")
    lam = math.sqrt(Rm * (d * 1e-4) / (4.0 * Ri))        # cm
    out = {"lambda": lam * 10.0}
    if x is not None:
        out["attenuation"] = math.exp(-(_nonneg(x, "x") / 10.0) / lam)
    return _rnd(out)


def ref_length_constant(Rm, Ri, d, x=None):
    import numpy as np
    from scipy.linalg import expm
    dc = d * 1e-4
    rm = Rm / (math.pi * dc)                 # ohm cm (membrane resistance of unit length)
    ri = 4 * Ri / (math.pi * dc * dc)        # ohm/cm (axial resistance per unit length)
    lam = math.sqrt(rm / ri)
    # semi-infinite cable V'' = V/lambda^2 with the decaying solution: [V, V'] at x from [1, -1/lambda]
    if x is None:
        return {"lambda": lam * 10.0}
    M = np.array([[0.0, 1.0], [1.0 / lam ** 2, 0.0]])
    v = expm(M * (x / 10.0)) @ np.array([1.0, -1.0 / lam])
    return {"lambda": lam * 10.0, "attenuation": float(v[0])}


_add("hp_length_constant", name="Length constant of an axon and steady-state attenuation", topic="nerve and muscle / cable properties",
     purpose="lambda = sqrt(Rm d/(4 Ri)) of a passive fibre and the steady voltage fraction e^(-x/lambda) at distance x.",
     inputs=[_n("Rm", "specific membrane resistance in ohm cm^2"), _n("Ri", "axoplasm resistivity in ohm cm"), _n("d", "fibre diameter in um"),
             _n("x", "distance from the injection site in mm (optional; needed for the attenuation)", required=False)],
     outputs=[_o("lambda", "length constant in mm"), _o("attenuation", "steady-state fraction of the voltage remaining at distance x")],
     primary="lambda", conditions=["Rm, Ri, d > 0", "x >= 0", "semi-infinite passive cable"], method="lambda = sqrt(rm/ri)",
     latex=r"\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0 e^{-x/\lambda}", run=length_constant, reference=ref_length_constant,
     wrong=lambda Rm, Ri, d, x=None: (lambda lam: dict({"lambda": lam * 10}, **({"attenuation": math.exp(-(x / 10) / lam)} if x is not None else {})))(math.sqrt(Rm * d * 1e-4 / (2 * Ri))),
     # x within 3 length constants: beyond that the matrix-exponential reference amplifies rounding through the growing mode
     rand=lambda rng: _drop(rng, (lambda Rm, Ri, d: {"Rm": Rm, "Ri": Ri, "d": d, "x": round(rng.uniform(0, 3) * math.sqrt(Rm * d * 1e-4 / (4 * Ri)) * 10, 3)})(
         _u(rng, 1000, 50000, 0), _u(rng, 30, 300, 0), _u(rng, 0.5, 20, 1)), "x"),
     phrases=["length constant", "axon length constant"],
     examples=[_ex({"Rm": 20000, "Ri": 100, "d": 10, "x": 1}, {"lambda": 2.2360679775, "attenuation": 0.639407319162}),
               _ex({"Rm": 5000, "Ri": 200, "d": 2, "x": 0.5}, {"lambda": 0.353553390593, "attenuation": 0.243116734434}),
               _ex({"Rm": 20000, "Ri": 100, "d": 10}, {"lambda": 2.2360679775})],
     extra_q=[_bad({"Rm": 20000, "Ri": 0, "d": 10, "x": 1})], level="advanced")


def conduction_velocity_scaling(v1, d1, d2, fiber):
    v1, d1, d2 = _pos(v1, "v1"), _pos(d1, "d1"), _pos(d2, "d2")
    _need(fiber in ("myelinated", "unmyelinated"), "fiber must be myelinated or unmyelinated")
    k = d2 / d1 if fiber == "myelinated" else math.sqrt(d2 / d1)
    return _rnd({"v2": v1 * k, "ratio": k})


_add("hp_conduction_velocity_scaling", name="Conduction velocity scaling with fibre diameter", topic="nerve and muscle / conduction",
     purpose="Conduction velocity is proportional to diameter for myelinated fibres and to its square root for unmyelinated fibres.",
     inputs=[_n("v1", "known conduction velocity in m/s"), _n("d1", "diameter of that fibre in um"), _n("d2", "diameter of the other fibre in um"),
             _ch("fiber", ["myelinated", "unmyelinated"], "fibre type")],
     outputs=[_o("v2", "conduction velocity of the other fibre in m/s"), _o("ratio", "ratio of the new to the known velocity")],
     primary="v2", conditions=["all inputs > 0"], method="v ~ d (myelinated), v ~ sqrt(d) (unmyelinated, cable theory)",
     latex=r"v \propto d \;(\text{myelinated}),\quad v \propto \sqrt{d}\;(\text{unmyelinated})", run=conduction_velocity_scaling,
     reference=lambda v1, d1, d2, fiber: (lambda k: {"v2": v1 * k, "ratio": k})(_sx("k - D/d", "k", D=d2, d=d1) if fiber == "myelinated" else math.sqrt(_sx("k - D/d", "k", D=d2, d=d1))),
     wrong=lambda v1, d1, d2, fiber: (lambda k: {"v2": v1 * k, "ratio": k})(math.sqrt(d2 / d1) if fiber == "myelinated" else d2 / d1),
     rand=lambda rng: {"v1": _u(rng, 0.5, 100, 1), "d1": _u(rng, 0.2, 20, 1), "d2": _u(rng, 0.2, 20, 1), "fiber": rng.choice(["myelinated", "unmyelinated"])},
     phrases=["conduction velocity scaling"],
     examples=[_ex({"v1": 60, "d1": 10, "d2": 5, "fiber": "myelinated"}, {"v2": 30, "ratio": 0.5}), _ex({"v1": 1, "d1": 1, "d2": 4, "fiber": "unmyelinated"}, {"v2": 2})],
     extra_q=[_bad({"v1": 60, "d1": 0, "d2": 5, "fiber": "myelinated"})], level="intermediate", weak="the scaling laws restated")


def nerve_conduction_velocity(distance, latency_proximal, latency_distal):
    D, lp, ld = _pos(distance, "distance"), _pos(latency_proximal, "latency_proximal"), _nonneg(latency_distal, "latency_distal")
    _need(lp > ld, "the proximal latency must exceed the distal latency")
    return _rnd({"NCV": D / (lp - ld)})


_add("hp_nerve_conduction_velocity", name="Nerve conduction velocity from two stimulation sites", topic="nerve and muscle / conduction",
     purpose="NCV = distance between stimulation sites / difference of the response latencies (mm/ms = m/s).",
     inputs=[_n("distance", "distance between stimulation sites in mm"), _n("latency_proximal", "latency from the proximal site in ms"),
             _n("latency_distal", "latency from the distal site in ms")],
     outputs=[_o("NCV", "conduction velocity in m/s")],
     primary="NCV", conditions=["latency_proximal > latency_distal"], method="NCV = d / (t_p - t_d)", latex=r"NCV = \frac{d}{t_p - t_d}",
     run=nerve_conduction_velocity, reference=lambda distance, latency_proximal, latency_distal: {"NCV": _sx("v*(p - q) - d", "v", d=distance, p=latency_proximal, q=latency_distal)},
     wrong=lambda distance, latency_proximal, latency_distal: {"NCV": distance / latency_proximal},
     rand=lambda rng: (lambda d: {"distance": _u(rng, 100, 400, 0), "latency_distal": d, "latency_proximal": round(d + rng.uniform(2, 10), 1)})(_u(rng, 2, 6, 1)),
     phrases=["nerve conduction velocity"],
     examples=[_ex({"distance": 250, "latency_proximal": 8.5, "latency_distal": 3.5}, {"NCV": 50}), _ex({"distance": 300, "latency_proximal": 9, "latency_distal": 4}, {"NCV": 60})],
     extra_q=[_bad({"distance": 250, "latency_proximal": 3.5, "latency_distal": 8.5})], level="beginner", weak="the definition re-solved with sympy")


def membrane_charge(Cm, dV, area):
    Cm, dV, area = _pos(Cm, "Cm"), _num(dV, "dV"), _pos(area, "area")
    _need(dV != 0, "the voltage change must not be zero")
    q = Cm * 1e-6 * (area * 1e-8) * (dV * 1e-3)
    return _rnd({"Q_pC": q * 1e12, "ions": abs(q) / E_CHARGE})


def ref_membrane_charge(Cm, dV, area):
    from scipy import constants as C
    q = _quad(lambda v: Cm * 1e-6 * area * 1e-8, 0.0, dV * 1e-3) if dV > 0 else -_quad(lambda v: Cm * 1e-6 * area * 1e-8, 0.0, -dV * 1e-3)
    return {"Q_pC": q * 1e12, "ions": abs(q) / C.e}


_add("hp_membrane_charge", name="Charge and number of ions to change the membrane potential", topic="nerve and muscle / membrane potential",
     purpose="Q = Cm A dV and the number of monovalent ions it represents (why few ions cross in an action potential).",
     inputs=[_n("Cm", "specific membrane capacitance in uF/cm^2"), _n("dV", "voltage change in mV"), _n("area", "membrane area in um^2")],
     outputs=[_o("Q_pC", "charge in pC"), _o("ions", "number of monovalent ions")],
     primary="ions", conditions=["Cm, area > 0", "dV not 0"], method="Q = C A dV; ions = |Q|/e", latex=r"Q = C_m A \Delta V,\quad N = \frac{|Q|}{e}",
     run=membrane_charge, reference=ref_membrane_charge,
     wrong=lambda Cm, dV, area: (lambda q: {"Q_pC": q * 1e12, "ions": abs(q) / FARADAY})(Cm * 1e-6 * area * 1e-8 * dV * 1e-3),
     rand=lambda rng: {"Cm": _u(rng, 0.5, 2, 2), "dV": _u(rng, 10, 130, 0), "area": _u(rng, 50, 20000, 0)},
     phrases=["membrane charge", "ions to change the membrane potential"],
     examples=[_ex({"Cm": 1, "dV": 100, "area": 1000}, {"Q_pC": 1, "ions": 6241509.07446}), _ex({"Cm": 0.9, "dV": 80, "area": 500}, {"Q_pC": 0.36})],
     extra_q=[_bad({"Cm": 1, "dV": 100, "area": 0})], level="intermediate")


def strength_duration(rheobase, chronaxie, t):
    I0, c, t = _pos(rheobase, "rheobase"), _pos(chronaxie, "chronaxie"), _pos(t, "t")
    return _rnd({"I_lapicque": I0 / (-math.expm1(-t * math.log(2.0) / c)), "I_weiss": I0 * (1.0 + c / t)})


def ref_strength_duration(rheobase, chronaxie, t):
    # RC membrane, tau = chronaxie/ln 2: a pulse I of width t is at threshold when V(t) = rheobase x R (R = 1)
    tau = chronaxie / math.log(2.0)
    vt = lambda I: _ivp(lambda s, y: [(I - y[0]) / tau], t, [0.0])[0]
    I = _brentq(lambda I: vt(I) - rheobase, rheobase, rheobase * (1 + 50 * tau / t) * 2)
    w = _sx("I - r*(1 + c/t)", "I", r=rheobase, c=chronaxie, t=t)
    return {"I_lapicque": I, "I_weiss": w}


_add("hp_strength_duration", name="Threshold current for a pulse width (strength-duration curve)", topic="nerve and muscle / excitability",
     purpose="Threshold current for a pulse of width t from rheobase and chronaxie: Lapicque (exponential RC) and Weiss (linear charge) forms.",
     inputs=[_n("rheobase", "rheobase current (any unit, e.g. mA)"), _n("chronaxie", "chronaxie in ms"), _n("t", "pulse width in ms")],
     outputs=[_o("I_lapicque", "threshold current by the Lapicque (exponential) law, in the unit of the rheobase"),
              _o("I_weiss", "threshold current by the Weiss (linear) law, in the unit of the rheobase")],
     primary="I_lapicque", conditions=["all inputs > 0"], method="Lapicque: tau = chronaxie/ln 2; Weiss: linear charge law",
     latex=r"I = \frac{I_{rh}}{1 - 2^{-t/c}}\;(\text{Lapicque}),\quad I = I_{rh}\left(1 + \frac{c}{t}\right)\;(\text{Weiss})",
     run=strength_duration, reference=ref_strength_duration,
     wrong=lambda rheobase, chronaxie, t: {"I_lapicque": rheobase / (1 - math.exp(-t / chronaxie)), "I_weiss": rheobase * (1 + t / chronaxie)},
     rand=lambda rng: {"rheobase": _u(rng, 0.1, 5, 2), "chronaxie": _u(rng, 0.1, 2, 2), "t": _u(rng, 0.05, 3, 2)},
     phrases=["strength-duration threshold", "strength-duration curve"],
     examples=[_ex({"rheobase": 1, "chronaxie": 0.5, "t": 0.5}, {"I_lapicque": 2, "I_weiss": 2}), _ex({"rheobase": 1, "chronaxie": 0.5, "t": 0.1}, {"I_lapicque": 7.72502395887, "I_weiss": 6}),
               _ex({"rheobase": 0.4, "chronaxie": 0.3, "t": 1}, {"I_lapicque": 0.444055927947, "I_weiss": 0.52})],
     extra_q=[_bad({"rheobase": 1, "chronaxie": 0, "t": 0.5})], level="advanced")


def hill_force_velocity(F0, a, b, v):
    F0, a, b, v = _pos(F0, "F0"), _pos(a, "a"), _pos(b, "b"), _nonneg(v, "v")
    vmax = b * F0 / a
    _need(v <= vmax, "the shortening velocity cannot exceed vmax = b F0/a")
    F = (F0 * b - a * v) / (v + b)
    return _rnd({"F": F, "power": F * v, "vmax": vmax})


_add("hp_hill_force_velocity", name="Hill muscle force-velocity relation", topic="nerve and muscle / muscle mechanics",
     purpose="Hill's equation (F + a)(v + b) = (F0 + a) b: force at shortening velocity v, the power F v and vmax = b F0/a.",
     inputs=[_n("F0", "isometric force in N"), _n("a", "Hill constant a in N"), _n("b", "Hill constant b in m/s"), _n("v", "shortening velocity in m/s")],
     outputs=[_o("F", "force in N"), _o("power", "power in W"), _o("vmax", "maximal shortening velocity in m/s")],
     primary="F", conditions=["F0, a, b > 0", "0 <= v <= vmax"], method="F = (F0 b - a v)/(v + b)", latex=r"(F + a)(v + b) = (F_0 + a)\,b",
     run=hill_force_velocity,
     reference=lambda F0, a, b, v: (lambda F: {"F": F, "power": F * v, "vmax": _sx("(a)*(w + b) - (F0 + a)*b", "w", F0=F0, a=a, b=b)})(_sx("(F + a)*(v + b) - (F0 + a)*b", "F", F0=F0, a=a, b=b, v=v)),
     wrong=lambda F0, a, b, v: (lambda F: {"F": F, "power": F * v, "vmax": a * F0 / b})((F0 * b + a * v) / (v + b)),
     rand=lambda rng: (lambda F0, r, b: {"F0": F0, "a": round(r * F0, 1), "b": b, "v": round(rng.uniform(0.05, 0.85) * b / r, 3)})(_u(rng, 20, 2000, 0), rng.uniform(0.15, 0.4), _u(rng, 0.1, 2, 2)),
     phrases=["Hill force-velocity", "muscle force-velocity"],
     examples=[_ex({"F0": 100, "a": 25, "b": 0.3, "v": 0.1}, {"F": 68.75, "power": 6.875, "vmax": 1.2}), _ex({"F0": 400, "a": 80, "b": 0.5, "v": 0.3}, {"F": 220, "power": 66, "vmax": 2.5})],
     extra_q=[_bad({"F0": 100, "a": 25, "b": 0.3, "v": 5})], level="advanced", weak="Hill's equation re-solved with sympy")


def muscle_max_power(F0, a, b):
    F0, a, b = _pos(F0, "F0"), _pos(a, "a"), _pos(b, "b")
    v = b * (math.sqrt(1.0 + F0 / a) - 1.0)
    F = (F0 * b - a * v) / (v + b)
    return _rnd({"v_opt": v, "F_opt": F, "P_max": F * v, "vmax": b * F0 / a})


def ref_muscle_max_power(F0, a, b):
    import sympy as sp
    w = sp.Symbol("w")
    P = w * (F0 * b - a * w) / (w + b)
    dP = sp.lambdify(w, sp.diff(P, w), "math")
    vmax = b * F0 / a
    v = _brentq(dP, 1e-12 * vmax, vmax * (1 - 1e-12))
    F = (F0 * b - a * v) / (v + b)
    return {"v_opt": v, "F_opt": F, "P_max": F * v, "vmax": vmax}


_add("hp_muscle_max_power", name="Maximal power of a muscle from Hill's equation", topic="nerve and muscle / muscle mechanics",
     purpose="The shortening velocity of maximal power v_opt = b (sqrt(1 + F0/a) - 1), the force there and the maximal power.",
     inputs=[_n("F0", "isometric force in N"), _n("a", "Hill constant a in N"), _n("b", "Hill constant b in m/s")],
     outputs=[_o("v_opt", "velocity of maximal power in m/s"), _o("F_opt", "force at maximal power in N"), _o("P_max", "maximal power in W"),
              _o("vmax", "maximal shortening velocity in m/s")],
     primary="P_max", conditions=["F0, a, b > 0"], method="dP/dv = 0 for P = v (F0 b - a v)/(v + b)",
     latex=r"v_{opt} = b\left(\sqrt{1 + F_0/a} - 1\right)", run=muscle_max_power, reference=ref_muscle_max_power,
     wrong=lambda F0, a, b: (lambda v: {"v_opt": v, "F_opt": F0 / 2, "P_max": v * F0 / 2, "vmax": b * F0 / a})(b * F0 / a / 2),
     rand=lambda rng: (lambda F0, r: {"F0": F0, "a": round(r * F0, 1), "b": _u(rng, 0.1, 2, 2)})(_u(rng, 20, 2000, 0), rng.uniform(0.15, 0.4)),
     phrases=["muscle maximal power", "maximal muscle power"],
     examples=[_ex({"F0": 100, "a": 25, "b": 0.3}, {"v_opt": 0.37082039325, "F_opt": 30.9016994375, "P_max": 11.4589803375, "vmax": 1.2}),
               _ex({"F0": 400, "a": 80, "b": 0.5}, {"v_opt": 0.724744871392, "P_max": 84.0408205773})],
     extra_q=[_bad({"F0": 100, "a": 0, "b": 0.3})], level="expert")


# ================================================================================================ body composition and metabolism
def bmi(weight, height):
    weight, height = _pos(weight, "weight"), _pos(height, "height")
    return _rnd({"BMI": weight / (height / 100.0) ** 2})


_add("hp_bmi", name="Body mass index", topic="body composition / anthropometry",
     purpose="BMI = weight/height^2 (kg/m^2).",
     inputs=[_n("weight", "weight in kg"), _n("height", "height in cm")], outputs=[_o("BMI", "body mass index in kg/m^2")],
     primary="BMI", conditions=["weight, height > 0"], method="BMI = W / H^2 (H in m)", latex=r"BMI = \frac{W}{H^2}",
     run=bmi, reference=lambda weight, height: {"BMI": _sx("B*h**2 - w", "B", w=weight, h=height / 100)},
     wrong=lambda weight, height: {"BMI": weight / (height / 100)},
     rand=lambda rng: {"weight": _u(rng, 30, 160, 1), "height": _u(rng, 140, 205, 0)},
     phrases=["body mass index", "BMI"],
     examples=[_ex({"weight": 70, "height": 175}, {"BMI": 22.8571428571}), _ex({"weight": 95, "height": 182}, {"BMI": 28.6801110977})],
     extra_q=[_bad({"weight": 70, "height": 0})], level="beginner", weak="the definition re-solved with sympy")


def body_surface_area(weight, height):
    W, H = _pos(weight, "weight"), _pos(height, "height")
    return _rnd({"BSA_DuBois": 0.007184 * W ** 0.425 * H ** 0.725, "BSA_Mosteller": math.sqrt(H * W / 3600.0),
                 "BSA_Haycock": 0.024265 * W ** 0.5378 * H ** 0.3964, "BSA_GehanGeorge": 0.0235 * W ** 0.51456 * H ** 0.42246})


def ref_bsa(weight, height):
    import mpmath as mp
    mp.mp.dps = 30
    W, H = mp.mpf(repr(weight)), mp.mpf(repr(height))
    return {"BSA_DuBois": float(mp.mpf("0.007184") * mp.exp(mp.mpf("0.425") * mp.log(W) + mp.mpf("0.725") * mp.log(H))),
            "BSA_Mosteller": float(mp.sqrt(H * W / 3600)),
            "BSA_Haycock": float(mp.mpf("0.024265") * mp.exp(mp.mpf("0.5378") * mp.log(W) + mp.mpf("0.3964") * mp.log(H))),
            "BSA_GehanGeorge": float(mp.mpf("0.0235") * mp.exp(mp.mpf("0.51456") * mp.log(W) + mp.mpf("0.42246") * mp.log(H)))}


_add("hp_body_surface_area", name="Body surface area (DuBois, Mosteller, Haycock, Gehan-George)", topic="body composition / anthropometry",
     purpose="Body surface area by the DuBois (1916), Mosteller (1987), Haycock (1978) and Gehan-George (1970) formulas.",
     inputs=[_n("weight", "weight in kg"), _n("height", "height in cm")],
     outputs=[_o("BSA_DuBois", "DuBois BSA in m^2"), _o("BSA_Mosteller", "Mosteller BSA in m^2"), _o("BSA_Haycock", "Haycock BSA in m^2"),
              _o("BSA_GehanGeorge", "Gehan-George BSA in m^2")],
     primary="BSA_Mosteller", conditions=["weight, height > 0"], method="published power-law formulas",
     latex=r"BSA_{Mosteller} = \sqrt{\frac{H\,W}{3600}},\quad BSA_{DuBois} = 0.007184\,W^{0.425}H^{0.725}", run=body_surface_area, reference=ref_bsa,
     wrong=lambda weight, height: {"BSA_DuBois": 0.007184 * weight ** 0.725 * height ** 0.425, "BSA_Mosteller": math.sqrt(height * weight) / 3600,
                                   "BSA_Haycock": 0.024265 * weight ** 0.3964 * height ** 0.5378, "BSA_GehanGeorge": 0.0235 * weight ** 0.42246 * height ** 0.51456},
     rand=lambda rng: {"weight": _u(rng, 3, 150, 1), "height": _u(rng, 50, 205, 0)},
     phrases=["body surface area", "BSA"],
     examples=[_ex({"weight": 70, "height": 175}, {"BSA_DuBois": 1.84814301812, "BSA_Mosteller": 1.84466196843, "BSA_Haycock": 1.84677634904, "BSA_GehanGeorge": 1.85385329752}),
               _ex({"weight": 55, "height": 160}, {"BSA_DuBois": 1.56317447234, "BSA_Mosteller": 1.56347191994})],
     extra_q=[_bad({"weight": 0, "height": 175})], level="beginner", weak="the published formulas evaluated in 30-digit mpmath")


def bmr_harris_benedict(weight, height, age, sex, version):
    W, H, A = _pos(weight, "weight"), _pos(height, "height"), _num(age, "age")
    fem = _sex(sex)
    _need(version in ("original", "revised"), "version must be original (1919) or revised (1984)")
    _need(A >= 18, "the equations are for adults")
    if version == "original":
        v = 655.0955 + 9.5634 * W + 1.8496 * H - 4.6756 * A if fem else 66.473 + 13.7516 * W + 5.0033 * H - 6.755 * A
    else:
        v = 447.593 + 9.247 * W + 3.098 * H - 4.330 * A if fem else 88.362 + 13.397 * W + 4.799 * H - 5.677 * A
    return _rnd({"BMR": v, "BMR_kJ": v * 4.184})


_HB = {("original", True): ("6550955/10000", "95634/10000", "18496/10000", "46756/10000"),
       ("original", False): ("66473/1000", "137516/10000", "50033/10000", "6755/1000"),
       ("revised", True): ("447593/1000", "9247/1000", "3098/1000", "4330/1000"),
       ("revised", False): ("88362/1000", "13397/1000", "4799/1000", "5677/1000")}


def ref_hb(weight, height, age, sex, version):
    c = _HB[(version, sex == "female")]
    v = _sx(f"B - {c[0]} - {c[1]}*W - {c[2]}*H + {c[3]}*A", "B", W=weight, H=height, A=age)
    return {"BMR": v, "BMR_kJ": v * 4.184}


_add("hp_bmr_harris_benedict", name="Basal metabolic rate by the Harris-Benedict equation", topic="metabolism / energy expenditure",
     purpose="Harris-Benedict BMR, original (1919) or revised by Roza and Shizgal (1984), in kcal/day and kJ/day (computed value).",
     inputs=[_n("weight", "weight in kg"), _n("height", "height in cm"), _n("age", "age in years"), _ch("sex", ["female", "male"], "sex for the coefficients"),
             _ch("version", ["original", "revised"], "original equation or the revised (Roza-Shizgal) equation")],
     outputs=[_o("BMR", "basal metabolic rate in kcal/day"), _o("BMR_kJ", "basal metabolic rate in kJ/day")],
     primary="BMR", conditions=["adult", "weight, height > 0"], method="published linear coefficients",
     latex=r"BMR_{m} = 66.473 + 13.7516\,W + 5.0033\,H - 6.755\,A", run=bmr_harris_benedict, reference=ref_hb,
     wrong=lambda weight, height, age, sex, version: (lambda v: {"BMR": v, "BMR_kJ": v * 4.184})(66.473 + 13.7516 * weight + 5.0033 * height + 6.755 * age),
     rand=lambda rng: {"weight": _u(rng, 40, 140, 1), "height": _u(rng, 145, 205, 0), "age": _u(rng, 18, 85, 0), "sex": rng.choice(["female", "male"]),
                       "version": rng.choice(["original", "revised"])},
     phrases=["Harris-Benedict BMR", "Harris-Benedict"],
     examples=[_ex({"weight": 70, "height": 175, "age": 30, "sex": "male", "version": "original"}, {"BMR": 1702.0125}),
               _ex({"weight": 70, "height": 175, "age": 30, "sex": "male", "version": "revised"}, {"BMR": 1695.667}),
               _ex({"weight": 60, "height": 165, "age": 40, "sex": "female", "version": "original"}, {"BMR": 1347.0595}),
               _ex({"weight": 60, "height": 165, "age": 40, "sex": "female", "version": "revised"}, {"BMR": 1340.383})],
     extra_q=[_bad({"weight": 70, "height": 175, "age": 10, "sex": "male", "version": "original"})], level="intermediate",
     weak="the published coefficients re-solved with sympy")


def bmr_mifflin(weight, height, age, sex):
    W, H, A = _pos(weight, "weight"), _pos(height, "height"), _num(age, "age")
    fem = _sex(sex)
    _need(A >= 18, "the equation is for adults")
    v = 10.0 * W + 6.25 * H - 5.0 * A + (-161.0 if fem else 5.0)
    return _rnd({"REE": v, "REE_kJ": v * 4.184})


_add("hp_bmr_mifflin_st_jeor", name="Resting energy expenditure by the Mifflin-St Jeor equation", topic="metabolism / energy expenditure",
     purpose="Mifflin-St Jeor (1990): 10 W + 6.25 H - 5 A + 5 (male) or - 161 (female), kcal/day (computed value).",
     inputs=[_n("weight", "weight in kg"), _n("height", "height in cm"), _n("age", "age in years"), _ch("sex", ["female", "male"], "sex for the constant")],
     outputs=[_o("REE", "resting energy expenditure in kcal/day"), _o("REE_kJ", "resting energy expenditure in kJ/day")],
     primary="REE", conditions=["adult", "weight, height > 0"], method="Mifflin-St Jeor", latex=r"REE = 10W + 6.25H - 5A + s",
     run=bmr_mifflin,
     reference=lambda weight, height, age, sex: (lambda v: {"REE": v, "REE_kJ": v * 4.184})(_sx("R - 10*W - 25*H/4 + 5*A - s", "R", W=weight, H=height, A=age, s=-161 if sex == "female" else 5)),
     wrong=lambda weight, height, age, sex: (lambda v: {"REE": v, "REE_kJ": v * 4.184})(10 * weight + 6.25 * height - 5 * age + (5 if sex == "female" else -161)),
     rand=lambda rng: {"weight": _u(rng, 40, 140, 1), "height": _u(rng, 145, 205, 0), "age": _u(rng, 18, 85, 0), "sex": rng.choice(["female", "male"])},
     phrases=["Mifflin-St Jeor", "Mifflin-St Jeor REE"],
     examples=[_ex({"weight": 70, "height": 175, "age": 30, "sex": "male"}, {"REE": 1648.75}), _ex({"weight": 60, "height": 165, "age": 40, "sex": "female"}, {"REE": 1270.25})],
     extra_q=[_bad({"weight": 0, "height": 175, "age": 30, "sex": "male"})], level="intermediate", weak="the published coefficients re-solved with sympy")


def bmr_katch_mcardle(weight, body_fat):
    W, f = _pos(weight, "weight"), _fr(body_fat, "body_fat", open_hi=True)
    lbm = W * (1.0 - f)
    return _rnd({"LBM": lbm, "BMR": 370.0 + 21.6 * lbm})


_add("hp_bmr_katch_mcardle", name="Basal metabolic rate by the Katch-McArdle equation", topic="metabolism / energy expenditure",
     purpose="Lean body mass W (1 - body fat) and BMR = 370 + 21.6 LBM in kcal/day (computed value).",
     inputs=[_n("weight", "weight in kg"), _n("body_fat", "body fat fraction", pct=True)],
     outputs=[_o("LBM", "lean body mass in kg"), _o("BMR", "basal metabolic rate in kcal/day")],
     primary="BMR", conditions=["weight > 0", "0 <= body fat < 1"], method="Katch-McArdle", latex=r"BMR = 370 + 21.6\,LBM",
     run=bmr_katch_mcardle,
     reference=lambda weight, body_fat: (lambda l: {"LBM": l, "BMR": _sx("B - 370 - 108*L/5", "B", L=l)})(_sx("L - W*(1 - f)", "L", W=weight, f=body_fat)),
     wrong=lambda weight, body_fat: {"LBM": weight * body_fat, "BMR": 370 + 21.6 * weight * body_fat},
     rand=lambda rng: {"weight": _u(rng, 40, 140, 1), "body_fat": _u(rng, 0.06, 0.45, 2)},
     phrases=["Katch-McArdle BMR", "Katch-McArdle"],
     examples=[_ex({"weight": 70, "body_fat": 0.2}, {"LBM": 56, "BMR": 1579.6}), _ex({"weight": 80, "body_fat": 0.25}, {"LBM": 60, "BMR": 1666})],
     extra_q=[_bad({"weight": 70, "body_fat": 1})], level="intermediate", weak="the published coefficients re-solved with sympy")


def met_energy(MET, weight, duration):
    MET, W, m = _pos(MET, "MET"), _pos(weight, "weight"), _pos(duration, "duration")
    vo2 = MET * 3.5 * W
    return _rnd({"VO2": vo2, "kcal_per_min": vo2 / 200.0, "kcal": vo2 / 200.0 * m})


_add("hp_met_energy", name="Energy expenditure of an activity from METs", topic="metabolism / energy expenditure",
     purpose="1 MET = 3.5 mL O2/kg/min; oxygen uptake MET x 3.5 x W and energy MET x 3.5 x W/200 kcal/min.",
     inputs=[_n("MET", "activity intensity in METs"), _n("weight", "weight in kg"), _n("duration", "duration in min")],
     outputs=[_o("VO2", "oxygen uptake in mL/min"), _o("kcal_per_min", "energy in kcal/min"), _o("kcal", "total energy in kcal")],
     primary="kcal", conditions=["all inputs > 0"], method="kcal/min = MET x 3.5 x W / 200 (about 5 kcal per L O2)",
     latex=r"\text{kcal/min} = \frac{MET \times 3.5 \times W}{200}", run=met_energy,
     reference=lambda MET, weight, duration: (lambda v: {"VO2": v, "kcal_per_min": v / 1000 * 5, "kcal": _sx("k - v*m/200", "k", v=v, m=duration)})(_sx("V - M*7*W/2", "V", M=MET, W=weight)),
     wrong=lambda MET, weight, duration: (lambda v: {"VO2": v, "kcal_per_min": v / 200, "kcal": v / 200 * duration})(MET * 3.5 * weight / 10),
     rand=lambda rng: {"MET": _u(rng, 1, 16, 1), "weight": _u(rng, 40, 140, 1), "duration": _u(rng, 5, 180, 0)},
     phrases=["MET energy expenditure", "energy expenditure from METs"],
     examples=[_ex({"MET": 8, "weight": 70, "duration": 30}, {"VO2": 1960, "kcal_per_min": 9.8, "kcal": 294}),
               _ex({"MET": 3.5, "weight": 60, "duration": 60}, {"VO2": 735, "kcal": 220.5})],
     extra_q=[_bad({"MET": 0, "weight": 70, "duration": 30})], level="beginner", weak="the convention re-solved with sympy")


def atwater_energy(carbohydrate, fat, protein, alcohol=0.0):
    c, f, p, a = _nonneg(carbohydrate, "carbohydrate"), _nonneg(fat, "fat"), _nonneg(protein, "protein"), _nonneg(alcohol, "alcohol")
    k = 4.0 * c + 9.0 * f + 4.0 * p + 7.0 * a
    _need(k > 0, "at least one amount must be positive")
    return _rnd({"kcal": k, "kJ": k * 4.184})


_add("hp_atwater_energy", name="Food energy by the Atwater factors", topic="metabolism / energy balance",
     purpose="Energy from macronutrients with the general Atwater factors 4 (carbohydrate), 9 (fat), 4 (protein) and 7 (alcohol) kcal/g.",
     inputs=[_n("carbohydrate", "carbohydrate in g"), _n("fat", "fat in g"), _n("protein", "protein in g"), _n("alcohol", "alcohol in g (optional; none if omitted)", required=False)],
     outputs=[_o("kcal", "energy in kcal"), _o("kJ", "energy in kJ")],
     primary="kcal", conditions=["amounts >= 0, not all zero"], method="4/9/4/7 kcal per g; 1 kcal = 4.184 kJ", latex=r"E = 4C + 9F + 4P + 7A",
     run=atwater_energy,
     reference=lambda carbohydrate, fat, protein, alcohol=0.0: (lambda k: {"kcal": k, "kJ": k * 4.184})(_sx("E - 4*c - 9*f - 4*p - 7*a", "E", c=carbohydrate, f=fat, p=protein, a=alcohol)),
     wrong=lambda carbohydrate, fat, protein, alcohol=0.0: (lambda k: {"kcal": k, "kJ": k * 4.184})(4 * carbohydrate + 4 * fat + 9 * protein + 7 * alcohol),
     rand=lambda rng: {"carbohydrate": _u(rng, 0, 400, 0), "fat": _u(rng, 1, 120, 0), "protein": _u(rng, 0, 150, 0), "alcohol": _u(rng, 0, 40, 0)},
     phrases=["Atwater energy", "food energy"],
     examples=[_ex({"carbohydrate": 300, "fat": 70, "protein": 80}, {"kcal": 2150, "kJ": 8995.6}), _ex({"carbohydrate": 300, "fat": 70, "protein": 80, "alcohol": 20}, {"kcal": 2290})],
     extra_q=[_bad({"carbohydrate": 0, "fat": 0, "protein": 0})], level="beginner", weak="the factors re-solved with sympy")


def body_fat_density(density):
    D = _num(density, "density")
    # above 1.1 g/mL both equations give a negative body fat (Siri is 0 at 1.1): no human body (review 2026-11-14)
    _need(0.9 < D <= 1.1, "body density must be above 0.9 and at most 1.1 g/mL (the two-compartment equations give 0-100 % fat there)")
    return _rnd({"fat_Siri": 495.0 / D - 450.0, "fat_Brozek": 457.0 / D - 414.2})


_add("hp_body_fat_from_density", name="Body fat from body density (Siri and Brozek)", topic="body composition / densitometry",
     purpose="Percent body fat by Siri (495/D - 450) and Brozek (457/D - 414.2) from whole-body density.",
     inputs=[_n("density", "body density in g/mL")],
     outputs=[_o("fat_Siri", "body fat in % (Siri)"), _o("fat_Brozek", "body fat in % (Brozek)")],
     primary="fat_Siri", conditions=["0.9 < density <= 1.1"], method="two-compartment models", latex=r"\%BF = \frac{495}{D} - 450",
     run=body_fat_density,
     reference=lambda density: {"fat_Siri": _sx("p - 495/D + 450", "p", D=density), "fat_Brozek": _sx("p - 457/D + 4142/10", "p", D=density)},
     wrong=lambda density: {"fat_Siri": 450 / density - 495, "fat_Brozek": 414.2 / density - 457},
     rand=lambda rng: {"density": _u(rng, 1.0, 1.09, 4)},
     phrases=["body fat from density", "Siri body fat"],
     examples=[_ex({"density": 1.05}, {"fat_Siri": 21.4285714286, "fat_Brozek": 21.0380952381}), _ex({"density": 1.07}, {"fat_Siri": 12.6168224299, "fat_Brozek": 12.9028037383})],
     extra_q=[_bad({"density": 2})], level="intermediate", weak="the published equations re-solved with sympy")


def hydrostatic_weighing(mass_air, mass_water, water_density, RV, GI=0.1):
    Ma, Mw, rho, RV, GI = _pos(mass_air, "mass_air"), _num(mass_water, "mass_water"), _pos(water_density, "water_density"), _nonneg(RV, "RV"), _nonneg(GI, "GI")
    _need(Mw < Ma and 0.95 < rho <= 1.0, "underwater mass below the mass in air; water density 0.95-1.0 g/mL")
    V = (Ma - Mw) / rho - RV - GI
    _need(V > 0, "the computed body volume is not positive")
    D = Ma / V
    _need(0.9 < D <= 1.1, "the computed density is outside 0.9-1.1 g/mL (Siri gives a body fat outside 0-100 %)")
    return _rnd({"volume": V, "density": D, "fat_Siri": 495.0 / D - 450.0})


def ref_hydrostatic_weighing(mass_air, mass_water, water_density, RV, GI=0.1):
    # Archimedes: the buoyant force equals the weight of displaced water (body tissue plus lung and gut gas)
    Vd = _sx("rho*Vd - (Ma - Mw)", "Vd", rho=water_density, Ma=mass_air, Mw=mass_water)
    V = Vd - RV - GI
    D = mass_air / V
    return {"volume": V, "density": D, "fat_Siri": 495 / D - 450}


_add("hp_hydrostatic_weighing", name="Body density and fat by hydrostatic weighing", topic="body composition / densitometry",
     purpose="Body volume (Ma - Mw)/rho_w - RV - GI gas, body density and Siri percent fat.",
     inputs=[_n("mass_air", "body mass in air in kg"), _n("mass_water", "underwater mass in kg"), _n("water_density", "water density in g/mL"),
             _n("RV", "residual lung volume in L"), _n("GI", "gastrointestinal gas in L (optional; the customary estimate if omitted)", required=False)],
     outputs=[_o("volume", "body volume in L"), _o("density", "body density in g/mL"), _o("fat_Siri", "body fat in % (Siri)")],
     primary="fat_Siri", conditions=["mass_water < mass_air", "computed density 0.9-1.1"], method="Archimedes' principle, then Siri",
     latex=r"D_b = \frac{M_a}{\frac{M_a - M_w}{\rho_w} - RV - V_{GI}}", run=hydrostatic_weighing, reference=ref_hydrostatic_weighing,
     wrong=lambda mass_air, mass_water, water_density, RV, GI=0.1: (lambda V: {"volume": V, "density": mass_air / V, "fat_Siri": 495 / (mass_air / V) - 450})((mass_air - mass_water) / water_density),
     rand=lambda rng: (lambda Ma: {"mass_air": Ma, "mass_water": round(Ma * rng.uniform(0.025, 0.06), 2), "water_density": _u(rng, 0.993, 0.998, 4),
                                   "RV": _u(rng, 1.0, 1.8, 2), "GI": 0.1})(_u(rng, 50, 110, 1)),
     phrases=["hydrostatic weighing", "underwater weighing"],
     examples=[_ex({"mass_air": 70, "mass_water": 3, "water_density": 0.9957, "RV": 1.2}, {"volume": 65.98934418, "density": 1.06077732503, "fat_Siri": 16.6389338441}),
               _ex({"mass_air": 80, "mass_water": 3.5, "water_density": 0.997, "RV": 1.5}, {"volume": 75.1301905717, "fat_Siri": 14.8680541625})],
     extra_q=[_bad({"mass_air": 70, "mass_water": 80, "water_density": 0.9957, "RV": 1.2})], level="advanced", weak="Archimedes' balance re-solved with sympy")


def lean_body_mass_boer(weight, height, sex):
    W, H = _pos(weight, "weight"), _pos(height, "height")
    fem = _sex(sex)
    v = 0.252 * W + 0.473 * H - 48.3 if fem else 0.407 * W + 0.267 * H - 19.2
    _need(0 < v < W, "the formula gives no valid lean mass for these values")
    return _rnd({"LBM": v, "fat_mass": W - v})


_add("hp_lean_body_mass_boer", name="Lean body mass by the Boer formula", topic="body composition / anthropometry",
     purpose="Boer (1984): male 0.407 W + 0.267 H - 19.2, female 0.252 W + 0.473 H - 48.3 (kg), and the remaining fat mass.",
     inputs=[_n("weight", "weight in kg"), _n("height", "height in cm"), _ch("sex", ["female", "male"], "sex for the formula")],
     outputs=[_o("LBM", "lean body mass in kg"), _o("fat_mass", "weight - LBM in kg")],
     primary="LBM", conditions=["0 < LBM < weight"], method="Boer anthropometric formula", latex=r"LBM_m = 0.407\,W + 0.267\,H - 19.2",
     run=lean_body_mass_boer,
     reference=lambda weight, height, sex: (lambda l: {"LBM": l, "fat_mass": weight - l})(_sx("L - 252*W/1000 - 473*H/1000 + 483/10", "L", W=weight, H=height) if sex == "female"
                                                                                       else _sx("L - 407*W/1000 - 267*H/1000 + 192/10", "L", W=weight, H=height)),
     wrong=lambda weight, height, sex: (lambda l: {"LBM": l, "fat_mass": weight - l})(0.407 * weight + 0.267 * height - 19.2 if sex == "female" else 0.252 * weight + 0.473 * height - 48.3),
     rand=lambda rng: {"weight": _u(rng, 65, 130, 1), "height": _u(rng, 150, 195, 0), "sex": rng.choice(["female", "male"])},
     phrases=["Boer lean body mass", "lean body mass"],
     examples=[_ex({"weight": 70, "height": 175, "sex": "male"}, {"LBM": 56.015, "fat_mass": 13.985}), _ex({"weight": 60, "height": 165, "sex": "female"}, {"LBM": 44.865})],
     extra_q=[_bad({"weight": 0, "height": 175, "sex": "male"})], level="intermediate", weak="the published formula re-solved with sympy")


def heart_rate_reserve(age, HR_rest, intensity):
    A, r, f = _num(age, "age"), _pos(HR_rest, "HR_rest"), _fr(intensity, "intensity")
    _need(5 <= A <= 100, "age must be between 5 and 100")
    mx = 220.0 - A
    _need(r < mx, "the resting heart rate must be below the estimated maximum")
    return _rnd({"HRmax": mx, "HRmax_Tanaka": 208.0 - 0.7 * A, "HRR": mx - r, "target": r + f * (mx - r)})


_add("hp_heart_rate_reserve", name="Maximal heart rate, heart rate reserve and Karvonen target", topic="metabolism / exercise",
     purpose="Estimated maximal heart rate (220 - age; Tanaka 208 - 0.7 age), the reserve HRmax - HRrest and the Karvonen target HRrest + intensity x reserve.",
     inputs=[_n("age", "age in years"), _n("HR_rest", "resting heart rate in beats/min"), _n("intensity", "exercise intensity (fraction of reserve)", pct=True)],
     outputs=[_o("HRmax", "age-predicted maximal heart rate (Fox formula) in beats/min"),
              _o("HRmax_Tanaka", "age-predicted maximal heart rate (Tanaka formula) in beats/min"), _o("HRR", "heart rate reserve in beats/min"),
              _o("target", "Karvonen target heart rate in beats/min")],
     primary="target", conditions=["5 <= age <= 100", "HR_rest below HRmax"], method="Karvonen with HRmax = 220 - age",
     latex=r"HR_{target} = HR_{rest} + I\,(HR_{max} - HR_{rest})", run=heart_rate_reserve,
     reference=lambda age, HR_rest, intensity: (lambda m: {"HRmax": m, "HRmax_Tanaka": _sx("t - 208 + 7*a/10", "t", a=age), "HRR": m - HR_rest,
                                                           "target": _sx("T - r - f*(m - r)", "T", r=HR_rest, f=intensity, m=m)})(_sx("m - 220 + a", "m", a=age)),
     wrong=lambda age, HR_rest, intensity: {"HRmax": 220 - age, "HRmax_Tanaka": 208 - 0.7 * age, "HRR": 220 - age - HR_rest, "target": intensity * (220 - age)},
     rand=lambda rng: {"age": _u(rng, 15, 80, 0), "HR_rest": _u(rng, 45, 90, 0), "intensity": _u(rng, 0.4, 0.9, 2)},
     phrases=["Karvonen target heart rate", "heart rate reserve"],
     examples=[_ex({"age": 40, "HR_rest": 60, "intensity": 0.7}, {"HRmax": 180, "HRmax_Tanaka": 180, "HRR": 120, "target": 144}),
               _ex({"age": 55, "HR_rest": 65, "intensity": 0.6}, {"HRmax": 165, "HRmax_Tanaka": 169.5, "target": 125})],
     extra_q=[{"q": "Karvonen target heart rate: age = 40, HR_rest = 60, intensity = 150%", "outcome": "INVALID_INPUT"}], level="beginner", weak="the published rules re-solved with sympy")


def elimination_half_life(CL, Vd, t=None):
    CL, Vd = _pos(CL, "CL"), _pos(Vd, "Vd")
    k = CL / Vd
    out = {"k": k, "half_life": math.log(2.0) / k}
    if t is not None:
        out["fraction_remaining"] = math.exp(-k * _nonneg(t, "t"))
    return _rnd(out)


def ref_half_life(CL, Vd, t=None):
    # one-compartment first-order elimination: V dC/dt = -CL C
    th = _brentq(lambda s: _ivp(lambda u, y: [-CL * y[0] / Vd], s, [1.0])[0] - 0.5, 1e-6 * Vd / CL, 10 * Vd / CL)
    out = {"k": CL / Vd, "half_life": th}
    if t is not None:
        out["fraction_remaining"] = _ivp(lambda s, y: [-CL * y[0] / Vd], t, [1.0])[0] if t > 0 else 1.0
    return out


_add("hp_elimination_half_life", name="First-order elimination: rate constant and half-life from clearance and volume", topic="metabolism / clearance kinetics",
     purpose="Physiology of a one-compartment first-order process: k = CL/Vd, half-life ln 2/k and the fraction remaining after t (not a dosing recommendation).",
     inputs=[_n("CL", "clearance in L/h"), _n("Vd", "volume of distribution in L"), _n("t", "elapsed time in h (optional)", required=False)],
     outputs=[_o("k", "elimination rate constant per hour"), _o("half_life", "half-life in h"), _o("fraction_remaining", "fraction remaining after t")],
     primary="half_life", conditions=["CL, Vd > 0", "t >= 0"], method="k = CL/Vd; t1/2 = ln 2 / k; C/C0 = exp(-k t)",
     latex=r"t_{1/2} = \frac{\ln 2\, V_d}{CL}", run=elimination_half_life, reference=ref_half_life,
     wrong=lambda CL, Vd, t=None: (lambda k: dict({"k": k, "half_life": 0.5 / k}, **({"fraction_remaining": math.exp(-k * t)} if t is not None else {})))(Vd / CL),
     rand=lambda rng: _drop(rng, {"CL": _u(rng, 0.5, 20, 2), "Vd": _u(rng, 5, 300, 1), "t": _u(rng, 0.5, 48, 1)}, "t"),
     phrases=["elimination half-life", "half-life from clearance"],
     examples=[_ex({"CL": 5, "Vd": 50, "t": 12}, {"k": 0.1, "half_life": 6.9314718056, "fraction_remaining": 0.301194211912}),
               _ex({"CL": 3, "Vd": 40, "t": 24}, {"k": 0.075, "half_life": 9.24196240747, "fraction_remaining": 0.165298888222}),
               _ex({"CL": 5, "Vd": 50}, {"k": 0.1, "half_life": 6.9314718056})],
     extra_q=[_bad({"CL": 0, "Vd": 50, "t": 12})], level="intermediate")


# ================================================================================================ core gaps (2026-11-12)
# Blood: the red-cell indices. Heart: the heart rate read from an ECG (R-R interval or grid boxes). Conventions: Hct a
# fraction, Hb in g/dL, the red-cell count in 10^12 cells/L (= 10^6 cells/uL); MCV in fL, MCH in pg, MCHC in g/dL.
# ECG paper: 1 small box = 1 mm, 1 large box = 5 mm, standard speed 25 mm/s unless stated (then 300/large boxes and
# 1500/small boxes); exactly one of the R-R interval, the small-box count and the large-box count is given; heart
# rates outside 20-300 beats/min are refused as not a heart rhythm reading.
def red_cell_indices(Hct, Hb, RBC):
    Hct = _fr(Hct, "Hct", open_lo=True, open_hi=True)
    Hb, RBC = _pos(Hb, "Hb"), _pos(RBC, "RBC")
    return _rnd({"MCV": Hct * 1000.0 / RBC, "MCH": Hb * 10.0 / RBC, "MCHC": Hb / Hct})


def ref_red_cell_indices(Hct, Hb, RBC):
    # per cell in SI: one litre of blood holds RBC x 1e12 cells; volume per cell (L) and hemoglobin per cell (g),
    # MCHC as hemoglobin mass per cell over cell volume (not Hb/Hct)
    cells_per_L = RBC * 1e12
    vol_L = Hct / cells_per_L                       # litres of cells per cell
    hb_g = Hb * 10.0 / cells_per_L                  # g/dL -> g/L, per cell
    MCV = vol_L * 1e15
    MCH = hb_g * 1e12
    MCHC = hb_g / vol_L / 10.0                      # g/L -> g/dL
    return {"MCV": MCV, "MCH": MCH, "MCHC": MCHC}


_add("hp_red_cell_indices", name="Red-cell indices: MCV, MCH and MCHC", topic="blood / red cells",
     purpose="Mean corpuscular volume MCV = Hct/RBC, mean corpuscular hemoglobin MCH = Hb/RBC and mean corpuscular hemoglobin concentration MCHC = Hb/Hct, from the hematocrit, hemoglobin and red-cell count.",
     inputs=[_n("Hct", "hematocrit (fraction)", pct=True), _n("Hb", "hemoglobin in g/dL"), _n("RBC", "red-cell count in 10^12 cells/L (= 10^6 cells/uL)")],
     outputs=[_o("MCV", "mean corpuscular volume in fL"), _o("MCH", "mean corpuscular hemoglobin in pg"),
              _o("MCHC", "mean corpuscular hemoglobin concentration in g/dL")],
     primary="MCV", conditions=["0 < Hct < 1", "Hb, RBC > 0"],
     method="MCV = Hct x 1000 / RBC; MCH = Hb x 10 / RBC; MCHC = Hb / Hct",
     latex=r"MCV = \frac{Hct}{RBC},\quad MCH = \frac{Hb}{RBC},\quad MCHC = \frac{Hb}{Hct}",
     run=red_cell_indices, reference=ref_red_cell_indices,
     wrong=lambda Hct, Hb, RBC: {"MCV": Hct * 10.0 / RBC, "MCH": Hb / RBC, "MCHC": Hb / Hct},
     rand=lambda rng: {"Hct": _u(rng, 0.2, 0.6, 2), "Hb": _u(rng, 6, 19, 1), "RBC": _u(rng, 2.5, 6.5, 2)},
     phrases=["red cell indices", "red-cell indices", "MCV MCH MCHC"],
     examples=[_ex({"Hct": 0.45, "Hb": 15, "RBC": 5}, {"MCV": 90, "MCH": 30, "MCHC": 33.3333333333}),
               _ex({"Hct": 0.3, "Hb": 8, "RBC": 4}, {"MCV": 75, "MCH": 20, "MCHC": 26.6666666667})],
     extra_q=[{"q": "red cell indices: Hct = 45%, Hb = 15, RBC = 5", "outputs": {"MCV": 90, "MCH": 30, "MCHC": 33.3333333333}},
              _bad({"Hct": 0.45, "Hb": 15, "RBC": 0})],
     level="beginner", weak="the per-cell volume and hemoglobin mass in SI (MCHC as MCH/MCV), the same definitions")


def ecg_heart_rate(RR=None, small_boxes=None, large_boxes=None, speed=None):
    given = 0
    for v in (RR, small_boxes, large_boxes):
        if v is not None:
            given += 1
    _need(given == 1, "give exactly one of the R-R interval RR, the number of small boxes or the number of large boxes")
    sp = 25.0 if speed is None else _pos(speed, "speed")
    if RR is not None:
        rr = _pos(RR, "RR")
    elif small_boxes is not None:
        rr = _pos(small_boxes, "small_boxes") * 1.0 / sp
    else:
        rr = _pos(large_boxes, "large_boxes") * 5.0 / sp
    hr = 60.0 / rr
    _need(20.0 <= hr <= 300.0, "the heart rate must lie between 20 and 300 beats/min")
    return _rnd({"HR": hr, "RR": rr, "RR_ms": rr * 1000.0})


def ref_ecg_heart_rate(RR=None, small_boxes=None, large_boxes=None, speed=None):
    sp = 25.0 if speed is None else speed
    if RR is not None:
        rr = _sx("r - x", "r", x=RR)
    else:
        mm = small_boxes * 1.0 if small_boxes is not None else large_boxes * 5.0
        rr = _sx("r*v - d", "r", v=sp, d=mm)                     # distance on the paper = speed x time
    return {"HR": _sx("h*r - 60", "h", r=rr), "RR": rr, "RR_ms": rr * 1000.0}


def wrong_ecg_heart_rate(RR=None, small_boxes=None, large_boxes=None, speed=None):
    sp = 25.0 if speed is None else speed
    if RR is not None:
        return {"HR": 1.0 / RR, "RR": RR, "RR_ms": RR * 1000.0}                # beats per second reported per minute
    if small_boxes is not None:
        rr = small_boxes * 5.0 / sp                                            # small and large boxes confused
    else:
        rr = large_boxes * 1.0 / sp
    return {"HR": 60.0 / rr, "RR": rr, "RR_ms": rr * 1000.0}


def _rand_ecg(rng):
    hr = rng.uniform(40, 180)
    r = rng.random()
    if r < 0.34:
        return {"RR": round(60.0 / hr, 3)}
    sp = 25 if rng.random() < 0.75 else 50
    if r < 0.67:
        d = {"small_boxes": round(60.0 / hr * sp, 1)}
    else:
        d = {"large_boxes": round(60.0 / hr * sp / 5.0, 1)}
    if sp == 50 or rng.random() < 0.5:
        d["speed"] = sp
    return d


_add("hp_ecg_heart_rate", name="Heart rate from the ECG (R-R interval or grid boxes)", topic="cardiovascular / electrocardiogram",
     purpose="The heart rate of a regular rhythm from the R-R interval, or from the number of small (1 mm) or large (5 mm) boxes between two R waves at the paper speed (25 mm/s unless stated: 1500/small boxes, 300/large boxes).",
     inputs=[_n("RR", "R-R interval in s (give one of RR, small_boxes, large_boxes)", required=False),
             _n("small_boxes", "number of small (1 mm) boxes between two R waves", required=False),
             _n("large_boxes", "number of large (5 mm) boxes between two R waves", required=False),
             _n("speed", "paper speed in mm/s (default 25)", required=False)],
     outputs=[_o("HR", "heart rate in beats/min"), _o("RR", "R-R interval in s"), _o("RR_ms", "R-R interval in ms")],
     primary="HR", conditions=["exactly one of RR, small boxes, large boxes", "regular rhythm", "20 <= HR <= 300 beats/min",
                               "1 small box = 1 mm, 1 large box = 5 mm, paper speed 25 mm/s unless stated"],
     method="HR = 60 / RR; RR = boxes x box length / paper speed (1500/small boxes, 300/large boxes at 25 mm/s)",
     latex=r"HR = \frac{60}{RR},\quad RR = \frac{n\,\ell_{box}}{v_{paper}}",
     run=ecg_heart_rate, reference=ref_ecg_heart_rate, wrong=wrong_ecg_heart_rate, rand=_rand_ecg,
     phrases=["heart rate from the ECG", "ECG heart rate", "heart rate from an ECG"],
     examples=[_ex({"RR": 0.8}, {"HR": 75, "RR": 0.8, "RR_ms": 800}),
               _ex({"large_boxes": 4}, {"HR": 75, "RR": 0.8, "RR_ms": 800}),
               _ex({"small_boxes": 15, "speed": 25}, {"HR": 100, "RR": 0.6, "RR_ms": 600}),
               _ex({"small_boxes": 20, "speed": 50}, {"HR": 150, "RR": 0.4, "RR_ms": 400})],
     extra_q=[{"q": "ECG heart rate: RR = 800 ms", "outputs": {"HR": 75, "RR": 0.8, "RR_ms": 800}},
              _bad({"RR": 0.8, "large_boxes": 4}), _bad({"large_boxes": 0.5})],
     level="beginner", weak="the defining relations (HR RR = 60, distance = speed x time) re-solved with sympy")


# ================================================================================================ levels
# Final level assignment (specification/levels.json is written from LEVELS): upper-undergraduate quantitative work
# (multi-step gas exchange, pressure-flow mechanics, constant-field and cable theory) is advanced; graduate-style
# derivations and inversions are expert.
for _k in ("hp_poiseuille_flow", "hp_laplace_wall_tension", "hp_bernoulli_pressure_drop", "hp_qrs_axis",
           "hp_alveolar_gas_equation", "hp_aa_gradient", "hp_alveolar_ventilation_co2", "hp_fick_diffusion",
           "hp_respiratory_time_constant", "hp_barometric_altitude", "hp_gas_volume_conversion", "hp_free_water_clearance",
           "hp_pah_renal_flow", "hp_chord_conductance", "hp_membrane_charge", "hp_hill_po2"):
    LEVELS[_k] = "advanced"
for _k in ("hp_severinghaus_po2", "hp_length_constant", "hp_ghk_voltage", "hp_strength_duration"):
    LEVELS[_k] = "expert"


# @@END@@
