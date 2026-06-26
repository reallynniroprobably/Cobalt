use crate::types::{
    Token,
    TokenList,
    TypeList,
    LexedFile,
    TokenType,
    OperatorType,
    MiscType,
    KeywordType,
    LiteralType
};

pub fn lex_file(file_contents: &String) -> TokenList {
    let mut tokens: TokenList = vec![Token::new()];
    let (mut in_comment, mut in_string, mut in_char, mut escaped) = (false, false, false, false);
    
    for c in file_contents.chars() {
        let lt: &mut String = &mut tokens.last_mut()
            .expect("No last value");
        
        if !in_comment {
            match c {
                ' ' | '\t' | '\n' => {
                    if in_string || in_char { lt.push(c); }
                    else if !lt.is_empty() { tokens.push(Token::new()); }
                }
                '"' => {
                    if in_string {
                        if escaped { lt.push(c); }
                        else {
                            tokens.push(Token::from(c));
                            tokens.push(Token::new());
                            in_string = false;
                        }
                    } else if in_char { lt.push(c); }
                    else {
                        if lt.is_empty() { lt.push(c); }
                        else { tokens.push(Token::from(c)); }
                        tokens.push(Token::new());
                        in_string = true;
                    }
                }
                '\'' => {
                    if in_char {
                        if escaped { lt.push(c); }
                        else {
                            tokens.push(Token::from(c));
                            tokens.push(Token::new());
                        }
                    } else if in_string || lt.is_empty(){ lt.push(c); }
                    else {
                        if lt.is_empty() { lt.push(c); }
                        else { tokens.push(Token::from(c)); }
                        tokens.push(Token::new());
                        in_char = true;
                    }
                }
                '\\' => {
                    if in_string || in_char {
                        if escaped {
                            lt.push(c);
                            escaped = false;
                        } else { escaped = true; }
                    } else {
                        if escaped {
                            escaped = false;
                            in_comment = true;
                        } else {
                            if !lt.is_empty() { tokens.push(Token::new()); }
                            escaped = true;
                        }
                    }
                }
                ';' | '(' | '<' | '{' | '[' | ')' | '>' | '}' | ']' | ',' | '+' | '-' | '*' | '/' => {
                    if in_string || in_char { lt.push(c); }
                    else {
                        if lt.is_empty() { lt.push(c); }
                        else { tokens.push(Token::from(c)); }
                        tokens.push(Token::new());
                    }
                }
                ':' => {
                    if in_string || in_char { lt.push(c); }
                    else {
                        if lt.as_str() == ":" {
                            lt.push(c);
                            tokens.push(Token::new());
                        } else {
                            if lt.is_empty() { lt.push(c); }
                            else { tokens.push(Token::from(c)); }
                        }
                    }
                }
                _ => { lt.push(c); }
            }
        }
    }

    return tokens;   
}
pub fn lex_program(file_contents: &String) -> LexedFile {
    let unlexed_tokens = lex_file(file_contents);
    let mut lexed_tokens: TypeList = vec![];
    
    for (i, t) in unlexed_tokens.iter().enumerate() {
        lexed_tokens.push(
            match t.as_str() {
                "+" => { TokenType::Operator(OperatorType::Add) }
                "-" => { TokenType::Operator(OperatorType::Minus) }
                "*" => { TokenType::Operator(OperatorType::Multiply) }
                "/" => { TokenType::Operator(OperatorType::Divide) }
                "<" => { TokenType::Operator(OperatorType::LessThan) }
                ">" => { TokenType::Operator(OperatorType::GreaterThan) }
                "&" => { TokenType::Operator(OperatorType::And) }
                "|" => { TokenType::Operator(OperatorType::Or) }
                "^" => { TokenType::Operator(OperatorType::Xor) }
                "=" => { TokenType::Operator(OperatorType::Set) }
                "!" => { TokenType::Operator(OperatorType::Not) }
    
                "(" => { TokenType::Misc(MiscType::OpenParentheses) }
                ")" => { TokenType::Misc(MiscType::CloseParentheses) }
                "[" => { TokenType::Misc(MiscType::OpenBracket) }
                "]" => { TokenType::Misc(MiscType::CloseBracket) }
                "{" => { TokenType::Misc(MiscType::OpenSquiggle) }
                "}" => { TokenType::Misc(MiscType::CloseSquiggle) }
                ";" => { TokenType::Misc(MiscType::Semicolon) }
                "," => { TokenType::Misc(MiscType::Comma) }
                ":" => { TokenType::Misc(MiscType::Colon) }
                "." => { TokenType::Misc(MiscType::Dot) }
                "'" => { TokenType::Misc(MiscType::SingleQuote) }
                "\"" => { TokenType::Misc(MiscType::DoubleQuote) }
    
                "as" => { TokenType::Keyword(KeywordType::As) }
                "if" => { TokenType::Keyword(KeywordType::If) }
                "for" => { TokenType::Keyword(KeywordType::For) }
                "while" => { TokenType::Keyword(KeywordType::While) }
                "else" => { TokenType::Keyword(KeywordType::Else) }
                "not" => { TokenType::Keyword(KeywordType::Not) }
                "return" => { TokenType::Keyword(KeywordType::Return) }
                "package" => { TokenType::Keyword(KeywordType::Package) }
                "match" => { TokenType::Keyword(KeywordType::Match) }

                // Boolean literals
                "true" => { TokenType::Literal(LiteralType::Boolean(true)) }
                "false" => { TokenType::Literal(LiteralType::Boolean(false)) }
                other => { 
                    // Checking if token is a number literal
                    if "0123456789".contains(other.chars().next().unwrap_or_default()) {
                        // Checking if token is a float literal
                        if unlexed_tokens[i + 1].as_str() == "." && "0123456789".contains(unlexed_tokens[i + 2].chars().next().unwrap_or_default()) {
                            TokenType::Literal(LiteralType::Float64(unlexed_tokens[i..i + 2]
                                .join("")
                                .parse()
                                .expect(
                                    format!("Failed to unwrap unknown token {i} as float64")
                                        .as_str()
                                )
                            ))
                        // Must be an int literal
                        } else {
                            TokenType::Literal(LiteralType::Integer128(other
                                .parse()
                                .expect(
                                    format!("Failed to unwrap unknown token {i} as int128")
                                        .as_str()
                                )
                            ))
                        }
                    // checking if either string or character literal or identifier
                    } else {
                        let last = lexed_tokens.last().unwrap_or(&TokenType::Null);
                        // Character literal
                        if let TokenType::Misc(MiscType::SingleQuote) = last {
                            TokenType::Literal(LiteralType::Character(other.parse().unwrap_or_default()))
                        // String literal
                        } else if let TokenType::Misc(MiscType::DoubleQuote) = last {
                            TokenType::Literal(LiteralType::StringLit(t.clone()))
                        } else {
                            TokenType::Identifier(t.clone())
                        }
                    }
                }
            }
        ); 
        
    }
    
    return (unlexed_tokens, lexed_tokens);
}