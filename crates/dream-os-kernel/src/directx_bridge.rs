//! open-directxブリッジ(2026-08-06新設)。
//!
//! ユーザー指示「dream-osをopen-directxとopen-cudaとaruaru-dbをベースに
//! 関連技術を実装開発、完成度と連携性の向上をして」への対応。
//!
//! `open-directx`(`directx-shader-translate`、path依存で再利用)が持つ
//! DXBC(D3D11 Compute Shaderバイトコード、`fxc.exe`実コンパイル済み)→
//! SPIR-V翻訳を、`dream-os-kernel`が既に持つVulkan実行基盤(`open_device`)
//! へそのまま接続する薄いブリッジ。**Windows専用のDirectXコンピュート
//! シェーダーバイナリを、DreamOSのWindows/Android共通Vulkan実行層で
//! そのまま動かせる**ことを実証する——これは「open-directx(DirectX
//! 互換層)・open-cuda(Vulkan実行基盤)・dream-os(統合層)」という3
//! リポジトリの連携性を具体的なコードで示す最初の実装。

use std::sync::Arc;

use anyhow::{Context, Result};
use directx_shader_translate::spirv_gen::translate_vector_add_shader;
use directx_shader_translate::OPENCUDA_VULKAN_DISPATCH_KERNEL_NAME;
use opencuda_core::{alloc_buffer, CompiledKernel, GpuDevice, KernelArg, LaunchConfig};

/// DXBCバイト列(`fxc.exe`でコンパイル済みのD3D11 Compute Shader、
/// `open-directx`の`vector_add.hlsl`と同じ契約: `InputA`/`InputB`/
/// `Output`のUAV束縛)を、実際にopen-directxの翻訳器でSPIR-Vへ変換し、
/// このマシンの実Vulkanデバイス上でディスパッチする。
///
/// `n`は`open-directx`側テストと同じ256要素契約
/// (`numthreads(64,1,1)` × 4ディスパッチグループ)を前提とする——
/// 呼び出し側は`n`が`kernel.local_size.0`の倍数になるよう注意すること。
pub fn dispatch_dxbc_vector_add(device: &Arc<dyn GpuDevice>, dxbc_bytes: &[u8], a: &[f32], b: &[f32]) -> Result<Vec<f32>> {
    anyhow::ensure!(a.len() == b.len(), "a and b must have the same length");
    let n = a.len();

    let kernel = translate_vector_add_shader(dxbc_bytes).context("DXBC->SPIR-V translation failed (open-directx)")?;

    let bytes = std::mem::size_of_val(a);
    let da = alloc_buffer(device, bytes)?;
    let db = alloc_buffer(device, bytes)?;
    let dc = alloc_buffer(device, bytes)?;

    da.copy_from_host(cast_f32_to_u8(a))?;
    db.copy_from_host(cast_f32_to_u8(b))?;

    let cfg = LaunchConfig::linear(n as u32, kernel.local_size.0);
    let spirv_bytes: Vec<u8> = kernel.spirv_words.iter().flat_map(|w| w.to_le_bytes()).collect();
    let compiled = CompiledKernel::spirv(OPENCUDA_VULKAN_DISPATCH_KERNEL_NAME, kernel.entry_point, spirv_bytes);

    device.launch_kernel(
        &compiled,
        &cfg,
        &[KernelArg::Ptr(da.as_ptr()), KernelArg::Ptr(db.as_ptr()), KernelArg::Ptr(dc.as_ptr()), KernelArg::Usize(n)],
    )?;
    device.synchronize()?;

    let mut c = vec![0.0f32; n];
    dc.copy_to_host(cast_f32_to_u8_mut(&mut c))?;
    Ok(c)
}

fn cast_f32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn cast_f32_to_u8_mut(v: &mut [f32]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, std::mem::size_of_val(v)) }
}
