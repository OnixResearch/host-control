# host-control

`host-control-core` defines product-neutral contracts for host control. The contracts do not depend on a transport.

The crate covers these surfaces:

- host topology and selectors
- bounded observations
- layouts and events
- wait and error results
- feature negotiation
- snapshot-first streams

The crate uses `#![no_std]` plus `alloc`. It accepts explicit values and performs deterministic checks.

It does not access files, environment state, clocks, processes, sockets, terminals, daemons, or persistent data. Consumers retain all product policy.

## Consumer mapping

| Surface | Canonical | Animus | Tile | Retained owner |
| --- | --- | --- | --- | --- |
| IDs | opaque host and resource IDs | string selectors | numeric session, tab, and pane IDs | consumer |
| positional selector | scope plus topology revision | live selector | revision-scoped index | consumer topology |
| status | unknown, idle, running, waiting, blocked, done, or failed | direct labels | `working` maps to `running`, and unsupported loss fails | consumer status source |
| observation | finite visible, recent, or detector-safe lines and bytes | bounded read DTO | bounded pane read | PTY or output host |
| stream | optional snapshot-first stream | optional | native snapshot-first mapping | transport host |
| layout | optional declarative tree | layout DTO | workspace layout adapter | UI or daemon |
| timeout | finite data only | milliseconds | explicit mapping to the consumer unit | consumer clock |

These functions remain outside this repository:

- sockets and named pipes
- PTYs and processes
- persistence and clocks
- rendering and daemon state
- native workspace state
- credentials, policy, and authorization

## Resource-watermark policy

<!-- r[impl host_control.shedding.policy] -->
<!-- r[impl host_control.shedding.boundary] -->

`host-control-core::shedding` computes a deterministic pressure decision from an explicit resident count, watermarks, and reacquisition gate.

The gate closes after the count exceeds the high watermark. It remains closed until the count reaches the low watermark.

The core can reject active items and items with live external sessions as shedding candidates. It does not select or evict an item.

The caller retains these functions:

- resource observation
- scheduling and candidate selection
- durable parking and eviction
- release and reacquisition
- proof that resource pressure decreased

The Celld pressure-shedding mechanism is a bounded design reference only. Host Control does not claim parity or implementation equivalence.

## Checks

Run all commands from the repository root:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p host-control-core --target wasm32-unknown-unknown
```

Optional `serde` and `preserves` features provide structural codecs. Decoded values re-enter core checks.

The codecs do not perform I/O or own transport behavior.

Passing tests prove deterministic DTO behavior and selected Animus and Tile mappings. They do not prove these properties:

- daemon correctness
- transport confidentiality or authentication
- process or PTY preservation
- terminal-output truth
- policy authorization
- arbitrary host compatibility
- whole-product readiness
