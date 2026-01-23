# SSL Mobile Guide

## Native Mobile App Development

SSL v4.0 supports building native iOS and Android apps from SSL code.

## Quick Start

### iOS

```bash
# Build iOS app
ssl build --target ios src/main.ssl

# Output: target/ios/
# - MyApp.xcodeproj
# - Sources/
# - Info.plist
```

### Android

```bash
# Build Android app
ssl build --target android src/main.ssl

# Output: target/android/
# - app/build.gradle
# - app/src/main/kotlin/
# - AndroidManifest.xml
```

## Example App

### SSL Source (main.ssl)

```ssl
import mobile { App, Screen, View, Text, Button }

struct Counter {
    count: Int
}

impl App for Counter {
    fn render(self) -> View {
        Screen {
            Text("Count: ${self.count}")
            
            Button("Increment") {
                self.count += 1
            }
            
            Button("Decrement") {
                self.count -= 1
            }
        }
    }
}

fn main() {
    Counter { count: 0 }.run()
}
```

## UI Components

### Basic Components

```ssl
// Text
Text("Hello, World!")
Text("Styled").bold().color("#FF0000")

// Button
Button("Tap Me") {
    print("Tapped!")
}

// Image
Image.network("https://example.com/image.png")
Image.asset("icon.png")

// Input
TextField("Enter name", on_change: |text| {
    name = text
})
```

### Layout

```ssl
// Vertical stack
VStack {
    Text("Top")
    Text("Middle")
    Text("Bottom")
}

// Horizontal stack
HStack {
    Text("Left")
    Spacer()
    Text("Right")
}

// Grid
Grid(columns: 3) {
    for item in items {
        ItemView(item)
    }
}

// List
List(items) { item ->
    ListItem(item.name, item.description)
}
```

### Navigation

```ssl
NavigationView {
    List(screens) { screen ->
        NavigationLink(screen.title, to: screen)
    }
}

// Tab bar
TabView {
    HomeScreen().tab("Home", icon: "house")
    SearchScreen().tab("Search", icon: "magnifyingglass")
    ProfileScreen().tab("Profile", icon: "person")
}
```

## Platform APIs

### iOS (SwiftUI)

```ssl
@ios
fn use_haptics() {
    // Generates SwiftUI code
    UIImpactFeedbackGenerator(style: .medium).impactOccurred()
}

@ios
fn open_settings() {
    // Opens iOS Settings
    UIApplication.shared.open(URL(string: UIApplication.openSettingsURLString)!)
}
```

### Android (Jetpack Compose)

```ssl
@android
fn show_toast(message: String) {
    // Generates Kotlin code
    Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
}

@android
fn vibrate() {
    val vibrator = context.getSystemService(Context.VIBRATOR_SERVICE) as Vibrator
    vibrator.vibrate(100)
}
```

## Cross-Platform Code

```ssl
fn get_platform() -> String {
    #if ios
        "iOS"
    #elif android
        "Android"
    #else
        "Unknown"
    #endif
}

// Platform-specific behavior
fn show_alert(message: String) {
    #if ios
        ios_alert(message)
    #elif android
        android_toast(message)
    #endif
}
```

## Project Configuration

### ssl.toml

```toml
[package]
name = "my-app"
version = "1.0.0"

[mobile]
bundle_id = "com.example.myapp"
display_name = "My App"
min_ios_version = "14.0"
min_android_sdk = 26

[mobile.ios]
team_id = "ABCDE12345"
signing_certificate = "Apple Development"

[mobile.android]
keystore = "release.keystore"
key_alias = "release"
```

## Building & Running

### iOS

```bash
# Build for simulator
ssl build --target ios --simulator

# Build for device
ssl build --target ios --device

# Open in Xcode
open target/ios/MyApp.xcodeproj
```

### Android

```bash
# Build debug APK
ssl build --target android

# Build release APK
ssl build --target android --release

# Install on device
adb install target/android/app/build/outputs/apk/release/app-release.apk
```

## Requirements

### iOS
- macOS with Xcode 14+
- Apple Developer account (for device deployment)
- CocoaPods or SPM for dependencies

### Android
- Android Studio or Android SDK
- JDK 17+
- Android NDK (for native code)

## Debugging

```bash
# iOS - open in Xcode
ssl build --target ios
open target/ios/MyApp.xcodeproj

# Android - connect to Android Studio
ssl build --target android
# Import target/android/ into Android Studio
```

## See Also

- [WASM Guide](WASM_GUIDE.md) - Web deployment
- [Edge Deployment](V4.0_FEATURES.md#3-edgeserverless-deployment)
- [Examples](../examples/mobile_demo.ssl)

---

*SSL v4.0.0 Mobile Guide - December 2024*
