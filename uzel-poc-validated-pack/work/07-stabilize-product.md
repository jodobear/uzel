# Work 07 — issue-driven stabilization

## Goal

Turn the accepted Linux POC into a coherent, self-tested product baseline without
mixing UI polish, upstream compatibility, protocol authority, or runtime
reliability fixes into one ad-hoc branch.

## Entry evidence

- Uzel PR #8 is merged on `master` as
  `1e4cc6abbff4bdded97bfd0b3de4d891f4dee29c`.
- The exact reviewed PR head was
  `c6dbc1e2522772e92aac661314be3bc4cadaaa19`; Codex reported no major issue.
- Locked Debian 13 build and Weston/WebKit smoke pass.
- A live desktop walkthrough exposed missing product behavior and visual debt
  that deterministic smoke does not cover.

## Work graph

GitHub issue #9 is the tracker. Each child issue owns one branch, one bounded
change, and one PR:

| Issue | Owner | Dependency |
|---|---|---|
| #10 committed renderer acceptance harness | product-test lane | first; supplies UI regression gate |
| #11 validated `nampplets` successor pin | compatibility lane | exact upstream/fork source evidence |
| #12 authoritative live identity evidence | NMP/runtime lane | authority decision before implementation |
| #13 signed `naddr` load UX | product lane | #10 |
| #14 settings and editable keybindings | product lane | #10 |
| #15 compact, hideable shell chrome | product lane | #10 and #14 information architecture |
| #16 relay maintenance and diagnostics | NMP/runtime lane | #12 |
| #17 rendered WebKit image proof | resource/runtime lane | #10 |
| #18 daemon control responsiveness | daemon lane | independent benchmark and bounded fix |
| #19 pre-pane retry identity lock | product/runtime lane | discovered by #10; fix after harness merge |
| #21 external napplet corpus | product-test lane | data lock independent; launch #13; image #17; pin #11 |

Parallel work is allowed only when file ownership and protocol ownership do not
overlap. Read-only investigation may run ahead. No implementation branch may
silently redefine NAP, NIP-5D, NMP, or `nampplets` behavior.

## Active issue — #10

- [x] Commit a deterministic Playwright renderer harness with an explicitly fake
  native boundary.
- [x] Exercise controls by accessible role/name, not CSS implementation details.
- [x] Fail on console errors, page errors, or unexpected external requests.
- [x] Cover base shell, Settings, Debug, signed-`naddr` review, and recovery
  states at fixed desktop viewports.
- [x] Preserve useful failure screenshots/traces outside Git; do not commit
  generated acceptance images or the user's desktop screenshots.
- [x] Prove process cleanup leaves no harness-owned server or browser process.
- [x] Keep mocked renderer acceptance distinct from real Weston/WebKit smoke.
- [x] Run repository checks, documentation audit, and `graphify update .`.
- [ ] Open one PR referencing #10; merge only after exact-head checks and Codex
  review are clean.

Exact pre-PR evidence: commit
`1847a7a5cbcd26723c7167320fc313e0c9fd0794`; `pnpm test:ui` 17/17;
Chromium `150.0.7871.186`; 10 accepted outcomes; 15 screenshots; zero cleanup
errors; every captured Chromium process exited; the isolated deliberate-fault
child exited nonzero without timeout and its process group exited. The run is
preserved under ignored
`.artifacts/ui-acceptance/2026-07-31T00-46-39-183Z.175943/`.

## Parallel issue — #21 data-only corpus

- [x] Freeze the exact source commit, license, publisher, naddr, signed event,
  author, kind, `d`, artifact path digest/length, aggregate, domains, and servers
  for Good Morning, Rubik Cube, Nap Feed, and WiFi Map.
- [x] Keep signed event records but no downloaded artifact blobs.
- [x] Verify lock structure, signed-event tuples, event hashes/signatures, and
  NIP-19 coordinates entirely offline with pinned `nak`.
- [x] Test that signature drift exits as trust failure while missing verifier
  tooling exits as infrastructure failure.
- [x] Record provenance, license limits, source/artifact limitation, and a
  deliberate refresh procedure.
- [x] Keep the corpus out of runtime registration, naddr launch, dependency
  pins, and product UI.

Exact branch evidence: corpus commit `b0ba7f9`; verifier commit `6580d9a`;
offline verifier 4/4; classification self-test trust=`2`, infrastructure=`3`;
`pnpm check`, lint/boundaries, Fallow, docs audit, and Rust formatting pass.
`pnpm test` passes the new corpus tests, all JavaScript tests, and Rust library
tests, then this non-Debian host cannot link the unchanged Tauri test binary
because pinned Nix WebKitGTK needs `GLIBC_2.42`. No runtime integration is
claimed; later live launch remains gated by #13 and rendered evidence by #17.

## Exit rule

Work 07 stays active while tracker #9 has accepted child issues. `STATUS.md`
records the current issue and exact evidence. A discovered defect becomes a
contextual issue or a failure of the active issue's acceptance contract; it does
not start an unrelated inline fix.
