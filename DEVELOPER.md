# Open Mastery

An open-source knowledge graph and mastery engine for mathematics. The map, not the territory.

## What This Is

A public good: a machine-readable DAG (directed acyclic graph) of math topics from 4th grade through university, paired with a traversal engine that answers one question — **"what should this student learn next?"**

The graph encodes what every math textbook knows implicitly: which concepts depend on which. Open Mastery makes that structure explicit, versioned, and forkable.

An LLM (Claude, GPT, whatever) handles the teaching and assessment. This project provides the **structure** — the optimal path through the material, the mastery gates, the spaced repetition. The system enforces one rule: you must demonstrate mastery before you advance. No shortcuts, no pace limits.

## Why

Math Academy proved the architecture works — kids going from pre-algebra to Calc BC in 18 months. But the knowledge graph is proprietary. Khan Academy made content free but the mastery system is weak. The missing piece is an open, rigorous prerequisite graph that anyone can use, fork, and improve.

This is that graph, plus the engine to traverse it.

## Architecture

```
graph/          Knowledge graph data (JSON). Nodes are topics, edges are prerequisites.
engine/         Rust crate. Traversal, student model, spaced repetition.
cli/            CLI interface. Pick a student, get the next topic, record mastery.
```

**The Graph** — Each node: topic name, course, difficulty, prerequisite edges, assessment criteria. Community-editable, versioned like Wikipedia.

**The Engine** — Given a student's mastery state and the graph, computes the optimal next topic. Topological sort + mastery gates + spaced repetition with decay.

**Student State** — Per-student, per-node: mastered or not, last assessed, confidence decay. JSON to start.

**The Tutor** — Not in this repo. That's whatever LLM you point at it. The engine says "teach this topic" and "assess with these problem types."

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

TBD — intended to be a public good. Open source license to be selected before going public.
