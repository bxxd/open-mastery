# 008: Expand Statistics Domain

Currently 3 nodes. Target: 12-15 nodes covering grades 6-10.

## Current Nodes

- `stat.dd.dot_plots_histograms` (grade 6)
- `stat.mc.mean_median_mode` (grade 6)
- `stat.prob.basic` (grade 7)

## Nodes to Add

### Data Display (unit: `dd`)
- `stat.dd.bar_line_graphs` — Bar graphs, line graphs. Grade 4-5. Root or prereq: none needed.
- `stat.dd.box_plots` — Box-and-whisker plots, quartiles, IQR. Grade 6. Prereq: `stat.mc.mean_median_mode`
- `stat.dd.scatter_plots` — Scatter plots, trend lines, correlation. Grade 8. Prereq: `geo.cp.plotting`

### Measures (unit: `mc`)
- `stat.mc.range_iqr_mad` — Range, interquartile range, mean absolute deviation. Grade 6. Prereq: `stat.mc.mean_median_mode`

### Probability (unit: `prob`)
- `stat.prob.compound` — Compound events, independent vs dependent. Grade 7. Prereq: `stat.prob.basic`, `frac.md.multiply`
- `stat.prob.counting` — Combinations, permutations, fundamental counting principle. Grade 8. Prereq: `stat.prob.compound`, `ops.mul.facts`
- `stat.prob.conditional` — Conditional probability, Bayes intro. Grade 9. Prereq: `stat.prob.compound`

### Inference (unit: `inf`)
- `stat.inf.sampling` — Sampling methods, bias, representative samples. Grade 7. Prereq: `stat.mc.mean_median_mode`
- `stat.inf.normal_distribution` — Bell curve, standard deviation, z-scores. Grade 9. Prereq: `stat.mc.range_iqr_mad`, `alg.fn.intro`
- `stat.inf.regression` — Linear regression, line of best fit, residuals. Grade 9. Prereq: `stat.dd.scatter_plots`, `alg.fn.linear`

## Done when

- 10+ new statistics nodes created with proper prereqs, bloom, assess, context
- Cross-domain edges validated (fractions, algebra, geometry connections)
- `validate_graph` passes
