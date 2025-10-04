use crate::grammar;
use quote::format_ident;
use unsynn::Ident;

pub struct Struct {
    pub bit_width: u8,
    pub error_type: Ident,
    pub name: Ident,
    pub body: Ident,
}

impl From<grammar::StructDef> for Struct {
    fn from(value: grammar::StructDef) -> Self {
        let bit_width = u8::try_from(value.bit_width.bit_width.content.width.content.value())
            .expect("8-bit value");
        Self {
            bit_width,
            error_type: format_ident!("{}Error", value.name),
            name: value.name,
            body: value.body.content,
        }
    }
}
