"""human-physiology-ai999x — offline human physiology answers with their formula, steps and verification.

    import human_physiology_ai999x as m
    m.ask("alveolar gas equation: FiO2 = 21%, Patm = 760 mmHg, PH2O = 47 mmHg, PaCO2 = 40 mmHg, RQ = 0.8").show()  # PAO2 99.73 mmHg
    m.solve("hp_poiseuille_flow", dP="10 mmHg", eta=3.5, L="10 cm", r="2 mm").show()                            # Q 23.93 mL/s
    m.capabilities()                   # every certified capability

Calculation of standard textbook physiology quantities: cardiovascular (cardiac output and index, stroke volume and
ejection fraction, the Fick principle, mean arterial and pulse pressure, systemic and pulmonary vascular resistance,
Poiseuille resistance and flow, vessels in series and parallel, Reynolds and Womersley numbers, Laplace wall tension,
compliance, Bernoulli drops, stroke work, Teichholz volumes, QT correction, the QRS axis, Starling filtration,
pulse wave velocity, Qp/Qs and shunt fraction); respiratory (lung capacities, minute and alveolar ventilation, Bohr
dead space, the alveolar gas equation, A-a gradient, oxygen content, delivery and consumption, Hill and Severinghaus
oxygen binding, Henderson-Hasselbalch, anion gap, Fick diffusion, respiratory mechanics, altitude, BTPS/STPD, the Weir
equation); renal (clearance, timed creatinine clearance, PAH flow, filtration fraction, free water clearance,
fractional excretion, CKD-EPI 2021 creatinine and creatinine-cystatin C, Cockcroft-Gault, tubular handling, glucose
titration, osmolality); body fluids (compartments, Watson water, osmolarity, osmotic pressure, fluid shifts,
Gibbs-Donnan); nerve and muscle (Nernst, Goldman-Hodgkin-Katz, chord conductance, time and length constants,
conduction velocity, strength-duration, Hill force-velocity and maximal power); body composition and metabolism (BMI,
body surface area, Harris-Benedict, Mifflin-St Jeor, Katch-McArdle, METs, Atwater, body fat, lean body mass, heart
rate reserve, elimination half-life). Questions use one documented form, '<capability phrase>: name = value, ...';
a question that is not determined is refused rather than guessed.

These are computed values of textbook equations, NOT medical advice: no diagnosis, treatment or dosing
recommendation.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.6.6"
_P = Product("human_physiology", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
