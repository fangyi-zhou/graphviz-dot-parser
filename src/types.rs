use petgraph::EdgeType;
use petgraph::Graph;
use std::collections::HashMap;

pub struct GraphAST {
    pub is_strict: bool,
    pub is_directed: bool,
    pub id: Option<String>,
    pub stmt: Vec<Stmt>,
}

impl GraphAST {
    fn to_graph_internal<Ty: EdgeType>(&self, g: &mut Graph<String, (), Ty>) {
        let mut nodes = HashMap::new();
        for stmt in &self.stmt {
            match stmt {
                Stmt::Node(n, _) => {
                    let idx = g.add_node(n.clone());
                    nodes.insert(n, idx);
                }
                Stmt::Edge(n1, n2, _) => {
                    g.add_edge(*nodes.get(&n1).unwrap(), *nodes.get(&n2).unwrap(), ());
                }
                _ => {}
            }
        }
    }

    pub fn to_directed_graph(&self) -> Option<Graph<String, ()>> {
        if self.is_directed {
            let mut g = Graph::new();
            self.to_graph_internal(&mut g);
            Some(g)
        } else {
            None
        }
    }

    pub fn to_undirected_graph(&self) -> Option<Graph<String, (), petgraph::Undirected>> {
        if !self.is_directed {
            let mut g = Graph::new_undirected();
            self.to_graph_internal(&mut g);
            Some(g)
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum AttributeType {
    Graph,
    Node,
    Edge,
}

pub type Attributes = HashMap<String, String>;

#[derive(Eq, PartialEq, Debug)]
pub enum Stmt {
    // Many features are currently not supported
    Node(String, Attributes),
    Edge(String, String, Attributes),
    Attr(AttributeType, Attributes),
    Assign(String, String),
    SubGraph(Option<String>, Vec<Stmt>),
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use std::collections::HashMap;
    #[test]
    fn test_petgraph_conversion_directed() {
        let g = GraphAST {
            is_strict: false,
            is_directed: true,
            id: None,
            stmt: vec![
                Stmt::Node(String::from("1"), HashMap::new()),
                Stmt::Node(String::from("2"), HashMap::new()),
                Stmt::Edge(String::from("1"), String::from("2"), HashMap::new()),
            ],
        };
        let graph = g.to_undirected_graph();
        assert_eq!(graph.is_none(), true);
        let graph = g.to_directed_graph();
        assert_eq!(graph.is_some(), true);
        let graph = graph.unwrap();
        assert_eq!(graph.is_directed(), true);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_petgraph_conversion_undirected() {
        let g = GraphAST {
            is_strict: false,
            is_directed: false,
            id: None,
            stmt: vec![
                Stmt::Node(String::from("1"), HashMap::new()),
                Stmt::Node(String::from("2"), HashMap::new()),
                Stmt::Edge(String::from("1"), String::from("2"), HashMap::new()),
            ],
        };
        let graph = g.to_directed_graph();
        assert_eq!(graph.is_none(), true);
        let graph = g.to_undirected_graph();
        assert_eq!(graph.is_some(), true);
        let graph = graph.unwrap();
        assert_eq!(graph.is_directed(), false);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }
}
