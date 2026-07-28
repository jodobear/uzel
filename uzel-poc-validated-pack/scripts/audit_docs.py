#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def write_manifest() -> None:
    files = []
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file() or path.name == "manifest.json":
            continue
        if "__pycache__" in path.parts or path.suffix == ".pyc":
            continue
        data = path.read_bytes()
        files.append(
            {
                "path": path.relative_to(ROOT).as_posix(),
                "bytes": len(data),
                "sha256": hashlib.sha256(data).hexdigest(),
            }
        )
    (ROOT / "manifest.json").write_text(
        json.dumps({"files": files}, indent=2) + "\n", encoding="utf-8"
    )


def strip_code(text: str) -> str:
    return re.sub(r"```.*?```", "", text, flags=re.S)


def main() -> int:
    errors: list[str] = []
    warnings: list[str] = []
    markdown = sorted(ROOT.rglob("*.md"))
    words = 0
    links = 0
    mermaid = 0

    for path in markdown:
        text = path.read_text(encoding="utf-8")
        words += len(re.findall(r"\b\w+[\w'-]*\b", strip_code(text)))
        if not re.search(r"^#\s+", text, flags=re.M):
            errors.append(f"missing H1: {path.relative_to(ROOT)}")

        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", text):
            links += 1
            raw = target.split("#", 1)[0]
            if not raw or "://" in raw or raw.startswith("mailto:"):
                continue
            resolved = (path.parent / raw).resolve()
            if not resolved.exists():
                errors.append(
                    f"broken link: {path.relative_to(ROOT)} -> {target}"
                )

        blocks = re.findall(r"```mermaid\s*\n(.*?)```", text, flags=re.S)
        mermaid += len(blocks)
        for idx, block in enumerate(blocks, 1):
            first = next((line.strip() for line in block.splitlines() if line.strip()), "")
            if not re.match(
                r"^(flowchart|graph|sequenceDiagram|stateDiagram(?:-v2)?|classDiagram|erDiagram|journey|timeline|C4\w*)\b",
                first,
            ):
                errors.append(
                    f"unknown mermaid start: {path.relative_to(ROOT)}#{idx}: {first}"
                )

        if len(text.split()) > 2200:
            warnings.append(f"large document: {path.relative_to(ROOT)}")

    policy_docs = [p for p in markdown if p.name != "AUDIT.md"]
    all_text = "\n".join(p.read_text(encoding="utf-8") for p in policy_docs)
    for forbidden in ["GSD", "work/07", "work/08", "templates/fact-record"]:
        if forbidden in all_text:
            errors.append(f"stale/forbidden reference present: {forbidden}")

    required = [
        "README.md",
        "AGENTS.md",
        "STATUS.md",
        "docs/00-scope.md",
        "docs/01-validation.md",
        "docs/02-architecture.md",
        "docs/03-provisional-design.md",
        "docs/04-execution.md",
        "docs/05-test-and-demo.md",
        "docs/06-extraction.md",
        "docs/07-source-baseline.md",
        "compatibility.lock",
        "docs/facts/FACT-001-kehto-package-line.md",
        "docs/facts/FACT-002-spec-revisions.md",
        "docs/facts/FACT-003-nampplets-linux-reuse.md",
        "docs/facts/FACT-004-nmp-facade.md",
        "docs/facts/FACT-005-webkit-tauri-trust.md",
        "docs/facts/FACT-006-csp-egress.md",
        "docs/facts/FACT-007-local-ipc.md",
        "docs/facts/FACT-008-toolchain.md",
        "reports/preflight.md",
        "reports/nampplets-linux-map.md",
        "reports/nmp-api-map.md",
        "reports/webkit-trust-spike.md",
        "work/00-validate.md",
        "work/01-scaffold.md",
        "work/02-linux-runner.md",
        "work/03-napplets.md",
        "work/04-daemon-nmp.md",
        "work/05-integrate.md",
        "work/06-hardening-demo.md",
        "templates/fact.md",
        "templates/handoff.md",
        "config/fallow.jsonc",
    ]
    for rel in required:
        if not (ROOT / rel).exists():
            errors.append(f"missing required file: {rel}")

    result = {
        "root": str(ROOT),
        "markdown_documents": len(markdown),
        "approx_words": words,
        "markdown_links": links,
        "mermaid_blocks": mermaid,
        "errors": errors,
        "warnings": warnings,
    }
    (ROOT / "audit-results.json").write_text(
        json.dumps(result, indent=2) + "\n", encoding="utf-8"
    )
    write_manifest()
    print(json.dumps(result, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
