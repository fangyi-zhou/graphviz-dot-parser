extern crate nom;

use crate::types::GraphAST;
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, tag_no_case};
use nom::character::complete::{char, digit0, digit1, none_of, satisfy, space0};
use nom::combinator::{map, opt, recognize, value};
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

fn parse_strict(s: &str) -> IResult<&str, bool> {
    let (s, _) = space0(s)?;
    let (s, out) = map(opt(tag_no_case("strict")), |tag| tag.is_some())(s)?;
    let (s, _) = space0(s)?;
    Ok((s, out))
}

fn parse_directed(s: &str) -> IResult<&str, bool> {
    let (s, _) = space0(s)?;
    let indirected = value(false, tag_no_case("graph"));
    let directed = value(true, tag_no_case("digraph"));
    let (s, out) = alt((directed, indirected))(s)?;
    let (s, _) = space0(s)?;
    Ok((s, out))
}

fn parse_graph(s: &str) -> IResult<&str, GraphAST> {
    let (s, is_strict) = parse_strict(s)?;
    let (s, is_directed) = parse_directed(s)?;
    let (s, id) = opt(parse_id)(s)?;
    let (s, _) = char('{')(s)?;
    let (s, _) = char('}')(s)?;
    let graph = GraphAST {
        is_strict,
        is_directed,
        id,
        stmt: vec![],
    };
    Ok((s, graph))
}

fn parse_id(s: &str) -> IResult<&str, String> {
    // Any string of alphabetic ([a-zA-Z\200-\377]) characters, underscores ('_') or digits ([0-9]), not beginning with a digit;
    let non_digits = satisfy(|c| {
        (c == '_')
            || (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= char::from(0o200) && c <= char::from(0o377))
    });
    let all_chars = satisfy(|c| {
        (c == '_')
            || (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= '0' && c <= '9')
            || (c >= char::from(0o200) && c <= char::from(0o377))
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
    let (s, _) = space0(s)?;
    Ok((s, id))
}

#[cfg(test)]
mod tests {
    use crate::types::GraphAST;
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
}
