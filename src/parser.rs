extern crate nom;

use crate::types::GraphAST;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, space0};
use nom::combinator::{map, opt};
use nom::IResult;

fn parse_strict(s: &str) -> IResult<&str, bool> {
    let (s, _) = space0(s)?;
    let (s, out) = map(opt(tag_no_case("strict")), |tag| tag.is_some())(s)?;
    let (s, _) = space0(s)?;
    Ok((s, out))
}

fn parse_directed(s: &str) -> IResult<&str, bool> {
    let (s, _) = space0(s)?;
    let indirected = map(tag_no_case("graph"), |_| false);
    let directed = map(tag_no_case("digraph"), |_| true);
    let (s, out) = alt((directed, indirected))(s)?;
    let (s, _) = space0(s)?;
    Ok((s, out))
}

fn parse_graph(s: &str) -> IResult<&str, GraphAST> {
    let (s, is_strict) = parse_strict(s)?;
    let (s, is_directed) = parse_directed(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = char('}')(s)?;
    let graph = GraphAST {
        is_strict,
        is_directed,
        id: None,
        stmt: vec![],
    };
    Ok((s, graph))
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_parse_empty_graph_1() {
        let input = "strict graph {}";
        if let Ok((rest, graph)) = crate::parser::parse_graph(input) {
            assert_eq!(rest, "");
            assert_eq!(graph.is_strict, true);
            assert_eq!(graph.is_directed, false);
        } else {
            panic!();
        }
    }
    #[test]
    fn can_parse_empty_graph_2() {
        let input = "graph {}";
        if let Ok((rest, graph)) = crate::parser::parse_graph(input) {
            assert_eq!(rest, "");
            assert_eq!(graph.is_strict, false);
            assert_eq!(graph.is_directed, false);
        } else {
            panic!();
        }
    }
    #[test]
    fn can_parse_empty_graph_3() {
        let input = "strict digraph {}";
        if let Ok((rest, graph)) = crate::parser::parse_graph(input) {
            assert_eq!(rest, "");
            assert_eq!(graph.is_strict, true);
            assert_eq!(graph.is_directed, true);
        } else {
            panic!();
        }
    }
    #[test]
    fn can_parse_empty_graph_4() {
        let input = "digraph {}";
        if let Ok((rest, graph)) = crate::parser::parse_graph(input) {
            assert_eq!(rest, "");
            assert_eq!(graph.is_strict, false);
            assert_eq!(graph.is_directed, true);
        } else {
            panic!();
        }
    }
}
