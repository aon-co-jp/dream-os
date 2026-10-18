#!/usr/bin/env bash
# Debug helper: show where the pinned VS Code wires the built-in Copilot packaging steps (usage: inspect.sh)
set -euo pipefail
WORK="${WORK:-$HOME/dream-vsc}"
cd "$WORK/vscode"
echo "== uses of the shim task / copilot in gulpfile.vscode.ts"
grep -n -i "copilot" build/gulpfile.vscode.ts | head -40
echo "== source copilot package"
ls extensions/copilot/node_modules/@github/copilot 2>&1 | head -12
echo "== output copilot extension"
ls "$WORK/VSCode-linux-x64/resources/app/extensions/copilot" 2>&1 | head -12
ls "$WORK/VSCode-linux-x64/resources/app/extensions/copilot/node_modules/@github" 2>&1 | head
echo "== product.json copilot refs"
grep -n -i "copilot" product.json | head -10
