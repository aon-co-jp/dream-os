//! `dream-os-raid-bridge`
//!
//! DreamOSの「共通実行基盤としてエコシステムの既存実装を再利用する」方針
//! (`dream-os-kernel/src/directx_bridge.rs`・`sbm.rs`等と同じパターン)を、
//! ユーザー指示にあった「RAID6 Z2実装+CPU/GPU/NPUハードウェアアクセラレータに
//! よるパリティ計算高速化」「4〜16枚のNVMe構成を想定したRAID6高速化」の
//! 領域へ適用したもの。
//!
//! **正直な開示(このモジュールがやっていること/やっていないこと)**:
//! - RAID-Z2(RAID6と数学的に同一、GF(2^8) Reed-SolomonのP/Qパリティ)の
//!   実装自体は`open-raid-z`(`open_raid_z_core::vdev::RaidZVdev`)に
//!   既に実在し、CPU実装に加えて`zfs_accel_hlsl`経由のD3D12/DirectML
//!   (Windows)・Vulkan Compute(Linux/macOS/Android)によるGPU/NPU
//!   アクセラレーションも既に実装・実機ベンチマーク済みだった
//!   (`open_raid_z_core/examples/raidz2_parity_benchmark.rs`、
//!   4〜8枚のNVMeを模したループバック構成)。**このクレートはその実装を
//!   ゼロから再実装するのではなく、path依存でそのまま呼び出すだけの
//!   薄いブリッジ**であり、DreamOS側で新しいRAID/GPUロジックを
//!   書いてはいない(車輪の再発明を避ける、このエコシステムの既存方針)。
//! - 4〜16枚のNVMeという要求のうち、このブリッジが実機検証したのは
//!   ループバックファイル(実NVMeではない)を使った4枚構成のみ
//!   (下記`tests/raid6_bridge_real.rs`参照)。実NVMeデバイス・16枚規模の
//!   検証はこのマシン(開発機、実NVMeドライブ複数枚を保有していない)では
//!   行えていない——`open_raid_z_core::block_device::BlockDevice`トレイトは
//!   実ブロックデバイスにも対応できる設計だが、その経路の実機検証は
//!   今回のスコープ外として正直に記録する。
//! - `opencuda_vulkan`(dream-os-kernelが使うVulkan Compute基盤)と
//!   `zfs_accel_hlsl`の実行基盤は現状**別々のデバイスハンドル**である
//!   (統合していない)。将来、両者を単一の`GpuDevice`抽象へ統合できるかは
//!   未検討(`zfs_accel_hlsl`はRAID-Z専用のGF(2^8)行列演算カーネルに
//!   特化しており、`opencuda_vulkan`の汎用compute kernelホワイトリストとは
//!   設計目的が異なるため、安易な統合は避けた)。

use open_raid_z_core::block_device::FileBackedDevice;
use open_raid_z_core::vdev::{RaidLevel, RaidZVdev};
use zfs_accel_hlsl::device::{detect_best_accelerator, AccelDevice, AccelKind};

/// RAID6(=RAID-Z2)のP/Qパリティ計算に使える最良のアクセラレータを検出する。
/// GPU/NPUが利用できない環境では`AccelKind::CpuFallback`が返る
/// (`open-raid-z`側の既存フォールバック設計をそのまま踏襲、DreamOS側での
/// 独自フォールバックロジックの追加は行っていない)。
pub fn detect_parity_accelerator() -> Option<AccelDevice> {
    match detect_best_accelerator() {
        Ok(dev) => {
            tracing::info!(kind = ?dev.kind, "RAID6/Z2パリティ計算アクセラレータを検出しました");
            Some(dev)
        }
        Err(err) => {
            tracing::warn!(?err, "アクセラレータ検出に失敗、CPU実装のみで動作します");
            None
        }
    }
}

/// このアクセラレータが実際にCPUへフォールバックするかどうか。
pub fn is_cpu_fallback(accel: &AccelDevice) -> bool {
    accel.kind == AccelKind::CpuFallback
}

/// テスト・ベンチマーク用: `num_data_disks`本のデータディスク+2本のパリティ
/// ディスク(RAID6/Z2固定)から成る、ループバックファイルベースの
/// `RaidZVdev`を構築する。実NVMeデバイスではない(上記モジュールdoc参照)。
pub fn build_loopback_raid6(
    dir: &std::path::Path,
    num_data_disks: usize,
    chunk_size: usize,
    stripe_count: u64,
    accel: Option<AccelDevice>,
) -> std::io::Result<RaidZVdev<FileBackedDevice>> {
    let total_disks = num_data_disks + 2;
    let disk_size = chunk_size as u64 * (stripe_count + 1);
    let devices: Vec<FileBackedDevice> = (0..total_disks)
        .map(|i| {
            let path = dir.join(format!("d{i}"));
            FileBackedDevice::create_fixed_size(&path, disk_size)
                .map_err(|e| std::io::Error::other(e.to_string()))
        })
        .collect::<std::io::Result<_>>()?;

    let mut vdev = RaidZVdev::new(devices, RaidLevel::Z2, chunk_size);
    if let Some(accel) = accel {
        vdev = vdev.with_accelerator(accel);
    }
    Ok(vdev)
}
