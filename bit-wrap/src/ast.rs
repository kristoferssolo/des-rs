use crate::grammar;
use unsynn::*;

pub struct Struct {
    pub bit_width: u128,
    pub error_type: Ident,
    pub name: Ident,
    pub body: Ident,
}

impl From<grammar::StructDef> for Struct {
    fn from(value: grammar::StructDef) -> Self {
        Self {
            bit_width: value.bit_width.bit_width.content.width.content.value(),
            error_type: value.error_type.error.content.error.content,
            name: value.name,
            body: value.body.content,
        }
    }
}
