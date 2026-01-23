use crate::ast::*;
use crate::lexer::Token;
use logos::Logos;

/// Convenience helper for main.rs
pub fn parse(source: &str) -> Result<Vec<Statement>, String> {
    let mut parser = Parser::new(source);
    parser.parse()
}

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
            .filter_map(|(tok, span)| match tok {
                Ok(t) => Some((t, span)),
                Err(e) => {
                    eprintln!("Lexer Error: {:?} at {:?}", e, span);
                    None
                }
            })
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
            Ok(Token::Async) => self.function_decl(), // Handle async fn
            Ok(Token::Let) => self.variable_decl(false),
            Ok(Token::Mut) => self.variable_decl(true),
            Ok(Token::If) => self.if_statement(),
            Ok(Token::For) => self.for_loop(),
            Ok(Token::While) => self.while_loop(),
            Ok(Token::Return) => self.return_statement(),
            Ok(Token::Spawn) => self.spawn_statement(),
            Ok(Token::Try) => self.try_recover_statement(),
            Ok(Token::Import) => self.import_statement(),
            Ok(Token::Unsafe) => self.unsafe_block(),
            Ok(Token::Natural) => self.natural_block(),
            Ok(Token::Visual) => self.visual_block(),
            Ok(Token::Html) => self.html_block(),
            Ok(Token::Trait) => self.trait_decl(),
            Ok(Token::Impl) => self.impl_trait(),
            Ok(Token::Type) => self.type_alias(),
            Ok(Token::Enum) => self.enum_decl(),
            Ok(Token::Window) => self.window_decl(),
            // SSL 3.2: Freestanding Environment
            Ok(Token::Hardware) => self.hardware_block(),
            Ok(Token::Platform) => self.platform_block(),
            Ok(Token::AtFreestanding) => self.module_attribute(),
            Ok(Token::AtNoStd) => self.module_attribute(),
            Ok(Token::AtTarget) => self.module_attribute(),
            Ok(Token::AtPanicHandler) | 
            Ok(Token::AtNoMangle) | 
            Ok(Token::AtNaked) | 
            Ok(Token::AtInterrupt) |
            Ok(Token::AtEntry) |
            Ok(Token::AtLinkSection) => self.attributed_function(),
            Ok(Token::Semicolon) => {
                self.advance();
                if self.is_at_end() || self.check(Token::RBrace) {
                    Ok(Statement::Empty)
                } else {
                    self.statement()
                }
            }
            _ => {
                let expr = self.expression()?;
                if self.match_token(Token::Assign) {
                    // SSL 3.0: Support for index-based assignments
                    match expr {
                        Expression::Identifier(name) => {
                            let value = self.expression()?;
                            self.match_token(Token::Semicolon);
                            Ok(Statement::Assignment(Assignment { name, value }))
                        }
                        Expression::Index { target, index } => {
                            let value = self.expression()?;
                            self.match_token(Token::Semicolon);
                            Ok(Statement::IndexAssignment { 
                                target: *target, 
                                index: *index, 
                                value 
                            })
                        }
                        _ => Err("Invalid assignment target".to_string())
                    }
                } else {
                    self.match_token(Token::Semicolon);
                    Ok(Statement::ExpressionStmt(expr))
                }
            }
        }
    }

    fn function_decl(&mut self) -> Result<Statement, String> {
        let is_async = self.match_token(Token::Async);
        self.consume(Token::Fn)?;
        let name = self.expect_identifier()?;
        
        let mut type_params = Vec::new();
        if self.match_token(Token::Lt) {
            loop {
                let param = self.expect_identifier()?;
                type_params.push(param);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            self.consume(Token::Gt)?;
        }

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
            type_params,
            params,
            return_type,
            body,
            is_async,
            attributes: vec![], // SSL 3.2: No attributes for regular functions
        }))
    }

    fn variable_decl(&mut self, start_mutable: bool) -> Result<Statement, String> {
        let mut mutable = start_mutable;
        if mutable {
            self.consume(Token::Mut)?;
        } else {
            self.consume(Token::Let)?;
            // SSL 3.0: Support 'let mut' syntax
            if self.match_token(Token::Mut) {
                mutable = true;
            }
        }
        let name = self.expect_identifier()?;
        
        // Parse optional type annotation: let x: Int = 42
        let var_type = if self.match_token(Token::Colon) {
            Some(self.type_annotation()?)
        } else {
            None
        };
        
        let value = if self.match_token(Token::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Statement::VariableDecl(VariableDecl { name, mutable, var_type, value }))
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

    fn visual_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Visual)?;
        
        // Get the opening brace and its position
        if !self.check(Token::LBrace) {
            return Err("Expected '{' after 'visual'".to_string());
        }
        let start_span = self.tokens[self.current].1.clone();
        self.advance(); // Consume '{'
        
        let start_pos = start_span.end;
        let mut brace_count = 1;
        let mut end_pos = start_pos;
        
        // Scan forward to find matching brace
        let mut temp_current = self.current;
        while temp_current < self.tokens.len() {
            let (token, span) = &self.tokens[temp_current];
            
            match token {
                Token::LBrace => brace_count += 1,
                Token::RBrace => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_pos = span.start;
                        break;
                    }
                }
                _ => {}
            }
            
            temp_current += 1;
        }
        
        if brace_count != 0 {
            return Err("Unmatched braces in visual block".to_string());
        }
        
        // Extract the text between braces
        let content = self.source[start_pos..end_pos].to_string();
        
        // Advance past all tokens up to and including the closing brace
        while self.current <= temp_current {
            self.advance();
        }
        
        Ok(Statement::VisualBlock(content.trim().to_string()))
    }

    fn spawn_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Spawn)?;
        
        // Check for 'on' keyword for distributed spawn
        if let Ok(Token::Identifier(s)) = self.peek() {
            if s == "on" {
                self.advance(); // consume 'on'
                
                // Expect string literal for address
                let address = match self.peek()? {
                    Token::StringLiteral(s) => {
                        self.advance();
                        s
                    },
                    _ => return Err("Expected string literal for address after 'spawn on'".to_string()),
                };
                
                self.consume(Token::LBrace)?;
                let mut body = Vec::new();
                while !self.check(Token::RBrace) && !self.is_at_end() {
                    body.push(self.statement()?);
                }
                self.consume(Token::RBrace)?;
                return Ok(Statement::SpawnOn { address, block: body });
            }
        }

        // Local spawn
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

    fn import_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Import)?;
        
        let mut path = Vec::new();
        path.push(self.expect_identifier()?);
        
        while self.match_token(Token::Dot) {
            path.push(self.expect_identifier()?);
        }
        
        let alias = if self.match_token(Token::As) {
            Some(self.expect_identifier()?)
        } else {
            None
        };
        
        Ok(Statement::Import { path, alias })
    }

    fn unsafe_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Unsafe)?;
        self.consume(Token::LBrace)?;
        
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;
        
        Ok(Statement::UnsafeBlock(body))
    }

    fn natural_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Natural)?;
        
        // Get the opening brace and its position
        if !self.check(Token::LBrace) {
            return Err("Expected '{' after 'natural'".to_string());
        }
        let start_span = self.tokens[self.current].1.clone();
        self.advance(); // Consume '{'
        
        let start_pos = start_span.end;
        let mut brace_count = 1;
        let mut end_pos = start_pos;
        
        // Scan forward to find matching brace
        let mut temp_current = self.current;
        while temp_current < self.tokens.len() {
            let (token, span) = &self.tokens[temp_current];
            
            match token {
                Token::LBrace => brace_count += 1,
                Token::RBrace => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_pos = span.start;
                        self.current = temp_current + 1; // Advance parser past this block
                        break;
                    }
                }
                _ => {}
            }
            temp_current += 1;
        }
        
        if brace_count > 0 {
            return Err("Unclosed natural block".to_string());
        }
        
        // Extract raw text
        let raw_text = &self.source[start_pos..end_pos];
        Ok(Statement::NaturalBlock(raw_text.trim().to_string()))
    }

    fn trait_decl(&mut self) -> Result<Statement, String> {
        self.consume(Token::Trait)?;
        let name = self.expect_identifier()?;
        self.consume(Token::LBrace)?;
        
        let mut methods = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            self.consume(Token::Fn)?;
            let method_name = self.expect_identifier()?;
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
            
            methods.push(TraitMethod {
                name: method_name,
                params,
                return_type,
            });
        }
        
        self.consume(Token::RBrace)?;
        Ok(Statement::TraitDecl(TraitDecl { name, methods }))
    }

    fn impl_trait(&mut self) -> Result<Statement, String> {
        self.consume(Token::Impl)?;
        let trait_name = self.expect_identifier()?;
        self.consume(Token::For)?;
        let for_type = self.expect_identifier()?;
        self.consume(Token::LBrace)?;
        
        let mut methods = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            if let Statement::FunctionDecl(func) = self.function_decl()? {
                methods.push(func);
            } else {
                return Err("Expected function declaration in impl block".to_string());
            }
        }
        
        self.consume(Token::RBrace)?;
        Ok(Statement::ImplTrait(ImplTrait {
            trait_name,
            for_type,
            methods,
        }))
    }

    fn type_alias(&mut self) -> Result<Statement, String> {
        self.consume(Token::Type)?;
        let name = self.expect_identifier()?;
        self.consume(Token::Assign)?;
        let target = self.type_annotation()?;
        Ok(Statement::TypeAlias(TypeAlias { name, target }))
    }

    fn enum_decl(&mut self) -> Result<Statement, String> {
        self.consume(Token::Enum)?;
        let name = self.expect_identifier()?;
        
        // Optional type params
        let mut type_params = Vec::new();
        if self.match_token(Token::Lt) {
            loop {
                type_params.push(self.expect_identifier()?);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            self.consume(Token::Gt)?;
        }
        
        self.consume(Token::LBrace)?;
        
        let mut variants = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            let variant_name = self.expect_identifier()?;
            let mut fields = Vec::new();
            
            // Tuple variants: Variant(Type, Type)
            if self.match_token(Token::LParen) {
                loop {
                    fields.push(self.type_annotation()?);
                    if !self.match_token(Token::Comma) {
                        break;
                    }
                }
                self.consume(Token::RParen)?;
            }
            
            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });
            
            self.match_token(Token::Comma); // Optional comma
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Statement::EnumDecl(EnumDecl {
            name,
            type_params,
            variants,
        }))
    }
    
    // ========================================================================
    // SSL 3.2: Freestanding Environment Parsers
    // ========================================================================
    
    /// Parse hardware block: hardware [volatile] { ... }
    fn hardware_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Hardware)?;
        
        // Check for volatile modifier
        let is_volatile = self.match_token(Token::Volatile);
        
        self.consume(Token::LBrace)?;
        
        let mut operations = Vec::new();
        
        while !self.check(Token::RBrace) && !self.is_at_end() {
            let op = self.parse_hardware_op()?;
            operations.push(op);
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Statement::HardwareBlock(HardwareBlock {
            is_volatile,
            operations,
        }))
    }
    
    /// Parse a single hardware operation
    fn parse_hardware_op(&mut self) -> Result<HardwareOp, String> {
        match &self.peek() {
            // cpu.* operations
            Ok(Token::Cpu) => {
                self.advance();
                self.consume(Token::Dot)?;
                self.parse_cpu_operation()
            }
            // memory[addr] operations
            Ok(Token::Memory) => {
                self.advance();
                self.parse_memory_operation()
            }
            // port.* operations (x86)
            Ok(Token::Port) => {
                self.advance();
                self.consume(Token::Dot)?;
                self.parse_port_operation()
            }
            // register operations
            Ok(Token::Register) => {
                self.advance();
                self.parse_register_operation()
            }
            // Variable declaration within hardware block
            Ok(Token::Let) => {
                self.variable_decl(false)?;
                // For now, we'll treat let statements as NoOp in hardware context
                Ok(HardwareOp::Nop)
            }
            _ => Err(format!("Unexpected token in hardware block: {:?}", self.peek()))
        }
    }
    
    /// Parse cpu.* operations
    fn parse_cpu_operation(&mut self) -> Result<HardwareOp, String> {
        let operation = self.expect_identifier()?;
        
        match operation.as_str() {
            "interrupt" => {
                self.consume(Token::Dot)?;
                let action = self.expect_identifier()?;
                self.consume(Token::LParen)?;
                self.consume(Token::RParen)?;
                match action.as_str() {
                    "disable" => Ok(HardwareOp::DisableInterrupts),
                    "enable" => Ok(HardwareOp::EnableInterrupts),
                    _ => Err(format!("Unknown interrupt action: {}", action))
                }
            }
            "halt" => {
                self.consume(Token::LParen)?;
                self.consume(Token::RParen)?;
                Ok(HardwareOp::Halt)
            }
            "nop" => {
                self.consume(Token::LParen)?;
                self.consume(Token::RParen)?;
                Ok(HardwareOp::Nop)
            }
            "fence" => {
                self.consume(Token::LParen)?;
                self.consume(Token::RParen)?;
                Ok(HardwareOp::MemoryFence)
            }
            "register" => {
                self.consume(Token::Dot)?;
                let reg_name = self.expect_identifier()?;
                self.consume(Token::Assign)?;
                let value = self.expression()?;
                Ok(HardwareOp::RegisterWrite {
                    register: reg_name,
                    value,
                })
            }
            _ => Err(format!("Unknown cpu operation: {}", operation))
        }
    }
    
    /// Parse memory operations
    fn parse_memory_operation(&mut self) -> Result<HardwareOp, String> {
        // Determine size suffix (memory.byte, memory.word, etc.)
        let size = if self.match_token(Token::Dot) {
            let size_str = self.expect_identifier()?;
            match size_str.as_str() {
                "byte" => MemorySize::Byte,
                "word" => MemorySize::Word,
                "dword" => MemorySize::DWord,
                "qword" => MemorySize::QWord,
                _ => return Err(format!("Unknown memory size: {}", size_str))
            }
        } else {
            MemorySize::QWord // Default to 64-bit
        };
        
        self.consume(Token::LBracket)?;
        let address = self.expression()?;
        self.consume(Token::RBracket)?;
        
        // Check if this is a write operation
        if self.match_token(Token::Assign) {
            let value = self.expression()?;
            Ok(HardwareOp::MemoryWrite {
                address,
                value,
                size,
            })
        } else {
            // This is a read - but we need a destination
            // For now, just return a read to a temp variable
            Ok(HardwareOp::MemoryRead {
                dest: "_temp".to_string(),
                address,
                size,
            })
        }
    }
    
    /// Parse port operations (x86)
    fn parse_port_operation(&mut self) -> Result<HardwareOp, String> {
        let operation = self.expect_identifier()?;
        
        match operation.as_str() {
            "read" => {
                self.consume(Token::LParen)?;
                let port = self.expression()?;
                self.consume(Token::RParen)?;
                Ok(HardwareOp::PortRead {
                    dest: "_temp".to_string(),
                    port,
                    size: MemorySize::Byte,
                })
            }
            "write" => {
                self.consume(Token::LParen)?;
                let port = self.expression()?;
                self.consume(Token::Comma)?;
                let value = self.expression()?;
                self.consume(Token::RParen)?;
                Ok(HardwareOp::PortWrite {
                    port,
                    value,
                    size: MemorySize::Byte,
                })
            }
            _ => Err(format!("Unknown port operation: {}", operation))
        }
    }
    
    /// Parse register operations
    fn parse_register_operation(&mut self) -> Result<HardwareOp, String> {
        let dest = self.expect_identifier()?;
        self.consume(Token::Assign)?;
        let source = self.expression()?;
        
        Ok(HardwareOp::RegisterRead {
            dest,
            source,
        })
    }
    
    /// Parse platform-specific block: platform(x86_64) { ... }
    fn platform_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Platform)?;
        self.consume(Token::LParen)?;
        
        let target_str = self.expect_identifier()?;
        let target = match target_str.as_str() {
            "x86_64" | "x86" | "x64" => TargetPlatform::X86_64,
            "arm64" | "aarch64" => TargetPlatform::ARM64,
            "apple_intel" => TargetPlatform::AppleIntel,
            "apple_silicon" | "m1" | "m2" | "m3" | "m4" => TargetPlatform::AppleSilicon,
            "steam_deck" | "steamdeck" => TargetPlatform::SteamDeck,
            "all" | "*" => TargetPlatform::All,
            _ => return Err(format!("Unknown target platform: {}", target_str))
        };
        
        self.consume(Token::RParen)?;
        self.consume(Token::LBrace)?;
        
        let mut body = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            body.push(self.statement()?);
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Statement::PlatformBlock { target, body })
    }
    
    /// Parse module-level attribute (@freestanding, @no_std, @target)
    fn module_attribute(&mut self) -> Result<Statement, String> {
        let mut attributes = Vec::new();
        
        loop {
            match &self.peek() {
                Ok(Token::AtFreestanding) => {
                    self.advance();
                    attributes.push(ModuleAttribute::Freestanding);
                }
                Ok(Token::AtNoStd) => {
                    self.advance();
                    attributes.push(ModuleAttribute::NoStd);
                }
                Ok(Token::AtTarget) => {
                    self.advance();
                    self.consume(Token::LParen)?;
                    let target_str = self.expect_identifier()?;
                    let target = match target_str.as_str() {
                        "x86_64" => TargetPlatform::X86_64,
                        "arm64" => TargetPlatform::ARM64,
                        "apple_intel" => TargetPlatform::AppleIntel,
                        "apple_silicon" => TargetPlatform::AppleSilicon,
                        "steam_deck" => TargetPlatform::SteamDeck,
                        _ => return Err(format!("Unknown target: {}", target_str))
                    };
                    self.consume(Token::RParen)?;
                    attributes.push(ModuleAttribute::Target(target));
                }
                _ => break
            }
        }
        
        Ok(Statement::ModuleDecl {
            attributes,
            name: None,
        })
    }
    
    /// Parse function with attributes (@panic_handler, @no_mangle, etc.)
    fn attributed_function(&mut self) -> Result<Statement, String> {
        let mut func_attributes = Vec::new();
        
        // Collect all attributes before the function
        loop {
            match &self.peek() {
                Ok(Token::AtPanicHandler) => {
                    self.advance();
                    func_attributes.push(FunctionAttribute::PanicHandler);
                }
                Ok(Token::AtNoMangle) => {
                    self.advance();
                    func_attributes.push(FunctionAttribute::NoMangle);
                }
                Ok(Token::AtNaked) => {
                    self.advance();
                    func_attributes.push(FunctionAttribute::Naked);
                }
                Ok(Token::AtInterrupt) => {
                    self.advance();
                    func_attributes.push(FunctionAttribute::Interrupt);
                }
                Ok(Token::AtEntry) => {
                    self.advance();
                    func_attributes.push(FunctionAttribute::Entry);
                }
                Ok(Token::AtLinkSection) => {
                    self.advance();
                    self.consume(Token::LParen)?;
                    let section = match self.peek()? {
                        Token::StringLiteral(s) => {
                            self.advance();
                            s
                        }
                        _ => return Err("Expected string for link_section".to_string())
                    };
                    self.consume(Token::RParen)?;
                    func_attributes.push(FunctionAttribute::LinkSection(section));
                }
                _ => break
            }
        }
        
        // Now parse the function declaration
        let is_async = self.match_token(Token::Async);
        self.consume(Token::Fn)?;
        let name = self.expect_identifier()?;
        
        let mut type_params = Vec::new();
        if self.match_token(Token::Lt) {
            loop {
                let param = self.expect_identifier()?;
                type_params.push(param);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
            self.consume(Token::Gt)?;
        }

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
            type_params,
            params,
            return_type,
            body,
            is_async,
            attributes: func_attributes,
        }))
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expression, String> {
        let mut left = self.logic_and()?;
        while self.match_token(Token::Or) {
            let right = self.logic_and()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op: Operator::Or,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn logic_and(&mut self) -> Result<Expression, String> {
        let mut left = self.comparison()?;
        while self.match_token(Token::And) {
            let right = self.comparison()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op: Operator::And,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.range()?;
        while let Ok(token) = self.peek() {
            let op = match token {
                Token::Equals => Operator::Equals,
                Token::NotEquals => Operator::NotEquals,
                Token::Lt => Operator::Lt,
                Token::Le => Operator::Le,
                Token::Gt => Operator::Gt,
                Token::Ge => Operator::Ge,
                _ => break,
            };
            self.advance();
            let right = self.range()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn range(&mut self) -> Result<Expression, String> {
        let mut left = self.term()?;
        while self.match_token(Token::Range) {
            let right = self.term()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op: Operator::Range,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut left = self.factor()?;
        while let Ok(token) = self.peek() {
            let op = match token {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.factor()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut left = self.pipe()?;
        while let Ok(token) = self.peek() {
            let op = match token {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                _ => break,
            };
            self.advance();
            let right = self.pipe()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn pipe(&mut self) -> Result<Expression, String> {
        let mut left = self.composition()?;
        while self.match_token(Token::PipeRight) {
            let function = self.composition()?;
            left = Expression::Pipe {
                value: Box::new(left),
                function: Box::new(function),
            };
        }
        Ok(left)
    }

    fn composition(&mut self) -> Result<Expression, String> {
        let mut left = self.unary()?;
        while let Ok(token) = self.peek() {
            let op = match token {
                Token::ComposeRight => Operator::ComposeRight,
                Token::ComposeLeft => Operator::ComposeLeft,
                _ => break,
            };
            self.advance();
            let right = self.unary()?;
            left = Expression::BinaryOp(BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        // Prefix operators can be added here (e.g. !, -)
        
        let mut expr = self.primary()?;
        
        // Handle postfix operators like await
        // Handle postfix operators like await
        while self.check(Token::Dot) {
            // Check the token AFTER the dot
            if let Ok(next_token) = self.peek_ahead(1) {
                if matches!(next_token, Token::Await) {
                    self.advance(); // consume Dot
                    self.advance(); // consume Await
                    expr = Expression::Await(Box::new(expr));
                } else {
                    // For now, if it's not .await, don't consume the dot here.
                    // This allows Dot to be handled by other parsers (like import)
                    // or just ignored if it's not valid here.
                    break;
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        match self.peek()? {
            Token::Integer(n) => {
                self.advance();
                Ok(Expression::IntLiteral(n))
            }
            // SSL 3.2: Hexadecimal literals (0xB8000)
            Token::FloatLiteral(f) => {
                self.advance();
                Ok(Expression::FloatLiteral(f))
            }
            Token::HexInteger(n) => {
                self.advance();
                Ok(Expression::IntLiteral(n))
            }
            // SSL 3.2: Binary literals (0b1010)
            Token::BinaryInteger(n) => {
                self.advance();
                Ok(Expression::IntLiteral(n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expression::StringLiteral(s))
            }
            Token::BoolLiteral(b) => {
                self.advance();
                Ok(Expression::BoolLiteral(b))
            }
            Token::Identifier(name) => {
                self.advance();
                let mut expr = if self.match_token(Token::DoubleColon) {
                    let variant_name = self.expect_identifier()?;
                    let mut args = Vec::new();
                    if self.match_token(Token::LParen) {
                        if !self.check(Token::RParen) {
                            loop {
                                args.push(self.expression()?);
                                if !self.match_token(Token::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(Token::RParen)?;
                    }
                    Expression::EnumConstruction {
                        enum_name: name,
                        variant_name,
                        args,
                    }
                } else if self.check(Token::Lt) && self.is_generic_call_lookahead() {
                    self.advance(); // consume Lt
                    // Generic function call: name<Type>(args)
                    let mut type_args = Vec::new();
                    loop {
                        type_args.push(self.type_annotation()?);
                        if !self.match_token(Token::Comma) {
                            break;
                        }
                    }
                    self.consume(Token::Gt)?;
                    self.consume(Token::LParen)?;
                    
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
                    
                    Expression::FunctionCall(FunctionCall {
                        name,
                        type_args,
                        args,
                    })
                } else if self.match_token(Token::LParen) {
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
            Token::LBrace => self.map_literal(),
            Token::Match => self.match_expression(),
            Token::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }
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
        Ok(Expression::FunctionCall(FunctionCall { name, type_args: vec![], args }))
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

    fn map_literal(&mut self) -> Result<Expression, String> {
        self.consume(Token::LBrace)?;
        let mut pairs = Vec::new();
        while !self.check(Token::RBrace) {
             let key = match self.peek()? {
                Token::Identifier(s) => s,
                Token::StringLiteral(s) => s,
                 _ => return Err("Expected string key in map literal".to_string()),
             };
             self.advance();
             self.consume(Token::Colon)?;
             let value = self.expression()?;
             pairs.push((key, value));
             if !self.match_token(Token::Comma) {
                 break;
             }
        }
        self.consume(Token::RBrace)?;
        Ok(Expression::MapLiteral(pairs))
    }

    fn match_expression(&mut self) -> Result<Expression, String> {
        self.consume(Token::Match)?;
        let value = self.expression()?;
        self.consume(Token::LBrace)?;
        
        let mut arms = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            let guard = if self.match_token(Token::If) {
                Some(self.expression()?)
            } else {
                None
            };
            
            self.consume(Token::FatArrow)?;
            let body = self.expression()?;
            
            // Optional comma
            self.match_token(Token::Comma);
            
            arms.push(MatchArm {
                pattern,
                guard,
                body,
            });
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Expression::Match {
            value: Box::new(value),
            arms,
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        match self.peek()? {
            Token::Underscore => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            Token::Integer(n) => {
                self.advance();
                Ok(Pattern::IntLiteral(n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Pattern::StringLiteral(s))
            }
            Token::Identifier(name) => {
                self.advance();
                
                if self.match_token(Token::DoubleColon) {
                    let variant = self.expect_identifier()?;
                    if self.match_token(Token::LParen) {
                        let mut args = Vec::new();
                        if !self.check(Token::RParen) {
                            loop {
                                args.push(self.parse_pattern()?);
                                if !self.match_token(Token::Comma) {
                                    break;
                                }
                            }
                        }
                        self.consume(Token::RParen)?;
                        Ok(Pattern::Constructor { name: variant, args })
                    } else {
                        Ok(Pattern::Constructor { name: variant, args: vec![] })
                    }
                } else if self.match_token(Token::LParen) {
                    let mut args = Vec::new();
                    if !self.check(Token::RParen) {
                        loop {
                            args.push(self.parse_pattern()?);
                            if !self.match_token(Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RParen)?;
                    Ok(Pattern::Constructor { name, args })
                } else {
                    Ok(Pattern::Variable(name))
                }
            }
            _ => Err("Expected pattern".to_string()),
        }
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
                    "Any" | "any" => Ok(Type::Any),
                    "List" => {
                        if self.match_token(Token::LBracket) {
                            let inner = self.type_annotation()?;
                            self.consume(Token::RBracket)?;
                            Ok(Type::List(Box::new(inner)))
                        } else if self.match_token(Token::Lt) {
                            let inner = self.type_annotation()?;
                            self.consume(Token::Gt)?;
                            Ok(Type::List(Box::new(inner)))
                        } else {
                            Ok(Type::List(Box::new(Type::Any)))
                        }
                    }
                    _ => {
                        if self.match_token(Token::Lt) {
                            let mut args = Vec::new();
                            loop {
                                args.push(self.type_annotation()?);
                                if !self.match_token(Token::Comma) {
                                    break;
                                }
                            }
                            self.consume(Token::Gt)?;
                            Ok(Type::Generic { base: s, args } )
                        } else {
                            Ok(Type::Custom(s))
                        }
                    }
                }
            }
            _ => Err(format!("Expected type, got {:?}", self.peek())),
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

    fn peek_ahead(&self, offset: usize) -> Result<Token, String> {
        if self.current + offset >= self.tokens.len() {
            Err("Unexpected EOF".to_string())
        } else {
            Ok(self.tokens[self.current + offset].0.clone())
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

    fn is_generic_call_lookahead(&self) -> bool {
        let mut depth = 0;
        let mut i = 0;
        // current is at '<'
        while let Ok(token) = self.peek_ahead(i) {
            match token {
                Token::Lt => depth += 1,
                Token::Gt => {
                    depth -= 1;
                    if depth == 0 {
                        // Found matching >, check next token
                        if let Ok(next) = self.peek_ahead(i + 1) {
                            return matches!(next, Token::LParen | Token::DoubleColon);
                        }
                        return false;
                    }
                }
                Token::Semicolon | Token::RBrace | Token::RParen => break,
                _ => {}
            }
            i += 1;
            if i > 20 { break; } // Safety limit
        }
        false
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

impl<'a> Parser<'a> {
    fn html_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::Html)?;
        
        // Get the opening brace and its position
        if !self.check(Token::LBrace) {
            return Err("Expected '{' after 'html'".to_string());
        }
        let start_span = self.tokens[self.current].1.clone();
        self.advance(); // Consume '{'
        
        let start_pos = start_span.end;
        let mut brace_count = 1;
        let mut end_pos = start_pos;
        
        // Scan forward to find matching brace
        let mut temp_current = self.current;
        while temp_current < self.tokens.len() {
            let (token, span) = &self.tokens[temp_current];
            
            match token {
                Token::LBrace => brace_count += 1,
                Token::RBrace => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        end_pos = span.start;
                        self.current = temp_current + 1; // Advance parser past this block
                        break;
                    }
                }
                _ => {}
            }
            temp_current += 1;
        }
        
        if brace_count > 0 {
            return Err("Unclosed html block".to_string());
        }
        
        // Extract raw text
        let raw_text = &self.source[start_pos..end_pos];
        Ok(Statement::HtmlBlock(raw_text.trim().to_string()))
    }



    fn window_decl(&mut self) -> Result<Statement, String> {
        self.consume(Token::Window)?;
        self.consume(Token::LBrace)?;
        
        let mut title = String::new();
        let mut width = 800;
        let mut height = 600;
        let mut transparent = false;
        let mut decorations = true;
        let mut html = String::new();
        
        while !self.check(Token::RBrace) {
            match self.peek()? {
                Token::Identifier(key) => {
                    self.advance(); // Consume key
                    self.consume(Token::Colon)?;
                    match key.as_str() {
                        "title" => {
                            if let Token::StringLiteral(s) = self.peek()? {
                                self.advance();
                                title = s;
                            } else {
                                return Err("Expected string for title".to_string());
                            }
                        },
                        "width" => {
                            if let Token::Integer(i) = self.peek()? {
                                self.advance();
                                width = i;
                            } else {
                                return Err("Expected integer for width".to_string());
                            }
                        },
                        "height" => {
                            if let Token::Integer(i) = self.peek()? {
                                self.advance();
                                height = i;
                            } else {
                                return Err("Expected integer for height".to_string());
                            }
                        },
                        "html" => {
                            if let Token::StringLiteral(s) = self.peek()? {
                                self.advance();
                                html = s;
                            } else {
                                return Err("Expected string for html".to_string());
                            }
                        },
                        "transparent" => {
                            if let Token::BoolLiteral(b) = self.peek()? {
                                self.advance();
                                transparent = b;
                            } else {
                                return Err("Expected boolean for transparent".to_string());
                            }
                        },
                        "decorations" => {
                            if let Token::BoolLiteral(b) = self.peek()? {
                                self.advance();
                                decorations = b;
                            } else {
                                return Err("Expected boolean for decorations".to_string());
                            }
                        },
                        _ => {
                            // Skip unknown property value
                            self.expression()?; 
                        }
                    }
                    if self.check(Token::Comma) {
                        self.advance();
                    }
                },
                _ => return Err("Expected identifier in window block".to_string()),
            }
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Statement::WindowDecl(WindowDecl {
            title,
            width,
            height,
            transparent,
            decorations,
            html,
        }))
    }

    fn material_decl(&mut self) -> Result<Statement, String> {
        self.consume(Token::Material)?;
        let name = self.expect_identifier()?;

        self.consume(Token::LBrace)?;
        
        let mut diffuse = String::new();
        let mut normal = None;
        let mut roughness = None;
        
        while !self.check(Token::RBrace) {
             match self.peek()? {
                Token::Identifier(key) => {
                    self.advance(); // Consume key
                    self.consume(Token::Colon)?;
                    match key.as_str() {
                        "diffuse" => {
                            if let Token::StringLiteral(s) = self.peek()? {
                                self.advance();
                                diffuse = s;
                            } else {
                                return Err("Expected string for diffuse".to_string());
                            }
                        },
                        "normal" => {
                            if let Token::StringLiteral(s) = self.peek()? {
                                self.advance();
                                normal = Some(s);
                            } else {
                                return Err("Expected string for normal".to_string());
                            }
                        },
                        "roughness" => {
                            if let Token::StringLiteral(s) = self.peek()? {
                                self.advance();
                                roughness = Some(s);
                            } else {
                                return Err("Expected string for roughness".to_string());
                            }
                        },
                        _ => {
                            // Skip unknown
                            self.expression()?;
                        }
                    }
                    if self.check(Token::Comma) {
                        self.advance();
                    }
                },
                _ => return Err("Expected identifier in material block".to_string()),
            }
        }
        
        self.consume(Token::RBrace)?;
        
        Ok(Statement::MaterialDecl(MaterialDecl {
            name,
            diffuse,
            normal,
            roughness,
        }))
    }
}
