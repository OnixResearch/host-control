## Goals

- Express a reference pressure-shedding policy with explicit watermarks.
- Make deterministic shed, keep, and admission decisions from supplied values.
- Keep the decision core pure and free of ambient effects.
- Leave resource scheduling and eviction authority with the caller.

## Policy shape

The reference policy declares these facts:

1. A resident-item count.
2. A high watermark.
3. A low watermark.
4. An admission gate for reacquisition of released items.

All values are explicit. No implicit default limit exists.

## Decision rules

The pure core returns one of three decisions:

- `Keep` — the resident count is at or below the high watermark.
- `Shed` — the resident count is above the high watermark.
- `RefuseReacquire` — the admission gate stays closed until the low watermark is reached.

The core accepts the resident count, both watermarks, and the current admission gate as values. It reads no clock, memory, or scheduler state.

An idle item is a candidate for shedding. An active item or one with live external sessions is not a shedding candidate; the caller decides which items are active.

## Output

The decision contains public, bounded fields:

- the resident count and both watermarks;
- the admission gate state;
- the disposition;
- implementation and policy identities;
- required non-claims.

The decision does not name concrete cells, prove eviction, or prove release.

## Functional core and shell

The pure core computes the disposition from supplied values.

The shell owns integration with a caller scheduler and resource observations. The caller owns durable parking, eviction, and reacquisition.

## Reference

The Celld watermark-shedding design is a bounded reference input. It is a comparison source, not a Host Control requirement, parity claim, or equivalence claim.

## Verification

Positive coverage includes keep below high, shed above high, and refuse until low.

Negative coverage includes reversed watermarks and missing values.

Boundary coverage rejects an implicit default limit, scheduler coupling, active-item eviction, live-session coupling, and eviction-promotion claims.
