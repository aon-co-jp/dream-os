# 設計思想＆開発方針＆開発環境ルール(DreamOS / dream-os)

作業ドライブは`F:\runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。
GitHubリポジトリ: [aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os)。

**構想開始日: 2026-08-06**(このリポジトリへの構想文書着手日。GitHub上の空リポジトリ
自体は2026-07-01に作成済みだった、`open-directx`と同じパターン)。

## このプロジェクトの役割(構想段階、コード未着手)

**DreamOS**(リポジトリ名・VPS/ローカルフォルダ名は`dream-os`)は、
**世の中に存在するほとんどのOSを統合・融合する、LinuxとTRON OSの
ハイブリッドカーネル**を目指す(ユーザー指示、2026-08-06)。単一の
カーネル上で、Windowsのpowershell・macOS/iPhoneアプリ・Androidアプリが
それぞれ互換性を保ったまま動作することを最終目標とする。裏付けとして
ユーザーが挙げている論拠: 「Claude Code Desktop上でRust+RPoem
(Poem互換)でアプリ/WEBサイトを開発すれば、インストーラーだけを
プラットフォームごとに用意し、システム自体は単一の万能互換基盤として
開発できる」という考え方。

**現時点ではコードは一切書かれておらず、構想フェーズのみ**。この
`CLAUDE.md`・[`README.md`](README.md)・[`PORTING.md`](PORTING.md)で
目的・スコープ・エコシステム内での位置づけ・**技術調査結果**を文書化
することが今回の唯一の成果物。

## 技術調査結果(2026-08-06、日英でのGoogle/GitHub検索に基づく、正直な開示)

ユーザー指示「まずは、日本語と英語でGoogleとGithub検索と調査から
始めて下さい」を受けて実施した調査。**結論を先に書く: 「Linuxと
TRON OSのハイブリッドカーネルで、Windows/macOS/iPhone/Androidの
アプリを単一カーネル上でネイティブ互換動作させる」という構想を
そのまま実現した既存のOSS実装・商用製品は見つからなかった。**
以下、要素ごとの調査結果と、現実的な代替アーキテクチャの検討。

### 1. Linux×TRON OSのハイブリッドカーネルについて

- **学術的な先行研究は実在する**: 2001年のIEEE論文
  ["Linux on ITRON": a hybrid operating system architecture for embedded systems](https://ieeexplore.ieee.org/document/994546/)
  が、汎用OS(Linux)とリアルタイムOS(μITRON仕様)を組み合わせた
  ハイブリッドアーキテクチャを提案している。ただし対象は**組み込み
  システムのリアルタイム制御**であり、「様々なアプリ互換性を持つ
  汎用デスクトップ/モバイルOS」という今回の目的とは方向性が異なる。
  また2001年の研究で、現在活発にメンテナンスされているOSSプロジェクト
  としては見つからなかった。
- **TRON Forum公式のT-Kernel 2.0**([GitHub: tron-forum/tkernel_2](https://github.com/tron-forum/tkernel_2))は
  現在も組み込み向けリアルタイムOSとして開発が続いている
  ([tron.org](https://www.tron.org/tron-project/what-is-t-kernel/t-kernel-2-0/))が、
  Linuxとの統合(ハイブリッドカーネル化)を謳った現行プロジェクトは
  見当たらなかった。
- **アーキテクチャ上の根本的な相性の悪さの指摘**: 調査で見つかった
  分析([Grokipedia記事](https://grokipedia.com/page/TRON_project)より)
  では、「TRONの決定論的リアルタイムカーネル・リソース分割設計は、
  Unix由来でプロセス指向・汎用互換性を優先するPOSIXモデルと衝突し、
  POSIX中心のエコシステムへの統合は非効率であることが実証されている」
  との指摘があった。組み込みRTOS市場でもTRON系は5%未満のシェアに
  留まり、FreeRTOS・VxWorks等に押されているとの分析もあった。
  **正直な評価**: TRONとLinuxを文字通り1つのカーネルへ融合するのは、
  学術的に一度試みられたが主流化しなかった、技術的難度の高いアプローチ
  だと考えられる。
- **GitHubでの実際の検索結果**: `hybrid kernel TRON`
  ・`hybrid kernel TRON Linux`等のキーワードでリポジトリ検索を行った
  ところ、該当する実装プロジェクトは0件だった(2026-08-06実施)。

### 2. 「1つのOS上でWindows/macOS/Androidアプリを同時に動かす」実例調査

個別プラットフォームの互換レイヤーは複数実在するが、**単一の統合OSとして
全てを同時に提供する製品・OSSは見つからなかった**。実在する個別要素技術:

| 対象 | 技術 | 方式 | 成熟度 |
|---|---|---|---|
| Windows→Linux | [Wine](https://www.winehq.org/)/[Proton](https://github.com/ValveSoftware/Proton)(Valve) | アプリ層でのWin32 API実装(カーネル統合ではない) | 高(Steam Deck等で実運用) |
| macOS→Linux | [Darling](https://www.darlinghq.org/) | Mach-O/Darwin ABI互換層 | 限定的(macOS本体ほどの網羅性は無い、開発ペースも緩やか) |
| Android→Linux | [Waydroid](https://lwn.net/Articles/901459/) | LXC/namespaceでAndroidをコンテナ実行(AndroidもLinuxカーネルベースなので比較的容易) | 高(実用レベル) |
| iOS(iPhone)→他OS | (無し、下記参照) | — | 事実上不可能 |

**最も参考になった実例**: **ChromeOS**が、Crostini(Linux、KVM経由の
`crosvm`——Rust製VMM)・ARCVM(Android、同じく`crosvm`上でAndroid
スタック全体をVM実行)という**複数ゲストOSを共通のVMMインフラ
(`crosvm`)上で動かす**方式を実際に本番採用している
([google/crosvm](https://github.com/google/crosvm))。「1つのカーネルへ
全てのOSのABIを直接実装する」のではなく、「軽量VM/コンテナごとに
各OSをホストし、共通基盤で統一的に管理する」という設計であり、これが
現実に量産・商用展開されている最も近い先例だと考えられる。

**もう1つの参考先例**: **seL4マイクロカーネル**上に、複数OSの
「パーソナリティ」(人格)を載せる研究がある。[Neptune OS](https://www.theregister.com/2022/02/24/neptune_os_sel4_windows/)は
seL4上にWindows NT互換のNTOSKRNL実行層を実装するプロジェクト、
[Genode OSフレームワーク](https://genode.org/)はseL4等のL4系
マイクロカーネル上でLinux(CAmkES VM経由)を含む複数OS環境を
コンポーネントとして構成する仕組みを提供している。これも「1つの
物理カーネルバイナリに全部の互換コードを書き込む」のではなく、
「マイクロカーネル+各OS用の実行パーソナリティを分離コンポーネント
として積み上げる」設計思想である。

### 3. iOS(iPhone)アプリの互換動作について(重要な制約、正直な開示)

**Apple純正ハードウェア以外でiOSアプリをネイティブ動作させる実用的な
方法は、2026年時点で存在しない。** 調査で判明した技術的制約:

- iOSはサードパーティアプリのサンドボックス内でのJITコンパイルを
  OSレベルで全面禁止している。エミュレータの多くはバイトコードの
  高速実行にJITを要するため、この制約に直接抵触する。
- ハードウェア仮想化機能・カーネルレベルのシステムアクセスも
  サードパーティアプリからは完全にブロックされている。
- 2026年時点で実用とされる代替手段は、いずれも**Apple製の実機上で
  クラウド/ストリーミング経由**か、Safari上のWebAssembly実行に
  限られる(TestMu AI・Redfinger等のクラウドAndroidストリーミング、
  Safari上でのゲームROM WASM実行等)——「Apple以外のハードウェア上で
  iOSアプリのバイナリをネイティブ実行する」というシナリオを満たす
  ものは見つからなかった。
- 非ジェイルブレイクのサイドローディング(AltStore/TrollStore経由の
  DolphiniOS等)も**Apple実機上でのみ**動作するiOSエミュレータであり、
  「他OS上でiOSアプリを動かす」話とは別物。

**正直な評価**: iOSアプリ互換性は、技術的制約(Apple自身による
JIT/仮想化ブロック)に加え、Apple Developer Program
利用規約・DMCA等の法的リスクも伴う領域であり、DreamOSのスコープに
含める場合は「Apple公式のクロスプラットフォームフレームワーク
(Mac Catalyst等)経由でのアプリ移植を促す」等、ネイティブiOSバイナリの
直接実行以外のアプローチを検討する必要がある。**この制約は今回の
調査で最も重要な発見であり、ユーザーに直接お伝えする**。

### 4. 「Windowsのpowershellが動く」について

これは既に**解決済みの技術**である。PowerShell(pwsh、PowerShell Core)は
.NET Core化されて以降オープンソース・クロスプラットフォーム化されており、
Linux/macOS上でネイティブに(WSL等の仮想化を介さず)動作する
(`apt install powershell`等で導入可能)。DreamOSがLinuxベースの
カーネルを採用するなら、pwshバイナリを同梱するだけでこの要件は
満たせる——独自実装の必要は無い。

### 5. 現実的なアーキテクチャ方針(暫定案、次回以降で要検討)

上記調査を踏まえた、次回セッション以降で検討すべき現実的な方向性:

1. **「1つのモノリシックなハイブリッドカーネル」ではなく、「Linuxカーネル
   をベースに、各OS向けの軽量VM/コンテナ・互換レイヤーを統一管理する」
   というChromeOS/crosvm型のアーキテクチャが最も実績があり現実的**。
2. Windows対応: Wine/Proton(またはvkd3d-proton、`open-directx`が
   目指す方向性とも合流できる可能性)を統合。
3. Android対応: Waydroid相当のコンテナ方式(Androidが元々Linuxカーネル
   ベースなので技術的難度は比較的低い)。
4. macOS対応: Darling(現状の成熟度は限定的、要継続調査)。
5. iOS対応: ネイティブバイナリ直接実行は非現実的(上記4参照)。
   Web/PWA経由の代替アプローチか、公式クロスプラットフォームSDK経由の
   移植支援ツールとしての位置づけを検討。
6. TRON/ITRONは、「デスクトップ/モバイル向け汎用互換OS」の基盤としてよりは、
   組み込み機器向けの将来分岐(PS5/6・Switch2等のゲーム機は独自RTOSに
   近い制約を持つ可能性がある)で再検討する方が筋が良い可能性がある。
7. RPoem/open-directx等、このエコシステムが既に持つRust製の
   クロスプラットフォーム実行基盤(Vulkan経由のGPU抽象化等)を、
   DreamOSの「共通実行基盤」として再利用できないか、open-directxの
   完成度が上がった段階で具体的に検討する。

## エコシステム内での位置づけ(既存プロジェクトとの関係、正直な暫定案)

- **[`open-directx`](https://github.com/aon-co-jp/open-directx)**:
  Windows専用API(DirectX)で書かれた既存アプリ/ゲームを、Linux・
  Android・将来的にはmacOS・PlayStationファミリーでも動かす
  クロスプラットフォームDirectX互換層(D3D API呼び出しのインターセプト+
  DXBC/DXILシェーダーの実行時翻訳→Vulkan実行、2026-07-25開発着手)。
  **ユーザー指示により、DreamOSより先にopen-directx自体の完成度を
  優先して高める方針**(2026-08-06時点)。
- **[`open-cuda`](https://github.com/aon-co-jp/open-cuda)**:
  CUDA互換のGPU計算基盤(`opencuda-vulkan`が実Vulkanバックエンド)。
  open-directxが実行時にVulkanディスパッチを行う際の基盤として既に
  利用されている。
- **[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm)**:
  LLM推論(GPT-2等)、`open-cuda`のVulkan GEMM/softmaxカーネルを利用。

DreamOSがこれらとどう統合されるか(例: open-directxのVulkan実行基盤を
共有アプリケーションサービス層として再利用する、複数プラットフォームへの
配布・パッケージング機構を提供する等)は**未確定**。推測で設計図を
確定させず、まずopen-directxの完成度向上(現在最優先で進行中)を通じて
「実際に複数OS上でクロスプラットフォームな実行基盤がどう機能するか」の
実証を積んでから、DreamOSのスコープを具体化する方針とする。

## 対応プラットフォーム(ロードマップ、2026-08-06に実機ベースで更新)

**フェーズ0(現在)**: 構想・文書化のみ。

**フェーズ1(実機あり、今すぐ着手可能)**:
1. Windows(NVIDIA GT730搭載PC実機)
2. Android(スマホ数台実機)

**フェーズ2以降(将来、ライセンス/許諾取得を前提とした保留)**:
3. macOS / iPhone(Apple Developer Program。iPhone側は上記調査結果の
   技術的制約——非公式ハードウェアでのネイティブ実行が不可能——も残る)
4. 各種Linuxディストリビューション
5. SONY PlayStation 5 / PlayStation 6(公式開発機材・NDA)
6. Nintendo Switch 2(公式開発機材・NDA)

**正直な開示**: macOS/iPhone・PS5/6・Switch2は各社の非公開SDK・NDA・
実機/開発機材が必須であり、現時点でこのエコシステムに実機・SDK・
開発者ライセンスは一切無い。ユーザーの2026-08-06指示により「将来
ライセンスが取得できたら着手する」という前提の保留とし、まずは実機の
あるWindows・Androidから着手する方針とした(詳細は下記「スコープの
絞り込み」節参照)。

## VPS/ローカルとの対応(2026-08-06新設)

- ローカル: `F:\runo\dream-os`(このclone)。
- VPS(conoha): `/root/dream-os`(新設予定、[`PORTING.md`](PORTING.md)参照)。

## スコープの絞り込み(2026-08-06、ユーザー指示による現実路線への転換)

ユーザー指示: 「現状では、GT730のWindows PCとAndroidスマホ数台しか
なく、メーカーのライセンスが必要な物は、将来解決する前提で将来許可を
もらう前提で今は、出来る事から始めて行きましょう!」

**現時点で実際に持っている実機・実権限に基づいたスコープ**:

- **今すぐ着手できる対象(実機あり)**: **Windows**(NVIDIA GT730搭載PC、
  `open-directx`/`open-cuda`が既にこの実機で実機検証を積んでいる)・
  **Android**(スマホ数台、`open-web-server`/`open-easy-web`のAndroid版で
  既にAPK配布・実機検証の実績あり)。
- **将来、ライセンス/許諾が取得できてから着手する対象(現時点では
  実機・SDK・開発者ライセンス無し)**: macOS/iPhone(Apple Developer
  Program、iOS側はさらに「非公式ハードウェアでのネイティブ実行は技術的に
  不可能」という上記調査結果の制約も残る)、SONY PlayStation 5/6
  (公式開発機材・NDA)、Nintendo Switch 2(公式開発機材・NDA)。
  これらは**「将来許諾を得られたら着手する」という前提つきの保留**であり、
  スコープから完全に外すわけではない。
- Linux×TRONハイブリッドカーネルという最終ビジョン自体は変更しないが、
  **最初のPoC(概念実証)は「Windows実機+Android実機」の2プラット
  フォームに限定**し、実際に動くものを積み上げてから次のプラット
  フォームへ拡張する方針とする(上記調査結果で確認した通り、Android
  はLinuxカーネルベースのため技術的難度が比較的低く、最初の一歩として
  現実的)。

- 次にすべきこと: (1) Windows PCの実機(GT730)を使い、既存の
  `open-directx`(DirectXシェーダー→Vulkan翻訳)・`open-cuda`
  (Vulkan計算基盤)を土台に、DreamOSの「共通実行基盤」としての
  再利用可能性を具体的に検証する、(2) Android側は`open-web-server`/
  `open-easy-web`のAndroid実装(`cargo ndk`クロスビルド、実機検証済み)を
  参考に、DreamOS用の最小Android連携PoCを検討する、(3) 上記2つが
  実際に「同じ実行基盤の上で」動くところまで確認できて初めて、
  「ハイブリッドカーネル」という中核テーマへ着手する土台ができたと
  言える——現時点ではまだその手前の段階であることを正直に記録しておく。

## PoC実装(2026-08-06、初のコード着手)

ユーザー指示「グラフィックボードなどの電力出力調整も可能なマイニングOS
も取り込んで」+「PoC設計・コード着手して下さい」への対応として、初めて
コードを書いた。**新規`crates/dream-os-kernel`**(Cargoワークスペース、
`Cargo.toml`をルートに新設):

1. **`src/lib.rs`**: `open-cuda`の`opencuda-vulkan`(既に実機Windows
   GT730・Android `aarch64-linux-android`クロスコンパイル実績のある
   Vulkan Computeバックエンド)をpath依存で再利用し、`open_device()`・
   `dispatch_vector_add_once()`という薄いラッパーを提供。「ゼロから
   独自カーネルを書く」のではなく「実績のある既存クロスプラットフォーム
   基盤を、DreamOSの共通実行層として組み立て直す」というChromeOS/
   crosvm型のアプローチを踏襲(CLAUDE.md技術調査結果の結論通り)。
2. **`src/power_profile.rs`**: グラフィックボード等の電力出力調整が
   可能なマイニングOS向けの`MiningPowerProfile`(0〜100%の
   `power_percent`)。**正直な開示**: `nvidia-smi -pl`のようなハード
   ウェアレベルの電力制限APIではなく(ベンダー固有APIに依存しない
   クロスプラットフォーム設計のため)、ディスパッチ間に休止を挟む
   **ソフトウェア側のデューティサイクル制御**で実効稼働率を調整する
   方式。単体テスト5件(0%=停止・100%=休止無し・50%/10%での期待休止
   時間・100超のクランプ)。
3. **`examples/dispatch_throttled.rs`+`shaders/vector_add.comp`**:
   `glslc`でコンパイルしたSPIR-Vを実Vulkanデバイスへ複数回ディスパッチし、
   `MiningPowerProfile`で実効稼働率を落とせることを実測する検証プログラム。

**実機検証(型チェックのみで完了と報告しない、エコシステム既存ルール
徹底)**: `cargo test --release`(5件全green)、
`cargo run --example dispatch_throttled --release -- 50 5`を実際に
Windows実機(NVIDIA GeForce GT 730)で実行し、`vector_add`の計算結果が
正しいこと(誤差チェック内蔵)、電力出力50%指定で実効デューティサイクル
約43.2%(目標50%に近い値、休止時間の粒度による誤差)を実測確認。
Android向けは`ANDROID_NDK_HOME`(27.1.12297006)を設定した上で
`cargo ndk -t aarch64-linux-android build --lib`を実行し、実際に
`aarch64`向けの`.rlib`が生成されることを確認(`opencuda-vulkan`・
`opencuda-core`・`opencuda-ir`を含め全クレートがクロスビルド成功)。

**正直な開示・未検証事項**: (1) Android実機での実行(実際にVulkan
デバイスを開けるか、`vkCreateInstance`が成功するか)は未検証——
`open-cuda`側の2026-07-25監査と同じく「クロスコンパイル成功」の段階に
留まる、実機/エミュレータでの動作確認は次回課題。(2) この`dispatch_
throttled`例はコンピュートシェーダ1本(`vector_add`)のみで、実際の
マイニングアルゴリズム(SHA256等のハッシュ計算カーネル)は未実装——
今回は「電力出力調整の仕組み」の実証に絞った。(3)
`nvidia-smi`相当のハードウェア電力制限APIとの連携(より正確な電力
制御)は将来の課題として`power_profile.rs`のdocに明記済み。

- 次にすべきこと: (1) 実際のマイニング相当のGPU計算カーネル
  (ハッシュ計算等)を`dream-os-kernel`に追加する、(2) Android実機/
  エミュレータでの実行検証、(3) open-directxの完成度向上が優先方針の
  ため、本格的な機能拡張よりもopen-directx側の進捗を待ちつつ小さく
  育てる方針を継続する。

## HANDOFF(直近の作業ログ、上が最新)

- **2026-08-06(続き2) 実機ベースでスコープを絞り込み**: 上記
  「スコープの絞り込み」節を参照。macOS/iPhone・PS5/6・Switch2は
  「将来ライセンス取得を前提とした保留」へ変更し、Windows(GT730実機)・
  Android(実機数台)の2プラットフォームで着手できることから始める方針に
  転換。まだ具体的なコード実装・PoC設計には着手していない(次回以降)。

- **2026-08-06(続き) 日英でのGoogle/GitHub調査を実施**: ユーザー指示
  「まずは、日本語と英語でGoogleとGithub検索と調査から始めて下さい」への
  対応。詳細は上記「技術調査結果」節参照。要点:
  1. Linux×TRONのハイブリッドカーネルは、2001年のIEEE論文に組み込み
     向けの先行研究はあるが、現行メンテナンスされているOSSは無く、
     GitHub検索でも該当リポジトリ0件。TRONとPOSIX系の設計思想の
     根本的な相性の悪さを指摘する分析も見つかった。
  2. Windows/macOS/Androidアプリを単一OSで同時に動かす製品・OSSは
     見つからなかったが、個別要素技術(Wine/Proton、Darling、Waydroid)は
     実在し、それらを「共通VMM基盤上で複数ゲストOSとして統合管理する」
     ChromeOS(crosvm)方式が最も実績のある現実的な先例だと判明。
     seL4+Genode/Neptune OSの「マイクロカーネル+OSパーソナリティ」方式も
     参考になる。
  3. **重要な発見**: iOSアプリをApple以外のハードウェアでネイティブ
     動作させる実用的な方法は現時点で存在しない(Apple自身がサードパーティ
     アプリのJIT・仮想化をOSレベルで全面禁止しているため)。この制約は
     構想全体に関わる重要な事実としてユーザーへ直接報告する。
  4. PowerShellのクロスプラットフォーム動作は既に解決済みの技術
     (pwsh/.NET Core、Linux/macOSでネイティブ動作)であり、DreamOS側で
     独自実装する必要は無い。
  - 次にすべきこと: (1) 上記調査結果、特にiOSの技術的制約についてユーザーの
    判断を仰ぐ(スコープから外す/代替アプローチにするか等)、(2) 採用する
    なら「ChromeOS/crosvm型のVMM統合アーキテクチャ」を軸に、次のフェーズ
    (具体的な技術選定・PoC設計)へ進む、(3) open-directxの完成度向上が
    優先方針のため、DreamOSの実装着手はそちらが一段落してからが現実的。

- **2026-08-06 新規作成**: ユーザー指示「(将来的には)今から新規リポジトリと
  VPSとローカルにもフォルダを新規作成して下さい。名前は、一般名称は
  DreamOS リポジトリやVPSやローカルドライブのフォルダ名は、dream-os」を
  受けて着手。GitHub上に2026-07-01作成済みの空リポジトリ`aon-co-jp/dream-os`
  (コミット0件)が既に存在していたため、それをローカルへclone・
  ブートストラップする形で対応(`open-directx`と同じ経緯)。ユーザーへの
  事前確認(`AskUserQuestion`)により、(1) GitHub作成先は`aon-co-jp`、
  (2) 初回スコープは構想フェーズのみ(コードは書かない)、との回答を得て
  実施。README.md(日本語)・README-English.md・本CLAUDE.md・PORTING.mdを
  新規作成しpush。
  VPS(conoha)側にも`/root/dream-os`フォルダを実際にSSHで新設し
  (プレースホルダの`README.md`のみ配置、コード・サービス登録は無し)、
  `ls /root/`で他プロジェクトと並んで存在することを確認済み。
  - 次にすべきこと: (1) open-directxの完成度がある程度高まった段階で、
    DreamOSの具体的な技術スコープ(open-directxのVulkan実行基盤を
    どう共有するか等)を再検討する、(2) VPS側の`/root/dream-os`は
    現状プレースホルダのみのため、実際にコードを書く段階になったら
    ローカルと同様にgit clone/git管理下へ置き換える。
