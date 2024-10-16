use itertools::Itertools;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, LitBool, LitStr, Token, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(vis);
    custom_keyword!(name);
    custom_keyword!(default);
    custom_keyword!(trim);
}

/// All known configuration options. Each variant of config struct may choose to
/// accept or reject any of them in their `Parse` implementation.
///
/// Expected parse stream format: `<KW> = <VAL>`.
#[derive(Clone, Debug, PartialEq, Eq, strum::EnumDiscriminants)]
#[strum_discriminants(
    vis(pub(self)),
    name(ConfigOptionType),
    derive(strum::Display, Hash),
    strum(serialize_all = "snake_case")
)]
pub enum ConfigOption {
    /// Custom visibility for the generated constant.
    ///
    /// E.g. `vis = pub(crate)`.
    Vis(kw::vis, Visibility),

    /// Custom name for generated constant.
    ///
    /// E.g. `name = "CUSTOM_NAME_DOCS"`.
    Name(kw::name, String),

    /// Use some default value when doc comments are absent.
    ///
    /// E.g. `default = "not documented"`.
    Default(kw::default, Expr),

    /// Trim each line or not.
    ///
    /// E.g. `trim = false`.
    Trim(kw::trim, bool),
}
impl Parse for ConfigOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::vis) {
            let kw = input.parse()?;
            input.parse::<Token![=]>()?;
            let vis = input.parse::<Visibility>()?;
            Ok(Self::Vis(kw, vis))
        } else if lookahead.peek(kw::name) {
            let kw = input.parse()?;
            input.parse::<Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            Ok(Self::Name(kw, name.value()))
        } else if lookahead.peek(kw::default) {
            let kw = input.parse()?;
            input.parse::<Token![=]>()?;
            let default = input.parse::<Expr>()?;
            Ok(Self::Default(kw, default))
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
impl ConfigOption {
    pub fn kw_span(&self) -> Span {
        match self {
            Self::Vis(kw, _) => kw.span(),
            Self::Name(kw, _) => kw.span(),
            Self::Default(kw, _) => kw.span(),
            Self::Trim(kw, _) => kw.span(),
        }
    }
}

/// Make sure there are no duplicate options.
/// Otherwise produces an error with detailed span info.
pub fn ensure_unique_options(opts: &[ConfigOption]) -> syn::Result<()> {
    for (ty, opts) in opts
        .iter()
        .into_group_map_by(|opt| ConfigOptionType::from(*opt))
        .into_iter()
    {
        match &opts[..] {
            [] => unreachable!(), // guaranteed by `into_group_map_by`
            [_unique] => continue,
            [first, rest @ ..] => {
                let initial_error = Error::new(
                    first.kw_span(),
                    format!("Option {ty} can only be declaration once"),
                );
                let final_error = rest.iter().fold(initial_error, |mut err, opt| {
                    err.combine(Error::new(opt.kw_span(), "Duplicate declaration here"));
                    err
                });
                Err(final_error)?
            }
        }
    }
    Ok(())
}
