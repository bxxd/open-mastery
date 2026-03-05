# 004: MCP Server Adapter

Wrap the engine core as an MCP server. This is the first usable interface — connect Claude Desktop and start tutoring.

## Deliverable

MCP server binary exposing engine tools and resources.

## Tools

```
get_frontier(student_id)                      → unlocked nodes ready to learn
get_node(node_id)                             → node details + prerequisites
record_mastery(student_id, node_id, level)    → update state, return newly unlocked
get_progress(student_id)                      → full mastery state
```

## Resources

```
graph://math/{course}        → course data
progress://{student_id}      → student state
```

## Depends on

- 003 (engine core)

## Done when

- `cargo run` starts MCP server
- Claude Desktop can connect and call all 4 tools
- A kid can actually learn a topic through Claude Desktop
