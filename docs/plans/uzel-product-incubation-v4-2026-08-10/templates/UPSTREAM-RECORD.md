# UPR-#### — <upstream interaction>

- **State:** observed | reproduced | existing_thread | commented | issue_open | local_patch | pr_open | accepted | merged | released | adopted | patch_removed | rejected | superseded | withdrawn
- **Visibility:** public | internal | embargoed
- **Upstream:**
- **Exact Uzel pin:**
- **Current upstream revision tested:**
- **Candidate-next probe:** not_run | green_advisory | changed | blocked_high_risk | failed_harness
- **Candidate-next evidence:** <path/digest or none>
- **Uzel issue/PR:**
- **Public/private upstream link:**
- **Owner:**
- **Disclosure constraint:** none | <reason and release condition>
- **Upstream fork/worktree/branch:**
- **Contribution requirements checked:** CONTRIBUTING | SECURITY | license | AI policy | authorship/signoff | DCO/CLA | tests/style
- **Named human submitter/approver:**
- **AI assistance and required disclosure:** none | tools/use/disclosure
- **Authorship/signoff/DCO/CLA disposition:**
- **Public text/patch human reviewer:**

## Lifecycle events

Append-only. Never rewrite an earlier transition.

| State transition | Timestamp | Exact upstream revision/pin | Actor | Evidence |
|---|---|---|---|---|
| observed | | | | |

## Problem and impact

Minimal general problem, affected capability/profile, user/security consequence.

## Reproduction

When the Disclosure constraint permits publication, provide a synthetic public reproducer
and results at the Uzel pin/current head. For internal or embargoed records, provide a
non-public evidence reference or record why reproduction is deferred; never expose restricted
upstream details merely to satisfy this field.

## Channel decision

Why comment, issue, PR, private disclosure or no upstream action is correct.

## Uzel workaround or patch

Exact patch, tests, owner, expiry/removal trigger and risk. A reviewed local patch may
unblock Uzel before upstream resolution; it never implies upstream acceptance.

## Upstream response

Maintainer feedback, accepted semantics and resulting revision/release.

## Adoption and patch removal

Record separate events for upstream merge, release, Uzel adoption at an exact pin and
local-patch removal. A merge is not a release, adoption or proof that the workaround is
now removable. `patch_removed` is allowed only after append-only `released` and `adopted`
events cite exact revisions/pins and evidence.
