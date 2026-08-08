//! 東芝Simulated Bifurcation Machine(SBM)にインスパイアされた、量子
//! アニーリング風の組合せ最適化カーネル(2026-08-06追加)。
//!
//! ユーザー指示「東芝の擬似的な量子コンピューター技術で富士通より普通の
//! グラフィックボード一枚のPCの方が100倍高性能と言うインターネット
//! ニュースを…調査して、取り込んで」への対応。
//!
//! ## 調査結果(2026-08-06、日英でGoogle/GitHub調査、正直な開示)
//!
//! - **東芝SBM(Simulated Bifurcation Machine)は実在の技術**であり、
//!   量子アニーリングにインスパイアされた**古典計算(GPU上で動く通常の
//!   浮動小数点演算)による組合せ最適化アルゴリズム**。2026年4月、東芝は
//!   前世代比10〜100倍の高速化を実現する新アルゴリズムを発表し、16GPU
//!   構成で100万変数の問題を約30分(CPU版シミュレーテッドアニーリングの
//!   約20,000倍高速)で解いたと報告している([東芝公式](https://digitalpr.jp/r/44929)、
//!   [EE Times Japan](https://eetimes.itmedia.co.jp/ee/articles/2604/09/news036.html))。
//! - **富士通デジタルアニーラとの比較**: デジタルアニーラは専用ASIC
//!   ハードウェアで超高速だが、変数数が8,192までとスケーラビリティに
//!   制約がある。一方、東芝SBMはGPU上で動作し10万変数以上の大規模問題に
//!   対応でき、規模・解の質の両面で優位という報告がある
//!   ([IEEE Spectrum](https://spectrum.ieee.org/toshiba--optimization-algorithm-speed-record-combinatorial-problems))。
//!   「グラフィックボード1枚が富士通より100倍高性能」という報道の文脈は、
//!   専用ASIC(デジタルアニーラ)に対して汎用GPU上のソフトウェア
//!   アルゴリズムが特定の問題規模・条件で優位性を示せる、という趣旨と
//!   考えられる——ただし公平な比較かどうかは前提条件(問題の種類・規模・
//!   世代)に強く依存する点には注意が必要(独立した第三者比較として
//!   [文部科学省調査研究](https://www.mext.go.jp/)でもSQBM+/デジタル
//!   アニーラ/NECベクトルアニーリングのベンチマーク比較が行われている
//!   ことを確認した)。
//! - **実装したもの**: SBMの核心アルゴリズムである**ballistic Simulated
//!   Bifurcation(bSB)**(Goto et al. 2019
//!   "Combinatorial optimization by simulating adiabatic bifurcations in
//!   nonlinear Hamiltonian systems"で提案された、断熱的パラメータランプ+
//!   反射境界条件による古典力学系のシミュレーション)を、
//!   `shaders/sbm_ising.comp`として実際にGPU(Vulkan Compute)上へ実装した。
//!   Ising模型(スピン変数+結合行列)の最小化問題を解く——組合せ最適化の
//!   核心部分は東芝の実装と同じ古典的な力学系シミュレーションだが、
//!   東芝SQBM+が持つ商用グレードの高度化(離散化アルゴリズムdSB・
//!   FPGA実装・100万変数規模対応等)は含まない、PoCレベルの実装である
//!   ことを正直に開示する。
//! - **正直な性能の開示**: 「100倍高性能」を主張するものではない——
//!   このPoCは64スピンの小規模Ising問題を1ワークグループで解く最小
//!   実装であり、東芝の商用実装との性能比較は行っていない(そもそも
//!   目的が異なる: 東芝側は実際の産業応用〈創薬・金融・配送最適化等〉
//!   向けの本番システム、こちらは「GPU上で量子アニーリング風アルゴリズム
//!   が実際に動き、正しい解へ収束するか」を検証するための最小実装)。

use std::sync::Arc;

use anyhow::Result;
use opencuda_core::{alloc_buffer, CompiledKernel, GpuDevice, KernelArg, LaunchConfig};

/// このPoC実装のスピン数(シェーダの`NUM_SPINS`定数と一致させる必要が
/// ある、共有メモリのサイズをコンパイル時に固定する設計のため)。
pub const NUM_SPINS: usize = 64;

/// Ising問題の実行結果。
pub struct SbmResult {
    /// 各スピンの最終値(+1.0または-1.0)。
    pub spins: Vec<f32>,
    /// Ising エネルギー `E = -sum_{i<j} J_ij * spin_i * spin_j`
    /// (組合せ最適化では、これを最小化することが目的関数の最小化に対応する
    /// ことが多い、例: MaxCut問題)。
    pub energy: f64,
}

/// 密な対称結合行列`j_matrix`(`NUM_SPINS x NUM_SPINS`、対角成分は0)を
/// GPU上でSBM(ballistic Simulated Bifurcation)により最適化する。
///
/// `init_x`はホスト側で決定的に生成した初期値(CPU参照実装と全く同じ
/// 初期値を使うことで、GPU/CPU双方の軌道を数値的に比較検証できる設計)。
#[allow(clippy::too_many_arguments)]
pub fn run_sbm_ising(
    device: &Arc<dyn GpuDevice>,
    spirv: &[u8],
    j_matrix: &[f32],
    init_x: &[f32],
    steps: u32,
    dt: f32,
    c0: f32,
    a0: f32,
) -> Result<SbmResult> {
    anyhow::ensure!(j_matrix.len() == NUM_SPINS * NUM_SPINS, "j_matrix must be {}x{}", NUM_SPINS, NUM_SPINS);
    anyhow::ensure!(init_x.len() == NUM_SPINS, "init_x must have {NUM_SPINS} elements");

    let d_j = alloc_buffer(device, j_matrix.len() * 4)?;
    let d_init = alloc_buffer(device, init_x.len() * 4)?;
    let d_out = alloc_buffer(device, NUM_SPINS * 4)?;

    d_j.copy_from_host(cast_f32_to_u8(j_matrix))?;
    d_init.copy_from_host(cast_f32_to_u8(init_x))?;

    let cfg = LaunchConfig::linear(NUM_SPINS as u32, NUM_SPINS as u32);
    let kernel = CompiledKernel::spirv("sbm_ising", "main", spirv.to_vec());

    device.launch_kernel(
        &kernel,
        &cfg,
        &[
            KernelArg::Ptr(d_j.as_ptr()),
            KernelArg::Ptr(d_init.as_ptr()),
            KernelArg::Ptr(d_out.as_ptr()),
            KernelArg::U32(steps),
            KernelArg::F32(dt),
            KernelArg::F32(c0),
            KernelArg::F32(a0),
        ],
    )?;
    device.synchronize()?;

    let mut spins = vec![0f32; NUM_SPINS];
    d_out.copy_to_host(cast_f32_to_u8_mut(&mut spins))?;

    let energy = ising_energy(j_matrix, &spins);
    Ok(SbmResult { spins, energy })
}

/// CPU参照実装(逐次)——GPU版と全く同じballistic SB更新式を、共有
/// メモリの代わりにホスト側配列で計算する。実機検証で数値一致を確認する
/// ための対照実装。
pub fn run_sbm_ising_cpu_reference(j_matrix: &[f32], init_x: &[f32], steps: u32, dt: f32, c0: f32, a0: f32) -> SbmResult {
    let n = NUM_SPINS;
    let mut x = init_x.to_vec();
    let mut y = vec![0f32; n];

    for step in 0..steps {
        let a_t = a0 * (step as f32 / steps as f32);
        let mut x_new = vec![0f32; n];
        let mut y_new = vec![0f32; n];
        for i in 0..n {
            let mut coupling = 0f32;
            for j in 0..n {
                coupling += j_matrix[i * n + j] * x[j];
            }
            let yn = y[i] + (-(a0 - a_t) * x[i] + c0 * coupling) * dt;
            let mut xn = x[i] + a0 * yn * dt;
            let mut yn2 = yn;
            if xn > 1.0 {
                xn = 1.0;
                yn2 = 0.0;
            } else if xn < -1.0 {
                xn = -1.0;
                yn2 = 0.0;
            }
            x_new[i] = xn;
            y_new[i] = yn2;
        }
        x = x_new;
        y = y_new;
    }

    let spins: Vec<f32> = x.iter().map(|&v| if v >= 0.0 { 1.0 } else { -1.0 }).collect();
    let energy = ising_energy(j_matrix, &spins);
    SbmResult { spins, energy }
}

fn ising_energy(j_matrix: &[f32], spins: &[f32]) -> f64 {
    let n = NUM_SPINS;
    let mut energy = 0f64;
    for i in 0..n {
        for j in (i + 1)..n {
            energy -= (j_matrix[i * n + j] as f64) * (spins[i] as f64) * (spins[j] as f64);
        }
    }
    energy
}

fn cast_f32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn cast_f32_to_u8_mut(v: &mut [f32]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, std::mem::size_of_val(v)) }
}
