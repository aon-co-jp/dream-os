// DreamOS Android連携PoC(2026-08-07): open-web-server/android(ProcessBuilder方式)
// と同じ設計を踏襲した最小Androidシェル。crates/dream-os-kernel/examples/
// dream_os_status.rs をcargo ndkでクロスビルドし、jniLibsへ配置した実行ファイルを
// MainActivityがProcessBuilderで起動し、標準出力(Vulkanデバイス名・
// open-directx DXBC->SPIR-V dispatch結果)を画面に表示する。
plugins {
    id("com.android.application") version "8.7.2" apply false
    id("org.jetbrains.kotlin.android") version "2.0.21" apply false
}
