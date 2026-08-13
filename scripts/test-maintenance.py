#!/usr/bin/env python3
from __future__ import annotations

import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def run(
    command: list[str], cwd: Path, env: dict[str, str] | None = None
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command, cwd=cwd, env=env, check=True, capture_output=True, text=True
    )


def init_repository(root: Path) -> None:
    run(["git", "init", "-q"], root)
    run(["git", "add", "."], root)
    run(
        [
            "git",
            "-c",
            "user.name=Uzel maintenance test",
            "-c",
            "user.email=uzel-maintenance@example.invalid",
            "commit",
            "-qm",
            "fixture",
        ],
        root,
    )


class MaintenanceTests(unittest.TestCase):
    def test_docs_check_is_read_only_after_relocation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="uzel-docs-check-") as raw:
            checkout = Path(raw) / "relocated-checkout"
            checkout.mkdir()
            shutil.copy2(ROOT / "package.json", checkout / "package.json")
            shutil.copytree(
                ROOT / "uzel-poc-validated-pack",
                checkout / "uzel-poc-validated-pack",
            )
            init_repository(checkout)

            completed = run(["pnpm", "--silent", "docs:check"], checkout)

            self.assertEqual(run(["git", "status", "--porcelain"], checkout).stdout, "")
            self.assertNotIn(str(checkout), completed.stdout)
            result = json.loads(completed.stdout)
            self.assertEqual(result["root"], ".")
            evidence = (
                checkout / "uzel-poc-validated-pack" / "audit-results.json"
            ).read_text(encoding="utf-8")
            self.assertNotIn(str(checkout), evidence)

if __name__ == "__main__":
    unittest.main()
