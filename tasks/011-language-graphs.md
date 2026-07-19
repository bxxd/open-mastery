# 011: Language graphs — English spelling + Japanese kana

Second subject: `graph/language/`. Proves the engine is subject-agnostic (math is the
beachhead, not the product) and ships with real consumers: two live learner deployments
via a canvas app that consumes graph exports (private deployment; the app's generic
engine is a candidate for extraction into this repo later).

## Deliverable

- `graph/language/english-spelling/` — phoneme→grapheme mastery as a DAG:
  - phonemic awareness primitives (segmenting, blending, elision)
  - syllable types (closed, open, silent-e, vowel team, r-controlled, consonant-le)
  - digraphs/trigraphs (ch, sh, th, ck, tch, dge, …)
  - orthographic rules as nodes (doubling, e-drop, y→i, plural rules)
  - morphology region stub (prefix/root/suffix construction — grows later)
- `graph/language/japanese-kana/` — hiragana as a DAG:
  - kana rows as node clusters (vowels → k-row → s-row …), dakuten/handakuten,
    digraph kana (きゃ …), sokuon/chōon
  - vocab nodes gated on kana coverage (a word is available when its kana are mastered)
- Both validate against `graph/schema.json` (extend schema only if a language need
  genuinely can't be expressed — flag, don't fork).
- JSON export consumable by external apps (same shape the math graph exports).

## Approach

Same as 002: LLM-bootstrap the node list at tight granularity (the "3 problems that
test exactly this node" rule), then refine edges against 2–3 published sequences
(Orton-Gillingham scope-and-sequence for spelling; standard kana pedagogy for
Japanese). Spelling granularity guide: if a node can't generate a 5-word dictation set
that isolates it, split it.

## Depends on

- 001 (schema), 003 (engine core — frontier over these graphs)

## Done when

Both graphs validate, topological sort yields a sane teaching order a teacher would
recognize, and the consumer app runs rounds off the exports with zero graph-side
special-casing per subject.
