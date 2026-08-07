package tokyo.runo.dreamos

import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

/**
 * DreamOS Android連携PoC(2026-08-07)。
 *
 * `open-web-server`/`open-easy-web`のAndroid実装(`ProcessBuilder`で
 * クロスコンパイル済みネイティブ実行ファイルを起動する方式、JNIではなく
 * サブプロセス方式)をそのまま踏襲する。本体は`dream-os-kernel`の
 * `dream_os_status`example(`cargo ndk -t aarch64-linux-android`で
 * クロスビルド、`jniLibs/arm64-v8a/libdreamosstatus.so`として同梱)——
 * open-cuda(`opencuda-vulkan`)経由の実Vulkan Compute dispatchと
 * open-directx製DXBC->SPIR-V翻訳ブリッジを実機のGPU上で実行し、結果を
 * 標準出力へ表示する自己完結バイナリ。
 *
 * このActivity自体はDreamOSのロジックを一切実装しない——ネイティブ
 * バイナリの起動・標準出力の読み取り・画面表示のみを担う薄いシェル。
 */
class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val txtOutput = findViewById<TextView>(R.id.txtOutput)
        val btnRun = findViewById<Button>(R.id.btnRun)

        btnRun.setOnClickListener {
            txtOutput.text = "実行中...\n"
            CoroutineScope(Dispatchers.Main).launch {
                val result = withContext(Dispatchers.IO) { runDreamOsStatus() }
                txtOutput.text = result
            }
        }
    }

    private fun runDreamOsStatus(): String {
        val binaryPath = File(applicationInfo.nativeLibraryDir, "libdreamosstatus.so")
        if (!binaryPath.exists()) {
            return "エラー: ネイティブバイナリが見つかりません: $binaryPath"
        }
        return try {
            val process = ProcessBuilder(binaryPath.absolutePath)
                .redirectErrorStream(true)
                .start()
            val output = BufferedReader(InputStreamReader(process.inputStream)).readText()
            val exitCode = process.waitFor()
            "binary: ${binaryPath.absolutePath}\nexit code: $exitCode\n\n$output"
        } catch (e: Exception) {
            "実行失敗: ${e.message}"
        }
    }
}
