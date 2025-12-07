//! SSL 4.0 Mobile Module
//!
//! Enables SSL programs to run as native iOS and Android apps.

pub mod ios;
pub mod android;
pub mod ui;

/// Mobile platform target
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MobilePlatform {
    /// iOS (iPhone, iPad)
    IOS,
    /// Android
    Android,
    /// Both platforms
    Both,
}

/// Mobile build configuration
#[derive(Debug, Clone)]
pub struct MobileConfig {
    /// Target platform
    pub platform: MobilePlatform,
    /// Application name
    pub app_name: String,
    /// Bundle identifier (e.g., com.example.app)
    pub bundle_id: String,
    /// App version
    pub version: String,
    /// Minimum iOS version (if targeting iOS)
    pub min_ios_version: String,
    /// Minimum Android SDK (if targeting Android)
    pub min_android_sdk: u32,
    /// Enable release optimizations
    pub release: bool,
}

impl Default for MobileConfig {
    fn default() -> Self {
        Self {
            platform: MobilePlatform::Both,
            app_name: "SSL App".to_string(),
            bundle_id: "org.sslang.app".to_string(),
            version: "1.0.0".to_string(),
            min_ios_version: "14.0".to_string(),
            min_android_sdk: 24,
            release: false,
        }
    }
}

impl MobileConfig {
    /// Create iOS-only configuration
    pub fn ios(app_name: &str, bundle_id: &str) -> Self {
        Self {
            platform: MobilePlatform::IOS,
            app_name: app_name.to_string(),
            bundle_id: bundle_id.to_string(),
            ..Default::default()
        }
    }
    
    /// Create Android-only configuration
    pub fn android(app_name: &str, bundle_id: &str) -> Self {
        Self {
            platform: MobilePlatform::Android,
            app_name: app_name.to_string(),
            bundle_id: bundle_id.to_string(),
            ..Default::default()
        }
    }
    
    /// Set minimum iOS version
    pub fn with_min_ios(mut self, version: &str) -> Self {
        self.min_ios_version = version.to_string();
        self
    }
    
    /// Set minimum Android SDK
    pub fn with_min_android_sdk(mut self, sdk: u32) -> Self {
        self.min_android_sdk = sdk;
        self
    }
    
    /// Enable release mode
    pub fn release(mut self) -> Self {
        self.release = true;
        self
    }
}

/// Result of mobile app build
#[derive(Debug)]
pub struct MobileBuildResult {
    /// Paths to generated files
    pub output_files: Vec<String>,
    /// Build log
    pub log: String,
    /// Success status
    pub success: bool,
    /// Platform-specific output
    pub platform_output: PlatformOutput,
}

/// Platform-specific build output
#[derive(Debug)]
pub enum PlatformOutput {
    /// iOS build output
    IOS {
        /// Path to .app bundle
        app_path: Option<String>,
        /// Path to Xcode project
        xcodeproj_path: Option<String>,
    },
    /// Android build output
    Android {
        /// Path to APK file
        apk_path: Option<String>,
        /// Path to AAB file
        aab_path: Option<String>,
        /// Path to Gradle project
        gradle_path: Option<String>,
    },
}

/// Mobile app entry configuration
#[derive(Debug, Clone)]
pub struct MobileAppInfo {
    /// App display name
    pub display_name: String,
    /// App icon path
    pub icon_path: Option<String>,
    /// Splash screen path
    pub splash_path: Option<String>,
    /// Required permissions
    pub permissions: Vec<MobilePermission>,
    /// App orientation
    pub orientation: AppOrientation,
}

/// Mobile app permissions
#[derive(Debug, Clone, PartialEq)]
pub enum MobilePermission {
    Camera,
    Microphone,
    Location,
    LocationAlways,
    Photos,
    Contacts,
    Calendar,
    Bluetooth,
    Network,
    Storage,
}

impl MobilePermission {
    /// Get iOS permission key
    pub fn ios_key(&self) -> &'static str {
        match self {
            Self::Camera => "NSCameraUsageDescription",
            Self::Microphone => "NSMicrophoneUsageDescription",
            Self::Location => "NSLocationWhenInUseUsageDescription",
            Self::LocationAlways => "NSLocationAlwaysUsageDescription",
            Self::Photos => "NSPhotoLibraryUsageDescription",
            Self::Contacts => "NSContactsUsageDescription",
            Self::Calendar => "NSCalendarsUsageDescription",
            Self::Bluetooth => "NSBluetoothAlwaysUsageDescription",
            Self::Network => "NSLocalNetworkUsageDescription",
            Self::Storage => "NSDocumentsFolderUsageDescription",
        }
    }
    
    /// Get Android permission string
    pub fn android_permission(&self) -> &'static str {
        match self {
            Self::Camera => "android.permission.CAMERA",
            Self::Microphone => "android.permission.RECORD_AUDIO",
            Self::Location => "android.permission.ACCESS_FINE_LOCATION",
            Self::LocationAlways => "android.permission.ACCESS_BACKGROUND_LOCATION",
            Self::Photos => "android.permission.READ_EXTERNAL_STORAGE",
            Self::Contacts => "android.permission.READ_CONTACTS",
            Self::Calendar => "android.permission.READ_CALENDAR",
            Self::Bluetooth => "android.permission.BLUETOOTH_CONNECT",
            Self::Network => "android.permission.INTERNET",
            Self::Storage => "android.permission.WRITE_EXTERNAL_STORAGE",
        }
    }
}

/// App orientation configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppOrientation {
    Portrait,
    Landscape,
    Both,
}

impl Default for MobileAppInfo {
    fn default() -> Self {
        Self {
            display_name: "SSL App".to_string(),
            icon_path: None,
            splash_path: None,
            permissions: vec![MobilePermission::Network],
            orientation: AppOrientation::Both,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mobile_config_default() {
        let config = MobileConfig::default();
        assert_eq!(config.platform, MobilePlatform::Both);
        assert_eq!(config.min_android_sdk, 24);
    }
    
    #[test]
    fn test_ios_config() {
        let config = MobileConfig::ios("MyApp", "com.example.myapp");
        assert_eq!(config.platform, MobilePlatform::IOS);
        assert_eq!(config.bundle_id, "com.example.myapp");
    }
    
    #[test]
    fn test_permission_keys() {
        assert_eq!(MobilePermission::Camera.ios_key(), "NSCameraUsageDescription");
        assert_eq!(MobilePermission::Camera.android_permission(), "android.permission.CAMERA");
    }
}
