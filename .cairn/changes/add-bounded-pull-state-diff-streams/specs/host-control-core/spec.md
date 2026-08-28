# Bounded Pull-State Difference Streams Delta

## ADDED Requirements

### Requirement: Pull-state synchronization keeps product authority outside Host Control

r[host_control.pull_state.boundary] Host Control MUST define only bounded host-state synchronization structure. Product meaning, polling schedules, transport, retries, storage, and clocks MUST remain consumer-owned.

#### Scenario: A consumer supplies explicit facts

- GIVEN a consumer supplies an admitted request, prior revision, producer result, identities, and limits
- WHEN the pure core evaluates synchronization
- THEN it MUST return a deterministic decision without I/O or ambient state

#### Scenario: Product state enters the core

- GIVEN a proposed delta depends on a product DTO or vendor transport type
- WHEN architecture validation runs
- THEN it MUST reject that dependency as outside Host Control

### Requirement: Pull-state values use typed bounded structure

r[host_control.pull_state.model] Requests and results MUST bind source, generation, prior revision, schema, producer, implementation, policy, features, and explicit response limits.

#### Scenario: A complete request is admitted

- GIVEN every required identity and non-zero limit is present
- WHEN request admission runs
- THEN it MUST admit the request with the exact supplied facts

#### Scenario: A request omits a bound

- GIVEN a request omits a part, byte, entry, or response bound
- WHEN request admission runs
- THEN it MUST reject the request before producer dispatch

### Requirement: Pull behavior requires explicit negotiation

r[host_control.pull_state.negotiation] Pull, delta, segmentation, and resnapshot capabilities MUST require explicit existing feature negotiation.

#### Scenario: Both peers support the selected profile

- GIVEN the producer and consumer negotiated one compatible pull profile
- WHEN a pull request is admitted
- THEN the request MAY use only the negotiated capabilities and limits

#### Scenario: A decodable unnegotiated delta arrives

- GIVEN a delta decodes but delta support was not negotiated
- WHEN result admission runs
- THEN it MUST reject the result as unsupported

### Requirement: Pull results have closed outcome classes

r[host_control.pull_state.result] A producer result MUST be no-change, delta, snapshot, resnapshot-required, or producer-failed. One class MUST NOT silently become another class.

#### Scenario: Producer state is unchanged

- GIVEN the current producer revision equals the last accepted revision
- WHEN the producer returns no-change
- THEN the consumer state MUST remain current at that exact revision

#### Scenario: Producer evaluation fails

- GIVEN the producer returns a bounded failure
- WHEN result admission runs
- THEN the result MUST remain producer-failed and MUST NOT become no-change

### Requirement: Deltas are exact base-to-target transitions

r[host_control.pull_state.delta] A delta MUST bind exact base and target revisions. Operations MUST use unique stable resource identities and typed add, replace, or remove classes.

#### Scenario: A valid delta advances state

- GIVEN every operation applies once to the exact accepted base revision
- WHEN pure delta application runs
- THEN it MUST return the exact target revision and updated host state

#### Scenario: The base revision is stale

- GIVEN a delta names a base revision different from the accepted revision
- WHEN pure delta application runs
- THEN it MUST return resnapshot-required without partial mutation

#### Scenario: Operations conflict

- GIVEN a delta repeats an identity or applies an impossible add, replace, or remove operation
- WHEN delta validation runs
- THEN it MUST reject the complete delta

### Requirement: Segmented transfer is complete before apply

r[host_control.pull_state.segmentation] Segments MUST bind one transfer identity, ordered indices, declared part count, complete identity, and aggregate limits. Partial assemblies MUST NOT become current state.

#### Scenario: All parts arrive exactly once

- GIVEN all bounded parts have one identity, complete order, and matching aggregate identity
- WHEN assembly validation runs
- THEN it MUST return one complete result for normal admission

#### Scenario: Parts are crossed or incomplete

- GIVEN parts contain a gap, duplicate, mixed identity, changed count, or excess bytes
- WHEN assembly validation runs
- THEN it MUST reject the assembly and expose no partial state

### Requirement: Verification covers success and failure paths

r[host_control.pull_state.verification] The focused rail MUST cover valid pull outcomes and stale, unsupported, crossed, duplicate, incomplete, oversized, malformed, and producer-failure cases.

#### Scenario: Focused conformance runs

- GIVEN the core, codecs, consumer mapping, and fixtures are present
- WHEN focused validation runs
- THEN all positive and negative cases MUST produce their declared decisions and non-claims
