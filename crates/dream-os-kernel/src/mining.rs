//! マイニング相当の実ハッシュ計算カーネル(2026-08-06追加、ユーザー指示
//! 「マイニング相当の実ハッシュ計算カーネル追加…実際にグラフィックボード
//! でマイニングも可能に」)。
//!
//! `shaders/sha256d_mine.comp`(単一64バイトブロックのSHA-256を2回適用、
//! Bitcoin等のPoWで実際に使われるdouble-SHA256と同じ構成)を実Vulkan
//! デバイス上でディスパッチし、CPU参照実装(`sha2`クレート)と数値一致
//! することを検証できる。
//!
//! ## 複数GPU(1枚〜数千枚)対応について(正直な開示)
//!
//! `open-cuda`の`opencuda_vulkan::VulkanDevice::new(id)`を実際に読んだ
//! ところ、`id`引数は**物理デバイス選択には使われておらず、常に
//! 「computeキューを持つ最初に見つかった物理デバイス」を開く**設計に
//! なっていることが判明した(`open-cuda/crates/opencuda-vulkan/src/
//! real.rs`の`VulkanDevice::new`実装を確認済み)。つまり、このマシンに
//! GPUが複数枚あったとしても、現状の`opencuda-vulkan`では2枚目以降を
//! 明示的に選んで開く手段が無い——**「1枚から数千枚のグラフィック
//! ボードでマイニング」を実現するには、まず`opencuda-vulkan`側に
//! 物理デバイスインデックス指定機能を追加する必要がある**(このリポジトリ
//! 単体では直せない、`open-cuda`側の変更が必要な既知のギャップとして
//! ここに明記する)。
//!
//! このマシン(開発環境)にはNVIDIA GeForce GT 730が1枚のみ搭載されている
//! ため(`open-cuda/CLAUDE.md`に記録済みの既知の制約)、複数GPUでの実機
//! 検証はそもそも不可能——本モジュールが提供する`MiningWorker`は
//! **単一デバイスに対する1ワーカー**の抽象化に留め、「N枚のGPUへ
//! nonce空間を分割して割り当てる」という上位のオーケストレーション層は
//! 呼び出し側(将来、複数の`MiningWorker`をVulkanデバイスの数だけ生成する
//! コード)に委ねる設計とした。
//!
//! **「数千枚」規模について**: 1台のマシンにGPUを数千枚搭載することは
//! 現実的ではない(一般的なマイニングリグでも数十枚が実務上の上限)。
//! 数千枚規模を実現するには、単一プロセス内の複数デバイス分散ではなく
//! **複数マシン(ノード)にまたがる分散マイニング**(Stratumプロトコル
//! 相当のプール通信層)が必要になる——これは本PoCのスコープを大きく
//! 超える別レイヤーの設計であり、今回は着手していないことを正直に
//! 明記する。

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use opencuda_core::{alloc_buffer, CompiledKernel, GpuDevice, KernelArg, LaunchConfig};

/// 1回のディスパッチで計算するnonce数(ワークグループサイズ256の倍数、
/// シェーダの`local_size_x=256`に合わせる)。
pub const DEFAULT_BATCH_SIZE: u32 = 256 * 4096;

/// 単一Vulkanデバイスに対する採掘(ハッシュ計算)ワーカー。
///
/// 複数GPU環境では、呼び出し側がデバイスの数だけ`MiningWorker`を生成し、
/// 各ワーカーへ重複しない`nonce_base`範囲を割り当てることで、原理的には
/// 複数デバイスへスケールする設計にしてある——ただし上記モジュールdoc
/// の通り、`opencuda-vulkan`側が現状1枚のGPUしか開けないため、この
/// 「原理的な」スケーラビリティは実機で検証できていない。
pub struct MiningWorker {
    device: Arc<dyn GpuDevice>,
    spirv: Vec<u8>,
}

/// 1バッチ分の採掘結果。
pub struct MiningBatchResult {
    /// このバッチで計算したハッシュ数。
    pub hashes: u64,
    /// このバッチのディスパッチ+同期にかかった実測時間。
    pub elapsed: Duration,
    /// 各nonceに対応する32バイトダイジェスト(ビッグエンディアン、
    /// SHA-256の慣例通り)。検証・share探索用に呼び出し側へ返す。
    pub digests: Vec<[u8; 32]>,
}

impl MiningWorker {
    pub fn new(device: Arc<dyn GpuDevice>, spirv: Vec<u8>) -> Self {
        Self { device, spirv }
    }

    /// `nonce_base..nonce_base+count`の範囲のnonceについて、
    /// `base_message`(32バイト、先頭4バイトはシェーダ側でnonceに
    /// 上書きされるためどんな値でもよい)を使ってdouble-SHA256を計算する。
    pub fn mine_batch(&self, base_message: [u32; 8], nonce_base: u32, count: u32) -> Result<MiningBatchResult> {
        let base_bytes: Vec<u8> = base_message.iter().flat_map(|w| w.to_le_bytes()).collect();
        let digest_bytes = count as usize * 32;

        let d_base = alloc_buffer(&self.device, base_bytes.len())?;
        let d_digests = alloc_buffer(&self.device, digest_bytes)?;
        d_base.copy_from_host(&base_bytes)?;

        let cfg = LaunchConfig::linear(count, 256);
        let kernel = CompiledKernel::spirv("sha256d_mine", "main", self.spirv.clone());

        let start = Instant::now();
        self.device.launch_kernel(
            &kernel,
            &cfg,
            &[
                KernelArg::Ptr(d_base.as_ptr()),
                KernelArg::Ptr(d_digests.as_ptr()),
                KernelArg::U32(nonce_base),
                KernelArg::U32(count),
            ],
        )?;
        self.device.synchronize()?;
        let elapsed = start.elapsed();

        let mut raw = vec![0u8; digest_bytes];
        d_digests.copy_to_host(&mut raw)?;

        let digests = raw
            .chunks_exact(32)
            .map(|chunk| {
                // シェーダは8xuint32(ホストのリトルエンディアン格納だが
                // 各ワード自体はSHA-256のビッグエンディアン語順)で書き込む
                // ため、ワード単位でビッグエンディアンbytesへ変換する。
                let mut digest = [0u8; 32];
                for (i, word_bytes) in chunk.chunks_exact(4).enumerate() {
                    let word = u32::from_le_bytes(word_bytes.try_into().unwrap());
                    digest[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
                }
                digest
            })
            .collect();

        Ok(MiningBatchResult { hashes: count as u64, elapsed, digests })
    }

    pub fn device_name(&self) -> String {
        self.device.info().name.clone()
    }
}

/// ハッシュレート(hashes/sec)を計算する。
pub fn hashrate(hashes: u64, elapsed: Duration) -> f64 {
    if elapsed.as_secs_f64() <= 0.0 {
        return 0.0;
    }
    hashes as f64 / elapsed.as_secs_f64()
}
