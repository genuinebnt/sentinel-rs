# Sentinel roadmap

Product scope is in [README.md](../README.md); story contracts and responsibility are in
[FEATURES.md](FEATURES.md). Use [STUDY_GUIDE.md](STUDY_GUIDE.md) as work raises a question.

## Current work

**Behavior evidence last recorded:** 27 October 2026.
**Active project:** P01, discovery and workflow runner.
**Active increment:** host identity, insertion and lookup; then one directed relation.

- **Observed:** the graph crate contains initial asset/relation/direction types and an
  unfinished graph structure/methods. This is inspected source, not a passing behavior demo.
- **Verified working:** no end-to-end runtime behavior recorded yet.
- **Current gap:** graph storage/operations are incomplete; no fixture-to-CLI/API run is
  demonstrated.
- **Next behavior:** insert a normalized synthetic host, retrieve it by returned identity,
  deliberately handle duplicates/unknown identities, and preserve existing identities
  when more hosts are inserted. The owner selects the representation and writes the code.
- **Return evidence:** no live hunting/time-saving outcome or benchmark improvement measured
  yet. Use local fixture demonstrations first; no payout/accuracy estimate is implied.
- **Learning evidence:** no teach-back/implemented behavior recorded yet.
- **Open learning question:** distinguish host value equality from graph identity and
  explain what the chosen storage promises about lookup/insertion cost.
- **Next project:** P02 after a useful P01 v1, carrying deeper P01 work visibly rather
  than requiring every advanced topic before change capture/search begins.

Replace this checkpoint with verified behavior, gaps, a learning question and the next
small increment. Update project status when warranted; keep no competing progress log.

## Delivery approach

Prioritize hunting usefulness plus transferable skills per implementation/maintenance
cost. This is a planning judgment; revise it when usage reveals a better next step.
P01 and P02 stay first. Close the evidence/report loop through P03/P04, then expand testing.
Use basic manual scope early; P05 later automates onboarding. One alert starts in P02;
a full proxy, richer DSL, ML and worker infrastructure grow from demonstrated needs.

Work fixture -> small behavior -> focused checks -> shared CLI/API use case -> useful
release -> profile/improve. Learn one or two concepts per increment. Keep deeper follow-ups
visible without blocking the next usable release. States: **Planned**, **In progress**,
**Usable v1**, **Complete**, **Conditional**. A usable v1 is not completion of every story.

## Ordered project tracker

Order is explicit; project IDs remain stable references and are not priority numbers.
A project's usable v1 can unlock the next item while deeper follow-ups remain tracked.

| Priority | Project and next useful scope | Why it earns this position | Status |
| --- | --- | --- | --- |
| 1 | P01: Discovery and workflow runner | Foundation reused by nearly every later feature; broad core Rust/graph/API learning. | In progress |
| 2 | P02: Change capture, historical search and one basic alert | Makes new surface and retained evidence actionable; strong storage/index/algorithm learning. | Planned |
| 3 | P03: Source understanding and one valuable fact/detector family | Converts opaque artifacts into explainable leads; introduces practical compiler/static-analysis theory. | Planned |
| 4 | P04: Minimal investigation notebook, evidence and report workflow | Turns tooling into validated research and records feedback for later intelligence. | Planned |
| 5 | P06: Capture import, replay and comparison first | Enables practical testing quickly without requiring a full interception product. | Planned |
| 6 | P07: Identity/access-control investigations | Uses retained requests/evidence to exercise application-specific reasoning and protocol skills. | Planned |
| 7 | P08: One focused verification/detection workflow at a time | Improves validation and signal; adds measured experiments, browser/OOB work as needed. | Planned |
| 8 | P09: Target-specific infrastructure/leak/dependency enrichment | Add a source only when it opens a concrete lead or useful join; reuse existing adapters/graph. | Planned |
| 9 | P05: Automated HackerOne onboarding, policy drift and program selection | Automate repeated policy/selection work once manual use shows what is worth integrating. | Planned |
| 10 | P10: Deeper temporal graph and evidence reasoning | Real multi-source questions justify advanced history, projections and graph algorithms. | Planned |
| 11 | P15: Learned hypothesis/variant ranking and local research memory | Use real reviewed outcomes and a clear evaluation task; useful rules already run earlier. | Planned |
| 12 | P12: Reliable alerts, scheduling and research events beyond the basic channel | Expand from early simple alerts when delivery/automation reliability is the bottleneck. | Planned |
| 13 | P11: Richer personal workflow language and adapters | Generalize repeated successful workflows; deepen compiler skills without speculative language features. | Planned |
| 14 | P14: Reliable cloud operation and measured worker extraction | Basic operation/cloud learning starts earlier; scale and queues follow demonstrated load or a bounded exercise. | Planned |
| Conditional | P16: Specialized mobile/binary/protocol hunting | Can move forward when an actual program offers a concrete opportunity; no universal-format mandate. | Conditional |

P13, P17 and P18 are retired as separate product projects: required persistence/recovery,
personal UI and testing/evaluation are incorporated into the projects they support.
Their removal does not remove useful safeguards or the learning topics. P01/P02 learning
exercises remain available without imposing an enterprise architecture.

## P01: Discovery and workflow runner

**Product:** given a seed domain and saved or authorized tool results, discover hosts,
retain provenance, build/query relationships, and expose a repeatable run through CLI/API.
This establishes one working flow across ingestion, graph, persistence and execution.

**Build in increments:**

1. Host identity/lookup, duplicate semantics and one directed relationship.
2. Saved discovery output -> validated observations -> graph -> CLI query.
3. Persist and reload a run; preserve identity/provenance and handle repeated import.
4. Add one controlled tool adapter with permissions, bounded output, timeout and cancellation.
5. Expose the same application operations through Axum; define run state and errors.
6. Add an enrichment stage, then a small dependency DAG and bounded execution.
7. Parse a tiny stage/dependency notation once the fixed workflow is useful.
8. Deploy a small synthetic-data workflow on GCP and profile one representative path.

**Learn:** adjacency and hashing; BFS/DFS; DAG validation/topological order; DP for
critical-path estimates under stated concurrency assumptions; ownership, newtypes,
iterators, traits/errors, processes, async/backpressure; API design/Axum; parser/AST/
semantic-check basics; GCP containers, IAM and execution lifecycles. Compare CSR only
after a baseline exists. Some deeper exercises can follow the usable v1.

**Usable v1:** import fixture results, query connected hosts via CLI/API, restart without
losing them, and explain duplicate/error behavior. Real-tool execution and deeper DAG,
compiler, cloud and performance work remain explicit follow-ups until demonstrated.

## P02: Versioned capture, change detection and trigram search

**Product:** capture authorized response/file versions, identify changes, search current
and historic evidence, and link hits back to assets and the run that captured them.
Capture here means artifacts; the intercepting proxy is P06.

**Build in increments:** compare complete discovery observations; capture fixture versions;
deduplicate content; show readable diffs; expose history and a full-scan search baseline;
add trigram candidate filtering and exact verification; then recover/rebuild the index.
Add structural change detection when P03 supplies the needed parser facts.

**Learn:** sets, hashing/content addressing, temporal identity, posting lists/intersection,
byte/Unicode semantics, buffered I/O, transactions, index lifecycle and crash recovery;
DP through small LCS/edit-distance exercises; query syntax/planning; API pagination;
blob/metadata separation and cloud storage operation when useful. Profile ingestion,
query latency, allocation/memory and storage amplification before optimizing them.

**Usable v1:** two retained versions survive restart, show a truthful changed/unchanged
result and are searchable with locations/context. **Full follow-ups:** tested trigram
index, short/Unicode cases, history filters, retention and interrupted-index recovery.

## P03: Source and surface understanding

**First slice:** one format, one fact family, source locations and honest coverage; then JS/source maps, HTML/config/source languages, static dataflow and versioned patterns.

**Learn:** AST/CST/visitors, spans, binding/control/dataflow, automata, recursive types, arena lifetimes, `Cow`, bounded decoding, parser recovery, differential/fuzz testing.

**Finish evidence:** fixture -> facts -> graph/search -> explainable lead; malformed or partial reads preserve uncertainty and cannot silently withdraw unsupported facts.

## P04: Personal investigation desk and HackerOne reports

**First slice:** one candidate, notes/evidence, explicit triage states and report export; then a simple personal cockpit, program board, coverage/next-action views, recordings, HackerOne templates/linting, follow-ups, reviewed report preparation, outcomes and payouts. Begin novelty checks with simple own-history matching; P15 deepens retrieval.

**Learn:** domain state machines, aggregate/transaction boundaries, API contracts, validation, redaction, immutable references and integration tests. Rust error/enumeration design and collection lifetimes now serve a complete user workflow.

**Finish evidence:** a local lead can be confirmed/dismissed and reported with trace; reopening, state transitions, sensitive exports and final HackerOne submissions are deliberate. No generic corporate issue tracker, Jira/Linear synchronization or team administration is required.

## P06: HTTP traffic recorder, proxy and manual replay workbench

**First slice:** saved HAR-style import -> traffic query -> local-server replay; then controlled HTTP/HTTPS proxy, sensitive raw store, organizer, TLS options, WebSockets as the target needs them. A browser extension is conditional convenience; introduce console panels with working operations rather than a full proxy-suite checklist.

**Learn:** HTTP/TLS framing/state, streaming, buffers, async ownership, cancellation, query parsing, protocol fidelity, protected secrets and process/network lifecycle.

**Finish evidence:** capture -> inspect -> deliberate send -> comparison -> evidence; scope promotion and sensitive traffic boundaries behave as the catalog specifies.

## P07: Identity, access-control and authentication-flow analyzer

**First slice:** two synthetic identities and one expected access-control rule; then cookie/session variables, bounded refresh/login macros, auth-flow mapping, mailbox/TOTP plumbing and richer protocol checks.

**Learn:** state machines, authorization matrices, equivalence/normalization, protocol invariants, typed capabilities, secret lifetimes and adversarial contract testing.

**Finish evidence:** legitimate differences and unauthorized access are distinguishable with a trace; sessions never cross program/identity boundaries accidentally.

## P08: Detection workbench, browser verification and callback lab

**First slice:** one local vulnerable/benign pair and one detector; then payload-set preflight, raw protocol inspection, capped races, logic/upload cases, benign armed probes, sandboxed rendering/dynamic traces and pluggable callback providers.

**Learn:** differential tests, experimental controls, statistical uncertainty, protocol discrepancies, sandbox/capability design, event correlation, async timing and budgets.

**Finish evidence:** known positive/negative outcomes with honest coverage, bounded execution and policy/operator-controlled sends; an imported template is never executed on import.

## P09: Target infrastructure and exposure intelligence

**First slice:** one new source and one useful graph query; then hosting changes, passive intelligence sources, archives, origins/favicons, public repository/registry exposure, secret lifecycle, library advisories and program-specific vocabulary/profiles. Keep this about attributable bounty targets and reportable weaknesses, not corporate posture compliance or fleet administration.

**Learn:** adapter contracts, provenance fusion, entity normalization, caching/TTL, incremental joins, software version semantics, multi-pattern matching and scheduling.

**Finish evidence:** every candidate traces to attributable evidence; passive source reads, direct checks, ownership and authorization remain distinct; secrets are protected.

## P05: HackerOne program and scope intelligence

**First slice:** a saved policy import and explainable decisions; then one HackerOne adapter, durable source jobs, freshness/partial-result handling, secrets and revocation. Add program discovery/watchlists and personal program selection without granting collection permission. Other platform adapters need a demonstrated hunting use case.

**Learn:** normalized selectors, tries/prefix indexes when justified, policy evaluation, revision/delta semantics, validated types, leases/fencing and robust provider integration.

**Finish evidence:** a narrowed/changed policy or failed source reload cannot leave a run executing on obsolete approval; unsupported selectors retain source evidence.

## P10: Temporal knowledge graph and evidence reasoning explorer

**First slice:** two sources support one relation; withdrawing one preserves the other. Then temporal queries, reversible identity resolution, bounty-relevant asset families, stable saved views, qualified relations, paths/communities/centrality and graph briefs. Model hosts, endpoints, identities/roles, observed software and relevant target cloud resources. Corporate device fleets, organizational governance and customer CMDBs are excluded.

**Learn:** multigraph/projection semantics, bitemporal reasoning, dependency invalidation, incremental computation, graph algorithms and access-aware traversal. Compare mutable adjacency with CSR snapshots after profiling; preserve stable public references.

**Finish evidence:** late/partial/conflicting inputs and rebuilds preserve logical truth, explain their effect and never leak through counts/intermediates. Incremental/full agree.

## P15: Bug-hunting memory, hypothesis ranking and ML

**First slice:** explainable rules plus lexical retrieval and a hypothesis record; then terms-aware public corpus, uncertainty/contradiction, novelty, graph signals, filtered vectors/hybrid retrieval, calibration, active learning and bounded local drafting. Measure useful improvements before adding ANN/quantization/neural complexity. Prioritize original bug candidates, code/response variants, attack-chain evidence, false-positive suppression, change significance and likely useful next tests. User validation and HackerOne outcomes supply feedback; detector counts do not measure success.

**Learn:** probabilistic reasoning, information value, learning-to-rank, retrieval metrics, HNSW/IVF/PQ electives, tensor/inference basics, dataset splits, calibration/abstention, model versioning and data-use deletion. Preserve a working rule baseline and rollback.

**Finish evidence:** suggestions cite support and gaps, change when evidence changes, respect policy filters, and beat a declared baseline on held-out task-specific evaluation.

## P12: Fast change alerts and research events

**First slice:** one event and one local test destination; then outbox/retries, signed webhooks where useful, inbox/digest/quiet hours and declared schedule/change triggers. One selected alert destination is sufficient. Extra channels/chat commands/Drive exports are conditional, not a required integration matrix. Reuse callback evidence from P08.

**Learn:** event contracts, authentication/replay prevention, idempotency, delivery state machines, priority queues, backoff, event-time versus arrival-time and API integration.

**Finish evidence:** duplicates/outages/redelivery/cancellation are predictable, destinations are controlled, data is redacted and a verified event cannot authorize arbitrary actions.

## P11: Personal workflow language and tool automation

**First slice:** evolve P01's useful plan format only as real workflows require; then conditions/loops and typed data flow. A general plugin ecosystem and full IDE/LSP product are excluded; small editor conveniences are conditional. Keep callbacks/triggers wired through P12 and reliable worker execution through P14.

**Learn:** grammars, name/type/capability checking, AST/IR, interpretation, dependency analysis and runtime budgets; study Wasm host boundaries only if plugins are justified.

**Finish evidence:** source -> diagnostics or validated plan -> bounded run -> stored results; an extension/loop cannot manufacture sending authority or widen egress.

## P14: Affordable cloud execution and personal operation

**First slice:** deploy the useful monolith with protected configuration, logs, backup and a restore check. Add a worker/queue only for a measured bottleneck or a bounded distributed-systems learning exercise; then fenced leases, checkpoint/retry/dead-letter paths, fairness/global budgets, transient workers and repeatable deployment/rollback. Keep unrelated responsibilities in the monolith. No multi-cloud matrix, mandatory Kubernetes/service mesh, external audit witnesses or enterprise control plane. Keep data migrations, retention and personal activity evidence usable.

**Learn:** at-least-once delivery, outbox, deduplication, consistency, message ordering, distributed tracing, failure injection, IAM/workload identity, network/storage costs and service lifecycle. Compare throughput/latency/cost with the in-process baseline.

**Finish evidence:** crashes between effect/ack, stale workers, duplicate/reordered messages and partial outages cannot silently corrupt state or expand permitted traffic.

## P16: Specialized mobile and protocol hunting — conditional

**Activation:** a selected authorized program actually uses the format and presents a worthwhile hunting opportunity. No commitment to cover every binary/protocol.

**First slice:** one saved format and one useful extraction/change question; then official-release tracking, mobile resource/static inspection, schema reconstruction, protocol decoding and structural API-gap candidates. Reuse all earlier evidence machinery.

**Learn:** binary/container formats, schema evolution, bounded parsing, language/protocol semantics and source mapping; native readers keep explicit unsupported/partial states.

**Finish evidence:** fixture -> extracted surface -> version diff/search -> reviewable candidate/report, without treating decoding as full validation or inferred routes as permission.

## Project structure

Use one Git repository and one Cargo workspace. Keep the existing `crates/graph` library
and add one application package when CLI/API behavior needs it. This is a proposed layout;
create each module only when its behavior is being built.

### Reference layout

This tree illustrates the application as P01/P02 grow; it is a design reference, not
an upstream code excerpt or a request to create every file now.

```text
sentinel-rs/
├── Cargo.toml                 # virtual workspace; local path dependencies
├── Cargo.lock                 # shared dependency resolution
├── README.md                  # purpose, scope and navigation
├── AGENTS.md                  # mentoring agreement
├── docs/
│   ├── PROJECT_ROADMAP.md
│   ├── FEATURES.md
│   └── STUDY_GUIDE.md
└── crates/
    ├── graph/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs         # small public graph API
    │       └── graph.rs       # representation, identity and algorithms
    └── sentinel/              # proposed application package
        ├── Cargo.toml
        ├── src/
        │   ├── main.rs       # startup and dependency construction
        │   ├── lib.rs        # module declarations and application API
        │   ├── config.rs     # settings and validation
        │   ├── discovery.rs  # observations, normalization and graph ingestion
        │   ├── runs.rs       # lifecycle, orchestration and cancellation
        │   ├── capture.rs    # P02 artifact versions and change behavior
        │   ├── search.rs     # P02 queries, candidates and exact verification
        │   ├── cli.rs        # arguments, presentation and exit behavior
        │   ├── http.rs       # Axum routing and transport mapping
        │   ├── storage.rs    # chosen store, transactions and record mapping
        │   └── tools.rs      # subprocess execution, limits and cleanup
        ├── tests/            # integration contracts and synthetic fixtures
        └── benches/          # benchmarks for measured questions
```

Graph code stays independent of HTTP, database clients, subprocesses and cloud SDKs.
Keep small modules as files; introduce directories when coherent subparts emerge.

Only the graph package exists today. Root `src/main.rs` is unused scaffold under the
virtual workspace manifest; decide its move/removal when implementing the application.
The proposed application modules are responsibilities, not a checklist to scaffold now.

### Lightweight DDD

Let domain vocabulary and invariants guide types: asset/run/finding identities,
normalized host values, observations versus inferred relations, and valid run transitions.
Keep a rule beside the behavior/type that owns it. Choose aggregate boundaries around
what must change consistently; the entire graph need not be one aggregate.

Discovery, runs, capture, search and findings are candidate feature boundaries. Refine
them from actual workflows; a module is not automatically a bounded context or service.
Use domain/application distinctions within modules when helpful, without requiring
`domain/`, `application/` and `infrastructure/` folders for every feature.

### Dependency and ownership rules

- CLI/HTTP -> application features -> graph and concrete I/O adapters. Pass ordinary
  inputs/results across those boundaries; HTTP responses do not belong in graph/runs.
- Features own their behavior and related types. Add capture, search and findings as
  feature modules later; avoid a growing global `models`, `utils` or `common` bucket.
- Start with concrete types/functions. Introduce a trait when real alternative behavior
  or a focused testing boundary needs it; no repository/service/DTO layer per operation.
- Start with focused operation/feature errors and map them at the CLI/HTTP boundary.
  Split transport/storage records from feature types when their requirements diverge.
- Keep internals private; expose only what callers need. A file split does not require
  a larger public API. Cross-feature orchestration belongs with the owning use case;
  avoid circular feature calls or a central application object containing all behavior.

### Grow in response to friction

Split `discovery.rs` into `discovery/` when parsing, normalization and ingestion become
coherent subparts. Similarly split HTTP routes or storage by feature when navigation
becomes difficult. Keep related code together; use no arbitrary line-count threshold.

Extract a crate for an understood reuse/dependency/build boundary. Extract a service
for runtime isolation/scaling or a bounded distributed-systems exercise. Both remain
in this repository. Services need explicit data ownership, message versions, retries/
idempotency, cancellation, budgets and recovery; moving code to a network is not itself
an improvement.

Keep synthetic fixtures near consumers; share only for actual reuse. Keep private data
in controlled ignored storage. Add migrations alongside the application/store when needed,
and deployment/scripts only for working operations. No generic plugin framework, DI
container, event bus or crate per feature before a concrete need.

References: [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html),
[Rust modules and privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html),
[packages and targets](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html),
[Axum workspace](https://github.com/tokio-rs/axum).
