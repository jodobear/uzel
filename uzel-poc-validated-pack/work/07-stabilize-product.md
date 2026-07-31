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

Parallel work is allowed only when file ownership and protocol ownership do not
overlap. Read-only investigation may run ahead. No implementation branch may
silently redefine NAP, NIP-5D, NMP, or `nampplets` behavior.

## Completed issue — #10

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
- [x] Open one PR referencing #10; merge only after exact-head checks and Codex
  review are clean.

Accepted evidence: PR #20 merged as
`83e2e1e7e1565b6fb4ab24ac5b8dc4ecb94fbfc0`; exact reviewed head
`b7eda9afc42b67c695e2e1ba7fe9f02be52ec5b6`; `pnpm test:ui` 17/17;
Chromium `150.0.7871.186`; 10 accepted outcomes; 15 screenshots; zero cleanup
errors; every captured Chromium process exited; the isolated deliberate-fault
child exited nonzero without timeout and its process group exited. The run is
preserved under ignored
`.artifacts/ui-acceptance/2026-07-31T00-46-39-183Z.175943/`.

## Active issue — #19

- [x] Reproduce the pre-pane retry path with an empty selected identity.
- [x] Keep public identity selection locked while recovery is pending, but give
  bootstrap a focused internal activation path.
- [x] Prove neither base surface launches before identity activation completes.
- [x] Prove a failed identity activation launches zero surfaces, remains
  recoverable, and succeeds on a second retry.
- [x] Keep user-driven pane restart diagnostics after successful identity
  activation and pane restart.
- [x] Make restart reconciliation wait for the selected profile rather than an
  already-ready shell.
- [x] Run renderer acceptance, repository gates, real Linux smoke, screenshot
  inspection, documentation audit, and `graphify update .`.
- [ ] Merge PR #22 only after exact-head checks and Codex review are clean.

Exact review evidence: PR #22 head
`8782260cb5d685eb0cce698682d77248ea7fc94f`; `pnpm test:ui` 22/22;
empty-identity and one-shot identity-failure recovery pass at fixed desktop
viewports; the real Weston/WebKit smoke reports `LINUX_RUN_SMOKE_OK` with both
network and native sentinels zero. Visible Debian interactive acceptance remains
a separate human-visible gate. The current review action is to synchronize this
handoff with `STATUS.md`; no product behavior changed after the accepted gates.

## Exit rule

Work 07 stays active while tracker #9 has accepted child issues. `STATUS.md`
records the current issue and exact evidence. A discovered defect becomes a
contextual issue or a failure of the active issue's acceptance contract; it does
not start an unrelated inline fix.
