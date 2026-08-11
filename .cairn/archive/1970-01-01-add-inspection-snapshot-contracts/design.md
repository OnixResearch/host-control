## Context

Consumers can expose host topology and bounded observations through Host Control, but each product invents separate inspector framing, cursor behavior, and stale-snapshot handling. A shared envelope can reduce UI and transport duplication if it does not absorb product semantics.

Rivet's Inspector is a design reference for grouped read-only views. Its dashboard, actor schema, session schema, storage, REPL, and mutation APIs are outside this change.

## Decisions

### Decision: Own the envelope, not domain payload meaning

**Choice:** Host Control defines source, subject, snapshot, section, item, relation, cursor, limits, freshness, redaction, and feature facts. Product adapters retain typed domain projections and interpretation.

**Rationale:** A universal key-value inspector would erase semantic and claim boundaries.

### Decision: Snapshot first and revision fenced

**Choice:** Every page binds one source identity, source generation, snapshot revision, policy identity, and page limits. Continuations must match those facts exactly.

**Rationale:** Mixing pages from different source revisions creates false relationships and impossible states.

### Decision: Read-only by construction

**Choice:** Inspection contracts contain no execute, approve, retry, cancel, edit, or arbitrary command variant. Products can offer separate mutation intents with their own authority.

**Rationale:** Viewing a resource must not grant control over it.

### Decision: Bound and classify visible content

**Choice:** Items expose stable safe fields, typed relation references, status, counts, and product-projected summaries under explicit byte and item limits. Raw transcripts, commands, credentials, environment values, physical paths, bearer tokens, and opaque payload dumps are outside the core contract.

**Rationale:** A common inspector must not become a cross-product data exfiltration path.

## Risks / Trade-offs

- Too little structure can force every renderer to understand every product. Mapping fixtures must prove the common envelope is useful.
- Too much structure can move actor or workflow semantics into Host Control. Domain fields must remain product-owned.
- Partial source failures can produce misleading aggregate screens. Each source and section needs an explicit status.
