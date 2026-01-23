//! Stream operators for reactive programming

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use super::Stream;

/// Debounce values - only emit after quiet period
pub fn debounce<T: Clone + Send + 'static>(
    stream: Stream<T>,
    delay: Duration,
) -> Stream<T> {
    let output = Stream::new();
    let last_emit: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let pending: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        *pending.lock().unwrap() = Some(value.clone());
        *last_emit.lock().unwrap() = Some(Instant::now());
        
        let out_clone = out.clone();
        let pending_clone = pending.clone();
        let last_clone = last_emit.clone();
        let delay_clone = delay;
        
        std::thread::spawn(move || {
            std::thread::sleep(delay_clone);
            
            let should_emit = last_clone.lock()
                .map(|l| l.map(|t| t.elapsed() >= delay_clone).unwrap_or(false))
                .unwrap_or(false);
            
            if should_emit {
                if let Some(v) = pending_clone.lock().unwrap().take() {
                    out_clone.push(v);
                }
            }
        });
    });
    
    output
}

/// Throttle values - emit at most once per interval
pub fn throttle<T: Clone + Send + 'static>(
    stream: Stream<T>,
    interval: Duration,
) -> Stream<T> {
    let output = Stream::new();
    let last_emit: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let should_emit = {
            let mut last = last_emit.lock().unwrap();
            match *last {
                None => {
                    *last = Some(Instant::now());
                    true
                }
                Some(t) if t.elapsed() >= interval => {
                    *last = Some(Instant::now());
                    true
                }
                _ => false,
            }
        };
        
        if should_emit {
            out.push(value.clone());
        }
    });
    
    output
}

/// Distinct values only - filter out consecutive duplicates
pub fn distinct<T: Clone + Send + PartialEq + 'static>(
    stream: Stream<T>,
) -> Stream<T> {
    let output = Stream::new();
    let last: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let should_emit = {
            let mut l = last.lock().unwrap();
            let emit = l.as_ref() != Some(value);
            if emit {
                *l = Some(value.clone());
            }
            emit
        };
        
        if should_emit {
            out.push(value.clone());
        }
    });
    
    output
}

/// Scan/reduce with accumulator
pub fn scan<T, U, F>(
    stream: Stream<T>,
    initial: U,
    f: F,
) -> Stream<U>
where
    T: Clone + Send + 'static,
    U: Clone + Send + 'static,
    F: Fn(&U, &T) -> U + Send + 'static,
{
    let output = Stream::new();
    let acc: Arc<Mutex<U>> = Arc::new(Mutex::new(initial));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let new_acc = {
            let mut a = acc.lock().unwrap();
            let result = f(&a, value);
            *a = result.clone();
            result
        };
        out.push(new_acc);
    });
    
    output
}

/// Buffer values into chunks
pub fn buffer<T: Clone + Send + 'static>(
    stream: Stream<T>,
    size: usize,
) -> Stream<Vec<T>> {
    let output = Stream::new();
    let buffer: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(Vec::new()));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let emit = {
            let mut b = buffer.lock().unwrap();
            b.push(value.clone());
            if b.len() >= size {
                let chunk = b.drain(..).collect::<Vec<_>>();
                Some(chunk)
            } else {
                None
            }
        };
        
        if let Some(chunk) = emit {
            out.push(chunk);
        }
    });
    
    output
}

/// Delay each value by a duration
pub fn delay<T: Clone + Send + 'static>(
    stream: Stream<T>,
    duration: Duration,
) -> Stream<T> {
    let output = Stream::new();
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let v = value.clone();
        let o = out.clone();
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            o.push(v);
        });
    });
    
    output
}

/// Take values while predicate is true
pub fn take_while<T, F>(stream: Stream<T>, predicate: F) -> Stream<T>
where
    T: Clone + Send + 'static,
    F: Fn(&T) -> bool + Send + 'static,
{
    let output = Stream::new();
    let active = Arc::new(Mutex::new(true));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let should_emit = {
            let mut a = active.lock().unwrap();
            if *a && predicate(value) {
                true
            } else {
                *a = false;
                false
            }
        };
        
        if should_emit {
            out.push(value.clone());
        }
    });
    
    output
}

/// Skip values while predicate is true
pub fn skip_while<T, F>(stream: Stream<T>, predicate: F) -> Stream<T>
where
    T: Clone + Send + 'static,
    F: Fn(&T) -> bool + Send + 'static,
{
    let output = Stream::new();
    let skipping = Arc::new(Mutex::new(true));
    let out = output.clone();
    
    stream.subscribe(move |value| {
        let should_emit = {
            let mut s = skipping.lock().unwrap();
            if *s {
                if !predicate(value) {
                    *s = false;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        };
        
        if should_emit {
            out.push(value.clone());
        }
    });
    
    output
}

/// Sample stream at intervals
pub fn sample<T: Clone + Send + 'static>(
    stream: Stream<T>,
    interval: Duration,
) -> Stream<T> {
    let output = Stream::new();
    let latest: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let out = output.clone();
    let lat = latest.clone();
    
    stream.subscribe(move |value| {
        *lat.lock().unwrap() = Some(value.clone());
    });
    
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(interval);
            if let Some(v) = latest.lock().unwrap().take() {
                out.push(v);
            }
            if output.is_completed() {
                break;
            }
        }
    });
    
    output.clone()
}

/// Window - collect values into time-based windows
pub fn window<T: Clone + Send + 'static>(
    stream: Stream<T>,
    duration: Duration,
) -> Stream<Vec<T>> {
    let output = Stream::new();
    let buffer: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(Vec::new()));
    let out = output.clone();
    let buf = buffer.clone();
    
    stream.subscribe(move |value| {
        buf.lock().unwrap().push(value.clone());
    });
    
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(duration);
            let chunk: Vec<T> = buffer.lock().unwrap().drain(..).collect();
            if !chunk.is_empty() {
                out.push(chunk);
            }
            if output.is_completed() {
                break;
            }
        }
    });
    
    output.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distinct() {
        let stream = Stream::from_iter(vec![1, 1, 2, 2, 3, 3, 1]);
        let distincted = distinct(stream);
        assert_eq!(distincted.collect(), vec![1, 2, 3, 1]);
    }
    
    #[test]
    fn test_scan() {
        let stream = Stream::from_iter(vec![1, 2, 3, 4, 5]);
        let sums = scan(stream, 0, |acc, x| acc + x);
        assert_eq!(sums.collect(), vec![1, 3, 6, 10, 15]);
    }
    
    #[test]
    fn test_buffer() {
        let stream = Stream::from_iter(vec![1, 2, 3, 4, 5, 6]);
        let buffered = buffer(stream, 2);
        assert_eq!(buffered.collect(), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }
    
    #[test]
    fn test_take_while() {
        let stream = Stream::from_iter(vec![1, 2, 3, 4, 5]);
        let taken = take_while(stream, |x| *x < 4);
        assert_eq!(taken.collect(), vec![1, 2, 3]);
    }
}
