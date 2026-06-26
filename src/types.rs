pub type Token = String;
pub type TokenList = Vec<Token>;
pub type TypeList = Vec<TokenType>;
pub type LexedFile = (TokenList, TypeList);

pub enum TokenType {
    Operator(OperatorType),
    Misc(MiscType),
    Keyword(KeywordType),
    Literal(LiteralType),
    Identifier(String),
    Null
}

pub enum OperatorType {
    Add,
    Minus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    And,
    Or,
    Xor,
    Set,
    Not
}

pub enum MiscType {
    OpenParentheses,
    CloseParentheses,
    OpenBracket,
    CloseBracket,
    OpenSquiggle,
    CloseSquiggle,
    Semicolon,
    Comma,
    Colon,
    Dot,
    SingleQuote,
    DoubleQuote
}

pub enum KeywordType {
    As,
    If,
    For,
    While,
    Else,
    Not,
    Return,
    Package,
    Match
    
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
    // Float16(f16), Unstable
    Float32(f32),
    Float64(f64),
    // Float128(f128), Unstable
    StringLit(String),
    Character(char),
    Boolean(bool),
    Void,
}