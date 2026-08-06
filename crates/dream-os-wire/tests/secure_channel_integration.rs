//! World Laboratory向け通信層(open-web-server-wireのSecureChannel再利用)の
//! 実際の往復検証(2026-08-06)。

use dream_os_wire::{CoordinatorChannel, WorkResultEnvelope, WorkerChannel};

#[test]
fn worker_submits_result_and_coordinator_verifies_it() {
    let shared_key = [42u8; 32];
    let mut worker = WorkerChannel::new(&shared_key);
    let mut coordinator = CoordinatorChannel::new(&shared_key);

    let envelope = WorkResultEnvelope {
        work_unit_id: "wu-0001".to_string(),
        worker_id: "worker-gt730".to_string(),
        result_json: serde_json::json!({ "nonce_base": 1000, "count": 64, "digest_sample": "abcd1234" }),
    };

    let frame = worker.submit(&envelope).expect("worker submit should succeed");
    let received = coordinator.receive(&frame).expect("coordinator should decrypt and verify");

    assert_eq!(received, envelope);
}

#[test]
fn replayed_result_frame_is_rejected() {
    let shared_key = [7u8; 32];
    let mut worker = WorkerChannel::new(&shared_key);
    let mut coordinator = CoordinatorChannel::new(&shared_key);

    let envelope = WorkResultEnvelope {
        work_unit_id: "wu-0002".to_string(),
        worker_id: "worker-android-1".to_string(),
        result_json: serde_json::json!({ "hashes": 16384 }),
    };

    let frame = worker.submit(&envelope).unwrap();
    coordinator.receive(&frame).expect("first submission should succeed");

    // 悪意あるノードが同じ結果フレームをそのまま再送し、多数決を
    // 水増ししようとするシナリオ(BOINCが本来対処すべき攻撃と同種)。
    let err = coordinator.receive(&frame).unwrap_err();
    assert!(err.to_string().contains("replayed") || err.to_string().contains("decrypt"), "unexpected error: {err}");
}

#[test]
fn tampered_result_is_rejected_without_decrypting_garbage() {
    let shared_key = [3u8; 32];
    let mut worker = WorkerChannel::new(&shared_key);
    let mut coordinator = CoordinatorChannel::new(&shared_key);

    let envelope = WorkResultEnvelope {
        work_unit_id: "wu-0003".to_string(),
        worker_id: "worker-3".to_string(),
        result_json: serde_json::json!({ "fake": "result" }),
    };

    let mut frame = worker.submit(&envelope).unwrap();
    // フレームの末尾(暗号文部分)を改ざん -> ノードが不正な計算結果を
    // でっち上げて送りつけようとするシナリオを模擬。
    let last = frame.len() - 1;
    frame[last] ^= 0xFF;

    let err = coordinator.receive(&frame).unwrap_err();
    assert!(err.to_string().contains("decrypt"), "expected a decrypt/tamper error, got: {err}");
}
