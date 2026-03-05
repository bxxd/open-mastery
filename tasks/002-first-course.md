# 002: Generate First Course — 4th Grade

Bottom of the DAG. Fewest prerequisites (mostly none). Proves the format works.

## Deliverable

`graph/math/4th-grade.json` — ~200-300 nodes with prerequisite edges.

## Approach

1. Use Claude to generate topic list at the right granularity
2. For each topic, identify direct prerequisites (within course only for now)
3. Validate: topological sort produces a sane learning order
4. Sanity check against Common Core 4th grade standards and 2-3 textbook TOCs

## Granularity test

If you can't write 3 problems that test exactly this node and nothing else, the node is too big. Split it.

## Depends on

- 001 (schema)

## Done when

- JSON file validates against schema
- Topological sort produces reasonable learning order
- Cross-checked against at least one real curriculum
