use std::{iter::Peekable, rc::Rc, vec::IntoIter};

use super::{BinaryOp, Node, ParseError, UnaryOp};
use crate::{position::Position, Token, TokenType};

type ParseResult = Result<Node, ParseError>;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    token: Token,
}

use TokenType::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter().peekable();
        Self {
            token: iter.next().unwrap_or(Token {
                ty: EOF,
                start: Default::default(),
                end: Default::default(),
            }),
            tokens: iter,
        }
    }

    fn peek(&mut self) -> &TokenType {
        match self.tokens.peek() {
            Some(token) => &token.ty,
            None => &EOF,
        }
    }

    fn advance(&mut self) {
        self.token = self.tokens.next().unwrap_or(Token {
            ty: EOF,
            start: Default::default(),
            end: Default::default(),
        });
    }

    fn error<T>(&self, msg: String, start: Position) -> Result<T, ParseError> {
        Err(ParseError {
            msg,
            start,
            end: self.token.end,
        })
    }

    fn skip_newlines(&mut self) -> u32 {
        let mut newlines = 0u32;
        while self.token.ty == Newline {
            self.advance();
            newlines += 1;
        }
        newlines
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let mut statements: Vec<Node> = vec![];
        self.skip_newlines();

        statements.push(self.statement()?);

        let mut more_statements = true;

        loop {
            let newlines = self.skip_newlines();
            if newlines == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            let statement = self.statement()?;
            if statement == Node::Eof {
                more_statements = false;
                continue;
            }
            statements.push(statement);
        }

        Ok(Node::Statements(statements))
    }

    pub fn statement(&mut self) -> ParseResult {
        self.expr()
    }

    fn expr(&mut self) -> ParseResult {
        match (self.token.ty.clone(), self.peek()) {
            (Identifier(name), Eq) => {
                self.advance();
                self.advance();
                Ok(Node::Assignment(name, Box::new(self.expr()?)))
            }
            _ => self.arith_expr(),
        }
    }

    fn arith_expr(&mut self) -> ParseResult {
        let result = self.term()?;

        Ok(match self.token.ty {
            Plus => {
                self.advance();
                Node::Binary(
                    Box::new(result),
                    BinaryOp::Add,
                    Box::new(self.arith_expr()?),
                )
            }
            Minus => {
                self.advance();
                Node::Binary(
                    Box::new(result),
                    BinaryOp::Sub,
                    Box::new(self.arith_expr()?),
                )
            }
            _ => result,
        })
    }

    fn term(&mut self) -> ParseResult {
        if matches!(self.token.ty, Int(_) | Float(_))
            && !matches!(
                self.peek(),
                Int(_)
                    | Float(_)
                    | Superscript(_)
                    | Plus
                    | Minus
                    | Star
                    | Dot
                    | Cross
                    | Slash
                    | Divide
                    | Percent
                    | Carrot
                    | RightParen
                    | Pipe
                    | RightFloor
                    | RightCeil
                    | Newline
                    | EOF
            )
        {
            return Ok(Node::Binary(
                Box::new(self.atom()?),
                BinaryOp::Mul,
                Box::new(self.term()?),
            ));
        }

        let result = self.factor()?;

        Ok(match self.token.ty {
            Star | Dot | Cross => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Mul, Box::new(self.term()?))
            }
            Slash | Divide => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Div, Box::new(self.term()?))
            }
            Percent => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Rem, Box::new(self.term()?))
            }
            _ => result,
        })
    }

    fn factor(&mut self) -> ParseResult {
        Ok(match self.token.ty {
            Plus => {
                self.advance();
                Node::Unary(UnaryOp::Pos, Box::new(self.factor()?))
            }
            Minus => {
                self.advance();
                Node::Unary(UnaryOp::Neg, Box::new(self.factor()?))
            }
            _ => self.power()?,
        })
    }

    fn power(&mut self) -> ParseResult {
        let result = self.prefix()?;

        Ok(match self.token.ty {
            Carrot => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Pow, Box::new(self.factor()?))
            }
            _ => result,
        })
    }

    fn prefix(&mut self) -> ParseResult {
        Ok(match self.token.ty {
            Sqrt => {
                self.advance();
                Node::Unary(UnaryOp::Sqrt, Box::new(self.prefix()?))
            }
            Cbrt => {
                self.advance();
                Node::Unary(UnaryOp::Cbrt, Box::new(self.prefix()?))
            }
            Fort => {
                self.advance();
                Node::Unary(UnaryOp::Fort, Box::new(self.prefix()?))
            }
            _ => self.postfix()?,
        })
    }

    fn postfix(&mut self) -> ParseResult {
        let result = self.call()?;

        Ok(match self.token.ty.clone() {
            Exclamation => {
                self.advance();
                Node::Unary(UnaryOp::Fact, Box::new(result))
            }
            Degree => {
                self.advance();
                Node::Unary(UnaryOp::Degree, Box::new(result))
            }
            Superscript(tokens) => {
                self.advance();
                Node::Binary(
                    Box::new(result),
                    BinaryOp::Pow,
                    Box::new(Parser::new(tokens).arith_expr()?),
                )
            }
            _ => result,
        })
    }

    fn call(&mut self) -> ParseResult {
        let start = self.token.start;
        let result = self.atom()?;

        match self.token.ty {
            LeftParen => {
                let name = match result {
                    Node::Identifier(ref name) => Rc::clone(name),
                    _ => panic!("expected identifier"),
                };
                self.advance();

                let args = self.list(RightParen)?;

                match self.token.ty {
                    Eq => {
                        self.advance();
                        let body = self.expr()?;
                        Ok(Node::FnDef(
                            name,
                            args.into_iter()
                                .map(|node| match node {
                                    Node::Identifier(name) => name,
                                    _ => panic!("expected identifier"),
                                })
                                .collect(),
                            Box::new(body),
                        ))
                    }
                    _ => match result {
                        Node::Identifier(name) => Ok(Node::Call(name, args)),
                        _ => self.error("expected identifier".into(), start),
                    },
                }
            }
            _ => Ok(result),
        }
    }

    fn atom(&mut self) -> ParseResult {
        let start = self.token.start;

        match self.token.ty.clone() {
            Int(x) => {
                self.advance();
                Ok(Node::Int(x))
            }
            Float(x) => {
                self.advance();
                Ok(Node::Float(x))
            }
            Identifier(name) => {
                self.advance();
                Ok(Node::Identifier(name))
            }
            LeftParen => {
                self.advance();
                let result = self.expr()?;

                if self.token.ty != RightParen {
                    return self.error(format!("expected {}", RightParen), start);
                }
                self.advance();

                Ok(result)
            }
            Pipe => {
                self.advance();
                let result = self.expr()?;

                if self.token.ty != Pipe {
                    return self.error(format!("expected {}", Pipe), start);
                }
                self.advance();

                Ok(Node::Unary(UnaryOp::Abs, Box::new(result)))
            }
            LeftFloor => {
                self.advance();
                let result = self.expr()?;

                match self.token.ty {
                    RightFloor => {
                        self.advance();
                        Ok(Node::Unary(UnaryOp::Floor, Box::new(result)))
                    }
                    RightCeil => {
                        self.advance();
                        Ok(Node::Unary(UnaryOp::Abs, Box::new(result)))
                    }
                    _ => self.error(format!("expected {} or {}", RightFloor, RightCeil), start),
                }
            }
            LeftCeil => {
                self.advance();
                let result = self.expr()?;

                if self.token.ty != RightCeil {
                    return self.error(format!("expected {}", RightCeil), start);
                }
                self.advance();

                Ok(Node::Unary(UnaryOp::Ceil, Box::new(result)))
            }
            EOF => Ok(Node::Eof),
            _ => self.error(
                format!(
                    "expected int, float, identifier, {}, {}, {}, or {}",
                    LeftParen, Pipe, LeftFloor, LeftCeil
                ),
                start,
            ),
        }
    }

    fn list(&mut self, end: TokenType) -> Result<Vec<Node>, ParseError> {
        let start = self.token.start;
        let mut nodes: Vec<Node> = vec![];

        while self.token.ty != end {
            nodes.push(self.expr()?);
            match &self.token.ty {
                Comma => self.advance(),
                token if *token == end => {}
                _ => return self.error(format!("expected {} or {}", Comma, end), start),
            };
        }

        if self.token.ty != end {
            return self.error(format!("expected {}", end), start);
        }
        self.advance();

        Ok(nodes)
    }
}
