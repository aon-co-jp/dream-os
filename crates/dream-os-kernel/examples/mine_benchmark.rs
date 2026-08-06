//! DreamOS PoC: マイニング相当の実ハッシュ計算カーネルのハッシュレート計測。
//!
//! `cargo run --example mine_benchmark --release -- [power_percent] [batches]`

use std::path::PathBuf;

use anyhow::{Context, Result};
use dream_os_kernel::mining::{hashrate, MiningWorker, DEFAULT_BATCH_SIZE};
use dream_os_kernel::{open_device, MiningPowerProfile};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let power_percent: u8 = args.next().and_then(|s| s.parse().ok()).unwrap_or(100);
    let batches: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(3);

    let spirv = std::fs::read(shader_path()).context(
        "failed to read shaders/sha256d_mine.spv. Compile it first: glslc shaders/sha256d_mine.comp -o shaders/sha256d_mine.spv",
    )?;
    let device = open_device(0)?;
    let worker = MiningWorker::new(device, spirv);
    println!("device: {}", worker.device_name());
    println!("mining power profile: {power_percent}% ({batches} batches x {DEFAULT_BATCH_SIZE} hashes)");

    let profile = MiningPowerProfile::new(power_percent);
    let base_message = [0u32, 0xdeadbeef, 0x12345678, 0, 0, 0, 0, 0];
    let mut nonce_base = 0u32;

    for batch in 0..batches {
        let result = worker.mine_batch(base_message, nonce_base, DEFAULT_BATCH_SIZE)?;
        nonce_base = nonce_base.wrapping_add(DEFAULT_BATCH_SIZE);
        let rate = hashrate(result.hashes, result.elapsed);
        println!("  batch {batch}: {} hashes in {:?} ({:.2} MH/s)", result.hashes, result.elapsed, rate / 1e6);

        let sleep = profile.sleep_after_dispatch(result.elapsed);
        if sleep == std::time::Duration::MAX {
            println!("  power_percent=0: stopping");
            break;
        }
        if !sleep.is_zero() {
            std::thread::sleep(sleep);
        }
    }

    Ok(())
}

fn shader_path() -> PathBuf {
    // 開発機ではビルド時の`CARGO_MANIFEST_DIR`を使うが、Android実機へ
    // バイナリ単体を`adb push`して実行する場合はこのパスが存在しない
    // (ビルド機のWindowsパスがバイナリに埋め込まれるだけのため)。
    // その場合はカレントディレクトリ相対の`shaders/...`へフォールバック
    // する(`adb shell "cd /data/local/tmp && ./mine_benchmark"`のように
    // シェーダを同じ相対レイアウトで配置して実行する想定)。
    let build_time_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders").join("sha256d_mine.spv");
    if build_time_path.exists() {
        return build_time_path;
    }
    PathBuf::from("shaders").join("sha256d_mine.spv")
}
