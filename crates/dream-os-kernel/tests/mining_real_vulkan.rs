//! 実Vulkanデバイス上でのマイニングカーネル検証(2026-08-06)。
//!
//! `sha256d_mine.spv`(事前コンパイル必須、`glslc shaders/sha256d_mine.comp
//! -o shaders/sha256d_mine.spv`)を実デバイスへディスパッチし、
//! RustCrypto製`sha2`クレートでのCPU参照実装(double-SHA256)と
//! バイト単位で完全一致することを検証する。

use std::path::PathBuf;

use dream_os_kernel::mining::MiningWorker;
use dream_os_kernel::open_device;
use sha2::{Digest, Sha256};

#[test]
fn gpu_sha256d_matches_cpu_reference_on_real_hardware() {
    let spv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders").join("sha256d_mine.spv");
    let spirv = match std::fs::read(&spv_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("skipping: {} not found, compile it first with glslc", spv_path.display());
            return;
        }
    };

    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("skipping: no real Vulkan device available: {e}");
            return;
        }
    };

    let worker = MiningWorker::new(device, spirv);
    let base_message: [u32; 8] = [0, 0xdeadbeef, 0x12345678, 0, 0, 0, 0, 0];
    let nonce_base = 1000u32;
    let count = 64u32;

    let result = worker.mine_batch(base_message, nonce_base, count).expect("gpu mine_batch failed");
    assert_eq!(result.hashes, count as u64);
    assert_eq!(result.digests.len(), count as usize);

    for (i, gpu_digest) in result.digests.iter().enumerate() {
        let nonce = nonce_base + i as u32;
        // シェーダは m0..m7 を「そのままSHA-256のビッグエンディアン
        // メッセージワード」として使うため、CPU参照側もビッグエンディアン
        // でバイト列を組み立てる(GPUバッファへの転送自体はリトル
        // エンディアンのメモリ表現だが、値そのものはネイティブu32として
        // 一致するため、ここでの変換対象はSHA-256の入力バイト順の方)。
        let mut msg = [0u8; 32];
        msg[0..4].copy_from_slice(&nonce.to_be_bytes());
        for (w, word) in base_message.iter().enumerate().skip(1) {
            msg[w * 4..w * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }

        let first = Sha256::digest(msg);
        let cpu_digest = Sha256::digest(first);

        assert_eq!(
            gpu_digest.as_slice(),
            cpu_digest.as_slice(),
            "mismatch at nonce {nonce}: gpu={gpu_digest:02x?} cpu={cpu_digest:02x?}"
        );
    }

    println!("device: {}", worker.device_name());
    println!("all {count} GPU digests match CPU sha2 reference (double-SHA256)");
}
