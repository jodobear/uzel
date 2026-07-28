# Uzel POC agent instructions

## Mission

Build only the POC defined in `docs/00-scope.md`. Preserve the future `napd`/Uzel/napplets boundaries while optimizing for a fast, demonstrable Linux vertical slice.

## Required method

1. Read the active `work/*.md` file and only the documents it names.
2. Validate its assumptions before changing production code.
3. Stop when observed source or runtime behavior contradicts the plan.
4. Record the fact, correct the affected design, then continue.
5. Keep one branch/worktree focused on one work slice.
6. Update `STATUS.md` with commits, commands, evidence, limitations, and next action.

A remembered API, branch name, chat message, README claim, or expected PR merge is not evidence. Use exact commits, source paths, package versions, command output, tests, and reproducible probes.

## Engineering rules

- KISS and Linux philosophy: small owners, explicit interfaces, composition, bounded state, independent failure.
- Reuse in this order: **reuse → adapt → contribute → build only the proven missing seam**.
- Use `nampplets`; do not create a competing runtime model.
- NMP is the sole Nostr query, relay, event-store, freshness, provenance, and diagnostics engine.
- Do not create a second profile, follow, relay, or event cache outside NMP.
- Do not invent cryptography, signing, key formats, secure channels, or random-number behavior.
- Keep functions cohesive, control flow shallow, errors typed, and queues/retries bounded.
- No generic `utils`, catch-all managers, silent fallback, fake implementation, or placeholder-success test.
- Prefer one complete path over a generic framework.

## Repository boundaries

```text
apps/uzel          product shell and presentation
apps/uzel-napd     daemon binary
crates/napd        reusable runtime mechanisms
crates/napd-protocol  internal shell/daemon contract
napplets/*         portable untrusted napplets
```

Rules:

- `crates/napd*` must not depend on Tauri, Svelte, Uzel styling, or demo napplet identities.
- Napplets must not import Uzel or runtime implementation code.
- Svelte does not own runtime truth or daemon protocol types.
- The Tauri Rust backend is the daemon client; daemon IPC need not be projected into TypeScript.
- Cross-napplet payloads have one schema owner under `contracts/`.

## Trust rules

- Napplet frames are untrusted.
- They receive no raw network, Tauri bridge, secret, host path, or caller-selected principal.
- The trusted host maps `MessageEvent.source` to the surface it created.
- Runtime identity comes from a verified manifest/exact build.
- The selected Nostr pubkey is a read context, not authenticated login.

## POC exclusions

Do not add FIPS, ContextVM, Relatr, Open Ranking, wallets, signers, Blossom, media, mounts, plugins, native/WASI napplets, Android, app discovery, auto-update, multi-WebView production work, custom WM/compositor work, or a public remote daemon protocol.

## Quality gate

Run only commands validated in `work/00-validate.md`. Expected categories:

- Rust formatting, Clippy, tests;
- frontend formatting, type checking, tests, accessibility smoke;
- Fallow for changed TypeScript/Svelte;
- Napplet conformance/fixture tests;
- hostile iframe/network tests;
- documentation audit.
