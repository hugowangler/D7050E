use super::ParseError;
use crate::ast::Node;

pub fn parse(s: String) -> Result<Vec<Box<Node>>, ParseError> {
    let result = crate::parse::grammar::ProgramParser::new().parse(&s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError {
            message: e.to_string(),
        }),
    };
}
