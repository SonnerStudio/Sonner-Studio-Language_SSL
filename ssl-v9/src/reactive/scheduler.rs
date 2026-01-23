//! Scheduler for reactive streams

use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::collections::VecDeque;

/// Scheduler for controlling stream execution
pub trait Scheduler: Send + Sync {
    /// Schedule a task for execution
    fn schedule(&self, task: Box<dyn FnOnce() + Send>);
    
    /// Schedule a task with delay
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration);
}

/// Immediate scheduler - executes on current thread
pub struct ImmediateScheduler;

impl Scheduler for ImmediateScheduler {
    fn schedule(&self, task: Box<dyn FnOnce() + Send>) {
        task();
    }
    
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration) {
        std::thread::sleep(delay);
        task();
    }
}

/// New thread scheduler - spawns new thread for each task
pub struct NewThreadScheduler;

impl Scheduler for NewThreadScheduler {
    fn schedule(&self, task: Box<dyn FnOnce() + Send>) {
        thread::spawn(move || task());
    }
    
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration) {
        thread::spawn(move || {
            thread::sleep(delay);
            task();
        });
    }
}

/// Thread pool scheduler
pub struct ThreadPoolScheduler {
    sender: mpsc::Sender<SchedulerMessage>,
    workers: Vec<JoinHandle<()>>,
}

enum SchedulerMessage {
    Task(Box<dyn FnOnce() + Send>),
    Shutdown,
}

impl ThreadPoolScheduler {
    /// Create a new thread pool with the given number of workers
    pub fn new(num_workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<SchedulerMessage>();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(num_workers);
        
        for _ in 0..num_workers {
            let recv = receiver.clone();
            let handle = thread::spawn(move || {
                loop {
                    let msg = {
                        let rx = recv.lock().unwrap();
                        rx.recv()
                    };
                    
                    match msg {
                        Ok(SchedulerMessage::Task(task)) => task(),
                        Ok(SchedulerMessage::Shutdown) => break,
                        Err(_) => break,
                    }
                }
            });
            workers.push(handle);
        }
        
        Self { sender, workers }
    }
    
    /// Create with number of CPUs
    pub fn with_cpus() -> Self {
        Self::new(num_cpus())
    }
    
    /// Shutdown the thread pool
    pub fn shutdown(self) {
        for _ in &self.workers {
            let _ = self.sender.send(SchedulerMessage::Shutdown);
        }
        for worker in self.workers {
            let _ = worker.join();
        }
    }
}

impl Scheduler for ThreadPoolScheduler {
    fn schedule(&self, task: Box<dyn FnOnce() + Send>) {
        let _ = self.sender.send(SchedulerMessage::Task(task));
    }
    
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration) {
        let sender = self.sender.clone();
        thread::spawn(move || {
            thread::sleep(delay);
            let _ = sender.send(SchedulerMessage::Task(task));
        });
    }
}

/// Single-threaded event loop scheduler
pub struct EventLoopScheduler {
    sender: mpsc::Sender<SchedulerMessage>,
    handle: Option<JoinHandle<()>>,
}

impl EventLoopScheduler {
    /// Create a new event loop scheduler
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<SchedulerMessage>();
        
        let handle = thread::spawn(move || {
            for msg in receiver {
                match msg {
                    SchedulerMessage::Task(task) => task(),
                    SchedulerMessage::Shutdown => break,
                }
            }
        });
        
        Self {
            sender,
            handle: Some(handle),
        }
    }
    
    /// Stop the event loop
    pub fn stop(&mut self) {
        let _ = self.sender.send(SchedulerMessage::Shutdown);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Default for EventLoopScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for EventLoopScheduler {
    fn schedule(&self, task: Box<dyn FnOnce() + Send>) {
        let _ = self.sender.send(SchedulerMessage::Task(task));
    }
    
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration) {
        let sender = self.sender.clone();
        thread::spawn(move || {
            thread::sleep(delay);
            let _ = sender.send(SchedulerMessage::Task(task));
        });
    }
}

impl Drop for EventLoopScheduler {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Trampoline scheduler - batches tasks to avoid stack overflow
pub struct TrampolineScheduler {
    queue: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
    running: Arc<Mutex<bool>>,
}

impl TrampolineScheduler {
    /// Create a new trampoline scheduler
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    fn drain(&self) {
        loop {
            let task = {
                let mut q = self.queue.lock().unwrap();
                q.pop_front()
            };
            
            match task {
                Some(t) => t(),
                None => break,
            }
        }
        *self.running.lock().unwrap() = false;
    }
}

impl Default for TrampolineScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for TrampolineScheduler {
    fn schedule(&self, task: Box<dyn FnOnce() + Send>) {
        self.queue.lock().unwrap().push_back(task);
        
        let should_run = {
            let mut r = self.running.lock().unwrap();
            if !*r {
                *r = true;
                true
            } else {
                false
            }
        };
        
        if should_run {
            self.drain();
        }
    }
    
    fn schedule_delayed(&self, task: Box<dyn FnOnce() + Send>, delay: Duration) {
        let queue = self.queue.clone();
        let running = self.running.clone();
        
        thread::spawn(move || {
            thread::sleep(delay);
            queue.lock().unwrap().push_back(task);
            
            let should_run = {
                let mut r = running.lock().unwrap();
                if !*r {
                    *r = true;
                    true
                } else {
                    false
                }
            };
            
            if should_run {
                loop {
                    let t = queue.lock().unwrap().pop_front();
                    match t {
                        Some(task) => task(),
                        None => break,
                    }
                }
                *running.lock().unwrap() = false;
            }
        });
    }
}

/// Get the number of CPUs
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[test]
    fn test_immediate_scheduler() {
        let scheduler = ImmediateScheduler;
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();
        
        scheduler.schedule(Box::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
        }));
        
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
    
    #[test]
    fn test_thread_pool() {
        let pool = ThreadPoolScheduler::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        
        for _ in 0..10 {
            let c = counter.clone();
            pool.schedule(Box::new(move || {
                c.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
        
        pool.shutdown();
    }
    
    #[test]
    fn test_event_loop() {
        let mut scheduler = EventLoopScheduler::new();
        let counter = Arc::new(AtomicUsize::new(0));
        
        for _ in 0..5 {
            let c = counter.clone();
            scheduler.schedule(Box::new(move || {
                c.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        thread::sleep(Duration::from_millis(50));
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        
        scheduler.stop();
    }
}
