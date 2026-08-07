//! DreamOS PoC: open-directx(DXBC->SPIR-V翻訳)ブリッジのAndroid/Windows
//! 実機検証用ベンチマーク(2026-08-07)。
//!
//! `tests/directx_bridge_real_vulkan.rs`と同じ検証を、`cargo test`が
//! 動かないAndroidクロスビルド環境でも`adb push`して直接実行できる
//! バイナリとして提供する(`examples/mine_benchmark.rs`と同じパターン)。
//! DXBCバイト列は`include_bytes!`でビルド時に埋め込まれるため、実機側に
//! open-directxリポジトリを配置する必要は無い。
//!
//! `cargo run --example directx_bridge_benchmark --release`

use dream_os_kernel::{dispatch_dxbc_vector_add, open_device};

const VECTOR_ADD_DXBC: &[u8] =
    include_bytes!("../../../../open-directx/crates/directx-shader-translate/shaders/vector_add.dxbc");

fn main() {
    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("no real Vulkan device available: {e}");
            std::process::exit(1);
        }
    };
    println!("device: {}", device.info().name);

    const N: usize = 256; // vector_add.hlslのnumthreads(64,1,1) x 4グループ契約
    let a: Vec<f32> = (0..N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| (N - i) as f32 * 0.5).collect();

    let c = dispatch_dxbc_vector_add(&device, VECTOR_ADD_DXBC, &a, &b).expect("DXBC->SPIR-V->Vulkan dispatch failed");

    let mut mismatches = 0usize;
    for i in 0..N {
        let expected = a[i] + b[i];
        if (c[i] - expected).abs() >= 1e-3 {
            mismatches += 1;
            eprintln!("mismatch at {i}: got {}, expected {expected}", c[i]);
        }
    }

    if mismatches == 0 {
        println!(
            "OK: open-directx製DXBC->SPIR-V翻訳が、dream-os-kernelのVulkan実行基盤(open-cuda再利用)上で\
             実際にディスパッチされ、{N}要素すべてCPU参照実装と一致した"
        );
    } else {
        eprintln!("MISMATCH: {mismatches}/{N} elements disagree");
        std::process::exit(1);
    }
}
