# 003: Engine Core — Domain Logic

Pure Rust. Load graph, load student state, compute frontier. No adapters yet.

## Deliverable

`engine/src/` — Rust crate with core domain logic.

## API

```rust
fn load_graph(path: &Path) -> Graph
fn load_progress(path: &Path) -> StudentState
fn get_frontier(graph: &Graph, state: &StudentState) -> Vec<Node>
fn get_node(graph: &Graph, id: &str) -> Node
fn record_mastery(state: &mut StudentState, node_id: &str, level: BloomLevel) -> Vec<NodeId>  // returns newly unlocked
fn get_progress(state: &StudentState) -> StudentState
```

## Data structures

```rust
HashMap<NodeId, Node>        // fast lookup
Vec<NodeId>                  // topological order (precomputed)
HashMap<NodeId, Vec<NodeId>> // children (reverse edges)
```

## Depends on

- 001 (schema — need to know what to deserialize)

## Done when

- Loads 4th grade JSON, computes frontier for empty student state
- `cargo test` passes
- No adapter code — pure domain logic
