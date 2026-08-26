# Design: Define durable agent-group contracts

## Context

Host Control identifies hosts, resources, and topologies. Consumers still compose agent groups whose supplied facts can refer to different topologies, partitions, or generations. The mismatch is semantic, not a transport concern.

The shared contract must identify and compare durable agent groups without discovering them, activating them, or defining product-specific lifecycle behavior.

An initial audit of Lattice durable orchestrations and Animus daemon workers records that durable group execution already has product owners. This change supplies only the authority-free identity and cohort-validation surface.

## Success Contract

Given in-memory group, member, partition, and generation facts, `host-control-core` returns a deterministic admitted or blocked cohort decision. A declared-partition requirement admits only members with the exact group identity, current topology revision, and current generation.

## Decisions

### 1. Keep group identity opaque and product-neutral

`DurableAgentGroupId` is an opaque safe identifier. `GroupMember` binds a consumer-owned member identity and capability-class label to one group, partition, topology revision, and generation. `PartitionLabel` and `TopologyRevision` are bounded consumer-owned labels.

Host Control does not define activation, lifecycle, execution, workspace, or provider semantics.

### 2. Model explicit group requirements

A `GroupCohortRequirement` declares:

- one cohort identity;
- required member identities or capability classes;
- one declared partition policy (`same_partition_required` or an explicit consumer-defined split);
- expected topology revision and generation when already selected;
- exact member and relation bounds.

The pure decision returns admitted members, safe issue classes, and a domain-separated BLAKE3 cohort identity.

### 3. Fail closed on ambiguity and drift

Validation rejects missing members, duplicates, unknown labels, empty required cohorts, mixed partitions under same-partition policy, stale generations, substituted topology revisions, and over-limit sets.

An identity match does not imply activation, authorization, reachability, compatibility, or truth.

### 4. Make replacement a generation transition

A group replacement or handoff creates a new generation and topology revision. Old-generation members cannot silently join the replacement cohort.

Consumers decide whether to rebuild, hand off, reject, or degrade. Host Control only validates supplied transition facts.

### 5. Keep effects, lifecycle, and observations with consumers

Lattice can record the selected group identity for one agent node or orchestration. Animus can require one declared partition before exposing related tools. Tile can project a named workspace group.

Each consumer retains discovery, activation, lifecycle, transport, authority, credentials, persistence, evidence, and effects.

### 6. Provide structural codecs only

Optional Serde and Preserves codecs encode and decode group DTOs. Decoded values re-enter core validation before use.

### 7. Reference the reviewed Jido pods concept

Jido pods at revision `548b2a345765ba33e687341c661bbbcbdda73d94` are a design reference for durable named grouping, nested topology, lazy activation, and partition tenancy. Host Control adapts only the identity and cohort-validation discipline. It does not port Elixir or OTP.

## Alternatives Considered

### Reuse host identity as group identity

Rejected because one host can contain several groups, partitions, sandboxes, workers, or workspace generations. A durable group can also span transport endpoints while retaining one product meaning.

### Put group matching in Lattice or Animus only

Rejected because the comparison rule is product-neutral and several consumers need compatible semantics. Lattice already owns execution; it does not need to re-derive admission.

### Skip the contract because Lattice orchestrations exist

Rejected. Lattice orchestrations are graph instances with product authority. They do not prove that a supplied member set is one named group under a declared partition policy.

## Risks and Controls

- Consumers can supply false group facts. Non-claims state that Host Control validates structure only.
- Partition labels can become a universal taxonomy. Labels remain consumer-owned bounded strings with exact comparison only.
- Generation rollover can interrupt active work. Consumers retain transition and continuity policy.

## Non-Claims

This design does not prove physical co-location, activation, transport security, adapter compatibility, authorization, sandboxing, lifecycle behavior, data consistency, effect success, production readiness, or release eligibility.
