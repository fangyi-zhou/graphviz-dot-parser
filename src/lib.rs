mod parser;
pub mod types;

use nom::error::Error;
use types::GraphAST;

pub fn parse(s: &str) -> Result<GraphAST, Error<&str>> {
    parser::parse(s)
}
