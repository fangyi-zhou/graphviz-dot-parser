use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let filename = args.next().unwrap();
    let input = fs::read_to_string(filename).unwrap();
    match graphviz_dot_parser::parse(input.as_str()) {
        Ok(graph) => {
            if graph.is_directed {
                println!("{:?}", graph.to_directed_graph().unwrap())
            } else {
                println!("{:?}", graph.to_undirected_graph().unwrap())
            }
        }
        Err(err) => println!("Unable to parse, error: {}", err),
    }
}
