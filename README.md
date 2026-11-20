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

> 📌 **DreamOSインストーラー環境(2026-10-18)**: USB/CDのGRUBで「DreamOS Installer」を選ぶと、ディスクの縮小(Windowsのデータ保持)・DreamOS用領域の作成ができるテキストのインストーラーが起動します。詳細は[`PORTING.md`](PORTING.md)末尾。DreamOS本体のOSはまだありません。

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

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
