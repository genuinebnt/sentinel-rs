# Sentinel study guide

Follow the current increment in [PROJECT_ROADMAP.md](PROJECT_ROADMAP.md). Pick one source,
read its usage, then trace one operation. Identify an invariant, ownership/API choice,
cost and failure case; adapt the idea yourself. Keep one main increment and at most one
optional deeper exercise. Reduce the challenge when stuck; resume a useful feature.

Sources below are reading options, not dependencies or authorship certifications. Inspect
the exact revision before teaching from code; snippets follow [AGENTS.md](../AGENTS.md).
Use the selected Rust version's standard-library contracts. Compiler-private/nightly APIs
are study material, not assumptions for stable application code.

## Rust and CS prompts by project

| Context | Rust/library/CS focus | Small evidence or transfer question |
| --- | --- | --- |
| P01: graph identity | Newtypes, derive contracts, `Copy` versus `Clone`, `HashMap::entry`, borrowed lookup through `Borrow`, `Vec::get`, `Option` | Demonstrate duplicate/unknown identities and explain value equality versus stable identity. |
| P01: traversal | `VecDeque`, sets, borrowed iterators, `impl Iterator`, lifetime relationships, `Iterator::filter_map`, collection versus streaming | Trace a cycle and disconnected graph by hand; explain traversal costs for your representation. |
| P01/P05: policy | `str` versus bytes, UTF-8 boundaries, `strip_suffix`, `split`, `Cow`, `From`/`TryFrom`, structured errors | Check host-label boundaries, exclusions and malformed inputs without sending requests. |
| P01/P02: persistence | `Read`/`Write`, buffering, RAII/`Drop` versus explicit durability, checked arithmetic, byte order, error propagation with `?` | Demonstrate reload and interrupted-write/rollback behavior; explain what is durable. |
| P01/P04: API boundaries | Traits and trait objects, generics and bounds, associated types, `From` error conversion, `Option::transpose`, `as_ref`/`as_deref` | Exercise the same use case through CLI/HTTP and distinguish transport from domain errors. |
| P01/P06/P14: concurrency | Futures, `Send`/`Sync`, `Arc`, guard lifetimes, channels, bounded queues, cancellation; later `Pin`/`Unpin` when polling requires it | Use a slow fixture and cancellation to demonstrate bounded work and released resources. |
| P03/P08: analysis | Recursive enums, exhaustive matches, visitors, spans, `Cow`, `mem::take`/`replace`, subprocess I/O and cleanup | Trace one extracted fact to a source location; distinguish inference and unsupported syntax. |
| P02/P03: matching | Finite automata, representation enums, reusable scratch state, byte-oriented processing, allocation control | Explain compilation versus search work and test difficult/malformed inputs. |
| P02: trigram/search | `slice::windows` where byte semantics apply, `char_indices` for Unicode boundaries, sorted vectors, posting-list intersection, iterator state and borrowing | Compare with exact scan, including short/Unicode queries and candidate false positives. |
| P02/P14: storage/layout | B-tree invariants, page formats, WAL/recovery, buffer ownership, alignment and checked offsets; `unsafe` only with a stated safety invariant | Explain recovery/publication invariants and compare a simple representation before optimizing. |
| P01/P14: cloud operation | Configuration validation, error context, structured tracing, graceful shutdown and resource lifecycles | Redeploy/restore synthetic data and diagnose one failure; state actual resource/cost assumptions. |
| P14: reliability | Explicit state machines, ownership across task boundaries, atomics and memory ordering only where justified; deterministic concurrency models | Inject duplicates, delay and crash-between-effect/ack; explain the consistency promise. |

## Foundations and graph work

| Stories / project context | Projects and first reading target | Rust/CS lesson and a small question |
| --- | --- | --- |
| GR-01/02/03; P01 graph work | [petgraph](https://github.com/petgraph/petgraph): graph types, one insertion path and BFS/DFS API. | Identity, adjacency, borrowed iteration and traversal state. What remains valid after insertion or removal? |
| GR-08/09; P01 identity follow-up | [slotmap](https://github.com/orlp/slotmap): key semantics; [rustc_index](https://github.com/rust-lang/rust/tree/main/compiler/rustc_index): typed indices. | Generational handles, newtypes, `PhantomData` and stable public identity versus compact local indexing. Which mixup does each prevent? |
| COL-01, REL-01; P01/P02 storage | [SQLx](https://github.com/transact-rs/sqlx): a transaction/pool example; [Turso](https://github.com/tursodatabase/turso): one pager/cursor operation only when exploring internals. | Durability versus serialization, transaction scope, async lifetimes, page invariants. What survives an interruption? |
| UI-01, GOV-09; P01/P04 interfaces | [Axum](https://github.com/tokio-rs/axum): state/extractor/error example; [Tower](https://github.com/tower-rs/tower): service/middleware boundary. | Traits, associated types, typed errors and dependency direction. Which behavior belongs outside HTTP? |
| UI-01/04, OPS-02/03, FIND-03/04; P01/P04 interfaces | [clap](https://github.com/clap-rs/clap): one derive-based subcommand/validation example. | CLI contracts, enums, parsing versus execution, useful help/errors and pipe-friendly output. Can one command exercise the feature without a console? |

## Collection, protocols and manual investigation

| Stories / when | Projects and first reading target | Rust/CS lesson and a small question |
| --- | --- | --- |
| COL-02/06/07, TRF-01, WB-01/08; P01, then P06 | [Hyper](https://github.com/hyperium/hyper): client/server and body-stream examples; [rustls](https://github.com/rustls/rustls): connection configuration examples. | HTTP framing, streaming bodies, ownership, protocol state and trust configuration. What is buffered, and when is a body too large? Use established TLS primitives. |
| SRC-02/04, IO-03; source collection/OOB | [Hickory DNS](https://github.com/hickory-dns/hickory-dns): a resolver/client example and record model. | Async resolution, record enums, cache/TTL, time/vantage and cancellation. Why is a DNS lookup different from reading a passive index? |
| COL-04/07, GOV-03/04, WB-06/09; before concurrent live activity | [governor](https://github.com/boinkor-net/governor): quota/keyed limiter docs and examples; [Tokio](https://github.com/tokio-rs/tokio): bounded channels/time. | Rate versus concurrency versus total budget, monotonic time, keyed state and backpressure. Do many workers accidentally multiply the allowed rate? |
| COL-08, REL-06, IAM-02; P01 execution | [mini-redis](https://github.com/tokio-rs/mini-redis): follow connection ownership and shutdown. | Futures, task lifecycle, `Send`/`Sync`, guard lifetimes and cooperative cancellation. Who owns cleanup after a failed task? This is a teaching project. |
| TRF-06, IO-02; streaming protocols | [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite): one stream/client example. | `Stream`/`Sink`, split ownership, close handshakes and bounded frame handling. How does a slow consumer affect memory? |
| COL-09, DET-09, CHG-03; headless browser | [chromiumoxide](https://github.com/mattsse/chromiumoxide): one browser/navigation example and event loop. | Child processes, event-driven APIs, resource cleanup and browser request interception. Browser automation itself does not provide network isolation; where must policy enforcement live? |
| IAM-04/05, GOV-09; auth-flow analysis | [oauth2-rs](https://github.com/ramosbugs/oauth2-rs): PKCE/client configuration example; [RustCrypto traits](https://github.com/RustCrypto/traits): interfaces for vetted primitives. | Validated/newtype state, secret handling and protocol-specific verification. A correct OAuth client helps study legal flows; it is not a complete vulnerability oracle. Use the relevant RFCs for analysis. |
| PAR-04, MOB-03, REL-07; API/protocol reconstruction | [Prost](https://github.com/tokio-rs/prost): message/encoding examples; [gRPC Rust / tonic](https://github.com/grpc/grpc-rust): current RPC examples. | Schema evolution, generated message types, streams and transport boundaries. What does decoding a message prove, and what remains unknown? |

## Parsing, evidence and detection

| Stories / when | Projects and first reading target | Rust/CS lesson and a small question |
| --- | --- | --- |
| PAR-01/03, DET-03; first HTML reader | [html5ever](https://github.com/servo/html5ever): tokenizer/tree-construction examples. | Token streams versus trees, malformed-input recovery and sink interfaces. Why does finding a string differ from understanding a document? |
| PAR-01/02/06, DET-02/09; JS/TS analysis | [Oxc](https://github.com/oxc-project/oxc): parser/visitor examples; [SWC](https://github.com/swc-project/swc): one visitor/fold. | Arenas, lifetimes, spans, exhaustive enum matching, traversal versus mutation and semantic binding. Choose one reader first; do not integrate both for completeness. |
| PAR-06/07/10, DET-02; additional source languages | [Tree-sitter](https://github.com/tree-sitter/tree-sitter): Rust binding usage and error nodes; [ast-grep](https://github.com/ast-grep/ast-grep): one structural pattern. | CST/AST distinctions, incremental parsing, captures and recovery gaps. When must a recovered tree be called partial analysis? |
| PAR-04/07, MOB-03; GraphQL | [apollo-rs](https://github.com/apollographql/apollo-rs): parser usage and syntax-tree access. | Grammar versus schema semantics, typed syntax nodes and diagnostics. Does parse success validate a server's authorization? |
| PAR-05/07, MOB-01; binary-format reading | [wasm-tools](https://github.com/bytecodealliance/wasm-tools): `wasmparser` reader and validator examples. | Streaming binary parsing, checked lengths, zero-copy lifetimes and validation stages. Wasm techniques inform bounded binary readers; APK/IPA unpacking needs its own format contract. |
| SEC-01/02, DET-03; secret candidate filtering | [aho-corasick](https://github.com/BurntSushi/aho-corasick): multi-pattern matching choices; [regex](https://github.com/rust-lang/regex): matching APIs then a narrow automata path. | Prefix prefiltering, automata, match semantics and bounded input. Why does a matching key shape remain an unverified secret candidate? |
| CHG-01/02/04, FIND-02/04; useful diffs | [difftastic](https://github.com/Wilfred/difftastic): a small syntax-aware diff flow. | Tree matching, ambiguity, presentation versus semantic equivalence and fallback behavior. Which reorderings are unsafe to normalize away? |

## Search, intelligence, operator knowledge

| Stories / when | Projects and first reading target | Rust/CS lesson and a small question |
| --- | --- | --- |
| SRCH-01/02/03, OPS-02/06; after a simple search baseline | [ripgrep](https://github.com/BurntSushi/ripgrep): ignore/search/result flow; [Tantivy](https://github.com/quickwit-oss/tantivy): tokenizer/index/query example. | Iterators, offsets, posting lists, immutable segments, memory versus I/O and incremental indexes. Which filtering must happen before results can be counted or returned? |
| SRCH-02, QA-07; trigram elective | [Google Code Search](https://github.com/google/codesearch): regexp-to-trigram query logic (Go); [Tantivy n-grams](https://github.com/quickwit-oss/tantivy/blob/main/src/tokenizer/ngram_tokenizer.rs): Rust offsets/tokenization. | Conservative candidate plans, posting intersection, byte/Unicode choices and exact verification. A tokenizer is not a complete trigram index. |
| SRCH-03/08, OPS-02; compact dictionaries/fuzzy lookup | [fst](https://github.com/BurntSushi/fst): set/map construction and automaton search examples. | Sorted construction, compact immutable representations and traversal. What tradeoff would make a plain map preferable first? |
| SRCH-05/06/07, INT-08; vector elective | [Qdrant](https://github.com/qdrant/qdrant): architecture, then one filtered segment/search operation. | HNSW, filtering/recall, segment lifecycle and concurrency. Study one algorithmic component without importing the distributed database architecture. |
| INT-02/04/07/08/09/10/11, CORP-05; hypothesis/retrieval/model work | [Candle](https://github.com/huggingface/candle): one inference example when a model is justified; revisit petgraph/Tantivy for evidence dependencies and cited retrieval first. | Explicit claims, traceable support, dependency invalidation and evaluation before learned ranking; later tensor layout, batching and inference memory. Does a local model improve the task over rules? It cannot supply action authority. |
| CORP-01/02/03/04, OPS-06/07, FIND-01/06/08; knowledge and lifecycle | Revisit [SQLx](https://github.com/transact-rs/sqlx) transaction examples and [Tantivy](https://github.com/quickwit-oss/tantivy) deletion/index lifecycle. | Policy-carrying records, reversible transitions and derived-data cleanup. Can a policy change remove an item from every shared retrieval path? No model is required for a useful notebook or tracker. |

## Workflows, operation and quality

| Stories / when | Projects and first reading target | Rust/CS lesson and a small question |
| --- | --- | --- |
| FLOW-03, TRF-05, SRCH-01; small query/DSL syntax | [winnow](https://github.com/winnow-rs/winnow): one parser example and error handling; revisit the [Rust compiler](https://github.com/rust-lang/rust) only for a focused later diagnostics/typed-IR comparison. | Parser composition, borrowed input, spans, syntax versus typed plans and error recovery. Can the smallest useful grammar express the present workflow? |
| FLOW-07, UI-05; personal workflow diagnostics | Revisit [winnow](https://github.com/winnow-rs/winnow) and focused compiler diagnostics. | Useful token errors, validation and readable plans come first. A full language-server/IDE product is excluded; deeper editor-protocol study can be a separate learning exercise if requested. |
| FLOW-06, REL-07; plugin capability boundary | [Wasmtime](https://github.com/bytecodealliance/wasmtime): embedding, host functions and resource-limiting docs/examples. | Capabilities, explicit host imports, fuel/memory/time limits and isolation. A Wasm runtime does not automatically make an overpowered host function safe. |
| IO-01/02, NOTIFY-01/03/04/05, REL-03; personal alerts and activity evidence | Revisit [Axum](https://github.com/tokio-rs/axum)/[Tower](https://github.com/tower-rs/tower), [RustCrypto traits](https://github.com/RustCrypto/traits) and [Tokio](https://github.com/tokio-rs/tokio) time/queue examples. | Request authentication, replay windows, idempotency/outbox, bounded retries and useful personal activity records. Start one alert destination; chat/Drive are conditional, and independent audit-witness infrastructure is excluded. |
| REL-06/07, QA-06; measured CPU/shared-state bottlenecks | [Crossbeam](https://github.com/crossbeam-rs/crossbeam): channel APIs before lock-free internals; [Loom](https://github.com/tokio-rs/loom): one small model. | Threaded versus async execution, bounded queues, atomics/memory ordering only when justified. Which interleavings does the model cover, and which distributed failures does it omit? |
| REL-08, COL-03, QA-05; observability | [tracing](https://github.com/tokio-rs/tracing): spans/subscribers and instrumentation examples; [Criterion.rs](https://github.com/bheisler/criterion.rs): one representative benchmark. | Causality across tasks, field cardinality, measurement setup and noise. What can be observed without recording target contents or credentials? |
| REL-05/09, GOV-09; GCP deployment | [Google Cloud Rust libraries](https://github.com/googleapis/google-cloud-rust): the specific service's client example and authentication docs when needed. | Credential lifetimes, IAM boundaries, retry/deadline behavior and resource cleanup. Generated SDK internals are not evidence for human-authored teaching snippets; inspect example provenance. Verify platform limits/costs in official GCP docs. |
| QA-01/02/03/06, COL-02, GOV-06; testing boundaries | [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs): local server/expectation example; [proptest](https://github.com/proptest-rs/proptest): generators/shrinking; [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz): a small harness. | Contract fixtures, generated invariant cases, minimal counterexamples and parser fuzz boundaries. Add the method that exposes the actual risk, not every tool at once. |

## Recon tools as domain study

[subfinder](https://github.com/projectdiscovery/subfinder) and
[ProjectDiscovery httpx](https://github.com/projectdiscovery/httpx) remain useful Go
references for SRC/COL/FLOW adapter behavior, schemas and source attribution. Learn
their capabilities from their own documentation, classify sends before enabling them,
and begin integration with saved fixture output. A popular tool's defaults do not
establish program authorization or satisfy Sentinel's resource limits.

## Deeper source entry points

Use these after the corresponding overview above. Links identify study locations,
not pinned teaching excerpts; verify the revision before use.

| Topic | Entry points | Question to follow |
| --- | --- | --- |
| Standard library and typed indices | [Rust library](https://github.com/rust-lang/rust/tree/main/library), [alloc](https://github.com/rust-lang/rust/tree/main/library/alloc/src), [IndexVec](https://github.com/rust-lang/rust/blob/main/compiler/rustc_index/src/vec.rs) | How do ownership, allocation and typed indexing support the public contract? |
| Regex and trigram planning | [regex-syntax](https://github.com/rust-lang/regex/tree/master/regex-syntax), [regex-automata](https://github.com/rust-lang/regex/tree/master/regex-automata), [Code Search query planning](https://github.com/google/codesearch/blob/master/index/regexp.go) | How does parsing produce a conservative candidate plan that exact matching can verify? |
| Syntax visitors | [SWC visitor crate](https://github.com/swc-project/swc/tree/main/crates/swc_ecma_visit) | How do traversal, mutation and source spans interact? |
| Database storage | [Turso storage](https://github.com/tursodatabase/turso/tree/main/core/storage), [B-tree](https://github.com/tursodatabase/turso/blob/main/core/storage/btree.rs), [pager](https://github.com/tursodatabase/turso/blob/main/core/storage/pager.rs) | Which invariants connect tree updates, pages and recovery? |
| GCP execution | [Cloud Run overview](https://docs.cloud.google.com/run/docs/overview/what-is-cloud-run), [Pub/Sub subscribers](https://docs.cloud.google.com/pubsub/docs/subscriber) | What execution, delivery and retry contracts would a worker need? |

## Database internals

Learn through the engine's contract -> one source operation -> an optional small model.
Use an existing store first; a general DBMS remains a separate future project.

| Where | Theory and useful experiment |
| --- | --- |
| P01 imports; P02 versions; P04 findings | Transactions/ACID, uniqueness, logical commit boundaries; interrupt writes and demonstrate restart/rollback. A long collection need not be one transaction. |
| P02 history queries | B+ trees: fanout, leaf links, splits, pages, key order and covering/secondary indexes; inspect range plans before building a tree. |
| P02 concurrent reads/writes | MVCC/isolation, snapshots, conflicts and cleanup; reproduce visibility for a real multi-query read. |
| P02 growing search index | LSM/SSTables, segments, merge/tombstones, compression and read/write/space amplification; compare against scan/simple index. Bloom filters earn their place through avoided negative reads. |
| P02 disk storage | Page/block layouts, offsets, endianness, checksums, format versions, buffering/cache locality and cold/warm reads; try a bounded derived format only if useful. |
| P02/P10 publication | Metadata, blobs and indexes need coherent version publication, orphan cleanup and rebuild; study reader lifetimes/reclamation. |
| Slow history/search queries | Query plans, selectivity/skew, cardinality estimates, joins and index costs. |
| P14 durable execution | WAL/checkpoints, isolation/recovery and backup/restore; distinguish buffered completion, crash recovery and power-loss durability. P12/P14 outbox/retries cover external-effect boundaries. |

Artifact history is not database MVCC; an activity log is not a recovery WAL. One database
transaction does not cover arbitrary files, object storage or remote effects. Specify the
failure model. Engine tree variants differ; study their actual invariants. Bloom filters
must preserve no false negatives for their indexed set. Record what transfers to a DBMS.

Study sources:
- [redb](https://github.com/cberner/redb): its documented copy-on-write B+ trees provide
  a Rust entry point for page/index and transaction design. Start with a read/write
  transaction example and the design document, then one tree operation.
- [Turso](https://github.com/tursodatabase/turso): study one pager/B-tree/recovery path
  in context. Compare its invariants rather than assuming every tree is interchangeable.
- [PostgreSQL B-tree implementation](https://www.postgresql.org/docs/current/btree.html)
  and [MVCC introduction](https://www.postgresql.org/docs/current/mvcc-intro.html):
  connect ordered pages and version visibility to the engine's actual guarantees.
- [PostgreSQL page layout](https://www.postgresql.org/docs/18/storage-page-layout.html):
  inspect how page headers, records and references represent durable state.
- [RocksDB](https://github.com/facebook/rocksdb) and its
  [Bloom filter design](https://github.com/facebook/rocksdb/wiki/RocksDB-Bloom-Filter):
  study LSM/segment/filter tradeoffs; this is a C++ reference, not a Rust snippet source.

## Systems and performance

Working baseline -> representative workload -> profile -> one hypothesis/change -> verify
behavior/quality -> comparable repeated measurements -> integrate if worthwhile. Bounded
learning experiments can stay separate. Compare exact algorithms to a simple reference;
ML/approximate search needs held-out quality evaluation and explicit cost/accuracy tradeoffs.
Record relevant data/seed, revision, compiler/target flags, machine, concurrency, cache state
and variance. Measure end-to-end latency/throughput, allocation/memory, I/O/storage and cost
as the question requires. Use fixtures/owned infrastructure, including skew and failure cases.

| Opportunity | Topics to investigate when useful |
| --- | --- |
| Graph/search/parser representations | Custom DSA, dense IDs/CSR, interning, contiguous layouts, cache locality, alignment/padding and false sharing. |
| Capture/parsing/query buffers | Bytes/slices/references, avoiding allocations/copies, streaming, arena lifetimes/reset/destructors and validated zero-copy; account for retained backing memory, alignment, endianness and validity. |
| Concurrent stages and workers | Async I/O versus CPU parallelism, threads, bounded queues, batching, cancellation, contention and oversubscription. |
| Shared-state bottleneck or small model | Atomics/memory ordering, happens-before, lock-free progress, ABA/reclamation; compare a lock/channel baseline. |
| Measured kernels | Branch prediction, inlining, `const fn`, monomorphization, code size and instruction cache. `const fn` does not force compile-time execution. |
| I/O and storage | Syscall counts, buffering/batching, compression, versioned file formats, varints/deltas, bitmasks and dense/sparse/compressed bitmaps; preserve evidence and durability. |
| Search/vector/ML batches | SIMD tails/alignment/CPU dispatch and scalar fallback; GPU setup/transfer/VRAM costs and numerical/quality checks against CPU baseline. |
| Unsafe/assembly exercise | State safety/ABI invariants, compare a safe reference, test supported architectures and retain fallback. Use established crypto/TLS primitives. |
| All modules | Idiomatic ownership/errors/iterators, useful traits/generics versus dynamic dispatch; weigh build time and abstraction costs. |

| Evidence tool | Question and limit |
| --- | --- |
| perf/flamegraphs; allocation profiler | Where time/allocations concentrate; samples do not explain causality, and instrumentation changes timing. |
| Benchmarks | Scaling and baseline/change distributions; control setup, cache, machine and dead-code artifacts. |
| Loom | Small instrumented concurrency models under explicit bounds; not proof of the native/distributed system. |
| Miri | Certain UB/races on executed supported paths; limited native/FFI/assembly coverage. |
| Property/differential/fuzz tests | Invariants and malformed-input counterexamples; use the method relevant to the risk. |
| Jepsen or a fault/history harness | P14 distributed histories under injected faults with an explicit consistency checker; does not establish pointer safety. |
| Stress/recovery runs | Saturation, tail latency and crash/retry behavior under the stated failure model; not all possible failures. |

On macOS, consider native sampling tools; use Linux perf/counters in a suitable Linux
environment. Verify current tool support before setup. For disassembly, inspect a sourced
small Rust kernel, predict costs and compare codegen; record ISA/features/compiler/flags.
Handwritten assembly can teach calling conventions, registers/clobbers and memory assumptions.
Shorter assembly is not evidence of speed. Use public/toy code on hosted Godbolt, keep
private material local, and validate assembly natively against the reference; Miri cannot
validate arbitrary assembly.

Focused sources and tools:
- [dhat-rs](https://github.com/nnethercote/dhat-rs), [Criterion](https://github.com/bheisler/criterion.rs), [Loom](https://github.com/tokio-rs/loom), [Miri](https://github.com/rust-lang/miri).
- [Jepsen](https://github.com/jepsen-io/jepsen), [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph), [samply](https://github.com/mstange/samply), [Compiler Explorer/Godbolt](https://github.com/compiler-explorer/compiler-explorer).
- [Rust assembly reference](https://doc.rust-lang.org/reference/inline-assembly.html), [memchr](https://github.com/BurntSushi/memchr), [stdarch](https://github.com/rust-lang/stdarch), [Crossbeam](https://github.com/crossbeam-rs/crossbeam).
- [Rayon](https://github.com/rayon-rs/rayon), [bumpalo](https://github.com/fitzgen/bumpalo), [zerocopy](https://github.com/google/zerocopy), [roaring-rs](https://github.com/RoaringBitmap/roaring-rs).
- [Zstandard](https://github.com/facebook/zstd).
