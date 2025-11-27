use crate::ast::*;
use crate::lexer::Token;
use logos::Logos;

pub struct Parser<'a> {
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    current: usize,
    source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexer = Token::lexer(source);
        let tokens: Vec<_> = lexer
            .spanned()
            .filter_map(|(tok, span)| tok.ok().map(|t| (t, span)))
            .collect();
        Parser {
            tokens,
            current: 0,
            source,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match &self.peek() {
            Ok(Token::Fn) => self.function_decl(),
            Ok(Token::Let) => self.variable_decl(false),
            Ok(Token::Mut) => self.variable_decl(true),
            Ok(Token::If) => self.if_statement(),
            Ok(Token::For) => self.for_loop(),
            Ok(Token::While) => self.while_loop(),
            Ok(Token::For) => self.for_loop(),
            Ok(Token::While) => self.while_loop(),
            Ok(Token::While) => self.while_loop(),
            Ok(Token::Return) => self.return_statement(),
            Ok(Token::Spawn) => self.spawn_statement(),
            Ok(Token::Try) => self.try_recover_statement(),
            _ => {
                let expr = self.expression()?;
                Ok(Statement::ExpressionStmt(expr))
            }
        }
    }

    fn function_decl(&mut self) -> Result<Statement, String> {
        self.consume(Token::Fn)?;
        let name = self.expect_identifier()?;
        self.consume(Token::LParen)?;
        
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.consume(Token::Colon)?;
                let param_type = self.type_annotation()?;
                params.push(Parameter {
                    name: param_name,
                    param_type,
                });
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RParen)?;

        let return_type = if self.match_token(Token::Arrow) {
            Some(self.type_annotation()?)
        } else {
            None
        };

        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        Ok(Statement::FunctionDecl(FunctionDecl {
            name,
            params,
            return_type,
            body,
        }))
    }

    fn variable_decl(&mut self, mutable: bool) -> Result<Statement, String> {
        if mutable {
            self.consume(Token::Mut)?;
        } else {
            self.consume(Token::Let)?;
        }
        let name = self.expect_identifier()?;
        let value = if self.match_token(Token::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Statement::VariableDecl(VariableDecl { name, mutable, value }))
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::If)?;
        let condition = self.expression()?;
        self.consume(Token::LBrace)?;
        let mut then_block = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            then_block.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        let else_block = if self.match_token(Token::Else) {
            self.consume(Token::LBrace)?;
            let mut block = Vec::new();
            while !self.check(Token::RBrace) && !self.is_at_end() {
                block.push(self.statement()?);
            }
            self.consume(Token::RBrace)?;
            Some(block)
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            condition,
            then_block,
            else_block,
        }))
    }

    fn for_loop(&mut self) -> Result<Statement, String> {
        self.consume(Token::For)?;
        let var = self.expect_identifier()?;
        // Expect "in"
        if let Ok(Token::Identifier(s)) = self.peek() {
            if s == "in" {
                self.advance();
            } else {
                return Err("Expected 'in'".to_string());
            }
        } else {
            return Err("Expected 'in'".to_string());
        }
        let iterable = self.expression()?;
        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;
        Ok(Statement::For(ForLoop { var, iterable, body }))
    }

    fn while_loop(&mut self) -> Result<Statement, String> {
        self.consume(Token::While)?;
        let condition = self.expression()?;
        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;
        Ok(Statement::While(WhileLoop { condition, body }))
    }

    fn return_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Return)?;
        let value = if !self.is_at_end() && !self.check(Token::RBrace) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Statement::Return(value))
    }

    fn spawn_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Spawn)?;
        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;
        Ok(Statement::Spawn(body))
    }

    fn try_recover_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Try)?;
        self.consume(Token::LBrace)?;
        let mut try_block = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            try_block.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        self.consume(Token::Recover)?;
        self.consume(Token::LParen)?;
        let error_var = self.expect_identifier()?;
        self.consume(Token::RParen)?;
        self.consume(Token::LBrace)?;
        let mut recover_block = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            recover_block.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        Ok(Statement::TryRecover {
            try_block,
            error_var,
            recover_block,
        })
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.binary_expression()
    }

    fn binary_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.primary()?;

        while let Ok(token) = self.peek() {
            let op = match token {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::Equals => Operator::Equals,
                Token::NotEquals => Operator::NotEquals,
                Token::Lt => Operator::Lt,
                Token::Le => Operator::Le,
                Token::Gt => Operator::Gt,
                Token::Ge => Operator::Ge,
                Token::Range => Operator::Range,
                _ => break,
            };
            self.advance();
            let right = self.primary()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        match self.peek()? {
            Token::Integer(n) => {
                self.advance();
                Ok(Expression::IntLiteral(n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expression::StringLiteral(s))
            }
            Token::Identifier(name) => {
                self.advance();
                let mut expr = if self.match_token(Token::LParen) {
                    self.function_call(name)?
                } else {
                    Expression::Identifier(name)
                };

                // Handle postfix operations (indexing)
                while self.match_token(Token::LBracket) {
                    let index = self.expression()?;
                    self.consume(Token::RBracket)?;
                    expr = Expression::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Ok(expr)
            }
            Token::LBracket => self.list_literal(),
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    fn function_call(&mut self, name: String) -> Result<Expression, String> {
        let mut args = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RParen)?;
        Ok(Expression::FunctionCall(FunctionCall { name, args }))
    }

    fn list_literal(&mut self) -> Result<Expression, String> {
        self.consume(Token::LBracket)?;
        let mut elements = Vec::new();
        if !self.check(Token::RBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RBracket)?;
        Ok(Expression::ListLiteral(elements))
    }

    fn type_annotation(&mut self) -> Result<Type, String> {
        match self.peek()? {
            Token::Identifier(s) => {
                self.advance();
                match s.as_str() {
                    "Int" => Ok(Type::Int),
                    "Float" => Ok(Type::Float),
                    "String" => Ok(Type::String),
                    "Bool" => Ok(Type::Bool),
                    "List" => {
                        self.consume(Token::LBracket)?;
                        let inner = self.type_annotation()?;
                        self.consume(Token::RBracket)?;
                        Ok(Type::List(Box::new(inner)))
                    }
                    _ => Ok(Type::Custom(s)),
                }
            }
            _ => Err("Expected type".to_string()),
        }
    }

    // Helper functions
    fn peek(&self) -> Result<Token, String> {
        if self.is_at_end() {
            Err("Unexpected EOF".to_string())
        } else {
            Ok(self.tokens[self.current].0.clone())
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current].0) == std::mem::discriminant(&token)
        }
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token) -> Result<(), String> {
        if self.check(token.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token, self.peek()))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.peek()? {
            Token::Identifier(s) => {
                self.advance();
                Ok(s)
            }
            _ => Err("Expected identifier".to_string()),
        }
    }
}
