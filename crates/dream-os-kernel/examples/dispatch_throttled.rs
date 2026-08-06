//! DreamOS PoC: 電力出力調整可能なマイニングOS向けディスパッチデモ。
//!
//! `shaders/vector_add.spv`(事前に`glslc`等でコンパイル)を実Vulkan
//! デバイス上で複数回ディスパッチし、`MiningPowerProfile`のデューティ
//! サイクル制御で実効稼働率を落とせることを、実測の休止時間の合計で
//! 確認する。
//!
//! 使い方: `cargo run --example dispatch_throttled -- <power_percent> <iterations>`
//! (例: `cargo run --example dispatch_throttled -- 50 5`)

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use dream_os_kernel::{dispatch_vector_add_once, open_device, MiningPowerProfile};

const N: usize = 200_000;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let power_percent: u8 = args.next().and_then(|s| s.parse().ok()).unwrap_or(100);
    let iterations: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(5);

    let spirv = std::fs::read(shader_path()).context(
        "failed to read shaders/vector_add.spv. Compile it first, e.g.: glslc shaders/vector_add.comp -o shaders/vector_add.spv",
    )?;

    let device = open_device(0)?;
    println!("device: {}", device.info().name);
    println!("mining power profile: {power_percent}% ({iterations} dispatches)");

    let profile = MiningPowerProfile::new(power_percent);
    let wall_clock_start = Instant::now();
    let mut total_dispatch_time = Duration::ZERO;
    let mut total_sleep_time = Duration::ZERO;

    for i in 0..iterations {
        let elapsed = dispatch_vector_add_once(&device, &spirv, N)?;
        total_dispatch_time += elapsed;
        println!("  dispatch {i}: {elapsed:?}");

        let sleep = profile.sleep_after_dispatch(elapsed);
        if sleep == Duration::MAX {
            println!("  power_percent=0: stopping (no further dispatches)");
            break;
        }
        if !sleep.is_zero() {
            std::thread::sleep(sleep);
        }
        total_sleep_time += sleep;
    }

    let wall_clock = wall_clock_start.elapsed();
    let actual_duty_cycle = if wall_clock.as_secs_f64() > 0.0 {
        100.0 * total_dispatch_time.as_secs_f64() / wall_clock.as_secs_f64()
    } else {
        0.0
    };

    println!("total dispatch time: {total_dispatch_time:?}");
    println!("total sleep time:    {total_sleep_time:?}");
    println!("wall clock:          {wall_clock:?}");
    println!("actual duty cycle:   {actual_duty_cycle:.1}% (target: {power_percent}%)");

    Ok(())
}

fn shader_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders").join("vector_add.spv")
}
