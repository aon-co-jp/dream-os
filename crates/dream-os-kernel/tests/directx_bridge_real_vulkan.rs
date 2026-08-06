//! open-directx(DXBC->SPIR-V翻訳)→dream-os-kernel(Vulkan実行基盤)の
//! 実機ブリッジ検証(2026-08-06)。
//!
//! `open-directx`側の`fxc.exe`実コンパイル済み`vector_add.dxbc`を
//! `include_bytes!`で直接取り込み、DreamOSのVulkan実行基盤上で
//! ディスパッチしてCPU参照実装と数値一致することを確認する。

use dream_os_kernel::{dispatch_dxbc_vector_add, open_device};

const VECTOR_ADD_DXBC: &[u8] =
    include_bytes!("../../../../open-directx/crates/directx-shader-translate/shaders/vector_add.dxbc");

#[test]
fn dxbc_vector_add_runs_on_dream_os_vulkan_backend_and_matches_cpu_reference() {
    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("skipping: no real Vulkan device available: {e}");
            return;
        }
    };
    println!("device: {}", device.info().name);

    const N: usize = 256; // vector_add.hlslのnumthreads(64,1,1) x 4グループ契約
    let a: Vec<f32> = (0..N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| (N - i) as f32 * 0.5).collect();

    let c = dispatch_dxbc_vector_add(&device, VECTOR_ADD_DXBC, &a, &b).expect("DXBC->SPIR-V->Vulkan dispatch failed");

    for i in 0..N {
        let expected = a[i] + b[i];
        assert!((c[i] - expected).abs() < 1e-3, "mismatch at {i}: got {}, expected {expected}", c[i]);
    }

    println!(
        "OK: open-directx製DXBC->SPIR-V翻訳が、dream-os-kernelのVulkan実行基盤(open-cuda再利用)上で\
         実際にディスパッチされ、{N}要素すべてCPU参照実装と一致した"
    );
}
