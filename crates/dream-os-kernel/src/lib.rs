//! dream-os-kernel: DreamOS構想(Linux×TRONハイブリッドカーネル、
//! Windows/macOS/Android/各種Linux互換)の最初の技術的PoC(2026-08-06)。
//!
//! **現時点のスコープ(正直な開示)**: 「カーネル」を新規に書くものでは
//! ない。`F:\runo\dream-os\CLAUDE.md`の技術調査結果で確認した通り、
//! 単一カーネルへ全プラットフォームのABIを直接実装するアプローチは
//! 前例が乏しく技術的難度が高い。代わりに、このエコシステムが既に
//! 実機検証済みの`open-cuda`の`opencuda-vulkan`(Vulkan Compute
//! バックエンド、Windows実機GT730で検証済み・Android
//! `aarch64-linux-android`向けクロスコンパイル成功実績あり)を、
//! **Windows/Androidで共通利用できる実行基盤**として薄くラップする
//! ことから着手する——「ゼロから独自OSカーネルを書く」のではなく
//! 「実績のある既存クロスプラットフォーム基盤を、DreamOSの共通実行層
//! として組み立て直す」というChromeOS/crosvm型のアプローチに沿う。
//!
//! 対応スコープはユーザー指示(2026-08-06)により、実機のあるWindows・
//! Androidの2プラットフォームに限定する(macOS/iPhone・PS5/6・Switch2は
//! 将来のライセンス取得を前提とした保留)。

pub mod directx_bridge;
pub mod flash_attention_bridge;
pub mod mining;
pub mod power_profile;
pub mod sbm;

pub use directx_bridge::dispatch_dxbc_vector_add;
pub use flash_attention_bridge::dispatch_flash_attention;
pub use mining::{hashrate, MiningBatchResult, MiningWorker};
pub use power_profile::MiningPowerProfile;
pub use sbm::{run_sbm_ising, run_sbm_ising_cpu_reference, SbmResult};

use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use opencuda_core::{alloc_buffer, CompiledKernel, GpuDevice, KernelArg, LaunchConfig};
use opencuda_vulkan::VulkanDevice;

/// このマシン上でVulkanデバイスを開く(Windows/Android共通の入口)。
///
/// `open-cuda`の`opencuda-vulkan`は元々ヘッドレスなCompute専用設計
/// (ウィンドウイングAPIへの依存が無い)であることが確認済みのため、
/// Windows/Android両方でこの同じ呼び出しが動く見込み
/// (`open-cuda/CLAUDE.md`2026-07-25エントリの監査結果を参照——
/// Android実機での`vkCreateInstance`成功自体は本PoCでは未検証、
/// 下記`README.md`の正直な開示を参照)。
pub fn open_device(device_id: usize) -> Result<Arc<dyn GpuDevice>> {
    let device = VulkanDevice::new(device_id).context("failed to open Vulkan device")?;
    Ok(device as Arc<dyn GpuDevice>)
}

/// `vector_add`カーネル(SPIR-V)を1回ディスパッチし、実行時間を計測する。
///
/// マイニングOS向けの電力出力調整(`power_profile::MiningPowerProfile`)は、
/// この関数が返す実測ディスパッチ時間を基準に休止時間を計算する設計
/// (呼び出し側が`sleep_after_dispatch`と組み合わせて使う、下記
/// `examples/dispatch_throttled.rs`参照)。
pub fn dispatch_vector_add_once(device: &Arc<dyn GpuDevice>, spirv: &[u8], n: usize) -> Result<std::time::Duration> {
    let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..n).map(|i| (n - i) as f32).collect();
    let bytes = n * std::mem::size_of::<f32>();

    let da = alloc_buffer(device, bytes)?;
    let db = alloc_buffer(device, bytes)?;
    let dc = alloc_buffer(device, bytes)?;

    da.copy_from_host(cast_f32_to_u8(&a))?;
    db.copy_from_host(cast_f32_to_u8(&b))?;

    let cfg = LaunchConfig::linear(n as u32, 256);
    let kernel = CompiledKernel::spirv("vector_add", "main", spirv.to_vec());

    let start = Instant::now();
    device.launch_kernel(
        &kernel,
        &cfg,
        &[KernelArg::Ptr(da.as_ptr()), KernelArg::Ptr(db.as_ptr()), KernelArg::Ptr(dc.as_ptr()), KernelArg::Usize(n)],
    )?;
    device.synchronize()?;
    let elapsed = start.elapsed();

    let mut c = vec![0.0f32; n];
    dc.copy_to_host(cast_f32_to_u8_mut(&mut c))?;
    let expected = n as f32;
    for (idx, &v) in c.iter().enumerate() {
        if (v - expected).abs() > 1e-3 {
            anyhow::bail!("mismatch at {idx}: got {v}, expected {expected}");
        }
    }

    Ok(elapsed)
}

fn cast_f32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn cast_f32_to_u8_mut(v: &mut [f32]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, std::mem::size_of_val(v)) }
}
