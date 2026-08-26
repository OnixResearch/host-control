# Tasks

## Phase 1: Boundary and audit

- [ ] [serial] Record Jido reference revision `548b2a345765ba33e687341c661bbbcbdda73d94` at `github.com/agentjido/jido`, license, and the selected pods concept in repository references. r[host_control.agent_group.boundary]
- [ ] [serial] Complete the reviewed generated-policy refresh: `task` and `after` marker kinds and the current workflow artifact shape, and pass normal repository validation. r[host_control.agent_group.boundary] r[host_control.agent_group.verification]
- [ ] [serial] Run and record current topology, selector, feature, codec, Animus mapping, Tile mapping, Lattice mapping, no-std, and Wasm baselines. r[host_control.agent_group.verification]
- [ ] [serial] Record the audit outcome that Lattice durable orchestrations own durable group execution and that this change supplies only the authority-free identity and cohort surface. r[host_control.agent_group.boundary]
- [ ] [serial] Add no-std durable-agent-group identity, member, partition-label, topology-revision, generation, cohort, requirement, decision, issue, transition, limit, and receipt DTOs. r[host_control.agent_group.model]
- [ ] [depends:host_control.agent_group.model] Implement pure structural validation and domain-separated BLAKE3 identities over supplied safe values. r[host_control.agent_group.admission]

## Phase 2: Admission and generations

- [ ] [depends:host_control.agent_group.admission] Implement exact same-partition and explicit split-policy decisions with fail-closed missing, duplicate, mixed, stale, substituted, unknown, empty, and over-limit outcomes. r[host_control.agent_group.admission]
- [ ] [depends:host_control.agent_group.admission] Implement group replacement and handoff generation and topology-revision validation without consumer lifecycle policy. r[host_control.agent_group.generation]
- [ ] [serial] Add optional Serde and Preserves codecs that re-enter core validation. r[host_control.agent_group.model]

## Phase 3: Consumer mappings and evidence

- [ ] [depends:host_control.agent_group.admission] Add frozen Animus declared-partition mapping fixtures without an Animus runtime dependency. r[host_control.agent_group.consumers]
- [ ] [depends:host_control.agent_group.admission] Add frozen Tile named-workspace-group mapping fixtures without a Tile runtime dependency. r[host_control.agent_group.consumers]
- [ ] [depends:host_control.agent_group.admission] Add frozen Lattice selected-group mapping fixtures without a Lattice runtime dependency. r[host_control.agent_group.consumers]
- [ ] [parallel] Add positive fixtures for one current group, explicit admitted split policy, exact generation replacement, codec round trips, and consumer mappings. r[host_control.agent_group.verification]
- [ ] [parallel] Add negative fixtures for mixed required partitions, stale generation, substituted topology revision, duplicate member, missing member, unknown label, malformed codec data, over-limit cohorts, and authority overclaims. r[host_control.agent_group.verification]
- [ ] [serial] Update README consumer mapping and non-claims. Add focused Cairn and Nix checks. r[host_control.agent_group.boundary] r[host_control.agent_group.verification]
- [ ] [depends:host_control.agent_group.verification] Run focused tests before and after core changes, workspace tests, Clippy, no-std, Wasm, codecs, Cairn validation and gates, and relevant Nix checks. r[host_control.agent_group.verification]
