# Changelog - SSL v9.3.2.0 "Aurora"

## [9.3.2.0] - 2026-01-23

### Added
- **Maximized Start**: The application now automatically starts in maximized window mode for a better out-of-the-box experience.
- **Improved Error Messages**: Informative emoji-tagged console output for graphics initialization status.

### Fixed
- **Critical Crash Fix**: Resolved "Launch Crash" on high-end hardware (Dell Latitude 5520, Lenovo Legion) by implementing safe WGPU initialization.
- **Version Display**: Updated all internal version strings from 9.3.1 to 9.3.2.0 across the UI and CLI.
- **Graceful Degradation**: Added support for automated fallback to low-power graphics adapters if high-performance adapters fail.

### Security
- **Safe Graphics Handling**: Replaced multiple unsafe code paths in the renderer with robust error recovery.
- **Dependency Tracking**: Clearer disclosure of system requirements (Visual C++ Redistributable).
