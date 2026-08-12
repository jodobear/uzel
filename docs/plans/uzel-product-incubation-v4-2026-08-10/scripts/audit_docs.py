#!/usr/bin/env python3
"""Structural and policy audit for the Uzel revision-4 plan pack.

MANIFEST.json and SHA256SUMS cover every payload file. They intentionally exclude
MANIFEST.json, SHA256SUMS and reports/audit.json to avoid self-referential hashes.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import tomllib
from pathlib import Path
from typing import Any

PACK_NAME = "uzel-product-incubation-v4-2026-08-10"
REVISION = 4
SOURCE_INGEST_SHA256 = "098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d"
EXPECTED_PROJECTION_DECISIONS = [f"D-{index:02d}" for index in range(1, 32)]

PAYLOAD_FILES = {
    "README.md",
    "00-GSD-INGEST.md",
    "00-GSD-INGEST-ADR.md",
    "01-BASELINE-REPLAY.md",
    "02-PRODUCT-ARCHITECTURE.md",
    "03-ROADMAP.md",
    "04-DELIVERY-QUALITY.md",
    "05-POST-M5-AUDIT.md",
    "06-START-RUNBOOK.md",
    "07-ECOSYSTEM-UPSTREAM.md",
    "08-DECISIONS-LEARNING.md",
    "09-PRODUCTION-MATURITY.md",
    "AUDIT.md",
    "prompts/01-reorient-current-gsd.md",
    "prompts/02-review-phase-1.md",
    "prompts/03-execute-phase-1.md",
    "prompts/04-post-m5-audit.md",
    "prompts/05-phase-closeout.md",
    "prompts/06-upstream-contribution.md",
    "prompts/07-milestone-learning.md",
    "templates/ADR.md",
    "templates/CAPABILITY-LEDGER.md",
    "templates/COMPAT-PROFILE.md",
    "templates/COMPAT-PROFILE.toml",
    "templates/LEARNING-NOTE.md",
    "templates/MILESTONE-LEARNING.md",
    "templates/PHASE-CLOSEOUT.md",
    "templates/PROFILE-TRANSITION.toml",
    "templates/SPEC-INTERPRETATION.md",
    "templates/TERM-REGISTRY.toml",
    "templates/UPSTREAM-RECORD.md",
    "templates/UPSTREAM-REGISTRY.toml",
    "reports/ecosystem-baseline-2026-08-10.md",
    "scripts/audit_docs.py",
}
META_FILES = {"MANIFEST.json", "SHA256SUMS", "reports/audit.json"}
EXPECTED_FILES = PAYLOAD_FILES | META_FILES
HASH_EXCLUSIONS = ["MANIFEST.json", "SHA256SUMS", "reports/audit.json"]

# Operational Markdown is checked for stale execution policy. Templates and the audit
# describe examples/history and are checked by their own invariants instead.
OPERATIONAL_MD = {
    rel
    for rel in PAYLOAD_FILES
    if rel.endswith(".md")
    and not rel.startswith("templates/")
    and rel != "AUDIT.md"
}

WORD_RE = re.compile(r"\b[\w'’-]+\b", re.UNICODE)
MD_LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
PLACEHOLDER_RE = re.compile(r"\b(?:TODO|TBD|FIXME|XXX)\b|<INSERT[^>]*>", re.I)
LANE_RE = re.compile(r"^## Lane (\d+)\b", re.M)
DELIVERY_RE = re.compile(r"^### Phase (\d+(?:\.\d+)?)\b", re.M)

EXPECTED_PHASES = [
    "2", "2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7",
    "3", "3.1", "3.2", "3.3",
    "4", "4.1", "4.2", "4.3",
    "5", "5.1", "5.2", "5.3",
    "6", "6.1", "6.2",
    "7", "7.1", "7.2", "7.3", "7.4", "7.5", "7.6", "7.7", "7.8", "7.9",
]

REQUIRED_TEXT: dict[str, list[str]] = {
    "00-GSD-INGEST-ADR.md": [
        "not an independent authority",
        "098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d",
        "D-01",
        "D-31",
        "not_yet_packaged",
        "GitHub Codex reviews the exact pushed PR SHA",
    ],
    "README.md": [
        "production candidate",
        "A5 attacks that exact candidate through twelve audit lanes",
        "sha256-exact-utf8-bytes-v1",
        "candidate-next shadow probe",
        "compatibility/conformance kit",
        "clean-room black-box fixture",
        "phase-pinned GSD/Codex/CodeRabbit/toolchain profile",
        "core L4 runtime-composability claim",
        "every enabled core capability at maturity L4",
        "teach humans and agents later",
        "per-principal admission",
        "canonical terminology",
        "independent critical-boundary security review",
        "no-silent-update",
        "no remote telemetry by default",
    ],
    "00-GSD-INGEST.md": [
        "Track A — hermetic exact-source replay",
        "Track B — current Nix package/native acceptance",
        "ecosystem, compatibility, maturity and knowledge baseline",
        "Uzel Runtime Compatibility Profile",
        "upstream registry",
        "capability-maturity ledger",
        "knowledge/education lanes",
        "canonical terminology registry",
        "admission/fairness baseline",
        "mandatory stop after delivery phase 7.9",
    ],
    "01-BASELINE-REPLAY.md": [
        "Materializing an exact fixed closure is allowed",
        "dependency lifecycle scripts are disabled by default",
        "Replay attempt budget",
        "unavailable_exact_closure",
        "Ecosystem, compatibility and maturity baseline",
        "candidate-next",
        "externally consumable compatibility",
        "phase-pinned",
    ],
    "02-PRODUCT-ARCHITECTURE.md": [
        "sha256-exact-utf8-bytes-v1",
        "sha256-canonical-cbor-v1",
        "reject before guest code",
        "no mid-session profile",
        "Supported guest trust tiers",
        "unsupported_arbitrary",
        "before any relay write",
        "before any upload body",
        "Machine-enforced architecture boundary checks",
        "externally consumable compatibility kit",
        "Admission, fairness and abuse resistance",
        "protected credential backend",
    ],
    "03-ROADMAP.md": [
        "M1 / GSD delivery phases 2, 2.1–2.7",
        "Phase 2.7 — external clean-room compatibility and composition capstone",
        "M5 / GSD delivery phases 7, 7.1–7.9 — production-candidate hardening",
        "required independent clean-room napplet",
        "blocked_no_independent_peer",
        "two clean rebuilds",
        "candidate ready for A5",
        "twelve-lane whole-system audit",
        "anti-starvation",
        "rotation",
        "independent critical-boundary security-review brief",
        "candidate/canary/stable",
    ],
    "04-DELIVERY-QUALITY.md": [
        "Phase-pinned orchestration toolchain",
        "$gsd-plan-phase N --reviews",
        "$gsd-execute-phase N",
        "$gsd-verify-work N",
        "$gsd-review --phase N --coderabbit",
        "Candidate-next shadow lane",
        "machine-enforced architecture boundary checker",
        "Interoperability and version-skew tests",
        "Supply-chain and release-evidence tests",
        "Phase learning extraction and curation",
        "docs/knowledge/index.internal.json",
        "canonical term IDs",
        "named human submitter",
    ],
    "05-POST-M5-AUDIT.md": [
        "twelve-lane",
        "Lane 11 — ecosystem compatibility and upstream stewardship",
        "Lane 12 — knowledge integrity and educational readiness",
        "pass_for_human_decision",
        "compatibility/conformance kit",
        "two clean exact-input candidate builds",
        "release-signing",
        "knowledge indexes",
        "an extraction-readiness lane and may not propose",
        "canonical terminology registry",
        "anti-starvation",
        "independent human critical-boundary security review",
        "opt-in canary",
    ],
    "06-START-RUNBOOK.md": [
        "$gsd-help --full",
        "Probe required skill discovery and hook behavior directly",
        "$gsd-plan-phase 1 --ingest docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md",
        "$gsd-review --phase 1 --coderabbit",
        "$gsd-plan-phase 1 --reviews",
        "$gsd-execute-phase 1",
        "$gsd-verify-work 1",
        "$gsd-extract-learnings",
        "workflow.use_worktrees false",
        "prompts/06-upstream-contribution.md",
        "prompts/07-milestone-learning.md",
        "selected from the recorded installed help",
    ],
    "07-ECOSYSTEM-UPSTREAM.md": [
        "Pin execution; observe movement; adopt through evidence",
        "Claim-specific authority",
        "sha256-exact-utf8-bytes-v1",
        "sha256-canonical-cbor-v1",
        "Externally consumable compatibility and conformance kit",
        "Machine-readable upstream registry",
        "Read-only upstream radar",
        "Candidate-next shadow probe",
        "Compatibility campaign",
        "Upstream contribution workflow",
        "dedicated upstream fork/worktree/branch",
        "commented",
        "issue_open",
        "pr_open",
        "released",
        "adopted",
        "patch_removed",
        "blocked_no_independent_peer",
        "AI-assisted-contribution",
        "named human submitter",
    ],
    "08-DECISIONS-LEARNING.md": [
        "Architecture Decision Records",
        "Spec Interpretation Records",
        "Upstream Interaction Records",
        "Learning Notes",
        "Capability ledgers",
        "Phase closeout",
        "GSD extraction is intake, not authority",
        "docs/knowledge/index.internal.json",
        "docs/knowledge/index.public.json",
        "verified_against",
        "last_reviewed",
        "Human-facing learning unit",
        "Agent-facing reference",
        "docs/knowledge/terms.toml",
        "canonical term IDs",
        "executable witness",
        "A5 knowledge handoff",
    ],
    "09-PRODUCTION-MATURITY.md": [
        "L4 — production candidate",
        "L5 — production approved",
        "Capability maturity ledger",
        "Composability definition",
        "blocked_no_independent_peer",
        "Guest-code trust tiers",
        "unsupported_arbitrary",
        "two clean exact-input builds",
        "release/provenance signing-key fingerprint",
        "M5 output is `candidate ready for A5`, never `production`",
        "anti-starvation",
        "client-key generation/import",
        "independent security reviewer",
        "candidate/canary/stable",
        "no-silent-update",
    ],
}

STALE_OPERATIONAL = [
    "uzel-product-incubation-v3-2026-08-10",
    "gsd-build/gsd-2",
    "blocked_no_peer",
    "product-alpha",
    "ten-lane",
    "ten lanes",
    "source-of-truth hierarchy",
    "upstreams.yaml",
]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def first_symlink_component(path: Path) -> Path | None:
    """Return the first existing symlink in an absolute output path."""
    absolute = path.absolute()
    current = Path(absolute.anchor)
    for part in absolute.parts[1:]:
        current /= part
        if current.is_symlink():
            return current
    return None


def rel_files(root: Path) -> set[str]:
    return {
        str(path.relative_to(root))
        for path in root.rglob("*")
        if path.is_file() and not path.is_symlink()
    }


def audit_pack_symlinks(root: Path, errors: list[str]) -> None:
    for path in sorted(root.rglob("*")):
        if path.is_symlink():
            errors.append(f"archive contains symlink: {path.relative_to(root)}")


def clean_target(raw: str) -> str:
    value = raw.strip()
    if value.startswith("<") and value.endswith(">"):
        value = value[1:-1]
    return re.split(r"\s+[\"']", value, maxsplit=1)[0]


def audit_links(root: Path, path: Path, text: str) -> list[str]:
    errors: list[str] = []
    for raw in MD_LINK_RE.findall(text):
        target = clean_target(raw)
        if not target or target.startswith(("#", "http://", "https://", "mailto:", "data:")):
            continue
        relative = target.split("#", 1)[0]
        if not relative:
            continue
        resolved = (path.parent / relative).resolve()
        try:
            resolved.relative_to(root.resolve())
        except ValueError:
            errors.append(f"{path.relative_to(root)}: relative link leaves pack: {target}")
            continue
        if not resolved.exists():
            errors.append(f"{path.relative_to(root)}: broken relative link: {target}")
    return errors


def audit_fences(rel: str, text: str) -> tuple[list[str], int]:
    errors: list[str] = []
    opened: tuple[int, str] | None = None
    mermaids = 0
    for line_no, line in enumerate(text.splitlines(), 1):
        candidate = line.lstrip(" ")
        if len(line) - len(candidate) > 3 or not candidate.startswith("```"):
            continue
        info = candidate[3:].strip()
        if opened is None:
            opened = (line_no, info)
            if info == "mermaid":
                mermaids += 1
        else:
            opened = None
    if opened:
        errors.append(f"{rel}: unclosed fence opened at line {opened[0]}")
    return errors, mermaids


def load_manifest(root: Path, errors: list[str]) -> dict[str, Any]:
    try:
        data = json.loads((root / "MANIFEST.json").read_text(encoding="utf-8"))
    except Exception as exc:  # noqa: BLE001
        errors.append(f"MANIFEST.json: cannot parse: {exc}")
        return {}
    if not isinstance(data, dict):
        errors.append("MANIFEST.json: root must be a JSON object")
        return {}
    if data.get("name") != PACK_NAME or data.get("revision") != REVISION:
        errors.append("MANIFEST.json: wrong pack name or revision")
    return data


def audit_hashes(root: Path, errors: list[str]) -> None:
    data = load_manifest(root, errors)
    if data.get("excluded_from_payload_hashes") != HASH_EXCLUSIONS:
        errors.append(
            "MANIFEST.json: excluded_from_payload_hashes must identify the exact "
            "self-referential/generated metadata exclusions"
        )
    entries = data.get("files", []) if isinstance(data, dict) else []
    mapped: dict[str, dict[str, Any]] = {}
    for entry in entries:
        if not isinstance(entry, dict) or not isinstance(entry.get("path"), str):
            errors.append("MANIFEST.json: invalid file entry")
            continue
        if entry["path"] in mapped:
            errors.append(f"MANIFEST.json: duplicate entry: {entry['path']}")
        mapped[entry["path"]] = entry
    if set(mapped) != PAYLOAD_FILES:
        errors.append(
            f"MANIFEST.json payload mismatch; missing={sorted(PAYLOAD_FILES-set(mapped))}, "
            f"extra={sorted(set(mapped)-PAYLOAD_FILES)}"
        )
    for rel in sorted(PAYLOAD_FILES & set(mapped)):
        path = root / rel
        if not path.is_file() or path.is_symlink():
            continue
        if mapped[rel].get("sha256") != sha256(path):
            errors.append(f"MANIFEST.json: hash mismatch: {rel}")
        if mapped[rel].get("bytes") != path.stat().st_size:
            errors.append(f"MANIFEST.json: byte count mismatch: {rel}")

    parsed: dict[str, str] = {}
    try:
        lines = (root / "SHA256SUMS").read_text(encoding="utf-8").splitlines()
    except Exception as exc:  # noqa: BLE001
        errors.append(f"SHA256SUMS: cannot read: {exc}")
        return
    for line_no, line in enumerate(lines, 1):
        if not line:
            continue
        match = re.fullmatch(r"([0-9a-f]{64})  (.+)", line)
        if not match:
            errors.append(f"SHA256SUMS:{line_no}: malformed line")
            continue
        digest, rel = match.groups()
        if rel in parsed:
            errors.append(f"SHA256SUMS: duplicate entry: {rel}")
        parsed[rel] = digest
    if set(parsed) != PAYLOAD_FILES:
        errors.append(
            f"SHA256SUMS payload mismatch; missing={sorted(PAYLOAD_FILES-set(parsed))}, "
            f"extra={sorted(set(parsed)-PAYLOAD_FILES)}"
        )
    for rel in sorted(PAYLOAD_FILES & set(parsed)):
        path = root / rel
        if path.is_file() and not path.is_symlink() and parsed[rel] != sha256(path):
            errors.append(f"SHA256SUMS: hash mismatch: {rel}")


def audit_toml(root: Path, errors: list[str]) -> None:
    for rel in (
        "templates/COMPAT-PROFILE.toml",
        "templates/PROFILE-TRANSITION.toml",
        "templates/TERM-REGISTRY.toml",
        "templates/UPSTREAM-REGISTRY.toml",
    ):
        try:
            path = root / rel
            if path.is_symlink():
                continue
            with path.open("rb") as handle:
                data = tomllib.load(handle)
        except Exception as exc:  # noqa: BLE001
            errors.append(f"{rel}: invalid TOML: {exc}")
            continue
        if data.get("schema_version") != 1:
            errors.append(f"{rel}: schema_version must be 1")


def audit_gsd_ingest_projection(root: Path, errors: list[str]) -> None:
    source_path = root / "00-GSD-INGEST.md"
    projection_path = root / "00-GSD-INGEST-ADR.md"
    source_digest = sha256(source_path)
    if source_digest != SOURCE_INGEST_SHA256:
        errors.append(
            "00-GSD-INGEST.md: authoritative digest changed; independently re-audit "
            "before updating the parser projection"
        )

    text = projection_path.read_text(encoding="utf-8")
    headings = list(re.finditer(r"^##\s+(.+?)\s*$", text, re.M))
    sections: dict[str, str] = {}
    for index, heading in enumerate(headings):
        start = heading.end()
        end = headings[index + 1].start() if index + 1 < len(headings) else len(text)
        sections[heading.group(1).strip().lower()] = text[start:end].strip()

    required_sections = [
        "status",
        "context",
        "decisions",
        "out of scope",
        "deferred",
        "dependencies",
        "implementation plan",
        "success criteria",
        "risks",
    ]
    for section in required_sections:
        if not sections.get(section):
            errors.append(f"00-GSD-INGEST-ADR.md: missing/nonempty parser section: {section}")

    if not sections.get("status", "").lower().startswith("accepted"):
        errors.append("00-GSD-INGEST-ADR.md: parser status must be Accepted")

    decision_ids = re.findall(
        r"^-\s+(D-\d{2})\s+—\s+\S", sections.get("decisions", ""), re.M
    )
    if decision_ids != EXPECTED_PROJECTION_DECISIONS:
        errors.append(
            "00-GSD-INGEST-ADR.md: decision sequence mismatch; "
            f"expected {EXPECTED_PROJECTION_DECISIONS}, found {decision_ids}"
        )

    bound = re.search(
        r"Source SHA-256:\s*`([0-9a-f]{64})`", sections.get("context", "")
    )
    bound_digest = bound.group(1) if bound else ""
    if bound_digest != SOURCE_INGEST_SHA256 or bound_digest != source_digest:
        errors.append(
            "00-GSD-INGEST-ADR.md: source digest binding does not match immutable authority"
        )


def audit_policy(root: Path, errors: list[str]) -> None:
    operational = "\n".join((root / rel).read_text(encoding="utf-8") for rel in sorted(OPERATIONAL_MD))
    for phrase in STALE_OPERATIONAL:
        if phrase in operational:
            errors.append(f"operational documents contain stale invariant: {phrase}")

    # No active implementation or extraction dependency on the planning-only companion
    # repository. Match the standalone historical repository name, not "napplet".
    if re.search(r"\bNapp\b", operational, re.I):
        errors.append("operational documents reference the inactive planning-only repository")

    roadmap = (root / "03-ROADMAP.md").read_text(encoding="utf-8")
    actual = DELIVERY_RE.findall(roadmap)
    if actual != EXPECTED_PHASES:
        errors.append(f"03-ROADMAP.md: phase sequence mismatch: {actual}")

    audit_text = (root / "05-POST-M5-AUDIT.md").read_text(encoding="utf-8")
    lanes = [int(value) for value in LANE_RE.findall(audit_text)]
    if lanes != list(range(1, 13)):
        errors.append(f"05-POST-M5-AUDIT.md: expected lanes 1..12, found {lanes}")
    if "Lane 11 — ecosystem compatibility and upstream stewardship" not in audit_text:
        errors.append("05-POST-M5-AUDIT.md: Lane 11 is not the ecosystem/upstream lane")
    if "Lane 12 — knowledge integrity and educational readiness" not in audit_text:
        errors.append("05-POST-M5-AUDIT.md: Lane 12 is not the knowledge/education lane")
    if re.search(r"^## Lane \d+.*extraction", audit_text, re.I | re.M):
        errors.append("05-POST-M5-AUDIT.md: contains a premature extraction audit lane")

    runbook = (root / "06-START-RUNBOOK.md").read_text(encoding="utf-8")
    for prompt_num in range(1, 8):
        prefix = f"prompts/{prompt_num:02d}-"
        if prefix not in runbook:
            errors.append(f"06-START-RUNBOOK.md: missing prompt reference {prompt_num:02d}")

    # Exact-profile and compatibility invariants must appear across architecture and
    # governance, not only in one overview.
    joined = "\n".join(
        (root / rel).read_text(encoding="utf-8")
        for rel in (
            "02-PRODUCT-ARCHITECTURE.md",
            "04-DELIVERY-QUALITY.md",
            "05-POST-M5-AUDIT.md",
            "07-ECOSYSTEM-UPSTREAM.md",
            "09-PRODUCTION-MATURITY.md",
        )
    )
    cross_cutting = [
        "sha256-exact-utf8-bytes-v1",
        "sha256-canonical-cbor-v1",
        "candidate-next",
        "compatibility/conformance kit",
        "clean-room",
        "blocked_no_independent_peer",
        "architecture-boundary",
        "two clean exact-input builds",
        "release-signing",
        "per-principal",
        "anti-starvation",
        "canonical terminology",
        "AI assistance",
        "independent critical-boundary security",
        "no-silent-update",
        "opt-in canary",
        "profile-transition",
        "templates/PROFILE-TRANSITION.toml",
        "procedural independence",
    ]
    for phrase in cross_cutting:
        if phrase.casefold() not in joined.casefold():
            errors.append(f"cross-cutting production invariant missing: {phrase}")

    upstream = (root / "07-ECOSYSTEM-UPSTREAM.md").read_text(encoding="utf-8")
    upstream_required = [
        "commented", "issue_open", "pr_open", "merged", "released", "adopted", "patch_removed",
        "CONTRIBUTING", "SECURITY", "DCO/CLA", "dedicated upstream fork/worktree/branch",
        "AI-assisted-contribution", "named human submitter", "authorship/signoff",
    ]
    for phrase in upstream_required:
        if phrase not in upstream:
            errors.append(f"07-ECOSYSTEM-UPSTREAM.md: missing upstream lifecycle/control: {phrase}")

    learning = (root / "08-DECISIONS-LEARNING.md").read_text(encoding="utf-8")
    learning_required = [
        "GSD extraction is intake, not authority",
        "docs/knowledge/index.internal.json",
        "docs/knowledge/index.public.json",
        "verified_against",
        "last_reviewed",
        "public", "internal", "embargoed",
        "Human-facing learning unit",
        "Agent-facing reference",
        "docs/knowledge/terms.toml", "canonical term IDs", "executable witness",
    ]
    for phrase in learning_required:
        if phrase not in learning:
            errors.append(f"08-DECISIONS-LEARNING.md: missing learning/visibility invariant: {phrase}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--json", dest="json_path")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    if not root.is_dir():
        print(f"error: not a directory: {root}", file=sys.stderr)
        return 2

    raw_output = Path(args.json_path) if args.json_path else root / "reports/audit.json"
    output = raw_output.absolute()
    symlink = first_symlink_component(output)
    if symlink is not None:
        print(f"error: audit report path contains symlink: {symlink}", file=sys.stderr)
        return 2

    canonical_output = root / "reports/audit.json"
    try:
        output.relative_to(root)
        output_is_inside_root = True
    except ValueError:
        output_is_inside_root = False
    if output_is_inside_root and output != canonical_output:
        print(
            "error: --json inside the audited pack must target reports/audit.json",
            file=sys.stderr,
        )
        return 2

    errors: list[str] = []
    warnings: list[str] = []
    audit_pack_symlinks(root, errors)
    actual = rel_files(root)
    payload_complete = PAYLOAD_FILES <= actual
    if actual != EXPECTED_FILES:
        errors.append(
            f"archive allowlist mismatch; missing={sorted(EXPECTED_FILES-actual)}, "
            f"extra={sorted(actual-EXPECTED_FILES)}"
        )

    audit_hashes(root, errors)
    audit_gsd_ingest_projection(root, errors)
    audit_toml(root, errors)

    md_files = sorted(path for path in root.rglob("*.md") if not path.is_symlink())
    total_words = 0
    mermaid_count = 0
    rows: list[dict[str, Any]] = []
    for path in md_files:
        rel = str(path.relative_to(root))
        text = path.read_text(encoding="utf-8")
        words = len(WORD_RE.findall(text))
        total_words += words
        fence_errors, mermaids = audit_fences(rel, text)
        errors.extend(fence_errors)
        errors.extend(audit_links(root, path, text))
        mermaid_count += mermaids
        lines = text.splitlines()
        for index in range(1, len(lines)):
            if lines[index].strip() and lines[index].strip() == lines[index - 1].strip():
                errors.append(f"{rel}:{index+1}: consecutive duplicate line")

        # Placeholder markers are defects in operational docs, but intentional in record
        # templates. Angle-bracket examples without TODO/TBD are not flagged.
        if rel in OPERATIONAL_MD:
            for match in PLACEHOLDER_RE.finditer(text):
                line_no = text.count("\n", 0, match.start()) + 1
                warnings.append(f"{rel}:{line_no}: placeholder marker {match.group(0)!r}")

        rows.append(
            {
                "path": rel,
                "words": words,
                "bytes": path.stat().st_size,
                "sha256": sha256(path),
                "mermaid": mermaids,
            }
        )

    for rel, phrases in REQUIRED_TEXT.items():
        path = root / rel
        if not path.is_file() or path.is_symlink():
            continue
        text = re.sub(r"\s+", " ", path.read_text(encoding="utf-8")).casefold()
        for phrase in phrases:
            normalized = re.sub(r"\s+", " ", phrase).casefold()
            if normalized not in text:
                errors.append(f"{rel}: missing required invariant: {phrase!r}")

    if payload_complete:
        audit_policy(root, errors)
    if mermaid_count < 15:
        errors.append(f"only {mermaid_count} Mermaid diagrams found; expected at least 15")

    report = {
        "status": "pass" if not errors and not warnings else "fail",
        "root": root.name,
        "revision": REVISION,
        "counts": {
            "files": len(actual),
            "payload_files": len(PAYLOAD_FILES),
            "markdown_files": len(md_files),
            "words": total_words,
            "mermaid_diagrams": mermaid_count,
            "errors": len(errors),
            "warnings": len(warnings),
        },
        "errors": errors,
        "warnings": warnings,
        "markdown_files": rows,
        "notes": [
            "Checks an exact archive allowlist and all payload hashes/byte counts.",
            "Checks Markdown fences, relative links, TOML templates, phase/lane sequences, prompt coverage and locked invariants.",
            "Checks production maturity, exact-profile, moving-upstream, contribution, learning and disclosure controls across documents.",
            "Mermaid diagrams are structurally counted; this audit does not render them.",
            "Source, package, provider, interoperability and platform claims remain implementation evidence gates.",
        ],
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    symlink = first_symlink_component(output)
    if symlink is not None:
        print(f"error: audit report path contains symlink: {symlink}", file=sys.stderr)
        return 2
    report_text = json.dumps(report, indent=2, sort_keys=True) + "\n"
    try:
        parent_fd = os.open(
            output.parent,
            os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
        )
        try:
            report_fd = os.open(
                output.name,
                os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_NOFOLLOW | os.O_CLOEXEC,
                0o644,
                dir_fd=parent_fd,
            )
        finally:
            os.close(parent_fd)
        with os.fdopen(report_fd, "w", encoding="utf-8") as handle:
            handle.write(report_text)
    except OSError as exc:
        print(f"error: cannot safely write audit report {output}: {exc}", file=sys.stderr)
        return 2

    print(json.dumps(report["counts"], indent=2, sort_keys=True))
    if errors:
        print("\nErrors:")
        for error in errors:
            print(f"- {error}")
    if warnings:
        print("\nWarnings:")
        for warning in warnings:
            print(f"- {warning}")
    print(f"\nReport: {output}")
    return 0 if not errors and not warnings else 1


if __name__ == "__main__":
    raise SystemExit(main())
