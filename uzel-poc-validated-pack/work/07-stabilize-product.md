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

Parallel work is allowed only when file ownership and protocol ownership do not
overlap. Read-only investigation may run ahead. No implementation branch may
silently redefine NAP, NIP-5D, NMP, or `nampplets` behavior.

## Active issue — #10

- [ ] Commit a deterministic Playwright renderer harness with an explicitly fake
  native boundary.
- [ ] Exercise controls by accessible role/name, not CSS implementation details.
- [ ] Fail on console errors, page errors, or unexpected external requests.
- [ ] Cover base shell, Settings, Debug, signed-`naddr` review, and recovery
  states at fixed desktop viewports.
- [ ] Preserve useful failure screenshots/traces outside Git; do not commit
  generated acceptance images or the user's desktop screenshots.
- [ ] Prove process cleanup leaves no harness-owned server or browser process.
- [ ] Keep mocked renderer acceptance distinct from real Weston/WebKit smoke.
- [ ] Run repository checks, documentation audit, and `graphify update .`.
- [ ] Open one PR referencing #10; merge only after exact-head checks and Codex
  review are clean.

## Exit rule

Work 07 stays active while tracker #9 has accepted child issues. `STATUS.md`
records the current issue and exact evidence. A discovered defect becomes a
contextual issue or a failure of the active issue's acceptance contract; it does
not start an unrelated inline fix.
