# POC documentation audit

## Verdict

Assumption validation existed conceptually in the previous pack, but it was not executable enough. This rewrite makes validation a hard Gate 0 and reduces the remaining documents to the minimum context needed for a fast POC.

## Problems found in the previous pack

1. The README and execution plan referenced `work/00` through `work/08`, but the `work/` directory was absent.
2. Fact and handoff templates were referenced but absent.
3. The audit report claimed an included audit script, but the script was absent.
4. The report claimed zero broken relative links, while the extracted pack contained broken links.
5. Several documents were labeled LLDDs before Linux `nampplets`, NMP, and WebKit assumptions had been proven.
6. The proposed source tree had too many crates for a quick POC.
7. Daemon IPC was projected into both Rust and TypeScript without a demonstrated need.
8. Bubblewrap, comprehensive diagnostics, generic IPC lifecycle, and broad persistence details were mandatory too early.
9. Acceptance mixed “fast visible demo” and “foundation hardening” into one all-or-nothing gate.

## Corrections

| Previous issue | Correction |
|---|---|
| Missing execution artifacts | Added seven real `work/*.md` files and two templates |
| Assumptions hidden inside LLDDs | Replaced them with Gate 0 plus a clearly provisional design |
| Premature crate decomposition | Reduced runtime shape to `napd` and `napd-protocol` |
| Unnecessary TS daemon contract | Tauri Rust backend owns daemon IPC; Svelte gets product projections |
| Over-broad first milestone | Split demo-complete from foundation-complete |
| Bubblewrap blocked speed | Kept as valuable hardening, not a fake or mandatory visual-demo dependency |
| Guessed limits/schemas | Deferred exact values to source probes and measured fixtures |
| Excess context | Reduced architecture corpus to seven focused documents |

## Diagram review

The nine Mermaid diagrams cover the levels needed for this POC:

- product outcome;
- validation decision loop;
- process topology;
- dependency boundaries;
- session start;
- Nostr read flow;
- inter-napplet routing;
- slice dependency graph;
- post-POC extraction.

They revealed and corrected three important design issues:

1. Svelte should not share or generate daemon IPC types; the Tauri Rust backend is the client boundary.
2. A fast POC does not need a large family of `napd-*` crates.
3. Exact IPC framing, storage schema, CSP, and identity projection must remain provisional until Gate 0.

A structural Mermaid check is included. Full rendering with a pinned Mermaid CLI remains part of repository tooling setup.

## Gate 0 resolution

Work 00 resolved the implementation assumptions with exact pins and executable Linux probes: reusable `nampplets` APIs, Kehto/package/spec revisions, NMP facade calls, Tauri/WebKit frame isolation, strict CSP construction, daemon IPC, and actual tool commands. Uzel explicitly accepted the exact unratified nampplets fork for the Linux-only POC. Apple catalog tests are outside Uzel scope; Bubblewrap remains deferred hardening.

## Confidence

- POC scope and boundaries: high
- validation method: high
- single-repository extraction path: high
- exact upstream adapter code: high for the pinned candidate
- Linux WebKit security details: high for the probed paths
- final implementation schedule: unblocked for Slice 01; later slices retain their own falsifying gates
