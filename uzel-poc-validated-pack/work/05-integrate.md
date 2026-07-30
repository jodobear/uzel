# Work 05 — composed demo

## Goal

Integrate the daemon, Linux runner, and two napplets into the user-visible POC.

## Tasks

- Start/connect daemon from Uzel.
- Select and persist a public read identity.
- Verify and start both exact builds.
- Render two panes with focus, resize, orientation, and fullscreen.
- Bind `napplet:profile/open` to the running profile-card handler.
- Show latest-known/degraded status without inventing freshness that the released NAP wire does not expose.
- Add a compact developer drawer for sessions, envelopes, NMP demand/evidence, and errors.
- Prove restart behavior.

## Acceptance

The deterministic demo works end to end; selecting a follow changes profile-card only through runtime-routed NAP-INC; the live demo works with configured relays.

## Non-goals

No command palette, workspaces, overview, Lua, host-WM adapter, remote assets, or design-system project.
