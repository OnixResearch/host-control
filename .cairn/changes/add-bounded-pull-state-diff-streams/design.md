## Goals

- Make pull-based Host Control state synchronization deterministic and bounded.
- Keep exact base revision, target revision, producer, schema, policy, and limit facts visible.
- Require complete validated assembly before a snapshot or difference becomes current.
- Keep all effects and product meaning outside the core.

## Architecture

The pure core receives already-loaded requests, prior accepted state, producer results, transfer parts, and explicit limits.

The core returns admission, assembly, apply, resnapshot, or rejection decisions. It performs no network, clock, file, process, retry, queue, or persistence operation.

Consumer shells schedule polls and obtain producer results. Transport adapters encode and move admitted values.

## Decisions

### Decision: The client controls update demand

**Choice:** A pull request names the last accepted revision and explicit response limits. A producer returns one bounded result for that request.

**Rationale:** Paused clients do not create unbounded server queues.

### Decision: Results have closed classes

**Choice:** Results are `NoChange`, `Delta`, `Snapshot`, `ResnapshotRequired`, or `ProducerFailed`.

**Rationale:** No-change, stale state, and producer failure have different recovery behavior.

### Decision: Every delta binds its base

**Choice:** A delta binds exact base and target revisions plus schema, producer, policy, and implementation identities.

**Rationale:** A valid difference for one state is unsafe for another state.

### Decision: Host Control owns only host-state difference structure

**Choice:** Delta entries use stable Host Control resource identities and typed add, replace, or remove operations.

**Rationale:** Generic product diff meaning belongs to product owners.

### Decision: Segmented results are incomplete until validated

**Choice:** Parts bind one transfer identity, ordered index, declared count, complete identity, and aggregate limits.

The assembler rejects gaps, duplicates, mixed transfers, excess parts, excess bytes, and changed identities. It exposes no partial current state.

**Rationale:** Incremental transport must not create partial semantic state.

### Decision: Producer errors remain terminal results

**Choice:** A producer failure has a stable bounded class and optional safe metadata. It cannot become `NoChange`.

**Rationale:** Hidden producer errors make stale state look current.

### Decision: Feature negotiation selects pull mode

**Choice:** Existing feature negotiation identifies supported pull, delta, segmentation, and resnapshot capabilities.

**Rationale:** A consumer must not infer support from a decodable message.

## Verification

Positive fixtures cover no-change, one delta, one full snapshot, segmented assembly, and explicit resnapshot.

Negative fixtures cover stale bases, duplicate resource operations, invalid ordering, unknown schema, crossed policy, gaps, duplicates, overflow, and producer failure.

Conformance includes a consumer mapping fixture that proves no transport or product type enters `host-control-core`.

## Risks and limits

A structurally valid delta can contain false producer observations. Host Control does not prove source truth.

Polling can increase latency or load. Consumer shells own schedules, deadlines, retries, and admission policy.

A complete round trip proves only the declared structural contract. It does not prove product meaning or release readiness.
