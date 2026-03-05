# Open Mastery

The open-source knowledge graph that every ed-tech company charges you $50/month to not give you access to.

**The structure of knowledge — what depends on what, what to learn next — is not proprietary.** It's the consensus of every textbook ever written. We just made it a YAML file you can fork.

## What This Is

A DAG of topics + a Rust engine that answers one question: **"what should this learner do next?"**

That's it. That's the product. Turns out that's also the hard part that nobody open-sources.

An LLM does the actual teaching (we're not reinventing that wheel). This project provides the **structure** — prerequisite graph, mastery gates, pedagogical guidance. You prove you know it, or you don't move on. No hand-waving. No "close enough." No participation trophies.

The engine is subject-agnostic. Math is the first graph (131 nodes, 4th grade through trig). But anything with prerequisite dependencies works — programming, music theory, chemistry, whatever. If knowledge builds on knowledge, this is your engine.

## The Story

Elena pours coffee and sits at the kitchen table. Her daughter Margot, seven, is already cross-legged on the couch with a tablet, talking to her AI tutor — a patient, slightly goofy character she named "Bix" months ago and has refused to rename since.

Bix is Claude underneath. But Bix doesn't decide what to teach. That comes from somewhere else.

Beneath the conversation, invisible to Margot, a single protocol connection hums. Bix is hooked into the Open Mastery graph — a sprawling lattice of human knowledge, open-source, versioned like software, maintained by thousands of contributors. Bix queries it dozens of times per session. *What has this child demonstrated? What's adjacent? What's the gentlest bridge from where she is to where the graph says she could go?*

Right now, Margot is multiplying single digits. The graph knows this not because someone told it but because Bix has been reporting micro-assessments upstream — little checkpoints woven into conversation so naturally Margot thinks she's just chatting. The graph's node for single-digit multiplication is green. Mastered. The six prerequisite nodes beneath it — addition fluency, skip-counting, commutative property — are green too, a little constellation of things Margot knows cold.

The graph also knows what's next. Not a single track, not a rigid sequence, but a *frontier* — the full set of nodes whose prerequisites Margot has met. Right now her frontier includes two-digit multiplication, intro to division, and — interestingly — area of rectangles, because that node depends on multiplication and spatial reasoning, and she's been doing both.

Bix picks area. Not randomly. The agent noticed Margot spent yesterday drawing floor plans of an imaginary bakery, so spatial concepts have high engagement probability. The graph doesn't choose *for* the agent. It tells the agent what's *available*. The agent chooses.

"Hey Margot," Bix says. "You know your bakery? What if we figured out how much floor space the kitchen takes up?"

Margot lights up.

---

Across town, Dev is fourteen and teaches himself. His AI agent is a custom rig he built on top of an open-weight model running on a machine in his closet. He calls it "Coach." Coach is blunt, fast, slightly sarcastic. Dev likes it that way.

Coach is connected to the same graph. Same protocol. Different endpoint — Dev's family runs the self-hosted version, because his mother is a software engineer and doesn't want her son's learning data on anyone else's servers. The graph data itself is just a cloned repo on a local drive.

Dev is deep into the graph. His math frontier is in eigenvalues and partial derivatives. His programming frontier is in memory management and graph traversal. His music frontier — because the graph has music now, a beautiful lattice donated by a retired theory professor in Vienna — is in secondary dominants.

He's working on eigenvalues this morning. Coach walks him through it, but Coach also knows, because the graph says so, that eigenvalues depends on matrix multiplication and determinants and geometric intuition for linear transformations, and that Dev's mastery of geometric intuition is borderline. So Coach weaves in re-anchoring. "Before we go further — when you apply this matrix to a vector, what happens *spatially*? Show me with a sketch."

Dev groans. Draws the sketch. Gets it. Moves forward.

This is the thing no static curriculum could do. The graph provides structure. The agent provides responsiveness. Together they create something that feels like a private tutor who has read every textbook and also knows exactly what *you* specifically are shaky on.

---

Pull back further. Four hundred thousand active learners connected to the Open Mastery graph. Some through the hosted endpoint. Some through self-hosted instances. Some through third-party apps — a micro-school platform in Austin, a language-learning startup in Seoul, an after-school program in Lagos that contributed the Yoruba literacy branch.

They all share the same structure underneath. When the micro-school in Austin marks a student as having mastered the quadratic formula, that means the same thing as when Dev's closet server marks it, because the node definition is the same, the prerequisites are the same, the assessment criteria are the same. A child can move from one system to another and their mastery travels with them — not as a transcript, not as a grade, but as a precise map of what they know and what they're ready for.

That's the bet: curriculum as shared infrastructure. Open, forkable, improvable. We'll see if it works.

> *Full narrative: [`docs/STORY.md`](docs/STORY.md)*

## The Math Graph

131 nodes. 9 domains. A start, not a finish — the real work is scaling to thousands of nodes while maintaining graph integrity. That's where every curriculum project dies: not at architecture, at curation. We're building in the open so others can help.

One YAML file per topic. Cross-domain edges because math doesn't care about your folder structure.

```
graph/math/
├── number-sense/          7 nodes     place value, rounding, patterns
├── operations/           17 nodes     the fundamentals you think you remember
├── fractions/            12 nodes     where most students' confidence goes to die
├── decimals/              5 nodes     fractions in a trench coat
├── geometry/             26 nodes     angles, shapes, area, volume, proofs
├── ratios-proportions/    6 nodes     the bridge to algebra
├── statistics/           12 nodes     making sense of data (and lying with it)
├── algebra/              38 nodes     expressions → equations → functions → quadratics → logs
└── trigonometry/          8 nodes     sin, cos, and "why do I need this"
```

Each node is a YAML file:

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

The `context` field tells the LLM *how* to teach. Not a script — direction. "Visual models before notation" not "Show the student a picture of a pizza." The LLM is smart enough to take it from there.

`_prompt.yaml` files cascade from subject → domain → unit → node. Want Montessori vibes? Fork the prompts. The DAG doesn't change.

## The Engine

Rust. Because when you have 3,000 nodes you don't want to wait for Python to think about it.

Two MCP servers share a core crate:

**Student MCP** — connect any LLM and start learning:

```
get_frontier(student_id)                    → "here's what you're ready for"
get_node(node_id)                           → topic details + how to teach it
record_mastery(student_id, node_id, level)  → new topics unlock
get_progress(student_id)                    → full mastery state
```

**Teacher MCP** — build curriculum by talking to Claude:

```
Browse:     list_domains, list_nodes, search_nodes, get_node, validate_graph
CRUD:       create_node, update_node, delete_node, add_prerequisite, remove_prerequisite
Prompts:    get_prompt_cascade, set_prompt
Git:        git_status, git_commit, git_log
Artifacts:  save_artifact, get_artifact, list_artifacts
```

Teachers literally just *talk* to build curriculum. "Add a node for completing the square." "It should require factoring trinomials." "Commit." Done. YAML on disk, git-versioned, no UI needed.

## How It Works

```
frontier(student) = {
  node for node in all_nodes
  if node not mastered by student
  AND all node.prereqs mastered by student
}
```

Seven lines of pseudocode. That's the entire algorithm. Everything else is plumbing.

The frontier is everything you've unlocked but haven't learned yet. Pick something. Demonstrate understanding. New stuff unlocks.

### Why This Works (When It Works)

The frontier isn't just an algorithm — it's Csikszentmihalyi's flow channel, computed from a graph. Flow state requires challenge matched to skill: too easy and you're bored, too hard and you're anxious. The frontier guarantees every available task is *achievable* (all prereqs mastered) but *novel* (the topic itself is new). That's the flow zone.

The drive to operate at the edge of your competence is one of the deepest human motivations. Kids don't need to be bribed into learning when every problem is at exactly the right difficulty. They don't need gamification. They need the thing games accidentally get right: a well-tuned difficulty curve. The DAG *is* the difficulty curve, personalized to each learner.

This is why mastery gates matter even though they slow things down. Without them, you get Khan Academy — students clicking through videos at 1.5x, "completing" topics they can't do. With them, velocity is real. A student moving fast through the graph actually knows the material, because the graph made them prove it at every step.

**The hard problem isn't the graph — it's verification.** Generating curriculum is cheap. Knowing whether a student *actually* understands completing the square, not just producing the right tokens? That's the moat. The `assess` types help (`explain`, `derive`, and `generate` are harder to fake than `solve`), and the mastery model uses Bayesian updates across multiple evidence points rather than single-shot assessment. But LLM-as-assessor is fundamentally an unsolved problem. We're building the scaffolding and working on it in the open.

## Why This Exists

Because someone asked "can I see the data for this graph?" and the answer was no.

The architecture — DAG of prerequisites + mastery gates + unlimited velocity — has been proven by proprietary platforms. The structure of what depends on what isn't secret. It's the consensus of every textbook ever written. The hard part was never the knowledge — it was making it explicit, machine-readable, and open.

Khan Academy made content free but the graph is implicit and their mastery system is... generous. Every other prerequisite graph is locked behind a paywall. Every homeschool parent with an LLM is rebuilding the same curriculum structure from scratch.

So here we are. An open YAML dump, a Rust engine, and a GitHub repo. Early, incomplete, and free.

## Quickstart

```bash
git clone https://github.com/bxxd/open-mastery.git
cd open-mastery
make release && make test    # build + verify
```

Add to your Claude config (`~/.claude/mcp.json` or `.mcp.json`):

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

Open Claude. Say "let's learn." That's it.

For HTTP/SSE (web clients, multi-user):

```bash
make run-student-sse    # port 3001
make run-teacher-sse    # port 3002
```

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

Hexagonal. Core is pure domain logic. Adapters are swappable crates. The engine doesn't know or care what's talking to it.

## Contributing

The graph is a hypothesis shipped at 80%. Students hitting walls = missing edges. Students breezing through = unnecessary edges. Real usage makes it better.

- **Add nodes** — new topics, new domains, deeper coverage
- **Fix edges** — the prerequisite relationships are the product
- **Improve context** — better teaching guidance for the LLM
- **Build new graphs** — programming, science, music, anything
- **Build adapters** — web frontend, CLI, mobile

See [`docs/GRAPH_FORMAT.md`](docs/GRAPH_FORMAT.md) for the graph spec. See [`DEVELOPER.md`](DEVELOPER.md) for architecture and philosophy.

## License

- **Graph data** (`graph/`) — [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Fork it, extend it, keep derivatives open.
- **Engine code** (`engine/`) — [MIT](https://opensource.org/licenses/MIT). Do whatever you want.
