//! DreamOS PoC: 東芝SBM風Ising最適化カーネルのAndroid/Windows実機検証用
//! ベンチマーク(2026-08-07)。
//!
//! `tests/sbm_real_vulkan.rs`と同じGPU/CPU一致検証を、`cargo test`が
//! 動かないAndroidクロスビルド環境でも`adb push`して直接実行できる
//! バイナリとして提供する(`examples/mine_benchmark.rs`と同じパターン)。
//!
//! `cargo run --example sbm_benchmark --release`

use std::path::PathBuf;

use dream_os_kernel::open_device;
use dream_os_kernel::sbm::{run_sbm_ising, run_sbm_ising_cpu_reference, NUM_SPINS};

fn deterministic_problem(seed: u64) -> (Vec<f32>, Vec<f32>) {
    let mut state = seed;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((state >> 33) as u32) as f32 / u32::MAX as f32
    };

    let n = NUM_SPINS;
    let mut j = vec![0f32; n * n];
    for i in 0..n {
        for k in (i + 1)..n {
            let v = (next() - 0.5) * 2.0;
            j[i * n + k] = v;
            j[k * n + i] = v;
        }
    }
    let init_x: Vec<f32> = (0..n).map(|_| (next() - 0.5) * 0.2).collect();
    (j, init_x)
}

fn main() {
    let spirv = match std::fs::read(shader_path()) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("failed to read shaders/sbm_ising.spv: {e}");
            std::process::exit(1);
        }
    };
    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("no real Vulkan device available: {e}");
            std::process::exit(1);
        }
    };
    println!("device: {}", device.info().name);

    let (j_matrix, init_x) = deterministic_problem(1234567);
    let steps = 100u32;
    let dt = 0.5f32;
    let c0 = 0.05f32;
    let a0 = 1.0f32;

    let gpu = run_sbm_ising(&device, &spirv, &j_matrix, &init_x, steps, dt, c0, a0).expect("gpu SBM run failed");
    let cpu = run_sbm_ising_cpu_reference(&j_matrix, &init_x, steps, dt, c0, a0);

    let mismatches = gpu.spins.iter().zip(cpu.spins.iter()).filter(|(g, c)| g != c).count();
    println!("spin mismatches: {mismatches}/{NUM_SPINS}");
    println!("gpu energy: {:.4}  cpu energy: {:.4}", gpu.energy, cpu.energy);

    if mismatches == 0 && (gpu.energy - cpu.energy).abs() < 1e-3 {
        println!("OK: GPU and CPU converged to the identical spin configuration");
    } else {
        eprintln!("MISMATCH: GPU/CPU results disagree");
        std::process::exit(1);
    }
}

fn shader_path() -> PathBuf {
    // Windows開発機ではビルド時の`CARGO_MANIFEST_DIR`を使うが、
    // `adb push`でAndroid実機へバイナリ単体を配置して実行する場合は
    // そのパスが存在しない(ビルド機のパスが埋め込まれるだけ)ため
    // カレントディレクトリ相対の`shaders/...`へフォールバックする。
    let embedded = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders").join("sbm_ising.spv");
    if embedded.exists() {
        embedded
    } else {
        PathBuf::from("shaders/sbm_ising.spv")
    }
}
