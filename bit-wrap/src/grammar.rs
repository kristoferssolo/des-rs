use unsynn::*;

keyword! {
    pub KwStruct = "struct";
    pub KwPub = "pub";
    pub KwBitWidth = "bit_width";
    pub KwError = "bit_conversion_error";
}

unsynn! {
    pub struct BitWidth {
        pub kw_bit_width: KwBitWidth,
        pub width: ParenthesisGroupContaining<LiteralInteger>,
    }

    pub struct Attribute {
        pub pound: Pound,
        pub bit_width: BracketGroupContaining<BitWidth>,
    }

    pub struct ErrorType {
        pub kw_bit_width: KwError,
        pub error: ParenthesisGroupContaining<Ident>,
    }

    pub struct AttributeError {
        pub pound: Pound,
        pub error: BracketGroupContaining<ErrorType>,
    }

    pub struct StructDef {
        pub bit_width: Attribute,
        pub error_type: AttributeError,
        pub vis: Option<KwPub>,
        pub kw_struct: KwStruct,
        pub name: Ident,
        pub body: ParenthesisGroupContaining<Ident>,
        pub semi: Semicolon,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
        #[bit_width(48)]
        #[bit_conversion_error()]
        pub struct Subkey(u64);
    "#;

    #[test]
    fn parse_attribute() {
        let mut iter = SAMPLE.to_token_iter();
        let sdef = iter
            .parse::<StructDef>()
            .expect("failed to parse StructDef");
        dbg!(sdef);
        assert!(false);
    }
}
