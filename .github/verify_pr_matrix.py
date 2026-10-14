"""Read-only, success-only admission of the complete exact-head Jarvis PR matrix."""
import json
import os
import time
import urllib.request

EXPECTED = {
    "ci.yml": {"quality", "windows-installer", "supply-chain"},
    "msrv.yml": {"rust-1-88"},
    "platform-foundation.yml": {
        "linux-x86_64", "linux-arm64", "macos-intel",
        "macos-apple-silicon", "windows-x86_64",
    },
    "web-driver.yml": {
        "Contained browser linux-x86_64", "Contained browser linux-aarch64",
        "Native namespace Hive",
    },
}


class Refused(Exception):
    """Evidence cannot authorize a merge."""


def successful_jobs(run, jobs, expected, sha, workflow):
    if (run.get("head_sha") != sha or run.get("event") != "pull_request"
            or run.get("path") != ".github/workflows/" + workflow):
        raise Refused("wrong commit, event or workflow")
    if run.get("status") != "completed" or run.get("conclusion") != "success":
        raise Refused("workflow is not complete and successful")
    names = [job.get("name") for job in jobs]
    if len(names) != len(expected) or set(names) != expected:
        raise Refused("missing, duplicate or unexpected matrix job")
    if any(job.get("status") != "completed" or job.get("conclusion") != "success"
           or job.get("head_sha") != sha or job.get("run_id") != run["id"]
           for job in jobs):
        raise Refused("job is missing exact-head provenance or success")


def main():
    repo = os.environ["GITHUB_REPOSITORY"]
    if repo != "99percentgrip/jarvis":
        raise Refused("unexpected repository")
    sha = os.environ["PR_HEAD_SHA"]
    if len(sha) != 40 or any(c not in "0123456789abcdef" for c in sha):
        raise Refused("invalid exact head")
    number = int(os.environ["PR_NUMBER"])
    token = os.environ["GH_TOKEN"]

    def fetch(path):
        request = urllib.request.Request(
            "https://api.github.com/repos/" + repo + "/" + path,
            headers={"Authorization": "Bearer " + token,
                     "Accept": "application/vnd.github+json",
                     "X-GitHub-Api-Version": "2022-11-28"},
        )
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)

    def inventory(path, key):
        rows = []
        for page in range(1, 101):
            data = fetch(path + ("&" if "?" in path else "?")
                         + "per_page=100&page=" + str(page))
            rows.extend(data[key])
            if len(rows) >= data["total_count"]:
                return rows
            if not data[key]:
                raise Refused("incomplete API pagination")
        raise Refused("inventory exceeded bound")

    def latest(workflow):
        rows = inventory("actions/workflows/" + workflow
                         + "/runs?event=pull_request&head_sha=" + sha,
                         "workflow_runs")
        rows = [r for r in rows if r.get("head_sha") == sha
                and r.get("event") == "pull_request"
                and r.get("path") == ".github/workflows/" + workflow
                and any(p.get("number") == number for p in r.get("pull_requests", []))]
        return max(rows, key=lambda r: r["id"]) if rows else None

    deadline = time.monotonic() + 170 * 60
    while time.monotonic() < deadline:
        pr = fetch("pulls/" + str(number))
        if pr["head"]["sha"] != sha or pr["base"]["repo"]["full_name"] != repo:
            raise Refused("PR head or base changed")
        admitted = {}
        pending = []
        for workflow, expected in EXPECTED.items():
            run = latest(workflow)
            if not run or run.get("status") != "completed":
                pending.append(workflow)
                continue
            jobs = inventory("actions/runs/" + str(run["id"]) + "/attempts/"
                             + str(run["run_attempt"]) + "/jobs", "jobs")
            successful_jobs(run, jobs, expected, sha, workflow)
            current = fetch("actions/runs/" + str(run["id"]))
            if (current["run_attempt"] != run["run_attempt"]
                    or current["conclusion"] != "success"):
                raise Refused("workflow attempt changed")
            admitted[workflow] = (run["id"], run["run_attempt"])
        if not pending:
            for workflow, identity in admitted.items():
                run = latest(workflow)
                if (not run or (run["id"], run["run_attempt"]) != identity
                        or run["status"] != "completed" or run["conclusion"] != "success"):
                    raise Refused("newer or changed workflow evidence")
            if fetch("pulls/" + str(number))["head"]["sha"] != sha:
                raise Refused("head changed during verification")
            print(json.dumps({"head_sha": sha, "pr": number, "runs": admitted,
                              "jobs": 12, "result": "success"}))
            return
        print("Waiting for exact-head workflows: " + ", ".join(pending), flush=True)
        time.sleep(30)
    raise Refused("required workflows remain missing or incomplete")


if __name__ == "__main__":
    main()
