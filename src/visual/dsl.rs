use crate::ast::{Expression, Statement};
use crate::parser::Parser;
use super::graph::{DataflowGraph, Node, NodeType, Edge};

/// Visual DSL for dataflow programming
pub struct VisualDSL {
    graph: DataflowGraph,
}

impl VisualDSL {
    /// Create a new visual DSL
    pub fn new() -> Self {
        VisualDSL {
            graph: DataflowGraph::new(),
        }
    }
    
    /// Parse visual DSL block from source
    pub fn parse(&mut self, source: &str) -> Result<DataflowPipeline, String> {
        // Extract visual { } block
        let visual_block = self.extract_visual_block(source)?;
        
        // Parse dataflow pipeline
        let pipeline = self.parse_pipeline(&visual_block)?;
        
        Ok(pipeline)
    }
    
    fn extract_visual_block(&self, source: &str) -> Result<String, String> {
        // Find visual { ... } block
        if let Some(start) = source.find("visual {") {
            let after_open = &source[start + 8..];
            if let Some(end) = self.find_matching_brace(after_open) {
                return Ok(after_open[..end].trim().to_string());
            }
        }
        Err("No visual block found".to_string())
    }
    
    fn find_matching_brace(&self, s: &str) -> Option<usize> {
        let mut depth = 1;
        for (i, c) in s.chars().enumerate() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        None
    }
    
    pub fn parse_pipeline(&mut self, block: &str) -> Result<DataflowPipeline, String> {
        // Parse pipeline: source → transform → sink
        let parts: Vec<&str> = block.split('→').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Err("Empty pipeline".to_string());
        }
        
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        
        // Create nodes for each stage
        for (i, part) in parts.iter().enumerate() {
            let node_type = if i == 0 {
                NodeType::Source
            } else if i == parts.len() - 1 {
                NodeType::Sink
            } else {
                NodeType::Transform
            };
            
            let node = Node {
                id: i,
                name: part.to_string(),
                node_type,
                position: (i as f32 * 100.0, 50.0),
            };
            
            nodes.push(node);
            
            // Create edge to next node
            if i > 0 {
                edges.push(Edge {
                    from: i - 1,
                    to: i,
                });
            }
        }
        
        // Update graph
        self.graph.nodes = nodes.clone();
        self.graph.edges = edges.clone();
        
        Ok(DataflowPipeline {
            nodes,
            edges,
            compiled_code: self.compile_to_code(&parts)?,
        })
    }
    
    fn compile_to_code(&self, stages: &[&str]) -> Result<String, String> {
        // Transform pipeline to SSL code
        let mut code = String::new();
        
        if stages.is_empty() {
            return Ok(code);
        }
        
        // Start with source
        code.push_str(&format!("let data = {}\n", stages[0]));
        
        // Apply transformations (only if there are more than 2 stages)
        if stages.len() > 2 {
            for stage in stages.iter().skip(1).take(stages.len() - 2) {
                code.push_str(&format!("let data = data.{}()\n", stage));
            }
        }
        
        // End with sink
        if stages.len() > 1 {
            code.push_str(&format!("data.{}()\n", stages[stages.len() - 1]));
        }
        
        Ok(code)
    }
    
    /// Get the dataflow graph
    pub fn graph(&self) -> &DataflowGraph {
        &self.graph
    }
}

/// Dataflow pipeline representation
#[derive(Debug, Clone)]
pub struct DataflowPipeline {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub compiled_code: String,
}

impl DataflowPipeline {
    /// Execute the pipeline
    pub fn execute(&self) -> Result<String, String> {
        // Execute the compiled code
        // (Would integrate with interpreter in full implementation)
        Ok(self.compiled_code.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visual_dsl_parsing() {
        let mut dsl = VisualDSL::new();
        let source = r#"
            visual {
                load_data → filter → map → collect
            }
        "#;
        
        let pipeline = dsl.parse(source).unwrap();
        assert_eq!(pipeline.nodes.len(), 4);
        assert_eq!(pipeline.edges.len(), 3);
    }
    
    #[test]
    fn test_pipeline_compilation() {
        let mut dsl = VisualDSL::new();
        let source = r#"
            visual {
                data_source → transform → output
            }
        "#;
        
        let pipeline = dsl.parse(source).unwrap();
        assert!(pipeline.compiled_code.contains("data_source"));
        assert!(pipeline.compiled_code.contains("transform"));
    }
}
