#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "graphify-out" / "manifest.json"
REPORT = ROOT / "graphify-out" / "GRAPH_REPORT.md"
DIRECT_REFRESH = "Run `graphify update .` after code changes (no API cost)."
LOCKED_REFRESH = "Run locked `pnpm graphify:refresh` after code changes."


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    for value in manifest.values():
        value.pop("mtime", None)
    MANIFEST.write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    report = REPORT.read_text(encoding="utf-8")
    if DIRECT_REFRESH not in report:
        raise SystemExit("Graphify freshness instruction changed; update normalizer")
    REPORT.write_text(report.replace(DIRECT_REFRESH, LOCKED_REFRESH), encoding="utf-8")


if __name__ == "__main__":
    main()
