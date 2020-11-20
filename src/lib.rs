use std::collections::HashMap;

mod parser;

pub struct GraphAST {
    pub is_strict: bool,
    pub is_directed: bool,
    pub id: String,
    pub stmt: Vec<Stmt>,
}

pub enum AttributeType {
    Graph,
    Node,
    Edge,
}

pub type Attributes = HashMap<String, String>;

pub enum Stmt {
    // port is unsupported
    Node(String, Attributes),
    Edge(String, String),
    Attr(AttributeType, Attributes),
    Assign(String, String),
    SubGraph(Option<String>, Vec<Stmt>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
