# Open Mastery

An open-source knowledge graph and mastery engine for mathematics. The map, not the territory.

**This is a public good.** The structure of mathematical knowledge — what depends on what, what to learn next — belongs to everyone. Not behind a paywall. Not locked in proprietary software. Open, versioned, forkable, and free.

## What This Is

A machine-readable DAG (directed acyclic graph) of math topics from 4th grade through university, paired with a traversal engine that answers one question — **"what should this student learn next?"**

The graph encodes what every math textbook knows implicitly: which concepts depend on which. Open Mastery makes that structure explicit, versioned, and forkable.

An LLM (Claude, GPT, whatever) handles the teaching and assessment. This project provides the **structure** — the optimal path through the material, the mastery gates, the spaced repetition. The system enforces one rule: you must demonstrate mastery before you advance. No shortcuts, no pace limits.

## Inspiration

On March 1, 2026, a parent posted that her third-grader — a kid who had "hated math" just a year earlier — casually finished Calc BC. A visualization of a full math knowledge graph followed: ~3,000 topics, 17 courses, from 4th grade to university. The tweet hit 1.8M views.

Someone replied: **"can I see the data for this graph?"**

They can't. It's proprietary.

That's the problem this project solves. The structure of how math builds on itself is not secret knowledge — it's the consensus of every textbook ever written. It should be open.

## Why This Needs to Exist

The architecture has been proven at scale:

- 3rd graders completing grades 6–12 + Calc BC in a single year
- 11-year-olds earning perfect 5s on AP Calc BC
- Groups of 11–13-year-olds going from basic pre-algebra to acing BC in ~18 months (~6–7 years of normal schooling)
- Kids who "hated math" falling in love with the subject

The architecture is simple: DAG of prerequisites + strict mastery gates + unlimited velocity. Remove the classroom-speed bottleneck, keep the competence constraint, let talent explode. This is deliberate practice (Ericsson-style) engineered at scale.

But every existing knowledge graph of this kind is proprietary. Khan Academy made content free but the mastery system is weak and the graph is implicit, not explicit. Nothing else comes close:

- **OSSU/math** — just curated links to free courses, no mastery enforcement or graph
- **Metacademy** — ML concepts only, abandoned
- **Common Core standards** — topic lists but NOT a dependency graph
- **OpenStax** — textbooks, no graph

The field is wide open. No public 3,000-node YAML dump. No reference engine. No GitHub org with community PRs. This is the biggest untapped education unlock since Khan Academy.

## The Key Insight: The Graph Is Reconstructable

The graph is not secret knowledge. It's the consensus of how math builds on itself — every textbook encodes it implicitly in chapter ordering. The work is making it explicit and granular. We can construct it:

1. **Bootstrap with LLMs** — "List every topic in Algebra 1. For each, list prerequisites." Do this for all domains. Get 80% of the graph in a day.
2. **Refine the 20%** — Cross-domain edges, granularity tuning, encompassing relationships. This is the curriculum expertise work.
3. **Community improves it** — Like Wikipedia. Doesn't need to be perfect on day one. Needs to be open and improvable.

## Core Concepts

### Knowledge Frontier

A student's position on the DAG — not a single "current topic" but the full boundary of what they've mastered. The engine computes the optimal next task from this frontier for maximum learning velocity.

### Mastery Gates

Strict. You demonstrate mastery or you don't advance. No partial credit at the gate level. No skipping. This is the constraint that makes unlimited velocity safe.

### Deliberate Practice at the Two-Sigma Level

The graph enables Ericsson-style deliberate practice at Bloom's two-sigma level. Every task is at the edge of ability (frontier computation). Every assessment requires deep engagement (explain, derive, generate — not just recall). The prompt cascade provides expert tutoring direction calibrated to each topic. The result: one-on-one tutoring outcomes at zero marginal cost.

### Encompassing Relationships (Future)

Advanced topics implicitly practice simpler sub-skills. A calculus problem exercises algebra. The graph will encode these weights so the engine can compress reviews — one advanced problem can knock out multiple simpler reviews. This prevents review overload as the graph grows.

### Spaced Repetition with Decay (Future)

Mastery isn't binary forever. Confidence decays over time. The student model will track last-assessed timestamps and schedule reviews before mastery drops below threshold. Encompassing relationships make this efficient — reviewing an advanced topic refreshes its prerequisites.

## Architecture

Two **MCP servers** share the same core engine. Any LLM client that speaks MCP — Claude Desktop, Claude Code, custom apps — connects and becomes either a tutor or a curriculum editor.

```
graph/math/                 Knowledge graph (YAML). One file per node, organized by domain.
  number-sense/             Domain directories
  operations/
  fractions/
  ...
plans/                      Curated paths through the graph (YAML).
engine/
├── crates/
│   ├── core/               Domain logic. Graph loading, traversal, mutations, mastery state.
│   ├── mcp-student/        Student MCP. Lean tutoring: frontier, teach, assess, master.
│   ├── mcp-teacher/        Teacher MCP. Full authoring: browse, CRUD, validate, git, artifacts.
│   ├── cli/                CLI adapter. Scripting, batch ops, graph validation. (planned)
│   └── web-server/         HTTP adapter. Graph explorer, progress dashboard. (planned)
└── Cargo.toml              Workspace root.
```

### The Graph

YAML files organized by mathematical domain (not grade level). One file per node. See `docs/GRAPH_FORMAT.md` for the full specification.

Each node: three-part ID (`domain.unit.topic`), prerequisite edges, assessment criteria, Bloom level, pedagogical context. `_prompt.yaml` files at each directory level provide cascading system prompts that control HOW the LLM teaches.

Grades are metadata (`typical_grade`), not the organizing principle. The graph is open to any learner regardless of age.

### Two MCP Servers

**Student MCP** (`open-mastery-student`) — lean tutoring engine:

```
Tools:
  get_frontier(student_id)                      → unlocked nodes ready to learn
  get_node(node_id)                             → node details + prompt cascade
  record_mastery(student_id, node_id, level)    → update state, return newly unlocked
  get_progress(student_id)                      → full mastery state
```

**Teacher MCP** (`open-mastery-teacher`) — full curriculum authoring toolkit:

```
Browse:     list_domains, list_nodes, search_nodes, get_node, validate_graph
CRUD:       create_node, update_node, delete_node, add_prerequisite, remove_prerequisite
Prompts:    get_prompt_cascade, set_prompt
Git:        git_status, git_commit, git_log
Artifacts:  save_artifact, get_artifact, list_artifacts
```

Teachers connect Claude to the teacher MCP and build curriculum through conversation. Every mutation writes YAML to disk. Git tracks versions.

### Student State

Per-student, per-node: mastery level, last assessed. JSON files. Separate from the graph — the graph is static and public, student state is dynamic and private.

### Plans

Curated paths through the graph. A plan filters the frontier — "only show nodes in this plan." Teachers create plans for grade levels, remediation, or acceleration. The DAG enforces prerequisites regardless.

### The Tutor

Not in this repo. That's whatever LLM client you point at the student MCP server. Claude Desktop connects, calls `get_frontier` to know what to teach, `get_node` to understand the concept and get teaching guidance (prompt cascade), teaches and assesses, then calls `record_mastery` when the student demonstrates it.

### Quickstart

```bash
cp .env.example .env        # configure graph/progress paths
make test                    # run all tests
make run                     # start student MCP (stdio)
make run-sse                 # start student MCP (HTTP/SSE on port 3001)
```

Copy `mcp.json.example` to your `.mcp.json` and update paths to connect Claude Desktop or Claude Code.

### How It Works

**For students:**
1. Connect Claude to the student MCP via `.mcp.json`
2. Kid opens Claude and starts learning
3. Claude calls `get_frontier` → "Here's what you can learn next"
4. Kid picks a topic (or Claude recommends one)
5. Claude calls `get_node` → gets teaching guidance from prompt cascade
6. Claude teaches, generates problems, assesses
7. Claude calls `record_mastery` → new nodes unlock
8. Repeat

**For teachers:**
1. Connect Claude to the teacher MCP via `.mcp.json`
2. Teacher says "show me the fractions domain"
3. Claude calls `list_nodes` → browseable list
4. Teacher says "add a node for mixed number addition"
5. Claude calls `create_node` → YAML file written to disk
6. Teacher says "commit these changes"
7. Claude calls `git_commit` → versioned

### Hexagonal Architecture

The engine is the core — pure domain logic with no opinion about how you interact with it. Ports and adapters:

```
                  ┌─────────────────────┐
  Claude Desktop ─┤                     ├─ graph/ (YAML files)
  Claude Code ────┤                     ├─ progress/ (JSON files)
  Codex / OpenAI ─┤    Engine Core      ├─ plans/ (YAML files)
  Web Frontend ───┤  (domain logic)     │
  Custom apps ────┤                     │
                  └─────────────────────┘
                  Student  Teacher  CLI  HTTP
                    MCP     MCP     ▲     ▲
                    ▲        ▲
                    adapters (ports)
```

Each adapter is its own crate, depending only on `core`:

- **`mcp-student`**: Lean tutoring. Connect Claude Desktop, kid starts learning immediately.
- **`mcp-teacher`**: Full authoring toolkit. Browse, create, edit, validate, git, artifacts.
- **`web-server`**: (planned) Graph visualization, progress dashboard, embedded chat, graph editor.
- **`cli`**: (planned) Scripting, batch operations, graph validation.

The engine doesn't know or care which adapter is calling it. Build one, build all, swap them out — the core never changes.

## Core Philosophy

### KISS (Keep It Simple, Stupid)

- Favor simplicity over complexity
- Implement the minimum viable solution first
- Add complexity only when necessary and justified

### One True Path (YOLO)

- No fallback logic — single execution path
- Choose the right approach and commit to it
- If the primary approach fails, that's feedback for the design
- Avoid defensive programming that masks root causes

### Performance First

- Rust. The engine should traverse thousands of nodes instantly.
- Measure before optimizing (benchmark-driven decisions)
- Use profiling tools to identify actual bottlenecks

### Evidence-Based Development

- Benchmark before optimizing
- Test assumptions with actual data
- Let facts on the ground prove theories

### Separation of Concerns

- Graph data, traversal engine, student state, and tutor interface are independent
- Each module is independently testable
- Single responsibility principle

### DRY — Don't Repeat Yourself

- Eliminate duplication
- Single source of truth for configuration, logic, and data structures
- Consistent abstractions over copy-paste solutions

## Debugging & Investigation

1. **Form a Theory** — Develop a hypothesis from symptoms
2. **Gather Evidence** — Don't assume, collect facts
3. **Divide and Conquer** — Isolate components, trace code paths
4. **Prove Your Case** — Let facts prove the theory before making changes
5. **Only Then Execute** — Targeted fixes based on evidence, not guesswork

## Tech Stack

- **Language**: Rust
- **Graph format**: YAML (one file per node, organized by domain)
- **Student state**: JSON (file-based to start)
- **Interface**: MCP servers (Model Context Protocol) — student + teacher
- **Tutor**: Any MCP client — Claude Desktop, Claude Code, Codex, or any custom app

## License

Dual licensed. The graph data (`graph/`) is **CC-BY-SA 4.0** — fork it, extend it, but keep derivatives open. The engine code (`engine/`) is **MIT** — use it however you want. See [LICENSE](LICENSE) for details.
