# Inspection Snapshot Contracts Delta

## ADDED Requirements

### Requirement: Inspection contracts are transport-neutral and authority-free

r[host_control.inspection.boundary] Host Control MUST define read-only inspection contracts without filesystem, process, environment, network, clock, terminal, daemon, UI, transport, persistence, or product mutation effects.

#### Scenario: Supplied snapshot is validated

- GIVEN a consumer supplies an in-memory inspection snapshot
- WHEN Host Control validates it
- THEN validation returns deterministic structural facts without host effects

### Requirement: Inspection snapshots use typed bounded structure

r[host_control.inspection.model] Host Control MUST define typed source, subject, snapshot, section, item, relation, cursor, continuation, freshness, redaction, feature, and limit values. Generic unbounded maps and opaque raw payload dumps MUST NOT be the canonical inspection shape.

#### Scenario: Complete snapshot is accepted

- GIVEN a snapshot has one source identity, generation, revision, policy, bounded sections, items, and relations
- WHEN model validation runs
- THEN Host Control admits the structure for further consumer projection

#### Scenario: Duplicate identity fails closed

- GIVEN a snapshot repeats a section or item identity within its scope
- WHEN model validation runs
- THEN Host Control denies the snapshot

### Requirement: Pagination is snapshot-first and revision-fenced

r[host_control.inspection.validation] Every inspection page and continuation MUST bind the exact source identity, source generation, snapshot revision, policy identity, redaction profile, and selected limits. Stale or mismatched continuations MUST fail closed.

#### Scenario: Matching continuation advances

- GIVEN a continuation matches the current source generation and snapshot revision
- WHEN the next bounded page is validated
- THEN the page can extend the same snapshot view

#### Scenario: Stale continuation is rejected

- GIVEN a continuation names an older generation or revision
- WHEN the next page is validated
- THEN Host Control rejects it instead of mixing snapshots

### Requirement: Inspection relations are explicit and bounded

r[host_control.inspection.relations] Relations MUST use typed source and subject references with declared relation kinds, direction, and scope. Dangling, ambiguous, cyclic-when-forbidden, or over-limit relations MUST fail validation.

#### Scenario: Cross-section relation is valid

- GIVEN two admitted items in the same snapshot have a declared supported relation
- WHEN relation validation runs
- THEN the relation remains an explicit reference rather than copied domain data

### Requirement: Inspection has no mutation variants

r[host_control.inspection.read_only] The core inspection contract MUST NOT include execute, approve, retry, cancel, edit, delete, or arbitrary command variants. Consumers MUST use separate product-owned mutation contracts and authority.

#### Scenario: Mutation intent is rejected

- GIVEN a purported inspection item contains a mutation or command request
- WHEN validation runs
- THEN Host Control rejects the item as outside the inspection contract

### Requirement: Structural codecs revalidate decoded values

r[host_control.inspection.codecs] Optional Serde and Preserves codecs MAY encode inspection values, but decoded values MUST re-enter the same core validation before use.

#### Scenario: Malformed decoded value fails

- GIVEN encoded data decodes structurally but violates snapshot limits or identity rules
- WHEN core validation runs
- THEN the value is rejected

### Requirement: Consumer mappings retain product semantics

r[host_control.inspection.consumers] Animus, Lattice, Molten, Tile, and other consumers MUST retain domain payload meaning, source truth, persistence, authorization, and mutation. Host Control mappings MUST reject lossy or authority-promoting projections.

#### Scenario: Product mapping preserves ownership

- GIVEN a product projects a domain summary into an inspection item
- WHEN the mapping fixture runs
- THEN product identity and non-claims remain visible and Host Control gains no domain authority

### Requirement: Inspection contracts have positive and negative verification

r[host_control.inspection.verification] The focused rail MUST cover valid snapshots and negative stale, duplicate, ambiguous, oversized, unsafe, codec, mutation, and consumer-mapping cases.

#### Scenario: Focused rail passes

- GIVEN all required core and consumer fixtures are present
- WHEN the focused rail runs
- THEN it reports structural conformance and exact non-claims
