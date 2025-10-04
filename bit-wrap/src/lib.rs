mod ast;
mod bit_wrapper;
mod codegen;
mod grammar;

use crate::bit_wrapper::impl_bit_wrapper;
use proc_macro::TokenStream;

#[proc_macro_derive(BitWrapper, attributes(bit_width, bit_conversion_error))]
pub fn derive_bit_wrapper(input: TokenStream) -> TokenStream {
    impl_bit_wrapper(&input.into()).into()
}
