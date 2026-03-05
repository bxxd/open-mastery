# 001: Define Graph Schema

Nail down `graph/schema.json` before generating any nodes. This is the contract everything else builds on.

## Deliverable

`graph/schema.json` — JSON Schema defining the node format.

## Node format (current thinking)

```json
{
  "id": "alg1.quadratic_formula",
  "name": "Quadratic Formula",
  "course": "algebra-1",
  "prerequisites": ["alg1.completing_the_square"],
  "assessment_types": ["solve", "derive", "apply_word_problem"],
  "bloom_level": "apply",
  "tags": ["quadratics", "formulas"]
}
```

## Open questions

- Is `bloom_level` per-node or per-assessment_type? (Current: per-node, mastery = "apply" minimum)
- Do we need a `description` field? Useful for LLM context, adds bulk.
- Tag taxonomy — freeform or controlled vocabulary?
- Course prefix conventions — `4g`, `5g`, `pa`, `alg1`, `geo`, `alg2`, `pc`, `cal1`, `cal2`?

## Done when

- Schema file exists and validates the example node above
- At least one course file can be validated against it
