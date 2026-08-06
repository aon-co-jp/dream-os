//! 東芝SBM(Simulated Bifurcation Machine)にインスパイアされた組合せ
//! 最適化カーネルの実Vulkanデバイス検証(2026-08-06)。
//!
//! GPU版の最終スピン配置・Isingエネルギーが、CPU参照実装(全く同じ
//! ballistic SB更新式を逐次計算)と一致することを検証する。

use std::path::PathBuf;

use dream_os_kernel::open_device;
use dream_os_kernel::sbm::{run_sbm_ising, run_sbm_ising_cpu_reference, NUM_SPINS};

/// 決定的な擬似乱数(線形合同法)で、対称なJ行列と初期x値を生成する
/// (外部乱数crateへの依存を避けた最小実装、テスト専用)。
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

#[test]
fn gpu_sbm_ising_matches_cpu_reference_on_real_hardware() {
    let spv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders").join("sbm_ising.spv");
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

    let (j_matrix, init_x) = deterministic_problem(1234567);
    let steps = 100u32;
    let dt = 0.5f32;
    let c0 = 0.05f32;
    let a0 = 1.0f32;

    let gpu = run_sbm_ising(&device, &spirv, &j_matrix, &init_x, steps, dt, c0, a0).expect("gpu SBM run failed");
    let cpu = run_sbm_ising_cpu_reference(&j_matrix, &init_x, steps, dt, c0, a0);

    assert_eq!(gpu.spins.len(), cpu.spins.len());
    let mismatches = gpu.spins.iter().zip(cpu.spins.iter()).filter(|(g, c)| g != c).count();
    assert_eq!(mismatches, 0, "GPU/CPU spin assignments disagree at {mismatches}/{NUM_SPINS} spins");

    // エネルギーは同一spins配列から計算するため厳密一致するはずだが、
    // f32/f64混在の丸め誤差を許容するため小さな許容誤差を設ける。
    assert!((gpu.energy - cpu.energy).abs() < 1e-3, "energy mismatch: gpu={} cpu={}", gpu.energy, cpu.energy);

    println!("GPU and CPU converged to the identical spin configuration ({NUM_SPINS} spins)");
    println!("Ising energy: {:.4} (lower = better for this minimization formulation)", gpu.energy);
}
