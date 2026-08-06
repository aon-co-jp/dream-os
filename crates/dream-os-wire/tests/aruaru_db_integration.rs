//! World Laboratory向け通信層(dream-os-wire)からaruaru-dbへの実接続検証
//! (2026-08-06)。
//!
//! `aruaru-server`(pgwire)が`DREAM_OS_ARUARU_PG_CONN`環境変数で指定された
//! 接続文字列で到達可能な場合のみ実行する(例:
//! `host=127.0.0.1 port=15432 user=dreamos dbname=aruaru`)。未設定/
//! 到達不能な環境では正直にスキップする(既存のこのエコシステムの
//! `#[ignore]`付き実DB統合テストと同じ設計方針)。

use dream_os_wire::{AruaruDbStore, WorkResultEnvelope};

#[tokio::test]
async fn worker_result_persists_into_aruaru_db_with_a_commit_id() {
    let conn_str = match std::env::var("DREAM_OS_ARUARU_PG_CONN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("skipping: DREAM_OS_ARUARU_PG_CONN not set (no aruaru-server instance configured)");
            return;
        }
    };

    let store = match AruaruDbStore::connect(&conn_str).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("skipping: could not connect to aruaru-server: {e:?}");
            return;
        }
    };

    store.ensure_schema().await.expect("ensure_schema should succeed against a real aruaru-server");

    let envelope = WorkResultEnvelope {
        work_unit_id: "wu-dreamos-aruaru-0001".to_string(),
        worker_id: "worker-gt730".to_string(),
        result_json: serde_json::json!({ "nonce_base": 1000, "count": 64 }),
    };

    let commit_id = store.persist_result(&envelope).await.expect("persist_result should succeed against a real aruaru-server");

    assert!(!commit_id.is_empty(), "aruaru_commit should return a non-empty commit id");
    println!("OK: persisted WorkResultEnvelope into aruaru-db, commit_id={commit_id}");
}
