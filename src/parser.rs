extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, opt};
use nom::IResult;

fn parse_strict(s: &str) -> IResult<&str, bool> {
    map(opt(tag_no_case("strict")), |tag| tag.is_some())(s)
}

fn parse_directed(s: &str) -> IResult<&str, bool> {
    let indirected = map(tag_no_case("graph"), |_| false);
    let directed = map(tag_no_case("digraph"), |_| true);
    alt((directed, indirected))(s)
}
