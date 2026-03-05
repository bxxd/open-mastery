# 005: Generate Remaining Courses

All 17 courses, bottom up. Cross-course edges are where the real value is.

## Deliverable

`graph/math/` — all course files with cross-course prerequisite edges.

## Courses (approximate order of generation)

1. 4th-grade (done in 002)
2. 5th-grade
3. prealgebra
4. algebra-1
5. geometry
6. algebra-2
7. precalculus
8. trigonometry
9. calculus-1 (AB)
10. calculus-2 (BC)
11. linear-algebra
12. multivariable-calculus
13. probability-statistics
14. discrete-math
15. methods-of-proof
16. differential-equations
17. math-for-ml

## Cross-course edge examples

- algebra-2 trig identities → geometry angle relationships
- probability → algebra-1 combinatorics
- calculus-1 limits → precalculus rational functions
- linear-algebra → algebra-2 systems of equations

## Depends on

- 001 (schema)
- 002 (proves the format works)

## Done when

- All courses generated and validated against schema
- Cross-course edges exist and make sense
- Full DAG topological sort produces a sane K-12+ learning path
