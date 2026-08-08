//! open-cuda連携ブリッジ第2弾(2026-08-08新設)。
//!
//! `directx_bridge.rs`(open-directx製DXBC→SPIR-V翻訳をこのマシンのVulkan
//! 実行基盤で動かす、2026-08-06新設)に続き、今回は`open-cuda`が
//! `open-cuda-llm`のDecoderLayerへ既に配線・実機検証済みの
//! **fused flash-attention SPIR-Vカーネル**(`opencuda-blas::
//! flash_attention_with_spirv`、QKᵀ・オンラインsoftmax・P·Vの累積を
//! 1回のcompute shaderディスパッチで完結させる設計、詳細は`open-cuda/
//! CLAUDE.md` 2026-08-07エントリ参照)を、dream-os-kernelのVulkan実行
//! 基盤(`open_device`)経由でそのまま呼び出せることを実証する。
//!
//! `directx_bridge`は「DirectX由来のシェーダーバイナリがVulkan層で動く」
//! ことを示す実装だったのに対し、こちらは「open-cudaのLLM推論向け
//! fusedカーネルを、dream-osの共通実行基盤からそのまま再利用できる」
//! ことを示す——DreamOSの「共通実行基盤」構想における2つ目の具体的な
//! 実装例(1つ目はDXBC/SPIR-Vブリッジ、2つ目はLLM推論カーネルの
//! 直接再利用)。
//!
//! シェーダ本体(`flash_attention.spv`)は`open-cuda`側の既存アセット
//! (`examples/flash_attention_vulkan_real/shaders/flash_attention.spv`)を
//! `include_bytes!`でそのまま取り込む——dream-os側で独自に再コンパイル・
//! 複製はしない(単一の真実源を`open-cuda`側に置く)。

use opencuda_core::GpuDevice;
use opencuda_blas::{flash_attention, flash_attention_with_spirv};

const FLASH_ATTENTION_SPIRV: &[u8] =
    include_bytes!("../../../../open-cuda/examples/flash_attention_vulkan_real/shaders/flash_attention.spv");

/// GPU(SPIR-V fused kernel)版とCPU参照実装の両方でFlash Attentionを実行し、
/// 両者が数値的に一致するかを確認した上でGPU側の結果を返す。
///
/// `dream_os_status`example等の呼び出し元は、返り値の`(output, matches)`の
/// うち`matches`をそのまま表示に使う想定(単なる「動いた/動かない」の
/// ステータスではなく、実際に計算した値がCPU参照実装と一致するかまで
/// 検証した「実際に計算された結果」を示す)。
pub fn dispatch_flash_attention(
    device: &dyn GpuDevice,
    q: &[f32],
    k: &[f32],
    v: &[f32],
    seq_len: usize,
    head_dim: usize,
    block_size: usize,
) -> anyhow::Result<(Vec<f32>, bool)> {
    let gpu_out = flash_attention_with_spirv(device, q, k, v, seq_len, head_dim, block_size, FLASH_ATTENTION_SPIRV)?;
    let cpu_out = flash_attention(q, k, v, seq_len, head_dim, block_size)?;

    let matches = gpu_out.len() == cpu_out.len()
        && gpu_out.iter().zip(cpu_out.iter()).all(|(g, c)| (g - c).abs() < 1e-3);

    Ok((gpu_out, matches))
}
