## Policy core

- [x] [serial] Add the typed reference shedding policy with explicit high and low watermarks and an admission gate. r[host_control.shedding.policy]
- [x] [serial] Implement the pure keep, shed, and refuse-reacquire decision. r[host_control.shedding.decision]
- [x] [serial] Document that the caller owns durable parking, eviction, and reacquisition. r[host_control.shedding.boundary]

## Verification

- [x] [parallel] Add positive cases for keep below high, shed above high, and refuse until low. r[host_control.shedding.verification]
- [x] [parallel] Add negative cases for reversed watermarks and missing values. r[host_control.shedding.verification]
- [x] [parallel] Add boundary cases that reject an implicit default limit, scheduler coupling, and active-item or live-session eviction. r[host_control.shedding.verification]
- [x] [serial] Run no-std, Wasm, package, workspace, Clippy, Cairn, and Nix checks, then document non-claims. r[host_control.shedding.verification]
