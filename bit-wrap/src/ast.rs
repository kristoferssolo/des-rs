use crate::grammar;
use unsynn::*;

pub struct Struct {
    pub attr: Attribute,
    pub name: Ident,
    pub body: Ident,
}

impl Struct {
    pub fn bit_width(&self) -> u128 {
        self.attr.bit_width
    }
}

pub struct Attribute {
    pub bit_width: u128,
}

impl From<grammar::Attribute> for Attribute {
    fn from(value: grammar::Attribute) -> Self {
        Self {
            bit_width: value.bit_width.content.bit_width.content.value(),
        }
    }
}

impl From<grammar::StructDef> for Struct {
    fn from(value: grammar::StructDef) -> Self {
        Self {
            attr: value.attr.into(),
            name: value.name,
            body: value.body.content,
        }
    }
}
