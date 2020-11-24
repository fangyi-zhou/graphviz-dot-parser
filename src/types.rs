use petgraph::Graph;
use std::collections::HashMap;

pub struct GraphAST {
    pub is_strict: bool,
    pub is_directed: bool,
    pub id: Option<String>,
    pub stmt: Vec<Stmt>,
}

impl GraphAST {
    pub fn to_directed_graph(&self) -> Option<Graph<String, ()>> {
        if self.is_directed {
            let mut g = Graph::new();
            let mut nodes = HashMap::new();
            for stmt in &self.stmt {
                match stmt {
                    Stmt::Node(n, _) => {
                        let idx = g.add_node(n.clone());
                        nodes.insert(n, idx);
                    }
                    Stmt::Edge(n1, n2) => {
                        g.add_edge(*nodes.get(&n1).unwrap(), *nodes.get(&n2).unwrap(), ());
                    }
                    _ => {}
                }
            }
            Some(g)
        } else {
            None
        }
    }

    pub fn to_undirected_graph(&self) -> Option<Graph<String, (), petgraph::Undirected>> {
        if !self.is_directed {
            let mut g = Graph::new_undirected();
            let mut nodes = HashMap::new();
            for stmt in &self.stmt {
                match stmt {
                    Stmt::Node(n, _) => {
                        let idx = g.add_node(n.clone());
                        nodes.insert(n, idx);
                    }
                    Stmt::Edge(n1, n2) => {
                        g.add_edge(*nodes.get(&n1).unwrap(), *nodes.get(&n2).unwrap(), ());
                    }
                    _ => {}
                }
            }
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
    // port is unsupported
    Node(String, Attributes),
    Edge(String, String),
    Attr(AttributeType, Attributes),
    Assign(String, String),
    SubGraph(Option<String>, Vec<Stmt>),
}
