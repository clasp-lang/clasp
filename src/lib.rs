//! Clasp: A declarative logic-based programming language for
//! cryptographically authenticating and verifying structured data.
//!
//! Inspired by Datalog-like authorization languages such as Binder and
//! logic-based credential formats like Macaroons.

pub mod ast;
pub mod token;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/clasp.rs");

/// Parse errors as omitted by our LALRPOP parser
type ParseError = lalrpop_util::ParseError<(), token::Token, &'static str>;

/// Parse a Clasp program with the parser
pub fn parse(program: &str) -> Result<ast::Program, ParseError> {
    parser::ProgramParser::parse(program)
}
