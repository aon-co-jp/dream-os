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

## 長期ビジョンの詳細化(2026-08-08、ユーザー指示「あくまで野心として」)

ユーザーから、最終到達点(野心)としてのDreamOSカーネル像がより具体的に
示された。**ユーザー自身が「あくまで野心として」と明言している**ため、
これは上記「スコープの絞り込み」節が定める**現実的な近未来の着手範囲
(Windows実機+Android実機のみ)を変更するものではなく**、その先にある
最終ゴールの解像度を上げる位置づけの記録である。

- **カーネルの中核**: LinuxとAndroid OS(Linuxカーネルベース)を中心に
  据える。
- **Windows互換**: PowerShell、Windowsの3DオンラインゲームアプリやCAD
  アプリが高速・スムーズに動作すること。
- **Linux/macOSアプリ互換**も前提とする。
- **ゲーム機互換(将来予定)**: PS6(2026-08-08時点で未発売・未発表の
  次世代機)のゲームが4K120FPS・Dolby Vision 2 Max
  (2026-08-08訂正: 当初「ULTRA」と記録していたが「Max」が正しい名称との
  ユーザー訂正を受けて修正。2026-08-08時点でこのセッションからは実在する
  公開仕様として裏付けが取れていない——将来正式仕様が公開された時点で
  改めて確認すること)相当で動作すること、Nintendo Switch 2のゲーム
  アプリの動作互換も予定。

**正直な評価(現時点の技術的距離)**: 上記は、Windows向けゲーム/CAD
アプリを別OS上で高速に動かす(Wine/Protonのような互換レイヤー、または
仮想化・GPUパススルーを伴うハイパーバイザー相当の技術)、および家庭用
ゲーム機のソフトウェアを非公式に動作させる(コンソールエミュレーション)
という、いずれも成熟した専門プロジェクト(Wine/Proton、QEMU/KVM、
各種コンシューマ機エミュレータ)が何年もかけて築いてきた領域に相当する。
現在の`dream-os`の実体(`crates/dream-os-kernel`のSBM Isingカーネル
PoC・Android実機での`open-cuda`/`open-directx`連携PoC・本日追加した
`dream-os-raid-bridge`)から見ると、この野心までの距離は非常に大きい
——「野心として」の位置づけをそのまま維持し、当面のPoCはスコープの
絞り込み節が定める実機ベースの範囲(Windows実機+Android実機)に
留めることを推奨する。

**法的・ライセンス面の留意事項(正直な開示)**: PS5/6・Nintendo Switch
シリーズの正式な開発対応には各社の公式開発機材・NDA・ライセンスが必須
(既存の「対応プラットフォーム」節に記載済み)。加えて、非公式な
コンシューマ機ソフトウェア実行環境(エミュレーション)は、クリーン
ルーム実装(リバースエンジニアリングのみに基づく独自実装)であれば
一般に違法ではない国が多い一方、**製造元の暗号化キー・BIOS/ファーム
ウェアの無断使用、DRM/コピープロテクションの回避は法域によって
DMCA等の反回避規定に抵触しうる**——将来この領域へ実際に着手する場合は、
必ず公式ライセンス経路(上記NDA取得)を前提とし、非公式な回避手段には
踏み込まない方針を維持すること。

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

## World Laboratory構想の本格設計(2026-08-06、[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)新設)

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

## open-directx連携+aruaru-db永続化を実装(2026-08-06)

ユーザー指示「dream-osをopen-directx・open-cuda・open-web-server・RPoem・
open-raid-z・aruaru-dbなどをベースに関連技術を実装開発、完成度と連携性の
向上をして」への対応。2つの具体的な実機検証済みブリッジを実装した。

### open-directx連携(`crates/dream-os-kernel/src/directx_bridge.rs`)

`open-directx`(`directx-shader-translate`、path依存で再利用)のDXBC
(Windows専用D3D11 Compute Shaderバイトコード、`fxc.exe`実コンパイル済み)
→SPIR-V翻訳を、dream-os-kernelのVulkan実行基盤へ直接接続した。
**「Windows専用DirectXコンピュートシェーダーバイナリが、DreamOSの
Windows/Android共通Vulkan実行層でそのまま動く」ことを実証**——
open-directx・open-cuda・dream-osの3リポジトリの連携性を具体的な
コードで示す最初の実装。

**実機検証(NVIDIA GT730)**: `tests/directx_bridge_real_vulkan.rs`——
open-directx側の実`vector_add.dxbc`(`include_bytes!`で直接取り込み)を
翻訳・ディスパッチし、256要素すべてCPU参照実装と一致することを確認。

### aruaru-db永続化(`crates/dream-os-wire/src/aruaru_persistence.rs`)

World Laboratory構想([world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md))の通信・永続化層
(第4層)のうち、これまで「実DBインスタンスが無いため未配線」としていた
永続化部分を、`aruaru-db`(ACID互換+Git-on-SQLバージョン管理)へ実際に
接続する形で実装した。`tokio-postgres`で`aruaru-server`(pgwire)へ
接続し、`WorkResultEnvelope`をINSERT+`aruaru_commit`でコミット、
バージョン管理されたcommit_idを取得する。

**実機検証**: 実際に`aruaru-server`(既存ビルド済みバイナリ、
`ARUARU_USERS`環境変数で認証情報設定)をローカルで起動し、
`tests/aruaru_db_integration.rs`で実接続・INSERT・`aruaru_commit`まで
一気通貫で検証(`commit_id=95932e6f...`を実際に取得)。

**実機検証で発見した実際の制約(正直な開示)**: `aruaru-query`のSQL
パーサーは`INSERT ... VALUES(...)`内の値をクォートを考慮しない単純な
`split(',')`で分割する簡易実装(意図的に絞り込まれたサブセット)。
そのため`result_json`(JSON文字列)にカンマが含まれると
`"INSERT: N columns but M values"`エラーになることを実際の接続で
発見・特定した。dream-os側でBase64エンコードしてから格納する回避策を
実装(aruaru-db側のパーサー自体は変更していない——影響範囲の広い
コア変更を避け、呼び出し側での回避を選択)。

**正直な開示・未実装**: (a) `open-web-server`/RPoemとの直接連携(HTTPコ
ーディネータとしての実配線)は今回未着手——World Laboratory設計文書の
「フェーズ1」相当。(b) `open-raid-z`との直接連携は、aruaru-db自体が
既にopen-raid-zとのZFS互換スナップショット連携を持つため
(`aruaru-dist::raid_z_backend`)、間接的に活用可能だが今回新規のコードは
書いていない。(c) `aruaru-llm`との連携は今回未着手。

**検証**: `cargo build --workspace --release`/`cargo test --workspace
--release`で全クレートregression無し。

- 次にすべきこと: (1) World Laboratoryコーディネータ本体(RPoem/
  open-web-server上への実HTTP API)の実装、(2) aruaru-llmとの連携
  (World Laboratoryのワークロード種別としてLLM推論タスクを追加)、
  (3) open-directxの完成度向上が優先方針のため、引き続き小さく育てる。

## 東芝SBM・DeepSeek・IOWNの調査+東芝SBM(量子アニーリング風最適化)の実装(2026-08-06)

ユーザー指示「東芝の擬似的な量子コンピューター技術で富士通より普通の
グラフィックボード一枚のPCの方が100倍高性能…DeepSeekのグラフィック
ボード数千枚のシステムをグラフィックボード一枚のPCに凝縮、圧縮する
折りたたみ技術や4層通信構成通信にIOWNの通信技術を…調査して、取り込んで」
への対応。日英でGoogle/GitHub再調査を実施。

### 1. 東芝SBM(Simulated Bifurcation Machine) — 実在・実装した

2026年4月、東芝は前世代比10〜100倍高速化の新アルゴリズムを発表、
16GPU構成で100万変数問題を約30分(CPU版シミュレーテッドアニーリングの
約20,000倍高速)で解いたと報告([東芝公式](https://digitalpr.jp/r/44929)、
[EE Times Japan](https://eetimes.itmedia.co.jp/ee/articles/2604/09/news036.html)、
[IEEE Spectrum](https://spectrum.ieee.org/toshiba--optimization-algorithm-speed-record-combinatorial-problems))。
富士通デジタルアニーラは専用ASICで超高速だが変数数8,192までの制約があり、
東芝SBMはGPU上で10万変数以上の大規模問題に対応できる——「グラフィック
ボード1枚が専用ASICに対して優位性を持ちうる」という報道の技術的背景を
確認できた(ただし比較条件〈問題規模・世代〉に強く依存する点は要注意)。

**実装**: SBMの核心アルゴリズムであるballistic Simulated Bifurcation
(bSB、Goto et al. 2019)を`shaders/sbm_ising.comp`(新規)として実際に
Vulkan Compute上へ実装した(64スピンのIsing模型、1ワークグループ内の
共有メモリでx/y状態を保持し`barrier()`で同期しながら時間発展)。
open-cuda側にも`sbm_ising`カーネルのディスパッチ経路を追加(既存の
`ensure_*_args`/`run_*_spirv`パターンを踏襲)。`crates/dream-os-kernel/
src/sbm.rs`(新規)にGPU実行関数+CPU参照実装(全く同じ更新式の逐次計算)
を実装。

**実機検証(NVIDIA GT730)**: `tests/sbm_real_vulkan.rs`——64スピンの
ランダムIsing問題(決定的な擬似乱数で生成)を解き、**GPU版とCPU参照
実装が完全に同一のスピン配置(64/64一致)へ収束すること**を確認
(`cargo test --release --test sbm_real_vulkan`)。`cargo test --workspace
--release`で全クレートregression無し。

**正直な開示**: 「100倍高性能」を主張するものではない——今回のPoCは
64スピンの小規模問題を解く最小実装であり、東芝の商用実装(SQBM+、
離散化アルゴリズムdSB・FPGA対応・100万変数規模)との性能比較は
行っていない。詳細・出典は`crates/dream-os-kernel/src/sbm.rs`の
モジュールdoc参照。

### 2. DeepSeekの「数千枚のGPUを1枚に凝縮・圧縮する折りたたみ技術」— 調査の結果、そのような技術は確認できなかった(正直な訂正)

日英で再調査したが、**「数千枚のGPUのシステムを1枚のGPUへ物理的/
論理的に折りたたむ・凝縮する技術」という報道・技術文書は見つからなかった**。
実際にDeepSeek関連で確認できたのは以下——性質の異なる話であることを
正直に整理する:

- DeepSeekの公式発表では、V3モデルの学習に2,048台のNVIDIA H800 GPU・
  278万GPU時間・推定580万ドルを要したとされる([BytePlus](https://www.byteplus.com/en/topic/409182))。
- 一方、SemiAnalysis等の独立系分析では、実際には最大5万台規模のGPU
  (16億ドル相当のインフラ投資)を保有しているとの指摘もあり、公式発表と
  実態には論争がある([Tom's Hardware](https://www.tomshardware.com/tech-industry/artificial-intelligence/deepseek-might-not-be-as-disruptive-as-claimed-firm-reportedly-has-50-000-nvidia-gpus-and-spent-usd1-6-billion-on-buildouts))。
  いずれにせよ「数千枚を1枚に圧縮した」という主張ではない。
- DeepSeekが実際に開発した効率化技術は**MLA(Multi-Head Latent
  Attention)**——推論時のKVキャッシュのメモリ使用量を圧縮する
  アーキテクチャ上の工夫であり、「物理的なGPUの台数を減らす」ものでは
  なく「1台あたりのメモリ効率を上げる」技術。この方向性自体は
  `open-cuda`が既に持つINT4/INT8/AWQ量子化(`opencuda-blas`、
  実装・検証済み)と概念的に同じ系統(メモリ効率化によるハードウェア
  要求の軽減)であり、**DreamOSとして新たに追加実装すべき固有の技術は
  見つからなかった**——既存のopen-cuda量子化機能がこの方向性の実装と
  して既に存在する、という結論とする。
- 次にすべきこと: 特になし(調査の結果、実装対象と呼べる具体的な技術が
  見つからなかったため)。もしユーザーが指す情報源(特定の記事URL等)が
  あれば、それを教えていただければ再調査する。

### 3. IOWN(NTTのオールフォトニクスネットワーク) — 物理インフラのため直接実装は不可、既存のRS-SmartTCP設計を再確認

IOWN/APN(オールフォトニクス・ネットワーク)は、NTTが構築する光ベースの
物理通信インフラであり、2023年3月にIOWN 1.0(100Gbps専用線サービス)が
提供開始されている([NTT Group](https://group.ntt/jp/group/iown/function/apn.html))。
GitHub上の実装や、ソフトウェアとして再現可能な技術ではない
(物理層の光ネットワーク設備そのもの)。

**このエコシステムでの既存の扱い**(`open-web-server`側で既に調査・
実装済み、今回新たに確認): `RS-SmartTCP`が「IOWN/APNのような超低
遅延・ジッター無し回線を検知した際にリトライ間隔等を積極化する適応
制御」として、IOWN自体ではなくIOWNが実現する回線特性に**適応する
ソフトウェア側の設計**を既に持っている(RTT/ジッター推定を
RFC 6298/RFC 9002と同じSRTT/RTTVAR方式で行う)。dream-os-wireの
将来の4重伝送路対応(今回未実装)でも、この既存のRS-SmartTCP設計を
再利用する方針を[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)へ追記した——
IOWN自体を「実装」することはできないが、IOWNのような低遅延回線が
将来利用可能になった際に自動的に活かせる適応制御という形で、
間接的に「取り込む」ことは可能という結論。
- 次にすべきこと: 実際にdream-os-wireへ複数伝送路対応を実装する際
  (World Laboratory設計文書フェーズ2以降)、RS-SmartTCPをpath依存で
  再利用する。

## World Laboratory構想: 通信・永続化層(第4層)を3層から4層へ拡張・実装(2026-08-06)

ユーザー指示「open-web-server・RPoem・open-raid-z・aruaru-dbのACID
互換・ZFS互換の4層4重通信の技術もdream-osに実装、移植して」+
「もう一度Google検索とGithub調査を日本語と英語でして」への対応。

**再調査結果**: BOINC自体は暗号学的な改ざん検知・リプレイ対策を標準
機構として持たず、「複製配布(同一ジョブを複数の無関係なPCへ)+多数決」
が結果検証の主軸であることを確認([BOINC Wiki: SecurityIssues](https://github.com/BOINC/boinc/wiki/SecurityIssues))。
一方、AEAD+シーケンス番号によるアンチリプレイ(ASN技術)は業界標準
パターンと確認でき、これは`open-web-server-wire::SecureChannel`が既に
実装している設計と完全に一致した。この裏付けを踏まえ、`docs/
[world-lab/docs/world-laboratory-design.md](https://github.com/aon-co-jp/world-lab/blob/main/docs/world-laboratory-design.md)のアーキテクチャを3層から4層(コーディネータ/
**通信・永続化層〈新設〉**/ワーカー/実行基盤)へ拡張。

**実装(設計文書だけに留めず、実際にコードを書いて実機検証)**:
新規`crates/dream-os-wire`(`open-web-server`〈path依存、
`open-web-server-wire`のSecureChannel再利用〉+`open-web-server-core`)。
`WorkResultEnvelope`+`WorkerChannel::submit()`/
`CoordinatorChannel::receive()`という薄いラッパーで、ワーカー→
コーディネータ間の計算結果送信をAEAD暗号化+リプレイ対策で保護する。
`tests/secure_channel_integration.rs`(3件全green): (1) 正常送受信の
往復一致、(2) **同一結果フレームのリプレイ拒否**(多数決の不正水増し
攻撃を模擬)、(3) **改ざんされた結果フレームの拒否**(AEADタグ検証で
復号自体が失敗、でっち上げた偽の計算結果を防ぐ)。`cargo build
--workspace --release`/`cargo test --workspace --release`で全クレート
regression無し。

**正直な開示・実装しなかったもの**: (a) 4重伝送路(TCP/UDP/QUIC/MPTCP、
`open-web-server-wire`に既存)は今回配線せずペイロード層のみ、(b) 4重DB
永続化(PostgreSQL/aruaru-db/マルチリージョン同期/独立監査ログ、
`open-web-server-ledger`)は実DBインスタンスが無いため配線せず設計方針
記録のみ、(c) TLS終端・相互認証(第1層・第2層)は将来のコーディネータ
実装側が担う想定で未検証。
- 次にすべきこと: (1) コーディネータ本体(RPoem/open-web-server上への
  実HTTP APIとしてのワークユニット配布・複製配布N-of-M実装)、
  (2) `open-web-server-ledger`の実配線(実DBインスタンス確保後)、
  (3) open-directxの完成度向上が優先方針のため、本格実装は次回以降。

## A. Androidバッチ分割修正(2026-08-06、実機で`device lost`を解消)

前節で発見した「Android実機で2バッチ目に`vkQueueSubmit failed: The
logical device has been lost`」を修正した。`mining.rs`に
`MOBILE_SUB_BATCH_SIZE`(16,384ハッシュ/ディスパッチ)+
`MiningWorker::mine_batch_split()`(大きな`total_count`を
`sub_batch_size`単位に分割して繰り返しディスパッチ、結果を結合)を
追加。`examples/mine_benchmark.rs`は`cfg!(target_os = "android")`で
Androidビルド時のみ自動的にサブバッチ分割を使うよう変更(デスクトップは
従来通り`DEFAULT_BATCH_SIZE`を1回のディスパッチで使う、性能を落とさない)。

**実機再検証(Android、Moto G53Y 5G、Adreno 619)**: `cargo ndk -t
aarch64-linux-android build --example mine_benchmark --release`で
クロスビルドし、`adb push`(PowerShell経由)で実機へ配置・実行。
**3バッチ(合計3,145,728ハッシュ)すべて成功、`device lost`は再発しなかった**
——修正前は2バッチ目で確実に失敗していたのに対し、明確な改善を実機で確認。
**正直な開示**: スループットは約0.10 MH/s(修正前の1バッチ目実測
0.48 MH/sより低下)——64回に分割したことでディスパッチ間オーバーヘッド
(コマンドバッファ記録・`vkQueueSubmit`・`vkDeviceWaitIdle`の往復)が
64倍になったのが原因。安定性のためのトレードオフとして正直に記録する
(次回、サブバッチサイズのチューニング〈安定性を保ちつつ大きくできるか〉
は再検討の余地がある)。

## B. open-cuda複数GPU対応の修正(2026-08-06)

`opencuda_vulkan::VulkanDevice::new(id)`の`id`引数が物理デバイス選択に
使われていなかった実バグを修正(詳細は下記実装記録)。このマシンには
GPUが1枚(NVIDIA GT730)のみのため、**2枚目以降のGPUを実際に選択できる
ことの実機検証は不可能**——`id`が範囲外の場合に正しくエラーになること、
`id=0`(唯一の実機構成)で従来通り動作することのみ実機検証済み。

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

- **2026-08-08 open-raid-zのRAID6/Z2実装をdream-osへブリッジ(実GPU検証済み)+
  GPU電力調整・LLM推薦機能の実機調査(コード追加は前者のみ、他は正直な
  計画記録)**: ユーザー指示「open-directx・open-cuda・aruaru-llm・
  open-web-server・RPoem・open-raid-z・aruaru-dbの良い部分をdream-osへ
  並行して取り込み、(a)4層4重通信、(b)ACID/ZFS互換、(c)CPU+NPU+GPU
  DirectXアクセラレータ対応のRAID6 Z2実装、(d)4〜16枚NVMe想定のRAID6
  高速化、を統合・実用性・完成度向上」+セッション中の追加指示「(e)
  グラフィックボードの電力調整可能なマイニングOS機能」「(f)推薦LLM
  ダウンロード機能・LLM利用機能」への対応。**大規模な構想のため一度に
  全部作らず、実際に検証できる範囲だけ実装し、残りは正直な設計文書化に
  留めた**(このエコシステムの既存方針通り)。

  ### (c)(d) 実装: 新規`crates/dream-os-raid-bridge`(実GPU検証済み)

  調査の結果、**(c)(d)は`open-raid-z`側に既にほぼ完全な形で実装済み**
  だったと判明——ゼロから作る必要はなく、再利用するだけで済んだ:
  `open-raid-z/open_runo_zfs_source/open_raid_z_core::vdev::RaidZVdev`
  (RAID-Z2/Z3、GF(2^8) Reed-SolomonのP/Qパリティ、`RaidLevel::Raid6`は
  `Z2`のエイリアス)+`zfs_accel_hlsl`(D3D12/DirectML〈Windows〉・
  Vulkan Compute〈Linux/Android、NPU/GPU自動検出、`device::
  detect_best_accelerator`〉)、さらに`open_raid_z_core/examples/
  raidz2_parity_benchmark.rs`で4〜8枚のNVMeを模したループバック
  ベンチマークまで既に存在していた。そこで新規crate
  `dream-os-raid-bridge`(`src/lib.rs`)をpath依存(コード複製ではない、
  `directx_bridge.rs`と同じ方針)で追加し、`detect_parity_accelerator`/
  `build_loopback_raid6`という薄いラッパーを提供、ルート`Cargo.toml`の
  `members`へ追加した。

  **実機検証(NVIDIA GT730、Windows)**: `tests/raid6_bridge_real.rs`
  2件——(1) 4データ+2パリティ(RAID6/Z2)構成でストライプ書き込み→
  読み出しのラウンドトリップ一致、(2) 1台のディスクを直接壊した状態
  からの`read_stripe_with_report`による自己修復(パリティ2本のRAID6は
  1台までのサイレント破損から復元できる)。**`detect_best_accelerator()`
  が実際に`AccelKind::Gpu`(CPUフォールバックではない)を返し、GT730の
  D3D12/DirectML経由でパリティ計算が実行されたことを`--nocapture`の
  実行結果で確認**(`RAID6パリティアクセラレータ: Gpu (CPUフォールバック:
  false)`)——「RAID6 Z2 + GPU DirectXアクセラレータ」という(c)の要求を
  実機で満たせることを実証。`cargo build --workspace --release`/
  `cargo test -p dream-os-raid-bridge --release`で確認、既存クレート
  (同時進行中の別セッションによる`flash_attention_bridge`等の未コミット
  変更を含む)との衝突・regressionは無いことも`cargo build --workspace`
  で確認した。

  **正直な開示・未実装/未検証**: (1) 4〜16枚という要求のうち実機検証は
  6枚構成(4データ+2パリティ)のループバックファイルのみ、実NVMe複数枚
  はこのマシンに無く検証不可能。(2) `opencuda_vulkan`(dream-os-kernelの
  Vulkan基盤)と`zfs_accel_hlsl`の実行基盤は別々のデバイスハンドルの
  ままで統合していない(設計目的が異なるため安易な統合は避けた)。
  (3) NPU経由のパリティ計算はこのマシンにNPUが無いため未検証(コード上は
  `AccelKind::Npu`分岐が存在しCPU/GPUと同じ経路を通る設計だが実機確認は
  次回課題)。

  ### (a)(b) 統合方針の再確認(コード変更なし、既存実装の棚卸し)

  調査の結果、(a)(4層4重通信・金融データ相当の耐障害性)・(b)(ACID/ZFS
  互換)は**dream-os側に既に部分実装済み**であることを再確認した(過去の
  HANDOFF参照): `crates/dream-os-wire`が`open-web-server-wire::
  SecureChannel`(AEAD+リプレイ対策、path依存)を再利用しペイロード層の
  改ざん・リプレイ耐性を実証済み(第1層の4重伝送路〈TCP/UDP/QUIC/MPTCP〉
  は未配線)、`crates/dream-os-wire/src/aruaru_persistence.rs`が
  `aruaru-db`(ACID互換+Git-on-SQL)への実接続・INSERT・コミットを実証
  済み(4重DB書き込み・`open-web-server-ledger`のマルチリージョン
  レプリケーション・独立監査ログは実DBインスタンス不足のため未配線)。
  今回のセッションでは、この2つを更に前進させるコードは書いていない
  (`open-raid-z`ブリッジに集中したため)——次回の課題として維持する。

  ### (e) グラフィックボード電力調整(マイニングOS機能): 実機で調査、
  ハードウェア制約により未実装(正直な開示)

  既存の`crates/dream-os-kernel/src/power_profile.rs`は
  ディスパッチ間の休止によるソフトウェア側デューティサイクル制御のみ
  (ハードウェアの電力制限APIは使っていない)だったため、今回
  NVML相当の実ハードウェア電力制限(`nvidia-smi -pl`等)が実機で使えるか
  調査した。**実際に`nvidia-smi -q -d POWER`・`nvidia-smi --query-gpu=
  power.draw,power.limit,power.min_limit,power.max_limit,power.
  default_limit --format=csv`をこのマシン(NVIDIA GT730、Driver
  475.14)で実行して確認した結果、Power Management/Power Draw/Power
  Limit等が全項目`N/A`だった**——GT730(Kepler世代の下位モデル)は
  NVML/nvidia-smiの電力管理機能自体をハードウェアレベルでサポートして
  いないことを実機で確認した(読み取りすら不可能、調整以前の問題)。
  この結果、(1) 実ハードウェア電力制限の読み取り・調整のいずれも
  このマシンでは検証不可能と判明したため実装を見送った(危険な当て
  推量でのNVML APIコード追加はしない、というユーザー指示の安全方針にも
  合致)、(2) 既存の`power_profile.rs`のソフトウェア側デューティサイクル
  制御が、このエコシステムで現状唯一実機検証可能な「電力調整」手段
  であることを再確認した。
  - 次にすべきこと: 電力管理に対応した実GPU(GT730以外)が入手できた
    場合、`nvml-wrapper`クレート(実在するRust製NVML安全バインディング、
    今回未追加)経由での読み取り→保守的な範囲での調整→読み戻し確認、
    という段階を踏んだ実装を検討する。それまでは`power_profile.rs`の
    ソフトウェア方式を正としてドキュメント化しておく。

  ### (f) 推薦LLMダウンロード・利用機能: aruaru-llmに既に実装済みと判明、
  dream-os側では未実装(正直な開示、reuse方針)

  調査の結果、**この機能は`aruaru-llm`側に既にほぼそのまま実装済み**
  だった: `aruaru-llm/src/hardware.rs`(`opencuda-vulkan`/
  `opencuda-directx`経由のGPU検出→VRAM容量からの推奨モデルサイズ判定、
  DirectX優先・Vulkanはクロスチェック)+`aruaru-llm/src/model_catalog.rs`
  (Hugging Face `resolve/main/`からのGPT-2系モデルのダウンロード・
  インストール、ライセンス注記付き、ユーザー明示リクエスト時のみ実行)。
  **このエコシステムの「既存実装の再利用を優先し重複実装を避ける」という
  一貫した方針に従い、今回dream-os側に同等機能を再実装するコードは
  書いていない**——安易な複製は既存方針(車輪の再発明回避)に反すると
  判断した。
  - 次にすべきこと: dream-osを「推薦LLMダウンロード・利用」機能の
    フロントエンドにしたい場合、`aruaru-llm`をサブプロセス起動
    (`open-web-server`のAndroid実装が使う`ProcessBuilder`方式、
    `android/app/src/main/java/.../MainActivity.kt`と同じパターン)する
    か、HTTP API経由で呼び出すクライアント層を`dream-os-kernel`または
    新規crateとして追加する設計を検討する(aruaru-llm本体は変更しない、
    read-onlyな連携のみ)。今回は時間の都合上、この設計の特定と
    HANDOFF記録までに留めた。

  - 次にすべきこと(全体、優先順): (1) `dream-os-raid-bridge`の実NVMe
    複数枚(4〜16枚)での実機検証(実ドライブ入手後)、(2) (a)(b)の
    4重伝送路・4重DB書き込みの本格配線(引き続き未着手)、(3) aruaru-llm
    クライアント層の設計・実装、(4) 電力管理対応GPU入手後の`nvml-wrapper`
    連携検討、(5) open-directxの完成度向上が引き続き優先方針のため、
    本格拡張は小さく育てる方針を継続。

- **2026-08-08 open-cuda製fused flash-attention SPIR-Vカーネルをdream-os
  の共通Vulkan実行基盤経由で実行、Windows実機・Android実機の両方で実証
  (rs-sync横断セッション、直前2026-08-07(続き3)HANDOFFの「次にすべきこと」
  への対応、`directx_bridge.rs`に続く2つ目の「共通実行基盤」実装例)**:
  1. **新規`crates/dream-os-kernel/src/flash_attention_bridge.rs`**:
     `open-cuda-llm`のDecoderLayerへ既に配線・実機検証済みの
     `opencuda-blas::flash_attention_with_spirv`(QKᵀ・オンラインsoftmax・
     P·Vを1回のディスパッチで完結させるfusedカーネル)を、
     `dream-os-kernel`の`open_device()`(Vulkan実行基盤)経由でそのまま
     呼び出す`dispatch_flash_attention()`を実装。シェーダ本体
     (`flash_attention.spv`)は`open-cuda`側の既存アセットを
     `include_bytes!`でそのまま取り込み(独自コピーは作らない)。
     `Cargo.toml`に`opencuda-blas`をpath依存として追加。
  2. **`examples/dream_os_status.rs`を拡張**: 既存のopen-directx DXBC→
     SPIR-Vディスパッチに続けて、8×16のQ/K/Vでflash-attentionを実行し、
     GPU結果とCPU参照実装(`opencuda-blas::flash_attention`)が数値一致
     するかまで実際に計算・比較して表示するようにした。
  3. **実機検証(型チェックのみで完了と報告しない方針を徹底)**:
     - Windows実機(NVIDIA GT 730): `cargo test --workspace --release`
       (新規`tests/flash_attention_bridge_real_vulkan.rs`含め全件green、
       既存の`directx_bridge`/`mining`/`sbm`/`aruaru_db_integration`/
       `secure_channel_integration`もregression無し)。
       `cargo run --example dream_os_status --release`を実行し、
       `open-cuda flash-attention SPIR-V dispatch: seq_len=8, head_dim=16,
       gpu==cpu_reference: true`・`result: OK`を実機ログで確認。
     - `cargo clippy --workspace --all-targets --release -- -D warnings`
       で、今回のチェーン拡張とは無関係のpre-existing警告2件
       (`directx_bridge.rs`の`manual_slice_size_calculation`、
       `sbm.rs`の`run_sbm_ising`の`too_many_arguments`、clippyのlint
       ルール追加によるもの)を検出・修正し、workspace全体で警告0件を
       達成。
     - **Android実機(Moto G53Y 5G、Adreno 619、シリアル`ZY22J7RFND`、
       `adb`接続確認済み)**: `cargo ndk -t aarch64-linux-android build
       --release --example dream_os_status`でクロスビルドし、
       `adb push`(PowerShell経由)で実機へ配置・実行。
       `vulkan device: OpenCUDA Vulkan Device (Adreno (TM) 619)`・
       `open-cuda flash-attention SPIR-V dispatch: seq_len=8, head_dim=16,
       gpu==cpu_reference: true`・`result: OK`を実機で確認——
       fused flash-attentionカーネルがモバイルGPU実機上でも正しく動作し
       CPU参照実装と一致することを初めて実証した。
     - **Android実機アプリ(APK)経由でも確認**: 上記クロスビルド済み
       バイナリを`android/app/src/main/jniLibs/arm64-v8a/
       libdreamosstatus.so`へ差し替え、`gradle :app:assembleDebug`
       (`BUILD SUCCESSFUL`)→`adb install -r`→実機のボタンを実際に
       タップ→`adb shell screencap`で実機画面をキャプチャし、
       アプリUI上に上記結果が表示されることをスクリーンショットで確認
       (シェーダーは`include_bytes!`でバイナリへ埋め込み済みのため、
       DXBC/flash-attention双方とも実行時の外部ファイル配置は不要——
       単一の実行ファイルをAPKへ同梱するだけで完結する設計)。
  4. **正直な開示・スコープの限界**: (a) 8×16という極めて小さいQ/K/V
     でのPoCであり、GPT-2 124M実寸(`head_dim=64`、`seq_len`は生成長に
     依存)での速度・大規模データでの数値安定性は未検証。(b) これは
     「dream-osの共通実行基盤からopen-cudaの既存fusedカーネルをそのまま
     再利用できる」ことの実証に留まり、dream-os自体が新しい推論
     ワークロード(例: World Laboratoryのワーカー層でのLLM推論タスク)を
     持つところまでは実装していない。(c) `directx_bridge`同様、
     Windows/Androidのみの検証で、AMD/Intel/Linux/macOS実機は未検証
     (実機が無いため)。
  - 次にすべきこと: (1) GPT-2実寸の`head_dim`/`seq_len`でのベンチマーク
    (現状は8×16のPoC規模のみ)、(2) World Laboratoryコーディネータ本体
    (RPoem/open-web-server上への実HTTP API)の実装は引き続き未着手、
    (3) `sbm_ising`が解ける組合せ最適化問題の具体候補の特定
    (open-directx/open-cuda側からの提案待ち、前回から変化なし)、
    (4) open-directxの完成度向上が優先方針のため、本格拡張は引き続き
    小さく育てる。

- **2026-08-07(続き3) open-directx/open-cuda/aruaru-llmとの関連性・連携性
  調査(ユーザー指示「4リポジトリの関連性・連携性・実用性・完成度を向上」、
  本リポジトリへのコード変更は無し、正直な開示)**: `open-cuda`側CLAUDE.md
  冒頭の保留タスク(東芝SBM〈`sbm_ising`〉/DeepSeek技術を8リポジトリへ
  組み込む構想)を踏まえ、`crates/dream-os-kernel/src/sbm.rs`
  (64スピンPoC)・`mining.rs`(open-cuda連携のsha256d_mineカーネル
  ディスパッチ)・`directx_bridge.rs`(open-directx連携)を再確認したが、
  この構想の前提である「`open-directx`/`open-cuda`側で`sbm_ising`を
  適用できる具体的な組合せ最適化問題を先に特定する」という調査自体は
  今回未着手(`open-directx`側の境界チェック付きチェーン生成の内部構造
  ・`open-cuda`側のGEMM/Attentionディスパッチスケジューリングのいずれも、
  安易に「SBMで最適化できそう」と決め打ちすると憶測に基づく実装になる
  リスクがあると判断したため、慎重に見送った)。`cargo build --workspace`
  で既存の健全性(警告0件)のみ再確認した。`open-cuda`側では今回
  `open-cuda-llm`のAttention経路への`flash_attention_with_spirv`配線
  (実機検証済み、詳細は`open-cuda/CLAUDE.md` 2026-08-07(続き5)HANDOFF
  参照)を実施しており、これは`sha256d_mine`カーネル(本リポジトリが
  依存する既存の連携経路)とは独立した別のディスパッチ経路のため、
  本リポジトリの`mining.rs`側の呼び出しには影響しない(念のため
  `cargo test -p dream-os-kernel`で既存テストの回帰も確認、変更なし)。
  - 次にすべきこと: (1) `open-directx`/`open-cuda`側それぞれで
    「`sbm_ising`が解ける組合せ最適化問題」の具体候補を1つ以上特定して
    から着手する(本リポジトリ単独では相手側の内部構造への理解が
    不足しており特定できない、両リポジトリ側からの提案を待つ形が
    現実的)、(2) Android実機連携(直前HANDOFF群)の延長として、
    `sbm_ising`は既に`opencuda_core::GpuDevice`経由のGPUディスパッチを
    持つ(`crates/dream-os-kernel/tests/sbm_real_vulkan.rs::
    gpu_sbm_ising_matches_cpu_reference_on_real_hardware`で実機検証
    済みと確認、本エントリを書く前に`cargo test -p dream-os-kernel`を
    実行して確認——当初「CPU実装のみ」と誤って書きかけたが、テスト
    実行結果で訂正した)ため、Android実機(Adreno 619)上でこの既存の
    GPU経路を使う設計自体は既に成立しているはずだが、Android実機上での
    `sbm_ising`のGPU経路の実行確認(`mining.rs`のsha256d_mineとは別に)
    は未確認のまま(次回確認対象)。

- **2026-08-07(続き2) DreamOS用Android連携アプリ(実APK、ProcessBuilder
  方式)を新規実装・実機画面で動作確認**: 直前のHANDOFFエントリ
  (`dream_os_status`のAndroid実機コマンドライン実行検証)を踏まえ、
  今回はその一歩先——`open-web-server`/`open-easy-web`の
  Android実装(JNIではなく`ProcessBuilder`でネイティブ実行ファイルを
  起動するサブプロセス方式)を参考にした**実際のAndroidアプリ
  (APK)**を新規実装し、実機のアプリ画面上での表示まで確認した。
  - 新規`F:\runo\dream-os\android`(Gradleプロジェクト、package
    `tokyo.runo.dreamos`)。`gradlew`/`gradle-wrapper.jar`等は
    `open-web-server/android`から複製(同じGradle 8.11.1系列)。
  - `cargo ndk -t aarch64-linux-android build -p dream-os-kernel
    --example dream_os_status --release`でクロスビルドした実行
    ファイルを`android/app/src/main/jniLibs/arm64-v8a/
    libdreamosstatus.so`として同梱(open-web-server方式と同じ
    `useLegacyPackaging=true`でnativeLibraryDir配下への実展開を強制)。
  - `MainActivity.kt`(新規)——ボタン押下で`ProcessBuilder`により
    このバイナリを起動し、標準出力をそのままTextViewへ表示する
    最小実装。DreamOS固有ロジックはActivity側に一切持たせず、
    表示ロジックのみを担う。
  - **実機検証(Moto G53Y 5G、Adreno 619、シリアル`ZY22J7RFND`)**:
    (a) `gradle :app:assembleDebug`(`JAVA_HOME`はAndroid Studio同梱
    jbrを使用)で**BUILD SUCCESSFUL**(`app-debug.apk`、約3.8MB)。
    (b) `adb install -r`→`adb shell am start`で実機へインストール・
    起動。(c) `adb shell input tap`でボタンを実タップし、
    `adb shell screencap`で実機画面を実際にキャプチャ——**アプリ画面上に
    `binary: /data/app/.../lib/arm64/libdreamosstatus.so`・
    `exit code: 0`・`vulkan device: OpenCUDA Vulkan Device
    (Adreno (TM) 619)`・`open-directx DXBC->SPIR-V dispatch: 256
    elements, 0 mismatches`・`result: OK`が表示されることを
    スクリーンショットで確認**——型チェック・ビルド成功・コマンドライン
    実行の確認だけでなく、実際のAndroidアプリUI上での動作を実証できた
    (直前エントリまでの到達点から一歩前進)。
  - **正直な開示・未着手事項**: (a) サブプロセス方式のまま(JNI/UniFFI
    等によるネイティブライブラリ直接リンクではない)——将来「DreamOS
    共通実行基盤」としてより深く統合するなら発展の余地がある。
    (b) `dream_os_status`example1本のみを同梱、mining/sbm等の他
    カーネルはこのアプリからは呼べない。(c) x86_64エミュレータ向け
    jniLibsは同梱せず実機arm64-v8aのみ対象。(d) `.gitignore`へ
    `android/.gradle/`・`android/app/build/`・`android/local.properties`
    等を追加し、ビルド成果物・SDKローカルパスはコミット対象から除外。
    git add/git commitは今回実行していない(ユーザー確認後の判断待ち)。
  - 次にすべきこと: (1) JNI/UniFFI等でのネイティブ直接リンク方式への
    発展検討、(2) mining/sbm等の他カーネルを選択実行できるUIへの拡張、
    (3) x86_64エミュレータ向けjniLibs追加、(4) World Laboratory
    コーディネータ本体の実装(引き続き未着手)、(5) open-directxの
    完成度向上が優先方針のため、本格拡張は引き続き小さく育てる。

- **2026-08-07(続き) 共通実行基盤の再検証+dream_os_statusのAndroid実機
  実行を新規確認**: ユーザー指示「open-directx/open-cudaを土台にした共通
  実行基盤の検証、Android連携PoCの実装・実機/ビルド検証を完了させる」への
  対応。コードの新規追加はコミット済みの直前HANDOFF時点で完了していたため、
  今回は(1)退行が無いことの再確認と、(2)これまで未実施だった
  `examples/dream_os_status.rs`(状態確認用の最小サンプル、OS/アーキ
  テクチャ・Vulkanデバイス名・open-directxブリッジの動作可否を1コマンドで
  表示する)のAndroid実機実行、の2点を実施。
  **再検証結果**: `cargo build --workspace --release`・`cargo test
  --workspace --release`をWindows実機(GT730)で実行し全件green(退行無し、
  power_profile単体テスト5件・GPU実機テスト5件〈mining/sbm/directx_bridge/
  aruaru_db_integration/secure_channel_integration〉すべて成功)。続けて
  `cargo ndk -t aarch64-linux-android build --release`(ライブラリ)・
  `--examples`(実行バイナリ)の両方をクロスビルドし、`dream-os-kernel`・
  `dream-os-wire`の`.rlib`および`dream_os_status`・`sbm_benchmark`・
  `directx_bridge_benchmark`・`mine_benchmark`の全実行バイナリが生成される
  ことを確認。**Android実機(Moto G53Y 5G、Adreno 619、シリアル
  `ZY22J7RFND`)へ`adb push`(PowerShell経由、既存の申し送り通り)で
  配置し実行**:
  - `dream_os_status`(今回初のAndroid実機実行): `vulkan device: OpenCUDA
    Vulkan Device (Adreno (TM) 619)`・`DXBC->SPIR-V dispatch: 256
    elements, 0 mismatches`・`result: OK`を実機で確認。
  - `sbm_benchmark`・`mine_benchmark`・`directx_bridge_benchmark`は
    shaders(`.spv`)を`/data/local/tmp/shaders/`へ同時pushしないと
    相対パス解決に失敗する点(`shaders/sbm_ising.spv: No such file or
    directory`)を今回発見・対処(shaders同梱pushで解消、コード変更は
    不要——実行手順上の注意点としてここに記録)。対処後、sbmは
    Windows実機と同じエネルギー値`-38.0885`でGPU/CPU完全一致、mining
    は3バッチとも`device lost`等の異常無く約0.10 MH/sで完走、
    directx_bridgeは256要素すべてCPU参照実装と一致。
  - コード変更・コミットは無し(検証のみ、ユーザー指示によりコミット
    未作成)。
  - 次にすべきこと: (1)
    今回判明した「Android実機実行時はshaders/ディレクトリを実行バイナリと
    同じ相対位置へ`adb push`する必要がある」という手順を、将来デプロイ
    自動化スクリプト(あれば)やREADME/PORTING.mdの実行手順節に明記する、
    (2) World Laboratoryコーディネータ本体(RPoem/open-web-server上への
    実HTTP API)の実装、(3) aruaru-llmとの連携(LLM推論タスクの
    ワークロード種別追加)、(4) 複数GPU対応は引き続き実機が無く未着手、
    (5) open-directxの完成度向上が優先方針のため、引き続き小さく育てる。

- **2026-08-07 sbm_ising/directx_bridgeのAndroid実機検証**: ユーザー指示
  「dream-os・open-directx・open-cuda・aruaru-llmの連携性強化・実用性
  向上・利便性向上・完成度向上」への対応として、上記「次にすべきこと」
  最優先項目だった「Android実機でのsbm_ising/directx_bridgeの実機検証」
  (これまでmining.rsのみAndroid実機検証済みで、sbm.rs/directx_bridge.rs
  はWindows実機のみだった)を実施。
  `crates/dream-os-kernel/examples/sbm_benchmark.rs`・
  `examples/directx_bridge_benchmark.rs`(新規)を、既存の
  `mine_benchmark.rs`と同じパターン(`cargo test`がAndroidクロス
  ビルド環境では動かせないため、`adb push`して直接実行できる単体
  バイナリとして提供)で追加。テスト本体(`tests/sbm_real_vulkan.rs`・
  `tests/directx_bridge_real_vulkan.rs`)と同じ検証ロジックを移植した。
  **実機検証結果**: Windows実機(NVIDIA GT730)で両方とも成功を確認した
  上で、`cargo ndk -t aarch64-linux-android build --release`でクロス
  ビルドし、PowerShell経由の`adb push`(Git BashからのUnixパス自動変換
  問題を過去に踏んだため、既存の申し送り通りPowerShellから`adb`を
  呼んだ)でAndroid実機(Moto G53Y 5G、Adreno 619)へ配置・実行。
  - `sbm_benchmark`: GPU/CPU参照実装が64/64スピン完全一致
    (Windows実機と同じエネルギー値`-38.0885`)。
  - `directx_bridge_benchmark`: open-directx製DXBC→SPIR-V翻訳が
    Android実機のVulkan実行基盤上でも256要素すべてCPU参照実装と一致。
  mining.rsで発見した`device lost`のような不安定挙動は今回発生せず
  (問題規模が小さいディスパッチ1回のみのため)。`cargo build
  --workspace --release`/`cargo test --workspace --release`で全クレート
  regression無し(GT730実機込み)。
  - 次にすべきこと: (1) World Laboratoryコーディネータ本体
    (RPoem/open-web-server上への実HTTP API)の実装、(2) aruaru-llmとの
    連携(World Laboratoryのワークロード種別としてLLM推論タスクを追加)、
    (3) 複数GPU対応は引き続きこのマシンに実機が無いため未着手のまま、
    (4) open-directxの完成度向上が優先方針のため、引き続き小さく育てる。

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
