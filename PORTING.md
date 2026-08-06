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
        ├── src/{lib,power_profile,mining,sbm}.rs
        ├── shaders/{vector_add,sha256d_mine,sbm_ising}.{comp,spv}
        ├── examples/{dispatch_throttled,mine_benchmark}.rs
        └── tests/{mining_real_vulkan,sbm_real_vulkan}.rs
    └── dream-os-wire/  # World Laboratory向け通信層(open-web-server-wire
                          # のSecureChannel再利用、AEAD暗号化+リプレイ対策)
        ├── Cargo.toml   # ../../../open-web-server/crates/open-web-server-*
        │                # へのpath依存(さらにopen-web-server-wireは
        │                # ../../../RS-SmartTCPへのpath依存を持つ)
        ├── src/lib.rs
        └── tests/secure_channel_integration.rs
```

`crates/dream-os-kernel`は`open-cuda`(`../../open-cuda`、同じ`F:\runo`
直下にcloneされている前提)へのpath依存を持つ。`crates/dream-os-wire`は
`open-web-server`(`../../../open-web-server`)へのpath依存を持ち、
さらに`open-web-server-wire`自体が`RS-SmartTCP`(`../../../RS-SmartTCP`、
`open-web-server`から見た相対位置)へのpath依存を持つ。移設する場合は
`open-cuda`・`open-web-server`・`RS-SmartTCP`をすべて同じ相対位置関係で
cloneしておくこと。 / `crates/dream-os-kernel` has a path dependency on
`open-cuda` (`../../open-cuda`, assumed cloned alongside this repo under
`F:\runo`). `crates/dream-os-wire` has a path dependency on
`open-web-server` (`../../../open-web-server`), which in turn has a path
dependency on `RS-SmartTCP` (`../../../RS-SmartTCP`, relative to
`open-web-server`). When relocating, keep `open-cuda`, `open-web-server`,
and `RS-SmartTCP` all cloned at the same relative positions.

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
