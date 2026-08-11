# Prompt — synthesize one bounded milestone learning digest

Run only at the final delivery phase of a milestone, after phase closeout evidence is
prepared and before final remote review. Do not add product scope and do not write a
polished course.

Read `08-DECISIONS-LEARNING.md`, the milestone phase summaries/closeouts, accepted ADRs,
Spec Interpretation Records, Upstream Records, Learning Notes, capability ledgers,
compatibility profile, canonical terminology registry, package/native reports and changed source/tests/schemas.

Create or update the milestone digest using `templates/MILESTONE-LEARNING.md`:

1. identify the exact source range, phase-pinned toolchain and compatibility-profile hash
   scheme/hash plus compatibility-kit manifest;
2. state only what the milestone actually proved;
3. extract durable decisions, non-obvious failure modes and negative results;
4. summarize upstream movement, contribution state and local-patch/adoption status;
5. produce a bounded human case-study outline and agent-reference delta using canonical term IDs;
6. classify each claim as fact, measured result, interpretation or Uzel policy;
7. set `public`, `internal` or `embargoed` visibility and remove private data, secrets and
   undisclosed security detail from public candidates;
8. link every non-elementary claim to an exact executable/inspectable fixture, vector,
   test, trace or measured report plus source/profile/decision/upstream evidence;
9. reconcile raw GSD extraction dispositions and regenerate internal/public knowledge
   indexes with visibility-leak checks;
10. mark contradictions or stale claims as blockers rather than resolving them through
    prose;
11. add only evidence-backed educational seeds.

Output exact files changed, evidence links, disclosure status, contradictions and any
`needs_review` trigger. Keep the digest small enough for later humans and agents to use.
