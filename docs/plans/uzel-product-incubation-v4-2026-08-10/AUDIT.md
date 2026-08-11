# Independent re-audit — Uzel product-incubation plan revision 4

**Audit date:** 10 August 2026
**Audit target:** all revision-4 planning, execution, governance, maturity, prompt and template artifacts in this directory
**Verdict:** **pass as an execution-control plan, subject to the implementation evidence gates listed below**

## Executive verdict

Revision 3 was not sufficient for the newly stated objective. It could direct a strong
product incubation programme, but it did not yet create a complete operating system for:

- earning a production-grade runtime claim capability by capability;
- following rapidly changing specifications, libraries, providers and orchestration tools
  without floating execution;
- contributing evidence and fixes upstream without creating issue/PR noise or confusing
  merge, release and adoption;
- preserving decisions, failed assumptions and implementation nuance as a trustworthy
  future educational corpus for humans and agents.

Revision 4 corrects those omissions. Its governing shape is now:

```text
immutable execution profile
        +
read-only ecosystem radar and candidate-next probes
        +
evidence-led compatibility campaigns
        +
recorded upstream interaction lifecycle
        +
capability maturity and production evidence
        +
curated decision/learning pipeline
        ↓
M5 exact L4 candidate
        ↓
mandatory A5 twelve-lane audit
        ↓
remediation and human residual-risk decision
        ↓
possible L5 production approval
```

The plan does **not** claim that an iframe/WebView alone safely executes arbitrary hostile
third-party code. Its production claim is deliberately bound to exact source/package and
compatibility-profile identities, a declared Linux/platform matrix, named capability
subsets and explicit guest trust tiers. Before L5, A5 must either accept a vetted-exact-
build support boundary or require and verify stronger process/OS/network isolation.

This is the correct senior-engineering posture. A broader claim without stronger evidence
would be marketing, not security engineering.

## Audit method

The plan was reviewed through the following lenses:

1. programme fit and phase coherence;
2. semantic ownership and dependency direction;
3. exact-build, compatibility and launch identity;
4. signing, irreversible side effects and recovery state machines;
5. guest, browser, native, local-control, filesystem, object and network boundaries;
6. multi-profile and multi-instance isolation;
7. production maturity, operations, migration and release evidence;
8. fast-moving specification/library/tool governance;
9. upstream issue/comment/PR/private-disclosure workflow;
10. decision, nuance, learning and education capture;
11. GSD/Codex/CodeRabbit command and worktree assumptions;
12. internal contradictions, stale terminology, broken links, phase/lane sequences and
    machine-readable template syntax.

The audit also compared current public ecosystem materials with the plan's assumptions.
Those materials are explicitly treated as dated planning evidence; M0 must resolve the
actual Uzel pins to immutable source bytes and rerun the source scan.

## Material findings and corrections

### H-01 — “Exact pin” was underspecified

A tag, branch, pull-request number, documentation URL or package name is mutable or
insufficient. Revision 4 requires immutable commit/tree/path/content identities for Git
sources and exact package/archive integrity for registry sources. Human locators remain
context only.

**Correction:** immutable identities now appear in the upstream registry, compatibility
profile, compatibility campaigns, phase closeouts and candidate freeze.

### H-02 — a human-readable compatibility statement could drift from shipped behavior

A prose table or reserialized TOML document is not an exact runtime contract. TOML has no
universal canonical serialization, and a profile digest cannot safely depend on an
unstated normalization implementation.

**Correction:** the Uzel Runtime Compatibility Profile is the exact validated UTF-8 TOML
byte sequence in the package. BOM is forbidden, line endings are LF, and the hash scheme
is `sha256-exact-utf8-bytes-v1`. The profile does not contain its own hash. A package
manifest plus compiled expected digest binds the bytes; startup recomputes the digest and
fails closed. The human rendering is generated from those exact bytes.

Launch-transcript hashing is a separate `sha256-canonical-cbor-v1` contract with test
vectors; it cannot be conflated with profile-byte identity.

### H-03 — compatibility was discoverable only after guest execution

Object-presence probing or an implicit fallback lets incompatible code execute before the
runtime knows that required semantics are unavailable.

**Correction:** exact source/build verification and required/optional capability
negotiation occur before guest code. Missing or invalid required capability rejects the
launch. Optional absence is explicit. The transcript is bound to instance, local profile,
actor, exact build, session and generation. There is no caller-selected profile, implicit
required-capability downgrade or mid-session profile replacement.

### H-04 — fast-moving dependencies were treated only as pinning risk

Freezing a dependency forever is not stewardship. Following a branch head is not
reproducibility. The previous plan lacked a standing mechanism between those extremes.

**Correction:** revision 4 adds:

- one canonical machine-readable upstream registry;
- a dated read-only ecosystem radar;
- an isolated no-secret candidate-next shadow probe that cannot modify production locks,
  profiles or releases;
- risk classification for API, wire/schema, authority, security, migration and interop
  changes;
- bounded compatibility campaigns for adoption;
- previous-green pins, rollback evidence and explicit defer/reject/adopt dispositions.

A green shadow probe is advisory, not adoption evidence.

### H-05 — the orchestration toolchain itself was outside compatibility governance

The plan depended on GSD command spellings while active documentation, development
branches and installed releases may expose different optional flags—including different
`--validate` surfaces.

**Correction:** GSD, Codex, CodeRabbit, Rust, Node and Nix are phase-pinned development
inputs. The runbook records the exact installed help/version output, uses a validation
option only when that pinned help defines the intended semantics, keeps plan-checker
verification enabled, feeds independent CodeRabbit evidence through `plan-phase
--reviews`, and always follows execution with `verify-work`. Unknown command drift stops
the phase rather than provoking an invented or silently omitted gate.

### H-06 — upstream observation, acceptance, merge, release and local adoption were conflated

An upstream PR merge does not mean a release exists, that Uzel has adopted it, or that a
local patch may be removed.

**Correction:** the Upstream Interaction Record uses separate states for observed,
reproduced, commented, issue open, local patch, PR open, accepted, merged, released,
adopted and patch removed. Every local patch has an exact base, owner, test, expiry/removal
trigger and review date.

### H-07 — the plan lacked a safe contribution channel-selection process

Without a gate, agents could create duplicate or low-value issues, comments and PRs, leak
private evidence, ignore repository contribution rules or universalize Uzel product
policy.

**Correction:** before any external contact, the process now requires exact-pin and
current-upstream reproduction, search of existing threads, reading current contributing,
security, license, DCO/CLA, style and test guidance, a minimal synthetic reproducer,
visibility review, dedicated upstream fork/worktree/branch and human review of non-trivial
public text and code. Channel selection distinguishes local misuse, existing-thread
evidence, library defects, missing seams, draft/spec ambiguity, documentation fixes,
product-only preferences and private security disclosure.

One canonical upstream thread is used per problem. No automatic public submission is
permitted.

### H-08 — conformance could remain an internal, self-confirming claim

First-party runtime and first-party fixtures can share the same mistaken assumptions.
Passing an internal test suite alone is not ecosystem composability.

**Correction:** M1 produces an externally consumable compatibility/conformance kit with
the exact profile, schemas, vectors, limits, black-box packaged-runtime harness and an
external-source clean-room fixture. A separately authored/commissioned clean-room peer
using only the public kit and packaged black-box harness is required for L4 runtime
composability. If absent, `blocked_no_independent_peer` blocks M5 rather than allowing the
plan to delete its core composability thesis. Community-maintained-peer evidence is a
stronger, separately labeled ecosystem-adoption claim.

### H-09 — “production grade” was a milestone label rather than an evidence model

A product may look complete while migration, resource bounds, release integrity or
interoperability remain unproven.

**Correction:** revision 4 adds per-capability maturity levels L0–L5 and capability
ledgers. M5 freezes an L4 production candidate only when every required-journey capability
has packaged integration, adversarial/failure evidence, independent interop where the
claim requires it, version-skew, migration, operations, supply-chain and recovery evidence.
Capabilities below L4 are disabled, removed or excluded from the active profile.

L5 requires A5 remediation and a human production decision; M5 is never called
production.

### H-10 — production evidence did not sufficiently cover falsification and release integrity

Ordinary unit and integration tests do not cover malformed protocol input, nondeterministic
artifacts, semantic version skew or operational recovery.

**Correction:** the plan now requires targeted fuzz/property suites, feasible sanitizers
and dynamic analysis, current/previous/unsupported profile fixtures, GUI/daemon/provider
skew, relay/signer/Blossom interop, four-hour pressure/soak evidence, SBOM, license and
advisory disposition, source/provenance mapping, previous-green artifacts, architecture-
boundary checks and two clean exact-input builds with every release-relevant variance
explained.

### H-11 — release/provenance signing was specified without lifecycle ownership

A signing key fingerprint is not enough if custody, rotation, revocation and compromise
response are undefined.

**Correction:** before L5, the plan requires named ownership, custody and use boundaries,
independent verification instructions, rotation, revocation, compromise quarantine and
reissue procedures. Release private material is excluded from normal CI, caches, logs,
review prompts and candidate-next probes.

### H-12 — application containment risked being described as an arbitrary-code sandbox

Exact hashes, CSP, origin separation, message validation and WebKit sandbox flags do not
prove safety against a browser-engine escape or hostile same-UID process.

**Correction:** guest trust tiers are explicit: `first_party_exact`,
`vetted_external_exact`, `experimental_external` and `unsupported_arbitrary`. M5/A5 audit
WebKit, website-data partitioning, media-worker isolation, systemd-user, SELinux,
network/destination enforcement and malicious guest fixtures. Production language must
match the evidence. Arbitrary hostile code receives no production safety claim unless
stronger isolation is separately proven.

### H-13 — architecture drift could defeat the intended boundaries without failing tests

A private adapter or daemon boundary can erode gradually through convenience imports,
Tauri callbacks or direct database/network access.

**Correction:** the plan requires a machine-enforced architecture-boundary checker and
self-tests. Only the private adapter may import provider/Nostr-engine types; runtime
mechanisms cannot depend on product/UI policy; the shell cannot open the daemon product
database; guests cannot receive native, network, signer, raw-path or direct-provider
bypasses.

### H-14 — irreversible signing/publication ordering remains a critical invariant

The previously identified defect—validating signer output after an API had already
published it—would make validation meaningless. The same applies to Blossom authorization
after upload bytes start moving.

**Correction retained and strengthened:** signer output is validated against the reviewed
canonical template before any relay write. Blossom authorization is validated before the
first upload body byte. A provider pin that cannot expose an equivalent staged/pre-send
seam blocks the owning phase rather than weakening the invariant.

### H-15 — moving-spec decisions were not preserved at the correct granularity

An ADR alone is too broad for every draft ambiguity, while an issue comment is too
transient. The previous plan lacked an exact interpretation artifact.

**Correction:** Spec Interpretation Records bind immutable conflicting sources, observed
behavior, chosen profile-limited interpretation, unsupported behavior, security and
persistence consequences, executable vectors, upstream route and recheck triggers.
Temporary profiles are never described as universal conformance.

### H-16 — phase summaries and chat could become the only memory of subtle failures

This makes later agents repeat mistakes and encourages educational material to invent a
clean story after the fact.

**Correction:** revision 4 defines a bounded record system: ADR, SIR, Upstream Record,
Learning Note, Capability Ledger, Phase Closeout and Milestone Learning Digest. Each has a
single purpose, evidence links, owner/status and visibility. Material lessons move from
temporal issue/chat/GSD output into the owning record before closeout.

### H-17 — automatic GSD learning extraction could become unreviewed authority

Generated learnings may be incomplete, duplicated or overgeneralized.

**Correction:** `gsd-extract-learnings` is raw intake only. Every item is dispositioned as
promotion to a durable record, ledger update, raw evidence, duplicate, rejection or
embargo. Only reviewed promotions become teaching inputs.

### H-18 — educational material lacked drift and disclosure controls

A future learning site or agent skill could silently teach superseded draft semantics or
leak internal/security details.

**Correction:** deterministic internal and public knowledge indexes carry stable IDs,
status, visibility, supersession, `verified_against`, `last_reviewed`, owner and affected
profile/capability. The public generator excludes internal/embargoed records and fails on
unknown visibility. Source/profile changes create review tasks; automation may mark
material `needs_review` but cannot silently rewrite explanatory prose as current fact.

### H-19 — educational output risked becoming a second implementation programme

Requiring polished tutorials in every product PR would slow delivery and create prose
before evidence stabilizes.

**Correction:** phase work captures only durable deltas and learning candidates. Each
milestone produces one bounded reviewed digest and educational backlog seeds. Full human
units, agent references and case studies are synthesized later from accepted records and
indexes. This keeps product development fast while preserving the raw material.

### H-20 — the post-M5 audit did not cover ecosystem stewardship or knowledge integrity

A runtime could pass code/security lanes while carrying stale profiles, silent patches or
false educational claims.

**Correction:** A5 now has twelve lanes. Lane 11 audits ecosystem compatibility, exact
pins, radar/campaign/upstream/local-patch health, independent interop and version support.
Lane 12 audits decision/learning traceability, generated indexes, visibility, disclosure,
drift and educational readiness. There is no extraction-readiness lane.

### H-21 — shared terminology could drift between agents, code and educational material

A moving ecosystem already uses overloaded nouns such as profile, actor, subject, build,
session, grant, capability, publication and delivery. Without a canonical concept record,
separate phases and agents can preserve syntax while changing meaning.

**Correction:** M0 creates a bounded `terms.toml` registry with stable term IDs, precise
definition/non-definition, aliases, semantic owner, profile/capability applicability,
immutable sources and supersession. Records, closeouts and teaching material reference
term IDs where ambiguity matters; changed meaning is superseded rather than silently
rewritten.

### H-22 — upstream AI assistance, authorship and signoff were not governed

A technically correct patch can still violate upstream contribution policy, misstate
authorship or make an agent appear accountable for claims it did not independently verify.

**Correction:** every public interaction records repository AI policy, named human
submitter/approver, AI assistance, truthful authorship, signoff and DCO/CLA disposition.
The human understands and takes responsibility. The process forbids fabricated maintainer
agreement, provenance, tests, authorship or signoff.

### H-23 — bounded queues did not prove fairness or anti-starvation

A runtime can remain globally bounded while one build/profile/session monopolizes workers,
subscriptions, composition or diagnostics and starves required journeys.

**Correction:** every guest-influenced work class defines an admission principal, global
and per-principal limits, fair scheduling or an explicit bounded alternative, typed
overload behavior, cancellation/cleanup and noisy/quiet plus many-principal
anti-starvation tests. These limits are part of the active compatibility profile and A5
resource/security evidence.

### H-24 — the NIP-46 client-key lifecycle remained too abstract

“Secure storage or session-only” did not cover random generation, copy minimization,
rotation, signer-side revocation, profile deletion, restore, compromise or truthful
limits of erasure.

**Correction:** M3 and A5 now require approved generation/import, one narrow owner,
meaningful zeroization where testable, protected persistence or explicit session-only
operation, no plaintext fallback, complete rotation/revocation/deletion/backup/restore and
compromise tests, and honest limits for swap, crash dumps, snapshots and hostile same-UID
access.

### H-25 — educational claims could remain non-executable prose

Source citations alone do not let a human or agent falsify a subtle runtime claim.

**Correction:** each promoted non-elementary teaching claim links to an exact fixture,
vector, test, trace or measured report, the command/inspection path, expected observable
result, canonical term IDs and exact source/profile identity. A5 audits these witnesses
and blocks polished claims that cannot be traced to current executable evidence.

### H-26 — compatibility-profile transitions lacked release-channel identity

A machine-readable profile diff is necessary but not sufficient when a mutable channel or
silent updater can replace package or napplet bytes underneath the reviewed profile.

**Correction:** candidate, canary and stable are now separate signed immutable package/RCP
states. Runtime and napplet updates cannot silently substitute bytes or increase authority.
Post-A5 canary use is explicit, bounded and reversible, with predeclared privacy-safe health
and rollback thresholds plus a second human promotion decision.

### H-27 — automated and self-review evidence could still be mistaken for production security approval

A system mediating signing, identity, untrusted guest input, network destinations and durable
side effects should not earn L5 solely from its implementation agents and automated review
tools.

**Correction:** A5 Lane 3 now requires a named human reviewer or team without implementation
ownership for the critical-boundary security review. Scope, conflicts, methods, exact
evidence, findings, remediation and retest are recorded. L5 also requires an opt-in canary
and a separate stable-release decision; any code/profile/package change creates a new
candidate rather than waiving the audit.

## Cross-document coherence result

The re-audited programme has one consistent sequence:

```text
Phase 1         M0 exact replay/package/authority/ecosystem baseline
2–2.7           M1 packaged guest boundary, Social Home and compatibility capstone
3–3.3           M2 daemon-owned offline authoring
4–4.3           M3 external signer and pre-send-verified publication
5–5.3           M4 bounded image/Blossom round trip
6–6.2           M4.5 scheduling, recovery and composition
7–7.9           M5 L4 production-candidate hardening and exact freeze
A5              mandatory twelve-lane audit; no implementation or automatic extraction
```

Each post-M0 integer or decimal phase is one bounded issue/worktree/primary-PR increment.
Continuous lanes—security, compatibility, maturity, decisions, upstream and learning—are
closeout obligations, not parallel feature programmes.

## Production-readiness assessment

Revision 4 now builds toward production in the right way:

- **secure:** authority, identity, grants, destinations, signing, objects, side effects,
  containment limits and incident response are explicit and adversarially tested;
- **composable:** negotiation, mediation, no-transitive-authority, backpressure, lifecycle,
  confused-deputy and independent-peer criteria are defined;
- **operable:** installation, instances, diagnostics, migration, backup, restore, rollback,
  supported versions and security response are part of the product claim;
- **reproducible:** execution and profiles bind immutable sources, locks, derivations,
  package bytes and release evidence;
- **evolvable:** upstream movement is observed without floating production, and adoption
  is evidence-led and reversible;
- **teachable:** decisions, ambiguities, negative results and operational nuance remain
  traceable to exact evidence and can be projected into human and agent resources.

The remaining risk is not plan incoherence. It is whether implementation can satisfy the
hard gates without narrowing the supported profile. The plan correctly permits narrowing
or disabling a claim instead of faking evidence.

## Residual implementation evidence gates

The following cannot be settled honestly in a planning pack and remain owned execution
or A5 gates:

- exact current source, lock, package and toolchain state in the Uzel repository;
- availability and integrity of the historical dependency closure;
- usefulness or correctness of isolated commit `b185ad1`;
- exact source and behavior of the selected NIP-5A/NIP-5D/NAP/library pins;
- whether the selected provider exposes a true sign-then-validate-then-submit seam;
- exact WebKit origin, storage, process and network behavior on the supported matrix;
- practical SELinux/systemd-user/media-worker confinement;
- secure persistence and recovery of the NIP-46 client identity/key;
- reproducibility of two clean exact-input builds;
- independent relay, signer, Blossom and napplet peer availability;
- measured startup, latency, memory, wakeup, queue, cancellation and soak budgets;
- release/provenance key custody and operational ability to revoke/quarantine;
- current repository-specific contribution, security, license and DCO/CLA requirements;
- installed GSD/Codex/CodeRabbit command semantics at the exact pinned versions.

Failure of one of these gates must block, narrow or amend the affected capability/profile.
It must not be converted into an undocumented assumption.

## Artifact audit expectations

The included audit script checks:

- exact archive allowlist;
- payload hashes and byte counts;
- Markdown fences and relative links;
- duplicate consecutive lines and unresolved placeholder markers;
- TOML template syntax;
- exact delivery-phase and A5-lane sequences;
- runbook/prompt coverage;
- required production, compatibility, upstream, learning and disclosure invariants;
- stale revision/command/terminology patterns.

Mermaid diagrams are structurally counted and fenced. They are not rendered by this pack;
a real Mermaid render remains a repository/tooling check.

## Final acceptance of the plan pack

Revision 4 is accepted for repository installation and GSD reorientation when its included
machine audit passes from a fresh extraction and its checksum matches.

Acceptance means:

- the plan is coherent enough to govern implementation;
- the programme can earn a bounded production claim;
- fast-moving upstreams have a controlled observation, contribution and adoption process;
- decisions and learning can become truthful educational resources;
- known uncertainty is assigned to explicit evidence gates.

It does **not** mean the implementation has already earned production, security,
composability, conformance or release approval.
