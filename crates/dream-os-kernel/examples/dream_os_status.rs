//! DreamOS Android連携PoC(2026-08-07)用の自己完結バイナリ。
//!
//! `open-web-server`/`open-easy-web`のAndroid実装が採用している
//! 「Rustバイナリをクロスコンパイルし、Android側は`ProcessBuilder`で
//! それを直接起動して標準出力を読む」という設計パターン(JNIではなく
//! サブプロセス方式、`open-web-server/android/app/build.gradle.kts`の
//! `packaging.jniLibs.useLegacyPackaging = true`コメント参照)を
//! そのままDreamOSにも適用するための最小バイナリ。
//!
//! シェーダは`include_bytes!`でビルド時に実行ファイル内へ埋め込まれる
//! ため(`directx_bridge_benchmark.rs`と同じ手法)、実機側に別途
//! `shaders/*.spv`を配置する必要が無く、単一の実行ファイルを
//! `adb push`するだけでAndroidアプリから起動できる——Android連携PoCの
//! ための実行のしやすさを優先した設計。
//!
//! `cargo run --example dream_os_status --release`

use dream_os_kernel::{dispatch_dxbc_vector_add, dispatch_flash_attention, open_device};

const VECTOR_ADD_DXBC: &[u8] =
    include_bytes!("../../../../open-directx/crates/directx-shader-translate/shaders/vector_add.dxbc");

fn main() {
    println!("=== DreamOS status ===");
    println!("os: {}", std::env::consts::OS);
    println!("arch: {}", std::env::consts::ARCH);

    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            println!("vulkan device: UNAVAILABLE ({e})");
            println!("=== end ===");
            std::process::exit(1);
        }
    };
    println!("vulkan device: {}", device.info().name);

    const N: usize = 256;
    let a: Vec<f32> = (0..N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| (N - i) as f32 * 0.5).collect();

    match dispatch_dxbc_vector_add(&device, VECTOR_ADD_DXBC, &a, &b) {
        Ok(c) => {
            let mismatches = (0..N).filter(|&i| (c[i] - (a[i] + b[i])).abs() >= 1e-3).count();
            println!("open-directx DXBC->SPIR-V dispatch: {N} elements, {mismatches} mismatches");
            if mismatches == 0 {
                println!("result: OK");
            } else {
                println!("result: MISMATCH");
            }
        }
        Err(e) => {
            println!("open-directx DXBC->SPIR-V dispatch: FAILED ({e})");
        }
    }

    // open-cuda連携の第2弾: open-cuda-llmが既に実配線済みのfused
    // flash-attention SPIR-Vカーネルを、dream-osの共通Vulkan実行基盤から
    // そのまま再利用できることを実証する(deterministicな8x16のQ/K/V、
    // GPU結果とCPU参照実装〈opencuda-blas::flash_attention〉が一致する
    // ことまで実際に計算・比較する)。
    const SEQ_LEN: usize = 8;
    const HEAD_DIM: usize = 16;
    let mk = |offset: f32| -> Vec<f32> {
        (0..SEQ_LEN * HEAD_DIM).map(|i| ((i as f32 + offset) * 0.01).sin()).collect()
    };
    let q = mk(0.0);
    let k = mk(1.0);
    let v = mk(2.0);
    match dispatch_flash_attention(device.as_ref(), &q, &k, &v, SEQ_LEN, HEAD_DIM, 4) {
        Ok((_out, matches)) => {
            println!("open-cuda flash-attention SPIR-V dispatch: seq_len={SEQ_LEN}, head_dim={HEAD_DIM}, gpu==cpu_reference: {matches}");
            println!("result: {}", if matches { "OK" } else { "MISMATCH" });
        }
        Err(e) => {
            println!("open-cuda flash-attention SPIR-V dispatch: FAILED ({e})");
        }
    }

    println!("=== end ===");
}
