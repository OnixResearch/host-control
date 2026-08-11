## Why

Host Control owns transport-neutral topology, bounded observations, layouts, events, waits, errors, feature negotiation, and snapshot-first streams. It does not define a generic read-only inspection envelope that lets products expose lifecycle, queue, process, session, workflow, actor, and evidence summaries without transferring domain authority.

Rivet's actor and agent inspectors provide a useful UX reference for one place to examine state, workflows, events, files, and processes. Host Control needs only the product-neutral snapshot and cursor contract. Product schemas, truth, mutation, persistence, and rendering must remain consumer-owned.

## What Changes

- Add typed inspection source, subject, snapshot, section, item, relation, cursor, continuation, freshness, and redaction contracts.
- Require snapshot-first reads with exact source generation and revision fencing.
- Support bounded section and item pagination without generic unbounded maps or raw payloads.
- Keep product-owned domain kinds and payload DTOs behind explicit adapter projections.
- Separate read-only inspection from mutation intents and authority.
- Add Animus, Lattice, Molten, and Tile mapping fixtures without runtime dependencies.

## Impact

- **Files**: `host-control-core` DTOs and pure validators, optional codecs, consumer mapping fixtures, docs, and `host-control-core` requirements.
- **Testing**: Positive snapshot and continuation cases plus negative stale cursor, unknown source, duplicate section, ambiguous relation, oversized page, unsafe payload, and mutation-confusion cases.
- **Boundary**: Host Control validates inspection structure and bounded navigation. It does not prove source truth, transport delivery, persistence, authorization, effect success, or product readiness.
