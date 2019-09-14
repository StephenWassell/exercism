pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        // pub mod attr {
        //     #[derive(Clone, Debug, PartialEq)]
        //     pub struct Attr {
        //         name: String,
        //         value: String,
        //     }

        //     impl Attr {
        //         pub fn new(name: &str, value: &str) -> Self {
        //             Attr {
        //                 name: name.to_string(),
        //                 value: value.to_string(),
        //             }
        //         }
        //     }
        // }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: crate::graph::graph_items::Attrs,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    self.clone()
                }

                pub fn get_attr(&self, name: &str) -> Option<&String> {
                    self.attrs.get(name)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: crate::graph::graph_items::Attrs,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    self.clone()
                }

                pub fn get_attr(&self, name: &str) -> Option<&String> {
                    self.attrs.get(name)
                }
            }
        }

        pub type Attrs = std::collections::HashMap<String, String>;
        pub type Edges = Vec<edge::Edge>;
        pub type Nodes = Vec<node::Node>;
    }

    #[derive(Clone, Debug)]
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

        pub fn with_nodes(&self, nodes: &[graph_items::node::Node]) -> Self {
            let mut ret = self.clone();
            ret.nodes.extend_from_slice(nodes);
            ret
        }

        pub fn with_edges(&self, edges: &[graph_items::edge::Edge]) -> Self {
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

        pub fn get_node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        pub fn get_attr(&self, name: &str) -> Option<&String> {
            self.attrs.get(name)
        }
    }
}
