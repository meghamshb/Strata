# Strata Agent Instructions

Strata is a source-based NautilusTrader distribution for research, deterministic simulation,
paper trading, and eventually live Hong Kong execution. Treat every trading path as though it
controls hundreds of millions of dollars, even when the enabled canary capital is HKD 2,500.
Correctness, containment, recovery, and auditability take precedence over feature velocity.

## Working Rules

- Read the affected NautilusTrader and Strata code before proposing or making changes. Search for
  existing domain types, traits, message patterns, lifecycle rules, and tests before adding one.
- Keep changes focused, typed, reviewable, and as small as correctness permits. Do not perform
  unrelated refactors or introduce speculative abstractions.
- Preserve upstream compatibility and keep the Strata patch set against NautilusTrader small and
  auditable. Isolate unavoidable core changes so they can be reviewed during upstream upgrades.
- Never commit, amend, push, tag, release, open or modify a pull request, or otherwise change
  remote state unless the user explicitly authorizes that exact action.
- Use concise imperative commit subjects. Do not add attribution metadata.

## Attribution

- Do not add an AI tool or model as an author, co-author, or contributor.
- Do not add `Co-authored-by:` trailers for AI tools or models.
- Do not add branded footers such as `Generated with ...` to commits, pull requests, release text,
  source files, or generated artifacts.

## Documentation Policy

- Root `AGENTS.md` and the minimal root `README.md` are the only standalone-document exceptions.
- Keep `README.md` limited to project identity and the current execution-safety status. Modify it
  only when the user explicitly requests the change.
- Do not add design documents, reports, architecture documents, diagrams, generated documentation,
  changelogs, roadmaps, additional READMEs, or a `docs/` tree.
- Necessary code comments, Rust API documentation for public interfaces, schemas, migrations, and
  operator-visible error text are permitted because they are part of executable correctness.

## NautilusTrader Source Policy

- Strata consumes NautilusTrader directly as exact pinned native Rust Git dependencies. It is not
  an external wrapper, parallel engine, generic facade, or HTTP layer over NautilusTrader.
- `origin` is `meghamshb/StrataMigrate`; `upstream` is `nautechsystems/nautilus_trader`.
- Pin engine imports in the root Cargo manifest to an exact reviewed release commit and record its
  release tag in the manifest comment. Never track `develop`, `nightly`, a wildcard dependency, or
  an unpinned Git revision.
- Perform upstream upgrades on temporary `sync/nautilus-<version>` branches. Review the full
  dependency and lockfile delta, rerun qualification, and remove the branch only after acceptance.
- Preserve upstream history, copyright, and license notices.
- Use NautilusTrader domain types, native traits, message bus, cache, clock, and component lifecycle.
  Do not duplicate its order, account, instrument, portfolio, or execution models.
- Modify an upstream core crate only when a required invariant, performance property, or extension
  point cannot be implemented cleanly in a Strata crate. Carry that change in a separate, pinned
  upstream fork or patch set; never copy NautilusTrader source into Strata. Supply focused tests
  plus benchmark evidence when performance is the reason.

## V0 Scope: HK/Futu Infrastructure Qualification

V0 proves a native Futu OpenD integration for NautilusTrader. It is an infrastructure
qualification milestone, not an alpha, revenue, or live-capital milestone.

- Build only `strata-hk`, `strata-futu`, `strata-runtime`, and `strata-reconcile`.
- `strata-hk` owns only Hong Kong market rules absent from NautilusTrader: sessions, board lots,
  tick sizes, fees, reference data, and corporate-action normalization.
- `strata-futu` owns the Futu OpenD protocol boundary: connection lifecycle, authentication,
  subscriptions, callback threading, reconnects, paper-account semantics, request correlation,
  and translation of quotes, orders, fills, and venue reports into native NautilusTrader events.
- `strata-runtime` owns explicit paper-only configuration, component composition, operator trading
  locks, and environment separation. It must not add a second engine or order model.
- `strata-reconcile` initially contains one capability only: compare Futu broker reports with
  NautilusTrader state and block new exposure when the comparison is incomplete or differs.
- Reserve `strata-ledger`, `strata-risk`, and `strata-strategy` as future component names. Do not
  create their crates, production behavior, public APIs, persistence, or dependencies during V0.
  Test-only deterministic strategies are permitted when required to qualify the adapter.
- Use NautilusTrader's existing portfolio, risk, execution, order-state, cache, and event-store
  facilities before proposing a Strata replacement or extension.
- Defer a dedicated ledger until event-store and broker reports demonstrably cannot satisfy a
  required settlement, accounting, tax, or audit control. Defer custom risk until a concrete HK or
  Futu policy cannot be expressed safely through NautilusTrader controls. Defer a strategy
  framework until adapter qualification is complete and a single strategy has an evidenced need.
- No RAG, LLM, news pipeline, strategy factory, multi-venue execution, live order submission, or
  P&L target is in V0 scope.

V0 is complete only when a paper-market quote flows through native NautilusTrader events to a
deterministic test decision and validated paper limit order, then survives acknowledgement, fill,
rejection, modification, cancellation, disconnect, duplicate callback, stale data, unknown order
outcome, forced restart, event-store tail replay, and broker-versus-state reconciliation. Any
reconciliation failure must lock new exposure until resolved.

## Module Boundaries

- `strata-hk`: HKEX sessions, board lots, tick sizes, instruments, fees, and corporate actions.
- `strata-futu`: native NautilusTrader data and execution clients for Futu OpenD.
- `strata-reconcile`: broker-versus-state comparison and new-exposure trading locks.
- `strata-runtime`: paper-only composition, configuration, lifecycle, and operator trading locks.
- `strata-risk`, `strata-ledger`, and `strata-strategy` are reserved future names, not V0 crates.
- Keep dependency direction acyclic. Venue, market, and reconciliation modules may depend on
  Nautilus core crates; only the runtime composes the complete V0 system.
- A strategy emits a forecast or portfolio target. Only centralized portfolio and risk components
  may produce an approved execution intent. Strategies never call the broker directly.

## Correctness and Trading Safety

- Use exact domain arithmetic for prices, quantities, money, fees, limits, P&L, and accounting.
  Floating-point arithmetic is forbidden for these values.
- Preserve deterministic event ordering, explicit clocks, idempotent client-order identifiers,
  journaled state transitions, and restart-safe reconstruction.
- Model every order transition as an explicit state machine. A timeout is not evidence of failure;
  reconcile an unknown outcome before retrying.
- Fail closed on stale or missing market data, arithmetic overflow, reconciliation differences,
  missing conversion prices, invalid market rules, dependency failures, and corrupt state.
- During partial failure, block new exposure while preserving cancel, cancel-all, and reduce-only
  operations whenever the broker connection permits them.
- No LLM, retrieval system, research agent, or model-generated output may submit or amend orders,
  change risk limits, unlock trading, claim a fill, or become authoritative broker state.
- Keep paper, shadow, canary, and production configurations mechanically distinct. Never infer the
  environment from an account identifier or a default value.
- Secrets must never appear in source, configuration committed to Git, logs, fixtures, panic output,
  traces, or test snapshots.

## Complexity and Memory

- Evaluate worst-case and amortized complexity, not best-case complexity.
- Target O(1) or amortized O(1) for event dispatch, identifier lookup, and current-state access.
- Target O(log n) for ordered books, schedules, timers, and indexed time-series operations.
- Permit O(n) only for explicitly bounded batches, snapshots, reconciliation passes, maintenance,
  or offline research. State the bound at the call site or in the owning type.
- Avoid unnecessary allocation, cloning, locking, dynamic dispatch, serialization, and format
  conversion on market-data and order hot paths.
- Prefer ownership and borrowing, bounded preallocated buffers, batch I/O, and explicit backpressure.
- Every queue, cache, retry policy, task set, and in-memory history must define an owner, maximum
  size, eviction or retention policy, overflow behavior, and recovery source.
- Unbounded queues, caches, retries, task spawning, and histories are forbidden.
- Unsafe Rust is forbidden unless the invariant is documented next to the block, no safe design can
  meet the requirement, benchmark evidence justifies it, focused soundness tests exist, and the
  change receives explicit review.
- Measure before optimizing. Performance claims require a reproducible benchmark, representative
  data, allocation measurements where relevant, and a stated regression threshold.

## Testing and Evidence

- Add tests proportional to risk: unit, property, integration, deterministic replay, restart,
  disconnect, duplicate-event, stale-data, reconciliation, fuzz, and benchmark coverage as relevant.
- Do not add test-only behavior to production paths or weaken tests to obtain a passing result.
- Verify identical decisions for identical ordered inputs across replay and live-shadow paths.
- Test queue saturation, backpressure, delayed and duplicated callbacks, out-of-order events,
  unknown order outcomes, clock discontinuities, broker reconnects, and state recovery.
- Treat validation as evidence for the exact revision tested. Rerun affected checks after every
  meaningful edit, rebase, dependency change, or upstream sync.
- Run the smallest focused checks during development, then workspace formatting, linting, tests,
  security checks, and relevant benchmarks before claiming review readiness.

## Live-Trading Gate

Live order submission remains disabled until all of the following are independently verified:

1. An official stable NautilusTrader v2 release is pinned by tag and commit.
2. Futu paper and shadow execution pass disconnect, timeout, restart, and recovery testing.
3. Orders, fills, fees, cash, settlements, and positions reconcile without manual database repair.
4. Kill, cancel-all, reduce-only, exposure, loss, liquidity, and stale-data controls pass failure drills.
5. An authorized operator enables a separate cash-only HKD 2,500 canary configuration manually.

Increasing capital never relaxes a control or bypasses a gate. It requires a new capacity,
liquidity, market-impact, operational-risk, and recovery review.
