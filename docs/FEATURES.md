# Sentinel feature catalog

## How to use this catalog

This is the personal HackerOne bug-hunting backlog. Stories should improve finding,
understanding, validating or reporting valuable original vulnerabilities, or provide
necessary personal operation. The owner implements the system and learns Rust/CS while
building useful workflows. Each row uses the operator's perspective.

Use [Product scope](../README.md#product-scope) for product boundaries. The dispositions
below narrow individual stories; they do not indicate implementation progress.

| Stories / area | Disposition | Meaning |
| --- | --- | --- |
| OPS-08 | Excluded | No shared-workspace/team assignment or enterprise collaboration product. ID retained for history. |
| PAR-05, MOB-01/02 | Conditional | Wasm/mobile readers activate for a selected program and a concrete hunting question. |
| TRF-07/08 | Conditional | Specialized TLS options/browser extension only when they improve the actual target workflow. |
| FLOW-06 | Conditional | Sandbox plugins only if personal fixed adapters cannot express needed work. No commercial ecosystem. |
| IO-04 | Conditional | Disposable webhook/mail receiver when the target feature needs that evidence. |
| NOTIFY-04/05 | Conditional | Remote chat commands/Drive export are conveniences, not required for product completion. |
| REL-07 | Conditional | Extract/scale a worker for a demonstrated bottleneck or explicit bounded learning exercise. |
| Enterprise facets of GR-07, REL-03, FLOW-07, FIND-05, REL-09 | Excluded | Corporate inventory/governance, audit witnesses, full IDE/LSP, ticket sync and infrastructure matrices are removed. |

Broad format/provider/class lists are options, not completeness requirements. Start with
GR-01/02 and the active [roadmap increment](PROJECT_ROADMAP.md#current-work). Read the
[study guide](STUDY_GUIDE.md) for relevant concepts and source targets.

## Rules shared by every story

- Program authorization, workbench configuration, data-use permissions, and payment
  eligibility are different concepts. A discovered asset, imported policy, model score,
  shared infrastructure, or third-party result cannot grant target authorization.
- Collector and tool-originated requests require an admitted, attested scope version;
  exclusions and the global denylist win. Check every destination and redirect. Enforce
  program rules, automation permissions, time windows, bounded resources, cancellation,
  and audit. Adaptive throttling slows further on blocks; it never rotates identity
  or transport to continue past a block. Honor applicable robots rules for crawling.
- Active tests with effects require a deliberate operator workbench action for each
  bounded run. Automation may prepare them. Only individually reviewed, named benign
  probes may run unattended after explicit arming for an eligible scope version.
  Arming is invalidated by a scope revision; a loop or plugin cannot bypass the gate.
- Workbench forwarding has its own explicit policy (TRF-02/03); it is not an exception
  inherited by collectors, automatic probes, or knowledge ingestion. Private proxy
  access is restricted to the operator's devices and authorized use.
- Preserve provenance, scope decisions, and distinctions between observed, inferred,
  stale, withdrawn, contradicted, unknown, and confirmed. Incomplete coverage cannot
  establish absence. Candidates are leads; the operator validates findings.
- Treat pages, imports, tool results, templates, notes derived from targets, and
  retrieved passages as untrusted data. They cannot become privileged instructions.
- Sensitive values and traffic stay encrypted and access controlled. Mask secrets in
  ordinary views and redact exports/notifications. Full encrypted backups are an
  explicit separate operation. Found credentials are never automatically tested.
- Target-derived content, traffic, secrets, findings, and embeddings never go to an
  external model service. Models handling them run on operator-controlled infrastructure.
  Approved external data sources receive only permitted identifiers, not content.
- Program/project data-use boundaries apply within graph traversal, counts, caches,
  search, learning, exports, and notifications, including intermediate results.
- Prototype against synthetic fixtures/local labs. Add the controls relevant to a
  live capability before that capability sends, exposes, or persists sensitive data.

## GOV: Programs and policy

| ID | User story | Acceptance note |
| --- | --- | --- |
| GOV-01 | Manage programs and separately authorized scopes so each engagement has the right boundaries. | Support hostname/wildcard, exact IP and CIDR selectors; retain policy source, exclusions, permitted operations and revision. |
| GOV-02 | Attest a policy and preview admission so I can explain what is permitted before running anything. | Decision includes reason and version; preview sends nothing; unsupported policy fails closed. |
| GOV-03 | Apply a global denylist and kill switch so I can stop activity immediately. | Denylist overrides every scope; running collectors, tools, macros and probes check cancellation before further sends. |
| GOV-04 | Encode rules of engagement so prohibited tools/classes, times and automation remain unavailable. | Rules belong to the attested revision; unknown rules use the restrictive interpretation. |
| GOV-05 | Import HackerOne policy so I avoid retyping programs and their asset types. | Preserve policy, exclusions, rewards/eligibility, revisions and unsupported selectors; importing never grants testing permission. Other platforms require an actual hunting use case. |
| GOV-06 | Review scope drift so collection does not continue on stale approval. | Structural target/rule changes invalidate affected approval and require re-attestation; source failure or reload ambiguity pauses affected work. |
| GOV-07 | Discover new programs, rewards and acquisitions so I can decide what to research. | Platform reads create notifications/watchlist evidence, never automatic target scope or target traffic. |
| GOV-08 | See platform-source health and safely refresh large imports so policy evidence remains trustworthy. | Show source age, partial/failed retrieval and checkpointed progress; stale workers cannot publish or reactivate approval. |
| GOV-09 | Protect my platform credentials so I control metadata access. | Least privilege, personal revocation and protected storage; metadata access is distinct from target authorization. No enterprise identity-management plane. |
| GOV-10 | Distinguish organizational assets, testing permission and bounty eligibility so I do not conflate them. | Source asset types, eligibility and policy assertions retain their own provenance and revisions. |

## SRC: Recon sources and infrastructure

| ID | User story | Acceptance note |
| --- | --- | --- |
| SRC-01 | Enable sources per program so I choose what identifiers leave my instance. | Missing setting is off; record source terms, credentials, permitted identifiers, rate limits and output schema; Isolated programs cannot enable external intelligence sources. |
| SRC-02 | Discover hosts from certificate transparency and passive DNS so I can expand my known surface. | Attribute every result; third-party results do not authorize contact; distinguish passive index reads from DNS requests. |
| SRC-03 | Import internet-scan, RDAP/WHOIS, ASN/BGP, reputation and web-archive evidence so I can connect infrastructure. | Support adapters such as Shodan, Censys, FOFA, ZoomEye, Netlas, LeakIX, URLScan, VirusTotal, OTX, GreyNoise and RIPE-style data without assuming their terms permit reuse. |
| SRC-04 | Collect authorized DNS/TLS observations so I can follow address and certificate changes. | A lookup or handshake is an outbound operation; record vantage, observation time and source. |
| SRC-05 | Track hosting/provider/CDN changes so infrastructure moves become visible. | Shared/reassigned IPs and names do not prove ownership; preserve historical association. |
| SRC-06 | Identify cloud buckets, functions, registries and CDN origins so exposed infrastructure is discoverable. | Attribute by evidence; mere naming does not authorize contact; anonymous checks are bounded and do not download object contents. |
| SRC-07 | Find candidate origins and matching favicons so forgotten assets become leads. | Use historical/certificate/public-index evidence; matches remain unverified and independently scope checked. |
| SRC-08 | Watch public organization repositories, package/container registries, Postman workspaces and pastes so public leaks are noticed. | Public, attributable sources only; no private access, bulk cloning or automatic credential validation. |
| SRC-09 | Track public engineering pages, job posts and status pages so stack hypotheses improve. | Label inferred technology separately from directly observed versions; no scope expansion. |
| SRC-10 | Register recon tools so existing tools contribute normal observations. | Include Amass/Chaos, subfinder/httpx, crawlers, authorized port discovery and hostname mutation families; adapters declare side effects and enforce controlled egress. |

## COL: Collection and live ingest

| ID | User story | Acceptance note |
| --- | --- | --- |
| COL-01 | Import saved observations so I can use Sentinel without contacting a target. | Validate schema, preserve source/time and handle duplicate delivery deliberately. |
| COL-02 | Collect pages, scripts and source maps so the surface stays current. | Bounded crawl follows scope/redirect/robots rules, identifiable User-Agent, per-host limits and run budgets. |
| COL-03 | Inspect run progress, failures and coverage so I know what actually completed. | Include fetched/skipped/failed outcomes, reasons, bytes and partial results; a failed run is not a clean negative result. |
| COL-04 | Start or schedule collection and cancel it so I control cadence. | Coalesce equivalent work; expose a run identity; cancellation stops further sends and records interruption. |
| COL-05 | Ingest my own captured traffic so browsing produces useful evidence. | Strip credential headers at the passive knowledge boundary; retain sensitive raw traffic only under its separate storage policy. |
| COL-06 | Use my own proxy/VPN or an approved regional vantage so normal infrastructure and geography can be tested. | Same scope, identity, audit and limits on every path; never retry around a target block by rotating origin. |
| COL-07 | Slow collection automatically when targets struggle so configured rates remain ceilings. | Backoff considers throttling, latency/errors and WAF-like refusal; bounded retry/jitter never becomes an unending loop. |
| COL-08 | Retry interrupted work so crashes do not cause invisible loss or duplicate effects. | Resume/checkpoint or clean rollback; duplicate delivery is idempotent; expose unresolved uncertainty. |
| COL-09 | Render authorized pages so client-only routes and visual evidence are available. | Browser sandbox limits resource use and every subrequest; no ambient traffic-store, credential or host-network access. |
| COL-10 | Import historic pages and bundles so comparisons predate my own collection. | Preserve archive/source times and distinguish archived content from current observation. |

## GR: Asset graph and inventory

| ID | User story | Acceptance note |
| --- | --- | --- |
| GR-01 | Insert and retrieve assets so observations have usable identities. | Begin with hosts, defined normalization/duplicates and ordinary missing-ID behavior. |
| GR-02 | Connect assets so I can ask what relates to a host. | Directed typed relationships, valid endpoints and explicit parallel-edge/self-loop semantics. |
| GR-03 | Walk paths and shared infrastructure so I can follow a lead. | Direction/filter aware neighbors, reachability and shortest unweighted paths; weighted paths explain their cost model. |
| GR-04 | Inspect evidence behind a relationship so I can judge its reliability. | Retain independent support, contradiction, rule/source versions and revisions; confidence differs from severity/freshness. |
| GR-05 | Withdraw one source's assertion so corrections do not erase other support. | Partial/failed scans remove nothing outside proven coverage; late revisions cannot resurrect withdrawn claims. |
| GR-06 | Inspect historic and current graph views so I can distinguish then-known from now-believed. | Keep observation, real-world validity and recorded times distinct; unknown bounds and late corrections remain explicit. |
| GR-07 | Represent assets relevant to a bounty program so clues connect usefully. | Hosts, endpoints, addresses, certificates, software, authorized source/mobile apps and relevant cloud resources retain trustworthy identity and evidence. No corporate CMDB, employee/device fleet, IAM-governance or compliance inventory. |
| GR-08 | Resolve equivalent entities so multiple tools do not create misleading duplicates. | Exact normalized identity first; equivalence/merge decisions are evidence backed, explainable and reversible; private address spaces and source IDs are namespaced. |
| GR-09 | Keep notes and saved views through rebuilds so analysis survives layout/storage changes. | Stable public references; snapshot-local traversal indices are not API identities; replay/full and incremental projections agree. |
| GR-10 | Find islands, groups, bridges, chokepoints and unusual assets so I can choose where to look. | Centrality/community/outlier/blast-radius/similar-neighborhood views explain structural signals as leads, not vulnerability proof. |
| GR-11 | Browse a large graph safely so hubs do not freeze the application or hide incomplete answers. | Bound depth/nodes/edges/time/payload, paginate expansions, report truncation/projection version, apply access rules to intermediates and counts. |
| GR-12 | Save arrangements, mark assets and export an investigation brief so graph work is reusable. | Interesting/tested/parked marks, isolate/compare/timeline/level-of-detail views, JSON/CSV/PNG/Markdown exports and non-canvas CLI queries. |

## PAR: Parsing and surface extraction

| ID | User story | Acceptance note |
| --- | --- | --- |
| PAR-01 | Extract endpoints, routes, parameters, flags, cookies, forms and dependencies so artifacts become searchable facts. | Retain raw/derived artifact reference and exact source locations where available. |
| PAR-02 | Decode scripts and source maps so minified deployments remain investigable. | Preserve original mappings, bounded decoding/decompression and explicit heuristic limitations. |
| PAR-03 | Read HTML, JSON, XML, YAML and configuration so non-script surfaces are covered. | Preserve relevant source semantics, aliases/duplicates and spans; cycles/expansion have budgets. |
| PAR-04 | Inspect GraphQL schemas/operations and gRPC descriptors/traffic so API structure is visible. | Parsing differs from schema/semantic validation; authorized introspection/reflection are separate sends. |
| PAR-05 | Inspect WebAssembly modules so binary imports, exports and references are evidence. | Separate parse success, validation and security analysis; no execution implied. |
| PAR-06 | Analyze authorized or publicly disclosed application source so code can reveal useful bug candidates. | Choose languages/frameworks relevant to the target; retain spans, bindings, dataflow limitations and unknown dynamic values. Hunting-oriented analysis, not a generic developer CI SAST replacement. |
| PAR-07 | See extraction coverage so unsupported input does not masquerade as no findings. | Distinguish read/partial/failed/unsupported and successful zero-fact reads; persist budgets and child-reader gaps with the artifact. |
| PAR-08 | Re-run extraction with a new rule version so improvements are reproducible. | Versioned derivations preserve prior evidence; same inputs/rules yield the same logical result. |
| PAR-09 | Recover publicly served repository history so accidental source exposure can be understood. | Only permitted served objects; bounded retrieval and source provenance, no private repository access. |
| PAR-10 | Maintain shared structural patterns and application-specific readers so detections can grow. | Versioned patterns; introduce shared analysis IR only when needed; unsupported lowering remains visible. |

## CHG: History, changes and triage signals

| ID | User story | Acceptance note |
| --- | --- | --- |
| CHG-01 | Version artifacts and deduplicate unchanged content so redeploys do not look like total change. | Content identity and observation history remain distinct; storage/restart preserves both. |
| CHG-02 | See structural changes so I can distinguish new routes/flags/libraries from formatting noise. | JS/JSON/HTML-aware views state heuristic limits; masked/reordered changes are not silently equated when semantics may differ. |
| CHG-03 | Compare screenshots and timings so behavior changes without source changes are visible. | Associate images and timing samples with versions; noisy measurements are not asserted as vulnerabilities. |
| CHG-04 | Inspect a change timeline so I can connect deployments and findings. | Trace changes to artifacts/facts; distinguish first appearance, last observed and confirmed removal. |
| CHG-05 | See explainable priorities so I can review useful changes first. | Begin with inspectable rules; show auth/admin/payment adjacency, novelty, freshness and stack-risk contributions. |
| CHG-06 | Review the morning inbox, live activity and collection health so I can orient quickly. | Unreviewed changes, run problems and scope health link to evidence; repeated/baseline events can be grouped or muted. |
| CHG-07 | Learn deployment cadence so I can choose better collection times. | Derive heat maps from retained version history, not added probing. |
| CHG-08 | Mute noisy patterns and keep saved alerts so attention matches my workflow. | Explain why an item appears; retain evidence and reversibility; quiet policies queue digest items rather than losing them. |

## SRCH: Search, similarity and retrieval

| ID | User story | Acceptance note |
| --- | --- | --- |
| SRCH-01 | Search current and historical evidence so I can find a string and its first appearance. | Text/regex/structured filters, stable offsets/context, program/time/kind boundaries and saved searches. |
| SRCH-02 | Explain a search so I understand candidate filtering and exact matches. | Trigram constraints never replace exact verification; short/Unicode queries have defined behavior. |
| SRCH-03 | Exclude noisy vendored/binary/oversized material by default so relevant hits remain visible. | Search-ignore and smart-case behavior are overridable; excluded search material remains stored. |
| SRCH-04 | Search bounded decoded content so compressed/encoded evidence is discoverable. | Derived forms link to raw hashes and offsets; no unbounded expansion. |
| SRCH-05 | Find variants, similar responses and duplicates so a known pattern leads to related evidence. | Begin with exact/simple baselines; distinguish near-duplicate matching from semantic similarity and confirm leads manually. |
| SRCH-06 | Combine lexical and local vector retrieval so exact tokens and meaning both matter. | Filter inside retrieval; cite evidence; hybrid ranking/reranking is measured against baselines. |
| SRCH-07 | Rebuild or change an index/model safely so search remains recoverable. | Versioned derived indexes, bounded segment build/merge/delete, model provenance and rollback; protect vectors like sensitive source data. |
| SRCH-08 | Pick entities interactively or through pipes so the same workflow works in UI and scripts. | Fuzzy host/parameter lookup, preview/multi-select, ranked results and machine-readable JSON/JSONL output. |

## TRF: Proxy, traffic and browser integration

| ID | User story | Acceptance note |
| --- | --- | --- |
| TRF-01 | Capture my own HTTP/HTTPS traffic so I retain exact investigation exchanges. | Full sensitive requests/responses are separately protected; interception CA is installed only on my own device and its key remains private. |
| TRF-02 | Configure workbench capture/forwarding separately from automated collection so normal browsing is usable. | Operator-only loopback/tunnel proxy; workbench filter can narrow default forwarding/capture, global denylist still wins, and sends record workbench context. |
| TRF-03 | Promote traffic into program knowledge deliberately so general browsing does not become program evidence accidentally. | Live attested program scope gates promotion and strips credentials. If no scope is attested anywhere, already-received traffic may be read under an explicit `unscoped` context; the first attestation ends this mode. It grants no new sends. |
| TRF-04 | Intercept, edit, forward or drop traffic so I can inspect a flow before it continues. | Clear paused state and explicit operator decisions; cancellation/disconnection cannot silently forward a held exchange. |
| TRF-05 | Search captured history so I can find requests by method/status/content/tags. | Token-level query errors, saved filters and request-response links; secrets masked in shared views. |
| TRF-06 | Inspect WebSocket sessions so I can understand ongoing protocols. | Capture directional frames with connection identity, timing and bounded retention. |
| TRF-07 | Configure client certificates, listeners, ALPN, passthrough and hostname overrides so authorized compatibility tests work. | Keys encrypted and not exported; passthrough is not decrypted/stored; legacy compatibility is explicit and narrowly scoped. |
| TRF-08 | Route my browser through an extension so ordinary research is captured conveniently. | Visible capture state, per-install authentication, scoped capture and no page-controlled privileged messaging. |
| TRF-09 | Import Burp, ZAP, Caido and HAR captures so prior investigation is reusable. | Bounded defensive parsing, source provenance and sensitive-data handling; imported hosts/exchanges grant no sending authority. |

## WB: Manual testing workbench

| ID | User story | Acceptance note |
| --- | --- | --- |
| WB-01 | Edit and resend a captured request so I can verify a lead. | Deliberate scoped, rate-limited, audited send; saved state includes identity and target context. |
| WB-02 | Compare two requests/responses so I can see meaningful differences. | Line/word/structured comparison with retained baselines and normalization choices. |
| WB-03 | Organize, annotate and tag requests so useful examples are easy to revisit. | Organizer entries link to original exchanges, notes and reviewed candidates. |
| WB-04 | Export a request or draft a PoC so reproduction is convenient. | Copy as curl/Python/fetch or draft an inert CSRF form; local generation sends nothing and redacts by default. |
| WB-05 | Minimize a reproduction so reports include only necessary inputs. | Draft removals locally; any verification request is an explicit gated workbench action, and unverified removals are labeled. |
| WB-06 | Run a bounded payload set so I can compare responses systematically. | Sniper/battering-ram/pitchfork/cluster-bomb families; preflight count/time/rules/budget, manual start, stop control and explainable response differences. |
| WB-07 | Transform payload representations so I can assess inconsistent input handling. | Encoding/case/comment/Unicode variants change request shape only; no identity rotation, block-bypass retry loop or relaxed budgets. |
| WB-08 | Inspect raw HTTP framing so protocol discrepancy leads can be reviewed. | Preserve HTTP/1.1 header order/case/duplicates; expose HTTP/2 constraints separately; draft probes never send themselves. |
| WB-09 | Send a capped race group so I can investigate concurrency behavior. | One manual bounded burst, per-copy audit/admission/timing, no unattended repetition and no DoS behavior. |
| WB-10 | Record and alter multi-step business flows so missing state transitions can be investigated. | Skip/reorder/repeat suggestions retain baseline evidence; each execution requires the applicable operator action. |
| WB-11 | Test file uploads and schema boundaries so feature-specific handling can be assessed. | Inert labelled test artifacts, permitted classes and cleanup records; schema-aware cases do not bypass send controls. |

## IAM: Test identities, sessions and authentication analysis

| ID | User story | Acceptance note |
| --- | --- | --- |
| IAM-01 | Manage my own authorized accounts by role/tenant so comparisons have valid identities. | Program-specific multi-account permission; protected credentials and session state, never another user's stolen session. |
| IAM-02 | Maintain cookie jars, variables, login macros and CSRF refresh so sessions remain usable. | Mask secrets, bounded retry, no reachability expansion, cancellation/kill-switch support and per-send audit. |
| IAM-03 | Replay under roles/tenants/anonymous context so I can assess access controls. | Compare expected authorization and response evidence; difference alone is a candidate, not a confirmed IDOR/BOLA. |
| IAM-04 | Inspect OAuth/OIDC handshakes so legal and observed transitions can be compared. | Code/implicit/client-credentials/PKCE, redirect/state/token claims and replay context mapped to relevant standards with exchange evidence. |
| IAM-05 | Assess SAML/SSO and JWT handling so authentication-related leads are not ignored. | Signature/audience/recipient/expiry/algorithm checks retain evidence; active validation stays manual and scoped. |
| IAM-06 | Analyze captured tokens so predictable structure is visible. | Per-position entropy, fixed prefixes and consecutive-value patterns; analysis sends nothing and does not equate entropy with security. |
| IAM-07 | Use a test mailbox and TOTP so account setup remains organized. | Operator-controlled identities only; seeds encrypted; mailbox receives only authorized test mail. |

## DET: Detection and verification coverage

| ID | User story | Acceptance note |
| --- | --- | --- |
| DET-01 | See evidence-backed candidates across vulnerability classes so leads are not lost. | Injection/SSRF/XXE, XSS/postMessage/prototype pollution, access control, cache/framing, auth/crypto, logic and upload families remain distinct. |
| DET-02 | Inspect source-to-sink paths so dataflow candidates are understandable. | Track sanitizers/control/dataflow and interprocedural limits; trace gaps and heuristic paths are explicit. |
| DET-03 | Review passive hardening/exposure checks so obvious risks surface without added tests. | CORS/CSP/security headers, cookie flags, debug/admin surfaces, source maps and exposed configuration use retained evidence. |
| DET-04 | Detect dangling DNS/provider references so takeover leads are visible. | Report the record/provider fingerprint; never register or claim a resource automatically. |
| DET-05 | Discover permitted content, parameters, versions and ports so known surface gaps can be investigated. | Context-derived vocabulary, declared tool side effects and hard run budgets; schema inference never authorizes a request. |
| DET-06 | Maintain a coverage matrix so unsupported classes and false positives are visible. | Include CSRF, clickjacking, SSTI, deserialization, host-header, cache deception, CRLF, traversal, mass assignment and GraphQL batching; record detector versions/status/measurements. |
| DET-07 | Arm a reviewed benign probe set so limited checks can run with explicit consent. | Off by default, fixed named versioned entries, eligible attested scope, no credentials/body/write effect, per-host/run budgets and immediate unarming. |
| DET-08 | Correlate a manual verification with its expected signal so I can confirm or reject a candidate. | Exact request/response, callback or timing evidence; assisted interpretation never substitutes for operator validation. |
| DET-09 | Observe a candidate in a sandboxed browser so static client-side traces can be checked. | Manual source-to-sink/postMessage/prototype-gadget observation, admitted subrequests and no automatic exploitation. |
| DET-10 | Record sensitive-data stops so investigation does not continue unnecessarily. | Capture minimum evidence, halt affected activity, flag cleanup/reporting needs and avoid storing excess third-party data. |

## SEC: Secrets, software and cloud exposure

| ID | User story | Acceptance note |
| --- | --- | --- |
| SEC-01 | Detect secret-shaped values so accidental disclosure becomes a reviewable lead. | Prefix/regex/entropy and bounded decoding, offline format/checksum validation and example/context discrimination. |
| SEC-02 | Track sensitive values safely so I can see presence/removal/rotation without spreading them. | Encrypted values, redacted displays, keyed deduplication and local sensitivity classification; no automatic provider login/validation. |
| SEC-03 | Identify libraries and versions so advisories can be correlated with deployments. | Banners/code/package evidence, direct versus inferred dependency and deployment context, advisory provenance; version match is not exploitability. |
| SEC-04 | Watch advisories and prioritize n-day leads so known affected versions are reviewed. | Local approved feeds such as OSV/CVE/retirement/EPSS-style metadata; freshness and confidence shown, never automatic exploitation. |
| SEC-05 | Inspect cloud management, databases/caches and registries so exposure is recorded. | Reachability/public metadata only by default; no automatic query of private data, login or mutation; bucket write checks require a separate operator action. |
| SEC-06 | Connect leaked IaC/configuration and SSRF dataflow so cloud-related leads have context. | Evidence-backed chains rather than a metadata-looking URL treated as proof; discovered cloud credentials remain protected and untested. |
| SEC-07 | Inventory third-party scripts, SRI, CI/CD, build artifacts and image layers so supply-chain exposure is visible. | Public attributable sources, immutable revisions/digests, change history and no supply-chain compromise behavior. |

## MOB: Mobile and API reconstruction

| ID | User story | Acceptance note |
| --- | --- | --- |
| MOB-01 | Inspect official APK/IPA releases so mobile surface joins web evidence. | Bounded static unpack/decompile, endpoint/key/config/component/permission evidence; no device compromise or execution implied. |
| MOB-02 | Watch app stores and publisher releases so mobile changes trigger review. | Program-owned official distribution and recorded version provenance. |
| MOB-03 | Reconstruct an API from observed endpoints, types and operations so undocumented surface is understandable. | REST/OpenAPI/Postman-style export plus GraphQL/gRPC views; observed, inferred and declared structure differ. |
| MOB-04 | Find structural API gaps so I can choose the next manual check. | Missing methods/version prefixes are candidates; no automatic request follows from a guess. |

## FLOW: Pipelines, tools and plugins

| ID | User story | Acceptance note |
| --- | --- | --- |
| FLOW-01 | Register native and personal CLI tools so reusable adapters replace ad-hoc files. | Declared schemas, side effects, version, sandbox and controlled egress; validate output and preserve provenance. |
| FLOW-02 | Compose typed stages so collection, extraction, storage and notification work together. | Visible inputs/dependencies/conditions, cancellation, progress and bounded outputs; tools cannot create their own unrestricted network path. |
| FLOW-03 | Write automation with variables, conditions and loops so workflows express real logic. | Parse/check with source diagnostics; every external operation still enforces capabilities, quotas, scope and audit. No unattended active-send constructor. |
| FLOW-04 | Trigger declared workflows from schedules, changes and verified webhooks so repeatable preparation is automatic. | Trigger is data, not authority; changes can prepare workbench tasks but cannot start forbidden tests. |
| FLOW-05 | Retry/recover stages and inspect dead letters so failed jobs can be handled deliberately. | Durable progress, bounded retries, idempotent effects, checkpoint/lease fencing and explicit redelivery. |
| FLOW-06 | Add a personal sandboxed extension when fixed adapters cannot handle a useful workflow. | Conditional: explicit capabilities, data boundaries, bounded resources and controlled egress. No public plugin marketplace or enterprise extension platform. |
| FLOW-07 | Get useful diagnostics while writing a workflow so errors are caught before execution. | Reuse the grammar/checker for token locations, validation and a readable plan. Formatting/completion are optional; a full language-server/IDE product is excluded. |
| FLOW-08 | Preview a plan and export results so workflows remain inspectable and interoperable. | Explain tools/targets/counts/policies; signed redacted export sinks, no imported output interpreted as commands. |
| FLOW-09 | Import and run approved nuclei-style templates so informational checks can share the pipeline. | Importing parses rather than executes; classify capability/side effects, sandbox and gate every send. Unattended informational templates cannot become an exploit path. |

## IO: Webhooks and out-of-band listeners

| ID | User story | Acceptance note |
| --- | --- | --- |
| IO-01 | Receive declared external events so approved workflows can respond. | Per-source rotatable secret, timestamp/body signature, five-minute freshness, event dedupe, size/rate bounds and optional IP allowlist. |
| IO-02 | Inspect delivery history and redeliver so integrations can be debugged. | Outbound signing/redaction, bounded retries and disable-on-failure; redelivery keeps event identity. |
| IO-03 | Receive HTTP/DNS callbacks so blind-test evidence is correlated. | Opaque correlation with the originating operator test, bounded retention and abuse-resistant receive-only listeners. |
| IO-04 | Receive disposable webhook/email traffic so outbound application behavior is visible. | Inbox and SMTP receiver never relay/send mail or become arbitrary forwarding services. |
| IO-05 | Use my own interaction-server adapter so callback infrastructure is replaceable. | Same correlation, access, retention and provenance; enabling a receiver does not send canaries. |

## INT: Local intelligence and assistance

| ID | User story | Acceptance note |
| --- | --- | --- |
| INT-01 | Rank candidates with reasons so I understand suggested priorities. | Rule baseline first; separate class/confidence/severity/freshness, show features and allow override. |
| INT-02 | Classify traces and abstain when uncertain so labels are useful. | CWE-rooted multi-label taxonomy, program tags, calibration and uncertainty sets; no unsupported severity certainty. |
| INT-03 | Get cited suggestions for what to inspect so I can use prior evidence. | Graph signals, related findings, feature playbooks and local retrieval; suggestions cannot send, admit a host or confirm a finding. |
| INT-04 | Draft requests, wordlists, workflows and reports so preparation takes less time. | Human review/run remains explicit; target text is bounded evidence separated from controlling instructions. |
| INT-05 | Learn from my outcomes so program-specific noise falls over time. | Only permitted Learn data supplies labels/priors; reliability-weighted evidence fusion, novelty and ranking remain explainable. |
| INT-06 | Detect response anomalies and soft-404 templates so routine noise does not look like new surface. | Compare rule/statistical baselines, change points and clustered shapes; record false positives and uncertainty. |
| INT-07 | Evaluate and roll back models so complexity earns its place. | Program/time-aware held-out sets, class imbalance, calibration, shadow mode, drift checks and baseline fallback; track precision/recall, dismissal and time to useful action. |
| INT-08 | Experiment with local embeddings and small neural models so similarity improves only when measured. | Exact scan baseline; ANN/quantization/filtering and reranking measured for recall/latency/memory; optional learned secret/sequence/graph features must beat simpler approaches. |
| INT-09 | Track research hypotheses so a lead has an explicit claim and validation plan. | Link supporting/contradicting evidence, assumptions, unknowns, proposed manual checks and outcomes; distinguish hypothesis from confirmed finding. |
| INT-10 | Revise conclusions when evidence changes so stale beliefs do not silently persist. | Trace evidence dependencies, rule/source/model versions and withdrawal/correction effects; show why a conclusion changed or became uncertain and preserve its history. |
| INT-11 | Prioritize informative next investigations so I can improve coverage with limited time. | Explain expected information value, uncertainty, relevance, effort and policy constraints using simple baselines first; recommendations cannot grant authority or send tests. |

## CORP: Public knowledge and data-use policy

| ID | User story | Acceptance note |
| --- | --- | --- |
| CORP-01 | Choose Learn, Use only or Isolated so engagement data is used appropriately. | Missing policy is Isolated. Learn permits consented local learning and permitted retrieval; Use only allows existing scoring without training/shared retrieval; Isolated contributes no scoring/training/shared intelligence. |
| CORP-02 | Tighten policy or delete a project so derived data follows the decision. | Remove shared index/cache/label contributions, record model-retraining implications; deletion includes derived embeddings and indexes. Loosening is explicit and audited. |
| CORP-03 | Import public reports/advisories/writeups so knowledge can inform local research. | Separate download/store/train permissions recorded first; prefer metadata/facts then short cited excerpts over full text. |
| CORP-04 | Curate and classify corpus entries so evidence quality is visible. | Redaction, canonical links/deduplication, per-class metadata, evidence levels, rejects and removable sources; never execute attachments or PoCs. |
| CORP-05 | Retrieve and learn from public knowledge without flattering evaluation so suggestions remain credible. | Weight below own outcomes, isolate from benchmarks and held-out tests, keep all content/derived vectors local. |

## OPS: Operator workflow and coverage

| ID | User story | Acceptance note |
| --- | --- | --- |
| OPS-01 | Rank programs and track time so I choose research deliberately. | Explain payout/scope/response/crowding/skill-fit/history factors; session timers, stop rules and weekly income/hit-rate review remain advisory. |
| OPS-02 | Build target vocabulary and version recon profiles so tests fit the application. | Derive words from observed routes/identifiers/history; keep source/config versions and program limits. |
| OPS-03 | Keep a reusable tool-belt so snippets/templates/payload sets/macros are organized. | Parameterized, searchable and versioned; cross-program reuse follows per-program data-use permissions. |
| OPS-04 | Model features, roles, objects and trust boundaries so testing reflects the application. | Role-resource and feature-class coverage with untested/tried/confirmed/ruled-out evidence; use WSTG/ASVS-style references as study/playbook inputs. |
| OPS-05 | Park leads and link chains so unfinished research can become useful later. | Passive evidence changes resurface leads; chaining claims keep evidence and combined-impact reasoning. |
| OPS-06 | Keep notes and a lab notebook so reports follow my actual investigation. | Timestamped requests/screenshots, block editing/templates/tables/callouts, evidence cards, links/backlinks and cited related panel; suggestions are never auto-inserted. |
| OPS-07 | Track my next research actions so parked leads, retests and broken jobs do not get lost. | A small personal task view links existing candidates, evidence and runs. No standalone generic issue tracker or corporate remediation workflow. |
| OPS-08 | Excluded: shared workspaces and team research administration. | Removed by the owner's 27 October 2026 scope clarification. No collaboration locking, team RBAC or assignment infrastructure. |

## FIND: Findings, reports and follow-up

| ID | User story | Acceptance note |
| --- | --- | --- |
| FIND-01 | Review and transition candidates so confirmed/dismissed/reopened outcomes are explicit. | Notes/assignee/evidence and reasoned triage; material evidence changes can surface a previously dismissed lead again. |
| FIND-02 | Check novelty so I avoid duplicate reports. | Compare own history and permitted disclosed reports; similarity is a suggestion, not proof of duplication. |
| FIND-03 | Build and lint a report so repro steps, impact and severity are supported. | Per-platform templates, CVSS assistance, framework-specific mitigation, missing-step/redaction/third-party-data checks; operator approves final content. |
| FIND-04 | Export a reproducible evidence bundle so a triager can understand the claim. | Exact permitted exchanges, scope/audit references, diffs, replay instructions and redacted recordings; export itself sends no tests. |
| FIND-05 | Prepare a reviewed HackerOne submission so the report matches the platform workflow. | Keep drafts, attachments, platform links and returned status; final submission is intentional and human-reviewed under current HackerOne rules. Jira/Linear/GitHub issue synchronization is excluded. |
| FIND-06 | Track report outcomes and payouts so the board reflects what happened. | Reported/duplicate/accepted/resolved/paid and bonus/program events from manual or approved platform imports; ledger entries never authorize tests. |
| FIND-07 | Follow up and watch fixes/regressions so closed work is not forgotten. | Timers and passive change evidence can prompt; saved PoC retests require operator initiation under current rules. |
| FIND-08 | Record retros and a pattern library so outcomes improve future research. | Why accepted/duplicate/not applicable and supporting evidence; permitted data only, no contamination from isolated labs. |

## NOTIFY: Alerts and exports

| ID | User story | Acceptance note |
| --- | --- | --- |
| NOTIFY-01 | Receive useful changes through my chosen channel so I can investigate promptly. | Console and one selected alert destination first; dedupe, redacted evidence links and bounded retries. Every chat/email/push vendor is not a requirement. |
| NOTIFY-02 | Use a notification inbox and delivery log so I can inspect failures without sending anything. | Read-only activity links to its source; fixed destination identity avoids leaking credential-bearing URLs. |
| NOTIFY-03 | Set quiet hours and alert budgets so notifications remain sustainable. | Queue into digest, optional critical-health/scope exceptions, bounded per-channel delivery and disable repeated failures. |
| NOTIFY-04 | Optionally initiate a named research action through an authenticated personal chat command. | Conditional convenience: linked operator identity and unchanged program/rate/audit controls, no arbitrary message execution. |
| NOTIFY-05 | Optionally export report drafts or notes to a destination I actually use. | Conditional convenience, redacted and explicitly configured; no required Drive/multi-vendor suite or export of raw credentials/traffic/CA keys. |

## UI: Console, CLI and API experience

| ID | User story | Acceptance note |
| --- | --- | --- |
| UI-01 | Use API/CLI/UI operations with consistent meaning so scripts and console agree. | Validation, stable identities/errors, pagination, idempotency, cancellation and authentication; add contracts per implemented operation. |
| UI-02 | Navigate actual hunting workflows so context is easy to follow. | Provide useful discovery, changes/search, evidence/findings, traffic/tests and program/run views as operations exist. A small personal cockpit, not a mandatory page for every catalog noun. |
| UI-03 | Filter, facet, sort, save and export views so repeated investigation is quick. | Shareable local state, keyboard access, entity menus/dossiers, previews, bulk actions and clear empty/loading/error/partial states. |
| UI-04 | Use a command bar and fuzzy picker so every implemented area is reachable. | Navigation, contexts, shortcuts and safe previews; decoders/converters/hashes/entropy/file helpers, snippets/quicklinks/calculations and plugin commands. |
| UI-05 | Read highlighted code and request bodies so evidence is understandable. | Syntax highlighting, raw/pretty/structural views and source mappings; rendering untrusted text cannot execute markup. |
| UI-06 | Understand proposed sends and personal system health so actions are predictable. | Show target, identity, policy, counts, failures and useful preferences; reversible record edits can support undo, but a request cannot be unsent or expired approval silently restored. |
| UI-07 | Protect access to my private instance so research and credentials remain under my control. | Appropriate personal authentication/revocation before exposing the service. Stronger login methods are optional; enterprise SSO, team RBAC and tenant administration are excluded. |

## REL: Storage, reliability and deployment

| ID | User story | Acceptance note |
| --- | --- | --- |
| REL-01 | Persist the system of record so restarts preserve research. | Programs/scopes/observations/facts/graph/history/traffic/findings/notes/workflows remain recoverable; transactions and schema migration prevent torn logical state. Storage engine remains an implementation choice. |
| REL-02 | Keep large artifacts and derived indexes manageable so growth does not overwhelm storage. | Content addressing, retention, bounded compaction/tiering/deletion and rebuildable search/vector/graph projections. |
| REL-03 | Retain a trustworthy personal activity trail so I can explain what happened. | Run/tool/request context, policy decisions and outcomes without secrets; verify record/export integrity as needed. Independent witness hosts, signature quorums and enterprise audit certification are excluded. |
| REL-04 | Back up, restore and fully export so the instance is recoverable and portable. | Encrypted separate backups, isolated restore drills, open-format exports and explicit handling of sensitive full backups. |
| REL-05 | Run the same personal tool locally or on GCP so research can continue affordably. | Modular monolith first, protected configuration, authentication, health and restart behavior. Worker profiles are conditional; no all-cloud/all-host deployment matrix. |
| REL-06 | Process durable jobs with backpressure so load and crashes remain controlled. | Priority, bounded queues/concurrency, at-least-once delivery, idempotency/outbox, leases/checkpoints/dead letters and no stale publication. |
| REL-07 | Scale one responsibility independently so expensive work does not stall the application. | Later service extraction, controlled cloud burst/scale-to-zero, coherent job/object/sandbox/fetch interfaces and preserved global/per-host ceilings. |
| REL-08 | Observe health and cost so operation stays explainable and affordable. | Logs/metrics/traces with bounded labels and no sensitive values; cost estimates/views, configured work ceilings and pause behavior. Cloud billing alerts alone are not hard spending caps. |
| REL-09 | Deploy the chosen GCP workload reproducibly so cloud learning supports useful automation. | Only necessary compute/storage/queue/secret services. No mandatory GKE, service mesh, multi-cloud control plane or large infrastructure framework. |

Database theory and systems techniques deepen existing stories through
[Database internals](STUDY_GUIDE.md#database-internals) and [Systems and performance](STUDY_GUIDE.md#systems-and-performance).
They do not mandate unsafe/assembly/custom storage where simpler code fits better.

## QA: Quality, labs and measured improvement

| ID | User story | Acceptance note |
| --- | --- | --- |
| QA-01 | Exercise a staged target with known deploy changes so extraction and diffs have ground truth. | Local labelled changes, valid negative examples and reproducible runs; no real-target traffic. |
| QA-02 | Measure parsers and detectors so coverage claims are honest. | Required/forbidden labels, malformed/truncated/unsupported inputs, differential/conformance/property/fuzz checks; incomplete labels do not justify false precision. |
| QA-03 | Compare relevant hunting tasks with baselines or existing tools so improvements are credible. | Authorized local scenarios, held-out ground truth and metrics for original useful leads, precision/recall, evidence quality and time saved. No whole-suite Acunetix/Aikido/Censys parity gate. |
| QA-04 | Keep experiments isolated so labs do not bias operational intelligence. | Separate lab fixtures and model evaluation infrastructure; Isolated projects do not contribute shared labels/priors/retrieval; benchmark data remains local. |
| QA-05 | Profile working features so optimization targets real gaps. | Representative baseline, CPU/allocations/memory/I/O/latency as relevant, one change at a time and correctness comparison; avoid benchmarks that cannot change a decision. |
| QA-06 | Test recovery and trust boundaries so rare failures are understandable. | Crash/retry/duplicate/reorder/cancellation, stale leases, scope drift, secret redaction, parser budgets and program/lab data leakage cases. |
| QA-07 | Compare advanced structures with simple baselines so learning remains grounded. | Optional CSR, tries, Bloom/sketch/Merkle/WAL, trigram, HNSW/IVF/PQ/LSH and small neural experiments; choose by present question, not catalog completion. |

## Boundaries and scope completeness

For a selected story, define the smallest current slice and its limits. A working slice
can be released while the broader story remains partial. Count a story as complete only
when its intended user behavior, integrations, supported interface and relevant error,
authorization, persistence/recovery and resource-bound cases are demonstrated. Use
the checks appropriate to that capability; not every story needs every kind of test.

Keep planned, partial, verified and known-limited work distinguishable when recording
progress. Product completion concerns the selected core bounty workflows and connected behavior,
plus explicit resolution or owner acceptance of limitations. Excluded stories and
unactivated conditional options are not completion requirements.
Performance and quality claims need reproducible comparisons under stated workloads.
Completion is relative to the agreed scope, not a claim that all possible bugs or
unknown future requirements have been eliminated.

Sentinel is for the operator's authorized research. It does not include unauthorized
targeting, denial of service, malware/C2, supply-chain compromise, claiming takeover
resources, using found credentials automatically, or hiding activity/rotating identity
to get around a defender's block. A model, pipeline, plugin, webhook or shared worker
does not relax those boundaries.

This is an independent local scope snapshot. No other Sentinel checkout, generated
checklist, legacy API contract or previous implementation is required to use it.
Future changes are made here explicitly; they do not follow another repository silently.

## Bounty-scope responsibility map

These are primary follow-through owners for the revised bounty scope, not implementation
gates. Read the exclusions and conditional stories in FEATURES.md before treating a row
as a commitment. OPS-08 is excluded; conditional formats/integrations are not core gates.
A foundational slice can appear earlier and a later project can deepen it. A project v1 does not mark
every assigned story complete. API/CLI, essential scope/security, tests, persistence,
observability and small relevant performance checks are part of each applicable project.

| Catalog stories | Primary project | Earlier work or integration |
| --- | --- | --- |
| GOV-01..GOV-10 | P05 | Basic scope/deny/attestation/cancel before P01 live execution. |
| SRC-01..SRC-10 | P09 | P01 starts one adapter; P05 adds provider-policy onboarding. |
| COL-01..COL-08 | P01 | P14 deepen durable operation and retries. |
| COL-09 | P08 | Sandbox before browser execution; P02 later consumes screenshots. |
| COL-10 | P02 | P09 adds broader historic sources. |
| GR-01..GR-03 | P01 | P10 deepens bounded temporal reasoning and query representations. |
| GR-04..GR-12 | P10 | Basic provenance begins P01; personal interface improvements continue in P04. |
| PAR-01..PAR-03 | P03 | P02 supplies artifacts and consumes structural facts. |
| PAR-04 | P03 | API/schema analysis follows the selected web/API target; advanced protocols are conditional. |
| PAR-05 | P16 | Conditional Wasm reading with common coverage/evidence handling. |
| PAR-06..PAR-10 | P03 | Broaden one source/format at a time; source exposure joins P09. |
| CHG-01..CHG-08 | P02 | P03 structural diffs, P12 delivery, P15 learned ranking deepen it. |
| SRCH-01..SRCH-04 | P02 | P03 adds structured fact filters. |
| SRCH-05..SRCH-07 | P15 | Simple duplicate/variant matching begins earlier. |
| SRCH-08 | P04 | Basic pipe-friendly CLI starts P01. |
| TRF-01..TRF-09 | P06 | P02 history/search and P04 evidence are reused. |
| WB-01..WB-05 | P06 | More testing modes arrive in P08. |
| WB-06..WB-11 | P08 | P07 supplies identities; controls apply before every live mode. |
| IAM-01..IAM-07 | P07 | Protected identity handling precedes any sensitive traffic use. |
| DET-01..DET-10 | P08 | P03 static candidates; P09 infrastructure/advisory evidence. |
| SEC-01..SEC-07 | P09 | P03 starts offline secret/library candidates and protected values. |
| MOB-01..MOB-02 | P16 | Conditional mobile research for an actual target. |
| MOB-03..MOB-04 | P03 | Observed API reconstruction and useful gap hypotheses support core web/API hunting. |
| FLOW-01..FLOW-02 | P01 | Full extension/language semantics grow in P11. |
| FLOW-03..FLOW-04 | P11 | Tiny compiler exercise P01; actual triggers integrate P12. |
| FLOW-05 | P14 | Basic bounded retries/checkpoint semantics begin P01. |
| FLOW-06..FLOW-09 | P11 | Minimal diagnostics/templates first; plugins and editor extras are conditional. |
| IO-01..IO-02 | P12 | Endpoint authentication/resource controls precede exposure. |
| IO-03..IO-05 | P08 | Reuse P12 delivery/integration infrastructure as it arrives. |
| INT-01..INT-11 | P15 | Rules/hypotheses start P03/P04; evidence reasoning deepens in P10. |
| CORP-01..CORP-05 | P15 | Appropriate isolation/retention boundaries apply from first stored data. |
| OPS-01..OPS-05 | P04 | P09 vocabulary/profiles; small app/lead views can ship P04/P07. |
| OPS-06..OPS-07 | P04 | Later UI and related-evidence improvements integrate P15/P04. |
| OPS-08 | Excluded | Team/workspace collaboration is outside the personal bounty scope. |
| FIND-01..FIND-08 | P04 | Retrieval, external events and retest prompts deepen in P12/P15. |
| NOTIFY-01..NOTIFY-05 | P12 | Start with one channel, expand incrementally. |
| UI-01..UI-07 | P04 | Each project supplies its real CLI/API/views and needed access controls. |
| REL-01..REL-04 | P14 | Personal persistence/recovery and activity records, integrated when needed; no external audit witnesses. |
| REL-05..REL-07 | P14 | Small monolith cloud deployment can start P01; Backup/recovery is integrated when data becomes valuable. |
| REL-08 | P14 | Lightweight health/cost/resource visibility begins with each running feature. |
| REL-09 | P14 | Select only the cloud services a working workload needs. |
| QA-01..QA-07 | P08 | Focused fixtures/checks and relevant measurements are continuous. |
