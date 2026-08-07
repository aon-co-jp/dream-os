# DreamOS (dream-os)

> 📌 **最近の更新(2026-08-07)**: Android実機(Adreno 619)で
> open-cuda(Vulkanデバイスオープン)・open-directx(DXBC→SPIR-V翻訳)
> 連携のPoCを実証する新規Androidアプリ(`android/`)を追加、Windows実機
> (GT730)・Android実機の両方で動作確認。SBM/DeepSeek組み込み構想
> (2026-08-06保留タスク)については、各リポジトリでの具体的な最適化
> 対象が依然未特定のため、憶測に基づく実装は避けて見送った(調査結果は
> [CLAUDE.md](CLAUDE.md)に正直に記録)。この調査の過程で
> `sbm_ising`が既に`opencuda_core::GpuDevice`経由のGPU実行パスを持つ
> ことを`cargo test`で再確認した(CPU限定という以前の誤記を訂正)。
>
> *English*: Added a new Android app (`android/`) demonstrating an
> open-cuda (Vulkan device open) + open-directx (DXBC→SPIR-V
> translation) integration PoC, verified on real hardware (Windows
> GT730 + Android Adreno 619). The Toshiba SBM/DeepSeek integration plan
> remains pending — no concrete per-repo optimization target has been
> identified yet, so no speculative implementation was made (see
> [CLAUDE.md](CLAUDE.md) for the honest write-up). Also corrected an
> earlier documentation error: `sbm_ising` already has a GPU dispatch
> path via `opencuda_core::GpuDevice`, confirmed via `cargo test`.

**構想段階のプロジェクトですが、2026-08-06より実機検証済みのPoC
(概念実証)コードが複数存在します。** この`README.md`・
[`CLAUDE.md`](CLAUDE.md)・[`PORTING.md`](PORTING.md)・
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)で
目的・スコープ・エコシステム内での位置づけ・実装状況を文書化しています。

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
Nintendo Switch 2・**IBMメインフレーム(z/OS系、命令セットエミュレータ
Hercules自体はOSSだが実OSにはIBM正式ライセンスが必要)**。
「AWS版メインフレーム」はOS互換レイヤーではなくアプリケーション移行
支援サービス(AWS Mainframe Modernization等)と判明、詳細は
[`CLAUDE.md`](CLAUDE.md)参照。

**正直な開示**: macOS/iPhone・PS5/6・Switch2・IBMメインフレームへの
対応には各社の非公開SDK・NDA・実機/開発機材・正式ライセンスが必須で
あり、現時点でこのエコシステムに実機・
SDK・開発者ライセンスは一切ありません。「将来ライセンスが取得できたら
着手する」という前提の保留とし、まずは実機のあるWindows・Androidから
着手する方針です(詳細は[`CLAUDE.md`](CLAUDE.md)の「スコープの絞り込み」
節参照)。

## 実装済みPoC(2026-08-06、実機検証済み)

`crates/dream-os-kernel`(`open-cuda`の`opencuda-vulkan`を再利用):

- **Windows/Android共通のVulkan実行基盤**: `vector_add`カーネルを
  Windows実機(NVIDIA GT730)・Android実機(Moto G53Y 5G、Adreno 619)
  双方で実際にディスパッチし動作確認済み。
- **電力出力調整可能なマイニングOS向け機能**: `MiningPowerProfile`
  (ソフトウェア側デューティサイクル制御)+`sha256d_mine`カーネル
  (double-SHA256)。GPU計算結果がCPU参照実装(`sha2`クレート)と完全
  一致することをWindows/Android両実機で確認。Android実機でのモバイル
  GPUドライバのタイムアウト(TDR)対策としてバッチ分割設計を実装。
- **東芝Simulated Bifurcation Machine(SBM)にインスパイアされた量子
  アニーリング風組合せ最適化**: `sbm_ising`カーネル(ballistic SB
  アルゴリズム、Ising模型の最小化)。GPU版とCPU参照実装が完全に同一の
  スピン配置へ収束することをWindows実機で確認。

`crates/dream-os-wire`(`open-web-server-wire`のSecureChannelを再利用):

- **World Laboratory構想(BOINC型の分散ボランティアコンピューティング)
  向けの結果送信保護**: AEAD暗号化+リプレイ攻撃対策で、ワーカーが送る
  計算結果の改ざん・リプレイを検知・拒否する`WorkerChannel`/
  `CoordinatorChannel`。正常送受信・リプレイ攻撃・改ざん結果、3つの
  シナリオを実際の暗号処理で検証済み。詳細な設計は
  [world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)
  参照。

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

上記PoCコードは実機検証済みですが、DreamOS本体(統合カーネル・
コーディネータ本体)はまだ構想・設計段階です。open-directxの完成度向上を
最優先方針としつつ、dream-osは小さく検証可能な単位で育てています
(詳細は[`CLAUDE.md`](CLAUDE.md)のHANDOFF参照)。

## 関連プロジェクト

- [aon-co-jp/open-directx](https://github.com/aon-co-jp/open-directx)
- [aon-co-jp/open-cuda](https://github.com/aon-co-jp/open-cuda)
- [aon-co-jp/aruaru-llm](https://github.com/aon-co-jp/aruaru-llm)
- [aon-co-jp/RUNO](https://github.com/aon-co-jp/RUNO) —
  エコシステム全体のメタ索引

## License

未定(構想段階のため)。
