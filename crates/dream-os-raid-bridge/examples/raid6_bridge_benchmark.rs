//! DreamOS PoC: RAID6/Z2ブリッジのrunnableデモ(2026-08-08)。
//!
//! `dream-os-kernel`の`examples/sbm_benchmark.rs`・`mine_benchmark.rs`と
//! 同じ位置づけ——「テストのソースコードを読まないと何も確認できない」
//! というギャップを埋めるための、1コマンドで実行できるデモ。
//! `tests/raid6_bridge_real.rs`と同じAPI(`build_loopback_raid6`/
//! `detect_parity_accelerator`)を使い、実際にストライプの書き込み・
//! 読み出しラウンドトリップと、1台のディスク破損からの自己修復を
//! 標準出力へ実況しながら実行する。
//!
//! **正直な開示**: `raid6_bridge_real.rs`と同様、実NVMeドライブではなく
//! ループバックファイルを使う(このマシンには複数枚の実NVMeが無いため)。
//! 検出されるアクセラレータ(GPU/NPU/CPU)は実機の構成に依存する——このPoCは
//! 「検出結果を偽装しない」という既存方針を維持し、`AccelKind::CpuFallback`
//! が返ってもエラーにはしない。
//!
//! `cargo run -p dream-os-raid-bridge --example raid6_bridge_benchmark --release`

use dream_os_raid_bridge::{build_loopback_raid6, detect_parity_accelerator, is_cpu_fallback};
use open_raid_z_core::block_device::BlockDevice;

fn main() {
    let tmp = tempfile::tempdir().expect("tempdir");

    let accel = detect_parity_accelerator();
    match &accel {
        Some(a) => println!(
            "RAID6パリティアクセラレータ: {:?} (CPUフォールバック: {})",
            a.kind,
            is_cpu_fallback(a)
        ),
        None => println!("アクセラレータ検出失敗、CPU実装のみで実行します"),
    }

    // 4データディスク+2パリティ = 6ディスク構成(4〜16 NVMe想定レンジの下限寄り)。
    const NUM_DATA_DISKS: usize = 4;
    const CHUNK_SIZE: usize = 4096;
    const STRIPE_COUNT: u64 = 8;

    let mut vdev = build_loopback_raid6(tmp.path(), NUM_DATA_DISKS, CHUNK_SIZE, STRIPE_COUNT, accel)
        .expect("build_loopback_raid6");

    // --- 1. 書き込み->読み出しラウンドトリップ ---
    let stripe_data = vec![0x5Au8; CHUNK_SIZE * NUM_DATA_DISKS];
    for i in 0..STRIPE_COUNT {
        vdev.write_stripe(i, &stripe_data).expect("write_stripe");
    }
    let mut roundtrip_ok = true;
    for i in 0..STRIPE_COUNT {
        let read_back = vdev.read_stripe(i).expect("read_stripe");
        if read_back != stripe_data {
            roundtrip_ok = false;
        }
    }
    println!(
        "roundtrip: {} stripes x {} bytes each, all match: {}",
        STRIPE_COUNT,
        CHUNK_SIZE * NUM_DATA_DISKS,
        roundtrip_ok
    );

    // --- 2. 1台のディスクを直接壊し、自己修復できるか確認 ---
    vdev.devices_mut()[1]
        .write_at(0, &vec![0xFFu8; CHUNK_SIZE])
        .expect("corrupt device 1");
    let (healed_data, healed_report) = vdev.read_stripe_with_report(0).expect("read_stripe_with_report");
    let self_heal_ok = healed_data == stripe_data && !healed_report.is_empty();
    println!(
        "self-heal after corrupting disk #1: data matches original: {}, reported healed/mismatched disks: {}",
        healed_data == stripe_data,
        healed_report.len()
    );

    if roundtrip_ok && self_heal_ok {
        println!("OK: RAID6/Z2 write/read roundtrip and single-disk self-heal both verified on this machine");
    } else {
        eprintln!("FAILED: roundtrip_ok={roundtrip_ok}, self_heal_ok={self_heal_ok}");
        std::process::exit(1);
    }
}
