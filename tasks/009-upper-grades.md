# 009: Upper Grade Content — Trig, Logs, Rational Expressions

Grade 10+ has 1 node. This task fills the gap between algebra and calculus.

## Rational Expressions (algebra domain)

Directory exists (`algebra/rational-expressions/`) but is empty.

| Node | Prereqs | Grade | Bloom |
|------|---------|-------|-------|
| `alg.rat.simplifying` | `alg.quad.factoring_gcf`, `frac.con.basics` | 9 | apply |
| `alg.rat.multiply_divide` | `alg.rat.simplifying`, `frac.md.multiply`, `frac.md.divide` | 9 | apply |
| `alg.rat.add_subtract` | `alg.rat.simplifying`, `frac.as.unlike_add` | 10 | apply |
| `alg.rat.equations` | `alg.rat.add_subtract`, `alg.eq.multi_step` | 10 | apply |

## Logarithms (new unit: algebra/logarithms/)

| Node | Prereqs | Grade | Bloom |
|------|---------|-------|-------|
| `alg.log.basics` | `alg.exp.exponent_rules` | 10 | understand |
| `alg.log.properties` | `alg.log.basics` | 10 | apply |
| `alg.log.equations` | `alg.log.properties`, `alg.eq.multi_step` | 10 | apply |
| `alg.log.change_of_base` | `alg.log.properties` | 10 | apply |

## Trigonometry (new domain: `trig`)

New directory: `trigonometry/`

| Node | Prereqs | Grade | Bloom |
|------|---------|-------|-------|
| `trig.rat.basics` | `geo.tri.special_right`, `frac.md.divide` | 10 | understand |
| `trig.rat.solving` | `trig.rat.basics`, `alg.eq.multi_step` | 10 | apply |
| `trig.rat.inverse` | `trig.rat.solving` | 10 | apply |
| `trig.uc.unit_circle` | `trig.rat.basics`, `geo.circ.properties` | 10 | understand |
| `trig.uc.radians` | `trig.uc.unit_circle`, `rat.prop.solving` | 10 | apply |
| `trig.id.pythagorean` | `trig.uc.unit_circle`, `geo.ap.pythagorean` | 11 | apply |
| `trig.id.sum_difference` | `trig.id.pythagorean` | 11 | analyze |
| `trig.gr.sinusoidal` | `trig.uc.radians`, `alg.fn.linear` | 11 | apply |

## Sequences & Series (new unit: algebra/sequences/)

| Node | Prereqs | Grade | Bloom |
|------|---------|-------|-------|
| `alg.seq.arithmetic` | `alg.fn.linear`, `ns.pat.sequences` | 9 | apply |
| `alg.seq.geometric` | `alg.seq.arithmetic`, `alg.exp.exponent_rules` | 9 | apply |
| `alg.seq.series` | `alg.seq.arithmetic`, `alg.seq.geometric` | 10 | apply |

## Done when

- All nodes created with prereqs, bloom, assess, context
- Trigonometry domain added to GRAPH_FORMAT.md domain prefix table (already listed as `trig`)
- `_prompt.yaml` files created for new units
- `validate_graph` passes
- Cross-domain edges validated
