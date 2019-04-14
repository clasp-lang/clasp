//! Clasp Abstract Syntax Tree (AST)
//!
//! Generated from the parser synthesized by LALRPOP. See `clasp.lalrpop`

/// Clasp programs: sequence of statements
pub struct Program<'input>(pub Vec<Statement<'input>>);

/// Statements within a Clasp program: toplevel directives that return no value
pub enum Statement<'input> {
    /// `use` statements
    Use(UseStatement<'input>),

    /// `message` statements
    Message(MessageStatement),

    /// `struct` statements
    Struct(StructStatement),
}

/// `use` statements
pub struct UseStatement<'input> {
    /// Path components
    path: Vec<Identifier<'input>>
}

/// `message` statements
pub struct MessageStatement {}

/// `struct` statements
pub struct StructStatement {}

/// Identifier names
pub struct Identifier<'input>(&'input str);

