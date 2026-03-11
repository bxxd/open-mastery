# Tasks

## Ship Path

| # | Task | Depends on | Status |
|---|------|-----------|--------|
| 001 | [Define graph schema](001-schema.md) | — | **done** |
| 002 | [Generate first course (4th grade)](002-first-course.md) | 001 | **done** |
| 003 | [Engine core — domain logic](003-engine-core.md) | 001 | **done** |
| 004 | [MCP server adapter](004-mcp-adapter.md) | 003 | **done** |
| 005 | [Generate remaining courses](005-remaining-courses.md) | 001, 002 | **done** (103 nodes, 8 domains) |
| 006 | [Web frontend](006-web-frontend.md) | 003 | todo |
| 007 | [Graph cleanup — transitive edges & bloom levels](007-graph-cleanup.md) | 005 | **done** |
| 008 | [Expand statistics domain](008-statistics-expansion.md) | 005 | **done** (3 → 12 nodes) |
| 009 | [Upper grades — trig, logs, rational expressions](009-upper-grades.md) | 005 | **done** (19 new nodes) |
| 010 | [Encompassing relationships](010-encompassing-relationships.md) | 007 | **done** (schema + engine + MCP + 10 annotated nodes) |

## What's Done

- `docs/GRAPH_FORMAT.md` — Full specification for YAML node format
- `graph/math/` — 131 nodes across 9 domains (number-sense, operations, fractions, decimals, geometry, ratios-proportions, statistics, algebra, trigonometry), grades 4-11, with encompassing relationships
- `engine/crates/core/` — Loads YAML graph, computes frontier, records mastery, validates DAG, mutations, prompt cascade, encompasses reverse index. 17 tests.
- `engine/crates/mcp-student/` — Lean tutoring MCP: get_frontier, get_node, record_mastery, get_progress. Dual transport (stdio + SSE).
- `engine/crates/mcp-teacher/` — Full authoring MCP: 18 tools for browse, CRUD, prompts, git, artifacts. Dual transport.
- `.mcp.json` — Configured for student (stdio + SSE) and teacher (stdio)

## What's Next

- 006: Web frontend (graph explorer, progress dashboard)

## Dependency Graph

```
001 (done) → 002 (done) → 005 (done) → 007 (done) → 010 (done)
                                      → 008 (done)
                                      → 009 (done)
001 (done) → 003 (done) → 004 (done) → first usable moment ✓
                         → 006 (todo)
```
