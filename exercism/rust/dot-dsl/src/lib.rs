
pub mod graph {
    use std::collections::HashMap;
    use graph_items::node::Node;
    use graph_items::edge::Edge;

    pub mod graph_items {
        pub mod edge {
            #[derive(Debug,PartialEq,Eq,Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge { 
                        from: from.to_string(),
                        to: to.to_string(),
                    }
                }

                pub fn with_attrs(self, _attributes: &[(&str, &str)]) -> Self {
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug,PartialEq,Eq,Clone)]
            pub struct Node {
                pub name: String,
                pub attributes: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attributes: HashMap::<String,String>::new()
                    }
                }

                pub fn with_attrs(mut self, _attributes: &[(&str, &str)]) -> Self {
                    self.attributes = _attributes
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                    self
                }

                pub fn get_attr(&self, _name: &str) -> Option<&str> {
                    self.attributes.get(_name).map(|s| s.as_str())
                }
            }
        }
    }

    #[derive(Debug,PartialEq,Eq,Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new()
            }
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_attrs(mut self, _attributes: &[(&str, &str)]) -> Self {
            self.attrs = _attributes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }

        pub fn get_node(self, _name: &str) -> Option<Node> {
            for node in self.nodes {
                if node.name == _name.to_string() {
                    return Some(node)
                }
            }

            None
        }
    }
}
