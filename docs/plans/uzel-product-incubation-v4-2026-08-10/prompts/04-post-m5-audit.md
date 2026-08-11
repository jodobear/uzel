# Prompt — conduct the mandatory A5 whole-system audit

The exact Uzel M5/7.9 candidate is frozen. Conduct A5 as an independent whole-system
audit, not a release celebration, cleanup sprint or package-design exercise.

Read `05-POST-M5-AUDIT.md`, the frozen source/Nix/evidence manifest, every phase review
artifact, authority/threat/durable-format/migration registries, previous-green fixtures
and known debt/unsupported claims.

First prove every lane audits the same Git SHA and Nix result. Then audit all twelve lanes:
architecture/product fit; correctness/state machines; application/protocol security;
Linux/platform hardening; data/migration/backup/recovery; concurrency/resources/
performance; Nix/supply-chain/CI; UX/visual/accessibility; operations/supportability;
maintainability/documentation; ecosystem compatibility/upstream stewardship; knowledge integrity/educational readiness.

Explicitly test:

- local profile vs actor vs viewed subject and signer-reported-key mismatch;
- first-party guest parity and absence of privileged Composer/Home/Profile shortcuts;
- trusted decision anti-spoof/nonces and guest focus/click-through attempts;
- build-scoped origins and website-data cleanup;
- destination resolution/connection binding, DNS rebinding, redirects, proxy bypass and
  local/self-hosted exception scope;
- the fact that co-located daemon modules are not process security isolation;
- two profiles, two instances, previous-green migration/restore/rollback;
- low-authority media-worker sandbox, protocol/resource limits, crash/recycle behavior
  and malicious files/servers;
- reviewed canonical event-template versus final signed event, including allowed filled
  fields, unreviewed-field injection and proof that validation precedes every relay write;
- signer-produced Blossom authorization validation before any upload body;
- cross-profile object/derived-cache isolation and media-worker restart before profile reuse;
- exact-revision future-action grant scope, revocation, due-time revalidation and signer
  refusal;
- all kill/recovery/unknown-side-effect boundaries;
- global/per-build/profile/session/capability admission, fairness, typed overload and
  anti-starvation under noisy/quiet and many-principal pressure;
- complete NIP-46 client-key generation/import, protected/session-only storage, rotation,
  revocation, deletion, backup/restore and compromise lifecycle;
- external-review privacy/provenance;
- canonical exact-UTF-8 RCP byte rules/hash scheme/package binding/startup verification
  versus immutable spec/proposal/package/tool/provider sources and generated human rendering;
- manifest/build identity, fail-before-guest required/optional negotiation, bound transcript
  and actual packaged conformance path;
- externally consumable compatibility kit, Uzel-authored clean-room fixture, and a
  separately authored/commissioned clean-room napplet using only the public kit for the
  core L4 runtime-composability claim; community-maintained-peer evidence is separate;
- no-transitive-authority composition and current/previous/unsupported version skew;
- interop across independent relay/signer/Blossom peers and no unresolved
  `blocked_no_independent_peer` for the core composable-runtime target;
- upstream/local-patch records separating public thread, accepted, merged, released,
  exact Uzel adoption and patch removal, including contributor/disclosure compliance;
- L4 capability ledgers, fuzz/sanitizer corpora, SBOM/license/advisory/provenance,
  two-clean-build comparison, architecture-boundary checker, release-signing key custody/
  rotation/revocation/compromise policy and security-response/incident artifacts;
- candidate-next isolation and phase-pinned GSD/Codex/CodeRabbit/toolchain evidence;
- canonical terminology and ADR/SIR/upstream/curated-GSD-learning/milestone-digest/
  education traceability, exact executable teaching witnesses, internal/public knowledge-
  index visibility/embargo checks and contradictions with source/tests/schema/profile/package.

Use independent reviewers and real package/native execution. Lane 3 must include a named
human security reviewer or team without implementation ownership; record conflicts,
scope, methods, exact evidence and findings. Automated tools do not replace that verdict.
Never send secrets, pairing URIs, production content or private diagnostics to reviewers.

Also test signed immutable candidate/canary/stable metadata, no-silent-update behavior,
opt-in canary scope, privacy-bounded health and rollback thresholds, previous-green
recovery and the requirement for a second named-human stable-release decision.

Produce exact candidate identity, lane reports/reviewers/methods, findings ledger,
cross-lane synthesis, unsupported claims, remediation/retest matrix, residual risk and
exactly one status: `fail`, `remediation_required`, or `pass_for_human_decision`.

Do not run `$gsd-complete-milestone` and do not choose future package/repository
boundaries. Remediation that changes authority, schema, dependency, package or shared
behavior creates a new candidate and reruns affected lanes plus integrated evidence.
Automation stops at `pass_for_human_decision` for the owner.
