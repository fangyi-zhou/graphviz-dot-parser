extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::space0;
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
