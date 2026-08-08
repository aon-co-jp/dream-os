//! open-cuda(fused flash-attention SPIR-Vカーネル)→dream-os-kernel
//! (Vulkan実行基盤)の実機ブリッジ検証(2026-08-08)。
//!
//! `open-cuda-llm`のDecoderLayerへ既に配線・実機検証済みの
//! `opencuda-blas::flash_attention_with_spirv`を、dream-os-kernelの
//! Vulkan実行基盤上でそのまま実行し、CPU参照実装(`flash_attention`)と
//! 数値一致することを確認する。

use dream_os_kernel::{dispatch_flash_attention, open_device};

#[test]
fn flash_attention_spirv_runs_on_dream_os_vulkan_backend_and_matches_cpu_reference() {
    let device = match open_device(0) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("skipping: no real Vulkan device available: {e}");
            return;
        }
    };
    println!("device: {}", device.info().name);

    const SEQ_LEN: usize = 8;
    const HEAD_DIM: usize = 16;
    let mk = |offset: f32| -> Vec<f32> { (0..SEQ_LEN * HEAD_DIM).map(|i| ((i as f32 + offset) * 0.01).sin()).collect() };
    let q = mk(0.0);
    let k = mk(1.0);
    let v = mk(2.0);

    let (_out, matches) =
        dispatch_flash_attention(device.as_ref(), &q, &k, &v, SEQ_LEN, HEAD_DIM, 4).expect("flash-attention dispatch failed");

    assert!(matches, "GPU flash-attention output does not match CPU reference within tolerance");

    println!(
        "OK: open-cuda製fused flash-attention SPIR-Vカーネルが、dream-os-kernelのVulkan実行基盤上で\
         実際にディスパッチされ、CPU参照実装(opencuda-blas::flash_attention)と一致した"
    );
}
