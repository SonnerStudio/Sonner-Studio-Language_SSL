//! SSL 4.0 Reactive Streams
//!
//! First-class reactive programming with streams, operators, and schedulers.

pub mod stream;
// pub mod operators;  // Temporarily disabled
pub mod scheduler;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// A reactive stream that emits values over time
pub struct Stream<T> {
    /// Internal state
    state: Arc<Mutex<StreamState<T>>>,
}

struct StreamState<T> {
    /// Buffered values
    buffer: VecDeque<T>,
    /// Subscribers
    subscribers: Vec<Box<dyn FnMut(&T) + Send>>,
    /// Error handler
    error_handler: Option<Box<dyn FnMut(&str) + Send>>,
    /// Completion flag
    completed: bool,
}

impl<T: Clone + Send + 'static> Stream<T> {
    /// Create a new empty stream
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(StreamState {
                buffer: VecDeque::new(),
                subscribers: Vec::new(),
                error_handler: None,
                completed: false,
            })),
        }
    }
    
    /// Create a stream from an iterator
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let stream = Self::new();
        for item in iter {
            stream.push(item);
        }
        stream
    }
    
    /// Create a stream from a vector
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self::from_iter(vec)
    }
    
    /// Push a value to the stream
    pub fn push(&self, value: T) {
        if let Ok(mut state) = self.state.lock() {
            if !state.completed {
                // Notify all subscribers
                for subscriber in &mut state.subscribers {
                    subscriber(&value);
                }
                state.buffer.push_back(value);
            }
        }
    }
    
    /// Subscribe to stream values
    pub fn subscribe<F>(&self, mut callback: F)
    where
        F: FnMut(&T) + Send + 'static,
    {
        if let Ok(mut state) = self.state.lock() {
            // Send buffered values
            for value in &state.buffer {
                callback(value);
            }
            state.subscribers.push(Box::new(callback));
        }
    }
    
    /// Handle errors
    pub fn on_error<F>(&self, handler: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        if let Ok(mut state) = self.state.lock() {
            state.error_handler = Some(Box::new(handler));
        }
    }
    
    /// Complete the stream
    pub fn complete(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.completed = true;
        }
    }
    
    /// Check if completed
    pub fn is_completed(&self) -> bool {
        self.state.lock().map(|s| s.completed).unwrap_or(true)
    }
    
    /// Map values through a function
    pub fn map<U, F>(self, f: F) -> Stream<U>
    where
        U: Clone + Send + 'static,
        F: Fn(&T) -> U + Send + 'static,
    {
        let output = Stream::new();
        let output_clone = output.state.clone();
        
        self.subscribe(move |value| {
            let mapped = f(value);
            if let Ok(mut state) = output_clone.lock() {
                for subscriber in &mut state.subscribers {
                    subscriber(&mapped);
                }
                state.buffer.push_back(mapped);
            }
        });
        
        output
    }
    
    /// Filter values
    pub fn filter<F>(self, predicate: F) -> Stream<T>
    where
        F: Fn(&T) -> bool + Send + 'static,
    {
        let output = Stream::new();
        let output_clone = output.state.clone();
        
        self.subscribe(move |value| {
            if predicate(value) {
                if let Ok(mut state) = output_clone.lock() {
                    for subscriber in &mut state.subscribers {
                        subscriber(value);
                    }
                    state.buffer.push_back(value.clone());
                }
            }
        });
        
        output
    }
    
    /// Take first n values
    pub fn take(self, n: usize) -> Stream<T> {
        let output = Stream::new();
        let output_clone = output.state.clone();
        let count = Arc::new(Mutex::new(0usize));
        
        self.subscribe(move |value| {
            let mut c = count.lock().unwrap();
            if *c < n {
                *c += 1;
                if let Ok(mut state) = output_clone.lock() {
                    for subscriber in &mut state.subscribers {
                        subscriber(value);
                    }
                    state.buffer.push_back(value.clone());
                }
            }
        });
        
        output
    }
    
    /// Skip first n values
    pub fn skip(self, n: usize) -> Stream<T> {
        let output = Stream::new();
        let output_clone = output.state.clone();
        let count = Arc::new(Mutex::new(0usize));
        
        self.subscribe(move |value| {
            let mut c = count.lock().unwrap();
            if *c >= n {
                if let Ok(mut state) = output_clone.lock() {
                    for subscriber in &mut state.subscribers {
                        subscriber(value);
                    }
                    state.buffer.push_back(value.clone());
                }
            }
            *c += 1;
        });
        
        output
    }
    
    /// Collect all values into a vector
    pub fn collect(&self) -> Vec<T> {
        self.state.lock()
            .map(|s| s.buffer.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Get the current buffer size
    pub fn len(&self) -> usize {
        self.state.lock().map(|s| s.buffer.len()).unwrap_or(0)
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Clone + Send + 'static> Default for Stream<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Send + 'static> Clone for Stream<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

/// Subject - both a stream source and sink
pub struct Subject<T> {
    stream: Stream<T>,
}

impl<T: Clone + Send + 'static> Subject<T> {
    /// Create a new subject
    pub fn new() -> Self {
        Self {
            stream: Stream::new(),
        }
    }
    
    /// Emit a value
    pub fn next(&self, value: T) {
        self.stream.push(value);
    }
    
    /// Get the stream for subscribing
    pub fn as_stream(&self) -> Stream<T> {
        self.stream.clone()
    }
    
    /// Complete the subject
    pub fn complete(&self) {
        self.stream.complete();
    }
}

impl<T: Clone + Send + 'static> Default for Subject<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// BehaviorSubject - remembers last value
pub struct BehaviorSubject<T> {
    stream: Stream<T>,
    current: Arc<Mutex<Option<T>>>,
}

impl<T: Clone + Send + 'static> BehaviorSubject<T> {
    /// Create with initial value
    pub fn new(initial: T) -> Self {
        let stream = Stream::new();
        stream.push(initial.clone());
        
        Self {
            stream,
            current: Arc::new(Mutex::new(Some(initial))),
        }
    }
    
    /// Emit a value
    pub fn next(&self, value: T) {
        if let Ok(mut current) = self.current.lock() {
            *current = Some(value.clone());
        }
        self.stream.push(value);
    }
    
    /// Get current value
    pub fn value(&self) -> Option<T> {
        self.current.lock().ok().and_then(|c| c.clone())
    }
    
    /// Get the stream
    pub fn as_stream(&self) -> Stream<T> {
        self.stream.clone()
    }
}

/// Create stream from interval
pub fn interval(millis: u64) -> Stream<u64> {
    let stream = Stream::new();
    let stream_clone = stream.clone();
    
    std::thread::spawn(move || {
        let mut count = 0u64;
        loop {
            if stream_clone.is_completed() {
                break;
            }
            stream_clone.push(count);
            count += 1;
            std::thread::sleep(std::time::Duration::from_millis(millis));
        }
    });
    
    stream
}

/// Merge multiple streams
pub fn merge<T: Clone + Send + 'static>(streams: Vec<Stream<T>>) -> Stream<T> {
    let output = Stream::new();
    
    for stream in streams {
        let out = output.clone();
        stream.subscribe(move |value| {
            out.push(value.clone());
        });
    }
    
    output
}

/// Combine latest values from multiple streams
pub fn combine_latest<T: Clone + Send + 'static>(
    streams: Vec<Stream<T>>,
) -> Stream<Vec<T>> {
    let output = Stream::new();
    let latest: Arc<Mutex<Vec<Option<T>>>> = Arc::new(Mutex::new(
        vec![None; streams.len()]
    ));
    
    for (i, stream) in streams.into_iter().enumerate() {
        let out = output.clone();
        let lat = latest.clone();
        
        stream.subscribe(move |value| {
            if let Ok(mut l) = lat.lock() {
                l[i] = Some(value.clone());
                
                // Check if all have values
                if l.iter().all(|v| v.is_some()) {
                    let combined: Vec<T> = l.iter()
                        .filter_map(|v| v.clone())
                        .collect();
                    out.push(combined);
                }
            }
        });
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stream_push_subscribe() {
        let stream = Stream::new();
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = results.clone();
        
        stream.subscribe(move |v: &i32| {
            results_clone.lock().unwrap().push(*v);
        });
        
        stream.push(1);
        stream.push(2);
        stream.push(3);
        
        assert_eq!(*results.lock().unwrap(), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_stream_from_iter() {
        let stream = Stream::from_iter(vec![1, 2, 3]);
        assert_eq!(stream.collect(), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_stream_map() {
        let stream = Stream::from_iter(vec![1, 2, 3]);
        let doubled = stream.map(|x| x * 2);
        
        // Need to trigger subscription
        let results = doubled.collect();
        assert_eq!(results, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_stream_filter() {
        let stream = Stream::from_iter(vec![1, 2, 3, 4, 5]);
        let evens = stream.filter(|x| x % 2 == 0);
        assert_eq!(evens.collect(), vec![2, 4]);
    }
    
    #[test]
    fn test_stream_take() {
        let stream = Stream::from_iter(vec![1, 2, 3, 4, 5]);
        let taken = stream.take(3);
        assert_eq!(taken.collect(), vec![1, 2, 3]);
    }
    
    #[test]
    fn test_subject() {
        let subject = Subject::new();
        let stream = subject.as_stream();
        
        subject.next(1);
        subject.next(2);
        
        assert_eq!(stream.collect(), vec![1, 2]);
    }
    
    #[test]
    fn test_behavior_subject() {
        let subject = BehaviorSubject::new(0);
        
        assert_eq!(subject.value(), Some(0));
        
        subject.next(5);
        assert_eq!(subject.value(), Some(5));
    }
}
