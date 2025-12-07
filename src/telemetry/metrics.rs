// Telemetry Metrics Definitions

use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

/// Metric types that can be collected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Metric {
    /// Function call with execution time
    FunctionCall {
        name: String,
        execution_time_us: u64,
        call_count: u64,
    },
    /// Memory allocation event
    MemoryAllocation {
        bytes: usize,
        location: String,
    },
    /// Expression evaluation time
    ExpressionEval {
        expr_type: String,
        execution_time_us: u64,
    },
    /// Module load event
    ModuleLoad {
        module_name: String,
        load_time_us: u64,
    },
    /// Error occurrence
    Error {
        error_type: String,
        message: String,
        location: String,
    },
    /// JIT compilation trigger
    JitCompilation {
        function_name: String,
        reason: String, // e.g., "hot_path", "explicit"
    },
}

/// Event represents a significant runtime occurrence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: u64, // Unix timestamp in microseconds
    pub metric: Metric,
}

impl Event {
    pub fn new(metric: Metric) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        
        Event { timestamp, metric }
    }
}

/// Timer for measuring execution time
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed_micros(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_creation() {
        let metric = Metric::FunctionCall {
            name: "test_func".to_string(),
            execution_time_us: 1000,
            call_count: 5,
        };
        
        let event = Event::new(metric);
        assert!(event.timestamp > 0);
    }
    
    #[test]
    fn test_timer() {
        let timer = Timer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = timer.elapsed_micros();
        assert!(elapsed >= 10_000); // At least 10ms
    }
}
