## Phase 1: Core inspection envelope

- [x] [serial] Pin the reviewed Rivet Inspector source and documentation revision, record selected UX concepts and license, and add the bounded reference to `README.md`. r[host_control.inspection.boundary]
- [x] [serial] Add no-std inspection source, subject, snapshot, section, item, relation, cursor, continuation, freshness, redaction, feature, and limit DTOs. r[host_control.inspection.model]
- [x] [depends:host_control.inspection.model] Implement pure validation for identities, uniqueness, bounds, revision fencing, continuation, relations, source status, and read-only shape. r[host_control.inspection.validation]
- [x] [serial] Add optional Serde and Preserves structural codecs that re-enter core validation. r[host_control.inspection.codecs]

## Phase 2: Consumer mappings and evidence

- [x] [parallel] Add positive fixtures for single-source snapshots, bounded sections, continuations, partial source status, and typed relations. r[host_control.inspection.verification]
- [x] [parallel] Add negative fixtures for stale cursors, unknown sources, duplicates, ambiguous relations, oversized pages, unsafe payload classes, and mutation intents. r[host_control.inspection.verification]
- [x] [serial] Add frozen Animus, Lattice, Molten, and Tile projection fixtures without runtime dependencies. r[host_control.inspection.consumers]
- [x] [serial] Run no-std, Wasm, codec, consumer, Cairn, Clippy, and workspace checks, then document non-claims. r[host_control.inspection.verification]
