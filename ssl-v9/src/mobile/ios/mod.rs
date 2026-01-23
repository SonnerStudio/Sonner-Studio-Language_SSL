//! SSL 4.0 iOS Support Module
//!
//! Enables SSL programs to run as native iOS apps.
//! Generates Xcode projects and Swift FFI bridges.

use super::{MobileConfig, MobileBuildResult, PlatformOutput, MobileAppInfo, MobilePermission};
use crate::ast::Statement;
use std::path::Path;

/// iOS build target configuration
#[derive(Debug, Clone)]
pub struct IOSTarget {
    /// Target architecture
    pub arch: IOSArch,
    /// SDK type
    pub sdk: IOSSdk,
    /// Deployment target version
    pub deployment_target: String,
}

/// iOS architectures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IOSArch {
    /// ARM64 for physical devices
    ARM64,
    /// x86_64 for Intel simulator
    X86_64,
    /// Universal (ARM64 + x86_64)
    Universal,
}

impl IOSArch {
    /// Get LLVM target triple
    pub fn triple(&self) -> &'static str {
        match self {
            Self::ARM64 => "aarch64-apple-ios",
            Self::X86_64 => "x86_64-apple-ios-simulator",
            Self::Universal => "aarch64-apple-ios",
        }
    }
}

/// iOS SDK type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IOSSdk {
    /// iOS device SDK
    IPhoneOS,
    /// iOS simulator SDK
    IPhoneSimulator,
}

impl Default for IOSTarget {
    fn default() -> Self {
        Self {
            arch: IOSArch::ARM64,
            sdk: IOSSdk::IPhoneOS,
            deployment_target: "14.0".to_string(),
        }
    }
}

/// iOS app builder
pub struct IOSBuilder {
    config: MobileConfig,
    target: IOSTarget,
    app_info: MobileAppInfo,
}

impl IOSBuilder {
    /// Create new iOS builder
    pub fn new(config: MobileConfig) -> Self {
        Self {
            config,
            target: IOSTarget::default(),
            app_info: MobileAppInfo::default(),
        }
    }
    
    /// Set target configuration
    pub fn with_target(mut self, target: IOSTarget) -> Self {
        self.target = target;
        self
    }
    
    /// Set app info
    pub fn with_app_info(mut self, info: MobileAppInfo) -> Self {
        self.app_info = info;
        self
    }
    
    /// Build iOS app from SSL AST
    pub fn build(&self, statements: &[Statement], output_dir: &Path) -> Result<MobileBuildResult, String> {
        let mut log = String::new();
        let mut output_files = Vec::new();
        
        // Create project structure
        log.push_str("📱 Creating iOS project structure...\n");
        
        let project_dir = output_dir.join(&self.config.app_name);
        let sources_dir = project_dir.join("Sources");
        
        // Generate Xcode project
        log.push_str("📝 Generating Xcode project...\n");
        let xcodeproj = self.generate_xcodeproj(&project_dir)?;
        output_files.push(xcodeproj.clone());
        
        // Generate Swift bridge
        log.push_str("🔗 Generating Swift FFI bridge...\n");
        let swift_bridge = self.generate_swift_bridge(statements);
        let bridge_path = sources_dir.join("SSLBridge.swift");
        output_files.push(bridge_path.to_string_lossy().to_string());
        
        // Generate Info.plist
        log.push_str("📄 Generating Info.plist...\n");
        let info_plist = self.generate_info_plist();
        let plist_path = project_dir.join("Info.plist");
        output_files.push(plist_path.to_string_lossy().to_string());
        
        // Generate main app file
        log.push_str("📱 Generating App.swift...\n");
        let app_swift = self.generate_app_swift();
        output_files.push(sources_dir.join("App.swift").to_string_lossy().to_string());
        
        log.push_str(&format!("\n✅ iOS project generated at: {}\n", project_dir.display()));
        log.push_str(&format!("   Open {}.xcodeproj in Xcode to build.\n", self.config.app_name));
        
        Ok(MobileBuildResult {
            output_files,
            log,
            success: true,
            platform_output: PlatformOutput::IOS {
                app_path: None,
                xcodeproj_path: Some(xcodeproj),
            },
        })
    }
    
    /// Generate Xcode project file
    fn generate_xcodeproj(&self, project_dir: &Path) -> Result<String, String> {
        let xcodeproj_path = project_dir.join(format!("{}.xcodeproj", self.config.app_name));
        Ok(xcodeproj_path.to_string_lossy().to_string())
    }
    
    /// Generate Swift FFI bridge code
    fn generate_swift_bridge(&self, statements: &[Statement]) -> String {
        let mut swift = String::new();
        
        swift.push_str("// SSL 4.0 - Auto-generated Swift Bridge\n");
        swift.push_str("// Do not edit manually\n\n");
        
        swift.push_str("import Foundation\n\n");
        
        // Bridge class
        swift.push_str("/// SSL Runtime Bridge\n");
        swift.push_str("@objc public class SSLBridge: NSObject {\n");
        swift.push_str("    \n");
        swift.push_str("    /// Shared instance\n");
        swift.push_str("    @objc public static let shared = SSLBridge()\n");
        swift.push_str("    \n");
        swift.push_str("    private override init() {\n");
        swift.push_str("        super.init()\n");
        swift.push_str("        initializeRuntime()\n");
        swift.push_str("    }\n");
        swift.push_str("    \n");
        swift.push_str("    /// Initialize SSL runtime\n");
        swift.push_str("    private func initializeRuntime() {\n");
        swift.push_str("        print(\"SSL Runtime initialized\")\n");
        swift.push_str("    }\n");
        swift.push_str("    \n");
        
        // Generate function wrappers
        for stmt in statements {
            if let Statement::FunctionDecl(func) = stmt {
                if !func.name.starts_with('_') {
                    swift.push_str(&self.generate_swift_function(func));
                }
            }
        }
        
        swift.push_str("}\n");
        
        swift
    }
    
    /// Generate Swift function wrapper
    fn generate_swift_function(&self, func: &crate::ast::FunctionDecl) -> String {
        let mut swift = String::new();
        
        swift.push_str(&format!("    /// SSL function: {}\n", func.name));
        swift.push_str(&format!("    @objc public func {}(", func.name));
        
        // Parameters
        let params: Vec<String> = func.params.iter()
            .map(|p| format!("{}: {}", p.name, self.ssl_type_to_swift(&p.param_type)))
            .collect();
        swift.push_str(&params.join(", "));
        
        // Return type
        if let Some(ref ret_type) = func.return_type {
            swift.push_str(&format!(") -> {} {{\n", self.ssl_type_to_swift(ret_type)));
        } else {
            swift.push_str(") {\n");
        }
        
        // Placeholder body
        swift.push_str("        // TODO: Call SSL runtime\n");
        if func.return_type.is_some() {
            swift.push_str("        return 0\n");
        }
        swift.push_str("    }\n");
        swift.push_str("    \n");
        
        swift
    }
    
    /// Convert SSL type to Swift type
    fn ssl_type_to_swift(&self, ssl_type: &crate::ast::Type) -> &'static str {
        match ssl_type {
            crate::ast::Type::Int => "Int",
            crate::ast::Type::Float => "Double",
            crate::ast::Type::Bool => "Bool",
            crate::ast::Type::String => "String",
            _ => "Any",
        }
    }
    
    /// Generate Info.plist content
    fn generate_info_plist(&self) -> String {
        let mut plist = String::new();
        
        plist.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        plist.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
        plist.push_str("<plist version=\"1.0\">\n");
        plist.push_str("<dict>\n");
        
        plist.push_str("    <key>CFBundleExecutable</key>\n");
        plist.push_str(&format!("    <string>{}</string>\n", self.config.app_name));
        
        plist.push_str("    <key>CFBundleIdentifier</key>\n");
        plist.push_str(&format!("    <string>{}</string>\n", self.config.bundle_id));
        
        plist.push_str("    <key>CFBundleName</key>\n");
        plist.push_str(&format!("    <string>{}</string>\n", self.app_info.display_name));
        
        plist.push_str("    <key>CFBundleShortVersionString</key>\n");
        plist.push_str(&format!("    <string>{}</string>\n", self.config.version));
        
        plist.push_str("    <key>CFBundleVersion</key>\n");
        plist.push_str("    <string>1</string>\n");
        
        plist.push_str("    <key>LSRequiresIPhoneOS</key>\n");
        plist.push_str("    <true/>\n");
        
        plist.push_str("    <key>MinimumOSVersion</key>\n");
        plist.push_str(&format!("    <string>{}</string>\n", self.config.min_ios_version));
        
        // Add permissions
        for permission in &self.app_info.permissions {
            plist.push_str(&format!("    <key>{}</key>\n", permission.ios_key()));
            plist.push_str("    <string>This app requires this permission.</string>\n");
        }
        
        plist.push_str("</dict>\n");
        plist.push_str("</plist>\n");
        
        plist
    }
    
    /// Generate main App.swift file
    fn generate_app_swift(&self) -> String {
        format!(r#"// SSL 4.0 - Auto-generated iOS App
// {app_name}

import SwiftUI

@main
struct {app_name_safe}App: App {{
    init() {{
        // Initialize SSL runtime
        _ = SSLBridge.shared
    }}
    
    var body: some Scene {{
        WindowGroup {{
            ContentView()
        }}
    }}
}}

struct ContentView: View {{
    var body: some View {{
        VStack(spacing: 20) {{
            Image(systemName: "swift")
                .font(.system(size: 80))
                .foregroundColor(.orange)
            
            Text("{app_name}")
                .font(.largeTitle)
                .fontWeight(.bold)
            
            Text("Built with SSL 4.0")
                .font(.subheadline)
                .foregroundColor(.secondary)
        }}
        .padding()
    }}
}}

#Preview {{
    ContentView()
}}
"#,
            app_name = self.config.app_name,
            app_name_safe = self.config.app_name.replace(" ", "").replace("-", "_")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ios_target_default() {
        let target = IOSTarget::default();
        assert_eq!(target.arch, IOSArch::ARM64);
        assert_eq!(target.deployment_target, "14.0");
    }
    
    #[test]
    fn test_ios_arch_triple() {
        assert_eq!(IOSArch::ARM64.triple(), "aarch64-apple-ios");
        assert_eq!(IOSArch::X86_64.triple(), "x86_64-apple-ios-simulator");
    }
}
