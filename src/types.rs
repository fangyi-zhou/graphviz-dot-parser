use std::collections::HashMap;
pub struct GraphAST {
    pub is_strict: bool,
    pub is_directed: bool,
    pub id: Option<String>,
    pub stmt: Vec<Stmt>,
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
