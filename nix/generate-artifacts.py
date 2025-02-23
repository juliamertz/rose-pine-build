#!/usr/bin/env python3

import subprocess
import requests

repo = "juliamertz/rose-pine-build"
response = requests.get("https://api.github.com/repos/{}/releases/latest".format(repo))
latest_tag = response.json().get("tag_name")

# Map nix system strings to cargo targets
systems = {
    "x86_64-linux": "x86_64-unknown-linux-gnu", 
    "aarch64-linux": "aarch64-unknown-linux-gnu",
    "x86_64-darwin": "x86_64-apple-darwin",
    "aarch64-darwin": "aarch64-apple-darwin",
}

def make_artifact_name(system: str, ext: str) -> str:
    template = "rose-pine-build-{}.{}"
    return template.format(systems[system], ext)

def make_url(repo: str, tag: str, system: str) -> str:
    template = "https://github.com/{repo}/releases/download/{tag}/{filename}"
    return template.format(
        filename = make_artifact_name(system, "tar.gz"),
        repo = repo,
        tag = tag,
    )

def prefetch_hash(url: str) -> str:
    result = subprocess.run(["nix-prefetch-url", url], capture_output=True, text=True, check=True)
    return result.stdout.strip()

def make_fetcher(url: str, sha256: str) -> str:
    fetcher = '''fetchurl {{ url = "{url}"; sha256 = "{sha256}"; }}'''
    return fetcher.format(url = url, sha256 = sha256)

outputs = []

for system, _ in systems.items():
    raw_url = make_url(repo, latest_tag, system)
    hash = prefetch_hash(raw_url)

    url = make_url("${repo}", "${tag}", system)
    fetcher = make_fetcher(url, hash)
    outputs.append(system + " = " + fetcher + ";")

template = '''{{ fetchurl, ... }}: let repo = "{repo}"; tag = "{tag}"; in {{ {outputs} }}'''
output = template.format(
    repo = repo,
    tag = latest_tag,
    outputs = "\n".join(outputs)
)

print(output)
