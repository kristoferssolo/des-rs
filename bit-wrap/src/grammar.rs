use unsynn::{
    BracketGroupContaining, Ident, LiteralInteger, ParenthesisGroupContaining, Pound, Semicolon,
    TokenIter, keyword, unsynn,
};

keyword! {
    pub KwStruct = "struct";
    pub KwPub = "pub";
    pub KwBitWidth = "bit_width";
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

    pub struct StructDef {
        pub bit_width: Attribute,
        pub vis: Option<KwPub>,
        pub kw_struct: KwStruct,
        pub name: Ident,
        pub body: ParenthesisGroupContaining<Ident>,
        pub semi: Semicolon,
    }
}
