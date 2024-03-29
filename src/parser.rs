use crate::{
    ast::AstNode,
    lexer::{Lexer, LexerError},
    token::Token,
    token_def::TokenType,
};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, LexerError> {
        let token = lexer.lex_next()?;
        Ok(Self {
            lexer,
            current_token: token,
        })
    }

    fn eat(&mut self, token_type: TokenType) -> Result<(), LexerError> {
        match &self.current_token {
            Some(token) => {
                if *token == token_type {
                    let curr_token = self.lexer.lex_next()?;
                    self.current_token = curr_token;
                } else {
                    panic!("Something went horribly wrong");
                }
            }
            None => {
                panic!("Already reached EOF!");
            }
        }
        Ok(())
    }

    fn consume(&mut self) -> Result<(), LexerError> {
        self.current_token = self.lexer.lex_next()?;
        Ok(())
    }

    fn parse_factor(&mut self) -> Result<AstNode, LexerError> {
        match &self.current_token {
            Some(token) => match token.token_type {
                TokenType::Number(number) => {
                    self.consume()?;
                    Ok(AstNode::Int(number))
                }
                TokenType::LeftBracket => {
                    self.eat(TokenType::LeftBracket)?;
                    let node = self.parse_expression()?;
                    self.eat(TokenType::RightBracket)?;
                    Ok(node)
                }
                _ => {
                    // TODO: Return propper error
                    panic!("Unknown token {:?}", self.current_token);
                }
            },
            None => {
                // TODO: Return propper error
                panic!("Already reached EOF!");
            }
        }
    }

    fn parse_term(&mut self) -> Result<AstNode, LexerError> {
        let mut node = self.parse_factor()?;
        if let Some(token) = self.current_token.clone() {
            if token == TokenType::RightBracket {
                return Ok(node);
            }
        }

        while let Some(token) = self.current_token.clone() {
            match token.token_type {
                TokenType::Multiply | TokenType::Divide => {
                    self.consume()?;
                    node = AstNode::BinaryOperation {
                        op: Box::new(token),
                        left: Box::new(node),
                        right: Box::new(self.parse_factor()?),
                    };
                    if let Some(new_curr_token) = self.current_token.clone() {
                        if new_curr_token == TokenType::RightBracket {
                            break;
                        }
                    }
                }
                TokenType::Plus | TokenType::Minus => {
                    break;
                }
                _ => {
                    panic!("Unexpected token {:?}", self.current_token);
                }
            }
        }
        Ok(node)
    }

    fn parse_expression(&mut self) -> Result<AstNode, LexerError> {
        let mut node = self.parse_term()?;
        if let Some(token) = self.current_token.clone() {
            if token == TokenType::RightBracket {
                return Ok(node);
            }
        }

        while let Some(token) = self.current_token.clone() {
            match token.token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.consume()?;
                    node = AstNode::BinaryOperation {
                        op: Box::new(token),
                        left: Box::new(node),
                        right: Box::new(self.parse_term()?),
                    };
                    if let Some(new_curr_token) = self.current_token.clone() {
                        if new_curr_token == TokenType::RightBracket {
                            break;
                        }
                    }
                }
                _ => {
                    panic!("Unexpected token {:?}", self.current_token);
                }
            }
        }
        Ok(node)
    }

    pub fn parse(&mut self) -> Result<AstNode, LexerError> {
        self.parse_expression()
    }
}
