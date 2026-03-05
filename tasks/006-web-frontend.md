# 006: Web Frontend

Graph explorer, progress dashboard, embedded chat, graph editor. Connects to the same engine core via HTTP adapter.

## Deliverable

Web app — start locally, opens in browser.

## Views

- **Explorer**: Browse the full DAG. Zoom, pan, filter by course. See prerequisite chains. Click a node to see details.
- **Student dashboard**: Knowledge frontier, progress, mastery history. What's unlocked, what's next.
- **Tutor**: Embedded chat with LLM. Agent at the bottom (idio-style). Teaches, assesses, records mastery.
- **Graph editor**: Add/edit/remove nodes and edges. The Wikipedia editing experience for math curriculum. Validation feedback (cycle detection, orphan nodes).

## Depends on

- 003 (engine core)
- Needs an HTTP adapter (sibling to MCP adapter, same core)

## Done when

- `cargo run` serves web app on localhost
- Can browse the graph, see a student's frontier, and edit nodes/edges
- Chat interface works with Claude API
