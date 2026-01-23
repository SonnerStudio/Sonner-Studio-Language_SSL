pub mod ast;
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub mod interpreter; // Might need to be wasm compatible later, but for now gate if it uses FS
pub mod lexer; // Lexer should be fine
#[cfg(not(target_arch = "wasm32"))]
pub mod lsp;
#[cfg(not(target_arch = "wasm32"))]
pub mod debugger;  // Time-Travel Debugging
// pub mod benchmarks;  // SSL 3.0: Performance benchmarks
// pub mod hotreload; // Live Programming / Hot Reload
#[cfg(not(target_arch = "wasm32"))]
pub mod ai;        // AI-First Programming (Keep for main.rs usage?)
#[cfg(not(target_arch = "wasm32"))]
pub mod visual;    // Visual Reactive Programming
pub mod parser;
#[cfg(not(target_arch = "wasm32"))]
pub mod stdlib;
#[cfg(not(target_arch = "wasm32"))]
pub mod nil;  // Natural Input Layer
#[cfg(not(target_arch = "wasm32"))]
pub mod aurora; // Aurora Compiler (JIT/AOT)
#[cfg(not(target_arch = "wasm32"))]
pub mod security; // Security Sandbox
#[cfg(not(target_arch = "wasm32"))]
pub mod telemetry; // Telemetry System
// pub mod freestanding; // SSL 3.2: Freestanding/Bare-Metal Environment
// pub mod wasm;  // SSL 4.0: WebAssembly Support
// pub mod package;  // Package Manager
// pub mod mobile;  // SSL 4.0: Native Mobile Apps (iOS/Android)
pub mod render;    // SSL v9 "Aurora" 3D Engine

// SSL 4.0 Advanced Features
// pub mod property_test;  // Property-Based Testing
// pub mod reactive;       // Reactive Streams
// pub mod edge;           // Edge/Serverless Deployment
// pub mod crdt;           // Conflict-free Replicated Data Types
// pub mod gpu;            // GPU/SIMD Support
// pub mod verify;         // Formal Verification
// pub mod content_hash;   // Content-Addressable Code
// pub mod effects;        // Algebraic Effects
// pub mod linear;         // Linear/Affine Types

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn run_app_wasm() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        platform::web::WindowExtWebSys,
    };

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("SSL v9 Web")
        .build(&event_loop)
        .unwrap();

    // Attach to DOM
    let window_doc = web_sys::window().unwrap();
    let document = window_doc.document().unwrap();
    let body = document.body().unwrap();
    let canvas = window.canvas().unwrap(); // winit 0.29 returns Option or Result? usually Option, but web-sys might panic. 0.29 returns Option.
    body.append_child(&canvas).unwrap();

    let window_arc = std::sync::Arc::new(window);
    
    // Initialize State (Async)
    let mut state = crate::render::State::new(window_arc.clone()).await;

    event_loop.run(move |event, target| {
        // winit 0.29 run closure takes (event, target). control_flow is set on target.
        target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, window_id } if window_id == window_arc.id() => {
                match event {
                     WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {
                        if !state.input(&event) {
                            match event {
                                WindowEvent::CloseRequested => target.exit(),
                                WindowEvent::Resized(physical_size) => {
                                    state.resize(physical_size);
                                }
                                WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {
                                     // Handle scale factor
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Event::AboutToWait => { // MainEventsCleared renamed to AboutToWait in 0.29
                window_arc.request_redraw();
            }
            _ => {}
        }
    }).unwrap(); // run returns Result in 0.29
}
