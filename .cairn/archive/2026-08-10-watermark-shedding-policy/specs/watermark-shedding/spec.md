# Watermark Shedding Policy Specification

## Purpose

Define a deterministic reference pressure-shedding policy with explicit watermarks for Host Control.

## MODIFIED Requirements

### Requirement: The policy is explicit and watermark-based

r[host_control.shedding.policy] Host Control MUST express the reference shedding policy with an explicit resident count, a high watermark, a low watermark, and an admission gate. It MUST NOT impose an implicit default limit.

#### Scenario: Operator supplies a policy

- GIVEN an operator authors a shedding policy with explicit high and low watermarks and an admission gate
- WHEN policy validation runs
- THEN the policy MUST validate
- AND every limit MUST be explicit

#### Scenario: Policy lacks a watermark

- GIVEN a shedding policy omits a high or low watermark
- WHEN policy validation runs
- THEN validation MUST fail
- AND no default watermark MUST be assumed

### Requirement: The core decides from supplied values

r[host_control.shedding.decision] The pure core MUST return one disposition: keep, shed, or refuse reacquisition. It MUST read no clock, memory, or scheduler state.

#### Scenario: Count below the high watermark

- GIVEN a resident count at or below the high watermark
- WHEN the core decides
- THEN the disposition MUST be keep

#### Scenario: Count above the high watermark

- GIVEN a resident count above the high watermark
- WHEN the core decides
- THEN the disposition MUST be shed

#### Scenario: Reacquisition gate is closed

- GIVEN the resident count is above the high watermark and the low watermark is not reached
- WHEN the core evaluates reacquisition
- THEN the disposition MUST refuse reacquisition
- AND it MUST stay closed until the low watermark is reached

### Requirement: Decisions do not prove eviction

r[host_control.shedding.boundary] Host Control MUST NOT claim eviction, release, scheduler, or resource-success authority. The caller owns these.

#### Scenario: Caller over-claims

- GIVEN a caller or documentation claims eviction, release, scheduler, or memory success
- WHEN ownership and boundary verification runs
- THEN that claim MUST fail verification
- AND the caller-owned boundary MUST remain explicit

### Requirement: Failure coverage remains explicit

r[host_control.shedding.verification] Positive and negative fixtures MUST cover every declared disposition and boundary.

#### Scenario: Complete focused matrix passes

- GIVEN policy, core, fixtures, documentation, and dependency evidence are complete
- WHEN focused no-std, Wasm, package, workspace, Clippy, Cairn, and Nix verification runs
- THEN below-high inputs MUST keep, above-high inputs MUST shed, and gated reacquisition MUST refuse until low
- AND each reversed, missing, coupled, or overclaiming input MUST fail as declared
