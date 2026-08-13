#!/usr/bin/env python3
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CACHE = ROOT / "graphify-out" / "cache"
ABSOLUTE_CHECKOUT = re.compile(
    r'/tmp/|/workspace/projects/|"(?:origin_file|source_file)"\s*:\s*"/'
)


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
