#!/usr/bin/env bash
# Print the gulp tasks that package Linux x64 for the pinned VS Code checkout (helps when a VS Code version renames tasks).
set -euo pipefail
WORK="${WORK:-$HOME/dream-vsc}"
export PATH="$WORK/node/bin:$PATH"
cd "$WORK/vscode"
npm run -s gulp -- --tasks-simple 2>/dev/null | tr -d '\r' | grep -E "linux-x64|^compile|^minify|^optimize|^core-ci|^extensions-ci|^compile-build" | sort
