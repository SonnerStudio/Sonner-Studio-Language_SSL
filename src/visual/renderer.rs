use super::graph::{DataflowGraph, NodeType};
use super::RenderFormat;

/// Live renderer for dataflow graphs
pub struct LiveRenderer {
    format: RenderFormat,
}

impl LiveRenderer {
    /// Create a new renderer
    pub fn new(format: RenderFormat) -> Self {
        LiveRenderer { format }
    }
    
    /// Render graph to string
    pub fn render(&self, graph: &DataflowGraph) -> String {
        match self.format {
            RenderFormat::SVG => self.render_svg(graph),
            RenderFormat::Terminal => self.render_terminal(graph),
            RenderFormat::HTML => self.render_html(graph),
            RenderFormat::PNG => "PNG rendering requires image library".to_string(),
        }
    }
    
    fn render_svg(&self, graph: &DataflowGraph) -> String {
        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" width="800" height="400">"#);
        svg.push('\n');
        
        // Render edges first (so they're behind nodes)
        for edge in &graph.edges {
            if let (Some(from_node), Some(to_node)) = (
                graph.get_node(edge.from),
                graph.get_node(edge.to),
            ) {
                svg.push_str(&format!(
                    r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
                    from_node.position.0 + 50.0,
                    from_node.position.1 + 25.0,
                    to_node.position.0,
                    to_node.position.1 + 25.0
                ));
                svg.push('\n');
            }
        }
        
        // Render nodes
        for node in &graph.nodes {
            let color = match node.node_type {
                NodeType::Source => "#4CAF50",
                NodeType::Sink => "#F44336",
                NodeType::Transform => "#2196F3",
                NodeType::Filter => "#FF9800",
                NodeType::Map => "#9C27B0",
                NodeType::Reduce => "#00BCD4",
            };
            
            svg.push_str(&format!(
                r#"  <rect x="{}" y="{}" width="100" height="50" fill="{}" stroke="black" stroke-width="2" rx="5"/>"#,
                node.position.0, node.position.1, color
            ));
            svg.push('\n');
            
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" fill="white" font-size="12" text-anchor="middle">{}</text>"#,
                node.position.0 + 50.0,
                node.position.1 + 30.0,
                node.name
            ));
            svg.push('\n');
        }
        
        svg.push_str("</svg>");
        svg
    }
    
    fn render_terminal(&self, graph: &DataflowGraph) -> String {
        let mut output = String::new();
        
        // ASCII art representation
        for node in &graph.nodes {
            let icon = match node.node_type {
                NodeType::Source => "📥",
                NodeType::Sink => "📤",
                NodeType::Transform => "⚙️",
                NodeType::Filter => "🔍",
                NodeType::Map => "🗺️",
                NodeType::Reduce => "📊",
            };
            
            output.push_str(&format!("[{}] {}", icon, node.name));
            
            // Check if there's an edge from this node
            if graph.edges.iter().any(|e| e.from == node.id) {
                output.push_str(" → ");
            } else {
                output.push_str("\n");
            }
        }
        
        output
    }
    
    fn render_html(&self, graph: &DataflowGraph) -> String {
        let svg = self.render_svg(graph);
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Dataflow Pipeline</title>
    <style>
        body {{ font-family: Arial, sans-serif; padding: 20px; }}
        svg {{ border: 1px solid #ccc; }}
    </style>
</head>
<body>
    <h1>Dataflow Pipeline</h1>
    {}
</body>
</html>"#,
            svg
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual::graph::Node;
    
    #[test]
    fn test_terminal_rendering() {
        let mut graph = DataflowGraph::new();
        graph.add_node(Node {
            id: 0,
            name: "source".to_string(),
            node_type: NodeType::Source,
            position: (0.0, 0.0),
        });
        
        let renderer = LiveRenderer::new(RenderFormat::Terminal);
        let output = renderer.render(&graph);
        
        assert!(output.contains("source"));
        assert!(output.contains("📥"));
    }
}
