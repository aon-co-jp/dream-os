# PORTING.md — dream-os お引越しファイル / Porting notes

> 対象バージョン: 構想段階(2026-08-06、コード未着手)。/ Target version:
> concept stage (2026-08-06, no code yet).

## 0. このリポジトリのスコープ / Scope of this repository

日本語: `dream-os`はWindows・macOS・Android・各種Linux向け(将来的には
SONY PlayStation 5/6・Nintendo Switch 2)のアプリケーションサービス層/
GPU抽象化層の構想リポジトリです。現時点でコードは一切ありません。詳細は
[`CLAUDE.md`](CLAUDE.md)・[`README.md`](README.md)参照。

English: `dream-os` is a concept-stage repository for an application
service layer / GPU abstraction layer targeting Windows, macOS, Android,
and various Linux distributions (with Sony PlayStation 5/6 and Nintendo
Switch 2 as future goals). No code exists yet. See [`CLAUDE.md`](CLAUDE.md)
and [`README.md`](README.md) for details.

## 1. 持っていくもの(ファイル一覧) / Files to bring along

```
dream-os/
├── README.md          # 日本語(主言語) / Japanese (primary language)
├── README-English.md  # 英語 / English
├── CLAUDE.md           # 開発方針・エコシステム内の位置づけ・HANDOFF
├── PORTING.md          # 本ファイル / this file
├── Cargo.toml           # ワークスペース定義(2026-08-06新設)
└── crates/
    └── dream-os-kernel/  # PoC: open-cudaのopencuda-vulkanを再利用した
                           # Windows/Android共通実行基盤+電力出力調整可能な
                           # マイニングOS向けデューティサイクル制御
        ├── Cargo.toml     # ../../../open-cuda/crates/opencuda-* へのpath依存
        ├── src/{lib,power_profile,mining,sbm,directx_bridge,
        │        flash_attention_bridge}.rs
        │        # flash_attention_bridge.rs(2026-08-08新設): open-cudaの
        │        # 融合flash attention SPIR-Vカーネルをdream-osの共通Vulkan
        │        # 実行基盤経由で呼び出す、Windows GT730・Android Adreno 619
        │        # 両実機で実証済み(8×16 toyサイズPoC、GPT-2スケールの
        │        # ベンチマークは未実施)
        ├── shaders/{vector_add,sha256d_mine,sbm_ising}.{comp,spv}
        ├── examples/{dispatch_throttled,mine_benchmark,sbm_benchmark,
        │            directx_bridge_benchmark,dream_os_status}.rs
        │            # sbm_benchmark/directx_bridge_benchmark(2026-08-06新設):
        │            # cargo testが使えないAndroidクロスビルド環境向けに、
        │            # adb pushして直接実行できる単体バイナリとして実装
        │            # (Windows GT730・Android Adreno 619両方で実機検証済み)
        └── tests/{mining_real_vulkan,sbm_real_vulkan,directx_bridge_real_vulkan,
        │         flash_attention_bridge_real_vulkan}.rs
        # directx_bridge.rsは../../../open-directx/crates/
        # directx-shader-translate へのpath依存を追加で持つ
    └── dream-os-raid-bridge/  # open-raid-zのRAID6/Z2実装ブリッジ(2026-08-08新設)
        │                # open_raid_z_core::vdev::RaidZVdev(GF(2^8)
        │                # Reed-Solomon)+zfs_accel_hlsl(D3D12/DirectML・
        │                # Vulkan GPU/NPUパリティ高速化)への薄いpath依存
        ├── Cargo.toml   # ../../../open-raid-z/open_runo_zfs_source/
        │                # open_raid_z_core へのpath依存
        ├── src/lib.rs
        ├── examples/raid6_bridge_benchmark.rs
        │            # 1コマンドで動くrunnableデモ(2026-08-08新設)。
        │            # 実GT730でGPUアクセラレータ(CPUフォールバックでない)・
        │            # 8ストライプ往復・1台破損からの自己修復を確認済み。
        │            # 実機複数NVMe(4〜16枚)検証・Androidクロスビルドは
        │            # 未実施(正直な開示、次回課題)。
        └── tests/raid6_bridge_real.rs
    └── dream-os-wire/  # World Laboratory向け通信・永続化層
        │                # (open-web-server-wireのSecureChannel再利用=
        │                # AEAD暗号化+リプレイ対策、aruaru-db永続化)
        ├── Cargo.toml   # ../../../open-web-server/crates/open-web-server-*
        │                # (さらにopen-web-server-wireは
        │                # ../../../RS-SmartTCPへのpath依存を持つ)
        ├── src/{lib,aruaru_persistence}.rs
        └── tests/{secure_channel_integration,aruaru_db_integration}.rs
        # aruaru_db_integration.rsはDREAM_OS_ARUARU_PG_CONN環境変数で
        # 指定したaruaru-server(pgwire)インスタンスへの実接続が必要
        # (未設定/到達不能時は正直にスキップする設計)
```

`crates/dream-os-kernel`は`open-cuda`(`../../open-cuda`、同じ`F:\runo`
直下にcloneされている前提)・`open-directx`(`../../open-directx`、
同様)の2つへのpath依存を持つ。`crates/dream-os-wire`は`open-web-server`
(`../../../open-web-server`)へのpath依存を持ち、さらに
`open-web-server-wire`自体が`RS-SmartTCP`(`../../../RS-SmartTCP`、
`open-web-server`から見た相対位置)へのpath依存を持つ。`dream-os-wire`の
`aruaru_persistence`モジュールは実行時に`aruaru-db`(`aruaru-server`
バイナリ、pgwire)への接続を必要とする(path依存ではなくネットワーク
接続、`DREAM_OS_ARUARU_PG_CONN`環境変数で接続先を指定)。移設する場合は
`open-cuda`・`open-directx`・`open-web-server`・`RS-SmartTCP`をすべて
同じ相対位置関係でcloneしておくこと。 / `crates/dream-os-kernel` has path
dependencies on `open-cuda` (`../../open-cuda`, assumed cloned alongside
this repo under `F:\runo`) and `open-directx` (`../../open-directx`,
likewise). `crates/dream-os-wire` has a path dependency on
`open-web-server` (`../../../open-web-server`), which in turn has a path
dependency on `RS-SmartTCP` (`../../../RS-SmartTCP`, relative to
`open-web-server`). `dream-os-wire`'s `aruaru_persistence` module needs a
runtime network connection to `aruaru-db` (the `aruaru-server` binary,
via pgwire) rather than a path dependency — configured via the
`DREAM_OS_ARUARU_PG_CONN` env var. When relocating, keep `open-cuda`,
`open-directx`, `open-web-server`, and `RS-SmartTCP` all cloned at the
same relative positions.

## 2. VPS/ローカルとの対応 / Local & VPS locations

- ローカル / Local: `F:\runo\dream-os`
- VPS(conoha): `/root/dream-os`(2026-08-06新設、プレースホルダの
  `README.md`のみ、git管理下ではない — created 2026-08-06, placeholder
  `README.md` only, not yet under git)

## 3. 移設・再開時の注意 / Notes for relocation / resuming

コードがまだ無いため、移設時は上記4ファイルをコピーするだけで完結します。
実装が始まった段階で、このセクションを`open-directx`/`RPoem`等と同様の
「ビルド手順」「持っていくもの」節へ拡充してください。

Since there is no code yet, relocating this repository is as simple as
copying the four files above. Once implementation begins, expand this
section to match the "build steps" / "files to bring" sections used by
`open-directx` / `RPoem`, etc.

---

Other languages: [English](PORTING-English.md) · [Deutsch](PORTING-German.md) ·
[Italiano](PORTING-Italian.md) · [Français](PORTING-French.md) ·
[Русский](PORTING-Russian.md) · [Українська](PORTING-Ukrainian.md) ·
[עברית](PORTING-Hebrew.md) · [فارسی](PORTING-Persian.md) ·
[العربية](PORTING-Arabic.md)

## Windows への導入と、Windows / Ubuntu での互換試験 (2026-10-14) / Windows install and compatibility tests

**導入**: `installer\windows\install.ps1`(ユーザー単位・管理者権限不要。`-Uninstall` で削除)。`%LOCALAPPDATA%\dream-os\bin` に、状態確認・RAID6・DXBC→SPIR-V のデモと `dream-os` コマンドを置きます。
`dream-os status|raid|dx` は Windows 上のネイティブ実行、`dream-os linux` は WSL2 の Ubuntu を開き、`dream-os both` は両方を**同時**に動かします(Windows はそのまま使えます)。
**正直な開示**: dream-os は、まだ単独で起動できる OS ではありません(Linux x TRON のハイブリッドカーネルは構想段階)。「Windows と切り替えて使う」「dream-os だけで起動する」ことはできず、ここでの「同時使用」は、Windows と WSL2(Linux)の並行実行です。
ビルドには、シェーダーコンパイラ `glslc`(Vulkan SDK / Ubuntu は `apt install glslc`)が要ります。`open-cuda` の `flash_attention.spv` は git 管理外なので、`glslc --target-env=vulkan1.1 flash_attention.comp -o flash_attention.spv` で作ります。

**試験結果**(Windows 11 + NVIDIA GT 730 / WSL2 Ubuntu + llvmpipe):
| 対象 | Windows | Ubuntu(WSL2) |
|---|---|---|
| dream-os(kernel・wire) | 全件成功 | 18 件成功 |
| dream-os-raid-bridge | 成功(GPU でパリティ計算、自己修復) | 成功(2026-10-16。CPU フォールバックで書き込み・読み出し・自己修復。全体で 25 件成功) |
| `dream_os_status`(GPU・DXBC→SPIR-V・Flash Attention) | OK(GT 730) | OK(llvmpipe) |
| open-cuda | 176 件成功 | 全件成功 |
| open-directx | 136 件成功 | 136 件成功 |
| open-cpu | 40 件成功 | 40 件成功 |
| aruaru-llm | 168 件成功 | 168 件成功 |
| aruaru-IME | 177 件成功 | CI で ibus / fcitx5 / インストールの通し試験に成功 |

見つけて直した不具合: `opencuda-blas` の `sgemm_auto_dispatch...nvidia` 試験が、NVIDIA 以外の Vulkan デバイス(WSL の llvmpipe)で失敗していた(NVIDIA でなければスキップするよう修正、open-cuda 側)。

English summary: install with `installer\windows\install.ps1` (per-user). `dream-os status|raid|dx` run natively on Windows; `dream-os linux` opens WSL2 Ubuntu; `dream-os both` runs both at once. dream-os is not yet a bootable standalone OS, so "switching" and "standalone boot" are not possible; concurrency means Windows and WSL2 side by side. Tests pass on Windows (GT 730) and Ubuntu/WSL2 (llvmpipe) except `dream-os-raid-bridge` on Ubuntu (no packaged `dxc`), untested. Fixed a GPU-vendor assumption in an open-cuda test (skips on non-NVIDIA devices).

## 日本語フォントの同梱 (2026-10-14) / Bundled Japanese font
`crates/dream-os-fonts` に、オープンソースの日本語フォント **IPAゴシック**(IPA Font License v1.0、OSI 承認、再配布可)を同梱しました(`fonts/ipag.ttf`、ライセンス全文 `fonts/IPA-LICENSE.txt`)。Ubuntu の公式パッケージ `fonts-ipafont-gothic` から取得。
Windows のインストーラーは `%LOCALAPPDATA%\dream-os\fonts\` に書き出します。`dream-os fonts` で、字形の網羅(ひらがな・カタカナ・漢字・記号・全角半角の見本 296 文字)を検査できます。
**試験**: 単体試験 4 件(見本の字形が全部ある・TTF として正しい・無い文字を報告する・ライセンス同梱)は Windows と Ubuntu で成功。`dream-os fonts` も両方で見本 296 文字すべてに字形あり。**描画も確認済み**(`render_text` と `examples/render_png`、`crates/dream-os-fonts/sample.png`): 「今日は、様です。ひらがな カタカナ 漢字変換 般若心経 ABC 123」を実際に絵にして目視で確認し、Windows と Ubuntu で出力 PNG がバイト単位で同一でした(単体試験は 5 件)。
なお、前に直した文字化けの原因は、フォントの不足ではなく、`.cmd` と PowerShell スクリプトの文字コード(UTF-8 の日本語を、Windows が別の文字コードで読んでいた)でした。フォント同梱は、dream-os 自身が日本語を表示するための別の備えです。
English: bundled the open-source IPAGothic font (IPA Font License v1.0) in `dream-os-fonts`, written to `%LOCALAPPDATA%\dream-os\fonts` by the installer; `dream-os fonts` checks glyph coverage (296 sample chars, none missing). 4 unit tests pass on Windows and Ubuntu; rasterization verified (identical PNG on Windows and Ubuntu). The earlier mojibake was a script-encoding problem, not a missing font.

### Ubuntu での dream-os-raid-bridge (2026-10-16) / raid-bridge on Ubuntu
`dxc`(Microsoft の DirectXShaderCompiler v1.9.2609 の公式配布 `linux_dxc_*.tar.gz`、約 13MB。利用者の許可を得て取得、`~/dxc` に展開、PATH と `LD_LIBRARY_PATH` に追加)でビルドは通るが、`zfs_accel_hlsl` の既定の `gpu` 機能は Windows 専用(D3D12/DirectML)で Linux ではコンパイルできなかった。`dream-os-raid-bridge/Cargo.toml` を、Windows は従来どおり `gpu`、Linux・macOS は `gpu` を外して `vulkan` 機能(Vulkan Compute)を使う指定に分けた。結果: Ubuntu で dream-os 全体の試験 25 件成功、デモは CPU フォールバックで書き込み・読み出し・1 台壊しての自己修復が一致(WSL2 の llvmpipe では GPU アクセラレーションは選ばれない)。
English: with the official `dxc` for Linux (downloaded with permission), the build passes, but `zfs_accel_hlsl`'s default `gpu` feature is Windows-only. `dream-os-raid-bridge` now uses `gpu` on Windows and `vulkan` (no `gpu`) elsewhere. On Ubuntu all 25 dream-os tests pass and the demo round-trips and self-heals on the CPU fallback.


## ディスク・マルチブート・インストーラー媒体(2026-10-18、ユーザー指示)
ユーザー指示「HDDのフォーマット機能とWindowsやLINUXも同時にインストールも同時起動も可能にし、Windowsから作れるUSBインストーラーをCDに焼いてもインストールできるように」を受け、`crates/dream-os-disk`を新設(詳細・使い方・試験結果は[`README.md`](README.md)同名節)。
- 構成: `gpt.rs`(GPT/CRC/プロテクティブMBR)、`layout.rs`(Windows/Linux/DreamOS配置計算)、`fat32.rs`(FAT32+LFN書き込み)、`iso9660.rs`(ISO9660+Joliet+El Torito UEFI)、`bootcfg.rs`(GRUB・QEMU)、`media.rs`(高レベル操作)、`main.rs`(CLI)。`installer/windows/make-usb.ps1`はUSB複写とCD書き込み起動。
- 設計判断: 「同時インストール」は、WindowsやLinuxの本体導入を代行せず(ライセンスと確実性のため)**領域の作成・ブートメニュー・共有領域まで**をDreamOSが担う。「同時起動」はマルチブート(GRUBメニュー)と、ホスト上のゲストとして動かすQEMUの2通り。NTFS/ext4は各OSのインストーラーがフォーマットする。実ディスクへの書き込みは完全一致の確認文字列つきで、PhysicalDrive0とマウント中は拒否。
- 未確認(次回の再開点): 実USBメモリへの書き込み、`format --device`の実ディスク試験、光学ドライブでの書き込み、実機UEFI起動(ブートローダーGRUBは未同梱・`--bootloader`で渡す)、NTFS/ext4のmkfs連携、10言語版ドキュメントへの反映。
- 検証環境の注意: WSL(Ubuntu)に`dosfstools`と`mtools`を追加導入した(FAT検証用)。8GiBの疎イメージを`/mnt/c`越しにddすると非常に遅いので、検証は小さいイメージを`/tmp`へコピーして行う。


## お引越し・再開メモ: dream-os-disk(2026-10-18、Windows再起動前) / Hand-off: dream-os-disk (2026-10-18, before Windows restart)

**日本語**
- **到達点**: `crates/dream-os-disk`(GPT・FAT32・ISO9660+Joliet+El Torito UEFI・GRUB/QEMU設定・CLI)と`installer/windows/make-usb.ps1`をpush済み(`d5ab8d2`)。単体試験15件成功。第三者ツールで構造を確認済み(`sfdisk --verify`、`fsck.vfat`=エラー無し、Windowsの`Mount-DiskImage`で日本語の長い名前を読めた)。
- **未コミットだった修正**: `make-usb.ps1`の「MBRのUSBでGPT初期化が失敗する」不具合の修正(`Clear-Disk`後に`PartitionStyle`を見て`Initialize-Disk`/`Set-Disk`を使い分け)。この`HANDOFF`と同じコミットに含めた。
- **中断した作業(最優先)**: 実USBメモリ(ディスク3「USB DISK 3.2」58.2GB)への書き込み。この環境からはUACが「ユーザーによりキャンセル」になり昇格できなかったため、**ユーザーが管理者PowerShellで実行する**手順にした。再起動でディスク番号が変わることがあるので、**必ず先に`-List`で番号を確認**すること。
  ```
  Set-ExecutionPolicy -Scope Process Bypass -Force
  F:/dream-os/installer/windows/make-usb.ps1 -List
  F:/dream-os/installer/windows/make-usb.ps1 -IsoPath <dreamos.iso> -DiskNumber <番号> -Execute -ConfirmText "ERASE <番号>"
  ```
- **`dreamos.iso`の再生成**(前回のISOは一時フォルダにあり、消えている可能性がある): WSL(Ubuntu、`grub-efi-amd64-bin`導入済み)で`grub-mkstandalone -O x86_64-efi -o BOOTX64.EFI --modules="part_gpt part_msdos fat ext2 chain search search_fs_file configfile normal echo ls loadenv efifwsetup efi_gop all_video" "boot/grub/grub.cfg=embed.cfg"`(`embed.cfg`は`search --no-floppy --file --set=root /EFI/BOOT/grub.cfg`と`configfile /EFI/BOOT/grub.cfg`の2行)。その後`cargo run -p dream-os-disk --release -- make-iso --payload <DIR> --out dreamos.iso --os windows,linux,dreamos --bootloader BOOTX64.EFI`。モジュール名の注意: `fwsetup`ではなく`efifwsetup`、`efi_uga`はx86_64-efiに無い。
- **未確認(実機)**: USBへの実書き込みとファイル照合、USBからのUEFI起動(GRUBメニュー→Windowsチェイン)、`format --device`の実ディスク試験、光学ドライブへの書き込み(`make-usb.ps1 -BurnCd`)、NTFS/ext4のmkfs連携、他言語READMEへの反映。
- **注意**: ① `F:/dream-os/vsc/`は別セッションの未追跡作業。触らない・コミットしない。② 8GiBの疎イメージを`/mnt/c`越しにddすると非常に遅いので、検証は小さいイメージを`/tmp`へ。③ WSLに`dosfstools`・`mtools`・`grub-efi-amd64-bin`を追加導入済み。④ `.ps1`はUTF-8 BOM+CRLFを保つ(`.gitattributes`)。
- **別PCへの引っ越し**: `git clone https://github.com/aon-co-jp/dream-os`、Rust導入、`cargo test -p dream-os-disk`(依存ゼロなので他リポジトリ不要。ただしワークスペース全体のビルドには`../open-web-server`等の兄弟リポジトリが要る)。WindowsのISO/USB作成は管理者権限と`Get-Disk`が使えるWindowsが必要。

**English**
- **State**: `crates/dream-os-disk` (GPT, FAT32, ISO9660+Joliet+El Torito UEFI, GRUB/QEMU config, CLI) and `installer/windows/make-usb.ps1` are pushed (`d5ab8d2`); 15 unit tests pass; structures verified by third-party tools (`sfdisk --verify`, `fsck.vfat` clean, Windows `Mount-DiskImage` reads the Japanese long names).
- **Fix that was uncommitted**: `make-usb.ps1` failed to GPT-initialise an MBR USB stick; it now checks `PartitionStyle` after `Clear-Disk` and uses `Initialize-Disk` or `Set-Disk`. Committed together with this note.
- **Interrupted (top priority)**: writing to a real USB stick (disk 3, "USB DISK 3.2", 58.2 GB). UAC elevation from this environment was cancelled three times, so **the user runs it from an elevated PowerShell**. Disk numbers can change after a reboot, so **run `-List` first** (commands above).
- **Regenerating `dreamos.iso`** (the earlier ISO was in a temp folder): build `BOOTX64.EFI` with `grub-mkstandalone` in WSL (modules and embed config above; use `efifwsetup`, not `fwsetup`; `efi_uga` does not exist for x86_64-efi), then `dream-os-disk make-iso --payload <DIR> --out dreamos.iso --os windows,linux,dreamos --bootloader BOOTX64.EFI`.
- **Not verified on hardware**: the actual USB write and file comparison, UEFI boot from the stick (GRUB menu → Windows chain-load), `format --device` on a real disk, disc burning (`-BurnCd`), NTFS/ext4 mkfs integration, other-language READMEs.
- **Cautions**: `F:/dream-os/vsc/` is another session's untracked work — do not touch or commit it. dd of an 8 GiB sparse image over `/mnt/c` is very slow; copy small images to `/tmp`. WSL now has `dosfstools`, `mtools`, `grub-efi-amd64-bin`. Keep `.ps1` as UTF-8 BOM + CRLF.
- **Moving to another PC**: clone the repo, install Rust, run `cargo test -p dream-os-disk` (zero dependencies; building the whole workspace also needs sibling repos such as `../open-web-server`). Making the USB needs Windows with admin rights and `Get-Disk`.

## 🔁 お引越し・再開用メッセージ / Resume message (2026-10-18)

**日本語(次のセッションの Claude へ)**
1. まず `F:\dream-os\vsc\BUILD-REPORT.md` を読む。DreamOS Code(Code-OSS 1.142.0、WSL2 の Ubuntu で `build.sh`)は、**node・clone・`npm ci` が完了し、`gulp vscode-linux-x64-min` も compile / bundle / minify / `package-linux-x64` まで成功**している(出力は `~/dream-vsc/VSCode-linux-x64`)。ただし**最後の工程 `prepareBuiltInCopilotRipgrepShim` が「Copilot SDK directory not found」で失敗**して終わっており、原因は**未調査**(調べ始めた `vsc/inspect.sh` の結果は未取得)。**`open-vsc-claude` の同梱・`verify.sh` は未実行なので「ビルドできた」とはまだ言えない**。
2. 次の手: (a) `wsl -d Ubuntu -- bash /mnt/f/dream-os/vsc/inspect.sh`(PowerShell)で、`build/gulpfile.vscode.ts` の Copilot 関連の配線と `extensions/copilot/node_modules/@github/copilot` の有無を確認、(b) DreamOS Code は GitHub Copilot を同梱する必要が無いので、Copilot の組み込みと最終工程を外す(`build.sh` の `package` 段階の前にソースを調整、または失敗後の出力をそのまま使う)方針を、**原因を確かめてから**選ぶ、(c) `build.sh` の bundle 段階(拡張同梱・`product-overlay.json`)→ `verify.sh`(Xvfb が要るなら `wsl -u root apt-get install -y xvfb`)、(d) 結果を `BUILD-REPORT.md` に正直に書いて `vsc/` をコミット。`-ci` 系タスクは事前工程が前提で単独では失敗する(`vscode-linux-x64-min` を使う、修正済み)。
3. `open-vsc-claude` は v0.1.2 を公開済み(`F:\open-vsc-claude`、デモ `https://easy-web.tokyo/open-vps-claude/`、テスト 4 種が通る、VS Code 1.85 / VSCodium 1.140 / Web 拡張ホストで確認)。**未了**: Marketplace / Open VSX 公開(所有者のアカウントとトークンが必要。手順は `PUBLISHING.md`)、Cosmo Router の実起動、実キーでの Claude 生成、実機 iOS/Android、翻訳のネイティブ確認。RPoem の外部サブグラフ中継は `babc658` で反映済み。
4. 秘密(トークン・API キー)はリポジトリに入れない。`F:\` 直下の txt に置き、値を画面・コミットに出さない。
5. 落とし穴: Git Bash は `/mnt/...` を書き換える(PowerShell を使う)、PowerShell の引用符内の `|` や `$変数` は WSL に渡る前に解釈される(処理は `.sh` に書く)、シェルの heredoc で python に `\n` を渡すと本物の改行になる(補助スクリプトは先にファイルへ)、CRLF のファイルは置換が静かに失敗する(Edit ツールを使う)、書き換え済みリポジトリで `git pull` しない。

**English (for the next Claude session)**
1. Read `F:\dream-os\vsc\BUILD-REPORT.md` first. DreamOS Code (Code-OSS 1.142.0, `build.sh` in WSL2 Ubuntu): **node, clone and `npm ci` are done, and `gulp vscode-linux-x64-min` got through compile / bundle / minify / `package-linux-x64`** (output in `~/dream-vsc/VSCode-linux-x64`). But the **last step `prepareBuiltInCopilotRipgrepShim` failed with "Copilot SDK directory not found"**, and the cause is **not investigated yet** (the result of `vsc/inspect.sh`, started to look into it, was never read). **Bundling `open-vsc-claude` and `verify.sh` have not run, so do not claim it builds yet.**
2. Next: (a) from PowerShell run `wsl -d Ubuntu -- bash /mnt/f/dream-os/vsc/inspect.sh` to see how `build/gulpfile.vscode.ts` wires Copilot and whether `extensions/copilot/node_modules/@github/copilot` exists, (b) DreamOS Code has no need to ship GitHub Copilot, so choose — **after confirming the cause** — to drop the built-in Copilot and that final step (adjust the source before the `package` stage in `build.sh`, or reuse the already packaged output), (c) run `build.sh`'s bundle stage (extension + `product-overlay.json`), then `verify.sh` (if Xvfb is missing: `wsl -u root apt-get install -y xvfb`), (d) record the real result in `BUILD-REPORT.md` and commit `vsc/`. The `-ci` tasks assume earlier stages and fail alone (use `vscode-linux-x64-min`; already fixed).
3. `open-vsc-claude` v0.1.2 is released (`F:\open-vsc-claude`, demo `https://easy-web.tokyo/open-vps-claude/`, all four test suites pass; verified on VS Code 1.85, VSCodium 1.140 and the web extension host). **Open**: Marketplace / Open VSX publication (owner account + token; see `PUBLISHING.md`), running the Cosmo Router, Claude generation with a real key, real iOS/Android devices, native review of translations. RPoem's external-subgraph forwarding is in (`babc658`).
4. Never put secrets (tokens, API keys) in repositories; keep them in txt files at the root of `F:\` and never print or commit their values.
5. Pitfalls: Git Bash rewrites `/mnt/...` (use PowerShell); `|` and `$vars` inside quoted PowerShell strings are interpreted before reaching WSL (put logic in a `.sh`); escapes in shell heredocs turn `\n` into real newlines (write helper scripts to files first); CRLF files make scripted replacement fail silently (use the Edit tool); never `git pull` in rewritten repositories.
