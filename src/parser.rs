use crate::{BinaryOp, Node, Token, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    token: Token,
}

use Token::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            token: tokens[0].clone(),
            tokens,
            index: 0,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        let next = self.tokens.get(self.index);
        self.token = match next {
            Some(token) => token.clone(),
            _ => EOF,
        };
    }

    fn skip_newlines(&mut self) -> u32 {
        let mut newlines = 0u32;
        while self.token == Newline {
            self.advance();
            newlines += 1;
        }
        newlines
    }

    pub fn parse(&mut self) -> Node {
        self.statements()
    }

    fn statements(&mut self) -> Node {
        let mut statements: Vec<Node> = vec![];
        self.skip_newlines();

        statements.push(self.statement());

        let mut more_statements = true;

        loop {
            let newlines = self.skip_newlines();
            if newlines == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            let statement = self.statement();
            if statement == Node::EOF {
                more_statements = false;
                continue;
            }
            statements.push(statement);
        }

        Node::Statements(statements)
    }

    pub fn statement(&mut self) -> Node {
        self.expr()
    }

    fn expr(&mut self) -> Node {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Node {
        self.and_expr()
    }

    fn and_expr(&mut self) -> Node {
        self.not_expr()
    }

    fn not_expr(&mut self) -> Node {
        self.comp_expr()
    }

    fn comp_expr(&mut self) -> Node {
        self.arith_expr()
    }

    fn arith_expr(&mut self) -> Node {
        let result = self.term();

        match self.token {
            Add => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Add, Box::new(self.arith_expr()))
            }
            Sub => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Sub, Box::new(self.arith_expr()))
            }
            _ => result,
        }
    }

    fn term(&mut self) -> Node {
        let result = self.factor();

        match self.token {
            Mul => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Mul, Box::new(self.term()))
            }
            Div => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Div, Box::new(self.term()))
            }
            Rem => {
                self.advance();
                Node::Binary(Box::new(result), BinaryOp::Rem, Box::new(self.term()))
            }
            _ => result,
        }
    }

    fn factor(&mut self) -> Node {
        match self.token {
            Add => {
                self.advance();
                Node::Unary(UnaryOp::Pos, Box::new(self.factor()))
            }
            Sub => {
                self.advance();
                Node::Unary(UnaryOp::Neg, Box::new(self.factor()))
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Node {
        let result = self.atom();

        match self.token {
            LParen => {
                self.advance();

                match result {
                    Node::Identifier(name) => {
                        let args = self.list(RParen);
                        Node::Call(name, args)
                    }
                    _ => panic!("expected identifier"),
                }
            }
            _ => result,
        }
    }

    fn atom(&mut self) -> Node {
        match self.token.clone() {
            Int(x) => {
                self.advance();
                Node::Int(x)
            }
            Float(x) => {
                self.advance();
                Node::Float(x)
            }
            Identifier(name) => {
                self.advance();
                Node::Identifier(name)
            }
            LParen => {
                self.advance();
                let result = self.expr();

                if self.token != RParen {
                    panic!("expected ')'");
                }
                self.advance();

                result
            }
            EOF => Node::EOF,
            _ => panic!("expected int, float, identifier, or '('"),
        }
    }

    fn list(&mut self, end: Token) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        while self.token != end {
            nodes.push(self.expr());
            match &self.token {
                Comma => self.advance(),
                t if *t == end => {}
                _ => panic!("expected ',' or '{}'", end),
            };
        }

        if self.token != end {
            panic!("expected '{}'", end);
        }
        self.advance();

        nodes
    }
}
