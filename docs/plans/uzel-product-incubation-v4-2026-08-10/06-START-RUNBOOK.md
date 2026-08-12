# Installation and Phase 1 restart runbook

## Destination

Install the pack in the active Uzel repository at:

```text
docs/plans/uzel-product-incubation-v4-2026-08-10/
```

Do not extract it into `.planning/`, the blocked `01-01` worktree, another repository, a
dirty checkout or a directory containing an older pack. Commit it through a planning-
only PR first; then create a fresh manual Phase 1 worktree from the merged integration
branch.

## Step 0 — pause and preserve the blocked execution

In the blocked top-level Codex/GSD session:

```text
$gsd-pause-work --report
```

Stop that session. Do not continue `01-01` or delete its worktree.

From an ordinary Uzel checkout, create both a durable local ref and a portable,
checksummed Git bundle outside the repository:

```bash
set -euo pipefail

repo_dir="/absolute/path/to/jodobear/uzel"
evidence_dir="/absolute/private/path/uzel-phase-1-b185ad1"
blocked_worktree="/absolute/path/to/blocked-01-01-worktree"
blocked_branch="gsd/phase-01-plan-01-replay"
wip_commit="b185ad1b8d9d034d151406b12aa189f5a6be970f"
wip_parent="431e37af5ca86196dbaf08a534a0a7626c4ae32c"

cd "$repo_dir"
umask 077
git rev-parse --is-inside-work-tree >/dev/null
repo_common_dir="$(realpath "$(git rev-parse --git-common-dir)")"
blocked_common_dir="$(
  realpath "$(git -C "$blocked_worktree" rev-parse --git-common-dir)"
)"
test "$blocked_common_dir" = "$repo_common_dir"
test "$(git -C "$blocked_worktree" branch --show-current)" = "$blocked_branch"
test "$(git -C "$blocked_worktree" rev-parse HEAD)" = "$wip_commit"
if ! git cat-file -e "$wip_commit^{commit}" 2>/dev/null; then
  test -f "$evidence_dir/b185ad1.bundle"
  git bundle verify "$evidence_dir/b185ad1.bundle"
  git fetch "$evidence_dir/b185ad1.bundle" \
    refs/heads/wip/phase-1-replay-b185ad1:refs/heads/wip/phase-1-replay-b185ad1
fi
test "$(git rev-parse "$wip_commit^")" = "$wip_parent"
git show --stat --oneline "$wip_commit"

if git show-ref --verify --quiet refs/heads/wip/phase-1-replay-b185ad1; then
  test "$(git rev-parse wip/phase-1-replay-b185ad1)" = "$wip_commit"
else
  git branch wip/phase-1-replay-b185ad1 "$wip_commit"
fi

mkdir -p "$evidence_dir"
if [ ! -e "$evidence_dir/b185ad1.bundle" ]; then
  git bundle create \
    "$evidence_dir/b185ad1.bundle" \
    wip/phase-1-replay-b185ad1
fi

git bundle verify "$evidence_dir/b185ad1.bundle"
bundle_head="$(
  git bundle list-heads "$evidence_dir/b185ad1.bundle" |
    awk '$2 == "refs/heads/wip/phase-1-replay-b185ad1" { print $1 }'
)"
test "$bundle_head" = "$wip_commit"

if [ ! -e "$evidence_dir/b185ad1-metadata.txt" ]; then
  {
    git show --format=fuller --stat --summary b185ad1
    printf '\nparent=%s\n' "$wip_parent"
    printf 'commit=%s\n' "$wip_commit"
    git worktree list --porcelain
  } > "$evidence_dir/b185ad1-metadata.txt"
fi
grep -Fx "parent=$wip_parent" \
  "$evidence_dir/b185ad1-metadata.txt"
grep -Fx "commit=$wip_commit" \
  "$evidence_dir/b185ad1-metadata.txt"

snapshot_dir="$evidence_dir/blocked-worktree-v2"
if [ -e "$snapshot_dir" ]; then
  (
    cd "$snapshot_dir"
    sha256sum -c SHA256SUMS
  )
else
  capture_dir="$(mktemp -d "$evidence_dir/.blocked-worktree-v2.XXXXXX")"
  case "$capture_dir" in
    "$evidence_dir"/.blocked-worktree-v2.*) ;;
    *) printf 'unsafe capture directory\n' >&2; exit 1 ;;
  esac
  trap 'rm -rf -- "$capture_dir"' EXIT

  git -C "$blocked_worktree" status --porcelain=v2 -z \
    > "$capture_dir/blocked-status-v2.z"
  git -C "$blocked_worktree" diff --binary \
    > "$capture_dir/blocked-unstaged.patch"
  git -C "$blocked_worktree" diff --cached --binary \
    > "$capture_dir/blocked-staged.patch"
  git -C "$blocked_worktree" ls-files --others --exclude-standard -z -- \
    > "$capture_dir/blocked-untracked.z"
  (
    cd "$blocked_worktree"
    tar --null --files-from="$capture_dir/blocked-untracked.z" \
      --no-recursion -cf "$capture_dir/blocked-untracked.tar"
  )
  (
    cd "$capture_dir"
    sha256sum \
      blocked-status-v2.z blocked-unstaged.patch blocked-staged.patch \
      blocked-untracked.z blocked-untracked.tar > SHA256SUMS
    sha256sum -c SHA256SUMS
  )
  mv "$capture_dir" "$snapshot_dir"
  trap - EXIT
fi

(
  cd "$evidence_dir"
  expected_checksum_paths="$(
    printf '%s\n' \
      b185ad1.bundle b185ad1-metadata.txt \
      blocked-worktree-v2/SHA256SUMS \
      blocked-worktree-v2/blocked-status-v2.z \
      blocked-worktree-v2/blocked-unstaged.patch \
      blocked-worktree-v2/blocked-staged.patch \
      blocked-worktree-v2/blocked-untracked.z \
      blocked-worktree-v2/blocked-untracked.tar |
      sort
  )"
  if [ -e SHA256SUMS ]; then
    actual_checksum_paths="$(awk 'NF == 2 { print $2 }' SHA256SUMS | sort)"
    if [ "$actual_checksum_paths" != "$expected_checksum_paths" ]; then
      printf '%s\n' \
        'stale evidence SHA256SUMS: preserve this directory and use a new evidence_dir or migrate it explicitly' >&2
      exit 1
    fi
    sha256sum -c SHA256SUMS
  else
    sha256sum \
      b185ad1.bundle b185ad1-metadata.txt \
      blocked-worktree-v2/SHA256SUMS \
      blocked-worktree-v2/blocked-status-v2.z \
      blocked-worktree-v2/blocked-unstaged.patch \
      blocked-worktree-v2/blocked-staged.patch \
      blocked-worktree-v2/blocked-untracked.z \
      blocked-worktree-v2/blocked-untracked.tar > SHA256SUMS.next
    mv SHA256SUMS.next SHA256SUMS
  fi
  sha256sum -c SHA256SUMS
)
```

Record the blocked worktree path/branch and evidence directory in the pause/incident
record. Do not push the WIP branch unless the owner explicitly chooses to. Do not prune
worktrees until Phase 1 assigns an evidence-based disposition.

## Step 1 — verify and install the planning pack

Download together:

```text
uzel-product-incubation-v4-2026-08-10.zip
uzel-product-incubation-v4-2026-08-10.sha256
```

Run from a clean checkout of the real remote PR base. The paused GSD history may remain
on a separate local integration ref; do not use that divergent ref as the planning PR
base merely to make the incident ancestry check pass. Replace paths and refs explicitly.

```bash
set -euo pipefail

pack_dir="/absolute/path/to/downloads"
repo_dir="/absolute/path/to/jodobear/uzel"
pack_name="uzel-product-incubation-v4-2026-08-10"
trusted_pack_sha256="<64-hex SHA-256 from a signed release or authenticated owner handoff>"

(
  cd "$pack_dir"
  printf '%s\n' "$trusted_pack_sha256" | grep -Eq '^[0-9a-f]{64}$'
  test "$(wc -l < "$pack_name.sha256")" -eq 1
  test "$(awk 'NR == 1 { print $1 }' "$pack_name.sha256")" = "$trusted_pack_sha256"
  printf '%s  %s\n' "$trusted_pack_sha256" "$pack_name.zip" | sha256sum -c -
  unzip -tq "$pack_name.zip"
  python3 - "$pack_name.zip" "$pack_name" <<'PY'
import stat
import sys
import zipfile
from pathlib import PurePosixPath

archive, expected_root = sys.argv[1:]
seen = set()
with zipfile.ZipFile(archive) as source:
    for entry in source.infolist():
        name = entry.filename
        path = PurePosixPath(name)
        parts = path.parts
        if (
            not parts
            or path.is_absolute()
            or "\\" in name
            or parts[0] != expected_root
            or any(part in {"", ".", ".."} for part in parts)
        ):
            raise SystemExit(f"unsafe archive path: {name!r}")
        normalized = "/".join(parts)
        if normalized in seen:
            raise SystemExit(f"duplicate archive path: {normalized!r}")
        seen.add(normalized)
        kind = stat.S_IFMT(entry.external_attr >> 16)
        if kind not in {0, stat.S_IFREG, stat.S_IFDIR}:
            raise SystemExit(f"special archive entry rejected: {name!r}")
PY
)

cd "$repo_dir"
git rev-parse --is-inside-work-tree >/dev/null
test -z "$(git status --porcelain)"

pr_base_ref="origin/<real-pr-base-branch>"
git fetch origin
pr_base_head="$(git rev-parse "$pr_base_ref")"
test "$(git rev-parse HEAD)" = "$pr_base_head"
blocked_parent="$(git rev-parse b185ad1^)"
git cat-file -e "$blocked_parent^{commit}"
test -n "$(git merge-base "$blocked_parent" "$pr_base_head")"

plan_branch="plan/uzel-incubation-v4"
if git show-ref --verify --quiet "refs/heads/$plan_branch"; then
  git switch "$plan_branch"
else
  git switch -c "$plan_branch" "$pr_base_head"
fi
test -z "$(git status --porcelain)"
git merge-base --is-ancestor "$pr_base_head" HEAD
prior_paths="$(git diff --name-only "$pr_base_head"...HEAD)"
if [ -n "$prior_paths" ]; then
  printf '%s\n' "$prior_paths" | grep -Ev \
    "^docs/plans/$pack_name/" >/dev/null && {
      printf 'planning branch contains unrelated paths\n' >&2
      exit 1
    }
fi

mkdir -p docs/plans
test ! -e "docs/plans/$pack_name"
unzip -q "$pack_dir/$pack_name.zip" -d docs/plans

(
  cd "docs/plans/$pack_name"
  sha256sum -c SHA256SUMS
  PYTHONDONTWRITEBYTECODE=1 python3 scripts/audit_docs.py .
)

git add "docs/plans/$pack_name"
git diff --cached --check
all_paths="$(git diff --name-only "$pr_base_head"...HEAD; git diff --cached --name-only)"
printf '%s\n' "$all_paths" | sed '/^$/d' | grep -Ev \
  "^docs/plans/$pack_name/" >/dev/null && {
    printf 'planning PR would contain unrelated paths\n' >&2
    exit 1
  }
if ! git diff --cached --quiet; then
  git commit -m "docs: adopt audited Uzel incubation plan v4"
fi
git push -u origin "$plan_branch"
```

Open and merge a planning-only contextual PR. It must contain no product source,
dependency lock, generated build output or `.planning` state change.

If repository instructions require a Graphify refresh after adding the pack auditor,
keep `graphify-out/` out of the planning-only PR. Merge the independently reviewed planning
PR first. Then create an isolated metadata branch from that exact integrated commit, run
`graphify update .`, commit only the durable refreshed `graphify-out/` artifacts and merge
the metadata-only PR. Do not create the Phase 1 worktree until the graph refresh is
integrated; a pre-merge graph must not advertise plan paths absent from its branch tree.

## Step 2 — create a clean manual Phase 1 worktree

After the planning PR is merged, first identify or prepare one clean integration ref that
contains both histories: the preserved paused GSD parent and the exact merged planning
pack. Preparing that ref is an explicit history-reconciliation step, not part of the
planning-only PR. Use a reviewed reconciliation PR when the integration ref is shared;
otherwise use a dedicated local integration branch and record its exact head. Do not
continue from remote `master` alone when it lacks the paused GSD history.

```bash
set -euo pipefail

repo_dir="/absolute/path/to/jodobear/uzel"
integration_ref="<exact-ref-containing-paused-gsd-history-and-merged-pack>"
pack_merge="<exact-merged-planning-pr-commit>"
phase_dir="/absolute/path/to/uzel-phase-1-v4"
phase_branch="phase/01-baseline-v4"

cd "$repo_dir"
git fetch origin
test -z "$(git status --porcelain)"
blocked_parent="$(git rev-parse b185ad1^)"
base_head="$(git rev-parse "$integration_ref")"
git merge-base --is-ancestor "$blocked_parent" "$base_head"
git merge-base --is-ancestor "$pack_merge" "$base_head"
git cat-file -e "$base_head:.planning/PROJECT.md"
git cat-file -e "$base_head:docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md"

if [ -e "$phase_dir" ]; then
  git -C "$phase_dir" rev-parse --is-inside-work-tree >/dev/null
  test "$(git -C "$phase_dir" branch --show-current)" = "$phase_branch"
  test "$(git -C "$phase_dir" rev-parse HEAD)" = "$base_head"
elif git show-ref --verify --quiet "refs/heads/$phase_branch"; then
  test "$(git rev-parse "$phase_branch")" = "$base_head"
  git worktree add "$phase_dir" "$phase_branch"
else
  git worktree add -b "$phase_branch" "$phase_dir" "$base_head"
fi

cd "$phase_dir"
test "$(git branch --show-current)" = "$phase_branch"
test "$(git rev-parse HEAD)" = "$base_head"
test -z "$(git status --porcelain)"
git status --short --branch
```

This worktree is the Phase 1 planning/execution boundary. GSD automatic worktrees remain
disabled under Codex. The historical replay may create a separate disposable checkout.

## Step 3 — inspect and reorient the existing GSD project

Start a fresh top-level Codex session in the new Phase 1 worktree. Do **not** run
`$gsd-resume-work`; it may resume the blocked plan before reconciliation.

Before invoking GSD, prove that the session and shell are rooted in the intended
worktree and record the Codex version:

```bash
set -euo pipefail
expected_branch="phase/01-baseline-v4"
repo_root="$(git rev-parse --show-toplevel)"
test "$(realpath "$repo_root")" = "$(realpath "$PWD")"
test "$(git branch --show-current)" = "$expected_branch"
test -z "$(git status --porcelain)"
managed_codex=/home/at/.bun/bin/codex
managed_codex_sha256=134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477
test -x "$managed_codex"
test "$(sha256sum "$managed_codex" | cut -d ' ' -f1)" = "$managed_codex_sha256"
test "$("$managed_codex" --version 2>&amp;1 | tail -n 1)" = "codex-cli 0.147.0"
```

Do not infer skill discovery or hook support from a remembered Codex version floor.
Record the managed Codex absolute path, version, executable SHA-256 and installed help in
the frozen evidence before invoking it. Record the actual GSD, CodeRabbit, Rust, Node and
Nix versions plus installed-help output; record the exact guide repository/path/revision/date when a minimum version is
cited. Probe required skill discovery and hook behavior directly. Freeze this
orchestration/toolchain profile for Phase 1; do not update it during planning, execution
or review. Current installed help, capability probes and the phase pin remain authoritative
if upstream requirements move later.

Run `$gsd-help --full` and confirm equivalent semantics for:

```text
plan-phase: --ingest, --ingest-format, --reviews; plan checker enabled by default
execute-phase: phase execution
verify-work: post-execution verification
review: --phase, --coderabbit
extract-learnings: phase argument, when present
phase: --insert, --edit
pause-work: --report
```

Online documentation, a development branch and the installed release may differ. Record
whether the pinned installed help exposes a state/plan `--validate` option for
`plan-phase` and/or `execute-phase`, and record its exact semantics. Use that option when
present and appropriate; otherwise run the documented form without it. In both cases,
plan-checker verification remains enabled and `verify-work` remains mandatory. Never use
`--skip-verify` for this programme without an explicit incident decision and equivalent
replacement evidence. If equivalent plan checking, local-CodeRabbit/GitHub-Codex review, state coherence,
execution or post-execution verification is unavailable, stop and create a bounded
compatibility/toolchain issue; update in a separate campaign, restart Codex, record the
new immutable version and repeat preflight.

Run:

```text
$gsd-health
$gsd-progress --forensic
```

Ensure project configuration resolves to:

```bash
node "$HOME/.codex/gsd-core/bin/gsd-tools.cjs" query config-set runtime codex
node "$HOME/.codex/gsd-core/bin/gsd-tools.cjs" query config-set workflow.use_worktrees false
node "$HOME/.codex/gsd-core/bin/gsd-tools.cjs" query config-set workflow.auto_advance false
node "$HOME/.codex/gsd-core/bin/gsd-tools.cjs" query config-set workflow._auto_chain_active false
```

Read back all four values from `.planning/config.json`; stop unless runtime is `codex`
and the three workflow booleans are false.

Print and paste the complete reorientation prompt:

```bash
cat docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/01-reorient-current-gsd.md
```

This planning-only step must:

- preserve the brownfield project and blocked incident;
- preserve integer phases 2–7;
- insert the exact decimal phase sequence in `03-ROADMAP.md` using current supported GSD
  phase-management commands;
- remove stale package-first and automatic post-M5 work;
- make 7.9 the candidate-freeze phase and A5 the hard stop;
- avoid product source/dependency changes.

Expected future sequence:

```text
2, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7,
3, 3.1, 3.2, 3.3,
4, 4.1, 4.2, 4.3,
5, 5.1, 5.2, 5.3,
6, 6.1, 6.2,
7, 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 7.7, 7.8, 7.9
```

Then run `$gsd-progress --forensic` again and inspect `.planning/ROADMAP.md`, `STATE.md`
and phase directories. Do not proceed if numbering is missing, duplicated, orphaned or
renumbered. Do not run new-project, new-milestone, onboarding or another full codebase
map unless `$gsd-health` proves corruption and a separate bounded repair is approved.

## Step 4 — replan Phase 1 in place

`00-GSD-INGEST.md` remains the programme authority. GSD 1.10.0's narrative parser does
not recognize its programme-specific headings and returns zero locked decisions. Use the
checksum-bound `00-GSD-INGEST-ADR.md` projection for the parser express path; independently
review that projection against the authoritative ingest whenever either file changes.

Run exactly one form, selected from the recorded installed help:

```text
# preferred when this installed version documents state/plan validation
$gsd-plan-phase 1 --ingest docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md --ingest-format narrative --validate

# otherwise
$gsd-plan-phase 1 --ingest docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md --ingest-format narrative
```

Do not run both forms or infer flag semantics from another release/branch.

Plan-checker verification must remain enabled; do not use `--skip-verify`. The plans must
preserve Phase 1 numbering, reconcile
`b185ad1`, separate exact-source replay from current package/native acceptance, classify
Vite/conformance-tool origins before materialization, enforce the bounded replay attempt
rule, and include the human stop before Phase 2, and create the ecosystem/upstream registry,
initial compatibility profile, manifest/exact-build identity interpretation, capability
ledgers and decision/learning structure required by plan `01-05`.

## Step 5 — independently review and converge Phase 1 plans

Print and paste:

```bash
cat docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/02-review-phase-1.md
```

Run the CodeRabbit diff review:

```text
$gsd-review --phase 1 --coderabbit
```

Commit the complete plan-review candidate first and record its SHA. Run CodeRabbit locally
over the immutable committed plan diff ending at that SHA, then push/open the plan PR
without changing it and ask GitHub Codex to review the same exact PR SHA against the complete
`prompts/02-review-phase-1.md`. Record CodeRabbit CLI/version/result, GitHub Codex review
identity, base/head SHAs, prompt digest, commands/requests and full findings. If
CodeRabbit returns `rate_limit` before findings, record that output and continue to
GitHub Codex; green GitHub Codex satisfies the gate. No other CodeRabbit failure enables
fallback. Do not use Claude, OpenCode, remote CodeRabbit or local Codex self-review.

When a Critical/High or blocking Medium finding exists, or an accepted finding changes
plan semantics, apply the batch, rerun plan/audit gates, commit a new complete candidate,
and restart both approved reviewer stages on that new SHA:

```text
$gsd-plan-phase 1 --reviews
$gsd-review --phase 1 --coderabbit
```

Use no more than three cycles. Do not execute with any Critical/High finding or a
Medium finding that threatens the phase outcome, authority, correctness, data integrity,
security or operability. Every remaining non-blocking Medium/Low finding needs an
explicit disposition and bounded rationale. Stop on GitHub Codex failure or on any
CodeRabbit failure other than recorded rate limiting, source/lock mutation, artifact-only
replay substitution, automatic Codex worktree assumption or product feature work in
Phase 1.

Reviewer prompts/fixtures must contain no keys, pairing URIs, credentials, production
content or unredacted private diagnostics.

## Step 6 — execute and verify only Phase 1

Print and paste:

```bash
cat docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/03-execute-phase-1.md
```

Run exactly one form, selected from the same pinned installed help:

```text
# when this installed version documents the intended execution validation
$gsd-execute-phase 1 --validate

# otherwise
$gsd-execute-phase 1
```

Do not run both forms. The following `verify-work` gate is required either way.

Use `--wave N` only for actual planned waves and execute them sequentially. Afterward:

```text
$gsd-verify-work 1
$gsd-progress --forensic
```

Complete Plan `01-06` first: commit the immutable evidence set, exact manifest, and public
projection as E under ordinary task-atomic commits. Plan `01-07` then generates the complete
fail-closed transition packet against exact E and commits only that packet as P. A human may
select `hold-phase-1` immediately after P validates. Before `approve-phase-2` may be offered,
attempt local CodeRabbit on exact P, push P unchanged, and require green GitHub Codex on P.
Recorded CodeRabbit `rate_limit` before findings activates the same approved fallback as
plan review. Any finding-driven commit or change to E, packet inputs, packet, profile,
indexes or evidence restarts E/P generation and review.

The human may inspect the evidence and choose hold without waiting for external review.
Only after the evidence-candidate review is green may the human choose approve. Present
separate verdicts for historical replay, current-source
replacement invariants, current Nix/native baseline plus any `not_yet_packaged` Phase 2
gate, `b185ad1`, authority/schema/threat
baseline, the exact compatibility profile and manifest/build-identity interpretation,
upstream/local-patch/maturity/knowledge baselines, CI/review measurements and
unresolved/retired claims. Do not cross this gate
with an automatic next command.

After the human decision record and `01-07-SUMMARY.md` are committed, attempt local
CodeRabbit again, push the new exact SHA, and require green GitHub Codex before merge or
before an `approve-phase-2` decision becomes effective. This final review may not mutate
the already reviewed evidence paths; any such mutation invalidates transition eligibility
and returns to the evidence-candidate review above.

## Step 7 — execute one bounded delivery phase at a time

For each listed phase from `2` through `7.9`:

1. create one contextual issue with one primary outcome;
2. create one clean manual issue branch/worktree from the integrated base;
3. run the read-only upstream radar when the phase touches an upstream-owned surface;
4. discuss and plan only that integer or decimal phase;
5. independently review and replan until no Critical/High or blocking Medium finding
   remains and the rest have explicit dispositions;
6. execute its plans sequentially;
7. run relevant local, package, security, UI, accessibility and compatibility gates;
8. run `$gsd-verify-work <phase>` and then `$gsd-extract-learnings <phase>` when the
   phase-pinned GSD version supports it; treat extraction as raw candidate evidence;
9. print/paste `prompts/05-phase-closeout.md`, curate extracted items and reconcile
   decision/profile/negotiation/upstream/learning/education/visibility deltas and update
   capability ledgers;
10. for milestone endpoints `2.7`, `3.3`, `4.3`, `5.3`, `6.2` and `7.9`, print/paste
    `prompts/07-milestone-learning.md` and create the bounded milestone learning digest;
11. rerun affected checks and perform the serial final PR review on the final SHA including
    source, tests, profile/registry, closeout and learning changes;
12. rerun `$gsd-verify-work <phase>` when closeout changed executable or acceptance
    evidence;
13. merge its one primary PR before creating the next phase worktree.

Typical shape after recording whether this phase pin supports the optional validation
flag:

```text
$gsd-discuss-phase 2
$gsd-plan-phase 2
$gsd-review --phase 2 --coderabbit
$gsd-plan-phase 2 --reviews  # only when blocking findings exist
$gsd-execute-phase 2
$gsd-verify-work 2
$gsd-extract-learnings 2  # when supported; raw candidate evidence
# curate closeout, final-SHA review, rerun affected verification, then merge
```

Append `--validate` only to the planning/execution commands for which the pinned installed
help defines the intended validation semantics; do not paste bracketed pseudo-arguments or
run both variants.

Use the exact phase section in `03-ROADMAP.md`. Never combine multiple listed phases in
one primary PR or pull parked capabilities into context without an explicit roadmap
amendment and local-CodeRabbit/GitHub-Codex review.

Before planning any upstream-owned surface, follow `07-ECOSYSTEM-UPSTREAM.md`: verify the
immutable current pin/profile, inspect the radar delta and decide whether a compatibility
campaign is required. For a material public contribution, print/paste
`prompts/06-upstream-contribution.md`, use a dedicated upstream fork/worktree/branch and
obtain human review before submission. An upstream merge never permits silent Uzel
adoption or local-patch removal.

## Step 8 — mandatory stop after delivery phase 7.9

Freeze the exact candidate and collect GSD evidence:

```text
$gsd-verify-work 7.9
$gsd-code-review 7.9 --depth=deep
$gsd-secure-phase 7.9
$gsd-ui-review 7.9
$gsd-validate-phase 7.9
$gsd-audit-uat
$gsd-progress --forensic
$gsd-milestone-summary
$gsd-audit-milestone
```

Print and paste:

```bash
cat docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/04-post-m5-audit.md
```

Follow the twelve-lane `05-POST-M5-AUDIT.md`. Do not run:

```text
$gsd-complete-milestone
```

until A5 passes, blocking remediation is re-audited and the owner explicitly approves
milestone completion and the next programme.

## Stop/recovery conditions

Pause and inspect rather than papering over state when the process loses incident
history, renumbers the project, cannot identify the integration base, touches the
blocked worktree unexpectedly, uses flags not confirmed by installed help, enables
GSD automatic Codex worktrees, cannot run GitHub Codex after a CodeRabbit pass or recorded rate limit, mutates
source/dependencies in a
planning-only step, leaves roadmap/state inconsistent or advances past the Phase 1 human
gate.

Preserve the current branch/worktree, run `$gsd-pause-work --report`, then inspect
`$gsd-health` and `$gsd-progress --forensic`. Repair only the smallest proven state issue.
