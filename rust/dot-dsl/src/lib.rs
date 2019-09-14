pub mod graph {
    pub mod graph_items {
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
    }

    pub struct Graph {
        pub edges: Vec<graph_items::edge::Edge>,
        pub nodes: Vec<graph_items::node::Node>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
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
