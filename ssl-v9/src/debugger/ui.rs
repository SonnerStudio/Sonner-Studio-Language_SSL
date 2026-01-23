use super::timeline::{Timeline, TimelineNavigator};

/// Debug commands available in REPL/CLI
pub enum DebugCommands {
    /// Step back one snapshot
    Back,
    
    /// Step forward one snapshot  
    Forward,
    
    /// Rewind to specific line
    RewindTo(usize),
    
    /// Rewind to start
    RewindToStart,
    
    /// Fast-forward to end
    FastForwardToEnd,
    
    /// Inspect variable
    Inspect { var: String },
    
    /// Inspect variable N steps ago
    InspectAt { var: String, steps_ago: usize },
    
    /// Show timeline summary
    Summary,
    
    /// Toggle recording
    ToggleRecording,
}

impl DebugCommands {
    /// Parse command from string
    pub fn parse(input: &str) -> Result<Self, String> {
        let input = input.trim();
        
        if input == "@back" || input == "@b" {
            Ok(DebugCommands::Back)
        } else if input == "@forward" || input == "@f" {
            Ok(DebugCommands::Forward)
        } else if input == "@start" {
            Ok(DebugCommands::RewindToStart)
        } else if input == "@end" {
            Ok(DebugCommands::FastForwardToEnd)
        } else if input == "@summary" || input == "@s" {
            Ok(DebugCommands::Summary)
        } else if input == "@record" {
            Ok(DebugCommands::ToggleRecording)
        } else if input.starts_with("@rewind ") {
            let line = input.trim_start_matches("@rewind ")
                .parse()
                .map_err(|_| "Invalid line number")?;
            Ok(DebugCommands::RewindTo(line))
        } else if input.starts_with("@inspect ") {
            let rest = input.trim_start_matches("@inspect ");
            
            // Check if it has "at" clause
            if let Some((var, steps)) = rest.split_once(" at ") {
                let steps_ago = steps.trim_start_matches('-')
                    .parse()
                    .map_err(|_| "Invalid step count")?;
                Ok(DebugCommands::InspectAt {
                    var: var.to_string(),
                    steps_ago,
                })
            } else {
                Ok(DebugCommands::Inspect {
                    var: rest.to_string(),
                })
            }
        } else {
            Err(format!("Unknown debug command: {}", input))
        }
    }
}

/// Debug UI for REPL integration
pub struct DebugUI {
    timeline: Timeline,
    enabled: bool,
}

impl DebugUI {
    /// Create a new debug UI
    pub fn new(source: &str) -> Self {
        DebugUI {
            timeline: Timeline::new(source),
            enabled: false,
        }
    }
    
    /// Enable/disable debugging
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.timeline.set_recording(enabled);
    }
    
    /// Check if debugging is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Execute a debug command
    pub fn execute(&mut self, cmd: DebugCommands) -> Result<String, String> {
        if !self.enabled && !matches!(cmd, DebugCommands::ToggleRecording) {
            return Err("Debugging not enabled. Use @record to start".to_string());
        }
        
        match cmd {
            DebugCommands::Back => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.back()
            }
            DebugCommands::Forward => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.forward()
            }
            DebugCommands::RewindTo(line) => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.rewind_to(line)
            }
            DebugCommands::RewindToStart => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.rewind_to_start()
            }
            DebugCommands::FastForwardToEnd => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.fast_forward_to_end()
            }
            DebugCommands::Inspect { var } => {
                let nav = TimelineNavigator::new(&mut self.timeline);
                nav.inspect(&var)
            }
            DebugCommands::InspectAt { var, steps_ago } => {
                let mut nav = TimelineNavigator::new(&mut self.timeline);
                nav.inspect_at(&var, steps_ago)
            }
            DebugCommands::Summary => {
                let nav = TimelineNavigator::new(&mut self.timeline);
                Ok(nav.summary())
            }
            DebugCommands::ToggleRecording => {
                self.enabled = !self.enabled;
                self.timeline.set_recording(self.enabled);
                Ok(format!("Recording {}", if self.enabled { "enabled ⏺️" } else { "disabled ⏸️" }))
            }
        }
    }
    
    /// Get access to timeline for interpreter integration
    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }
}

/// Help text for debug commands
pub const DEBUG_HELP: &str = r#"
🔍 Time-Travel Debug Commands:
├─ @record              Toggle recording on/off
├─ @back / @b           Step back one snapshot
├─ @forward / @f        Step forward one snapshot
├─ @start               Rewind to beginning
├─ @end                 Fast-forward to end
├─ @rewind <line>       Jump to specific line
├─ @inspect <var>       Show variable value
├─ @inspect <var> at -N Show variable N steps ago
└─ @summary / @s        Show timeline info

Examples:
  @back               # Go back one step
  @rewind 10          # Jump to line 10
  @inspect x at -5    # Show x from 5 steps ago
"#;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_parsing() {
        assert!(matches!(
            DebugCommands::parse("@back").unwrap(),
            DebugCommands::Back
        ));
        
        assert!(matches!(
            DebugCommands::parse("@rewind 10").unwrap(),
            DebugCommands::RewindTo(10)
        ));
        
        assert!(matches!(
            DebugCommands::parse("@inspect x").unwrap(),
            DebugCommands::Inspect { var } if var == "x"
        ));
        
        assert!(matches!(
            DebugCommands::parse("@inspect y at -5").unwrap(),
            DebugCommands::InspectAt { var, steps_ago } 
            if var == "y" && steps_ago == 5
        ));
    }
}
