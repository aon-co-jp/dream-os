//! `dream-os-raid-bridge`の実機検証テスト。
//!
//! **正直な開示**: 実NVMeドライブは使っていない(ループバックファイル、
//! `open_raid_z_core`側の既存ベンチマークと同じ手法)。このマシンには
//! 複数枚の実NVMeが無いため、実ディスクI/Oでの検証は行えない。
//! ここで検証しているのは「open-raid-zのRAID6/Z2実装+
//! zfs_accel_hlslのアクセラレータ検出を、dream-osから実際に呼び出して
//! 正しく書き込み・読み出し・自己修復できること」である。

use dream_os_raid_bridge::{build_loopback_raid6, detect_parity_accelerator, is_cpu_fallback};
use open_raid_z_core::block_device::BlockDevice;

#[test]
fn raid6_write_read_roundtrip_with_detected_accelerator() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let accel = detect_parity_accelerator();
    if let Some(a) = &accel {
        eprintln!(
            "RAID6パリティアクセラレータ: {:?} (CPUフォールバック: {})",
            a.kind,
            is_cpu_fallback(a)
        );
    } else {
        eprintln!("アクセラレータ検出失敗、CPU実装のみで検証します");
    }

    // 4データディスク+2パリティ = 6ディスク構成(4〜16 NVMe想定レンジの下限寄り)。
    let chunk_size = 4096;
    let stripe_count = 8;
    let mut vdev = build_loopback_raid6(tmp.path(), 4, chunk_size, stripe_count, accel)
        .expect("build_loopback_raid6");

    let stripe_data = vec![0x5Au8; chunk_size * 4];
    for i in 0..stripe_count {
        vdev.write_stripe(i, &stripe_data).expect("write_stripe");
    }
    for i in 0..stripe_count {
        let read_back = vdev.read_stripe(i).expect("read_stripe");
        assert_eq!(read_back, stripe_data, "stripe {i} roundtrip mismatch");
    }
}

#[test]
fn raid6_self_heals_single_disk_corruption() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let chunk_size = 4096;
    let stripe_count = 1;
    // CPU実装のみで検証(アクセラレータ有無に依らず自己修復ロジックは
    // 同じ経路を通る想定だが、このテストは自己修復の正しさに焦点を絞るため
    // 意図的にNoneを渡す)。
    let mut vdev = build_loopback_raid6(tmp.path(), 4, chunk_size, stripe_count, None)
        .expect("build_loopback_raid6");

    let stripe_data = vec![0x7Eu8; chunk_size * 4];
    vdev.write_stripe(0, &stripe_data).expect("write_stripe");

    // 1台のディスクを直接壊す(サイレント破損を模擬)。RAID6/Z2は
    // パリティ2本のため1台までのサイレント破損から復元できるはず。
    vdev.devices_mut()[1]
        .write_at(0, &vec![0xFFu8; chunk_size])
        .expect("corrupt device 1");

    let (read_back, healed) = vdev.read_stripe_with_report(0).expect("read_stripe_with_report");
    assert_eq!(read_back, stripe_data, "self-heal did not recover correct data");
    assert!(!healed.is_empty(), "expected read_stripe_with_report to report a healed/mismatched disk");
}
