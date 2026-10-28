# Sentinel

A personal HackerOne bug-bounty automation and intelligence tool, built to learn advanced
Rust, algorithms, database internals, cloud services and distributed systems. The owner
implements it; the assistant mentors. A top ranking is an ambition, not a guarantee.
This repository defines its own scope independently of earlier implementations.

## Start here

| Document | Purpose |
| --- | --- |
| [Roadmap](docs/PROJECT_ROADMAP.md) | Current work, prioritized end-to-end projects and project structure. |
| [Features](docs/FEATURES.md) | Story contracts, scope dispositions and project responsibility. Read only the relevant stories. |
| [Study guide](docs/STUDY_GUIDE.md) | Rust/CS, database and performance learning with source-reading targets. |
| [AGENTS.md](AGENTS.md) | Mentoring agreement and rules for source-based examples. |

The implementation is early and unfinished. The Cargo workspace currently contains only
the graph library. Plans and discussions are not evidence of working behavior.

## Product scope

Include work that helps select a target, discover or understand a bug, validate impact,
retain evidence, prepare a strong report, or operate this personal tool affordably:

- Authorized recon and program/scope intelligence; versioned capture and change detection.
- Graph/history/search memory; parsing, source maps, static analysis and API reconstruction.
- Traffic replay, owned test identities, access-control/business-logic investigations,
  focused detectors, browser and out-of-band evidence.
- Explainable hypotheses, variant/duplicate search, ranking and ML evaluated against rules.
- Personal notes, reproducible evidence, reviewed reports, workflows, alerts and GCP operation.

Mobile/Wasm, advanced protocols/TLS, plugins, extra integrations and distributed workers
are conditional on a useful target, measured need or bounded learning exercise.
Enterprise tenants/team administration, CMDB/GRC/SOC operations, corporate CI enforcement,
audit-witness infrastructure, ticket synchronization, commercial IDE/plugin platforms
and whole-product or internet-wide coverage parity are outside scope. Studying a target's
SSO/cloud/dependency behavior for a bounty remains useful research.

## Build and research rules

Ship small working slices in a modular monolith, then profile and improve. Use one Git
repository/Cargo workspace; extract crates and services only when justified. Advanced
learning experiments may stay separate instead of becoming production requirements.

Develop with synthetic fixtures/local labs. Live activity needs explicit instruction and
program permission, scope/exclusion checks, budgets, cancellation and provenance. Neither
an asset discovery nor model output grants authority. Do not bypass blocks or automatically
test found credentials. Human validation and report review precede submission; recheck the
[HackerOne Code of Conduct](https://www.hackerone.com/policies/code-of-conduct) when relevant.
Keep private evidence, traffic, credentials and embeddings under operator control and out
of external model services, hosted code tools, logs and committed fixtures.

Success means useful original validated findings, reproducible evidence, less manual work
and affordable operation. Measure lead/report time, precision/recall on labelled scenarios,
duplicate/dismissal reasons and actual outcomes. Platform
[Signal and Impact](https://docs.hackerone.com/en/articles/8369891-signal-impact) differ from
scanner counts. Completion concerns selected core workflows and accepted limitations;
conditional ideas are not an obligation to implement every option.
