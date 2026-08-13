#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import shutil
import stat
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

    def test_graphify_refresh_discards_machine_local_cache_every_time(self) -> None:
        with tempfile.TemporaryDirectory(prefix="uzel-graphify-refresh-") as raw:
            checkout = Path(raw) / "relocated-checkout"
            (checkout / "scripts").mkdir(parents=True)
            (checkout / "graphify-out").mkdir()
            (checkout / "bin").mkdir()
            shutil.copy2(ROOT / ".gitignore", checkout / ".gitignore")
            for name in (
                "refresh-graphify.sh",
                "check-graphify-portability.py",
                "normalize-graphify-output.py",
            ):
                shutil.copy2(ROOT / "scripts" / name, checkout / "scripts" / name)
            (checkout / "graphify-out" / "graph.json").write_text(
                '{"nodes":[],"links":[]}\n', encoding="utf-8"
            )
            (checkout / "graphify-out" / "manifest.json").write_text(
                '{\n  "README.md": {\n    "ast_hash": "stable",\n    "semantic_hash": ""\n  }\n}\n',
                encoding="utf-8",
            )
            (checkout / "graphify-out" / "GRAPH_REPORT.md").write_text(
                "Run locked `pnpm graphify:refresh` after code changes.\n",
                encoding="utf-8",
            )
            fake = checkout / "bin" / "graphify"
            fake.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, sys\n"
                "from pathlib import Path\n"
                "assert sys.argv[1:] == ['update', '.']\n"
                "root = Path.cwd()\n"
                "cache = root / 'graphify-out/cache/stat-index.json'\n"
                "cache.parent.mkdir(parents=True, exist_ok=True)\n"
                "cache.write_text(json.dumps({str(root / 'README.md'): {'size': 1}}))\n"
                "counter = Path(os.environ['GRAPHIFY_TEST_COUNTER'])\n"
                "run = int(counter.read_text()) + 1\n"
                "counter.write_text(str(run))\n"
                "(root / 'graphify-out/manifest.json').write_text(json.dumps({'README.md': {'mtime': run, 'ast_hash': 'stable', 'semantic_hash': ''}}))\n"
                "(root / 'graphify-out/GRAPH_REPORT.md').write_text('Run `graphify update .` after code changes (no API cost).\\n')\n",
                encoding="utf-8",
            )
            fake.chmod(fake.stat().st_mode | stat.S_IXUSR)
            init_repository(checkout)
            counter = Path(raw) / "count"
            counter.write_text("0", encoding="utf-8")
            env = os.environ.copy()
            env["PATH"] = f"{checkout / 'bin'}:{env['PATH']}"
            env["GRAPHIFY_TEST_COUNTER"] = str(counter)

            for _ in range(2):
                run(["bash", "scripts/refresh-graphify.sh"], checkout, env)
                self.assertFalse((checkout / "graphify-out" / "cache").exists())

            self.assertEqual(counter.read_text(encoding="utf-8"), "2")
            self.assertEqual(run(["git", "status", "--porcelain"], checkout).stdout, "")


if __name__ == "__main__":
    unittest.main()
