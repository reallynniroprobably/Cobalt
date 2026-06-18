pub struct Token {
    pub v: String,
    pub t: TokenType
}

impl Token {
    fn new() -> Self { return Self { v: String::new(), t: TokenType::Null }; }
    fn from(v: char) -> Self { return Self { v: String::from(v), t: TokenType::Null }; }
    fn identify(&mut self) {
        self.t = match self.v.as_str() {
            "+" => TokenType::Operator(OperatorType::Add)
        }
    }
}

pub enum TokenType {
    Identifier(String),
    Literal(LiteralType),
    Keyword(String),
    Operator(OperatorType),
    Null
}

pub enum OperatorType {
    Add,
    Minus,
    Multiply,
    Divide,
    AddOne,
    MinusOne
}

pub enum LiteralType {
    UInteger8(u8),
    Integer8(i8),
    UInteger16(u16),
    Integer16(i16),
    UInteger32(u32),
    Integer32(i32),
    UInteger64(u64),
    Integer64(i64),
    UInteger128(u128),
    Integer128(i128),
    // Float16(f16),
    Float32(f32),
    Float64(f64),
    // Float128(f128),
    String(String),
    Character(char),
    Boolean(bool)
}
type LexedFile = Vec<Token>;

pub fn lex_file(file_contents: &String) -> LexedFile {
    let mut tokens: LexedFile = vec![Token::new()];
    let (mut in_comment, mut in_string, mut in_char, mut escaped) = (false, false, false, false);
    
    for c in file_contents.chars() {
        let lt: &mut String = &mut tokens.last_mut()
            .expect("No last value")
            .v;
        
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
                ';' | '(' | '<' | '{' | '[' | ')' | '>' | '}' | ']' | ',' => {
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