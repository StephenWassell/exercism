pub mod graph_items;

use std::collections::HashMap;
use graph_items::edge::Edge;
use graph_items::node::Node;

pub type Attrs = std::collections::HashMap<String, String>;
pub type Edges = Vec<Edge>;
pub type Nodes = Vec<Node>;

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub attrs: Attrs,
    pub edges: Edges,
    pub nodes: Nodes,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            attrs: HashMap::new(),
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn with_nodes(&self, nodes: &[Node]) -> Self {
        let mut ret = self.clone();
        ret.nodes.extend_from_slice(nodes);
        ret
    }

    pub fn with_edges(&self, edges: &[Edge]) -> Self {
        let mut ret = self.clone();
        ret.edges.extend_from_slice(edges);
        ret
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        let mut ret = self.clone();
        for (name, value) in attrs {
            ret.attrs.insert(name.to_string(), value.to_string());
        }
        ret
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name == name)
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        Some(&self.attrs.get(name)?)
    }
}
