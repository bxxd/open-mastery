# 010: Encompassing Relationships

Advanced topics implicitly exercise simpler sub-skills. This enables compressed reviews — one advanced problem refreshes multiple foundations.

## Format

Optional `encompasses` field on node YAML:

```yaml
id: alg.quad.formula
prereqs:
  - alg.quad.completing_square
encompasses:
  ops.oo.basics: 0.3
  alg.exp.combining: 0.5
  alg.eq.two_step: 0.7
```

Weight (0-1): how strongly practicing this node refreshes the sub-skill.

## Implementation Steps

### 1. Schema
- Add `encompasses` to YAML format (optional field, map of node_id → weight)
- Update `docs/GRAPH_FORMAT.md`

### 2. Engine (core crate)
- Add `encompasses: HashMap<String, f32>` to `Node` struct
- Parse during graph load
- Validate: all encompassed nodes exist, weights 0-1, no self-references
- Build reverse index: `encompassed_by: HashMap<String, Vec<(String, f32)>>`

### 3. Teacher MCP
- `get_node` shows encompasses and encompassed_by (reverse lookup)
- `create_node` / `update_node` accept encompasses field
- `suggest_encompasses(node_id)` — future: LLM-assisted suggestions

### 4. Annotate graph
- Start with high-value nodes: algebra nodes encompassing arithmetic
- LLM-assisted: "Given this node's assessment types, which simpler nodes does it exercise?"
- Teacher validation pass

### 5. Spaced repetition (future, separate task)
- Use encompasses weights to compute review compression
- Recent practice on node Y partially refreshes all nodes Y encompasses

## Design Decisions

- **Encompasses vs prereqs**: Different things. Prereqs gate advancement. Encompasses drives review scheduling.
- **A node can encompass non-direct-prereqs.** `alg.quad.formula` encompasses `ops.oo.basics` even though OoO is many levels below.
- **Weights are approximate.** 0.3 = lightly exercises, 0.7 = strongly exercises. Teachers tune these.

## Done when

- Engine loads and validates `encompasses` field
- Teacher MCP exposes encompasses in get_node and accepts it in create/update
- 10+ nodes annotated with encompasses as proof of concept
- `validate_graph` checks encompasses references
