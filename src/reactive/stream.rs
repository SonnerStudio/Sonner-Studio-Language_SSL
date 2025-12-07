//! Stream type definitions and core functionality

use std::sync::{Arc, Mutex};

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Buffer size limit
    pub buffer_size: usize,
    /// Whether to replay buffered values to new subscribers
    pub replay: bool,
    /// Number of values to replay (0 = all)
    pub replay_count: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            buffer_size: 1000,
            replay: true,
            replay_count: 0,
        }
    }
}

/// Stream error types
#[derive(Debug, Clone)]
pub enum StreamError {
    /// Buffer overflow
    BufferFull,
    /// Stream completed
    Completed,
    /// Custom error
    Custom(String),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::BufferFull => write!(f, "Stream buffer is full"),
            StreamError::Completed => write!(f, "Stream is completed"),
            StreamError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

/// Hot vs Cold stream distinction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StreamKind {
    /// Cold stream - replays all values to each subscriber
    Cold,
    /// Hot stream - only new values after subscription
    Hot,
}

/// Backpressure strategy
#[derive(Debug, Clone, Copy)]
pub enum BackpressureStrategy {
    /// Drop newest values when buffer is full
    DropNewest,
    /// Drop oldest values when buffer is full
    DropOldest,
    /// Error when buffer is full
    Error,
    /// Block until space available
    Block,
}

/// Stream builder for fluent configuration
pub struct StreamBuilder<T> {
    config: StreamConfig,
    kind: StreamKind,
    backpressure: BackpressureStrategy,
    initial_values: Vec<T>,
}

impl<T: Clone + Send + 'static> StreamBuilder<T> {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            config: StreamConfig::default(),
            kind: StreamKind::Cold,
            backpressure: BackpressureStrategy::DropOldest,
            initial_values: Vec::new(),
        }
    }
    
    /// Set as hot stream
    pub fn hot(mut self) -> Self {
        self.kind = StreamKind::Hot;
        self
    }
    
    /// Set as cold stream
    pub fn cold(mut self) -> Self {
        self.kind = StreamKind::Cold;
        self
    }
    
    /// Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }
    
    /// Set replay count
    pub fn replay(mut self, count: usize) -> Self {
        self.config.replay = true;
        self.config.replay_count = count;
        self
    }
    
    /// Set backpressure strategy
    pub fn backpressure(mut self, strategy: BackpressureStrategy) -> Self {
        self.backpressure = strategy;
        self
    }
    
    /// Add initial values
    pub fn with_values(mut self, values: Vec<T>) -> Self {
        self.initial_values = values;
        self
    }
    
    /// Build the stream
    pub fn build(self) -> super::Stream<T> {
        let stream = super::Stream::new();
        for value in self.initial_values {
            stream.push(value);
        }
        stream
    }
}

impl<T: Clone + Send + 'static> Default for StreamBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Subscription handle - allows unsubscribing
pub struct Subscription {
    id: usize,
    unsubscribe_fn: Option<Box<dyn FnOnce() + Send>>,
}

impl Subscription {
    /// Create a new subscription
    pub fn new(id: usize, unsubscribe: impl FnOnce() + Send + 'static) -> Self {
        Self {
            id,
            unsubscribe_fn: Some(Box::new(unsubscribe)),
        }
    }
    
    /// Get subscription ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Unsubscribe from the stream
    pub fn unsubscribe(mut self) {
        if let Some(f) = self.unsubscribe_fn.take() {
            f();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stream_builder() {
        let stream: super::super::Stream<i32> = StreamBuilder::new()
            .buffer_size(100)
            .hot()
            .with_values(vec![1, 2, 3])
            .build();
        
        assert_eq!(stream.collect(), vec![1, 2, 3]);
    }
}
