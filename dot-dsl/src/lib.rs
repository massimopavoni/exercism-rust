macro_rules! dot_struct_with_attributes {
    ($name: ident, $($field: ident: $type: ty),*) => {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $name {
            $(pub $field: $type,)*
            pub attrs: HashMap<String, String>,
        }

        impl $name {
            pub fn attr(&self, key: &str) -> Option<&str> {
                self.attrs.get(key).map(|s| s.as_str())
            }

            pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                self.attrs = attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
                self
            }
        }
    };
}

pub mod graph {
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;

    dot_struct_with_attributes!(Graph, nodes: Vec<Node>, edges: Vec<Edge>);

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.id == id)
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            dot_struct_with_attributes!(Edge, node1: String, node2: String);

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Self {
                        node1: node1.to_owned(),
                        node2: node2.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            dot_struct_with_attributes!(Node, id: String);

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_owned(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }
    }
}
