## Why

Host Control already requires negotiated snapshot-first streams. It does not define a client-paced pull contract for bounded state differences.

Push streams can retain work for paused clients. An invalid difference can also corrupt client state when its base revision is stale.

Jane Street provides useful bounded references:

- `polling_state_rpc` at `386b9280ed56e894fc98f8b798693a8d05c36b52` uses client polling to avoid queued updates.
- `ppx_diff` at `b3550478bb7057b6d8bd9cac5067ef72f89976f3` defines typed get and apply operations with explicit invalid-application behavior.
- `streamable` at `5f1fb4971605f609a4d544b9244b5e7f27f9eee7` defines incremental round trips and stable format versions.
- `pipe_with_writer_error` at `9d103d73dba9ba25b608625f711d63bad6b29fda` keeps producer failure visible to consumers.
- `maybe_pushback` at `0b364b456f216eab23390813728e730538d09d42` makes pressure behavior explicit.

All five sources use the MIT license. They are design inputs, not product dependencies or parity requirements.

## What Changes

- Add a pure pull-state request and result contract for Host Control snapshots.
- Add typed no-change, bounded-delta, complete-snapshot, and resnapshot-required outcomes.
- Bind every delta to exact base and target revisions, schema, producer, policy, and limits.
- Add bounded segmented transfer with complete-before-apply assembly.
- Keep producer failure distinct from no-change and transport failure.
- Add positive, negative, stale-base, malformed-part, overflow, and producer-failure fixtures.

## Impact

The change extends `host-control-core` and its conformance fixtures. Transport, storage, polling schedules, clocks, retries, and product adapters remain outside the core.

The change does not replace push streams. Consumers select pull behavior through existing feature negotiation.

## Dependencies

This change uses existing Host Control identity, feature-negotiation, snapshot, error, and limit contracts.

## Non-goals

- Do not define generic product-state diffs.
- Do not queue server work for inactive clients.
- Do not execute transport, retry, timer, persistence, or polling effects.
- Do not apply partial or stale differences.
- Do not treat round-trip success as semantic correctness.
- Do not claim compatibility with Jane Street wire formats.
