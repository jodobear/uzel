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
import secrets
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
SAFE_PATH_COMPONENT = re.compile(r"^[A-Za-z0-9._@+=,-]+$")
ROOT = Path(__file__).resolve().parents[1]
APPROVED_REFS = ("refs/heads/master",)
CATEGORY_NAMES = (
    "product_client", "product_events", "testkit_vectors", "version_authority",
    "lifecycle_recovery", "instance_profile_scope", "nmp_ownership_projection", "pin_parity_input",
)
BLOCKER = "Committed candidate lacks source-backed admission evidence and an admitted project-probe sandbox contract."
NEXT_PROBE = "Napp owners publish committed behavioral evidence, vectors, and an approved read-only probe contract."
ROLLBACK = "Preserve the accepted POC pin; revert only the future Napp pin and narrow adapter together."
D18_RULE = "One handoff per semantic candidate; qualification never authorizes publication or adapter implementation."
D12_ROUTING = "Reusable fixes use a dedicated jodobear/napp branch and issue, enter the contribution ledger only after Uzel validation, and are not mutated by this plan."
RESUME_COMMAND = "$gsd-plan-phase 1 --research; then create one narrow Rust/Tauri adapter plan only after all three preconditions; retain D-15 pin/adapter revert."
DEFAULT_NAPP_REPO = Path("/workspace/projects/napplets/napp-uzel/napp")
APPROVED_PLAN_PATH = ".planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-01-PLAN.md"
QUALIFICATION_PATH = "evidence/phase-01/candidate-qualification.md"
HANDOFF_PATH = "evidence/phase-01/napp-dependency.md"


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


def write_confined(root: Path, relative: Path, data: bytes) -> None:
    """Write beneath root without following a symlink in any path component."""
    if relative.is_absolute() or any(part in ("", ".", "..") for part in relative.parts):
        raise CheckError("unsafe artifact path")
    flags = os.O_RDONLY | os.O_DIRECTORY
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    directory_fd = os.open(root, flags)
    try:
        for component in relative.parts[:-1]:
            try:
                os.mkdir(component, 0o700, dir_fd=directory_fd)
            except FileExistsError:
                pass
            next_fd = os.open(component, flags, dir_fd=directory_fd)
            os.close(directory_fd)
            directory_fd = next_fd
        try:
            existing = os.stat(relative.name, dir_fd=directory_fd, follow_symlinks=False)
        except FileNotFoundError:
            existing = None
        if existing is not None and (not stat.S_ISREG(existing.st_mode) or existing.st_nlink != 1):
            raise CheckError("output leaf must be a regular single-link file")
        file_flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
        if hasattr(os, "O_NOFOLLOW"):
            file_flags |= os.O_NOFOLLOW
        temporary = "." + relative.name + ".tmp-" + secrets.token_hex(16)
        file_fd = os.open(temporary, file_flags, 0o600, dir_fd=directory_fd)
        try:
            opened = os.fstat(file_fd)
            if not stat.S_ISREG(opened.st_mode) or opened.st_nlink != 1:
                raise CheckError("temporary output is not a regular single-link file")
            os.fchmod(file_fd, 0o600)
            with os.fdopen(file_fd, "wb", closefd=False) as target:
                target.write(data)
                target.flush()
                os.fsync(target.fileno())
            os.replace(temporary, relative.name, src_dir_fd=directory_fd, dst_dir_fd=directory_fd)
            os.fsync(directory_fd)
        finally:
            os.close(file_fd)
            try:
                os.unlink(temporary, dir_fd=directory_fd)
            except FileNotFoundError:
                pass
    finally:
        os.close(directory_fd)


def write_repo_output(path: Path, data: bytes) -> None:
    if path.is_absolute():
        try:
            relative = path.relative_to(ROOT)
        except ValueError as error:
            raise CheckError("output path is outside the Uzel repository") from error
    else:
        relative = path
    write_confined(ROOT, relative, data)


def missing_categories() -> dict[str, dict[str, Any]]:
    return {
        name: {
            "status": "missing", "evidence": "missing-in-committed-tree", "sha256": None,
            "owner": "jodobear/napp with NMP/Uzel evidence"
            if name in {"nmp_ownership_projection", "pin_parity_input"} else "jodobear/napp",
        }
        for name in CATEGORY_NAMES
    }


def declared_probes() -> list[dict[str, Any]]:
    return [{"kind": "project", "raw_declaration": "missing-in-committed-tree",
             "probe_status": "skipped-unsafe", "rejection_code": "no-candidate-declared-sandbox-contract",
             "required": True}]


def controlled_runner() -> dict[str, Any]:
    return {"git": git_binary(), "shell": False, "environment": "allowlist-only",
            "network": "disabled-by-policy", "candidate_argv": "finite-literal-grammar"}


def approved_reachability(repo: Path, commit: str) -> dict[str, Any]:
    reachable = []
    for ref in APPROVED_REFS:
        if run_git(repo, ["show-ref", "--verify", "--quiet", ref]).returncode == 0 \
                and run_git(repo, ["merge-base", "--is-ancestor", commit, ref]).returncode == 0:
            reachable.append(ref)
    return {"approved_refs": reachable, "reachable": bool(reachable)}


def raw_origin(repo: Path) -> str:
    """Read literal local origin without includes or URL rewrite expansion."""
    return must_git(repo, ["config", "--local", "--no-includes", "--get", "remote.origin.url"]).decode().strip()


def safe_literal_path(value: str) -> bool:
    """Accept only literal, relative Git tree paths with non-empty components."""
    if not value or value.startswith(("/", "-", ":")) or ":" in value or "//" in value:
        return False
    components = value.split("/")
    return all(component not in ("", ".", "..")
               and not component.startswith("-")
               and bool(SAFE_PATH_COMPONENT.fullmatch(component))
               for component in components)


def git_invariants_equal(before: dict[str, Any], after: dict[str, Any], *, allow_declared_outputs: bool = False) -> bool:
    keys = ("root", "head", "head_bytes_sha256", "raw_index", "index_serialization_sha256",
            "refs_sha256", "git_dir", "common_dir", "object_dir", "protected", "objects")
    status_key = "status_guard_sha256" if allow_declared_outputs else "status_sha256"
    return all(before.get(key) == after.get(key) for key in (*keys, status_key))


def ref_01d_preconditions() -> list[dict[str, str]]:
    return [
        {"name": "authority-set", "status": "blocked", "requirement": "Plan 03 authority set at one committed exact SHA."},
        {"name": "qualified-candidate", "status": "blocked", "requirement": "Exact reachable Napp commit qualified-for-research."},
        {"name": "replay-evidence", "status": "blocked", "requirement": "Plan-01 evidence admitted at this contract with exact manifest digest."},
    ]


def required_napp_deliverables() -> dict[str, Any]:
    return {
        "behavior": ["committed product client evidence", "committed product event evidence",
                     "committed testkit/lifecycle/version/pin vectors",
                     "approved bounded read-only project-probe sandbox contract",
                     "Napp runtime authority with NMP as sole Nostr/store/signer/publication owner"],
        "repository_binding": {
            "uzel": "jodobear/uzel evidence-only candidate handoff; PR/issue pending separate authorization",
            "napp": "jodobear/napp dependency at observed commit",
        },
    }


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
    head_path = repo_git_path(repo, must_git(repo, ["rev-parse", "--git-path", "HEAD"]).decode().strip())
    head_bytes = head_path.read_bytes()
    status = must_git(repo, ["status", "--porcelain=v2", "-z"])
    status_guard = must_git(repo, ["status", "--porcelain=v2", "-z", "--", ".",
                                   ":(exclude)evidence/phase-01/candidate-qualification.md",
                                   ":(exclude)evidence/phase-01/napp-dependency.md"])
    values = {
        "root": str(repo), "head": must_git(repo, ["rev-parse", "HEAD"]).decode().strip(),
        "head_bytes_sha256": sha256(head_bytes),
        "raw_index": file_state(index_path),
        "index_serialization_sha256": sha256(must_git(repo, ["ls-files", "-s", "-z"])),
        "refs_sha256": sha256(must_git(repo, ["for-each-ref", "--sort=refname", "--format=%(refname) %(objectname)"])),
        "status_sha256": sha256(status), "status_guard_sha256": sha256(status_guard),
        "git_dir": str(git_dir), "common_dir": str(common_dir), "object_dir": str(object_dir),
        "protected": {
            "HEAD": file_state(head_path),
            "common-HEAD": file_state(common_dir / "HEAD"),
            **{name: file_state(common_dir / name) for name in ("packed-refs", "refs", "logs", "worktrees")},
        },
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
        try:
            object_id = fields[2].decode("ascii")
        except (IndexError, UnicodeDecodeError) as error:
            raise CheckError("invalid canonical ls-tree inventory") from error
        if len(fields) != 3 or not HEX.fullmatch(object_id):
            raise CheckError("invalid canonical ls-tree inventory")
        objects.append(object_id)
    return objects


def candidate_argv_ok(argv: list[str], commit: str, object_ids: set[str], stdin: bytes | None = None) -> tuple[bool, str]:
    """Exhaustive candidate declaration grammar; rejection precedes process creation."""
    if not argv or argv[0] != "git" or any("\n" in item or any(ch in item for ch in ";&|`$<>") for item in argv):
        return False, "forbidden-token"
    if argv in (["git", "rev-parse", "--verify", "--end-of-options", commit + "^{commit}"],
                ["git", "rev-parse", "--verify", "--end-of-options", commit + "^{tree}"],
                ["git", "ls-tree", "-rz", "--full-tree", commit]):
        return True, "admitted"
    if len(argv) >= 6 and argv[:5] == ["git", "ls-tree", "-rz", "--full-tree", commit] and argv[5] == "--":
        valid = bool(argv[6:]) and all(safe_literal_path(path) for path in argv[6:])
        return valid, "admitted-paths" if valid else "unsafe-path"
    if len(argv) == 6 and argv[:5] == ["git", "show", "--no-ext-diff", "--no-textconv", "--no-renames"]:
        return False, "invalid-show-shape"
    if len(argv) == 7 and argv[:6] == ["git", "show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format="]:
        value = argv[6]
        valid = value.startswith(commit + ":") and safe_literal_path(value[len(commit) + 1:])
        return valid, "admitted-show" if valid else "unsafe-show"
    if len(argv) == 3 and argv[:2] == ["git", "cat-file"] and argv[2] == "--batch-check=%(objectname) %(objecttype) %(objectsize)":
        try:
            decoded = stdin.decode("ascii") if stdin is not None else ""
        except UnicodeDecodeError:
            decoded = ""
        lines = decoded[:-1].split("\n") if decoded.endswith("\n") else []
        valid = bool(lines) and all(HEX.fullmatch(line) and line in object_ids for line in lines)
        return valid, "admitted-batch" if valid else "unsafe-batch-stdin"
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
    write_confined(ROOT, Path(artifact_relative), inventory)
    objects = set(parse_inventory(inventory))
    origin = raw_origin(repo)
    categories = missing_categories()
    record = {
        "schema": "uzel.ref-candidate/v1", "repository": EXPECTED_REPOSITORY, "origin": origin,
        "observed_commit": commit, "observed_tree": tree, "tree_inventory_path": artifact_relative,
        "tree_inventory_sha256": sha256(inventory), "approved_ref_reachability": approved_reachability(repo, commit),
        "pinned_blobs": {path: pinned_blob(repo, commit, path, inventory_paths) for path in ["AGENTS.md", "README.md", ".planning/PROJECT.md", ".planning/ROADMAP.md", ".planning/REQUIREMENTS.md"]},
        "working_tree_evidence": "excluded", "admission_categories": categories,
        "declared_probes": declared_probes(),
        "controlled_runner": controlled_runner(),
        "mutation_snapshots": {"napp_before": before_napp, "uzel_before": before_uzel},
        "missing_categories": [*CATEGORY_NAMES, "declared_executable_probes"], "result": "stop",
        "blocker": BLOCKER, "owner": "jodobear/napp", "next_probe": NEXT_PROBE,
        "rollback": ROLLBACK, "d18_rule": D18_RULE,
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


def require_committed_record(path: Path, expected_relative: str) -> Path:
    """Bind validation to the reviewed record bytes in the current commit."""
    if path.is_absolute():
        try:
            relative = path.resolve().relative_to(ROOT)
        except ValueError as error:
            raise CheckError("record path is outside the Uzel repository") from error
    else:
        relative = path
    resolved = ROOT / relative
    if relative.as_posix() != expected_relative or resolved.is_symlink():
        raise CheckError("record path is not the fixed committed evidence path")
    committed = must_git(ROOT, ["show", "--no-ext-diff", "--no-textconv", "--no-renames",
                                "--format=", "HEAD:" + expected_relative])
    if resolved.read_bytes() != committed:
        raise CheckError("record bytes differ from the reviewed HEAD blob")
    return resolved


def repository_from_origin(origin: str) -> str:
    match = re.fullmatch(r"(?:git@github\.com:|https://github\.com/)([^/]+/[^/]+?)(?:\.git)?", origin)
    if not match:
        raise CheckError("origin is not a canonical GitHub repository URL")
    return match.group(1)


def validate_snapshot(value: Any, expected_root: Path) -> None:
    keys = {"root", "head", "head_bytes_sha256", "raw_index", "index_serialization_sha256", "refs_sha256",
            "status_sha256", "status_guard_sha256", "git_dir", "common_dir", "object_dir", "protected", "objects", "fingerprint"}
    if not isinstance(value, dict) or set(value) != keys or value.get("root") != str(expected_root):
        raise CheckError("mutation snapshot shape/root mismatch")
    unsigned = {key: item for key, item in value.items() if key != "fingerprint"}
    if value["fingerprint"] != sha256(json_bytes(unsigned)):
        raise CheckError("mutation snapshot fingerprint mismatch")


def validate_record(record: dict[str, Any], repo: Path, expected_repository: str,
                    expected_commit: str, expected_result: str) -> None:
    required = {"schema", "repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "pinned_blobs", "working_tree_evidence", "admission_categories", "declared_probes", "controlled_runner", "mutation_snapshots", "missing_categories", "result", "blocker", "owner", "next_probe", "rollback", "d18_rule"}
    if set(record) != required or record.get("schema") != "uzel.ref-candidate/v1":
        raise CheckError("qualification fields are missing or unknown")
    if expected_result != "stop" or expected_repository != EXPECTED_REPOSITORY \
            or record["repository"] != expected_repository or record["observed_commit"] != expected_commit \
            or record["result"] != "stop":
        raise CheckError("qualification identity/result mismatch")
    repo = root_path(repo)
    origin = raw_origin(repo)
    if repository_from_origin(origin) != expected_repository or record["origin"] != origin:
        raise CheckError("qualification repository/origin mismatch")
    tree = must_git(repo, ["rev-parse", "--verify", "--end-of-options", expected_commit + "^{tree}"]).decode().strip()
    if record["observed_tree"] != tree:
        raise CheckError("qualification tree mismatch")
    exact_path = f".artifacts/phase-01/napp/{expected_commit}/tree.bin"
    if record["tree_inventory_path"] != exact_path or not exact_path.endswith("/tree.bin"):
        raise CheckError("tree inventory path is not the fixed .bin path")
    unresolved_artifact = ROOT / exact_path
    current = ROOT
    for component in Path(exact_path).parts:
        current = current / component
        if current.is_symlink():
            raise CheckError("tree inventory path contains a symlink")
    artifact = unresolved_artifact.resolve()
    expected_root = (ROOT / ".artifacts/phase-01/napp").resolve()
    if expected_root not in artifact.parents or artifact.is_symlink() or not artifact.is_file() or stat.S_IMODE(artifact.stat().st_mode) != 0o600:
        raise CheckError("tree inventory confinement/type/mode failed")
    actual = must_git(repo, ["ls-tree", "-rz", "--full-tree", expected_commit])
    if artifact.read_bytes() != actual or record["tree_inventory_sha256"] != sha256(actual):
        raise CheckError("tree inventory bytes or digest mismatch")
    inventory_paths = {entry.partition(b"\t")[2].decode("utf-8", "surrogateescape")
                       for entry in actual.split(b"\0") if entry}
    expected_pinned = {path: pinned_blob(repo, expected_commit, path, inventory_paths)
                       for path in ["AGENTS.md", "README.md", ".planning/PROJECT.md", ".planning/ROADMAP.md", ".planning/REQUIREMENTS.md"]}
    static_expected = {
        "approved_ref_reachability": approved_reachability(repo, expected_commit),
        "pinned_blobs": expected_pinned,
        "working_tree_evidence": "excluded",
        "admission_categories": missing_categories(),
        "declared_probes": declared_probes(),
        "controlled_runner": controlled_runner(),
        "missing_categories": [*CATEGORY_NAMES, "declared_executable_probes"],
        "blocker": BLOCKER, "owner": "jodobear/napp", "next_probe": NEXT_PROBE,
        "rollback": ROLLBACK, "d18_rule": D18_RULE,
    }
    for field, expected in static_expected.items():
        if record.get(field) != expected:
            raise CheckError("qualification provenance mismatch: " + field)
    snapshots = record["mutation_snapshots"]
    if set(snapshots) != {"napp_before", "napp_after", "uzel_before", "uzel_after"}:
        raise CheckError("mutation snapshot set mismatch")
    for name in ("napp_before", "napp_after"):
        validate_snapshot(snapshots[name], repo)
    for name in ("uzel_before", "uzel_after"):
        validate_snapshot(snapshots[name], ROOT)
    if not git_invariants_equal(snapshots["napp_before"], snapshots["napp_after"]) \
            or not git_invariants_equal(snapshots["uzel_before"], snapshots["uzel_after"], allow_declared_outputs=True):
        raise CheckError("undeclared Git mutation detected")


def qualification(args: argparse.Namespace) -> None:
    path = require_committed_record(Path(args.record), QUALIFICATION_PATH)
    record = read_record(path)
    validate_record(record, Path(args.repo), args.expected_repository, args.expected_commit, args.expected_result)
    print("qualification: pass")


def handoff(args: argparse.Namespace) -> None:
    if root_path(Path(args.repo)) != ROOT or args.plan != APPROVED_PLAN_PATH:
        raise CheckError("handoff requires the fixed Uzel root and approved Plan-01 path")
    qualification_path = require_committed_record(Path(args.qualification), QUALIFICATION_PATH)
    handoff_path = require_committed_record(Path(args.handoff), HANDOFF_PATH)
    qualification_record = read_record(qualification_path)
    handoff_record = read_record(handoff_path)
    validate_record(qualification_record, Path(args.napp_repo), EXPECTED_REPOSITORY, EXPECTED_COMMIT, "stop")
    copied = ["repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "admission_categories", "declared_probes", "missing_categories", "working_tree_evidence", "mutation_snapshots", "blocker", "owner", "next_probe", "rollback", "d18_rule"]
    for field in copied:
        if handoff_record.get(field) != qualification_record[field]:
            raise CheckError("handoff field mismatch: " + field)
    required = {"schema", *copied, "result", "plan_contract", "ref_01d_preconditions", "d12_routing",
                "required_napp_deliverables", "exact_heads", "resume_command", "handoff_write_snapshots"}
    if set(handoff_record) != required or handoff_record["schema"] != "uzel.napp-dependency/v1" or handoff_record["result"] != "stop":
        raise CheckError("handoff schema/result mismatch")
    contract = handoff_record["plan_contract"]
    contract_commit = contract.get("commit")
    if not isinstance(contract_commit, str) or not HEX.fullmatch(contract_commit):
        raise CheckError("handoff contract commit is not an exact object id")
    plan_spec = contract_commit + ":" + args.plan
    if must_git(ROOT, ["cat-file", "-t", plan_spec]).decode().strip() != "blob":
        raise CheckError("approved Plan-01 object is not a blob")
    plan_bytes = must_git(ROOT, ["show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", plan_spec])
    plan_oid = must_git(ROOT, ["rev-parse", "--verify", "--end-of-options", plan_spec]).decode().strip()
    if contract != {"path": args.plan, "commit": contract_commit, "blob_oid": plan_oid, "content_sha256": sha256(plan_bytes), "fixed_production_commit": "19519c378c2e775c6ad4b042cfd9aadd89f766b9", "replay_manifest_path": ".artifacts/phase-01/replay/manifest.json", "replay_manifest_status": "pending-plan-01"}:
        raise CheckError("committed Plan-01 parity contract mismatch")
    exact_expected = {
        "ref_01d_preconditions": ref_01d_preconditions(),
        "d12_routing": D12_ROUTING,
        "required_napp_deliverables": required_napp_deliverables(),
        "exact_heads": {"uzel": contract_commit, "napp": EXPECTED_COMMIT},
        "resume_command": RESUME_COMMAND,
    }
    for field, expected in exact_expected.items():
        if handoff_record.get(field) != expected:
            raise CheckError("handoff resume contract mismatch: " + field)
    snapshots = handoff_record["handoff_write_snapshots"]
    if set(snapshots) != {"napp_before", "napp_after", "uzel_before", "uzel_after"}:
        raise CheckError("handoff mutation snapshot set mismatch")
    napp_repo = root_path(Path(args.napp_repo))
    for name in ("napp_before", "napp_after"):
        validate_snapshot(snapshots[name], napp_repo)
    for name in ("uzel_before", "uzel_after"):
        validate_snapshot(snapshots[name], ROOT)
    if not git_invariants_equal(snapshots["napp_before"], snapshots["napp_after"]) \
            or not git_invariants_equal(snapshots["uzel_before"], snapshots["uzel_after"], allow_declared_outputs=True):
        raise CheckError("handoff write caused undeclared Git mutation")
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
                ["git", "show", "$(curl x)"], ["curl", "https://example.invalid"], ["git", "show", "a;b"],
                ["git", "ls-tree", "-rz", "--full-tree", EXPECTED_COMMIT, "--", ":/README.md"],
                ["git", "ls-tree", "-rz", "--full-tree", EXPECTED_COMMIT, "--", "/README.md"],
                ["git", "ls-tree", "-rz", "--full-tree", EXPECTED_COMMIT, "--", "foo//bar"],
                ["git", "cat-file", "a" * 40]]
    before = snapshot(ROOT)
    if not all(candidate_argv_ok(value, EXPECTED_COMMIT, object_ids)[0] for value in positive):
        raise CheckError("literal grammar rejected a positive case")
    if any(candidate_argv_ok(value, EXPECTED_COMMIT, object_ids)[0] for value in negative):
        raise CheckError("literal grammar admitted a negative case")
    if candidate_argv_ok(["git", "cat-file", "--batch-check=%(objectname) %(objecttype) %(objectsize)"], EXPECTED_COMMIT, object_ids, b"a" * 40 + b"\n")[0] is False:
        raise CheckError("literal grammar rejected safe batch form")
    for malformed in (b"a" * 40 + b"\xff\n", b"a" * 40 + b"\r\n", b"a" * 40, b"\n"):
        if candidate_argv_ok(["git", "cat-file", "--batch-check=%(objectname) %(objecttype) %(objectsize)"], EXPECTED_COMMIT, object_ids, malformed)[0]:
            raise CheckError("literal grammar admitted malformed batch stdin")
    with tempfile.TemporaryDirectory(prefix="uzel-confined-write-") as test_root_name:
        test_root = Path(test_root_name)
        target = test_root / "target"
        target.write_bytes(b"preserve")
        os.link(target, test_root / "output")
        try:
            write_confined(test_root, Path("output"), b"changed")
        except CheckError:
            pass
        else:
            raise CheckError("confined writer admitted a hard-linked output")
        if target.read_bytes() != b"preserve":
            raise CheckError("confined writer mutated a hard-link target")
        (test_root / "output").unlink()
        write_confined(test_root, Path("output"), b"atomic")
        if (test_root / "output").read_bytes() != b"atomic":
            raise CheckError("confined writer failed atomic replacement")
    after = snapshot(ROOT)
    if before["fingerprint"] != after["fingerprint"]:
        raise CheckError("self-test changed repository state")
    print("self-test: pass (negative probes rejected before spawn)")


def emit(args: argparse.Namespace) -> None:
    repo = root_path(Path(args.repo))
    if repo != root_path(DEFAULT_NAPP_REPO) or args.expected_commit != EXPECTED_COMMIT \
            or args.record != QUALIFICATION_PATH:
        raise CheckError("qualification writer requires the fixed Napp candidate and evidence path")
    record = candidate_record(repo, args.expected_commit)
    validate_record(record, repo, EXPECTED_REPOSITORY, EXPECTED_COMMIT, "stop")
    target = Path(args.record)
    prefix = "# Napp Candidate Qualification\n\nCommitted-object evidence only; sibling working-tree material is excluded.\n\n"
    def write() -> None:
        write_repo_output(target, (prefix + MARKER_BEGIN + "\n" + canonical_record(record) + "\n" + MARKER_END + "\n").encode())
    write()
    record["mutation_snapshots"]["napp_after"] = snapshot(root_path(Path(args.repo)))
    record["mutation_snapshots"]["uzel_after"] = snapshot(ROOT)
    write()
    if snapshot(root_path(Path(args.repo)))["fingerprint"] != record["mutation_snapshots"]["napp_after"]["fingerprint"] \
            or snapshot(ROOT)["fingerprint"] != record["mutation_snapshots"]["uzel_after"]["fingerprint"]:
        raise CheckError("qualification evidence write did not stabilize")
    print("qualification record written: " + str(target))


def emit_handoff(args: argparse.Namespace) -> None:
    if args.plan != APPROVED_PLAN_PATH or args.qualification != QUALIFICATION_PATH \
            or args.handoff != HANDOFF_PATH:
        raise CheckError("handoff writer requires the approved Plan-01 and evidence paths")
    qualification_record = read_record(Path(args.qualification))
    napp_repo = root_path(Path(args.napp_repo))
    if napp_repo != root_path(DEFAULT_NAPP_REPO):
        raise CheckError("handoff writer requires the fixed Napp repository")
    validate_record(qualification_record, napp_repo, EXPECTED_REPOSITORY, EXPECTED_COMMIT, "stop")
    write_snapshots = {"napp_before": snapshot(napp_repo), "uzel_before": snapshot(ROOT)}
    plan_path = args.plan
    plan_bytes = must_git(ROOT, ["show", "--no-ext-diff", "--no-textconv", "--no-renames", "--format=", "HEAD:" + plan_path])
    plan_oid = must_git(ROOT, ["rev-parse", "--verify", "--end-of-options", "HEAD:" + plan_path]).decode().strip()
    if must_git(ROOT, ["cat-file", "-t", "HEAD:" + plan_path]).decode().strip() != "blob":
        raise CheckError("approved Plan-01 object is not a blob")
    copied = ["repository", "origin", "observed_commit", "observed_tree", "tree_inventory_path", "tree_inventory_sha256", "approved_ref_reachability", "admission_categories", "declared_probes", "missing_categories", "working_tree_evidence", "mutation_snapshots", "blocker", "owner", "next_probe", "rollback", "d18_rule"]
    record = {field: qualification_record[field] for field in copied}
    record.update({
        "schema": "uzel.napp-dependency/v1", "result": qualification_record["result"],
        "plan_contract": {"path": plan_path, "commit": must_git(ROOT, ["rev-parse", "HEAD"]).decode().strip(), "blob_oid": plan_oid,
                          "content_sha256": sha256(plan_bytes), "fixed_production_commit": "19519c378c2e775c6ad4b042cfd9aadd89f766b9",
                          "replay_manifest_path": ".artifacts/phase-01/replay/manifest.json", "replay_manifest_status": "pending-plan-01"},
        "ref_01d_preconditions": ref_01d_preconditions(),
        "d12_routing": D12_ROUTING,
        "required_napp_deliverables": required_napp_deliverables(),
        "exact_heads": {"uzel": must_git(ROOT, ["rev-parse", "HEAD"]).decode().strip(), "napp": qualification_record["observed_commit"]},
        "resume_command": RESUME_COMMAND,
        "handoff_write_snapshots": write_snapshots,
    })
    target = Path(args.handoff)
    prefix = "# Napp Dependency Handoff\n\nOwned by `jodobear/napp`; consumed by `jodobear/uzel`. This is a candidate/pending stop, not publication authority.\n\n"
    def write() -> None:
        write_repo_output(target, (prefix + MARKER_BEGIN + "\n" + canonical_record(record) + "\n" + MARKER_END + "\n").encode())
    write()
    write_snapshots["napp_after"] = snapshot(napp_repo)
    write_snapshots["uzel_after"] = snapshot(ROOT)
    write()
    if snapshot(napp_repo)["fingerprint"] != write_snapshots["napp_after"]["fingerprint"] \
            or snapshot(ROOT)["fingerprint"] != write_snapshots["uzel_after"]["fingerprint"]:
        raise CheckError("handoff evidence write did not stabilize")
    print("handoff record written: " + str(target))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    test = commands.add_parser("self-test"); test.set_defaults(func=self_test)
    write = commands.add_parser("write-qualification"); write.add_argument("--repo", required=True); write.add_argument("--expected-commit", required=True); write.add_argument("--record", required=True); write.set_defaults(func=emit)
    write_handoff = commands.add_parser("write-handoff"); write_handoff.add_argument("--qualification", required=True); write_handoff.add_argument("--handoff", required=True); write_handoff.add_argument("--plan", required=True); write_handoff.add_argument("--napp-repo", default=str(DEFAULT_NAPP_REPO)); write_handoff.set_defaults(func=emit_handoff)
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
