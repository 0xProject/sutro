use hex;
use logos::{Lexer, Logos};
use zkp_u256::{One, Zero, U256};

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
    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9.]*", |lexer| lexer.slice().to_string())]
    Identifier(String),

    // Literals
    #[token("true", |_| U256::one())]
    #[token("false", |_| U256::zero())]
    #[regex(r"0x[0-9a-fA-F]+", |lexer| U256::from_hex_str(&lexer.slice()[2..]))]
    #[regex(r"[0-9]+", |lexer| U256::from_decimal_str(lexer.slice()))]
    Literal(U256),

    #[regex(r#""([^"\r\n\\]|\\.)*""#, string_literal)]
    LiteralString(String),
    #[regex(r#"hex"([0-9a-fA-F][0-9a-fA-F])*""#, hex_data)]
    LiteralStringHex(Vec<u8>),

    // Ignored syntax
    #[regex(r#"//[^\n]*"#, logos::skip)]
    LineComment,
    // TODO: Allow * in block comment when not followed by /
    // See <https://stackoverflow.com/questions/16160190/regular-expression-to-find-c-style-block-comments>
    #[regex(r#"/\*[^*]*\*/"#, logos::skip)]
    BlockComment,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

fn string_literal(lexer: &mut Lexer<Token>) -> String {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    slice.to_string()
}

fn hex_data(lexer: &mut Lexer<Token>) -> Result<Vec<u8>, hex::FromHexError> {
    let slice = lexer.slice();
    let slice = &slice[4..slice.len() - 1];
    hex::decode(slice)
}

pub fn tokenize(string: &str) -> Lexer<Token> {
    Token::lexer(string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let example = include_str!("erc20.yul");
        let tokens = Token::lexer(example).collect::<Vec<_>>();
        dbg!(tokens);
    }
}
