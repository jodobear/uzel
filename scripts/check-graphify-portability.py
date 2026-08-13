#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CACHE = ROOT / "graphify-out" / "cache"
ABSOLUTE_CHECKOUT = re.compile(
    r'/tmp/|/workspace/projects/|"(?:origin_file|source_file)"\s*:\s*"/'
)
DIRECT_REFRESH = "Run `graphify update .` after code changes"


def tracked_graph_files() -> list[Path]:
    listed = subprocess.run(
        ["git", "-C", str(ROOT), "ls-files", "-z", "--", "graphify-out"],
        check=True,
        capture_output=True,
    )
    return [ROOT / raw.decode("utf-8") for raw in listed.stdout.split(b"\0") if raw]


def main() -> int:
    failures: list[str] = []
    tracked = tracked_graph_files()
    tracked_cache = [path for path in tracked if CACHE in path.parents]
    if tracked_cache:
        failures.append(f"machine-local Graphify cache is tracked ({len(tracked_cache)} files)")
    if CACHE.exists() and any(CACHE.iterdir()):
        failures.append("machine-local Graphify cache remains after canonical refresh")

    manifest_path = ROOT / "graphify-out" / "manifest.json"
    if manifest_path.exists():
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        timestamped = [key for key, value in manifest.items() if "mtime" in value]
        if timestamped:
            failures.append(
                f"checkout-dependent mtimes remain in Graphify manifest ({len(timestamped)} entries)"
            )

    report_path = ROOT / "graphify-out" / "GRAPH_REPORT.md"
    if report_path.exists() and DIRECT_REFRESH in report_path.read_text(encoding="utf-8"):
        failures.append("Graph Report bypasses the locked graphify:refresh entrypoint")

    for path in tracked:
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        if ABSOLUTE_CHECKOUT.search(text):
            failures.append(f"absolute checkout path in canonical output: {path.relative_to(ROOT)}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    print("Graphify canonical output is portable; machine-local cache is absent and untracked.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
