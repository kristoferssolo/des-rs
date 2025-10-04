use crate::{codegen::generate_impl, grammar::StructDef};
use unsynn::{IParse, ToTokens, TokenStream};

pub fn impl_bit_wrapper(input: &TokenStream) -> TokenStream {
    let parsed = input
        .to_token_iter()
        .parse::<StructDef>()
        .expect("StructDef parsing");
    generate_impl(&parsed.into())
}
