#!/usr/bin/env bash
# Build "DreamOS Code": Code-OSS (the MIT-licensed open-source core of VS Code) for Linux x64, with the
# open-vsc-claude extension bundled as a built-in extension.  Run inside Linux or WSL2 (not on /mnt/*: use a native disk).
#
#   ./build.sh                       # everything, resumable (finished stages are skipped)
#   VSCODE_TAG=1.142.0 ./build.sh    # pin another VS Code release
#   WORK=$HOME/dream-vsc ./build.sh  # build directory (needs ~15 GB free, 8+ GB RAM)
#
# Stages: deps -> node -> clone -> install (npm ci) -> package (gulp) -> bundle (extension + product overlay)
set -euo pipefail

VSCODE_TAG="${VSCODE_TAG:-1.142.0}"
NODE_VERSION="${NODE_VERSION:-24.18.0}"   # = .nvmrc of the pinned VS Code tag
WORK="${WORK:-$HOME/dream-vsc}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXT_VSIX="${EXT_VSIX:-}"                  # path to open-vsc-claude-<version>.vsix (default: newest next to this repo, else GitHub release)
SRC="$WORK/vscode"
OUT="$WORK/VSCode-linux-x64"
log() { printf '\n==> %s\n' "$*"; }
stamp() { touch "$WORK/.done-$1"; }
done_() { [ -f "$WORK/.done-$1" ]; }

case "$WORK" in /mnt/*) echo "WORK must be on a native Linux disk, not $WORK" >&2; exit 1;; esac
mkdir -p "$WORK"

# ---- 1. system packages (only checked here; installing needs root)
log "checking system packages"
missing=()
for p in libx11-dev libxkbfile-dev libsecret-1-dev libkrb5-dev pkg-config make g++ python3 git curl xz-utils; do
  dpkg -s "$p" >/dev/null 2>&1 || missing+=("$p")
done
if [ "${#missing[@]}" -gt 0 ]; then
  echo "missing packages: ${missing[*]}" >&2
  echo "install with:  sudo apt-get install -y ${missing[*]}   (WSL from Windows: wsl -u root apt-get install -y ${missing[*]})" >&2
  exit 1
fi

# ---- 2. Node.js (user-level, does not touch the system Node)
if ! done_ node; then
  log "installing Node.js $NODE_VERSION"
  curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" -o "$WORK/node.tar.xz"
  rm -rf "$WORK/node" && mkdir -p "$WORK/node" && tar -xJf "$WORK/node.tar.xz" -C "$WORK/node" --strip-components=1
  rm -f "$WORK/node.tar.xz"; stamp node
fi
export PATH="$WORK/node/bin:$PATH"
node -v

# ---- 3. source
if ! done_ clone; then
  log "cloning microsoft/vscode @ $VSCODE_TAG"
  rm -rf "$SRC"
  git clone --depth 1 --branch "$VSCODE_TAG" https://github.com/microsoft/vscode.git "$SRC"
  stamp clone
fi

# ---- 4. dependencies (builds native modules; the longest download)
if ! done_ install; then
  log "npm ci (this takes a while)"
  (cd "$SRC" && npm ci)
  stamp install
fi

# ---- 5. package
if ! done_ package; then
  log "packaging with gulp"
  export NODE_OPTIONS="--max-old-space-size=8192"
  cd "$SRC"
  tasks="$(npm run -s gulp -- --tasks-simple 2>/dev/null | tr -d '\r')"
  task=""
  # NOT the *-ci variants: they assume core-ci / extensions-ci / minify already ran (fails with ENOENT out-vscode-min/...).
  # The non-ci task runs the whole pipeline (compile-build, optimize, minify, package).
  for t in vscode-linux-x64-min vscode-linux-x64; do
    if printf '%s\n' "$tasks" | grep -qx "$t"; then task="$t"; break; fi
  done
  [ -n "$task" ] || { echo "no linux-x64 packaging task found in this VS Code version" >&2; exit 1; }
  echo "gulp task: $task"
  npm run gulp "$task"
  [ -d "$OUT" ] || { echo "expected output $OUT not found" >&2; exit 1; }
  stamp package
fi

# ---- 6. bundle open-vsc-claude + branding overlay
log "bundling open-vsc-claude and applying the product overlay"
if [ -z "$EXT_VSIX" ]; then
  EXT_VSIX="$(ls -t "$HERE"/../../open-vsc-claude/open-vsc-claude-*.vsix /mnt/f/open-vsc-claude/open-vsc-claude-*.vsix 2>/dev/null | head -1 || true)"
fi
if [ -z "$EXT_VSIX" ] || [ ! -f "$EXT_VSIX" ]; then
  url="$(curl -fsSL https://api.github.com/repos/aon-co-jp/open-vsc-claude/releases/latest | python3 -c 'import json,sys; print([a["browser_download_url"] for a in json.load(sys.stdin)["assets"] if a["name"].endswith(".vsix")][0])')"
  EXT_VSIX="$WORK/open-vsc-claude.vsix"; curl -fsSL "$url" -o "$EXT_VSIX"
fi
echo "extension package: $EXT_VSIX"
tmp="$(mktemp -d)"; python3 -m zipfile -e "$EXT_VSIX" "$tmp"
EXT_DIR="$OUT/resources/app/extensions/open-vsc-claude"
rm -rf "$EXT_DIR"; mkdir -p "$EXT_DIR"; cp -r "$tmp/extension/." "$EXT_DIR/"; rm -rf "$tmp"
node "$HERE/apply-overlay.js" "$OUT/resources/app/product.json" "$HERE/product-overlay.json"

log "done: $OUT  (run ./verify.sh, or $OUT/bin/code-oss)"
