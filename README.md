# host-control

Transport-neutral, product-neutral host topology, selector, bounded observation, layout, event, wait, error, feature-negotiation, and snapshot-first stream contracts.

`host-control-core` is `#![no_std]` plus `alloc`. It takes explicit values and performs deterministic validation only. It does not read files or environment state, inspect clocks/processes, open sockets, render terminals, mutate daemons, persist data, or own consumer policy.

## Consumer mapping

| Surface | Canonical | Animus | Tile | Retained owner |
| --- | --- | --- | --- | --- |
| IDs | opaque validated host/resource IDs | string selectors | numeric session/tab/pane IDs | consumer |
| positional selector | scope + topology revision | live selector | revision-scoped index | consumer topology |
| status | unknown/idle/running/waiting/blocked/done/failed | direct labels | working→running; unsupported loss rejects | consumer status source |
| observation | visible/recent/detector-safe, finite lines/bytes | bounded read DTO | bounded pane read | PTY/output host |
| stream | optional snapshot-first | optional | native snapshot-first mapping | transport host |
| layout | optional negotiated declarative tree | layout DTO | workspace layout adapter | UI/daemon |
| timeout | finite data only | milliseconds | consumer native unit mapped explicitly | consumer clock |

Sockets, named pipes, PTYs, processes, persistence, clocks, rendering, daemon state, native workspace state, credentials, policy, and authorization remain outside this repository.

## Verification

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p host-control-core --target wasm32-unknown-unknown
```

Optional `serde` and `preserves` features expose structural codecs that re-enter core validation; they do not own I/O or transport.

Passing tests establish only deterministic DTO and selected Animus/Tile mapping conformance. They do not prove daemon correctness, transport confidentiality/authentication, process or PTY preservation, terminal-output truth, policy authorization, arbitrary host compatibility, or whole-product readiness.
