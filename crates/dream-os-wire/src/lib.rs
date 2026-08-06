//! dream-os-wire: World Laboratory構想(world-lab/docs/world-laboratory-design.md(https://github.com/aon-co-jp/world-lab))の
//! ワーカー↔コーディネータ間通信・永続化層(2026-08-06新設)。
//!
//! ユーザー指示「open-web-server・RPoem・open-raid-z・aruaru-dbのACID
//! 互換・ZFS互換の4層4重通信の技術もdream-osに実装、移植して」への対応。
//!
//! ## 実際に移植・再利用したもの(正直な開示)
//!
//! `open-web-server-wire`(`../../../open-web-server/crates/
//! open-web-server-wire`、path依存で再利用)の**第4層防御通信
//! (`SecureChannel`)を、World Laboratoryのワーカー→コーディネータ間の
//! 計算結果送信に実際に再利用した**——ゼロから再実装するのではなく、
//! 実績のある既存クレートをそのままpath依存する設計(`dream-os-kernel`が
//! `opencuda-vulkan`を再利用したのと同じ方針)。
//!
//! - **第1層(TLS)・第2層(相互認証)**: `open-web-server-wire`の
//!   `tls`モジュール・証明書ベースの相互認証機構は、World Laboratoryの
//!   ワーカー↔コーディネータ間TCP接続にもそのまま適用できる設計だが、
//!   本クレートでは**アプリケーション層のメッセージ暗号化(第3層+第4層)
//!   のみを実際に配線した**(TLS終端自体は呼び出し側=将来の
//!   コーディネータHTTPサーバー実装〈RPoem/open-web-server流用予定〉が
//!   担う想定、今回はそこまで実装せず設計文書に留める、下記参照)。
//! - **第3層(AEAD暗号化)+第4層(リプレイ対策)**: `SecureChannel`
//!   (ChaCha20-Poly1305 AEAD、seq/timestampをAADに紐付けたリプレイ防止)
//!   を`WorkResultEnvelope`(計算結果の送信用メッセージ)へ直接適用した。
//!   BOINCが本来必要とする「不正な結果の改ざん検知」「同一結果の
//!   リプレイによる多数決の不正操作を防ぐ」という要件と、この第4層の
//!   設計目的(改ざん検知+リプレイ対策)が完全に合致するため、
//!   world-lab/docs/world-laboratory-design.md§3.3の結果検証の一部として位置づける。
//!
//! ## 実装しなかったもの(正直な開示、設計のみ)
//!
//! - **4重伝送路(TCP/UDP/QUIC/MPTCP)**: `open-web-server-wire`は
//!   `quic_channel`/`udp_channel`/`mptcp_channel`を持つが、今回は
//!   `SecureChannel`(ペイロード層)のみを配線した。World Laboratoryの
//!   ワーカーは家庭のPC/スマホ(不安定な回線が前提)であり、複数伝送路の
//!   同時活用は将来的に有効な可能性があるが、今回はまず「1本の安全な
//!   経路」を確立することを優先した。
//! - **aruaru-db永続化(2026-08-06実装・実機検証済み)**: `aruaru_
//!   persistence`モジュールが`aruaru-server`(pgwire)へ`tokio-postgres`
//!   で接続し、`WorkResultEnvelope`をテーブルへINSERT+`aruaru_commit`で
//!   コミットする。ACID互換のトランザクション性+Git-on-SQLのバージョン
//!   管理(commit_id)の両方を実際に活用した永続化。詳細は
//!   `aruaru_persistence`モジュールdoc参照。
//! - **4重DB永続化のうちPostgreSQL/マルチリージョン同期/独立監査ログ**:
//!   `open-web-server-ledger`(`PostgresWal`・`MultiRegionReplicator`・
//!   `audit_log`)は引き続き未配線(aruaru-db以外の3系統は今回のスコープ
//!   外)。World Laboratoryが実際に大量の結果を永続化する段階
//!   (フェーズ2以降)で、`open-web-server-
//!   ledger`をコーディネータ側の永続化層としてそのままpath依存で
//!   再利用する設計方針だけをworld-lab/docs/world-laboratory-design.mdへ追記した
//!   (コードの実配線は次回以降)。

pub mod aruaru_persistence;

pub use aruaru_persistence::AruaruDbStore;

use anyhow::{Context, Result};
use open_web_server_wire::replay_guard::SecureChannel;
use serde::{Deserialize, Serialize};

/// ワーカーがコーディネータへ送る計算結果の envelope。
///
/// world-lab/docs/world-laboratory-design.md§3.2の「ワークユニット」に対応する結果
/// メッセージ。`dream-os-kernel::mining::MiningBatchResult`のような
/// 実際の計算結果を、ワーカーIDと共にコーディネータへ安全に送るための
/// 契約。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkResultEnvelope {
    pub work_unit_id: String,
    pub worker_id: String,
    /// 計算結果本体(用途に依存する自由形式、例: マイニングなら
    /// ダイジェストのhex文字列一覧、LLM推論なら出力トークン列等)。
    pub result_json: serde_json::Value,
}

/// ワーカー側の送信チャネル。`SecureChannel::encrypt`を`WorkResultEnvelope`
/// のJSONシリアライズと組み合わせる薄いラッパー。
pub struct WorkerChannel {
    inner: SecureChannel,
}

impl WorkerChannel {
    pub fn new(shared_key: &[u8; 32]) -> Self {
        Self { inner: SecureChannel::new(shared_key) }
    }

    /// 結果をJSONシリアライズし、`SecureChannel`で暗号化+リプレイ対策
    /// フレームへ変換する。
    pub fn submit(&mut self, envelope: &WorkResultEnvelope) -> Result<Vec<u8>> {
        let json = serde_json::to_vec(envelope).context("failed to serialize WorkResultEnvelope")?;
        self.inner.encrypt(&json).context("failed to encrypt work result")
    }
}

/// コーディネータ側の受信チャネル。改ざん検知・リプレイ拒否を経た上で
/// `WorkResultEnvelope`へ復元する。
pub struct CoordinatorChannel {
    inner: SecureChannel,
}

impl CoordinatorChannel {
    pub fn new(shared_key: &[u8; 32]) -> Self {
        Self { inner: SecureChannel::new(shared_key) }
    }

    pub fn receive(&mut self, frame: &[u8]) -> Result<WorkResultEnvelope> {
        let json = self.inner.decrypt(frame).context("failed to decrypt/verify work result frame")?;
        serde_json::from_slice(&json).context("failed to deserialize WorkResultEnvelope")
    }
}
