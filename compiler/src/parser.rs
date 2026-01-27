use crate::lexer::Token;
use crate::ast::*;
use anyhow::{Result, anyhow};
use logos::Logos;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    _source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens: Vec<_> = Token::lexer(source).collect::<std::result::Result<Vec<_>, _>>().unwrap_or_default();
        Self {
            tokens,
            pos: 0,
            _source: source,
        }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        let token = self.peek();
        match token {
            Some(Token::Universe) => self.parse_universe_decl(),
            Some(Token::Func) => self.parse_func_decl(),
            Some(Token::If) => self.parse_if_stmt(),
            Some(Token::While) => self.parse_while_stmt(),
            Some(Token::Return) => self.parse_return_stmt(),
            Some(Token::Ident(_)) if self.peek_next() == Some(&Token::Assign) => self.parse_assign_stmt(),
            _ => {
                let expr = self.parse_expr()?;
                self.consume(Token::Semicolon)?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_universe_decl(&mut self) -> Result<Stmt> {
        self.consume(Token::Universe)?;
        let name = self.consume_ident()?;
        self.consume(Token::LBrace)?;
        
        let mut energy = None;
        let mut body = Vec::new();
        
        while !self.check(Token::RBrace) && !self.is_at_end() {
            if self.check(Token::Energy) {
                self.consume(Token::Energy)?;
                self.consume(Token::Colon)?;
                if let Some(Token::Number(n)) = self.next() {
                    energy = Some(n);
                }
                self.consume(Token::Semicolon)?;
            } else {
                body.push(self.parse_statement()?);
            }
        }
        
        self.consume(Token::RBrace)?;
        Ok(Stmt::UniverseDecl { name, energy, body })
    }

    fn parse_func_decl(&mut self) -> Result<Stmt> {
        self.consume(Token::Func)?;
        let name = self.consume_ident()?;
        self.consume(Token::LParen)?;
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                params.push(self.consume_ident()?);
                if !self.match_token(Token::Comma) { break; }
            }
        }
        self.consume(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::FuncDecl { name, params, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        self.consume(Token::LBrace)?;
        let mut stmts = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt> {
        self.consume(Token::If)?;
        self.consume(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.consume(Token::RParen)?;
        let then_block = self.parse_block()?;
        let mut else_block = None;
        if self.match_token(Token::Else) {
            else_block = Some(self.parse_block()?);
        }
        Ok(Stmt::IfStmt { cond, then_block, else_block })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt> {
        self.consume(Token::While)?;
        self.consume(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.consume(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::WhileStmt { cond, body })
    }

    fn parse_assign_stmt(&mut self) -> Result<Stmt> {
        let name = self.consume_ident()?;
        self.consume(Token::Assign)?;
        let expr = self.parse_expr()?;
        self.consume(Token::Semicolon)?;
        Ok(Stmt::AssignStmt(name, expr))
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt> {
        self.consume(Token::Return)?;
        let expr = self.parse_expr()?;
        self.consume(Token::Semicolon)?;
        Ok(Stmt::ReturnStmt(expr))
    }

    // Expression parsing (Pratt Parser simplified)
    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_prec: u8) -> Result<Expr> {
        let mut left = self.parse_primary()?;
        
        while let Some(op_token) = self.peek() {
            let prec = self.get_precedence(op_token);
            if prec == 0 || prec < min_prec { break; }
            
            let token = self.next().unwrap();
            let op = self.token_to_op(&token)?;
            let right = self.parse_binary(prec + 1)?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        let token = self.next().ok_or_else(|| anyhow!("Unexpected end of input"))?;
        match token {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::String(s) => Ok(Expr::String(s)),
            Token::Ident(name) => {
                if self.match_token(Token::LParen) {
                    let mut args = Vec::new();
                    if !self.check(Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if !self.match_token(Token::Comma) { break; }
                        }
                    }
                    self.consume(Token::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Ident(name))
                }
            },
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            },
            Token::Signal => {
                self.consume(Token::LParen)?;
                let target = self.parse_expr()?;
                self.consume(Token::Comma)?;
                let data = self.parse_expr()?;
                self.consume(Token::RParen)?;
                Ok(Expr::Signal(Box::new(target), Box::new(data)))
            }
            _ => Err(anyhow!("Unexpected token: {:?}", token)),
        }
    }

    // Helpers
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() { self.pos += 1; }
        t
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn check(&self, token: Token) -> bool {
        self.peek() == Some(&token)
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: Token) -> Result<()> {
        if self.check(expected.clone()) {
            self.pos += 1;
            Ok(())
        } else {
            Err(anyhow!("Expected {:?}, found {:?}", expected, self.peek()))
        }
    }

    fn consume_ident(&mut self) -> Result<String> {
        if let Some(Token::Ident(s)) = self.next() {
            Ok(s)
        } else {
            Err(anyhow!("Expected identifier"))
        }
    }

    fn get_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Eq | Token::Ne => 1,
            Token::Lt | Token::Gt | Token::Le | Token::Ge => 2,
            Token::Plus | Token::Minus => 3,
            Token::Star | Token::Slash => 4,
            _ => 0,
        }
    }

    fn token_to_op(&self, token: &Token) -> Result<Op> {
        match token {
            Token::Plus => Ok(Op::Add),
            Token::Minus => Ok(Op::Sub),
            Token::Star => Ok(Op::Mul),
            Token::Slash => Ok(Op::Div),
            Token::Eq => Ok(Op::Eq),
            Token::Ne => Ok(Op::Ne),
            Token::Lt => Ok(Op::Lt),
            Token::Gt => Ok(Op::Gt),
            Token::Le => Ok(Op::Le),
            Token::Ge => Ok(Op::Ge),
            _ => Err(anyhow!("Not an operator: {:?}", token)),
        }
    }
}
