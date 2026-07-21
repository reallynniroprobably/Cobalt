#[derive(PartialEq)]
pub enum TokenType {
    Identifer,
    Null,
    StringLiteral(String),
    DoubleQuote,
    CharLiteral(char),
    SingleQuote,
    Not,
    Equals,
    Semicolon,
    OpeningParentheses,
    ClosingParentheses,
    OpeningBracket,
    ClosingBracket,
    OpeningSquiggle,
    ClosingSquiggle,
    Colon,
    As,
    Return
}

pub struct Token {
    pub raw: String,
    pub token_type: TokenType
}
impl Token {
    fn new() -> Token { return Token { raw: String::new(), token_type: TokenType::Null }; }
    fn from(value: char) -> Token { return Token { raw: String::from(value), token_type: TokenType::Null }; }
}

fn classify_token(raw: &String) -> TokenType {
    match raw.as_str() {
        "!" => TokenType::Not,
        "=" => TokenType::Equals,
        ";" => TokenType::Semicolon,
        "(" => TokenType::OpeningParentheses,
        ")" => TokenType::ClosingParentheses,
        "[" => TokenType::OpeningBracket,
        "]" => TokenType::ClosingBracket,
        "{" => TokenType::OpeningSquiggle,
        "}" => TokenType::ClosingSquiggle,
        ":" => TokenType::Colon,
        "as" => TokenType::As,
        "return" => TokenType::Return,
        
        _ => TokenType::Identifer
    }
}

pub fn tokenise(file_contents: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![Token::new()];
    {
        let (mut in_string, mut in_char, mut in_comment, mut escaped): (bool, bool, bool, bool) = (false, false, false, false);
        for c in file_contents.chars() {
            let mut ct: &mut Token = tokens.last_mut().unwrap();
            if in_comment {
                if c == '\n' { in_comment = false; }
                continue;
            }
            match c {
                ' '|'\t'|'\n' => { // Whitespace case
                    if !ct.raw.is_empty() {
                        ct.token_type = classify_token(&(ct.raw));
                        tokens.push(Token::new());
                    }
                }
                '"' => {
                    if in_string {
                        if escaped {
                            ct.raw.push(c);
                        } else {
                            if !ct.raw.is_empty() {
                                ct.token_type = TokenType::StringLiteral(ct.raw.clone());
                                ct = tokens.push_mut(Token::from(c));
                            } 
                            else { ct.raw.push(c); }
                            ct.token_type = TokenType::DoubleQuote;
                            tokens.push(Token::new());
                            in_string = false;
                        }
                    } else if in_char { ct.raw.push(c); }
                    else {
                        if !ct.raw.is_empty() {
                            ct.token_type = classify_token(&(ct.raw));
                            ct = tokens.push_mut(Token::from(c));
                        } else { ct.raw.push(c); }
                        ct.token_type = TokenType::DoubleQuote;
                        tokens.push(Token::new());
                        in_string = true;
                    }
                }
                '\'' => {
                    if in_char {
                        if escaped {
                            ct.raw.push(c);
                        } else {
                            if !ct.raw.is_empty() {
                                ct.token_type = TokenType::CharLiteral(ct.raw.chars().next().unwrap());
                                ct = tokens.push_mut(Token::from(c));
                            } 
                            else { ct.raw.push(c); }
                            ct.token_type = TokenType::SingleQuote;
                            tokens.push(Token::new());
                            in_char = false;
                        }
                    } else if in_string { ct.raw.push(c); }
                    else {
                        if !ct.raw.is_empty() {
                            ct.token_type = classify_token(&(ct.raw));
                            ct = tokens.push_mut(Token::from(c));
                        } else { ct.raw.push(c); }
                        ct.token_type = TokenType::SingleQuote;
                        tokens.push(Token::new());
                        in_char = true;
                    }
                }
                '/' => {
                    if escaped {
                        if in_string { ct.raw.push(c); }
                        else { in_comment = true; }
                        escaped = false;
                    } else {
                        if !ct.raw.is_empty() {
                            ct.token_type = classify_token(&(ct.raw));
                            tokens.push(Token::new());
                        }
                        escaped = true;
                    }
                }
                _ => {
                    if "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz".contains(c) { ct.raw.push(c); }
                    else {
                        if !ct.raw.is_empty() {
                            ct.token_type = classify_token(&(ct.raw));
                            tokens.push(Token::from(c));
                        } else { ct.raw.push(c); }
                        tokens.push(Token::new());
                    }
                }
            }
            
        }
        if tokens.last().unwrap().raw.is_empty() { tokens.pop(); }
    }
    return tokens;
}