use std::{
    any::Any,
    collections::HashMap
};
use crate::types::{
    TypeList,
    TokenType,
    OperatorType,
    MiscType,
    KeywordType,
    LiteralType
};

struct CobaltType {
    identifier: &'static str,
    values: HashMap<&'static str, Option<Box<dyn Any>>>
}
struct CobaltVariable {
    identifier: &'static str,
    type_identifier: &'static str,
    value: CobaltType,
}
