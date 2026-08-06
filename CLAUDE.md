# 設計思想＆開発方針＆開発環境ルール(DreamOS / dream-os)

作業ドライブは`F:\runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。
GitHubリポジトリ: [aon-co-jp/dream-os](https://github.com/aon-co-jp/dream-os)。

**構想開始日: 2026-08-06**(このリポジトリへの構想文書着手日。GitHub上の空リポジトリ
自体は2026-07-01に作成済みだった、`open-directx`と同じパターン)。

## このプロジェクトの役割(構想段階、コード未着手)

**DreamOS**(リポジトリ名・VPS/ローカルフォルダ名は`dream-os`)は、
Windows・Mac・Android・各種Linuxディストリビューションに対応する
アプリケーションサービス層/GPU抽象化層で、将来的には**SONY
PlayStation 5/6・Nintendo Switch 2**への対応も視野に入れる
(ユーザー指示、2026-08-06)。

**現時点ではコードは一切書かれておらず、構想フェーズのみ**。この
`CLAUDE.md`・[`README.md`](README.md)・[`PORTING.md`](PORTING.md)で
目的・スコープ・エコシステム内での位置づけを文書化することが今回の
唯一の成果物。

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

## 対応プラットフォーム(ロードマップ、正直な優先順位)

**フェーズ0(現在)**: 構想・文書化のみ。

**フェーズ1以降(未着手、優先順位はユーザー指示の記載順)**:
1. Windows
2. macOS
3. Android
4. 各種Linuxディストリビューション
5. (将来的に)SONY PlayStation 5 / PlayStation 6
6. (将来的に)Nintendo Switch 2

**正直な開示**: PS5/6・Switch2は各社の非公開SDK・NDA・実機/開発機材が
必須であり、現時点でこのエコシステムに実機・SDK・開発者ライセンスは
一切無い。「将来的に対応したい」という方向性の明記であり、具体的な
実装着手時期・実現可能性は未確定。

## VPS/ローカルとの対応(2026-08-06新設)

- ローカル: `F:\runo\dream-os`(このclone)。
- VPS(conoha): `/root/dream-os`(新設予定、[`PORTING.md`](PORTING.md)参照)。

## HANDOFF(直近の作業ログ、上が最新)

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
