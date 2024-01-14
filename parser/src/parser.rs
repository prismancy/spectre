use std::{iter::Peekable, rc::Rc, vec::IntoIter};

use crate::{BinaryOp, Node, UnaryOp};
use common::SpectreError;
use lexer::{Token, TokenType};
use TokenType::*;

type ParseResult = Result<Node, SpectreError>;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter().peekable();
        Self {
            token: iter.next().unwrap_or(Token {
                ty: EOF,
                range: Default::default(),
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
            range: Default::default(),
        });
    }

    fn error<T>(&self, msg: String, reason: String, start: usize) -> Result<T, SpectreError> {
        Err(SpectreError {
            msg,
            reason,
            range: start..self.token.range.end,
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

    pub fn parse(&mut self) -> ParseResult {
        self.statements()
    }

    fn statements(&mut self) -> ParseResult {
        let mut statements: Vec<Node> = vec![];
        self.skip_newlines();

        statements.push(self.statement()?);

        let mut more_statements = true;

        loop {
            let newlines = self.skip_newlines();
            if newlines == 0 {
                more_statements = false;
            }

            if !more_statements || self.token.ty == RightBrace {
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
                Ok(Node::Assignment(name, Box::new(self.or_expr()?)))
            }
            _ => self.or_expr(),
        }
    }

    fn or_expr(&mut self) -> ParseResult {
        let result = self.and_expr()?;

        match self.token.ty {
            Or => {
                self.advance();
                Ok(Node::Binary(
                    Box::new(result),
                    BinaryOp::Or,
                    Box::new(self.or_expr()?),
                ))
            }
            _ => Ok(result),
        }
    }

    fn and_expr(&mut self) -> ParseResult {
        let result = self.not_expr()?;

        match self.token.ty {
            And => {
                self.advance();
                Ok(Node::Binary(
                    Box::new(result),
                    BinaryOp::And,
                    Box::new(self.and_expr()?),
                ))
            }
            _ => Ok(result),
        }
    }

    fn not_expr(&mut self) -> ParseResult {
        match self.token.ty {
            Not => {
                self.advance();
                Ok(Node::Unary(UnaryOp::Not, Box::new(self.not_expr()?)))
            }
            _ => self.comp_expr(),
        }
    }

    fn comp_expr(&mut self) -> ParseResult {
        let result = self.arith_expr()?;

        macro_rules! comp_expr {
            ($($token:tt),*) => {
                match self.token.ty {
                    $(
                        $token => {
                            self.advance();
                            Ok(Node::Binary(Box::new(result), BinaryOp::$token, Box::new(self.comp_expr()?)))
                        },
                    )*
                    _ => Ok(result),
                }
            };
        }

        comp_expr!(EqEq, Neq, Lt, Lte, Gt, Gte)
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
                    | Not
                    | EqEq
                    | Neq
                    | Lt
                    | Lte
                    | Gt
                    | Gte
                    | And
                    | Or
                    | RightParen
                    | LeftBrace
                    | RightBrace
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
        let start = self.token.range.start;
        let result = self.atom()?;

        match self.token.ty {
            LeftParen => {
                let list_start = self.token.range.start;
                let name = match result {
                    Node::Identifier(ref name) => Rc::clone(name),
                    _ => panic!("expected identifier"),
                };
                self.advance();

                let args = self.list(list_start, RightParen)?;

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
                        _ => self.error(
                            "expected token".to_string(),
                            "there should be an identifier here".to_string(),
                            start,
                        ),
                    },
                }
            }
            _ => Ok(result),
        }
    }

    fn atom(&mut self) -> ParseResult {
        let start = self.token.range.start;

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
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", RightParen),
                        start,
                    );
                }
                self.advance();

                Ok(result)
            }
            Pipe => {
                self.advance();
                let result = self.expr()?;

                if self.token.ty != Pipe {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", Pipe),
                        start,
                    );
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
                    _ => self.error(
                        "expected token".to_string(),
                        format!("expected {} or {}", RightFloor, RightCeil),
                        start,
                    ),
                }
            }
            LeftCeil => {
                self.advance();
                let result = self.expr()?;

                if self.token.ty != RightCeil {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", RightCeil),
                        start,
                    );
                }
                self.advance();

                Ok(Node::Unary(UnaryOp::Ceil, Box::new(result)))
            }
            If => self.if_expr(),
            While => self.while_expr(),
            EOF => Ok(Node::Eof),
            _ => self.error(
                "expected token".to_string(),
                format!(
                    "expected int, float, identifier, {}, {}, {}, {}, or {}",
                    LeftParen, Pipe, LeftFloor, LeftCeil, If
                ),
                start,
            ),
        }
    }

    fn if_expr(&mut self) -> ParseResult {
        self.advance();

        let condition = self.expr()?;

        if self.token.ty != LeftBrace {
            return self.error(
                "expected token".to_string(),
                format!("expected {}", LeftBrace),
                self.token.range.start,
            );
        }

        let body = self.block()?;

        self.skip_newlines();

        let mut else_case: Option<Box<Node>> = None;
        if self.token.ty == Else {
            else_case = Some(Box::new(self.else_expr()?));
        }

        Ok(Node::If(Box::new(condition), Box::new(body), else_case))
    }

    fn else_expr(&mut self) -> ParseResult {
        self.advance();

        match self.token.ty {
            LeftBrace => self.block(),
            If => self.if_expr(),
            _ => self.error(
                "expected token".to_string(),
                format!("expected {} or {}", LeftBrace, If),
                self.token.range.start,
            ),
        }
    }

    fn while_expr(&mut self) -> ParseResult {
        self.advance();

        let condition = self.expr()?;

        if self.token.ty != LeftBrace {
            return self.error(
                "expected token".to_string(),
                format!("expected {}", LeftBrace),
                self.token.range.start,
            );
        }

        let body = self.block()?;

        Ok(Node::While(Box::new(condition), Box::new(body)))
    }

    fn list(&mut self, start: usize, end: TokenType) -> Result<Vec<Node>, SpectreError> {
        let mut nodes: Vec<Node> = vec![];

        while self.token.ty != end {
            nodes.push(self.expr()?);
            match &self.token.ty {
                Comma => self.advance(),
                token if *token == end => {}
                _ => {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {} or {}", Comma, end),
                        start,
                    )
                }
            };
        }

        if self.token.ty != end {
            return self.error(
                "expected token".to_string(),
                format!("expected {}", end),
                start,
            );
        }
        self.advance();

        Ok(nodes)
    }

    fn block(&mut self) -> ParseResult {
        self.advance();

        let statements = self.statements()?;

        if self.token.ty != RightBrace {
            return self.error(
                "expected token".to_string(),
                format!("expected {}", RightBrace),
                self.token.range.start,
            );
        }
        self.advance();

        Ok(statements)
    }
}
