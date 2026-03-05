# Open Math Graph — Builder Brief

## What You're Building

An open-source, machine-readable DAG of all K-12+ math knowledge. ~3,000 nodes, prerequisite edges, mastery assessment criteria. The structure that's been locked behind paywalls, made free.

## Why Now

LLMs eliminated the two historical barriers: (1) graph construction no longer requires years of expert curation — the prerequisite structure is textbook consensus, reconstructable by prompting, and (2) the tutor is free — Claude/GPT can explain, generate problems, and grade per-topic. The only missing piece is the graph itself. Zero repos exist. Empty field.

## Architecture

The graph is YAML files in a domain-organized hierarchy. One file per node. The filesystem is the schema.

```yaml
# graph/math/fractions/equivalence/equivalent.yaml

id: frac.eq.equivalent
prereqs:
  - frac.con.basics
  - ops.mul.factors_and_multiples
bloom: apply
assess:
  - identify
  - generate
  - explain
typical_grade: 4
context: >
  Equivalent fractions name the same amount. Multiply or divide
  numerator and denominator by the same number. Visual proof:
  same shaded area, different number of pieces.
```

The graph is a DAG. Edges = "you must master A before attempting B." Nodes are organized by mathematical domain, not grade level. Prerequisites cross domains freely. The full curriculum is one unified graph.

### Three Layers

```
graph/math/          The DAG. Nodes + prereqs + assessment + prompt cascade.
                     Static, public, git-committed. The open good.

plans/               Curated paths through the graph. "4th Grade Common Core,"
                     "Catch Up on Fractions," "Skip to Algebra." Forkable by
                     teachers. Filters the frontier, doesn't change the DAG.

student state        Per-student mastery beliefs. Dynamic, private, runtime.
                     Evidence in, probability out. Bayesian updates.
```

### Directory Structure

Organized by mathematical domain:

```
graph/math/
├── _prompt.yaml              ← subject-level teaching philosophy
├── number-sense/             ← domain
│   ├── place-value/          ← unit
│   │   ├── thousands.yaml    ← node
│   │   └── millions.yaml
│   ├── rounding/
│   └── patterns/
├── operations/
├── fractions/
├── decimals/
├── geometry/
├── ratios-proportions/
├── algebra/
├── trigonometry/
├── calculus/
└── linear-algebra/
```

### Node IDs

Three-part keys that mirror the directory path: `domain.unit.topic`

```
ns.pv.thousands          → number-sense/place-value/thousands.yaml
ops.mul.facts            → operations/multiplication/facts.yaml
frac.eq.comparing        → fractions/equivalence/comparing.yaml
alg.eq.quadratic_formula → algebra/equations/quadratic_formula.yaml
```

### Prompt Cascade

`_prompt.yaml` files at each directory level define HOW the LLM teaches. They cascade like `.gitignore` — subject → domain → unit → node context. A Montessori school forks the repo, replaces the top-level `_prompt.yaml`, and every node changes teaching style. The DAG stays the same.

## Critical Design Decisions

### 1. Nodes are instructions for the LLM, not content

The graph doesn't teach. The graph tells the LLM what to teach, in what order, how to assess, and what pedagogical approach to use. The `context` field is terse expert guidance: "visual models before notation." The LLM generates all teaching, problems, and feedback.

### 2. Granularity matters more than coverage

"Fractions" is too coarse. "Adding Fractions with Unlike Denominators" is the right grain. Each node is a single teachable concept that can be independently assessed. If you can't write 3 problems that test exactly this node and nothing else, the node is too big. Split it.

### 3. Prerequisites are the whole game

The edges ARE the product. Getting nodes wrong is recoverable (rename, split, merge). Getting edges wrong means kids hit walls (missing prereq) or waste time (redundant prereq).

Rules for edges:
- **Necessary, not sufficient.** Edge A→B means "you cannot learn B without A." Not "A is helpful for B."
- **Direct, not transitive.** If A→B→C, don't add A→C. The engine handles transitivity.
- **Test by asking:** "Can a student who has mastered everything EXCEPT node A still learn node B?" If yes, no edge. If no, edge.

### 4. Cross-domain edges are where the value is

Within a domain, sequencing is mostly linear and obvious. The insight is cross-domain: algebra needs fractions. Trigonometry needs geometry. These cross-links are what lets a kid skip ahead non-linearly — the third-grader doing Calc BC was possible because the graph knew exactly which geometry and algebra topics were needed, and which weren't.

### 5. Domain-organized, not grade-organized

Grades are metadata (`typical_grade: 4`), not the hierarchy. Math doesn't care how old you are. Fractions build on fractions. Algebra builds on algebra. An 8-year-old a year ahead just has a frontier that crosses into what schools call "5th grade territory." The graph doesn't care.

Plans provide grade-level views for teachers who need them: "4th Grade Common Core" is a list of node IDs. The plan filters the frontier. The DAG ignores it.

### 6. Assessment types per node

Not just "can you solve this." Bloom's taxonomy matters:
- **Know:** State the formula/definition
- **Understand:** Explain why it works, identify errors
- **Apply:** Solve standard problems
- **Analyze:** Solve novel/word problems, combine with other concepts

A node is "mastered" at Apply level minimum. Analyze is stretch. Know/Understand alone is not mastery — kids who can state the quadratic formula but can't use it haven't mastered the node.

### 7. The graph is a hypothesis, not a truth

Ship v1 knowing it's 80% right. Real students hitting real walls will reveal the missing edges. Real students breezing through will reveal the unnecessary edges. Build in feedback: "I couldn't do this, what am I missing?" = missing prerequisite edge. "I already knew this" = node should have been skippable given prior mastery.

Your kids are the moat. Every open source project that wins has a ruthless feedback loop. Kids hitting walls = missing edges. Kids breezing through = unnecessary edges. That's data that costs $49/month to collect behind a paywall. You get it at the dinner table.

The third-grader doing Calc BC isn't impressive because of the software. It's impressive because someone removed the artificial ceiling. That's all this project does — removes the ceiling and makes sure nobody can put it back.

## Generation Strategy

Domain by domain, bottom up. Start with foundational domains (number sense, operations), then build domains that depend on them (fractions, decimals), then higher domains (algebra, geometry, etc.). Each domain:

1. List all topics at the right granularity
2. For each topic, identify direct prerequisites (within domain AND cross-domain)
3. Validate: does the topological sort produce a sane learning order?
4. Sanity check against 2-3 real textbook TOCs

Cross-validate against: Common Core standards (topic lists), Khan Academy skill tree (implicit graph), AP curriculum frameworks (Calc AB/BC, Stats), textbook TOCs (Stewart, Blitzer, etc.)

## What the Traversal Engine Does

```
input:  graph + student_mastery_state + optional plan filter
output: ordered list of "learn next" nodes

algorithm:
  1. Find all nodes where ALL prerequisites are mastered
  2. Filter out already-mastered nodes
  3. Optionally filter by active plan
  4. Rank by topological position
  5. Return frontier
```

That's it. No ML, no adaptive algorithm, no spaced repetition (yet). Just topological sort filtered by mastery. A kid opens the app, sees "here's what you can learn next," picks one, Claude teaches it, kid demonstrates mastery, node unlocks children. Repeat.

For non-beginners: a **placement diagnostic** walks the graph, assesses key nodes, and marks them mastered. The frontier jumps to the right spot without working through everything from scratch.

Spaced repetition (review schedule for mastered nodes) is Phase 2. Adaptive diagnostics (placement testing) is Phase 3. The core loop works without them.

## Mastery Model

Mastery is Bayesian, not binary. Each node has a belief (probability the student has mastered it). Evidence updates the belief:

- Correct answer on a hard problem → high likelihood ratio → belief increases
- Wrong answer on an easy problem → low likelihood ratio → belief decreases
- Belief crosses threshold → node is "mastered" → downstream nodes unlock

Thresholds, likelihood ratios, and decay are engine configuration. The graph says WHAT to assess. The engine decides HOW MUCH evidence is enough.

## License

- **Graph data** (`graph/`): CC-BY-SA — copyleft, forces derivatives to stay open
- **Engine code**: MIT — maximum adoption

The graph is a public good. Ship it.
