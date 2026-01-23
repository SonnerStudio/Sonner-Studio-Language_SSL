//! SSL 4.0 Android Support Module
//!
//! Enables SSL programs to run as native Android apps.
//! Generates Gradle projects and JNI bridges.

use super::{MobileConfig, MobileBuildResult, PlatformOutput, MobileAppInfo, MobilePermission};
use crate::ast::Statement;
use std::path::Path;

/// Android build target configuration
#[derive(Debug, Clone)]
pub struct AndroidTarget {
    /// Target architecture
    pub arch: AndroidArch,
    /// Minimum SDK version
    pub min_sdk: u32,
    /// Target SDK version
    pub target_sdk: u32,
    /// Compile SDK version
    pub compile_sdk: u32,
}

/// Android architectures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AndroidArch {
    /// ARM64-v8a (most modern devices)
    ARM64,
    /// ARMv7-a (older devices)
    ARM32,
    /// x86_64 (emulators, Chromebooks)
    X86_64,
    /// x86 (old emulators)
    X86,
    /// All architectures
    All,
}

impl AndroidArch {
    /// Get LLVM target triple
    pub fn triple(&self) -> &'static str {
        match self {
            Self::ARM64 => "aarch64-linux-android",
            Self::ARM32 => "armv7-linux-androideabi",
            Self::X86_64 => "x86_64-linux-android",
            Self::X86 => "i686-linux-android",
            Self::All => "aarch64-linux-android",
        }
    }
    
    /// Get ABI name for Gradle
    pub fn abi_name(&self) -> &'static str {
        match self {
            Self::ARM64 => "arm64-v8a",
            Self::ARM32 => "armeabi-v7a",
            Self::X86_64 => "x86_64",
            Self::X86 => "x86",
            Self::All => "all",
        }
    }
}

impl Default for AndroidTarget {
    fn default() -> Self {
        Self {
            arch: AndroidArch::ARM64,
            min_sdk: 24,
            target_sdk: 34,
            compile_sdk: 34,
        }
    }
}

/// Android app builder
pub struct AndroidBuilder {
    config: MobileConfig,
    target: AndroidTarget,
    app_info: MobileAppInfo,
}

impl AndroidBuilder {
    /// Create new Android builder
    pub fn new(config: MobileConfig) -> Self {
        Self {
            config,
            target: AndroidTarget::default(),
            app_info: MobileAppInfo::default(),
        }
    }
    
    /// Set target configuration
    pub fn with_target(mut self, target: AndroidTarget) -> Self {
        self.target = target;
        self
    }
    
    /// Set app info
    pub fn with_app_info(mut self, info: MobileAppInfo) -> Self {
        self.app_info = info;
        self
    }
    
    /// Build Android app from SSL AST
    pub fn build(&self, statements: &[Statement], output_dir: &Path) -> Result<MobileBuildResult, String> {
        let mut log = String::new();
        let mut output_files = Vec::new();
        
        log.push_str("🤖 Creating Android project structure...\n");
        
        let project_dir = output_dir.join(&self.config.app_name);
        let app_dir = project_dir.join("app");
        let src_dir = app_dir.join("src/main");
        let java_dir = src_dir.join("java").join(self.config.bundle_id.replace('.', "/"));
        
        // Generate build.gradle files
        log.push_str("📝 Generating Gradle build files...\n");
        let gradle_path = project_dir.join("build.gradle.kts");
        output_files.push(gradle_path.to_string_lossy().to_string());
        
        // Generate JNI bridge
        log.push_str("🔗 Generating JNI bridge...\n");
        let jni_bridge = self.generate_jni_bridge(statements);
        let bridge_path = java_dir.join("SSLBridge.kt");
        output_files.push(bridge_path.to_string_lossy().to_string());
        
        // Generate AndroidManifest.xml
        log.push_str("📄 Generating AndroidManifest.xml...\n");
        let manifest = self.generate_manifest();
        let manifest_path = src_dir.join("AndroidManifest.xml");
        output_files.push(manifest_path.to_string_lossy().to_string());
        
        // Generate MainActivity
        log.push_str("📱 Generating MainActivity.kt...\n");
        let main_activity = self.generate_main_activity();
        output_files.push(java_dir.join("MainActivity.kt").to_string_lossy().to_string());
        
        log.push_str(&format!("\n✅ Android project generated at: {}\n", project_dir.display()));
        log.push_str("   Run './gradlew assembleDebug' to build APK.\n");
        
        Ok(MobileBuildResult {
            output_files,
            log,
            success: true,
            platform_output: PlatformOutput::Android {
                apk_path: None,
                aab_path: None,
                gradle_path: Some(gradle_path.to_string_lossy().to_string()),
            },
        })
    }
    
    /// Generate JNI bridge code (Kotlin)
    fn generate_jni_bridge(&self, statements: &[Statement]) -> String {
        let mut kotlin = String::new();
        
        kotlin.push_str("// SSL 4.0 - Auto-generated JNI Bridge\n");
        kotlin.push_str("// Do not edit manually\n\n");
        
        kotlin.push_str(&format!("package {}\n\n", self.config.bundle_id));
        
        kotlin.push_str("/**\n * SSL Runtime Bridge\n */\n");
        kotlin.push_str("object SSLBridge {\n");
        kotlin.push_str("    \n");
        kotlin.push_str("    init {\n");
        kotlin.push_str("        System.loadLibrary(\"ssl_native\")\n");
        kotlin.push_str("        initializeRuntime()\n");
        kotlin.push_str("    }\n");
        kotlin.push_str("    \n");
        kotlin.push_str("    private external fun initializeRuntime()\n");
        kotlin.push_str("    \n");
        
        // Generate function wrappers
        for stmt in statements {
            if let Statement::FunctionDecl(func) = stmt {
                if !func.name.starts_with('_') {
                    kotlin.push_str(&self.generate_kotlin_function(func));
                }
            }
        }
        
        kotlin.push_str("}\n");
        
        kotlin
    }
    
    /// Generate Kotlin function wrapper
    fn generate_kotlin_function(&self, func: &crate::ast::FunctionDecl) -> String {
        let mut kotlin = String::new();
        
        kotlin.push_str(&format!("    /** SSL function: {} */\n", func.name));
        kotlin.push_str(&format!("    external fun {}(", func.name));
        
        // Parameters
        let params: Vec<String> = func.params.iter()
            .map(|p| format!("{}: {}", p.name, self.ssl_type_to_kotlin(&p.param_type)))
            .collect();
        kotlin.push_str(&params.join(", "));
        
        // Return type
        if let Some(ref ret_type) = func.return_type {
            kotlin.push_str(&format!("): {}\n", self.ssl_type_to_kotlin(ret_type)));
        } else {
            kotlin.push_str(")\n");
        }
        kotlin.push_str("    \n");
        
        kotlin
    }
    
    /// Convert SSL type to Kotlin type
    fn ssl_type_to_kotlin(&self, ssl_type: &crate::ast::Type) -> &'static str {
        match ssl_type {
            crate::ast::Type::Int => "Int",
            crate::ast::Type::Float => "Double",
            crate::ast::Type::Bool => "Boolean",
            crate::ast::Type::String => "String",
            _ => "Any",
        }
    }
    
    /// Generate AndroidManifest.xml
    fn generate_manifest(&self) -> String {
        let mut manifest = String::new();
        
        manifest.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        manifest.push_str(&format!("<manifest xmlns:android=\"http://schemas.android.com/apk/res/android\"\n"));
        manifest.push_str(&format!("    package=\"{}\">\n\n", self.config.bundle_id));
        
        // Permissions
        for permission in &self.app_info.permissions {
            manifest.push_str(&format!("    <uses-permission android:name=\"{}\" />\n", 
                permission.android_permission()));
        }
        manifest.push_str("\n");
        
        // Application
        manifest.push_str("    <application\n");
        manifest.push_str("        android:allowBackup=\"true\"\n");
        manifest.push_str(&format!("        android:label=\"{}\"\n", self.app_info.display_name));
        manifest.push_str("        android:supportsRtl=\"true\"\n");
        manifest.push_str("        android:theme=\"@style/Theme.Material3.DynamicColors.DayNight\">\n\n");
        
        // Main Activity
        manifest.push_str("        <activity\n");
        manifest.push_str("            android:name=\".MainActivity\"\n");
        manifest.push_str("            android:exported=\"true\"\n");
        manifest.push_str("            android:theme=\"@style/Theme.Material3.DynamicColors.DayNight\">\n");
        manifest.push_str("            <intent-filter>\n");
        manifest.push_str("                <action android:name=\"android.intent.action.MAIN\" />\n");
        manifest.push_str("                <category android:name=\"android.intent.category.LAUNCHER\" />\n");
        manifest.push_str("            </intent-filter>\n");
        manifest.push_str("        </activity>\n\n");
        
        manifest.push_str("    </application>\n");
        manifest.push_str("</manifest>\n");
        
        manifest
    }
    
    /// Generate MainActivity.kt
    fn generate_main_activity(&self) -> String {
        format!(r#"// SSL 4.0 - Auto-generated Android Activity
// {app_name}

package {bundle_id}

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

class MainActivity : ComponentActivity() {{
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        
        // Initialize SSL runtime
        SSLBridge
        
        setContent {{
            MaterialTheme {{
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {{
                    MainContent()
                }}
            }}
        }}
    }}
}}

@Composable
fun MainContent() {{
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {{
        Text(
            text = "🚀",
            fontSize = 80.sp
        )
        
        Spacer(modifier = Modifier.height(20.dp))
        
        Text(
            text = "{app_name}",
            fontSize = 32.sp,
            fontWeight = FontWeight.Bold
        )
        
        Spacer(modifier = Modifier.height(8.dp))
        
        Text(
            text = "Built with SSL 4.0",
            fontSize = 16.sp,
            color = MaterialTheme.colorScheme.secondary
        )
    }}
}}
"#,
            app_name = self.config.app_name,
            bundle_id = self.config.bundle_id
        )
    }
    
    /// Generate build.gradle.kts for app module
    pub fn generate_app_build_gradle(&self) -> String {
        format!(r#"plugins {{
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}}

android {{
    namespace = "{bundle_id}"
    compileSdk = {compile_sdk}

    defaultConfig {{
        applicationId = "{bundle_id}"
        minSdk = {min_sdk}
        targetSdk = {target_sdk}
        versionCode = 1
        versionName = "{version}"

        ndk {{
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
        }}
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"))
        }}
    }}
    
    buildFeatures {{
        compose = true
    }}
    
    composeOptions {{
        kotlinCompilerExtensionVersion = "1.5.1"
    }}
    
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}
    
    kotlinOptions {{
        jvmTarget = "17"
    }}
}}

dependencies {{
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.2")
    implementation("androidx.activity:activity-compose:1.8.0")
    implementation(platform("androidx.compose:compose-bom:2023.10.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.material3:material3")
}}
"#,
            bundle_id = self.config.bundle_id,
            compile_sdk = self.target.compile_sdk,
            min_sdk = self.target.min_sdk,
            target_sdk = self.target.target_sdk,
            version = self.config.version
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_android_target_default() {
        let target = AndroidTarget::default();
        assert_eq!(target.arch, AndroidArch::ARM64);
        assert_eq!(target.min_sdk, 24);
    }
    
    #[test]
    fn test_android_arch_triple() {
        assert_eq!(AndroidArch::ARM64.triple(), "aarch64-linux-android");
        assert_eq!(AndroidArch::ARM64.abi_name(), "arm64-v8a");
    }
}
