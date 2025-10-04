use crate::{codegen::generate_impl, grammar::StructDef};
use unsynn::*;

pub fn impl_bit_wrapper(input: &TokenStream) -> TokenStream {
    let parsed = input.to_token_iter().parse::<StructDef>().unwrap();
    generate_impl(&parsed.into())
}
