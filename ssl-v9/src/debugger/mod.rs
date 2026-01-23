// Time-Travel Debugging Module
// Enables stepping backward/forward through program execution

pub mod snapshots;
pub mod timeline;
pub mod ui;

pub use snapshots::{StateSnapshot, SnapshotManager};
pub use timeline::{Timeline, TimelineNavigator};
pub use ui::{DebugCommands, DebugUI};
