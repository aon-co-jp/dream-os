# DreamOS (dream-os)

**構想段階のプロジェクトです。現時点でコードは一切存在しません。**
この`README.md`・[`CLAUDE.md`](CLAUDE.md)・[`PORTING.md`](PORTING.md)は、
目的・スコープ・エコシステム内での位置づけを文書化するための構想メモです。

## 目的

Windows・macOS・Android・各種Linuxディストリビューションに対応する
アプリケーションサービス層/GPU抽象化層を目指します。将来的には
**SONY PlayStation 5 / PlayStation 6・Nintendo Switch 2**への対応も
視野に入れています(2026-08-06、ユーザー指示による方向性)。

## 対応プラットフォーム(ロードマップ、優先順位順)

1. Windows
2. macOS
3. Android
4. 各種Linuxディストリビューション
5. (将来的に)SONY PlayStation 5 / PlayStation 6
6. (将来的に)Nintendo Switch 2

**正直な開示**: PS5/6・Switch2への対応には各社の非公開SDK・NDA・実機/
開発機材が必須であり、現時点でこのエコシステムに実機・SDK・開発者
ライセンスは一切ありません。方向性の明記であり、実現可能性・着手時期は
未確定です。

## エコシステム内での位置づけ

このプロジェクトは`aon-co-jp`エコシステムの一部で、特に以下と関連します
(具体的な統合方法は未確定、詳細は[`CLAUDE.md`](CLAUDE.md)参照):

- [`open-directx`](https://github.com/aon-co-jp/open-directx) —
  Windows専用DirectXアプリ/ゲームをLinux/Android/将来的にはmacOS/
  PlayStationファミリーでも動かすクロスプラットフォーム互換層。
  **現在このエコシステム内で最優先で完成度を高めている最中**のプロジェクト。
- [`open-cuda`](https://github.com/aon-co-jp/open-cuda) — CUDA互換GPU
  計算基盤(実Vulkanバックエンド)。
- [`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm) — LLM推論
  (open-cudaのVulkanカーネルを利用)。

## 現状

構想フェーズのみ。コード・ビルド設定・CI等は一切ありません。
まずopen-directxの完成度向上を通じて実証を積んでから、DreamOSの
具体的な技術スコープを固める方針です(詳細は[`CLAUDE.md`](CLAUDE.md)の
HANDOFF参照)。

## 関連プロジェクト

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) —
  エコシステム全体のメタ索引

## License

未定(構想段階のため)。
