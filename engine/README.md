# Open Mastery Engine

Rust. Given a knowledge graph and a student's mastery state, answers one question: **"what should this student learn next?"**

## What It Does

```
input:  graph + student_mastery_state
output: ordered list of "learn next" nodes
```

1. Find all nodes where ALL prerequisites are mastered
2. Filter out already-mastered nodes
3. Rank by: course progression > prerequisite count > Bloom level
4. Return top N

No ML. No adaptive algorithm. Topological sort filtered by mastery state.

## Quickstart

```bash
# From repo root
cp .env.example .env
make test          # run all tests
make mcp-server    # start HTTP/SSE server on port 3001
make run           # start stdio transport (for .mcp.json)
```

See `mcp.json.example` for Claude Desktop / Claude Code configuration.

## Future

- **Phase 2**: Spaced repetition — review schedule for mastered nodes, confidence decay
- **Phase 3**: Adaptive diagnostics — placement testing, skip what you already know

The core loop works without them.

## License

MIT. Use it however you want.
