//! Simple recursive descent parser for Yul
// TODO: Typed identifier support
// TODO: Pass `libyul`s tests
// See <https://github.com/ethereum/solidity/tree/develop/test/libyul/yulSyntaxTests>

use super::{ast, tokenize, Token};
use crate::require;
use thiserror::Error;
use zkp_u256::U256;

/// Parser errors
// TODO: Add source location
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token.")]
    UnexpectedToken,
    #[error("String literal to large")]
    StringToLarge,
    #[error("Unexpected end of file.")]
    UnexpectedEof,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse_file(string: &str) -> Result<ast::SourceFile> {
    let tokens = tokenize(string).collect::<Vec<_>>();
    let mut parser = Parser(&tokens);
    parser.parse_file()
}

pub fn parse_object(string: &str) -> Result<ast::Object> {
    let tokens = tokenize(string).collect::<Vec<_>>();
    let mut parser = Parser(&tokens);
    parser.parse_object()
}

pub fn parse_block(string: &str) -> Result<Vec<ast::Statement>> {
    let tokens = tokenize(string).collect::<Vec<_>>();
    let mut parser = Parser(&tokens);
    parser.parse_block()
}

#[derive(Clone, PartialEq, Debug)]
struct Parser<'a>(&'a [Token]);

impl<'a> Parser<'a> {
    /// Tries a parse and rolls back on failure.
    fn try_parse<F, T>(&mut self, parser: F) -> Result<T>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        let mut dummy = self.clone();
        let result = parser(&mut dummy);
        if result.is_ok() {
            *self = dummy;
        }
        result
    }

    fn peek(&self) -> Result<&'a Token> {
        self.0.first().ok_or(Error::UnexpectedEof)
    }

    fn next(&mut self) -> Result<&'a Token> {
        let token = self.peek()?;
        self.0 = &self.0[1..];
        Ok(token)
    }

    fn tag(&mut self, tag: Token) -> Result<()> {
        let token = self.peek()?;
        require!(token == &tag, Error::UnexpectedToken);
        self.0 = &self.0[1..];
        Ok(())
    }

    fn identifier(&mut self) -> Result<&'a String> {
        match self.next()? {
            Token::Identifier(string) => Ok(string),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn literal_string(&mut self) -> Result<&'a String> {
        match self.next()? {
            Token::LiteralString(string) => Ok(string),
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn parse_file(&mut self) -> Result<ast::SourceFile> {
        let mut objects = Vec::new();
        while !self.0.is_empty() {
            objects.push(self.parse_object()?);
        }
        Ok(ast::SourceFile { objects })
    }

    fn parse_object(&mut self) -> Result<ast::Object> {
        self.tag(Token::Object)?;
        let name = self.literal_string()?.clone();
        self.tag(Token::BraceOpen)?;
        self.tag(Token::Code)?;
        let code = self.parse_block()?;
        let mut data = Vec::new();
        while self.tag(Token::BraceClose).is_err() {
            data.push(self.parse_data()?);
        }
        Ok(ast::Object { name, code, data })
    }

    fn parse_block(&mut self) -> Result<Vec<ast::Statement>> {
        self.tag(Token::BraceOpen)?;
        let mut statements = Vec::new();
        while self.tag(Token::BraceClose).is_err() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement> {
        Ok(match self.peek()? {
            Token::BraceOpen => {
                ast::Statement::Block {
                    code: self.parse_block()?,
                }
            }
            Token::Function => {
                self.tag(Token::Function)?;
                let name = self.identifier()?.clone();
                self.tag(Token::ParenOpen)?;
                let arguments = if self.tag(Token::ParenClose).is_ok() {
                    Vec::new()
                } else {
                    let arguments = self.parse_indentifiers()?;
                    self.tag(Token::ParenClose)?;
                    arguments
                };
                let mut returns = Vec::new();
                if self.tag(Token::Returns).is_ok() {
                    returns = self.parse_indentifiers()?;
                }
                let code = self.parse_block()?;
                ast::Statement::FunctionDefinition {
                    name,
                    arguments,
                    returns,
                    code,
                }
            }
            Token::Let => {
                self.tag(Token::Let)?;
                let variables = self.parse_indentifiers()?;
                let value = if let Ok(Token::Assign) = self.peek() {
                    self.tag(Token::Assign)?;
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                ast::Statement::VariableDeclaration { variables, value }
            }
            Token::If => {
                self.tag(Token::If)?;
                let condition = self.parse_expression()?;
                let code = self.parse_block()?;
                ast::Statement::If { condition, code }
            }
            Token::Switch => {
                self.tag(Token::Switch)?;
                let condition = self.parse_expression()?;
                let mut cases = Vec::new();
                while self.tag(Token::Case).is_ok() {
                    let value = if let ast::Expression::Literal(value) = self.parse_expression()? {
                        Ok(value)
                    } else {
                        Err(Error::UnexpectedToken)
                    }?;
                    let code = self.parse_block()?;
                    cases.push(ast::SwitchCase::Case { value, code });
                }
                if self.tag(Token::Default).is_ok() {
                    let code = self.parse_block()?;
                    cases.push(ast::SwitchCase::Default { code });
                }
                ast::Statement::Switch { condition, cases }
            }
            Token::For => {
                self.tag(Token::For)?;
                let pre = self.parse_block()?;
                let condition = self.parse_expression()?;
                let post = self.parse_block()?;
                let body = self.parse_block()?;
                ast::Statement::ForLoop {
                    pre,
                    condition,
                    post,
                    body,
                }
            }
            Token::Break => {
                self.tag(Token::Break)?;
                ast::Statement::Break
            }
            Token::Continue => {
                self.tag(Token::Continue)?;
                ast::Statement::Continue
            }
            Token::Leave => {
                self.tag(Token::Leave)?;
                ast::Statement::Leave
            }
            _ => {
                let expression = self.parse_expression()?;
                if let ast::Expression::Identifier(name) = expression {
                    match self.peek() {
                        Ok(Token::Comma) | Ok(Token::Assign) => {
                            // Expression was actually an assignment.
                            let mut variables = Vec::new();
                            variables.push(name);
                            while self.tag(Token::Comma).is_ok() {
                                variables.push(self.identifier()?.clone());
                            }
                            self.tag(Token::Assign)?;
                            let value = self.parse_expression()?;
                            ast::Statement::Assignment { variables, value }
                        }
                        _ => {
                            ast::Statement::Expression {
                                expression: ast::Expression::Identifier(name),
                            }
                        }
                    }
                } else {
                    ast::Statement::Expression { expression }
                }
            }
        })
    }

    fn parse_indentifiers(&mut self) -> Result<Vec<String>> {
        let mut result = Vec::new();
        result.push(self.identifier()?.clone());
        while self.tag(Token::Comma).is_ok() {
            result.push(self.identifier()?.clone());
        }
        Ok(result)
    }

    fn parse_data(&mut self) -> Result<ast::ObjectData> {
        match self.peek()? {
            Token::Data => {
                self.tag(Token::Data)?;
                let name = self.literal_string()?.clone();
                let value = match self.next()? {
                    Token::LiteralStringHex(value) => Ok(value.clone()),
                    // String slices are always UTF-8
                    // See <https://doc.rust-lang.org/stable/std/primitive.str.html#method.as_bytes>
                    Token::LiteralString(string) => Ok(string.as_bytes().to_vec()),
                    _ => Err(Error::UnexpectedToken),
                }?;
                Ok(ast::ObjectData::Literal { name, value })
            }
            Token::Object => {
                let object = self.parse_object()?;
                Ok(ast::ObjectData::Object(object))
            }
            _ => Err(Error::UnexpectedToken),
        }
    }

    fn parse_expression(&mut self) -> Result<ast::Expression> {
        match self.peek()? {
            Token::Literal(value) => {
                self.tag(Token::Literal(value.clone()))?;
                Ok(ast::Expression::Literal(value.clone()))
            }
            Token::LiteralString(_) => {
                // Convert to bytes32
                let value = self.literal_string()?;
                let value = string_to_value(value)?;
                Ok(ast::Expression::Literal(value))
            }
            Token::Identifier(_) => {
                let name = self.identifier()?.clone();
                if let Ok(Token::ParenOpen) = self.peek() {
                    // Function call
                    self.tag(Token::ParenOpen)?;
                    if self.tag(Token::ParenClose).is_ok() {
                        // without arguments
                        Ok(ast::Expression::FunctionCall {
                            name,
                            arguments: Vec::new(),
                        })
                    } else {
                        // with arguments
                        let mut arguments = Vec::new();
                        arguments.push(self.parse_expression()?);
                        while self.tag(Token::Comma).is_ok() {
                            arguments.push(self.parse_expression()?);
                        }
                        self.tag(Token::ParenClose)?;
                        Ok(ast::Expression::FunctionCall { name, arguments })
                    }
                } else {
                    // Plain identifier
                    Ok(ast::Expression::Identifier(name))
                }
            }
            _ => Err(Error::UnexpectedToken),
        }
    }
}

fn string_to_value(string: &str) -> Result<U256> {
    let utf8 = string.as_bytes();
    require!(utf8.len() <= 32, Error::StringToLarge);
    let mut padded = [0_u8; 32];
    padded[0..utf8.len()].copy_from_slice(utf8);
    Ok(U256::from_bytes_be(&padded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn lexer() {
        let example = include_str!("erc20.yul");
        let tokens_vec = Token::lexer(example).collect::<Vec<_>>();
        let mut tokens = Parser(&tokens_vec.as_slice());
        dbg!(tokens.parse_file().unwrap());
        // dbg!(ast);
    }
}
