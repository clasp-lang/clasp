/// Tokens in the Clasp grammar
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// `*`
    Star,

    /// `,`
    Comma,

    /// `.`
    Dot,

    /// `;`
    Semi,

    /// `:`
    Colon,

    /// `{`
    CurlyBraceOpen,

    /// `}`
    CurlyBraceClose,

    /// `<[<]`
    LessFollowedByLess,

    /// `<[]`
    LessFollowedByOther,

    /// `>[>]`
    GreaterFollowedByGreater,

    /// `>[]`
    GreaterFollowedByOther,

    /// `message`
    KeywordMessage,

    /// `type`
    KeywordStruct,

    /// `use`
    KeywordUse,
}
