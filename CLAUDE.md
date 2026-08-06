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

### 2.5. メインフレーム(IBM z/OS系・AWS版)互換性について(2026-08-06追加調査)

ユーザー指示「dream-osに、汎用機(メインフレーム)OSのIBMとその他の
バージョンとAWS版のメインフレームなども取り込んで互換性を保って」への
対応として調査。**結論**: 汎用機OS(z/OS等)を「取り込んで互換性を保つ」
には、命令セットエミュレーションと実OSライセンスという**別々の2つの
壁**があり、後者はこのエコシステムの他の保留プラットフォーム
(macOS/iPhone・PS5/6・Switch2)と同種の「ライセンス取得待ち」問題である
ことが分かった。

- **[Hercules](http://www.hercules-390.org/)(命令セットエミュレータ、
  OSS/OSI認定ライセンス)**: System/370・ESA/390・64bit
  z/Architectureの命令セット・チャネルプログラムをソフトウェアで
  実装したエミュレータ。1999年開発開始、現在も
  [sdl-hercules-390](https://sdl-hercules-390.github.io/html/hercfaq.html)
  としてアクティブにメンテナンスされている。Windows/Linux/macOS/
  Solaris/FreeBSDに対応(2026年4月時点でIBM AIX/POWERへの
  ネイティブ移植も進んでいるとの情報あり)。**重要な制約**: Herculesは
  あくまで「CPU命令セット+入出力デバイスのエミュレーション」であり、
  **OS自体は同梱しない**——動かすには別途OSイメージ(ディスク/テープ
  イメージ)が必要。
- **z/OS自体のライセンス問題(最大の壁)**: 調査結果より引用——
  「OS/390・z/OS、およびその他のESA/z/Architecture OSは特定の
  マシンにライセンスされており、実際にIBMからライセンスを取得しない
  限りPC上で動かすことはできない」。**したがって、実際のIBM z/OSを
  DreamOSへ合法的に組み込むことは現時点で不可能**——ライセンス費用・
  IBM社との契約が別途必要。ホビイスト向けに合法的に動かせるものとしては
  **MVS 3.8j**(1970年代のIBM製OS、著作権が事実上パブリックドメイン
  扱いとして広く配布されている)が実務上の唯一の選択肢だが、これは
  現行のz/OSとはアーキテクチャ世代が大きく異なる(z/Architectureでは
  なくSystem/370世代)。
- **「AWS版のメインフレーム」について**: 調査の結果、これは
  **エミュレートされたメインフレームOSという意味ではなく、
  [AWS Mainframe Modernization](https://docs.aws.amazon.com/prescriptive-guidance/latest/replatform-mainframe-apps-shared-db2/mainframe-modernization.html)/
  [AWS Transform for mainframe](https://aws.amazon.com/about-aws/whats-new/2025/05/aws-transform-mainframe-generally-available)
  という、既存z/OSアプリ(COBOL/PL1等)をクラウドへ移行・
  リプラットフォーム/リファクタリングするための**AWSのマネージド
  サービス・ツール群**であると判明した。つまり「メインフレームOSの
  実行環境そのもの」ではなく「メインフレーム上で動いていたアプリケー
  ションをクラウドネイティブなアーキテクチャへ変換する開発支援
  サービス」であり、DreamOSが目指す「OSレベルの互換性」とは**そもそも
  対象のレイヤーが異なる**(アプリケーション移行ツールであり、
  カーネル/OS互換レイヤーではない)。z/OS上のCOBOLプログラムをx86
  (Linux/Windows)上でそのまま動かすには、Micro Focus Enterprise
  Server等の別のツールセットでの互換性エラー対応が必要になる、との
  記述も確認した。
- **正直な結論**: (1) 命令セットエミュレーション自体(Hercules)は
  OSSで実在し合法的に利用可能——これはDreamOSに技術的に組み込める
  可能性がある要素。(2) しかし実際のz/OS・その後継OSを動かすには
  IBMからの正式ライセンスが必須で、現時点でこのエコシステムには
  それが無い。(3) 「AWS版メインフレーム」は互換レイヤーではなく
  別レイヤーの移行支援サービスであり、DreamOS側で「互換性を保つ」
  対象として直接統合する性質のものではない。

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
7. **IBMメインフレーム(z/OS系)互換(2026-08-06追加)**: 命令セット
   エミュレーション自体(Hercules、OSS)は技術的に組み込み可能な要素だが、
   実際のz/OS実行にはIBMからの正式ライセンスが必要(現時点で未取得)。
   「AWS版メインフレーム」はOS互換レイヤーではなくアプリケーション移行
   支援サービス(詳細は上記「技術調査結果」2.5節参照)であり、DreamOS側で
   直接統合する対象ではないと判断。

**正直な開示**: macOS/iPhone・PS5/6・Switch2・IBMメインフレーム(z/OS系)は
各社の非公開SDK・NDA・実機/開発機材・正式ライセンスが必須であり、
現時点でこのエコシステムに実機・SDK・開発者ライセンスは一切無い。
ユーザーの2026-08-06指示により「将来
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

## World Laboratory構想の本格設計(2026-08-06、`docs/world-laboratory-design.md`新設)

ユーザー指示「C(World Laboratory構想の本格設計)とD(一旦区切って
open-directx優先に戻る)を集中して」への対応。[`docs/world-laboratory-
design.md`](docs/world-laboratory-design.md)を新設し、BOINC/Folding@home/
Golem Network/Petalsという実在の先行事例から教訓を抽出した3層アーキ
テクチャ案(コーディネータ層=RPoem/open-web-server再利用、ワーカー層=
`dream-os-kernel`拡張、実行基盤層=既存opencuda-vulkan)、ワークユニット
設計、N-of-M多数決による結果検証(既存の`open-web-server-ledger::
MultiRegionReplicator`と同じパターンの転用可能性)、Sybil耐性の課題、
フェーズ0〜4の段階的ロードマップを記載。**今回は設計文書のみ、実装は
一切行っていない**。
- 次にすべきこと: フェーズ1(信頼できるノード群内でのコーディネータ↔
  ワーカー間の最小実装)へ着手するかどうかは、open-directxの完成度向上が
  優先方針のため、次回以降ユーザーと相談の上で判断する。

## マイニング相当の実ハッシュ計算カーネル+Android実機検証(2026-08-06)

ユーザー指示「マイニング相当の実ハッシュ計算カーネル追加、または
Android実機検証の両方を同時に進めて実際にグラフィックボードでマイニング
も可能にして」への対応。

**実装**: `crates/dream-os-kernel/src/mining.rs`(新規)+
`shaders/sha256d_mine.comp`(新規)——32バイト固定長メッセージに対する
double-SHA256(Bitcoin等のPoWで実際に使われる構成)を実Vulkanデバイス上で
バッチディスパッチする`MiningWorker`。**`open-cuda`側にも小さな変更を
加えた**: `opencuda-vulkan::VulkanDevice::launch_kernel`はカーネル名の
ホワイトリスト方式(`vector_add`/`matmul`/`raid6_*`/`softmax`のみ)だった
ため、`sha256d_mine`カーネルを同じ設計パターン(`ensure_*_args`+
`run_*_spirv`+`dispatch_spirv`共有経路)で追加した
(`open-cuda/crates/opencuda-vulkan/src/real.rs`、`cargo build -p
opencuda-vulkan --features real-vulkan --release`で警告0件を確認)。

**実機検証(Windows、NVIDIA GeForce GT 730)**: `tests/mining_real_vulkan.rs`
——GPU計算した64個のdouble-SHA256ダイジェストが、RustCrypto製`sha2`
クレートによるCPU参照実装と完全一致することを実証(`cargo test --release
--test mining_real_vulkan`)。**実装過程で見つけた実バグ**: 当初CPU参照側の
メッセージバイト順を`to_le_bytes()`で組み立てていたため不一致になった
——シェーダ側は`m0..m7`をそのままSHA-256のビッグエンディアンメッセージ
ワードとして使う設計のため、CPU参照側も`to_be_bytes()`で組み立てる必要が
あった(修正後、64件全て一致)。`examples/mine_benchmark.rs`でハッシュ
レート計測(このマシンで約0.4〜0.53 MH/s)も実測。

**実機検証(Android、Moto G53Y 5G実機、`adb`接続確認済みの実デバイス)**:
`cargo ndk -t aarch64-linux-android build --example mine_benchmark
--release`でクロスビルドし、`adb push`(**正直な開示・ハマった点**:
Git Bash〈MSYS〉から`adb.exe`〈ネイティブWindows実行ファイル〉へUnix風の
絶対パスを渡すと、MSYSの自動パス変換が誤動作し`push`が壊れたファイルを
転送する事象に遭遇——`PowerShell`ツール経由での`adb push`に切り替えて
解決した。次回同様の作業をする際はBashではなくPowerShellから`adb`を
呼ぶこと)で実機へ配置、実際に実行した:
- **実際に`OpenCUDA Vulkan Device (Adreno (TM) 619)`という実GPU名で
  デバイスが開けた**——`open-cuda`側の2026-07-25監査(クロスコンパイル
  成功のみ確認、実機実行は未検証)を今回のPoCで一歩進め、**Android実機
  での`vkCreateInstance`成功・実ディスパッチ成功を初めて実証した**。
- 1バッチ目(1,048,576ハッシュ)は実際に成功し完了(0.48 MH/s、Windows
  GT730とほぼ同等の実測値)。
- **2バッチ目で`vkQueueSubmit failed: The logical device has been
  lost`(実ドライバのTDR〈Timeout Detection and Recovery〉相当のエラー)
  が発生**——正直に記録する実際の発見。モバイルGPUドライバは長時間の
  単一コンピュートディスパッチに対して、デスクトップGPUより厳しい
  タイムアウト制約を持つことが知られており、大きなバッチサイズ
  (`DEFAULT_BATCH_SIZE=1048576`)を1回のディスパッチにまとめる現在の
  設計はAndroid実機では2回目以降に不安定になることが実機で確認できた。
  **次回の課題**として、バッチサイズを小さく分割してディスパッチする
  (またはタイムスライシング)設計への変更が必要。

## 複数GPU・NPU・大規模スケール("1枚〜数千枚"・NPU複数・LLM同時稼働)について(正直な開示、未実装)

ユーザーから「1枚から十数枚から数千枚の想定のグラフィックボードとあれば
NPUも同時稼働可能で複数のNPUにも対応してマイニングやLLMも可能に」との
指示があった。調査・設計検討の結果、以下を正直に記録する:

1. **単一マシン内の複数GPU**: `opencuda_vulkan::VulkanDevice::new(id)`を
   実際にソースコードで確認したところ、`id`引数は物理デバイス選択には
   使われておらず、**常に「computeキューを持つ最初に見つかった物理
   デバイス」を開く**設計だった(`open-cuda/crates/opencuda-vulkan/src/
   real.rs`)。つまり現状、1台のマシンに複数のGPUがあっても2枚目以降を
   明示的に選んで開く手段が無い——`open-cuda`側に物理デバイスインデックス
   指定機能を追加する必要がある**既知のギャップ**(`mining.rs`の
   モジュールdocに明記済み)。このマシンにはGT730が1枚のみのため、
   複数GPUでの実機検証はそもそも不可能。
2. **NPU対応**: `opencuda-vulkan`はVulkan Compute専用設計であり、NPU
   (ベンダー固有の推論アクセラレータAPI、例: Qualcomm Hexagon NPU
   〈SNPE/QNN〉・Apple Neural Engine・Intel NPU等)は全く別のAPI体系
   ——Vulkanでは触れない。NPU対応には、ベンダーごとの別バックエンド
   クレートを新規に書く必要があり、今回は着手していない。
3. **「数千枚」規模・LLM同時稼働**: 1台のマシンにGPUを数千枚搭載する
   ことは現実的ではなく、実現するには**複数マシン(ノード)にまたがる
   分散処理**(ネットワーク越しのタスク分配・結果集約プロトコル)が
   必要になる——これは単一プロセスのVulkanディスパッチとは全く別の
   レイヤーの設計であり、今回のPoCの範囲を大きく超える。下記
   「World Laboratory構想」節で、この方向性についての調査結果を記録する。

- 次にすべきこと: (1) Android実機での安定動作のためバッチサイズ分割
  設計、(2) `opencuda-vulkan`への物理デバイスインデックス指定機能の
  追加(open-cuda側の変更、複数GPU実機が無いためこのエコシステムでの
  検証は現状不可能)、(3) NPUバックエンドの新規設計(着手前に日英調査
  必須)、(4) 下記World Laboratory構想の技術調査。

## World Laboratory構想(分散ボランティアコンピューティング、2026-08-06、調査のみ・未実装)

ユーザーの提案「世界中のインターネットに接続しているパソコンやタブレット
やスマホなどでワールドラボラトリー構想として、個人レベルでは持てない
研究所レベルや大学やメーカーレベルのAI開発環境を世界中で協力して、自宅の
安価なPCとオンライン環境でもAI開発を手助けできる可能性」について、
**今回はコードを書かず、実在する先行事例の調査のみ**を行った(規模が
非常に大きい構想のため、着手前にまず現実的な先例を把握する)。

**実在する先行事例(すべて実際に稼働してきた実績のあるボランティア
コンピューティング/分散コンピューティングの枠組み)**:
- **[BOINC](https://boinc.berkeley.edu/)**(UC Berkeley発、2002年〜):
  SETI@home・Folding@home等で使われた、世界中の個人PCの余剰計算資源を
  科学計算(創薬・気候変動・宇宙探査等)へ提供するボランティア
  コンピューティングの標準基盤。**教訓**: 中央のタスクサーバーが
  ワークユニットを分割配布し、クライアントが結果を返す方式。信頼性の
  低い個人PCからの結果検証(複数クライアントへ同一タスクを配って結果を
  突き合わせる等)が必須設計要素。
- **[Folding@home](https://foldingathome.org/)**: タンパク質folding
  シミュレーションのボランティア分散計算、GPU/CPU双方対応。COVID-19研究
  でも実際に使われた実績あり。
- 分散AI学習・推論に特化した近年のプロジェクトとしては、ブロックチェーン
  ベースの分散GPU市場([Golem Network](https://www.golem.network/)等)や、
  近年の分散LLM推論プロジェクト(Petals等、大規模モデルを複数の
  ボランティアノードで分割推論する仕組み)も実在する。

**正直な結論**: この構想自体は技術的に前例のある方向性であり、
「不可能」ではない。しかし実現には(1)タスク分配・結果検証・信頼性の
低いノードへの耐性を持つ分散システムの設計、(2)ネットワークプロトコル
(現状の`dream-os-kernel`は単一プロセス内のVulkanディスパッチのみ)、
(3)セキュリティ(悪意あるノードによる不正な計算結果の混入対策)、
という単一マシンでのGPU/NPU活用とは全く異なるレイヤーの本格的な
新規開発が必要であり、**今回のセッションでは調査・方向性の記録に留め、
実装には着手していない**(現在のPoCの優先度・open-directx優先方針との
兼ね合いもあり、次回以降に本格的な設計フェーズとして着手するかを
ユーザーと相談する)。

## メインフレーム(IBM z/OS系・AWS版)互換性の調査(2026-08-06)

ユーザー指示「dream-osに、汎用機(メインフレーム)OSのIBMとその他の
バージョンとAWS版のメインフレームなども取り込んで互換性を保って」への
対応として日英調査を実施(詳細は上記「技術調査結果」2.5節参照)。
要点: (1) 命令セットエミュレータ`Hercules`はOSSで実在・現行メンテナンス
継続中——技術的に組み込み可能な要素。(2) しかし実際のz/OS実行には
IBMからの正式ライセンスが必須で、現時点でこのエコシステムには無い。
(3) 「AWS版メインフレーム」はOS互換レイヤーではなく
AWS Mainframe Modernization/AWS Transform for mainframeという
**アプリケーション移行支援サービス**であり、DreamOSが目指す
OSレベル互換性とは対象レイヤーが異なると判明。よってIBMメインフレーム
互換は「フェーズ2以降(将来のライセンス取得を前提とした保留)」の
プラットフォームロードマップへ7番目として追加した(macOS/iPhone・
PS5/6・Switch2と同じ扱い)。コード変更は今回無し(調査・文書化のみ)。
- 次にすべきこと: 実IBMライセンス取得の目処が立った場合、Hercules
  (OSS命令セットエミュレータ)をどう組み込むかの技術検討に着手する。
  それまでは他の保留プラットフォームと同様、優先度は低いまま維持する。

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
