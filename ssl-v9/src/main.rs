use clap::Parser;
use ssl_v9::cli::{Cli, Commands};
use ssl_v9::ast::Statement;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() {
    let args = Cli::parse();

    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
    let demo_path = exe_dir.join("modern_demo.ssl");

    let command = args.command.unwrap_or_else(|| {
        if demo_path.exists() {
             Commands::Run { 
                 file: Some(demo_path.to_str().unwrap().to_string()), 
                 debug: false, 
                 watch: false, 
                 ai_review: false,
                 args: vec![] 
             }
        } else {
             Commands::Run { 
                 file: None, 
                 debug: false, 
                 watch: false, 
                 ai_review: false,
                 args: vec![] 
             }
        }
    });

    match command {
        Commands::Compile { source } => {
            println!("🚀 SSL v9 Hybrid Compiler");
            println!("   Compiling: {}", source);
        }
        Commands::Run { file, debug, watch: _, ai_review: _, args: _ } => {
            if let Some(path) = file {
                run_file(&path, debug);
            } else {
                let exe_path = std::env::current_exe().unwrap_or_default();
                let exe_dir = exe_path.parent().unwrap_or(std::path::Path::new("."));
                let demo_path = exe_dir.join("modern_demo.ssl");
                if demo_path.exists() {
                     run_file(demo_path.to_str().unwrap(), debug);
                } else {
                println!("SSL REPL v9.3.2 (Hybrid) - Standard Mode");
                let welcome_html = r#"
                    <div style="display:flex; justify-content:center; align-items:center; height:100vh; background: linear-gradient(135deg, #1a1a1a 0%, #0d0d0d 100%); color: white; font-family: sans-serif; flex-direction: column;">
                        <h1 style="font-size: 3rem; margin-bottom: 1rem; background: linear-gradient(to right, #00ff88, #00aaff); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">SSL v9.3.2 Aurora</h1>
                        <p style="opacity: 0.8;">Hybrid Runtime Ready</p>
                    </div>
                "#.to_string();
                
                let ast = vec![
                    Statement::WindowDecl(ssl_v9::ast::WindowDecl {
                        title: "SSL v9.3.2 - Welcome".to_string(),
                        width: 1024,
                        height: 768,
                        transparent: false,
                        decorations: true,
                        html: welcome_html,
                    })
                ];
                launch_hybrid_runtime(ast, "internal_welcome".to_string());
                }
            }
        }
        Commands::Doctor => {
            println!("🩺 SSL Doctor v9.3.2");
            println!("✅ SSL Core: v9.3.2 (Hybrid Runtime)");
            println!("Press Enter to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

fn run_file(path: &str, _debug: bool) {
    use std::fs;
    match fs::read_to_string(path) {
        Ok(content) => {
             match ssl_v9::parser::parse(&content) {
                Ok(ast) => {
                    if has_html_content(&ast) {
                        launch_hybrid_runtime(ast, path.to_string());
                    } else {
                        let mut interp = ssl_v9::interpreter::Interpreter::new();
                        if let Err(e) = interp.interpret(ast) {
                             eprintln!("Runtime Error: {}", e);
                        }
                    }
                },
                Err(e) => eprintln!("Parse Error: {}", e),
            }
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn has_html_content(ast: &[Statement]) -> bool {
    for stmt in ast {
        if let Statement::HtmlBlock(_) = stmt { return true; }
        if let Statement::WindowDecl(_) = stmt { return true; }
        if let Statement::FunctionDecl(f) = stmt {
            for s in &f.body {
                 if let Statement::HtmlBlock(_) = s { return true; }
                 if let Statement::WindowDecl(_) = s { return true; }
            }
        }
    }
    false
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Region {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug)]
enum SSLUserEvent {
    Reload,
    SetIgnoreMouse(bool),
    UpdateRegions(Vec<Region>),
    CheckMouse((i32, i32)),
    Rotate(f32),
}

fn launch_hybrid_runtime(ast: Vec<Statement>, file_path: String) {
    println!("🌟 Initializing SSL v9 Hybrid Runtime...");
    
    // Store-safe diagnostics
    if let Ok(cwd) = std::env::current_dir() {
        println!("📁 Working Directory: {:?}", cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        println!("📦 Executable Path: {:?}", exe);
    }
    
    let make_html = |ast: &Vec<Statement>| -> String {
        let mut html_content = String::new();
        html_content.push_str("<!DOCTYPE html><html><head><meta charset='UTF-8'><style>body { font-family: system-ui, -apple-system, sans-serif; }</style></head><body style='background:transparent; overflow: hidden; margin: 0;'>");
        for stmt in ast {
             match stmt {
                Statement::HtmlBlock(html) => html_content.push_str(html),
                Statement::WindowDecl(decl) => html_content.push_str(&decl.html),
                _ => {}
            }
        }
        html_content.push_str("</body></html>");
        html_content
    };

    let html_response = make_html(&ast);
    let mut title = "SSL v9 App".to_string();
    let mut width = 800.0;
    let mut height = 600.0;
    let mut transparent = false;
    let mut decorations = true;

    for stmt in &ast {
        if let Statement::WindowDecl(decl) = stmt {
            title = decl.title.clone();
            width = decl.width as f64;
            height = decl.height as f64;
            transparent = decl.transparent;
            decorations = decl.decorations;
        }
    }

    use tao::event_loop::EventLoopBuilder;
    let event_loop = EventLoopBuilder::<SSLUserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let webview_ipc_proxy = proxy.clone();
    let mouse_proxy = proxy.clone();
    let watch_path = file_path.clone();
    
    // Store-safe file watcher (disabled in release to prevent sandbox hangs)
    #[cfg(debug_assertions)]
    if watch_path != "internal_welcome" {
        std::thread::spawn(move || {
            use std::time::{Duration, SystemTime};
            match std::fs::metadata(&watch_path) {
                Ok(metadata) => {
                    let mut last_modified = metadata.modified().unwrap_or(SystemTime::now());
                    println!("✅ File watcher enabled for: {}", watch_path);
                    loop {
                        std::thread::sleep(Duration::from_millis(500));
                        if let Ok(metadata) = std::fs::metadata(&watch_path) {
                            if let Ok(modified) = metadata.modified() {
                                if modified > last_modified {
                                    last_modified = modified;
                                    let _ = proxy.send_event(SSLUserEvent::Reload);
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("⚠️ File watcher disabled (Store mode?): {}", e);
                }
            }
        });
    }

    // Store-safe mouse tracking (optional, won't block if fails)
    std::thread::spawn(move || {
        use device_query::{DeviceQuery, DeviceState};
        match std::panic::catch_unwind(|| DeviceState::new()) {
            Ok(device_state) => {
                println!("✅ Mouse tracking enabled");
                loop {
                    let mouse = device_state.get_mouse();
                    let _ = mouse_proxy.send_event(SSLUserEvent::CheckMouse(mouse.coords));
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            },
            Err(_) => {
                eprintln!("⚠️ Mouse tracking disabled (Store mode?)");
            }
        }
    });

    let window = match WindowBuilder::new()
        .with_title(title)
        .with_inner_size(tao::dpi::LogicalSize::new(width, height))
        .with_transparent(transparent)
        .with_decorations(decorations)
        .with_maximized(true)
        .build(&event_loop) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("❌ Failed to create window: {:?}", e);
                eprintln!("💡 Unable to create application window. Please check your display settings.");
                panic!("Cannot create application window");
            }
        };
    
    println!("✅ Window created successfully");
    
    let window_arc = std::sync::Arc::new(window);
    
    // Store-safe WGPU initialization with timeout
    println!("🎨 Initializing WGPU renderer...");
    use std::sync::mpsc;
    use std::time::Duration;
    
    let (tx, rx) = mpsc::channel();
    let window_clone_for_init = window_arc.clone();
    
    std::thread::spawn(move || {
        let state = pollster::block_on(ssl_v9::render::State::new(window_clone_for_init));
        let _ = tx.send(state);
    });
    
    // Wait up to 10 seconds for WGPU initialization
    let mut state = match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(s) => {
            println!("✅ WGPU initialized successfully");
            s
        },
        Err(_) => {
            eprintln!("⚠️ WGPU initialization timeout/failure");
            eprintln!("💡 This may be due to incompatible graphics hardware or outdated drivers.");
            eprintln!("❌ The application cannot continue without graphics support.");
            eprintln!("💡 Please try the following:");
            eprintln!("   1. Update your graphics drivers to the latest version");
            eprintln!("   2. Ensure your system meets the minimum requirements");
            eprintln!("   3. Try running on a system with compatible graphics hardware");
            panic!("WGPU initialization failed - graphics hardware or drivers incompatible");
        }
    };

    // Initialize audio on main thread (safe for non-Send components)
    state.init_audio();

    let mut interp = ssl_v9::interpreter::Interpreter::new();
    let mut initial_logic = Vec::new();
    for stmt in &ast {
        match stmt {
            Statement::HtmlBlock(_) | Statement::WindowDecl(_) | Statement::MaterialDecl(_) => {},
             _ => initial_logic.push(stmt.clone()),
        }
    }
    if !initial_logic.is_empty() {
        let _ = interp.interpret(initial_logic);
    }

    use std::sync::{Arc, RwLock, Mutex};
    let interp_shared = Arc::new(Mutex::new(interp));
    let interp_ipc = interp_shared.clone();
    let html_store = Arc::new(RwLock::new(html_response));
    let html_store_clone = html_store.clone();

    let webview = WebViewBuilder::new(&*window_arc)
        .with_custom_protocol("asset".into(), move |request| {
            use std::path::PathBuf;
            use std::borrow::Cow;
            let path = request.uri().path();
            if path == "/index.html" || path == "/" {
                 return wry::http::Response::builder()
                    .header("Content-Type", "text/html")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Cow::from(html_store_clone.read().unwrap().as_bytes().to_vec()))
                    .unwrap();
            }
            let final_path = if path.starts_with('/') { &path[1..] } else { path };
            let file_path = PathBuf::from(final_path);
            match std::fs::read(&file_path) {
                Ok(content) => {
                     let content_type = match file_path.extension().and_then(|e| e.to_str()) {
                        Some("png") => "image/png",
                        Some("css") => "text/css",
                        Some("js") => "application/javascript",
                         _ => "application/octet-stream",
                     };
                     wry::http::Response::builder().header("Content-Type", content_type).header("Access-Control-Allow-Origin", "*").body(Cow::from(content)).unwrap()
                }
                Err(_) => wry::http::Response::builder().status(404).body(Cow::from(vec![])).unwrap()
            }
        })
        .with_url("asset://localhost/index.html")
        .unwrap()
        .with_transparent(transparent)
        .with_ipc_handler(move |string| {
            if string.starts_with("IGNORE_MOUSE:") {
                let parts: Vec<&str> = string.split(':').collect();
                if let Some(val_str) = parts.get(1) {
                    let _ = webview_ipc_proxy.send_event(SSLUserEvent::SetIgnoreMouse(*val_str == "true"));
                }
            } else if string.starts_with("UPDATE_REGIONS:") {
                 let json_str = &string["UPDATE_REGIONS:".len()..];
                 if let Ok(regions) = serde_json::from_str::<Vec<Region>>(json_str) {
                      let _ = webview_ipc_proxy.send_event(SSLUserEvent::UpdateRegions(regions));
                 }
            } else if string.starts_with("CALL_SSL:") {
                let payload = &string["CALL_SSL:".len()..];
                let parts: Vec<&str> = payload.splitn(2, ':').collect();
                if parts.len() >= 1 {
                    let func_name = parts[0];
                    let args_json = if parts.len() > 1 { parts[1] } else { "[]" };
                    if let Ok(json_args) = serde_json::from_str::<serde_json::Value>(args_json) {
                         let mut ssl_args = Vec::new();
                         if let Some(arr) = json_args.as_array() {
                             for v in arr {
                                 if v.is_string() { ssl_args.push(ssl_v9::interpreter::Value::String(v.as_str().unwrap().to_string())); }
                                 else if v.is_number() { ssl_args.push(ssl_v9::interpreter::Value::Int(v.as_i64().unwrap_or(0))); }
                             }
                         }
                         let mut locked_interp = interp_ipc.lock().unwrap();
                         match locked_interp.env.get(func_name) {
                             Ok(func_val) => { let _ = locked_interp.execute_value_call(func_val, ssl_args, vec![], Some(func_name.to_string())); },
                             Err(_) => {},
                         }
                    }
                }
            }
        })
        .build()
        .unwrap();

    let mut current_regions: Vec<Region> = Vec::new();
    let mut currently_ignoring = false;
    let window_clone = window_arc.clone();
    let path_clone = file_path.clone();
    let html_store_updater = html_store.clone();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                state.update();
                let _ = state.render();
            }
            Event::MainEventsCleared => { state.window.request_redraw(); }
            Event::WindowEvent { ref event, window_id, .. } if window_id == state.window.id() => {
                 if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => { state.resize(*physical_size); }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => { state.resize(**new_inner_size); }
                        _ => {}
                    }
                 }
            }
            Event::UserEvent(SSLUserEvent::SetIgnoreMouse(ignore)) => {
                let _ = window_clone.set_ignore_cursor_events(ignore);
                currently_ignoring = ignore;
            }
            Event::UserEvent(SSLUserEvent::UpdateRegions(regions)) => { current_regions = regions; }
            Event::UserEvent(SSLUserEvent::CheckMouse((global_x, global_y))) => {
                if let Ok(win_pos) = window_clone.outer_position() {
                     let rel_x = global_x as f64 - win_pos.x as f64;
                     let rel_y = global_y as f64 - win_pos.y as f64;
                     let mut inside = false;
                     for r in &current_regions {
                         if rel_x >= r.x && rel_x <= (r.x + r.width) && rel_y >= r.y && rel_y <= (r.y + r.height) {
                            inside = true; break;
                         }
                     }
                     let should_ignore = !inside;
                     if should_ignore != currently_ignoring {
                         let _ = window_clone.set_ignore_cursor_events(should_ignore);
                         currently_ignoring = should_ignore;
                     }
                }
            }
            Event::UserEvent(SSLUserEvent::Reload) => {
                if let Ok(content) = std::fs::read_to_string(&path_clone) {
                    if let Ok(new_ast) = ssl_v9::parser::parse(&content) {
                         *html_store_updater.write().unwrap() = make_html(&new_ast);
                         let mut locked_interp = interp_shared.lock().unwrap();
                         let mut logic_only = Vec::new();
                         for s in &new_ast {
                             match s {
                                 Statement::WindowDecl(_) | Statement::HtmlBlock(_) | Statement::MaterialDecl(_) => {},
                                 _ => logic_only.push(s.clone()),
                             }
                         }
                         if !logic_only.is_empty() { let _ = locked_interp.interpret(logic_only); }
                         let _ = webview.evaluate_script("window.location.reload()");
                    }
                }
            }
            Event::UserEvent(SSLUserEvent::Rotate(angle)) => { state.rotate_object(angle); }
            _ => (),
        }
    });
}
