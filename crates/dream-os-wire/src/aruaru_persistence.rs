//! aruaru-db永続化層(2026-08-06新設)。
//!
//! ユーザー指示「dream-osをopen-directx・open-cuda・open-web-server・
//! RPoem・open-raid-z・aruaru-dbなどをベースに関連技術を実装開発、
//! 完成度と連携性の向上をして」への対応。
//!
//! World Laboratory構想(world-lab/docs/world-laboratory-design.md(https://github.com/aon-co-jp/world-lab))の通信・
//! 永続化層(第4層)のうち、これまで「実DBインスタンスが無いため未配線」
//! と記録していた永続化部分を、`aruaru-db`(ACID互換+Git-on-SQLの
//! バージョン管理、`aruaru-server`が話すPostgreSQL wireプロトコル)へ
//! 実際に接続する形で実装する。`tokio-postgres`クライアントで
//! `aruaru-server`(pgwire、既定で認証無し・開発用途)へ接続し、
//! `WorkResultEnvelope`をテーブルへINSERTした上で`aruaru_commit(...)`を
//! 呼び、コミットIDを持つバージョン管理された結果として永続化する。

use anyhow::{Context, Result};
use base64::Engine;
use tokio_postgres::{Client, NoTls};

use crate::WorkResultEnvelope;

/// `aruaru-server`(pgwire)への接続。
pub struct AruaruDbStore {
    client: Client,
}

impl AruaruDbStore {
    /// `conn_str`例: `"host=127.0.0.1 port=15432 user=dreamos dbname=aruaru"`
    /// (`aruaru-server`は既定で認証を要求しない開発用構成のため、
    /// `user`/`dbname`は任意の値で構わない)。
    pub async fn connect(conn_str: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await.context("failed to connect to aruaru-server (pgwire)")?;
        // pgwireクライアントの慣例: 接続処理を別タスクで駆動し続ける必要がある。
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("aruaru-db pgwire connection error: {e}");
            }
        });
        Ok(Self { client })
    }

    /// `world_lab_results`テーブルを用意する(既に存在すれば無視、
    /// `CREATE TABLE IF NOT EXISTS`)。
    pub async fn ensure_schema(&self) -> Result<()> {
        self.client
            .simple_query("CREATE TABLE IF NOT EXISTS world_lab_results (work_unit_id TEXT, worker_id TEXT, result_json TEXT)")
            .await
            .context("failed to create world_lab_results table")?;
        Ok(())
    }

    /// `WorkResultEnvelope`を1行INSERTし、`aruaru_commit`でコミットして
    /// commit_idを返す(ACID互換+Git版管理の両方を実際に活用する経路)。
    ///
    /// **実機検証で発見した実際の制約(正直な開示)**: `aruaru-query`の
    /// SQLパーサー(`parser.rs::parse_insert`)は`VALUES(...)`内の値を
    /// クォートを考慮しない単純な`split(',')`で分割する簡易実装
    /// (フルSQL構文パーサーではなく、意図的に絞り込まれたサブセット)。
    /// そのため`result_json`(JSON文字列)にカンマが含まれると
    /// `"INSERT: N columns but M values"`エラーになることを実際の
    /// `aruaru-server`インスタンスへの接続で発見した。回避策として
    /// `result_json`をBase64エンコードしてから格納する(カンマ・
    /// クォート等パーサーが誤分割しうる文字を含まない安全な文字集合の
    /// みになる)。
    pub async fn persist_result(&self, envelope: &WorkResultEnvelope) -> Result<String> {
        let result_json = serde_json::to_string(&envelope.result_json).context("failed to serialize result_json")?;
        let result_json_b64 = base64::engine::general_purpose::STANDARD.encode(result_json.as_bytes());

        let insert_sql = format!(
            "INSERT INTO world_lab_results (work_unit_id, worker_id, result_json) VALUES ('{}', '{}', '{}')",
            escape_sql_literal(&envelope.work_unit_id),
            escape_sql_literal(&envelope.worker_id),
            result_json_b64,
        );
        self.client.simple_query(&insert_sql).await.context("failed to insert world lab result")?;

        let commit_msg = format!("world laboratory result: {}", envelope.work_unit_id);
        let commit_sql = format!("SELECT aruaru_commit('{}')", escape_sql_literal(&commit_msg));
        let rows = self.client.simple_query(&commit_sql).await.context("failed to run aruaru_commit")?;

        for msg in rows {
            if let tokio_postgres::SimpleQueryMessage::Row(row) = msg {
                if let Some(commit_id) = row.get(0) {
                    return Ok(commit_id.to_string());
                }
            }
        }
        anyhow::bail!("aruaru_commit did not return a commit_id")
    }
}

/// 単純な文字列エスケープ(単一引用符の重複、`aruaru-wire`が持つ
/// SQLインジェクション対策と同じ発想の最小限の実装——本来はプリペアド
/// ステートメントを使うべきだが、`aruaru-wire`の拡張プロトコル
/// 対応状況〈`describe_portal`が動的スキーマのため空列を返す既知の
/// 制約、`aruaru-db/CLAUDE.md`参照〉を踏まえ、既存のこのエコシステムの
/// 他クライアントコードと同じ`simple_query`+手動エスケープ方式に揃えた)。
fn escape_sql_literal(s: &str) -> String {
    s.replace('\'', "''")
}
