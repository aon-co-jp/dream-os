//! 電力出力調整が可能なマイニングOS向けの電源プロファイル(2026-08-06新設)。
//!
//! `open-web-server`/`open-easy-web`/`RPoem`等、このエコシステム全体で
//! 既に確立されている「省電力/省メモリ/常時電源接続」という独立フラグの
//! 組み合わせパターンを踏襲する。DreamOSではこれに加え、マイニング
//! ワークロード(GPU計算の連続ディスパッチ)向けに**電力出力(デューティ
//! サイクル)を0〜100%で調整できる**`mining_power_percent`を追加する。
//!
//! **正直な開示・スコープ**: これは`nvidia-smi -pl <watts>`のような、
//! GPUファームウェア/ドライバに対するハードウェアレベルの電力制限
//! (power limit)API呼び出しではない——本クレートはVulkanのみを使う
//! クロスプラットフォーム設計(Windows/Android両対応)のため、ベンダー
//! 固有の電力管理APIには依存しない。代わりに、ディスパッチの合間に
//! `mining_power_percent`に応じた休止時間を挟む**ソフトウェア側の
//! デューティサイクル制御**により、実効GPU稼働率(≒消費電力)を
//! 概算的に調整する。真のハードウェア電力制限API連携は将来の課題として
//! 明記する(下記`MiningPowerProfile`のdocコメント参照)。

use std::time::Duration;

/// マイニングOS向けの電力出力調整プロファイル。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MiningPowerProfile {
    /// 電力出力(0〜100%)。100%は休止無しでディスパッチし続ける
    /// (ソフトウェア側で追加の休止は入れない、GPU本来の稼働率のまま)。
    /// 0%はディスパッチを完全に止める。
    pub power_percent: u8,
}

impl Default for MiningPowerProfile {
    fn default() -> Self {
        Self { power_percent: 100 }
    }
}

impl MiningPowerProfile {
    pub fn new(power_percent: u8) -> Self {
        Self { power_percent: power_percent.min(100) }
    }

    /// 1回のディスパッチ後に挟むべき休止時間を計算する。
    ///
    /// 設計: `power_percent`をディスパッチ稼働時間の比率とみなし、
    /// `dispatch_duration`(直近1回のディスパッチ+同期にかかった実測時間)
    /// を基準に、残りの`(100-power_percent)`%相当の時間だけ休止する
    /// (`sleep_duration = dispatch_duration * (100-power_percent) /
    /// power_percent`)——単純な固定スリープではなく、実測ディスパッチ
    /// 時間に比例させることで、マシン性能に関わらず狙った比率のデューティ
    /// サイクルに近づける設計。`power_percent == 0`は稼働率0%として
    /// `Duration::MAX`を返す(呼び出し側はこれを「停止」の合図として扱う
    /// こと)。`power_percent >= 100`は休止無し(`Duration::ZERO`)。
    pub fn sleep_after_dispatch(&self, dispatch_duration: Duration) -> Duration {
        if self.power_percent == 0 {
            return Duration::MAX;
        }
        if self.power_percent >= 100 {
            return Duration::ZERO;
        }
        let power = self.power_percent as f64;
        let idle_ratio = (100.0 - power) / power;
        Duration::from_secs_f64(dispatch_duration.as_secs_f64() * idle_ratio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_power_means_no_sleep() {
        let profile = MiningPowerProfile::new(100);
        assert_eq!(profile.sleep_after_dispatch(Duration::from_millis(10)), Duration::ZERO);
    }

    #[test]
    fn zero_power_means_stop() {
        let profile = MiningPowerProfile::new(0);
        assert_eq!(profile.sleep_after_dispatch(Duration::from_millis(10)), Duration::MAX);
    }

    #[test]
    fn half_power_sleeps_roughly_as_long_as_it_dispatched() {
        let profile = MiningPowerProfile::new(50);
        let sleep = profile.sleep_after_dispatch(Duration::from_millis(10));
        assert!((sleep.as_secs_f64() - 0.010).abs() < 1e-6, "expected ~10ms sleep, got {sleep:?}");
    }

    #[test]
    fn low_power_sleeps_much_longer_than_it_dispatched() {
        let profile = MiningPowerProfile::new(10);
        let sleep = profile.sleep_after_dispatch(Duration::from_millis(10));
        // 10%稼働 -> 稼働:休止 = 1:9 -> 90msの休止が期待値
        assert!((sleep.as_secs_f64() - 0.090).abs() < 1e-6, "expected ~90ms sleep, got {sleep:?}");
    }

    #[test]
    fn power_percent_above_100_is_clamped() {
        let profile = MiningPowerProfile::new(255);
        assert_eq!(profile.power_percent, 100);
    }
}
