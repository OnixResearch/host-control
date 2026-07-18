# Agent notes

- Preserve the `host-control-core` no-std functional-core boundary.
- Keep filesystem, process, environment, network, clock, terminal, daemon, UI, transport, and persistence effects out of the core.
- Add positive and negative tests together.
- Use BLAKE3 for repository-owned identities.
- Run focused Cargo tests and Clippy before commits. Do not push unless explicitly requested.
