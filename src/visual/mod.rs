// Visual Reactive Programming Module
// Enables drag-and-drop dataflow pipelines

pub mod dsl;
pub mod graph;
pub mod renderer;

pub use dsl::{VisualDSL, DataflowPipeline};
pub use graph::{DataflowGraph, Node, Edge};
pub use renderer::LiveRenderer;

/// Visual programming configuration
#[derive(Debug, Clone)]
pub struct VisualConfig {
    /// Enable live preview
    pub live_preview: bool,
    
    /// Auto-layout nodes
    pub auto_layout: bool,
    
    /// Render to format (SVG, PNG, etc.)
    pub render_format: RenderFormat,
}

#[derive(Debug, Clone)]
pub enum RenderFormat {
    SVG,
    PNG,
    HTML,
    Terminal, // ASCII art for CLI
}

impl Default for VisualConfig {
    fn default() -> Self {
        VisualConfig {
            live_preview: true,
            auto_layout: true,
            render_format: RenderFormat::SVG,
        }
    }
}
