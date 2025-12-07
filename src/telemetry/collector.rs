// Telemetry Collector
// Aggregates and stores runtime metrics

use crate::telemetry::metrics::{Event, Metric};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TelemetryCollector {
    events: Arc<Mutex<Vec<Event>>>,
    function_stats: Arc<Mutex<HashMap<String, FunctionStats>>>,
    enabled: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub struct FunctionStats {
    pub name: String,
    pub total_calls: u64,
    pub total_execution_time_us: u64,
    pub avg_execution_time_us: u64,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        TelemetryCollector {
            events: Arc::new(Mutex::new(Vec::new())),
            function_stats: Arc::new(Mutex::new(HashMap::new())),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    pub fn enable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = true;
    }

    pub fn disable(&self) {
        let mut enabled = self.enabled.lock().unwrap();
        *enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    /// Record a metric event
    pub fn record(&self, metric: Metric) {
        if !self.is_enabled() {
            return;
        }

        let event = Event::new(metric.clone());
        
        // Store event
        {
            let mut events = self.events.lock().unwrap();
            events.push(event);
        }

        // Update function stats if it's a function call
        if let Metric::FunctionCall { name, execution_time_us, .. } = metric {
            let mut stats = self.function_stats.lock().unwrap();
            let entry = stats.entry(name.clone()).or_insert(FunctionStats {
                name: name.clone(),
                total_calls: 0,
                total_execution_time_us: 0,
                avg_execution_time_us: 0,
            });
            
            entry.total_calls += 1;
            entry.total_execution_time_us += execution_time_us;
            entry.avg_execution_time_us = entry.total_execution_time_us / entry.total_calls;
        }
    }

    /// Get all collected events
    pub fn get_events(&self) -> Vec<Event> {
        self.events.lock().unwrap().clone()
    }

    /// Get function statistics
    pub fn get_function_stats(&self) -> HashMap<String, FunctionStats> {
        self.function_stats.lock().unwrap().clone()
    }

    /// Get hot path candidates (functions with high call count or execution time)
    pub fn get_hot_paths(&self, min_calls: u64, min_avg_time_us: u64) -> Vec<FunctionStats> {
        let stats = self.function_stats.lock().unwrap();
        stats.values()
            .filter(|s| s.total_calls >= min_calls || s.avg_execution_time_us >= min_avg_time_us)
            .cloned()
            .collect()
    }

    /// Clear all collected data
    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
        self.function_stats.lock().unwrap().clear();
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> TelemetrySummary {
        let events = self.events.lock().unwrap();
        let stats = self.function_stats.lock().unwrap();
        
        TelemetrySummary {
            total_events: events.len(),
            total_functions: stats.len(),
            most_called_function: stats.values()
                .max_by_key(|s| s.total_calls)
                .map(|s| s.name.clone()),
            hottest_function: stats.values()
                .max_by_key(|s| s.avg_execution_time_us)
                .map(|s| s.name.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TelemetrySummary {
    pub total_events: usize,
    pub total_functions: usize,
    pub most_called_function: Option<String>,
    pub hottest_function: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_basic() {
        let collector = TelemetryCollector::new();
        
        collector.record(Metric::FunctionCall {
            name: "test_func".to_string(),
            execution_time_us: 1000,
            call_count: 1,
        });
        
        let events = collector.get_events();
        assert_eq!(events.len(), 1);
        
        let stats = collector.get_function_stats();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats.get("test_func").unwrap().total_calls, 1);
    }
    
    #[test]
    fn test_hot_path_detection() {
        let collector = TelemetryCollector::new();
        
        // Simulate multiple calls to same function
        for i in 0..100 {
            collector.record(Metric::FunctionCall {
                name: "hot_function".to_string(),
                execution_time_us: 500 + i,
                call_count: 1,
            });
        }
        
        let hot_paths = collector.get_hot_paths(50, 0);
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].name, "hot_function");
        assert_eq!(hot_paths[0].total_calls, 100);
    }
    
    #[test]
    fn test_enable_disable() {
        let collector = TelemetryCollector::new();
        
        collector.disable();
        collector.record(Metric::FunctionCall {
            name: "test".to_string(),
            execution_time_us: 100,
            call_count: 1,
        });
        
        assert_eq!(collector.get_events().len(), 0);
        
        collector.enable();
        collector.record(Metric::FunctionCall {
            name: "test".to_string(),
            execution_time_us: 100,
            call_count: 1,
        });
        
        assert_eq!(collector.get_events().len(), 1);
    }
}
