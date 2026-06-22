use crate::lexer::{Token, TokenType};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Identifier(String),
    Array(Vec<Expression>),
    Object(Vec<(String, Expression)>),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Pipe {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    Match {
        expr: Box<Expression>,
        arms: Vec<(String, Expression)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    VariableDecl {
        name: String,
        value: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        var: String,
        iter: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token {
            token_type: TokenType::Eof,
            line: 0,
            column: 0,
        })
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens.get(self.pos + offset).unwrap_or(&Token {
            token_type: TokenType::Eof,
            line: 0,
            column: 0,
        })
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<()> {
        if std::mem::discriminant(&self.current().token_type) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("Expected {:?}, got {:?}", expected, self.current().token_type))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while !matches!(self.current().token_type, TokenType::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        self.skip_newlines();

        match &self.current().token_type {
            TokenType::Let => self.parse_variable_decl(),
            TokenType::Fn => self.parse_function_decl(),
            TokenType::Return => self.parse_return(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
            TokenType::Break => {
                self.advance();
                Ok(Statement::Break)
            }
            TokenType::Continue => {
                self.advance();
                Ok(Statement::Continue)
            }
            TokenType::Identifier(_) => {
                // Check if this is an assignment
                if let TokenType::Identifier(name) = &self.current().token_type {
                    let name = name.clone();
                    self.advance();
                    
                    if matches!(self.current().token_type, TokenType::Equal) {
                        self.advance();
                        let value = self.parse_expression()?;
                        self.consume_statement_end();
                        return Ok(Statement::Assignment { name, value });
                    }
                    
                    // Not an assignment, backtrack
                    self.pos -= 1;
                }
                
                let expr = self.parse_expression()?;
                self.consume_statement_end();
                Ok(Statement::Expression(expr))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume_statement_end();
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn consume_statement_end(&mut self) {
        while matches!(self.current().token_type, TokenType::Newline | TokenType::Semicolon) {
            self.advance();
        }
    }

    fn parse_variable_decl(&mut self) -> Result<Statement> {
        self.expect(TokenType::Let)?;

        let name = match &self.current().token_type {
            TokenType::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(anyhow!("Expected identifier after 'let'")),
        };

        self.expect(TokenType::Equal)?;

        let value = self.parse_expression()?;
        self.consume_statement_end();

        Ok(Statement::VariableDecl { name, value })
    }

    fn parse_function_decl(&mut self) -> Result<Statement> {
        self.expect(TokenType::Fn)?;

        let name = match &self.current().token_type {
            TokenType::Identifier(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(anyhow!("Expected function name")),
        };

        self.expect(TokenType::LeftParen)?;

        let mut params = Vec::new();
        if !matches!(self.current().token_type, TokenType::RightParen) {
            loop {
                if let TokenType::Identifier(p) = &self.current().token_type {
                    params.push(p.clone());
                    self.advance();
                } else {
                    return Err(anyhow!("Expected parameter name"));
                }

                if matches!(self.current().token_type, TokenType::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenType::RightParen)?;
        self.expect(TokenType::LeftBrace)?;

        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Statement::FunctionDecl { name, params, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !matches!(self.current().token_type, TokenType::RightBrace | TokenType::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }

        self.expect(TokenType::RightBrace)?;

        Ok(statements)
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.expect(TokenType::Return)?;

        let expr = if matches!(self.current().token_type, TokenType::Newline | TokenType::Semicolon | TokenType::RightBrace | TokenType::Eof) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.consume_statement_end();
        Ok(Statement::Return(expr))
    }

    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.expect(TokenType::If)?;

        let condition = self.parse_expression()?;

        self.expect(TokenType::LeftBrace)?;
        self.skip_newlines();
        let then_body = self.parse_block()?;

        let else_body = if matches!(self.current().token_type, TokenType::Else) {
            self.advance();
            if matches!(self.current().token_type, TokenType::LeftBrace) {
                self.expect(TokenType::LeftBrace)?;
                self.skip_newlines();
                Some(self.parse_block()?)
            } else if matches!(self.current().token_type, TokenType::If) {
                Some(vec![self.parse_if_statement()?])
            } else {
                None
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Result<Statement> {
        self.expect(TokenType::While)?;

        let condition = self.parse_expression()?;

        self.expect(TokenType::LeftBrace)?;
        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Statement::While { condition, body })
    }

    fn parse_for(&mut self) -> Result<Statement> {
        self.expect(TokenType::For)?;

        let var = match &self.current().token_type {
            TokenType::Identifier(v) => {
                let name = v.clone();
                self.advance();
                name
            }
            _ => return Err(anyhow!("Expected variable name in for loop")),
        };

        if !matches!(self.current().token_type, TokenType::Identifier(ref s) if s == "in") {
            return Err(anyhow!("Expected 'in' in for loop"));
        }
        self.advance();

        let iter = self.parse_expression()?;

        self.expect(TokenType::LeftBrace)?;
        self.skip_newlines();
        let body = self.parse_block()?;

        Ok(Statement::For { var, iter, body })
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_pipe()
    }

    fn parse_pipe(&mut self) -> Result<Expression> {
        let mut left = self.parse_or()?;

        while matches!(self.current().token_type, TokenType::PipeOp) {
            self.advance();
            let right = self.parse_or()?;
            left = Expression::Pipe {
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_and()?;

        while matches!(self.current().token_type, TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;

        while matches!(self.current().token_type, TokenType::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_power()?;

        loop {
            let op = match &self.current().token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Percent => BinaryOp::Modulo,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        if matches!(self.current().token_type, TokenType::Power) {
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Power,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        match &self.current().token_type {
            TokenType::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            TokenType::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.current().token_type {
                TokenType::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();

                    if !matches!(self.current().token_type, TokenType::RightParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if matches!(self.current().token_type, TokenType::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.expect(TokenType::RightParen)?;
                    expr = Expression::Call {
                        func: Box::new(expr),
                        args,
                    };
                }
                TokenType::Dot => {
                    self.advance();
                    if let TokenType::Identifier(prop) = &self.current().token_type {
                        let property = prop.clone();
                        self.advance();
                        expr = Expression::MemberAccess {
                            object: Box::new(expr),
                            property,
                        };
                    } else {
                        return Err(anyhow!("Expected property name after '.'")); 
                    }
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket)?;
                    expr = Expression::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.current().token_type.clone() {
            TokenType::Null => {
                self.advance();
                Ok(Expression::Null)
            }
            TokenType::True => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            TokenType::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expression::Number(num))
            }
            TokenType::String(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expression::String(string))
            }
            TokenType::Identifier(name) => {
                let ident = name.clone();
                self.advance();
                Ok(Expression::Identifier(ident))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                if !matches!(self.current().token_type, TokenType::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if matches!(self.current().token_type, TokenType::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(TokenType::RightBracket)?;
                Ok(Expression::Array(elements))
            }
            TokenType::LeftBrace => {
                self.advance();
                let mut pairs = Vec::new();

                if !matches!(self.current().token_type, TokenType::RightBrace) {
                    loop {
                        let key = match &self.current().token_type {
                            TokenType::Identifier(k) => {
                                let key = k.clone();
                                self.advance();
                                key
                            }
                            TokenType::String(s) => {
                                let key = s.clone();
                                self.advance();
                                key
                            }
                            _ => return Err(anyhow!("Expected key in object literal")),
                        };

                        self.expect(TokenType::Colon)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));

                        if matches!(self.current().token_type, TokenType::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(TokenType::RightBrace)?;
                Ok(Expression::Object(pairs))
            }
            TokenType::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.expect(TokenType::LeftBrace)?;
                self.skip_newlines();
                let mut then_stmts = Vec::new();
                while !matches!(self.current().token_type, TokenType::RightBrace) {
                    then_stmts.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                self.expect(TokenType::RightBrace)?;

                let then_expr = if then_stmts.len() == 1 {
                    if let Statement::Expression(e) = &then_stmts[0] {
                        e.clone()
                    } else {
                        return Err(anyhow!("If expression requires expression body"));
                    }
                } else {
                    return Err(anyhow!("If expression must have single expression"));
                };

                let else_branch = if matches!(self.current().token_type, TokenType::Else) {
                    self.advance();
                    Some(Box::new(self.parse_primary()?))
                } else {
                    None
                };

                Ok(Expression::If {
                    condition: Box::new(condition),
                    then_branch: Box::new(then_expr),
                    else_branch,
                })
            }
            _ => Err(anyhow!("Unexpected token: {:?}", self.current().token_type)),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
