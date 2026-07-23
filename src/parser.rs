use crate::lexer::{Token, TokenType};
use std::collections::HashMap;

enum Node {
    Identifier(String),
    MemberAccess {
        object: Box<Node>,
        member: String,
        is_static: bool
    },
    
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    UInt64(u64),
    Int128(i128),
    UInt128(u128),
    String(String),

    
    StructLiteral {
        type_identifier: String,
        data: HashMap<String, Node>
    },
    
    CallFunction {
        callee: Box<Node>,
        arguments: Vec<Node>
    },
    
    Package {
        package_identifier: String
    },

    Import {
        libraries: Vec<Library>
    },

    DefineFunction {
        identifier: String,
        return_type: Box<Node>,
        parameters: Vec<(String, Node)>,
        body: Vec<Node>
    },

    DefineVariable {
        identifier: String,
        type_identifier: Box<Node>,
        data: Option<Box<Node>>
    },

    Return {
        return_value: Box<Node>
    }
}

struct Library {
    library: Node,
    alias: Option<String>
}

fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    
    
    return nodes;
}

fn parse_expression() {
    
}
