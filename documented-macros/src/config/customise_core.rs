use syn::{
    parse::{Parse, ParseStream},
    LitBool, LitStr, Token,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(trim);
}

/// All known configuration options. Each variant of config struct may choose to
/// accept or reject any of them in their `Parse` implementation.
///
/// Expected parse stream format: `<KW> = <VAL>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigOption {
    /// Custom name for generated constant.
    ///
    /// E.g. `name = "CUSTOM_NAME_DOCS"`
    Name(kw::name, String),

    /// Trim each line or not.
    ///
    /// E.g. `trim = false`.
    Trim(kw::trim, bool),
}
impl Parse for ConfigOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::name) {
            let kw = input.parse()?;
            input.parse::<Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            Ok(Self::Name(kw, name.value()))
        } else if lookahead.peek(kw::trim) {
            let kw = input.parse()?;
            input.parse::<Token![=]>()?;
            let trim = input.parse::<LitBool>()?;
            Ok(Self::Trim(kw, trim.value))
        } else {
            Err(lookahead.error())
        }
    }
}
