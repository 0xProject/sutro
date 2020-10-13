use logos::Logos;

#[derive(Clone, PartialEq, Logos, Debug)]
pub enum Token {
    // Keywords
    #[token("object")]
    Object,
    #[token("code")]
    Code,
    #[token("data")]
    Data,
    #[token("function")]
    Function,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("for")]
    For,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("leave")]
    Leave,

    // Syntax
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token(r"{")]
    BraceOpen,
    #[token(r"}")]
    BraceClose,
    #[token(",")]
    Comma,
    #[token(":=")]
    Assign,
    #[token("->")]
    Returns,
    #[token(":")]
    Typed,

    // Identifiers
    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9.]*")]
    Identifiers,

    // Literals
    #[regex(r"0x[0-9a-fA-F]+")]
    LiteralNumberHex,
    #[regex(r"[0-9]+")]
    LiteralNumberDecimal,
    #[token("true")]
    LiteralTrue,
    #[token("false")]
    LiteralFalse,
    #[regex(r#""([^"\r\n\\]|\\.)*""#)]
    LiteralString,
    #[regex(r#"hex"([0-9a-fA-F][0-9a-fA-F])*""#)]
    LiteralStringHex,

    // Comments
    #[regex(r#"//[^\n]*"#)]
    LineComment,
    #[regex(r#"/\*.*\*/"#)]
    BlockComment,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let example = include_str!("example.yul");
        let mut lexer = Token::lexer(example);

        for (token, span) in lexer.spanned() {
            println!("{:?}: \"{}\"", token, &example[span]);
        }
    }
}
