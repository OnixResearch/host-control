## Phase 1: Reference and contract intake

- [ ] [serial] [task:reference-intake] Record the five reviewed Jane Street revisions, selected concepts, MIT notices, and rejected wire-format dependencies in `README.md`. r[host_control.pull_state.boundary]
- [ ] [serial] [task:pull-model] Define no-std request, result, revision, identity, limit, delta-entry, part, and failure values. r[host_control.pull_state.model]
- [ ] [serial] [task:feature-contract] Extend feature negotiation for pull, delta, segmentation, and resnapshot capabilities. r[host_control.pull_state.negotiation]

## Phase 2: Pure decisions

- [ ] [serial] [task:result-admission] [after:pull-model] Implement pure admission for no-change, delta, snapshot, resnapshot-required, and producer-failed results. r[host_control.pull_state.result]
- [ ] [serial] [task:delta-apply] [after:result-admission] Implement revision-fenced add, replace, and remove decisions for Host Control resource identities. r[host_control.pull_state.delta]
- [ ] [serial] [task:part-assembly] [after:result-admission] Implement bounded complete-before-apply segmented assembly. r[host_control.pull_state.segmentation]

## Phase 3: Conformance and evidence

- [ ] [parallel] [task:positive-fixtures] [after:delta-apply] Add no-change, delta, snapshot, segmented, and resnapshot fixtures. r[host_control.pull_state.verification]
- [ ] [parallel] [task:negative-fixtures] [after:part-assembly] Add stale, crossed, duplicate, gap, overflow, malformed, and producer-failure fixtures. r[host_control.pull_state.verification]
- [ ] [serial] [task:consumer-mapping] [after:positive-fixtures] Add one transport-neutral consumer mapping with no product or transport types in the core. r[host_control.pull_state.boundary]
- [ ] [serial] [task:gates] [after:negative-fixtures] Run no-std, Wasm, workspace, Clippy, Octet, Cairn, and Nix gates. r[host_control.pull_state.verification]
