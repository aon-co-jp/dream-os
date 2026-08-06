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
└── PORTING.md          # 本ファイル / this file
```

現時点でこれ以上のファイルはありません(構想段階のため)。 / There is
nothing else at this stage (concept phase only).

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
