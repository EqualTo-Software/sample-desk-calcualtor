// # Grammar
//
// expression    => primary operator expression
// primary       => number | variable_name | '(' expression ')' | function_call
// function_call => function_name '(' expr ')'
// operator      => '+' | '-' | '*' | '/'

use crate::{
    lexer::Lexer,
    tokens::{Operator, Token},
};

#[derive(Debug)]
pub enum Function {
    Cos,
    Sin,
    Tan,
    Log,
    Exp,
}

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Variable(String),
    Function {
        index: Function,
        arg: Box<Node>,
    },
    BinaryOp {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
    UnaryOp {
        op: UnaryOperator,
        right: Box<Node>,
    },
}

#[derive(Debug)]
pub struct ParserError {
    pub position: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

pub struct Parser {
    lexer: Lexer,
    next_token: Token,
    peek_token: Token,
}

fn binding_power(op: &Operator) -> u8 {
    match op {
        Operator::Plus => 1,
        Operator::Minus => 1,
        Operator::Times => 3,
        Operator::Divide => 3,
        Operator::Power => 5,
    }
}

impl Parser {
    pub fn parse(input_text: &str) -> Result<Node, ParserError> {
        let mut lexer = Lexer::new(input_text);
        let next_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut parser = Parser {
            lexer,
            next_token,
            peek_token,
        };
        parser.parse_expression(0)
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Node, ParserError> {
        let mut lhs = self.parse_primary()?;
        loop {
            let op = match &self.next_token {
                Token::EoI => break,
                Token::CloseParenthesis => break,
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                Token::Times => Operator::Times,
                Token::Divide => Operator::Divide,
                Token::Power => Operator::Power,
                unexpected => {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: format!(
                            "Expected operator: +, -,*, /, ^ or EoI, ) but found '{}'",
                            unexpected
                        ),
                    })
                }
            };

            let l_bp = binding_power(&op);
            if l_bp < min_bp {
                break;
            }

            self.advance_tokens();
            let rhs = self.parse_expression(l_bp)?;

            lhs = Node::BinaryOp {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn advance_tokens(&mut self) {
        self.next_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_primary(&mut self) -> Result<Node, ParserError> {
        let next_token = self.next_token.clone();
        match next_token {
            Token::Illegal(s) => Err(ParserError {
                position: self.lexer.get_position(),
                message: s,
            }),
            Token::EoI => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected end of Input".to_string(),
            }),
            Token::Number(value) => {
                self.advance_tokens();
                Ok(Node::Number(value))
            }
            Token::Plus => {
                self.advance_tokens();
                let primary = self.parse_primary()?;
                Ok(Node::UnaryOp {
                    op: UnaryOperator::Plus,
                    right: Box::new(primary),
                })
            }
            Token::Minus => {
                self.advance_tokens();
                let primary = self.parse_primary()?;
                Ok(Node::UnaryOp {
                    op: UnaryOperator::Minus,
                    right: Box::new(primary),
                })
            }
            Token::Times => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '*'".to_string(),
            }),
            Token::Power => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '^'".to_string(),
            }),
            Token::Divide => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '/'".to_string(),
            }),
            Token::Name(name) => {
                self.advance_tokens();
                if self.next_token != Token::OpenParenthesis {
                    return Ok(Node::Variable(name));
                }
                self.advance_tokens();
                let argument = self.parse_expression(0)?;

                if self.next_token != Token::CloseParenthesis {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: "Expecting: ')'".to_string(),
                    });
                }
                self.advance_tokens();

                let index = match name.as_str() {
                    "Cos" => Function::Cos,
                    "Sin" => Function::Sin,
                    "Tan" => Function::Tan,
                    "Log" => Function::Log,
                    "Exp" => Function::Exp,
                    _ => {
                        return Err(ParserError {
                            position: self.lexer.get_position(),
                            message: format!("Invalid function name: '{}'", name),
                        });
                    }
                };
                Ok(Node::Function {
                    index,
                    arg: Box::new(argument),
                })
            }
            Token::OpenParenthesis => {
                self.advance_tokens();
                let primary = self.parse_expression(0)?;
                if self.next_token != Token::CloseParenthesis {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: "Expecting: ')'".to_string(),
                    });
                }
                self.advance_tokens();
                Ok(primary)
            }
            Token::CloseParenthesis => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: ')'".to_string(),
            }),
        }
    }
}
