# BUILD-REPORT (DreamOS Code / Code-OSS)

## 2026-10-18 時点 / Status as of 2026-10-18

**日本語**
- 環境: WSL2 Ubuntu 26.04(32コア、RAM 15GB、ext4に936GB空き)。依存は `wsl -u root apt-get install -y ...` で導入済み(パスワード不要)。
- `build.sh` の進捗(`~/dream-vsc/.done-*`): **node ✅ / clone ✅(`microsoft/vscode` 1.142.0)/ install ✅(`npm ci`)/ package ❌ / bundle ❌**。
- 1回目の `package`: `vscode-linux-x64-min-ci` を選んだため `out-vscode-min/...` が無く `ENOENT` で失敗(**`-ci` 系は事前工程が前提**)。`build.sh` を `vscode-linux-x64-min` 優先に修正した。
- 2回目の `package`(約12分): compile・bundle・minify・**`package-linux-x64` まで成功**(出力 `~/dream-vsc/VSCode-linux-x64`)。その後の最終工程 `prepareBuiltInCopilotRipgrepShim` が
  `Copilot SDK directory not found at .../extensions/copilot/node_modules/@github/copilot/sdk` で失敗して終了。**原因は未調査**(`inspect.sh` を作ったが結果は未取得)。
- 未実行: `open-vsc-claude` の同梱(bundle 段階)、`verify.sh`。したがって「ビルドできた」とはまだ言えない。
- 方針の案(原因を確かめてから選ぶ): DreamOS Code は GitHub Copilot を同梱する必要が無いので、Copilot の組み込みと最終工程を外す。
- 再開: `build.sh` を再実行すると `package` 段階から(`vscode-linux-x64-min`)やり直す。出力が既にあるので、確認用に `inspect.sh` を先に見るとよい。

**English**
- Environment: WSL2 Ubuntu 26.04 (32 cores, 15 GB RAM, 936 GB free on ext4). Dependencies installed with `wsl -u root apt-get install -y ...` (no password needed).
- `build.sh` progress (`~/dream-vsc/.done-*`): **node ✅ / clone ✅ (`microsoft/vscode` 1.142.0) / install ✅ (`npm ci`) / package ❌ / bundle ❌**.
- First `package` run: picked `vscode-linux-x64-min-ci`, which needs `out-vscode-min/...` from earlier stages and failed with `ENOENT` (**the `-ci` tasks assume earlier stages**). `build.sh` now prefers `vscode-linux-x64-min`.
- Second `package` run (about 12 min): compile, bundle, minify and **`package-linux-x64` succeeded** (output `~/dream-vsc/VSCode-linux-x64`). The final step `prepareBuiltInCopilotRipgrepShim` then failed with
  `Copilot SDK directory not found at .../extensions/copilot/node_modules/@github/copilot/sdk`. **Cause not investigated** (`inspect.sh` was written but its result was never read).
- Not run: bundling `open-vsc-claude` (bundle stage) and `verify.sh`. So do not claim it builds yet.
- Idea (choose after confirming the cause): DreamOS Code does not need to ship GitHub Copilot, so drop the built-in Copilot and that final step.
- Resume: running `build.sh` again redoes the `package` stage (`vscode-linux-x64-min`). Output already exists, so read `inspect.sh` output first.

## 再開コマンド / Resume commands (PowerShell)

```powershell
wsl -d Ubuntu -- bash /mnt/f/dream-os/vsc/build.sh
wsl -d Ubuntu -- bash /mnt/f/dream-os/vsc/verify.sh      # needs xvfb: wsl -u root apt-get install -y xvfb
```

## 落とし穴 / Pitfalls

- **Git Bash はパスを書き換える**: `wsl ... /mnt/f/...` が `C:/Program Files/Git/mnt/f/...` になる。PowerShell から実行するか `MSYS_NO_PATHCONV=1` を付ける。 / *Git Bash rewrites `/mnt/...` paths; use PowerShell or `MSYS_NO_PATHCONV=1`.*
- PowerShell の引用符内の `|` や `$変数` は WSL に渡る前に解釈される。複雑な処理は `.sh` ファイルに書いて実行する。 / *Pipes and `$vars` inside quoted PowerShell strings are interpreted before reaching WSL; put logic in a `.sh` file.*
- パッケージ化のタスク名は VS Code の版で変わりうる。`build.sh` は `gulp --tasks-simple` から探すが、**この 1.142.0 で実際に通るかは未確認**。失敗したらログ末尾のタスク名を確認する。 / *The gulp task name may differ per VS Code version; `build.sh` discovers it, but it is unconfirmed on 1.142.0.*
- メモリ: `NODE_OPTIONS=--max-old-space-size=8192`。RAM 15GB で足りるかは未確認。 / *Memory sufficiency on 15 GB is unconfirmed.*
