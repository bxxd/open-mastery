# Open Math Graph

A machine-readable DAG of mathematical knowledge, from 4th grade through university. ~3,000 nodes, prerequisite edges, assessment criteria. Free forever.

## Structure

```
graph/
├── math/
│   ├── 4th-grade.json
│   ├── 5th-grade.json
│   ├── prealgebra.json
│   ├── algebra-1.json
│   ├── geometry.json
│   ├── algebra-2.json
│   ├── precalculus.json
│   ├── calculus-1.json
│   └── ...
└── schema.json
```

Each file contains nodes for one course. Cross-course prerequisites reference nodes by full ID.

## Node Format

```json
{
  "id": "alg1.quadratic_formula",
  "name": "Quadratic Formula",
  "course": "algebra-1",
  "prerequisites": ["alg1.completing_the_square", "alg1.square_roots_of_negatives"],
  "assessment_types": ["solve", "derive", "apply_word_problem"],
  "bloom_level": "apply",
  "tags": ["quadratics", "formulas"]
}
```

- **id**: `{course_prefix}.{topic}` — globally unique
- **prerequisites**: Direct only. No transitive edges. "You cannot learn B without A."
- **assessment_types**: What mastery looks like for this node
- **bloom_level**: `know` → `understand` → `apply` → `analyze`. Mastery = `apply` minimum.

## Contributing

The graph is a hypothesis. It gets better with use. If a student hits a wall, there's a missing edge. If a student breezes through, there's an unnecessary edge.

Open a PR. Fix an edge. Add a node. Split one that's too coarse.

## License

CC-BY-SA 4.0. Extend it, fork it, build on it — but keep derivatives open.
