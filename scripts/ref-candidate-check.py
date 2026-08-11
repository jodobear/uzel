#!/usr/bin/env python3
"""Fail-closed evidence checker for the Phase 01 Napp candidate.

This tool intentionally knows one finite Git read grammar.  It never executes a
candidate supplied command, never reads the sibling checkout, and stores the
binary committed-tree inventory in this Uzel checkout's ignored .artifacts/.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Iterable


EXPECTED_COMMIT = "0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e"
EXPECTED_REPOSITORY = "jodobear/napp"
MARKER_BEGIN = "<!-- ref-candidate-record:begin -->"
MARKER_END = "<!-- ref-candidate-record:end -->"
HEX = re.compile(r"^[0-9a-f]{40,64}$")
SAFE_PATH = re.compile(r"^(?!-)(?!.*(?:^|/)(?:\.|\.\.)(?:/|$))[A-Za-z0-9._/@+=,:-]+(?:/[A-Za-z0-9._/@+=,:-]+)*$")
ROOT = Path(__file__).resolve().parents[1]


class CheckError(RuntimeError):
    pass


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def json_bytes(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode()


def canonical_record(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def root_path(repo: Path) -> Path:
    return Path(run_git(repo, ["rev-parse", "--show-toplevel"]).stdout.decode().strip()).resolve()


def safe_env() -> tuple[dict[str, str], tempfile.TemporaryDirectory[str]]:
    home = tempfile.TemporaryDirectory(prefix="uzel-ref-candidate-home-")
    os.chmod(home.name, 0o700)
    return ({
        "LC_ALL": "C", "LANG": "C", "HOME": home.name, "XDG_CONFIG_HOME": home.name,
        "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull,
        "GIT_OPTIONAL_LOCKS": "0", "GIT_NO_REPLACE_OBJECTS": "1", "GIT_NO_LAZY_FETCH": "1",
        "GIT_TERMINAL_PROMPT": "0", "GIT_PROTOCOL_FROM_USER": "0", "GIT_PAGER": "cat",
        "PAGER": "cat", "GIT_ATTR_NOSYSTEM": "1",
    }, home)


def git_binary() -> str:
    path = shutil.which("git", path=os.defpath)
    if not path or not os.path.isabs(path):
        raise CheckError("absolute Git executable unavailable")
    return str(Path(path).resolve())


def repo_git_path(repo: Path, value: str) -> Path:
    path = Path(value)
    if not path.is_absolute():
        path = repo / path
    return path.resolve()


def run_git(repo: Path, argv: list[str], stdin: bytes | None = None) -> subprocess.CompletedProcess[bytes]:
    """Run trusted fixed Git argv. Candidate argv never reaches this function directly."""
    env, home = safe_env()
    try:
        fixed = [git_binary(), "--no-pager", "--no-replace-objects",
                 "-c", "core.hooksPath=/dev/null", "-c", "core.fsmonitor=false",
                 "-c", "maintenance.auto=false", "-c", "gc.auto=0", "-c", "diff.external=",
                 "-c", "core.pager=cat", "-c", "filter.lfs.clean=", "-c", "filter.lfs.smudge=",
                 "-c", "filter.lfs.process=", "-c", "credential.helper=", *argv]
        return subprocess.run(fixed, cwd=str(repo), env=env, input=stdin,
                              stdin=subprocess.PIPE if stdin is not None else subprocess.DEVNULL,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=False, check=False)
    finally:
        home.cleanup()


def must_git(repo: Path, argv: list[str], stdin: bytes | None = None) -> bytes:
    result = run_git(repo, argv, stdin)
    if result.returncode:
        raise CheckError("Git read failed: " + " ".join(argv) + ": " + result.stderr.decode("utf-8", "replace").strip())
    return result.stdout


def file_state(path: Path) -> dict[str, Any]:
    try:
        item = path.lstat()
    except FileNotFoundError:
        return {"exists": False}
    kind = "symlink" if stat.S_ISLNK(item.st_mode) else "directory" if stat.S_ISDIR(item.st_mode) else "file"
    result: dict[str, Any] = {"exists": True, "kind": kind, "mode": stat.S_IMODE(item.st_mode), "size": item.st_size,
                              "mtime_ns": item.st_mtime_ns, "ctime_ns": item.st_ctime_ns}
    if kind == "symlink":
        result["target"] = os.readlink(path)
    if kind == "file":
        result["sha256"] = sha256(path.read_bytes())
    return result


def object_manifest(objects: Path) -> dict[str, Any]:
    entries: list[dict[str, Any]] = []
    if not objects.exists():
        return {"entries": entries, "sha256": sha256(b"")}
    for path in sorted(objects.rglob("*"), key=lambda item: os.fsencode(str(item.relative_to(objects)))):
        row = {"path_hex": os.fsencode(str(path.relative_to(objects))).hex(), **file_state(path)}
        entries.append(row)
    return {"entries": entries, "sha256": sha256(json_bytes(entries))}


def snapshot(repo: Path) -> dict[str, Any]:
    repo = root_path(repo)
    git_dir = repo_git_path(repo, must_git(repo, ["rev-parse", "--git-dir"]).decode().strip())
    common_dir = repo_git_path(repo, must_git(repo, ["rev-parse", "--git-common-dir"]).decode().strip())
    object_dir = repo_git_path(repo, must_git(repo, ["rev-parse", "--git-path", "objects"]).decode().strip())
    index_path = repo_git_path(repo, must_git(repo, ["rev-parse", "--git-path", "index"]).decode().strip())
    values = {
        "root": str(repo), "head": must_git(repo, ["rev-parse", "HEAD"]).decode().strip(),
        "head_bytes_sha256": sha256(must_git(repo, ["rev-parse", "HEAD"])),
        "raw_index": file_state(index_path),
        "index_serialization_sha256": sha256(must_git(repo, ["ls-files", "-s", "-z"])),
        "refs_sha256": sha256(must_git(repo, ["for-each-ref", "--sort=refname", "--format=%(refname) %(objectname)"])),
        "status_sha256": sha256(must_git(repo, ["status", "--porcelain=v2", "-z"])),
        "git_dir": str(git_dir), "common_dir": str(common_dir), "object_dir": str(object_dir),
        "protected": {name: file_state(common_dir / name) for name in ("HEAD", "packed-refs", "refs", "logs", "worktrees")},
        "objects": object_manifest(object_dir),
    }
    values["fingerprint"] = sha256(json_bytes(values))
    return values


def parse_inventory(data: bytes) -> list[str]:
    objects: list[str] = []
    for entry in data.split(b"\0"):
        if not entry:
            continue
        header, _sep, _path = entry.partition(b"\t")
        fields = header.split()
        if len(fields) != 3 or not HEX.fullmatch(fields[2].decode("ascii", "ignore")):
            raise CheckError("invalid canonical ls-tree inventory")
        objects.append(fields[2].decode())
    return objects


def candidate_argv_ok(argv: list[str], commit: str, object_ids: set[str], stdin: bytes | None = None) -> tuple[bool, str]:
    """Exhaustive candidate declaration grammar; rejection precedes process creation."""
    if not argv or argv[0] != "git" or any("\n" in item or any(ch in item for ch in ";&|`$<>") for item in argv):
        return False, "forbidden-token"
    commit_forms = {commit + "^{commit}", commit + "^{tree}"}
    if argv in (["git", "rev-parse", "--verify", "--end-of-options", commit + "^{commit}"],
                ["git", "rev-parse", "--verify", "--end-of-options", commit + "^{tree}"],
                ["git", "ls-tree", "-rz", "--full-tree", commit]):
        return True, "admitted"
    if len(argv) >= 6 and argv[:5] == ["git", "ls-tree", "-rz", "--full-tree", commit] and argv[5] == "--":
        return (all(SAFE_PATH.fullmatch(path) for path in argv[6:]), "admitted-paths" if all(SAFE_PATH.fullmatch(path) for path in argv[6:]) else "unsafe-path")
    if len(argv) == 6 and argv[:5] == ["git", "show", "--no-ext-diff", "--no-textconv", "--no-renames"]:
        return False, "invalid-show-shape"
    if len(argv) == 7 and argv[:6] == ["git", "show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format="]:
        value = argv[6]
        return (value.startswith(commit + ":") and bool(SAFE_PATH.fullmatch(value[len(commit) + 1:])), "admitted-show" if value.startswith(commit + ":") else "unsafe-show")
    if len(argv) == 3 and argv[:2] == ["git", "cat-file"] and argv[2] == "--batch-check=%(objectname) %(objecttype) %(objectsize)":
        valid = stdin is not None and all(line in object_ids for line in stdin.decode("ascii", "ignore").splitlines()) and stdin.endswith(b"\n")
        return valid, "admitted-batch" if valid else "unsafe-batch-stdin"
    if len(argv) == 3 and argv[:2] == ["git", "cat-file"]:
        return argv[2] in object_ids, "admitted-object" if argv[2] in object_ids else "unknown-object"
    if len(argv) == 4 and argv[:3] == ["git", "cat-file", "-e"]:
        return argv[3] in object_ids or argv[3] == commit + "^{commit}", "admitted-exists" if argv[3] in object_ids or argv[3] == commit + "^{commit}" else "unknown-object"
    if len(argv) == 4 and argv[:3] in (["git", "cat-file", "-t"], ["git", "cat-file", "-s"]):
        return argv[3] in object_ids, "admitted-object" if argv[3] in object_ids else "unknown-object"
    return False, "not-in-literal-grammar"


def pinned_blob(repo: Path, commit: str, path: str, inventory_paths: set[str]) -> dict[str, Any]:
    if path not in inventory_paths:
        return {"path": path, "status": "missing-in-committed-tree"}
    oid = must_git(repo, ["rev-parse", "--verify", "--end-of-options", commit + ":" + path]).decode().strip()
    data = must_git(repo, ["show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", commit + ":" + path])
    return {"path": path, "status": "present", "blob_oid": oid, "content_sha256": sha256(data)}


def candidate_record(repo: Path, commit: str) -> dict[str, Any]:
    repo = root_path(repo)
    before_napp, before_uzel = snapshot(repo), snapshot(ROOT)
    if commit != EXPECTED_COMMIT or not HEX.fullmatch(commit):
        raise CheckError("expected full candidate commit only")
    must_git(repo, ["cat-file", "-e", commit + "^{commit}"])
    tree = must_git(repo, ["rev-parse", "--verify", "--end-of-options", commit + "^{tree}"]).decode().strip()
    inventory = must_git(repo, ["ls-tree", "-rz", "--full-tree", commit])
    inventory_paths = {entry.partition(b"\t")[2].decode("utf-8", "surrogateescape") for entry in inventory.split(b"\0") if entry}
    artifact_relative = f".artifacts/phase-01/napp/{commit}/tree.bin"
    artifact = ROOT / artifact_relative
    artifact.parent.mkdir(parents=True, exist_ok=True)
    artifact.write_bytes(inventory)
    os.chmod(artifact, 0o600)
    objects = set(parse_inventory(inventory))
    origin = must_git(repo, ["remote", "get-url", "origin"]).decode().strip()
    refs = must_git(repo, ["for-each-ref", "--sort=refname", "--format=%(refname)"]).decode().splitlines()
    approved = [ref for ref in refs if run_git(repo, ["merge-base", "--is-ancestor", commit, ref]).returncode == 0]
    categories = {}
    names = ["product_client", "product_events", "testkit_vectors", "version_authority", "lifecycle_recovery",
             "instance_profile_scope", "nmp_ownership_projection", "pin_parity_input"]
    for name in names:
        owner = "jodobear/napp" if name not in {"nmp_ownership_projection", "pin_parity_input"} else "jodobear/napp with NMP/Uzel evidence"
        categories[name] = {"status": "missing", "evidence": "missing-in-committed-tree", "sha256": None, "owner": owner}
    record = {
        "schema": "uzel.ref-candidate/v1", "repository": EXPECTED_REPOSITORY, "origin": origin,
        "observed_commit": commit, "observed_tree": tree, "tree_inventory_path": artifact_relative,
        "tree_inventory_sha256": sha256(inventory), "approved_ref_reachability": {"approved_refs": approved, "reachable": bool(approved)},
        "pinned_blobs": {path: pinned_blob(repo, commit, path, inventory_paths) for path in ["AGENTS.md", "README.md", ".planning/PROJECT.md", ".planning/ROADMAP.md", ".planning/REQUIREMENTS.md"]},
        "working_tree_evidence": "excluded", "admission_categories": categories,
        "declared_probes": [{"kind": "project", "raw_declaration": "missing-in-committed-tree", "probe_status": "skipped-unsafe", "rejection_code": "no-candidate-declared-sandbox-contract", "required": True}],
        "controlled_runner": {"git": git_binary(), "shell": False, "environment": "allowlist-only", "network": "disabled-by-policy", "candidate_argv": "finite-literal-grammar"},
        "mutation_snapshots": {"napp_before": before_napp, "uzel_before": before_uzel},
        "missing_categories": names + ["declared_executable_probes"], "result": "stop",
        "blocker": "Committed candidate lacks source-backed admission evidence and an admitted project-probe sandbox contract.",
        "owner": "jodobear/napp", "next_probe": "Napp owners publish committed behavioral evidence, vectors, and an approved read-only probe contract.",
        "rollback": "Preserve the accepted POC pin; revert only the future Napp pin and narrow adapter together.",
        "d18_rule": "One handoff per semantic candidate; qualification never authorizes publication or adapter implementation.",
    }
    record["mutation_snapshots"]["napp_after"] = snapshot(repo)
    record["mutation_snapshots"]["uzel_after"] = snapshot(ROOT)
    return record


def read_record(path: Path) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    if text.count(MARKER_BEGIN) != 1 or text.count(MARKER_END) != 1:
        raise CheckError("record requires exactly one marker-delimited canonical JSON block")
    raw = text.split(MARKER_BEGIN, 1)[1].split(MARKER_END, 1)[0].strip()
    value = json.loads(raw)
    if canonical_record(value) != raw:
        raise CheckError("record JSON is not canonical")
    return value


def validate_record(record: dict[str, Any], repo: Path, expected_commit: str, expected_result: str) -> None:
    required = {"schema", "repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "pinned_blobs", "working_tree_evidence", "admission_categories", "declared_probes", "controlled_runner", "mutation_snapshots", "missing_categories", "result", "blocker", "owner", "next_probe", "rollback", "d18_rule"}
    if set(record) != required:
        raise CheckError("qualification fields are missing or unknown")
    if record["repository"] != EXPECTED_REPOSITORY or record["observed_commit"] != expected_commit or record["result"] != expected_result:
        raise CheckError("qualification identity/result mismatch")
    exact_path = f".artifacts/phase-01/napp/{expected_commit}/tree.bin"
    if record["tree_inventory_path"] != exact_path or not exact_path.endswith("/tree.bin"):
        raise CheckError("tree inventory path is not the fixed .bin path")
    artifact = (ROOT / exact_path).resolve()
    expected_root = (ROOT / ".artifacts/phase-01/napp").resolve()
    if expected_root not in artifact.parents or artifact.is_symlink() or not artifact.is_file() or stat.S_IMODE(artifact.stat().st_mode) != 0o600:
        raise CheckError("tree inventory confinement/type/mode failed")
    actual = must_git(root_path(repo), ["ls-tree", "-rz", "--full-tree", expected_commit])
    if artifact.read_bytes() != actual or record["tree_inventory_sha256"] != sha256(actual):
        raise CheckError("tree inventory bytes or digest mismatch")
    if record["working_tree_evidence"] != "excluded" or record["result"] == "qualified-for-research":
        raise CheckError("current candidate must remain fail-closed")
    snapshots = record["mutation_snapshots"]
    if snapshots["napp_before"]["fingerprint"] != snapshots["napp_after"]["fingerprint"]:
        raise CheckError("Napp mutation snapshot changed")
    for category in record["admission_categories"].values():
        if category["status"] != "missing":
            raise CheckError("current candidate may not claim an unverified category")


def qualification(args: argparse.Namespace) -> None:
    record = read_record(Path(args.record))
    validate_record(record, Path(args.repo), args.expected_commit, args.expected_result)
    print("qualification: pass")


def handoff(args: argparse.Namespace) -> None:
    qualification_record = read_record(Path(args.qualification))
    handoff_record = read_record(Path(args.handoff))
    validate_record(qualification_record, Path(args.napp_repo), EXPECTED_COMMIT, "stop")
    copied = ["repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "admission_categories", "declared_probes", "missing_categories", "working_tree_evidence", "mutation_snapshots", "blocker", "owner", "next_probe", "rollback", "d18_rule"]
    for field in copied:
        if handoff_record.get(field) != qualification_record[field]:
            raise CheckError("handoff field mismatch: " + field)
    required = {"schema", *copied, "result", "plan_contract", "ref_01d_preconditions", "d12_routing", "required_napp_deliverables", "exact_heads", "resume_command"}
    if set(handoff_record) != required or handoff_record["schema"] != "uzel.napp-dependency/v1" or handoff_record["result"] != "stop":
        raise CheckError("handoff schema/result mismatch")
    contract = handoff_record["plan_contract"]
    contract_commit = contract.get("commit")
    if not isinstance(contract_commit, str) or not HEX.fullmatch(contract_commit):
        raise CheckError("handoff contract commit is not an exact object id")
    plan_spec = contract_commit + ":" + args.plan
    plan_bytes = must_git(ROOT, ["show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", plan_spec])
    plan_oid = must_git(ROOT, ["rev-parse", "--verify", "--end-of-options", plan_spec]).decode().strip()
    if contract != {"path": args.plan, "commit": contract_commit, "blob_oid": plan_oid, "content_sha256": sha256(plan_bytes), "fixed_production_commit": "19519c378c2e775c6ad4b042cfd9aadd89f766b9", "replay_manifest_path": ".artifacts/phase-01/replay/manifest.json", "replay_manifest_status": "pending-plan-01"}:
        raise CheckError("committed Plan-01 parity contract mismatch")
    if handoff_record["exact_heads"].get("uzel") != contract_commit:
        raise CheckError("handoff exact Uzel head must equal parity contract commit")
    if len(handoff_record["ref_01d_preconditions"]) != 3:
        raise CheckError("REF-01D requires three independent preconditions")
    print("handoff: pass")


def self_test(_: argparse.Namespace) -> None:
    object_ids = {"a" * 40}
    positive = [["git", "rev-parse", "--verify", "--end-of-options", EXPECTED_COMMIT + "^{commit}"],
                ["git", "rev-parse", "--verify", "--end-of-options", EXPECTED_COMMIT + "^{tree}"],
                ["git", "ls-tree", "-rz", "--full-tree", EXPECTED_COMMIT],
                ["git", "ls-tree", "-rz", "--full-tree", EXPECTED_COMMIT, "--", "README.md"],
                ["git", "show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", EXPECTED_COMMIT + ":README.md"],
                ["git", "cat-file", "-e", "a" * 40], ["git", "cat-file", "-t", "a" * 40], ["git", "cat-file", "-s", "a" * 40]]
    negative = [["git", "write-tree"], ["git", "hash-object", "-w", "x"], ["git", "cat-file", "--textconv", "a" * 40],
                ["git", "cat-file", "--filters", "a" * 40], ["git", "show", "--textconv", "x"], ["git", "show", "--ext-diff", "x"],
                ["git", "-c", "alias.x=!touch", "x"], ["git", "--config-env=x=y", "status"], ["git", "update-ref", "x", "y"],
                ["git", "symbolic-ref", "HEAD", "x"], ["git", "update-index", "x"], ["git", "maintenance", "run"], ["git", "lfs", "install"],
                ["git", "show", "$(curl x)"], ["curl", "https://example.invalid"], ["git", "show", "a;b"]]
    before = snapshot(ROOT)
    if not all(candidate_argv_ok(value, EXPECTED_COMMIT, object_ids)[0] for value in positive):
        raise CheckError("literal grammar rejected a positive case")
    if any(candidate_argv_ok(value, EXPECTED_COMMIT, object_ids)[0] for value in negative):
        raise CheckError("literal grammar admitted a negative case")
    if candidate_argv_ok(["git", "cat-file", "--batch-check=%(objectname) %(objecttype) %(objectsize)"], EXPECTED_COMMIT, object_ids, b"a" * 40 + b"\n")[0] is False:
        raise CheckError("literal grammar rejected safe batch form")
    after = snapshot(ROOT)
    if before["fingerprint"] != after["fingerprint"]:
        raise CheckError("self-test changed repository state")
    print("self-test: pass (negative probes rejected before spawn)")


def emit(args: argparse.Namespace) -> None:
    record = candidate_record(Path(args.repo), args.expected_commit)
    target = Path(args.record)
    target.write_text("# Napp Candidate Qualification\n\nCommitted-object evidence only; sibling working-tree material is excluded.\n\n" + MARKER_BEGIN + "\n" + canonical_record(record) + "\n" + MARKER_END + "\n", encoding="utf-8")
    print("qualification record written: " + str(target))


def emit_handoff(args: argparse.Namespace) -> None:
    qualification_record = read_record(Path(args.qualification))
    plan_path = args.plan
    plan_bytes = must_git(ROOT, ["show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", "HEAD:" + plan_path])
    plan_oid = must_git(ROOT, ["rev-parse", "--verify", "--end-of-options", "HEAD:" + plan_path]).decode().strip()
    copied = ["repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "admission_categories", "declared_probes", "missing_categories", "working_tree_evidence", "mutation_snapshots", "blocker", "owner", "next_probe", "rollback", "d18_rule"]
    record = {field: qualification_record[field] for field in copied}
    record.update({
        "schema": "uzel.napp-dependency/v1", "result": qualification_record["result"],
        "plan_contract": {"path": plan_path, "commit": must_git(ROOT, ["rev-parse", "HEAD"]).decode().strip(), "blob_oid": plan_oid,
                          "content_sha256": sha256(plan_bytes), "fixed_production_commit": "19519c378c2e775c6ad4b042cfd9aadd89f766b9",
                          "replay_manifest_path": ".artifacts/phase-01/replay/manifest.json", "replay_manifest_status": "pending-plan-01"},
        "ref_01d_preconditions": [
            {"name": "authority-set", "status": "blocked", "requirement": "Plan 03 authority set at one committed exact SHA."},
            {"name": "qualified-candidate", "status": "blocked", "requirement": "Exact reachable Napp commit qualified-for-research."},
            {"name": "replay-evidence", "status": "blocked", "requirement": "Plan-01 evidence admitted at this contract with exact manifest digest."},
        ],
        "d12_routing": "Reusable fixes use a dedicated jodobear/napp branch and issue, enter the contribution ledger only after Uzel validation, and are not mutated by this plan.",
        "required_napp_deliverables": {"behavior": ["committed product client evidence", "committed product event evidence", "committed testkit/lifecycle/version/pin vectors", "approved bounded read-only project-probe sandbox contract", "Napp runtime authority with NMP as sole Nostr/store/signer/publication owner"], "repository_binding": {"uzel": "jodobear/uzel evidence-only candidate handoff; PR/issue pending separate authorization", "napp": "jodobear/napp dependency at observed commit"}},
        "exact_heads": {"uzel": must_git(ROOT, ["rev-parse", "HEAD"]).decode().strip(), "napp": qualification_record["observed_commit"]},
        "resume_command": "$gsd-plan-phase 1 --research; then create one narrow Rust/Tauri adapter plan only after all three preconditions; retain D-15 pin/adapter revert.",
    })
    target = Path(args.handoff)
    target.write_text("# Napp Dependency Handoff\n\nOwned by `jodobear/napp`; consumed by `jodobear/uzel`. This is a candidate/pending stop, not publication authority.\n\n" + MARKER_BEGIN + "\n" + canonical_record(record) + "\n" + MARKER_END + "\n", encoding="utf-8")
    print("handoff record written: " + str(target))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    test = commands.add_parser("self-test"); test.set_defaults(func=self_test)
    write = commands.add_parser("write-qualification"); write.add_argument("--repo", required=True); write.add_argument("--expected-commit", required=True); write.add_argument("--record", required=True); write.set_defaults(func=emit)
    write_handoff = commands.add_parser("write-handoff"); write_handoff.add_argument("--qualification", required=True); write_handoff.add_argument("--handoff", required=True); write_handoff.add_argument("--plan", required=True); write_handoff.set_defaults(func=emit_handoff)
    check = commands.add_parser("qualification"); check.add_argument("--repo", required=True); check.add_argument("--expected-repository", required=True); check.add_argument("--expected-commit", required=True); check.add_argument("--record", required=True); check.add_argument("--expected-result", required=True); check.set_defaults(func=qualification)
    hand = commands.add_parser("handoff"); hand.add_argument("--repo", required=True); hand.add_argument("--napp-repo", required=True); hand.add_argument("--qualification", required=True); hand.add_argument("--handoff", required=True); hand.add_argument("--plan", required=True); hand.set_defaults(func=handoff)
    try:
        args = parser.parse_args()
        args.func(args)
    except (CheckError, OSError, json.JSONDecodeError) as error:
        print("ref-candidate-check: " + str(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
