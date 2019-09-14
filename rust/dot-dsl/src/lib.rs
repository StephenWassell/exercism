pub mod graph {
	use std::collections::HashMap;

    pub mod graph_items {
        pub mod attr {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Attr {
                name: String,
                value: String,
            }

            impl Attr {
                pub fn new(name: &str, value: &str) -> Self {
                    Attr {
                        name: name.to_string(),
                        value: value.to_string(),
                    }
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                    }
                }
            }
        }

        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                name: String,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                    }
                }
            }
    	}

        pub type Attrs = std::collections::HashMap<String, String>;
        pub type Edges = Vec<edge::Edge>;
        pub type Nodes = Vec<node::Node>;
    }

    pub struct Graph {
        pub attrs: graph_items::Attrs,
        pub edges: graph_items::Edges,
        pub nodes: graph_items::Nodes,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                attrs: HashMap::new(),
                edges: Vec::new(),
                nodes: Vec::new(),
            }
        }

        pub fn with_nodes(&mut self, nodes: &[graph_items::node::Node]) -> &mut Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(&mut self, edges: &[graph_items::edge::Edge]) -> &mut Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn get_node(&mut self, name: &str) -> Option<graph_items::node::Node> {
            None
        }
    }
}
