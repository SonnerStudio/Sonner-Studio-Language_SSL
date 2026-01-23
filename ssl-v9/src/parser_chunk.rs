
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
}
