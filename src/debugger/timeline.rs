use super::snapshots::{StateSnapshot, SnapshotManager};
use std::fmt;

/// Represents the execution timeline with navigation capabilities
pub struct Timeline {
    /// Snapshot manager
    manager: SnapshotManager,
    
    /// Source code lines for reference
    source_lines: Vec<String>,
    
    /// Whether recording is active
    recording: bool,
}

impl Timeline {
    /// Create a new timeline
    pub fn new(source: &str) -> Self {
        Timeline {
            manager: SnapshotManager::new(),
            source_lines: source.lines().map(String::from).collect(),
            recording: true,
        }
    }
    
    /// Enable/disable recording
    pub fn set_recording(&mut self, enabled: bool) {
        self.recording = enabled;
    }
    
    /// Check if recording is enabled
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    
    /// Get current position info
    pub fn current_position(&self) -> Option<TimelinePosition> {
        self.manager.current().map(|snap| {
            TimelinePosition {
                snapshot_id: snap.id,
                line_number: snap.line,
                source_line: self.source_lines.get(snap.line.saturating_sub(1))
                    .cloned()
                    .unwrap_or_default(),
                total_snapshots: self.manager.len(),
            }
        })
    }
    
    /// Get access to snapshot manager
    pub fn manager_mut(&mut self) -> &mut SnapshotManager {
        &mut self.manager
    }
    
    /// Get access to snapshot manager (immutable)
    pub fn manager(&self) -> &SnapshotManager {
        &self.manager
    }
}

/// Information about current timeline position
#[derive(Debug, Clone)]
pub struct TimelinePosition {
    pub snapshot_id: usize,
    pub line_number: usize,
    pub source_line: String,
    pub total_snapshots: usize,
}

impl fmt::Display for TimelinePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Snapshot #{}/{} | Line {}: {}",
            self.snapshot_id + 1,
            self.total_snapshots,
            self.line_number,
            self.source_line.trim()
        )
    }
}

/// Timeline navigator with high-level commands
pub struct TimelineNavigator<'a> {
    timeline: &'a mut Timeline,
}

impl<'a> TimelineNavigator<'a> {
    /// Create a new navigator
    pub fn new(timeline: &'a mut Timeline) -> Self {
        TimelineNavigator { timeline }
    }
    
    /// Step back one snapshot
    pub fn back(&mut self) -> Result<String, String> {
        let snapshot = self.timeline.manager.step_back()
            .ok_or("Already at the beginning")?;
        
        Ok(format!("⏪ Stepped back to line {}", snapshot.line))
    }
    
    /// Step forward one snapshot
    pub fn forward(&mut self) -> Result<String, String> {
        let snapshot = self.timeline.manager.step_forward()
            .ok_or("Already at the end")?;
        
        Ok(format!("⏩ Stepped forward to line {}", snapshot.line))
    }
    
    /// Rewind to specific line
    pub fn rewind_to(&mut self, line: usize) -> Result<String, String> {
        let snapshot = self.timeline.manager.goto_line(line)
            .ok_or(format!("No snapshot near line {}", line))?;
        
        Ok(format!("⏮️ Rewound to line {}", snapshot.line))
    }
    
    /// Jump to beginning
    pub fn rewind_to_start(&mut self) -> Result<String, String> {
        self.timeline.manager.goto(0)
            .ok_or("No snapshots recorded")?;
        
        Ok("⏮️ Rewound to start".to_string())
    }
    
    /// Jump to end (current execution point)
    pub fn fast_forward_to_end(&mut self) -> Result<String, String> {
        let last_idx = self.timeline.manager.len().saturating_sub(1);
        self.timeline.manager.goto(last_idx)
            .ok_or("No snapshots recorded")?;
        
        Ok("⏭️ Fast-forwarded to end".to_string())
    }
    
    /// Inspect variable at current point
    pub fn inspect(&self, var_name: &str) -> Result<String, String> {
        let snapshot = self.timeline.manager.current()
            .ok_or("No current snapshot")?;
        
        snapshot.environment.get(var_name)
            .map(|val| format!("{} = {:?}", var_name, val))
            .ok_or(format!("Variable '{}' not found", var_name))
    }
    
    /// Inspect variable N steps ago
    pub fn inspect_at(&mut self, var_name: &str, steps_ago: usize) -> Result<String, String> {
        let current_idx = self.timeline.manager.current_index;
        let target_idx = current_idx.saturating_sub(steps_ago);
        
        let snapshot = self.timeline.manager.goto(target_idx)
            .ok_or("Invalid step count")?;
        
        let result = snapshot.environment.get(var_name)
            .map(|val| format!("{} = {:?} (at snapshot #{})", var_name, val, snapshot.id))
            .ok_or(format!("Variable '{}' not found", var_name));
        
        // Restore current position
        self.timeline.manager.current_index = current_idx;
        
        result
    }
    
    /// Show timeline summary
    pub fn summary(&self) -> String {
        if let Some(pos) = self.timeline.current_position() {
            format!(
                "📊 Timeline Summary:\n\
                 ├─ Current: {}\n\
                 ├─ Total Snapshots: {}\n\
                 └─ Memory Usage: {} KB",
                pos,
                self.timeline.manager.len(),
                self.timeline.manager.memory_usage() / 1024
            )
        } else {
            "No snapshots recorded yet".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;
    
    #[test]
    fn test_timeline_navigation() {
        let source = "let x = 1\nlet y = 2\nlet z = 3";
        let mut timeline = Timeline::new(source);
        let env = Environment::new();
        
        // Record some snapshots
        timeline.manager_mut().record(1, &env);
        timeline.manager_mut().record(2, &env);
        timeline.manager_mut().record(3, &env);
        
        let mut nav = TimelineNavigator::new(&mut timeline);
        
        // Navigate back
        assert!(nav.back().is_ok());
        let current_line = nav.timeline.manager().current().unwrap().line;
        assert_eq!(current_line, 2);
        
        // Navigate forward
        assert!(nav.forward().is_ok());
        let current_line = nav.timeline.manager().current().unwrap().line;
        assert_eq!(current_line, 3);
    }
}
