## Why

Host Control owns transport-neutral topology, bounded observations, layouts, events, waits, errors, feature negotiation, and snapshot-first streams. It tracks bounded resource state but does not yet express a reference pressure-shedding policy with explicit watermarks.

Celld sheds memory pressure with high and low resident-cell watermarks. Above the high watermark, it durably parks and fences least-recently-used idle cells and refuses to reacquire released cells until the low watermark is reached. That is a clean, transferable reference for an admission and shedding policy over bounded resources.

Host Control should own the deterministic shedding decision core. The caller owns the resource scheduler, durable parking, eviction, and reacquisition authority.

## What Changes

- Add a typed reference pressure-shedding policy with explicit high and low watermarks.
- Make a deterministic shed, keep, and admission decision from supplied counts and watermark values.
- Keep the decision core free of clocks, filesystems, processes, and ambient state.
- Add positive, negative, and boundary fixtures for high, low, idle, active, and reacquisition cases.
- Reference the reviewed Celld watermark-shedding design as a bounded, non-parity input.

## Impact

The change gives callers a deterministic, reviewable shedding policy. It does not evict cells, park state, or run a scheduler.

Shedding policy identities and receipts remain BLAKE3.

## Dependencies

This change has no downstream prerequisite.

## Non-goals

- Do not add a scheduler, a process, or a memory manager.
- Do not implement durable parking, eviction, or reacquisition of concrete cells.
- Do not read clocks, memory pressure, or CPU load.
- Do not claim parity with, or equivalence to, the Celld shedding implementation.
- Do not convert a shedding decision into a proof of successful eviction or release.
