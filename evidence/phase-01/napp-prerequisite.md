# Napp external prerequisite

This packet records a committed-object STOP. It grants no acceptance, mutation, publication, adapter, Cargo, lock, runner, or fixture authority.

candidate repository: `jodobear/napp`  
candidate commit: `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e`  
candidate tree: `b12b0c13b11ce5f64e4fd91025789ae692438f38`  
result: stop

## Fixed qualification

Commands executed from Uzel HEAD `c4fabecd2b022fc2c53c9aa9dc5d2847e551a4d3` on 2026-08-13:

```sh
python3 scripts/ref-candidate-check.py self-test
python3 scripts/ref-candidate-check.py qualification \
  --repo /workspace/projects/napplets/napp-uzel/napp \
  --expected-repository jodobear/napp \
  --expected-commit 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e \
  --record evidence/phase-01/candidate-qualification.md \
  --expected-result stop
```

Observed results:

```text
self-test: pass (negative probes rejected before spawn)
qualification: pass
```

The checker used its fixed finite Git grammar, expected repository, expected full commit, and expected terminal result. It did not execute candidate-supplied commands, network probes, or sibling working-tree material. `evidence/phase-01/candidate-qualification.md` and `evidence/phase-01/napp-dependency.md` remain canonical read-only inputs.

## Missing admission categories

The committed candidate remains missing every required category:

1. product client
2. product events
3. testkit vectors
4. version authority
5. lifecycle recovery
6. instance/profile scope
7. NMP ownership projection
8. pin-parity input
9. declared executable probes

Canonical blocker: `Committed candidate lacks source-backed admission evidence and an admitted project-probe sandbox contract.` The committed candidate declares no safe project probe contract, so its required probe remains `skipped-unsafe`; this is a STOP, not a substitute validation.

## External sibling observation

Planning-time observation was Napp `HEAD` and `master` at `96e7d706e15b6f1ed4d1f22a368c57601394e0fc`, distance `10` from candidate to `HEAD`, with dirty state. Execution remeasured only with read-only Git:

```sh
git -C /workspace/projects/napplets/napp-uzel/napp rev-parse HEAD master
git -C /workspace/projects/napplets/napp-uzel/napp rev-list --count 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e..HEAD
git -C /workspace/projects/napplets/napp-uzel/napp status --short
```

Observed `HEAD`: `96e7d706e15b6f1ed4d1f22a368c57601394e0fc`  
Observed `master`: `96e7d706e15b6f1ed4d1f22a368c57601394e0fc`  
Observed distance: `10`  
Observed status: dirty — tracked modifications under `.planning/`, `AGENTS.md`, and `README.md`; untracked planning, documentation, reports, `docs/`, `evidence/`, archive, templates, and tool material.

This is excluded external state, not candidate qualification evidence. It is recorded only to prove that issue #42 gives Uzel no authority to accept or mutate the sibling checkout.

## Handoff and inventory disposition

The current handoff validator requires this archived plan object:

```text
.planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-01-PLAN.superseded.md
```

Read-only lookup failed exactly: `fatal: path '.planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-01-PLAN.superseded.md' does not exist in 'HEAD'` (exit `128`).

handoff validation: unavailable

`evidence/phase-01/napp-dependency.md` therefore remains unchanged and no handoff pass is claimed.

The fixed qualification emitted `.artifacts/phase-01/napp/0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e/tree.bin`. `git check-ignore -v` reports `/.artifacts/` and `git ls-files --error-unmatch` exits `1`: the inventory is ignored, untracked, unstaged, and uncommitted. It is not product evidence.

## Required next authority

REF-07 status: blocked

Phase 1 status: open

The accepted POC pin remains preserved. Authority owner: `jodobear/napp`. Next action: **Napp owner/source-authority resolution and committed qualifying evidence**.

Issue #42 and PR #43 remain the single Phase 1 delivery unit. No second issue, plan, branch, or PR is created; this unit resumes only after `jodobear/napp` publishes qualifying committed evidence.
