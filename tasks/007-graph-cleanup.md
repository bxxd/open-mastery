# 007: Graph Cleanup — Transitive Edges & Bloom Levels

Fix structural issues found in graph review (March 2026, 103 nodes).

## Transitive Edges to Remove

The rule: "direct, not transitive." If A→B→C, don't also have A→C.

| Node | Remove prereq | Already reached via |
|------|--------------|---------------------|
| `ns.pat.sequences` | `ops.add.within_1000` | `ops.mul.facts` |
| `geo.ls.classifying_2d` | `geo.ang.basics` | `geo.ls.lines_and_symmetry` |
| `ops.oo.basics` | `ops.add.within_1000` | `ops.div.facts` → `ops.mul.facts` → ... |
| `ops.mul.factors_and_multiples` | `ops.mul.facts` | `ops.div.facts` |
| `rat.pct.increase_decrease` | `rat.pct.basics` | `rat.prop.solving` |
| `geo.meas.metric_conversion` | `ns.pv.powers_of_10` | `dec.ops.multiply` |
| `geo.meas.unit_conversion` | `ops.mul.facts` | `ops.mul.by_10_100_1000` |

Verify each one before removing — trace the transitive path and confirm the indirect route exists.

## Bloom Level Fixes

| Node | Current | Change to | Reason |
|------|---------|-----------|--------|
| `ns.pv.thousands` | apply | understand | Root node — entry point, learning the concept |
| `alg.quad.formula` | apply | analyze | Derivation from completing the square |
| `geo.prf.two_column` | apply | analyze | Proof-based |
| `geo.tri.angle_sum` | apply | analyze | Proof-based |

## Done when

- All transitive edges removed (validate with `validate_graph`)
- Bloom levels updated
- `cargo test` passes
- Graph still loads cleanly with no orphans or broken links
