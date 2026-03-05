# Graph Format Specification

The knowledge graph is stored as YAML files in a hierarchical directory structure. One file per node. The filesystem is the schema.

## Directory Structure

Organized by **mathematical domain**, not grade level. Grades are metadata on nodes (`typical_grade`), not the organizing principle. The graph is open to any learner regardless of age.

```
graph/math/
├── _prompt.yaml                          ← SUBJECT level
│
├── number-sense/                         ← DOMAIN level
│   ├── _prompt.yaml
│   ├── place-value/                      ← UNIT level
│   │   ├── _prompt.yaml
│   │   ├── thousands.yaml                ← NODE (leaf)
│   │   ├── millions.yaml
│   │   ├── decimals_to_thousandths.yaml
│   │   └── powers_of_10.yaml
│   ├── rounding/
│   │   ├── whole_numbers.yaml
│   │   └── decimals.yaml
│   └── patterns/
│       └── sequences.yaml
│
├── operations/
│   ├── _prompt.yaml
│   ├── addition/
│   ├── subtraction/
│   ├── multiplication/
│   ├── division/
│   ├── order-of-operations/
│   └── word-problems/
│
├── fractions/
│   ├── _prompt.yaml
│   ├── concepts/
│   ├── equivalence/
│   ├── add-subtract/
│   ├── multiply-divide/
│   └── mixed-numbers/
│
├── decimals/
│   ├── _prompt.yaml
│   ├── place-value/
│   ├── operations/
│   └── conversion/
│
├── geometry/
│   ├── _prompt.yaml
│   ├── angles/
│   ├── lines-shapes/
│   ├── area-perimeter/
│   ├── volume/
│   ├── coordinate-plane/
│   └── measurement/
│
├── ratios-proportions/
├── algebra/
├── trigonometry/
├── statistics/
├── calculus/
└── linear-algebra/
```

Three levels: **subject → domain → unit → node**. The DAG edges (prerequisites) cross all boundaries freely. The directory hierarchy is for humans and prompt scoping — the engine ignores it when building the graph.

## Node Files

Each `.yaml` file (not prefixed with `_`) is a node in the DAG.

```yaml
# graph/math/fractions/concepts/basics.yaml

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

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Globally unique three-part key: `{domain}.{unit}.{topic}` (e.g., `frac.con.basics`, `alg.eq.quadratic_formula`). Mirrors directory path. |
| `prereqs` | list | Direct prerequisite node IDs. Cross-domain is expected. No transitive edges. Empty list `[]` for root nodes. |
| `bloom` | enum | Minimum Bloom level for mastery: `know`, `understand`, `apply`, `analyze`. Most nodes are `apply`. |
| `assess` | list | Assessment types that demonstrate mastery at the stated Bloom level. |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `context` | string | Pedagogical guidance for the LLM tutor. Terse, expert-level. Not a script — direction. |
| `tags` | list | Freeform tags for filtering/grouping (e.g., `number_sense`, `place_value`). |
| `typical_grade` | integer | Grade level where this is typically taught (4-12+). Advisory only — does not gate access. |

### Node ID Convention

Three-part keys: `domain.unit.topic`. The ID mirrors the file's location in the directory tree.

Domain prefixes:

| Prefix | Domain | Directory |
|--------|--------|-----------|
| `ns` | Number Sense | `number-sense/` |
| `ops` | Operations | `operations/` |
| `frac` | Fractions | `fractions/` |
| `dec` | Decimals | `decimals/` |
| `geo` | Geometry | `geometry/` |
| `rat` | Ratios & Proportions | `ratios-proportions/` |
| `alg` | Algebra | `algebra/` |
| `trig` | Trigonometry | `trigonometry/` |
| `calc` | Calculus | `calculus/` |
| `la` | Linear Algebra | `linear-algebra/` |
| `stat` | Statistics & Probability | `statistics/` |
| `dm` | Discrete Math | `discrete-math/` |

Unit abbreviations are short and obvious: `pv` (place-value), `mul` (multiplication), `eq` (equivalence/equations), `con` (concepts), `as` (add-subtract), `md` (multiply-divide), etc.

Examples:
- `ns.pv.thousands` → `number-sense/place-value/thousands.yaml`
- `ops.mul.facts` → `operations/multiplication/facts.yaml`
- `frac.eq.comparing` → `fractions/equivalence/comparing.yaml`
- `alg.eq.quadratic_formula` → `algebra/equations/quadratic_formula.yaml`

### Assessment Types

The `assess` field tells the LLM what kinds of problems to generate. The LLM decides the specific problems, difficulty, and wording.

| Type | What it tests |
|------|---------------|
| `identify` | Recognize or name the concept |
| `compare` | Order, rank, or choose between examples |
| `represent` | Draw, model, or convert between representations |
| `recall` | State facts, definitions, or formulas from memory |
| `solve` | Execute a standard procedure |
| `explain` | Articulate why something works or justify a step |
| `explain_pattern` | Identify and describe a pattern or rule |
| `generate` | Produce examples that satisfy constraints |
| `convert` | Transform between equivalent forms |
| `order` | Arrange items by value or property |
| `derive` | Prove or derive a formula/result |
| `word_problem` | Apply the concept in a real-world context |

This list is extensible. New assessment types can be added as the graph grows.

## Prompt Files

`_prompt.yaml` files define cascading system prompts. They are NOT nodes in the DAG. They control HOW the LLM teaches.

```yaml
# graph/math/_prompt.yaml (subject level)
system: >
  You are a math tutor. Socratic method — guide the student
  to discover answers, don't give them directly. Verify
  understanding before advancing. Be encouraging but honest.
```

```yaml
# graph/math/fractions/_prompt.yaml (domain level)
system: >
  Fractions are where many students lose confidence. Visual
  models are essential — fraction strips, number lines, area
  models. Always connect back to division and fair sharing.
  Build from concrete to symbolic.
```

```yaml
# graph/math/fractions/equivalence/_prompt.yaml (unit level)
system: >
  Same amount, different names. Visual proof is the foundation.
  Shading the same area with different-sized pieces. Connect
  to multiplication and division of numerator/denominator.
```

### Prompt Cascade

When the engine serves a node, it walks up the directory tree and concatenates all `_prompt.yaml` files from root to leaf. Most specific level can override or extend.

For `graph/math/fractions/concepts/basics.yaml`, the LLM receives:

```
[system prompt from graph/math/_prompt.yaml]           ← subject
[system prompt from graph/math/fractions/_prompt.yaml]  ← domain
[system prompt from graph/math/fractions/concepts/_prompt.yaml]  ← unit (if exists)
[node context from basics.yaml]                         ← node
[student state: mastered prereqs, current belief]       ← runtime
```

This is how a Montessori school forks the graph: replace `graph/math/_prompt.yaml` with their teaching philosophy. Every node underneath changes behavior. The DAG structure stays identical.

## The DAG

### How It Works

The engine loads ALL `.yaml` node files from all directories, builds one flat graph (`{node_id → node}`), and validates edges. The directory structure is invisible to the DAG — only `id` and `prereqs` fields matter.

**Frontier computation** (the core algorithm):

```
frontier(student) = {
  node for node in all_nodes
  if node not mastered by student
  AND all node.prereqs are mastered by student
}
```

That's it. The frontier is everything unlocked but not yet learned. The student (or teacher, or plan) picks from the frontier.

### Root Nodes

Nodes with `prereqs: []` are entry points. A brand-new student's frontier is ALL root nodes. Multiple roots are expected — a student doesn't have to start with place value; they could start with basic geometry or patterns.

For students who aren't beginners (your 8, 10, 12-year-old who's a year ahead), a **placement diagnostic** marks nodes as mastered without working through them. The engine walks the DAG: "Show me you know multiplication facts" → mastered → frontier jumps past all nodes that only needed multiplication as a prereq.

Placement is a plan-level concern, not a graph-level concern. The graph says what depends on what. Where you START is about the student.

### Edges

Prerequisites are the product. Getting nodes wrong is recoverable (rename, split, merge). Getting edges wrong means kids hit walls (missing prereq) or waste time (unnecessary prereq).

Rules:
- **Necessary, not sufficient.** Edge A→B means "you cannot learn B without A." Not "A is helpful for B."
- **Direct, not transitive.** If A→B→C, don't add A→C. The engine computes transitivity.
- **Test by asking:** "Can a student who has mastered everything EXCEPT A still learn B?" If yes, no edge. If no, edge.

### Encompassing Relationships (Future)

An advanced topic implicitly exercises simpler sub-skills. A calculus problem exercises algebra. A proportion word problem exercises fraction arithmetic. The graph can encode this with an optional `encompasses` field on nodes:

```yaml
# Future: not yet implemented in the engine
id: alg.quad.formula
prereqs:
  - alg.quad.completing_square
encompasses:
  - ops.oo.basics: 0.3
  - alg.exp.combining: 0.5
  - alg.eq.two_step: 0.7
```

The weight (0-1) indicates how strongly practicing this node exercises the sub-skill. This enables **compressed reviews**: one advanced problem can knock out multiple simpler reviews. Instead of separately drilling order of operations, combining expressions, and two-step equations, a single quadratic formula problem exercises all three.

This is how the system avoids review overload as the graph grows. Without encompassing relationships, spaced repetition becomes a treadmill — thousands of nodes each demanding periodic review. With them, practicing at the frontier keeps foundations fresh.

### Spaced Repetition and Mastery Decay (Future)

Mastery isn't binary forever. A student who demonstrated fraction addition six months ago may have lost fluency. The engine will track mastery decay:

- Each mastered node has a **confidence** that decays over time
- Decay rate depends on how deeply the node was mastered and how often it's encompassed by advanced work
- When confidence drops below threshold, the node re-enters the frontier as a **review task**
- Reviewing an advanced node refreshes its prerequisites (via encompassing relationships)

The graph doesn't encode decay rates — those are engine configuration. The graph provides the structure (which nodes encompass which) that makes efficient review scheduling possible.

### Placement Diagnostics

For students who aren't beginners, the engine can run a **placement diagnostic** — binary-search the DAG to find the frontier quickly. Test a mid-level node; if mastered, skip everything below it. If not, drill down.

The graph structure makes this efficient: test `ops.mul.facts` (mid-graph). Mastered → mark it and all ancestors as mastered, test the next level up. Not mastered → test prerequisites to find the gap. A well-connected graph with ~100 nodes can be placed in ~10-15 questions.

Placement is a runtime operation, not a graph concern. But a well-structured graph (with meaningful prerequisite chains, not too shallow, not too deep) makes placement work well.

### Cross-Domain Edges

The whole point of the graph. A fractions node depends on a division node. An algebra node depends on fractions and geometry. The DAG doesn't care about directory boundaries.

```yaml
# graph/math/fractions/equivalence/equivalent.yaml
id: frac.eq.equivalent
prereqs:
  - frac.con.basics              # same domain
  - ops.mul.factors_and_multiples # cross-domain: operations
```

```yaml
# graph/math/algebra/quadratics/formula.yaml
id: alg.quad.formula
prereqs:
  - alg.quad.completing_square   # same domain
```

These cross-links are what lets a student skip ahead non-linearly. The graph knows exactly which geometry and algebra topics are actually needed for calculus, and which aren't.

### Validation

The engine loads ALL `.yaml` node files, builds one unified graph, and validates:

1. **All prereq IDs resolve** — every ID in every `prereqs` list exists as a node
2. **No cycles** — the graph is a valid DAG (topological sort succeeds)
3. **No orphans** — every node is either a root (no prereqs) or reachable from a root
4. **ID uniqueness** — no duplicate node IDs across all files
5. **ID matches path** — the three-part ID should correspond to the file's directory location

Validation runs at load time. If any check fails, the engine refuses to start. Bad graph = loud failure, not silent corruption.

## Plans (Separate Concern)

Plans are curated paths through the graph. They live outside the graph directory.

```
plans/
├── k12/
│   ├── 4th-grade.yaml           ← "typical 4th grade" = these node IDs
│   ├── 5th-grade.yaml
│   └── algebra-1.yaml
├── remedial/
│   ├── fraction-gaps.yaml       ← "catch up on fractions" path
│   └── algebra-readiness.yaml
└── accelerated/
    └── skip-to-algebra.yaml     ← skip what you can prove you know
```

A plan file:

```yaml
name: 4th Grade Common Core
description: Standard 4th grade math curriculum
nodes:
  - ns.pv.thousands
  - ns.pv.millions
  - ops.add.within_1000
  - ops.mul.facts
  - frac.con.basics
  - frac.eq.equivalent
  # ...
```

The engine computes the frontier from the graph. The plan **filters** it — "only show nodes in this plan." A teacher picks a plan, a student works through it, but the DAG enforces prerequisites regardless. A teacher can't assign equivalent fractions if the student hasn't mastered fraction basics.

Plans are forkable. A school takes `4th-grade.yaml`, removes some nodes, adds others, publishes their version. The graph underneath is the same shared public good.

## Student State (Separate Concern)

Student state is NOT in the graph. It's runtime data, per-student, stored separately.

The graph is static, public, git-committed — the open good.
Student state is dynamic, private, per-user — the runtime.

Student state includes:
- Per-node mastery beliefs (probabilities, not binary)
- Evidence log (responses, timestamps)
- Belief updates (Bayesian, using likelihood ratios from assessment responses)

The mastery model is evidence-based:
- Student answers a problem → evidence
- Correct answer on a hard problem → strong signal of mastery (high LR)
- Wrong answer on an easy problem → strong signal of gap (low LR)
- Posterior probability crosses threshold → node is "mastered," downstream nodes unlock

Mastery thresholds, likelihood ratios, and decay parameters are engine configuration, not graph data. The graph says WHAT to assess. The engine decides HOW MUCH evidence is enough.

## How the Engine Uses the Graph

```
Load all .yaml files from graph/
  → Build unified DAG (nodes + edges from prereqs)
  → Validate (resolve prereqs, detect cycles, check uniqueness)
  → Build prompt cascade (walk _prompt.yaml files per directory)

On get_frontier(student_id):
  → Find all nodes where ALL prereqs have mastery belief above threshold
  → Filter out already-mastered nodes
  → Optionally filter by active plan
  → Return ordered by topological position

On get_node(node_id):
  → Return node data + assembled prompt cascade for that node's path

On record_evidence(student_id, node_id, response):
  → Update mastery belief via Bayesian update
  → If belief crosses threshold: mark mastered, compute newly unlocked nodes
  → Return updated frontier
```

## Deliberate Practice and the Two-Sigma Effect

The graph exists to enable **deliberate practice** (Ericsson) at the **two-sigma level** (Bloom). One-on-one tutoring produces learning outcomes two standard deviations above traditional classroom instruction. The graph + LLM architecture achieves this:

1. **The graph keeps the student at the edge of ability.** Frontier computation ensures every task is challenging but achievable — all prerequisites are mastered, so the student has what they need, but the topic itself is new.

2. **The graph prevents wasted time.** No unnecessary prerequisites (strict "necessary, not sufficient" edges). No redundant review (encompassing relationships compress reviews). No pace limits (unlimited velocity for students who demonstrate mastery).

3. **The prompt cascade provides expert tutoring direction.** Each node carries pedagogical guidance calibrated to the topic and learner level. The LLM doesn't guess how to teach fractions — the graph tells it: "visual models first, concrete to symbolic, common misconception is larger denominator = larger fraction."

4. **Mastery gates ensure no gaps.** Every node is gated. Students cannot advance with unresolved gaps. This is what makes unlimited velocity safe — speed without shortcuts.

5. **The assessment types drive deliberate practice, not drill.** `explain` and `derive` require deep understanding. `word_problem` requires transfer. `generate` requires creativity. The `assess` field tells the LLM to vary the cognitive demand, not just repeat the same procedure.

The graph structure directly enables the learning science. Bad edges = students hitting walls or wasting time. Missing nodes = gaps that surface later. Weak context = generic tutoring instead of expert-level guidance. The graph is the product.

## Design Principles

**The graph is instructions for the LLM, not content.** The LLM generates all teaching, problems, and feedback. The graph tells it what to teach, in what order, how to assess, and what pedagogical approach to use. The graph controls the LLM.

**Keep nodes lean.** The `context` field is terse expert guidance, not scripted dialogue. "Visual models before notation" not "Show the student a picture of a pizza cut into 8 slices and ask them to identify 3/8." The LLM is good at generating specifics from direction.

**Domain-organized, not grade-organized.** The mathematical structure of "fractions build on fractions" is more useful than "4th grade comes before 5th grade." Grades are metadata (`typical_grade`), not the hierarchy. Any learner can enter anywhere their mastery allows.

**The graph is a hypothesis.** Ship at 80% correct. Real students hitting real walls reveal missing edges. Real students breezing through reveal unnecessary edges. The feedback loop improves the graph.

**Fork-friendly.** A Montessori school replaces `_prompt.yaml` files. A homeschool parent adds nodes. A curriculum designer creates a new plan. The structure supports all of this through standard git operations: fork, branch, PR, merge.

## File Format Summary

| File | Purpose | Part of DAG? |
|------|---------|--------------|
| `*.yaml` (in graph/) | Node definition | Yes — each is a node |
| `_prompt.yaml` | System prompt for that directory level | No — prompt cascade only |
| `*.yaml` (in plans/) | Curated path through the graph | No — filters the frontier |

Everything is plain text. Everything is git-diffable. Everything is forkable. No database. No infrastructure. Just files.

## Current Graph Stats

As of March 2026:

- **131 nodes** across 9 domains
- **~210 edges** (prerequisite relationships)
- **2 root nodes** (`ns.pv.thousands`, `geo.ang.basics`)
- **Grade span**: 4–11
- **Bloom levels**: understand, apply, analyze
- **Longest chain**: ~15 nodes deep (arithmetic → trig identities)
- **19 prompt files** (1 subject + 18 domain/unit level)

Domains: number-sense (7), operations (17), fractions (12), decimals (5), geometry (26), ratios-proportions (6), statistics (12), algebra (38), trigonometry (8)

The graph is a work in progress. Target: ~3,000 nodes covering 4th grade through university (calculus, linear algebra, differential equations, discrete math, real analysis). Community contributions welcome.
