/// Dataflow graph representation
#[derive(Debug, Clone)]
pub struct DataflowGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl DataflowGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        DataflowGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    
    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
    
    /// Get node by ID
    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Topological sort for execution order
    pub fn execution_order(&self) -> Result<Vec<usize>, String> {
        let mut order = Vec::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut in_progress = vec![false; self.nodes.len()];
        
        for node in &self.nodes {
            if !visited[node.id] {
                self.dfs(node.id, &mut visited, &mut in_progress, &mut order)?;
            }
        }
        
        order.reverse();
        Ok(order)
    }
    
    fn dfs(
        &self,
        node_id: usize,
        visited: &mut Vec<bool>,
        in_progress: &mut Vec<bool>,
        order: &mut Vec<usize>,
    ) -> Result<(), String> {
        if in_progress[node_id] {
            return Err("Cycle detected in dataflow graph".to_string());
        }
        
        if visited[node_id] {
            return Ok(());
        }
        
        in_progress[node_id] = true;
        
        // Visit successors
        for edge in &self.edges {
            if edge.from == node_id {
                self.dfs(edge.to, visited, in_progress, order)?;
            }
        }
        
        in_progress[node_id] = false;
        visited[node_id] = true;
        order.push(node_id);
        
        Ok(())
    }
}

/// Node in the dataflow graph
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub node_type: NodeType,
    pub position: (f32, f32), // (x, y) for visual layout
}

/// Node types in dataflow
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Source,      // Data source
    Transform,   // Data transformation
    Sink,        // Data output
    Filter,      // Data filtering
    Map,         // Data mapping
    Reduce,      // Data reduction
}

/// Edge connecting two nodes
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_graph_creation() {
        let mut graph = DataflowGraph::new();
        
        graph.add_node(Node {
            id: 0,
            name: "source".to_string(),
            node_type: NodeType::Source,
            position: (0.0, 0.0),
        });
        
        graph.add_node(Node {
            id: 1,
            name: "sink".to_string(),
            node_type: NodeType::Sink,
            position: (100.0, 0.0),
        });
        
        graph.add_edge(Edge { from: 0, to: 1 });
        
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }
    
    #[test]
    fn test_execution_order() {
        let mut graph = DataflowGraph::new();
        
        // Create linear pipeline: 0 → 1 → 2
        for i in 0..3 {
            graph.add_node(Node {
                id: i,
                name: format!("node{}", i),
                node_type: NodeType::Transform,
                position: (i as f32 * 50.0, 0.0),
            });
        }
        
        graph.add_edge(Edge { from: 0, to: 1 });
        graph.add_edge(Edge { from: 1, to: 2 });
        
        let order = graph.execution_order().unwrap();
        assert_eq!(order, vec![0, 1, 2]);
    }
}
