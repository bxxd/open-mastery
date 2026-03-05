# Open Mastery

An open-source knowledge graph and mastery engine for mathematics.

**The structure of mathematical knowledge — what depends on what, what to learn next — belongs to everyone.** Not behind a paywall. Not locked in proprietary software. Open, versioned, forkable, and free.

## What This Is

A machine-readable DAG of math topics from 4th grade through pre-calculus, paired with a Rust engine that answers one question: **"what should this student learn next?"**

An LLM handles the teaching. This project provides the **structure** — the prerequisite graph, mastery gates, and pedagogical guidance. You demonstrate mastery before you advance. No shortcuts, no pace limits.

### The Graph

131 nodes across 9 mathematical domains. One YAML file per topic. Cross-domain prerequisite edges.

```
graph/math/
├── number-sense/          7 nodes    (place value, rounding, patterns)
├── operations/           17 nodes    (add, subtract, multiply, divide, order of ops)
├── fractions/            12 nodes    (concepts, equivalence, arithmetic)
├── decimals/              5 nodes    (place value, operations, conversion)
├── geometry/             26 nodes    (angles, shapes, area, volume, coordinate plane, proofs)
├── ratios-proportions/    6 nodes    (ratios, rates, percent)
├── statistics/           12 nodes    (data display, probability, inference)
├── algebra/              38 nodes    (expressions, equations, functions, quadratics, logs)
└── trigonometry/          8 nodes    (ratios, unit circle, identities, graphing)
```

Each node:

```yaml
id: frac.con.basics
prereqs:
  - ops.div.facts
bloom: understand
assess:
  - identify
  - compare
  - represent
typical_grade: 4
context: >
  Fractions as parts of a whole.
  Visual models (pizza, number line) before symbolic notation.
  Numerator = pieces selected, denominator = total equal pieces.
```

`_prompt.yaml` files at each directory level cascade into teaching instructions. A Montessori school forks the prompts; the DAG stays the same.

### The Engine

Rust. Two MCP servers share a core crate.

**Student MCP** — connect Claude (or any LLM) and start learning:

```
get_frontier(student_id)                    → what to learn next
get_node(node_id)                           → topic details + teaching guidance
record_mastery(student_id, node_id, level)  → unlock new topics
get_progress(student_id)                    → full mastery state
```

**Teacher MCP** — build curriculum through conversation:

```
Browse:     list_domains, list_nodes, search_nodes, get_node, validate_graph
CRUD:       create_node, update_node, delete_node, add_prerequisite, remove_prerequisite
Prompts:    get_prompt_cascade, set_prompt
Git:        git_status, git_commit, git_log
Artifacts:  save_artifact, get_artifact, list_artifacts
```

## Quickstart

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- An MCP client (Claude Desktop, Claude Code, or any MCP-compatible app)

### Build

```bash
git clone https://github.com/bxxd/open-mastery.git
cd open-mastery
make build          # debug build
make release        # optimized build
make test           # run tests
```

### Connect to Claude Desktop or Claude Code

Copy `mcp.json.example` to your MCP config and update paths:

```json
{
  "mcpServers": {
    "open-mastery-student": {
      "command": "/path/to/open-mastery-student",
      "args": ["--stdio"],
      "env": {
        "GRAPH_DIR": "/path/to/open-mastery/graph/math",
        "PROGRESS_DIR": "/path/to/open-mastery/progress"
      }
    },
    "open-mastery-teacher": {
      "command": "/path/to/open-mastery-teacher",
      "args": ["--stdio"],
      "env": {
        "GRAPH_DIR": "/path/to/open-mastery/graph/math",
        "REPO_DIR": "/path/to/open-mastery"
      }
    }
  }
}
```

For Claude Desktop: `~/.claude/mcp.json`
For Claude Code: `.mcp.json` in your workspace root

### Start Learning

1. Open Claude with the student MCP connected
2. Say "let's do math" — Claude calls `get_frontier` and shows available topics
3. Pick a topic — Claude calls `get_node` for teaching guidance
4. Work through problems — Claude assesses using the node's `assess` types
5. Demonstrate mastery — Claude calls `record_mastery`, new topics unlock

### Build Curriculum

1. Open Claude with the teacher MCP connected
2. "Show me the algebra domain" → `list_nodes`
3. "Add a node for completing the square" → `create_node`
4. "This should require factoring trinomials" → `add_prerequisite`
5. "Validate the graph" → `validate_graph`
6. "Commit" → `git_commit`

### HTTP/SSE Mode

For web clients or multi-user setups:

```bash
make run-student-sse    # student MCP on port 3001
make run-teacher-sse    # teacher MCP on port 3002
```

## How It Works

The core algorithm:

```
frontier(student) = {
  node for node in all_nodes
  if node not mastered by student
  AND all node.prereqs mastered by student
}
```

That's it. The frontier is everything unlocked but not yet learned. The student picks from the frontier. The DAG enforces prerequisites. Mastery gates ensure no gaps.

## Why

The architecture has been proven:

- 3rd graders completing Calc BC in a single year
- 11-year-olds earning perfect 5s on AP Calc BC
- Kids who "hated math" falling in love with the subject

The formula: DAG of prerequisites + strict mastery gates + unlimited velocity. Remove the pace limit, keep the competence constraint.

But every existing knowledge graph like this is proprietary. Khan Academy's graph is implicit. Nothing else has an open, forkable, machine-readable prerequisite graph with mastery enforcement.

## Architecture

```
                  ┌─────────────────────┐
  Claude Desktop ─┤                     ├─ graph/ (YAML)
  Claude Code ────┤    Engine Core      ├─ progress/ (JSON)
  Any MCP client ─┤  (Rust, domain logic)│
  Web Frontend ───┤                     │
                  └─────────────────────┘
                  Student  Teacher  CLI  HTTP
                    MCP     MCP     ▲     ▲
                           adapters (ports)
```

Hexagonal architecture. The core is pure domain logic. Each adapter is its own crate. Build one, build all, swap them — the core never changes.

## Contributing

The graph is a hypothesis. Ship at 80% correct. Real students hitting real walls reveal missing edges. Real students breezing through reveal unnecessary edges.

Ways to contribute:

- **Add nodes** — missing topics, new domains, deeper coverage
- **Fix edges** — missing prerequisites, unnecessary prerequisites
- **Improve context** — better pedagogical guidance for the LLM
- **Add prompt files** — teaching philosophy at domain/unit level
- **Build adapters** — web frontend, CLI tools, new MCP features

See `docs/GRAPH_FORMAT.md` for the full graph specification and `DEVELOPER.md` for architecture details.

## License

Dual licensed:

- **Graph data** (`graph/`) — [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Fork it, extend it, keep derivatives open.
- **Engine code** (`engine/`) — [MIT](https://opensource.org/licenses/MIT). Use it however you want.
