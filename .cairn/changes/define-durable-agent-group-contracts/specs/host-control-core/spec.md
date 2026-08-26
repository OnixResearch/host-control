# Host Control Core

## ADDED Requirements

### Requirement: Durable agent-group contracts are authority-free

r[host_control.agent_group.boundary] Host Control MUST define durable-agent-group identity, membership, partition, topology, generation, and cohort validation without filesystem, process, environment, network, clock, terminal, daemon, UI, transport, persistence, credential, discovery, activation, authorization, or product mutation effects.

#### Scenario: Supplied group facts are validated

- GIVEN a consumer supplies in-memory group, partition, topology, generation, member, and requirement values
- WHEN Host Control validates them
- THEN it MUST return deterministic structural facts or typed issues
- AND it MUST perform no host effect.

### Requirement: Durable agent groups use typed bounded values

r[host_control.agent_group.model] Host Control MUST define opaque group identity, member identity, partition label, topology revision, generation, cohort, requirement, decision, issue, transition, limit, and metadata-only receipt values.

#### Scenario: Complete group cohort is represented

- GIVEN a bounded set of current members each binds one group identity, partition label, topology revision, and generation
- WHEN model validation runs
- THEN Host Control MUST admit the typed structure for cohort evaluation
- AND no member value may contain a service handle or product payload.

#### Scenario: Duplicate member is rejected

- GIVEN a cohort repeats one member identity
- WHEN model validation runs
- THEN Host Control MUST reject the cohort.

### Requirement: Group admission fails closed on mismatch

r[host_control.agent_group.admission] A declared-partition requirement MUST admit only an exact non-empty bounded member set whose required members cite one matching group identity, one declared partition, one matching topology revision, and the current generation. Missing, duplicate, mixed, stale, substituted, unknown, empty, or over-limit facts MUST fail closed.

#### Scenario: Same-partition cohort is admitted

- GIVEN all required members cite the exact selected group, one declared partition, the current topology revision, and the current generation
- WHEN cohort admission runs
- THEN it MUST return an admitted decision with a deterministic BLAKE3 cohort identity.

#### Scenario: Members name different partitions

- GIVEN a consumer requires members in one partition and the supplied members cite different partition labels
- WHEN cohort admission runs
- THEN Host Control MUST return a mixed-partition denial
- AND it MUST NOT select either partition as a fallback.

#### Scenario: Explicit split policy is supplied

- GIVEN a consumer-owned requirement explicitly permits a bounded split and every relation is declared
- WHEN cohort admission runs
- THEN Host Control MAY admit the supplied structure
- AND the decision MUST preserve each partition, group, and generation rather than claiming one group.

### Requirement: Group replacement is generation-fenced

r[host_control.agent_group.generation] Group replacement and handoff facts MUST create an explicit new generation and topology revision. Old-generation members MUST NOT silently join the replacement cohort.

#### Scenario: Replacement generation is current

- GIVEN a consumer supplies an admitted predecessor, a distinct next generation, a distinct topology revision, and exact replacement members
- WHEN transition validation runs
- THEN Host Control MAY return a structurally valid transition
- AND activation, continuity, migration, and effect decisions MUST remain consumer-owned.

#### Scenario: Old member is reused silently

- GIVEN a replacement cohort includes a member bound only to the prior generation or topology revision
- WHEN transition validation runs
- THEN Host Control MUST reject the transition.

### Requirement: Consumer mappings retain product ownership

r[host_control.agent_group.consumers] Animus, Tile, Lattice, and other consumers MUST retain group discovery, activation, lifecycle, observation truth, capability semantics, transport, authorization, credentials, persistence, effects, and evidence claims.

#### Scenario: Lattice records a selected group

- GIVEN Lattice supplies a bounded selected group identity for one agent node or orchestration
- WHEN the mapping fixture runs
- THEN structural identity, partition, and generation facts MAY be admitted
- AND Host Control MUST gain no Lattice execution, scheduling, persistence, or effect authority.

#### Scenario: Animus requires one declared partition

- GIVEN Animus supplies exact member facts for one required partition of a group
- WHEN the mapping fixture runs
- THEN same-partition structure MAY be admitted
- AND Host Control MUST NOT activate or authorize any tool, worker, or host effect.

### Requirement: Durable agent-group contracts have positive and negative verification

r[host_control.agent_group.verification] The focused rail MUST cover valid same-group and explicit split cohorts, replacement generations, codecs, consumer mappings, and negative missing, duplicate, mixed, stale, substituted, unknown, malformed, over-limit, and authority-overclaim cases.

#### Scenario: Agent-group rail passes

- GIVEN all required no-std, Wasm, codec, consumer, positive, and negative fixtures are present
- WHEN the focused rail runs
- THEN structural decisions MUST be deterministic
- AND invalid facts MUST fail without effects or promoted claims.
