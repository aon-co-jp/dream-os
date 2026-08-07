plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "tokyo.runo.dreamos"
    compileSdk = 35

    defaultConfig {
        applicationId = "tokyo.runo.dreamos"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        // 実機はarm64-v8a(このPoCで検証したMoto G53Y 5G等)のみを対象とする。
        // Vulkan compute自体は`opencuda-vulkan`側のreal-vulkan featureで
        // arm64-v8a向けにのみクロスビルド・実機検証済み(dream-os/CLAUDE.md参照)。
        ndk {
            abiFilters += listOf("arm64-v8a")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        viewBinding = false
    }

    // open-web-server/androidと同じ理由(ProcessBuilderへ実ファイルパスを渡す
    // 必要があるため、nativeLibraryDir配下への旧来型展開を強制する)。
    packaging {
        jniLibs {
            useLegacyPackaging = true
        }
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.9.0")
}
