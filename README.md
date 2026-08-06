# DreamOS (dream-os)

**構想段階のプロジェクトです。現時点でコードは一切存在しません。**
この`README.md`・[`CLAUDE.md`](CLAUDE.md)・[`PORTING.md`](PORTING.md)は、
目的・スコープ・エコシステム内での位置づけを文書化するための構想メモです。

## 目的

**世の中に存在するほとんどのOSを統合・融合する、LinuxとTRON OSの
ハイブリッドカーネル**を目指します。単一のカーネル上でWindowsの
PowerShell・macOS/iPhoneアプリ・Androidアプリがそれぞれ互換性を保ったまま
動作することが最終目標です。将来的には**SONY PlayStation 5 / 6・
Nintendo Switch 2**への対応も視野に入れています(2026-08-06、ユーザー
指示による方向性)。

## 技術調査(2026-08-06実施、正直な要約)

日英でのGoogle/GitHub調査を実施した結果、**「Linux×TRONのハイブリッド
カーネルで全プラットフォームのアプリをネイティブ動作させる」という構想を
そのまま実現した既存実装は見つかりませんでした**。特に重要な発見として、
**iOSアプリをApple以外のハードウェアでネイティブ動作させる実用的な方法は
2026年時点で存在しません**(Apple自身がサードパーティアプリのJIT・
仮想化をOS全体で禁止しているため)。一方、ChromeOS(`crosvm`という共通
VMM基盤上でLinux/Androidを統合管理する方式)が最も実績のある現実的な
先例として見つかりました。詳細な調査結果・要素技術ごとの成熟度比較・
今後の現実的な設計方針は[`CLAUDE.md`](CLAUDE.md)の「技術調査結果」節を
参照してください。

## 対応プラットフォーム(ロードマップ、2026-08-06実機ベースで更新)

**フェーズ1(実機あり、今すぐ着手)**: Windows(NVIDIA GT730搭載PC)・
Android(スマホ数台)。

**フェーズ2以降(将来、ライセンス/許諾取得を前提とした保留)**: macOS/
iPhone・各種Linuxディストリビューション・SONY PlayStation 5/6・
Nintendo Switch 2。

**正直な開示**: macOS/iPhone・PS5/6・Switch2への対応には各社の非公開
SDK・NDA・実機/開発機材が必須であり、現時点でこのエコシステムに実機・
SDK・開発者ライセンスは一切ありません。「将来ライセンスが取得できたら
着手する」という前提の保留とし、まずは実機のあるWindows・Androidから
着手する方針です(詳細は[`CLAUDE.md`](CLAUDE.md)の「スコープの絞り込み」
節参照)。

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
