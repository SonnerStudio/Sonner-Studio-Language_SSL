# SSL v9 Installation & Security Guide

## Requirements
- **Operating System**: Windows 10 or Windows 11 (64-bit)
- **Graphics**: WGPU compatible hardware (DirectX 12, Vulkan, or Metal)
- **Dependency**: [Microsoft Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

## Getting Started
1. **Download** the `ssl-v9.exe` from the GitHub Releases page.
2. **Move** the file to a folder of your choice (e.g., `C:\SSL\`).
3. **Run** the executable.
   - *Note: On the first run, Windows SmartScreen may show a warning because SSL is an independent executable. Click "More Info" -> "Run anyway".*

## Security Information
SSL v9 is built with a focus on privacy and performance:
- **No Telemetry**: We do not collect or send usage data.
- **No External Servers**: The hybrid runtime execution happens strictly on your local machine.
- **Asset Isolation**: The `asset://` protocol is used to securely load local files without standard browser vulnerabilities.

## Troubleshooting
- **Black Screen**: Ensure your graphics drivers are updated.
- **Missing DLL error**: Install the Visual C++ Redistributable (linked above).
- **Startup Crash**: Check the console for logs. SSL v9 now includes detailed error reporting for hardware compatibility.
