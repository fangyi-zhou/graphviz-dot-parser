extern crate nom;

use crate::types::{Attributes, GraphAST, Stmt};
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, tag, tag_no_case};
use nom::character::complete::{char, digit0, digit1, multispace0, none_of, one_of, satisfy};
use nom::combinator::{eof, map, opt, recognize, value};
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

fn skip_space_and_comments(s: &str) -> IResult<&str, ()> {
    let (s, _) = multispace0(s)?;
    Ok((s, ()))
}

fn parse_strict(s: &str) -> IResult<&str, bool> {
    let (s, _) = skip_space_and_comments(s)?;
    let (s, out) = map(opt(tag_no_case("strict")), |tag| tag.is_some())(s)?;
    let (s, _) = skip_space_and_comments(s)?;
    Ok((s, out))
}

fn parse_directed(s: &str) -> IResult<&str, bool> {
    let (s, _) = skip_space_and_comments(s)?;
    let indirected = value(false, tag_no_case("graph"));
    let directed = value(true, tag_no_case("digraph"));
    let (s, out) = alt((directed, indirected))(s)?;
    let (s, _) = skip_space_and_comments(s)?;
    Ok((s, out))
}

fn parse_attributes(s: &str) -> IResult<&str, Attributes> {
    let (s, _) = skip_space_and_comments(s)?;
    let a_list = many0(terminated(
        map(tuple((parse_id, char('='), parse_id)), |(fst, _, snd)| {
            (fst, snd)
        }),
        opt(terminated(one_of(",;"), skip_space_and_comments)),
    ));
    let (s, attr_list) = many0(preceded(char('['), terminated(a_list, char(']'))))(s)?;
    Ok((s, attr_list.concat()))
}

fn parse_node_statement(s: &str) -> IResult<&str, Stmt> {
    let (s, id) = parse_id(s)?;
    let (s, attrs) = parse_attributes(s)?;
    Ok((s, Stmt::Node(id, attrs)))
}

fn parse_edge_statement<'a>(is_directed: bool) -> impl Fn(&'a str) -> IResult<&'a str, Stmt> {
    let parse_edge_op = if is_directed { tag("->") } else { tag("--") };
    move |s| {
        let (s, id_from) = parse_id(s)?;
        let (s, _) = skip_space_and_comments(s)?;
        let (s, _) = parse_edge_op(s)?;
        let (s, _) = skip_space_and_comments(s)?;
        let (s, id_to) = parse_id(s)?;
        // TODO: Subgraph
        // TODO: Multiple edges in single statement
        let (s, attrs) = parse_attributes(s)?;
        Ok((s, Stmt::Edge(id_from, id_to, attrs)))
    }
}

fn parse_statement(is_directed: bool) -> impl Fn(&str) -> IResult<&str, Stmt> {
    move |s| {
        let (s, _) = skip_space_and_comments(s)?;
        let (s, stmt) = alt((parse_edge_statement(is_directed), parse_node_statement))(s)?;
        let (s, _) = skip_space_and_comments(s)?;
        let (s, _) = opt(char(';'))(s)?;
        Ok((s, stmt))
    }
}

fn parse_graph(s: &str) -> IResult<&str, GraphAST> {
    let (s, is_strict) = parse_strict(s)?;
    let (s, is_directed) = parse_directed(s)?;
    let (s, id) = opt(parse_id)(s)?;
    let (s, _) = char('{')(s)?;
    let (s, stmt) = many0(parse_statement(is_directed))(s)?;
    let (s, _) = skip_space_and_comments(s)?;
    let (s, _) = char('}')(s)?;
    let (s, _) = skip_space_and_comments(s)?;
    let (s, _) = eof(s)?;
    let graph = GraphAST {
        is_strict,
        is_directed,
        id,
        stmt,
    };
    Ok((s, graph))
}

pub fn parse(s: &str) -> Result<GraphAST, nom::error::Error<&str>> {
    nom::Finish::finish(parse_graph(s)).map(|(_, g)| g)
}

fn parse_id(s: &str) -> IResult<&str, String> {
    // Any string of alphabetic ([a-zA-Z\200-\377]) characters, underscores ('_') or digits ([0-9]), not beginning with a digit;
    let non_digits = satisfy(|c| {
        (c == '_')
            || ('a'..='z').contains(&c)
            || ('A'..='Z').contains(&c)
            || (char::from(0o200)..=char::from(0o377)).contains(&c)
    });
    let all_chars = satisfy(|c| {
        (c == '_')
            || ('a'..='z').contains(&c)
            || ('A'..='Z').contains(&c)
            || ('0'..='9').contains(&c)
            || (char::from(0o200)..=char::from(0o377)).contains(&c)
    });
    let id_string = map(recognize(pair(non_digits, many0(all_chars))), |s| {
        String::from(s)
    });

    // a numeral [-]?(.[0-9]+ | [0-9]+(.[0-9]*)? );
    let id_numeral = map(
        recognize(pair(
            opt(char('-')),
            alt((
                recognize(pair(char('.'), digit1)),
                recognize(pair(digit1, opt(tuple((char('.'), digit0))))),
            )),
        )),
        String::from,
    );

    // any double-quoted string ("...") possibly containing escaped quotes (\")1;
    let id_quoted = preceded(
        char('\"'),
        terminated(
            map(
                many0(escaped_transform(
                    none_of("\"\\"),
                    '\\',
                    value('\"', char('\"')),
                )),
                |v| v.into_iter().collect(),
            ),
            char('\"'),
        ),
    );

    // HTML: Not supported
    let (s, id) = alt((id_string, id_numeral, id_quoted))(s)?;
    let (s, _) = skip_space_and_comments(s)?;
    Ok((s, id))
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    fn parse(input: &str) -> (&str, GraphAST) {
        match crate::parser::parse_graph(input) {
            Ok((rest, graph)) => (rest, graph),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn can_parse_empty_graph_1() {
        let input = "strict graph {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.is_strict, true);
        assert_eq!(graph.is_directed, false);
    }
    #[test]
    fn can_parse_empty_graph_2() {
        let input = "graph {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.is_strict, false);
        assert_eq!(graph.is_directed, false);
    }
    #[test]
    fn can_parse_empty_graph_3() {
        let input = "strict digraph {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.is_strict, true);
        assert_eq!(graph.is_directed, true);
    }
    #[test]
    fn can_parse_empty_graph_4() {
        let input = "digraph {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.is_strict, false);
        assert_eq!(graph.is_directed, true);
    }

    #[test]
    fn can_parse_empty_graph_with_id() {
        let input = "graph g {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.id, Some(String::from("g")));
    }

    #[test]
    fn can_parse_numeric_id() {
        let input = "graph 2.34 {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.id, Some(String::from("2.34")));
    }

    #[test]
    fn can_parse_quoted_id() {
        let input = "graph \"2.34\" {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.id, Some(String::from("2.34")));
    }

    #[test]
    fn can_parse_quoted_id_with_space() {
        let input = "graph \"2 . 34\" {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.id, Some(String::from("2 . 34")));
    }

    #[test]
    fn can_parse_quoted_id_with_escape() {
        let input = "graph \"2\\\"34\" {}";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.id, Some(String::from("2\"34")));
    }

    #[test]
    fn can_parse_graph_with_nodes() {
        let input = "graph {
            1;
        }";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(graph.stmt, vec![Stmt::Node(String::from("1"), vec![])])
    }

    #[test]
    fn can_parse_graph_with_nodes_and_edges() {
        let input = "graph {
            1;
            2;
            1 -- 2;
        }";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(
            graph.stmt,
            vec![
                Stmt::Node(String::from("1"), vec![]),
                Stmt::Node(String::from("2"), vec![]),
                Stmt::Edge(String::from("1"), String::from("2"), vec![]),
            ]
        )
    }

    #[test]
    fn can_parse_graph_with_nodes_and_edges_directed() {
        let input = "digraph {
            1;
            2;
            1 -> 2;
        }";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(
            graph.stmt,
            vec![
                Stmt::Node(String::from("1"), vec![]),
                Stmt::Node(String::from("2"), vec![]),
                Stmt::Edge(String::from("1"), String::from("2"), vec![]),
            ]
        )
    }

    #[test]
    fn can_parse_graph_without_semicolon() {
        let input = "digraph {
            1
            2
            1 -> 2
        }";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(
            graph.stmt,
            vec![
                Stmt::Node(String::from("1"), vec![]),
                Stmt::Node(String::from("2"), vec![]),
                Stmt::Edge(String::from("1"), String::from("2"), vec![]),
            ]
        )
    }

    #[test]
    fn can_parse_graph_without_linebreak() {
        let input = "digraph {
            1 2 1 -> 2
        }";
        let (rest, graph) = parse(input);
        assert_eq!(rest, "");
        assert_eq!(
            graph.stmt,
            vec![
                Stmt::Node(String::from("1"), vec![]),
                Stmt::Node(String::from("2"), vec![]),
                Stmt::Edge(String::from("1"), String::from("2"), vec![]),
            ]
        )
    }
}
