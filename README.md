# DreamOS (dream-os)

*English*: [README-English.md](README-English.md) ·
*Other languages*: [Deutsch](README-German.md) · [Italiano](README-Italian.md) ·
[Français](README-French.md) · [Русский](README-Russian.md) ·
[Українська](README-Ukrainian.md) · [עברית](README-Hebrew.md) ·
[فارسی](README-Persian.md) · [العربية](README-Arabic.md)

> 📌 **最近の更新(2026-10-18)**: dream-os 用 Visual Studio Code として、Code-OSS(VS Code 本体の OSS 版)をビルド・パッケージし `open-vsc-claude` を同梱する手順とスクリプトを [`vsc/`](vsc/README.md) に追加(WSL で `npm ci` と `gulp` の package まで成功、最終工程の Copilot 準備で失敗・原因未調査、同梱と `verify.sh` は未実行。再開は [`vsc/BUILD-REPORT.md`](vsc/BUILD-REPORT.md))。
>
> *Added [`vsc/`](vsc/README.md): scripts to build and package Code-OSS (the open-source core of VS Code) as "DreamOS Code" with `open-vsc-claude` bundled. In WSL: `npm ci` and the gulp package step succeeded, the final Copilot step failed (cause not investigated), bundling and `verify.sh` have not run — resume via [`vsc/BUILD-REPORT.md`](vsc/BUILD-REPORT.md).*

> 📌 **最近の更新(2026-08-08)**: `dream-os-raid-bridge`(open-raid-zの
> RAID6/Z2実装ブリッジ)に、テストコードを読まなくても1コマンドで動作
> 確認できるrunnableデモ`examples/raid6_bridge_benchmark.rs`を追加
> (`cargo run -p dream-os-raid-bridge --example raid6_bridge_benchmark
> --release`)。NVIDIA GT730実機で、RAID6パリティ計算アクセラレータが
> 実際に`Gpu`(CPUフォールバックではない)であること、8ストライプの
> 書き込み・読み出しラウンドトリップ一致、1台のディスクを直接壊した
> 状態からの自己修復(パリティ2本による復元)を実際に確認した。
> `dream-os-kernel/src/lib.rs`にTODO/未実装/`allow(dead_code)`は
> 見当たらず、修正すべき粗は無かった(正直な開示)。
>
> *English*: Added a runnable demo
> (`examples/raid6_bridge_benchmark.rs`) to `dream-os-raid-bridge` (the
> open-raid-z RAID6/Z2 bridge) so it can be exercised with a single
> command without reading test source. Verified on real NVIDIA GT730
> hardware: the detected parity accelerator is actually `Gpu` (not a CPU
> fallback), an 8-stripe write/read roundtrip matches, and the bridge
> self-heals after directly corrupting one disk (RAID6's two parity
> disks recover it). Checked `dream-os-kernel/src/lib.rs` for
> TODO/unimplemented/`allow(dead_code)` markers — found none, so no fix
> was needed there (honest disclosure).

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

> 📌 **お引越し・再開メモ(2026-10-18)**: `dream-os-disk`の実USB書き込み試験が途中です。再開手順・未確認項目・注意点は[`PORTING.md`](PORTING.md)末尾「お引越し・再開メモ」(日英)を参照。

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

## ディスクのフォーマット・マルチブート・インストーラー媒体(2026-10-18、`dream-os-disk`)

`crates/dream-os-disk`(外部依存ゼロ)で、次を実装しました。

- **HDDのフォーマット**: GPT(+プロテクティブMBR)の作成、EFIシステム領域と共有領域のFAT32フォーマット。NTFS/ext4/swapは領域だけ作り、中身のフォーマットはWindows/Linuxのインストーラーに任せます(そのほうが安全で確実なため)。
- **Windows・Linux・DreamOSの同居**: 1台のディスクに3つを並べる配置計算(`plan`)、起動メニュー(GRUB。Windowsはブートマネージャへチェイン)の生成(`grub-cfg`)。**同時起動**はQEMU(Windowsは`whpx`、Linuxは`kvm`)の起動コマンド生成(`qemu`)で、片方をホスト、もう片方をゲストにします。
- **USB/CDインストーラー**: `make-iso`が、UEFIブート可能なISO9660(Joliet=日本語の長い名前対応、El Torito)を作ります。**同じISOをCD/DVDに焼け、USBへも複写できます**。Windowsから使う`installer/windows/make-usb.ps1`は、USBメモリ(BusType=USBかつシステムディスクでない物だけ)を選んで複写、`-BurnCd`でWindows標準の書き込みツールを開きます。

```
dream-os-disk plan     --disk 500G --os windows=200G,linux=100G,dreamos --shared 20G --swap 8G
dream-os-disk format   --image disk.img --size 8192M --os windows=2048M,linux     # 試験用イメージ
dream-os-disk format   --device \\.\PhysicalDrive2 --size-bytes N --confirm "ERASE \\.\PhysicalDrive2 N" --os ...   # 実ディスク(全消去)
dream-os-disk make-iso --payload DIR --out dreamos.iso --os windows,linux,dreamos --bootloader BOOTX64.EFI
.\installer\windows\make-usb.ps1 -List ; -IsoPath dreamos.iso -DiskNumber 2 -Execute -ConfirmText "ERASE 2"
```

**安全策**: 実ディスクの`format`は`--confirm "ERASE <パス> <バイト数>"`が完全一致しないと動かず、`PhysicalDrive0`とマウント中のデバイスは拒否します。`make-usb.ps1`は既定が「確認だけ」で、実行には`-Execute`と`-ConfirmText "ERASE <番号>"`が要り、内蔵ディスクは候補に出ません。

**正直な現状**: 単体試験15件成功。GPTはLinuxの`sfdisk --verify`、FAT32は`fsck.vfat`(エラー無し)と`mtools`、ISOはWindowsの`Mount-DiskImage`(日本語の長いファイル名を含め読めた)で、それぞれ第三者のツールにより確認しました。**未確認**: (1)実際のUSBメモリへの書き込み(手元に無く、拒否動作のみ確認)、(2)実ディスクの`format --device`、(3)光学ドライブへの書き込み、(4)実機のUEFI起動 — DreamOS自身はまだ起動できるOSではないため、**UEFIブートローダー(GRUBなど)は利用者が`--bootloader`で渡す必要があり**、同梱していません。(5)WindowsやLinuxの本体の導入は、それぞれの公式インストーラー(ISOは利用者が用意)で行います。DreamOSがWindowsの導入を代行することはしません。

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

---

作者 / Author: 石 塚 正 浩（MASAHIRO ISHIZUKA） (https://github.com/aon-co-jp)
