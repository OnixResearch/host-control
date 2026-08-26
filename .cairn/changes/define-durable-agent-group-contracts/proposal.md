# Proposal: Define durable agent-group contracts

## Why

Host Control owns transport-neutral host topology and selectors. Consumers such as Animus, Tile, and Lattice can group agents and workspaces into durable sets, but no product-neutral contract proves that supplied group facts refer to one named topology, one partition, and one generation.

A consumer can read members from one topology while resolving a workspace elsewhere. Each member can be valid in isolation while the group composition is invalid.

Jido, the Elixir agent framework at `github.com/agentjido/jido` revision `548b2a345765ba33e687341c661bbbcbdda73d94`, names durable named agent groups ("pods") with nested topology, lazy activation, and partition-safe tenancy. An initial review finds that Lattice durable orchestrations already own durable group execution, and Animus daemon workers own concrete lifecycle. What is missing is the authority-free group identity and cohort-validation contract, not a new runtime.

Host Control should adapt only the authority-free group identity, membership, partition, and generation validation contract. Products retain discovery, transport, authorization, activation, lifecycle, persistence, effects, and release decisions.

## What Changes

- Add opaque durable-agent-group identity, member, partition label, topology revision, generation, cohort, requirement, and decision DTOs. r[host_control.agent_group.model]
- Add pure validation for exact same-group, declared-partition, missing member, duplicate member, stale generation, substituted topology, unknown label, and over-limit cases. r[host_control.agent_group.admission]
- Bind group replacement and handoff to explicit generation transitions. r[host_control.agent_group.generation]
- Add Animus, Tile, and Lattice mapping fixtures without runtime dependencies. r[host_control.agent_group.consumers]
- Add positive and negative no-std, codec, Wasm, consumer, and Cairn verification. r[host_control.agent_group.verification]

## Lifecycle Prerequisite

Host Control keeps the native `.cairn/` lifecycle and generated policy. Current Cairn commands rejected the repository until the generated policy gained the `task` and `after` marker kinds and the workflow schema gained the current artifact shape. This package records that reviewed policy refresh as a completed prerequisite; it must not be reverted.

## Impact

- **Core**: `host-control-core` receives only pure DTOs, validators, decisions, and BLAKE3 identities over supplied safe values.
- **Consumers**: Lattice can record the selected group identity for one agent node or orchestration. Animus can require members of one declared partition. Tile can project a named workspace group.
- **Boundary**: no product adapter or effect moves into Host Control.

## Out of Scope

- Discovery, activation, lifecycle, transport, authorization, credentials, persistence, clocks, effects, and teardown execution.
- Proving that supplied group facts match physical reality, that a group is live, or that same-group members are compatible.
- Duplicating Lattice durable orchestrations or Animus daemon worker lifecycle.

## Affected Specs

- `host-control-core`: group model, admission, generations, consumer mappings, and verification.
