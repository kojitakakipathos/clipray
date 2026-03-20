#!/usr/bin/env bash
# Bump Clipray release version in package.json, package-lock.json (via npm),
# src-tauri/Cargo.toml ([package] only), and src-tauri/tauri.conf.json.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

usage() {
  echo "Usage: $0 <major|minor|patch|X.Y.Z>" >&2
  echo "  Uses npm version for package.json and package-lock.json, then aligns Tauri / Cargo." >&2
  exit 1
}

[[ $# -eq 1 ]] || usage

ARG="$1"

case "$ARG" in
  major | minor | patch)
    npm version "$ARG" --no-git-tag-version
    ;;
  *)
    if [[ ! "$ARG" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
      echo "Error: invalid version '$ARG' (use major, minor, patch, or semver X.Y.Z)" >&2
      exit 1
    fi
    npm version "$ARG" --no-git-tag-version --allow-same-version
    ;;
esac

NEW="$(node -p "require('./package.json').version")"
export CLIPRAY_NEW_VERSION="$NEW"

node <<'NODE'
const fs = require("fs");

const v = process.env.CLIPRAY_NEW_VERSION;
if (!v || !/^\d+\.\d+\.\d+$/.test(v)) {
  console.error("CLIPRAY_NEW_VERSION is invalid");
  process.exit(1);
}

function updateCargoToml(filePath) {
  let text = fs.readFileSync(filePath, "utf8");
  const nl = text.includes("\r\n") ? "\r\n" : "\n";
  const lines = text.split(/\r?\n/);
  let inPackage = false;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (/^\[package\]\s*$/.test(line)) {
      inPackage = true;
      continue;
    }
    if (/^\[/.test(line)) {
      inPackage = false;
      continue;
    }
    if (inPackage && /^version\s*=/.test(line)) {
      lines[i] = line.replace(/^version\s*=\s*"[^"]*"/, `version = "${v}"`);
      fs.writeFileSync(filePath, lines.join(nl));
      return;
    }
  }
  console.error(`Could not find [package] version in ${filePath}`);
  process.exit(1);
}

function updateTauriConf(filePath) {
  let text = fs.readFileSync(filePath, "utf8");
  const nl = text.includes("\r\n") ? "\r\n" : "\n";
  const data = JSON.parse(text);
  data.version = v;
  const out = JSON.stringify(data, null, 2) + nl;
  fs.writeFileSync(filePath, out.replace(/\n/g, nl));
}

updateCargoToml("src-tauri/Cargo.toml");
updateTauriConf("src-tauri/tauri.conf.json");
NODE

echo "Version set to ${NEW}"
