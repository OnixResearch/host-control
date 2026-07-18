# Host Control Core Specification

## Purpose

Own product-neutral deterministic host-control protocol semantics without host authority.

## Requirements

### Requirement: Core remains deterministic and authority-free

r[host_control.core.purity] `host-control-core` MUST remain `no_std` plus `alloc` and MUST NOT access filesystem, process, environment, network, clock, terminal, daemon, UI, transport, or persistence effects.

#### Scenario: Wait resolution uses supplied events
- GIVEN a validated wait and finite supplied events
- WHEN resolution runs
- THEN the core MUST return a deterministic outcome without host effects.

### Requirement: Selectors preserve lifetime and revision

r[host_control.core.selectors] Selectors MUST bind opaque identity, lifetime, host scope, and revision where positional references are used.

#### Scenario: Positional revision drifts
- GIVEN an index from another scope or revision
- WHEN selector validation runs
- THEN the core MUST reject it as stale.

### Requirement: Observations are bounded and redacted

r[host_control.core.observation] Observations MUST carry finite line and byte budgets, source, truncation, redaction, measured counts, and optional BLAKE3 metadata.

#### Scenario: Unbounded output is requested
- GIVEN no finite budget or unsafe raw output
- WHEN observation admission runs
- THEN the core MUST reject the request.

### Requirement: Optional features are negotiated

r[host_control.core.features] Layout and snapshot-first streaming MUST require explicit feature negotiation.

#### Scenario: Incremental update precedes snapshot
- GIVEN snapshot-first is negotiated but no snapshot was accepted
- WHEN an incremental update arrives
- THEN the core MUST reject stream ordering.

### Requirement: Consumer mappings are loss-aware

r[host_control.core.mappings] Animus and Tile mappings MUST preserve native identity and status distinctions or return a deterministic loss diagnostic.

#### Scenario: Tile receives failed status
- GIVEN Tile's profile has no exact failed state
- WHEN mapping runs
- THEN the core MUST reject rather than map failure to completion.

### Requirement: Serialization is optional and revalidated

r[host_control.core.serialization] Optional serde or Preserves adapters MUST revalidate decoded DTOs and MUST NOT introduce transport or daemon authority.

#### Scenario: Structurally decoded payload is invalid
- GIVEN decoded values contain an invalid selector, feature, timeout, or output bound
- WHEN adapter validation runs
- THEN the payload MUST be rejected before admission.

### Requirement: Consumer conformance has positive and negative evidence

r[host_control.core.conformance] Conformance MUST cover Animus and Tile mappings plus stale selectors, unbounded reads, lossy status, stream ordering, unsupported features, and malformed values.

#### Scenario: Reviewed fixtures run
- GIVEN positive and negative consumer fixtures
- WHEN conformance tests run
- THEN supported mappings MUST pass and invalid behavior MUST fail deterministically.

### Requirement: Claims remain bounded

r[host_control.core.non_claims] Release evidence MUST NOT claim daemon correctness, transport security, PTY preservation, terminal truth, policy authorization, arbitrary compatibility, or whole-product readiness.

#### Scenario: Conformance passes
- GIVEN all deterministic checks pass
- WHEN evidence is reported
- THEN it MUST retain all protocol and product non-claims.
