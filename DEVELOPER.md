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

The field is wide open. No public 3,000-node JSON dump. No reference engine. No GitHub org with community PRs. This is the biggest untapped education unlock since Khan Academy.

## The Key Insight: The Graph Is Reconstructable

The graph is not secret knowledge. It's the consensus of how math builds on itself — every textbook encodes it implicitly in chapter ordering. The work is making it explicit and granular. We can construct it:

1. **Bootstrap with LLMs** — "List every topic in Algebra 1. For each, list prerequisites." Do this for all 17 courses. Get 80% of the graph in a day.
2. **Refine the 20%** — Cross-course edges, granularity tuning, encompassing relationships. This is the curriculum expertise work.
3. **Community improves it** — Like Wikipedia. Doesn't need to be perfect on day one. Needs to be open and improvable.

## Core Concepts

### Knowledge Frontier

A student's position on the DAG — not a single "current topic" but the full boundary of what they've mastered. The engine computes the optimal next task from this frontier for maximum learning velocity.

### Mastery Gates

Strict. You demonstrate mastery or you don't advance. No partial credit at the gate level. No skipping. This is the constraint that makes unlimited velocity safe.

### Encompassing Relationships

Advanced topics implicitly practice simpler sub-skills. A calculus problem exercises algebra. The engine uses these weights to compress reviews — one advanced problem can knock out multiple simpler reviews.

### Spaced Repetition with Decay

Mastery isn't binary forever. Confidence decays over time. The student model tracks last-assessed timestamps and schedules reviews before mastery drops below threshold. Hierarchical — reviewing an advanced topic refreshes its prerequisites.

## Architecture

```
graph/          Knowledge graph data (JSON). Nodes are topics, edges are prerequisites.
engine/         Rust crate. Traversal, student model, spaced repetition.
cli/            CLI interface. Pick a student, get the next topic, record mastery.
```

**The Graph** — Each node: topic name, course, difficulty, prerequisite edges, encompassing weights, assessment criteria. Community-editable, versioned like Wikipedia.

**The Engine** — Given a student's mastery state and the graph, computes the knowledge frontier and selects the optimal next task. Topological sort + mastery gates + spaced repetition with decay + review compression via encompassing relationships.

**Student State** — Per-student, per-node: mastery level, last assessed, confidence decay curve. JSON to start.

**The Tutor** — Not in this repo. That's whatever LLM you point at it. The engine says "teach this topic" and "assess with these problem types." The tutor is already built — Claude can explain any math topic, generate problems at any difficulty, check work, and adapt.

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
- **Graph format**: JSON (serde)
- **Student state**: JSON (file-based to start)
- **CLI**: clap
- **Tutor integration**: LLM API calls (Claude/OpenAI) — separate concern

## License

AGPL-3.0. This is a public good. If you use it or improve it, your improvements stay open too. The knowledge graph of mathematics belongs to everyone — no one gets to lock it back up behind a paywall.
