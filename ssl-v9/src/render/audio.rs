use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder};
use std::io::BufReader;
use std::fs::File;

pub struct AudioEngine {
    _stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    sinks: Vec<Sink>,
}

unsafe impl Send for AudioEngine {}
unsafe impl Sync for AudioEngine {}

impl AudioEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            _stream,
            stream_handle,
            sinks: Vec::new(),
        }
    }

    pub fn play_sound(&mut self, path: &str) {
        if let Ok(file) = File::open(path) {
            let file = BufReader::new(file);
            if let Ok(source) = Decoder::new(file) {
                if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                    sink.append(source);
                    sink.detach(); // Fire and forget for now
                    // self.sinks.push(sink); // Or keep track if we need to stop/control
                }
            } else {
                println!("Failed to decode audio: {}", path);
            }
        } else {
            println!("Failed to open audio file: {}", path);
        }
    }
    
    pub fn play_spatial(&mut self, path: &str, _position: [f32; 3], _listener_position: [f32; 3]) {
        // Placeholder for spatial audio (would calculate volume/pan)
        self.play_sound(path);
    }
}
