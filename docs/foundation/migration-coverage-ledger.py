#!/usr/bin/env python3
"""Rebuild the migration coverage ledger from pinned Git trees without executing assets.

Usage: python3 docs/foundation/migration-coverage-ledger.py
The original text census stays pinned; changed main blobs are marked for re-review.
"""
from __future__ import annotations

import gzip
import hashlib
import json
import subprocess
from collections import Counter
from pathlib import Path

BASE = "59d174ef032a7cd6cb82edfa2fe093c30d4fa138"
MAIN = "1e01c4edd7bcda876b6b9c72f6888f0782fd39a7"
ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = Path(__file__).resolve().parent


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(ROOT), *args])


def tree(commit: str) -> dict[str, str]:
    result = {}
    for line in git("ls-tree", "-r", "-z", commit).split(b"\0"):
        if not line:
            continue
        metadata, path = line.split(b"\t", 1)
        mode, kind, blob = metadata.decode("ascii").split()
        assert kind == "blob" and mode in {"100644", "100755", "120000"}
        result[path.decode("utf-8")] = blob
    return result


def load_inventory() -> list[dict]:
    with gzip.open(EVIDENCE / "2026-10-26-migration-identifier-inventory-v3.jsonl.gz", "rt") as stream:
        return [json.loads(line) for line in stream]


def main() -> None:
    original, current = tree(BASE), tree(MAIN)
    findings = load_inventory()
    binary = json.loads((EVIDENCE / "2026-10-26-migration-binary-candidates.json").read_text())["candidates"]
    matched = {row["file"]: row for row in findings}
    binaries = {row["path"]: row for row in binary}
    assert len(original) == 2302 and len(current) == 2303
    assert len(matched) == 1084 and len(binaries) == 110
    assert not (set(matched) & set(binaries))
    assert set(matched) | set(binaries) <= set(original)
    assert all(original[path] == row["blob_sha"] for path, row in binaries.items())
    entries = []
    extra = 0
    for path, blob in sorted(current.items()):
        source_blob = original.get(path)
        if path in matched:
            row = matched[path]
            assert row["source_commit"] == BASE
            item = {"id": row["finding_id"], "segment": "matched-text", "classification": row["classification"],
                    "review": row["review_status"], "source_commit": BASE, "lines": row["lines"],
                    "evidence": row["supporting_evidence"]}
        elif path in binaries:
            row = binaries[path]
            item = {"id": "BINARY-" + row["blob_sha"][:16], "segment": "binary-lead",
                    "classification": "UNKNOWN", "review": row["review_status"], "source_commit": BASE,
                    "evidence": "filename/magic-byte triage only; contents not approved"}
        elif source_blob is not None:
            item = {"id": "NONMATCH-" + hashlib.sha256(path.encode("utf-8")).hexdigest()[:24],
                    "segment": "nonmatching-tracked", "classification": "UNKNOWN", "review": "NOT_TESTED",
                    "source_commit": BASE, "evidence": "no match in pinned text grep; not semantic clearance"}
        else:
            extra += 1
            item = {"id": f"MAIN-ADDED-{extra:04d}", "segment": "added-on-main",
                    "classification": "UNKNOWN", "review": "NOT_TESTED", "source_commit": MAIN,
                    "evidence": "not present on original baseline; direct source review pending"}
        if path.startswith(("apps/", "crates/", "scripts/", ".github/", "registry/", "xtask/")):
            risk = "high-priority operational lead; unverified unless source-reviewed"
        else:
            risk = "untriaged; documentation/fixture/assets may still be operational"
        entries.append({"path": path, "main_blob": blob, "base_blob": source_blob,
                        "reinspection_required": source_blob != blob, "risk": risk,
                        "owner": "nearest applicable AGENTS.md", "disposition": "review before migration acceptance" if item["classification"] == "UNKNOWN" else "v3 limited scope only", **item})
    assert len(entries) == len(current) and len({e["id"] for e in entries}) == len(entries)
    # Descriptors are NOT harvested assets or an executed release. Five candidate
    # routes plus the two tested browser image routes are separately unverified.
    artifacts = [
        {"id": "ARTIFACT-" + target.upper().replace("-", "_"), "target": target,
         "archive": archive, "status": "UNKNOWN for any independent final-source candidate",
         "source": ".github/workflows/release.yml; scripts/jarvis_package.py; scripts/jarvis_release_candidate.py",
         "required": ["archive", "archive.sha256", "archive.candidate.json", "archive.accepted.json", "manifest.json"]}
        for target, archive in (
            ("linux-x86_64", "jarvis-linux-x86_64.tar.gz"),
            ("linux-aarch64", "jarvis-linux-aarch64.tar.gz"),
            ("darwin-x86_64", "jarvis-darwin-x86_64.tar.gz"),
            ("darwin-aarch64", "jarvis-darwin-aarch64.tar.gz"),
            ("windows-x86_64", "jarvis-windows-x86_64.zip"),
        )
    ]
    artifacts.extend({"id": "DRIVER-" + arch.upper(), "archive": f"jarvis-web-driver-{arch}.tar.gz",
                      "status": "UNKNOWN for independent final-source candidate",
                      "source": ".github/workflows/web-driver.yml; scripts/jarvis_release_candidate.py"}
                     for arch in ("amd64", "arm64"))
    counts = dict(Counter(e["segment"] for e in entries))
    classifications = dict(Counter(e["classification"] for e in entries))
    result = {"schema": 1, "original_commit": BASE, "current_main_commit": MAIN,
              "counts": counts, "classifications": classifications,
              "limitations": ["current audit-branch-only files excluded; main tracked tree enumerated exactly",
                              "binary/archives not unpacked; output names are unexecuted descriptors",
                              "nonmatching is not migration clearance; modified main blobs require reinspection",
                              "generated installation/runtime user state and live-provider accounts not accessed"],
              "tracked_files": entries, "generated_package_routes": artifacts}
    assert counts == {"nonmatching-tracked": 1108, "matched-text": 1084, "binary-lead": 110, "added-on-main": 1}, counts
    output = EVIDENCE / "2026-10-26-migration-coverage-ledger.json"
    output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(f"coverage={len(entries)} segments={counts} classes={classifications} generated_routes={len(artifacts)}")


if __name__ == "__main__":
    main()
