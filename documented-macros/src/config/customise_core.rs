use itertools::Itertools;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Error, Expr, LitBool, LitStr, Token, Visibility,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(vis);
    custom_keyword!(name);
    custom_keyword!(default);
    custom_keyword!(trim);
}

/// A configuration option that includes the span info. Each kind of
/// customisation struct may choose to accept or reject any of them.
///
/// Expected parse stream format: `<KW> = <VAL>`.
#[derive(Clone, Debug)]
pub struct ConfigOption {
    /// The whole config span, from the keyword to the value.
    pub span: Span,

    /// The config key-value pair.
    pub data: ConfigOptionData,
}
impl Parse for ConfigOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use ConfigOptionData as Data;
        use ConfigOptionKind as Kind;

        let span = input.span();

        let kind = input.parse::<ConfigOptionKind>()?;
        input.parse::<Token![=]>()?;
        let data = match kind {
            Kind::Vis => Data::Vis(input.parse()?),
            Kind::Name => Data::Name(input.parse()?),
            Kind::Default => Data::Default(input.parse()?),
            Kind::Trim => Data::Trim(input.parse()?),
        };

        Ok(Self { span, data })
    }
}

/// The data of all known configuration options.
#[derive(Clone, Debug, PartialEq, Eq, strum::EnumDiscriminants)]
#[strum_discriminants(
    vis(pub(self)),
    name(ConfigOptionKind),
    derive(strum::Display, Hash),
    strum(serialize_all = "snake_case")
)]
pub enum ConfigOptionData {
    /// Custom visibility for the generated constant.
    ///
    /// E.g. `vis = pub(crate)`.
    Vis(Visibility),

    /// Custom name for generated constant.
    ///
    /// E.g. `name = "CUSTOM_NAME_DOCS"`.
    Name(LitStr),

    /// Use some default value when doc comments are absent.
    ///
    /// E.g. `default = "not documented"`.
    Default(Expr),

    /// Trim each line or not.
    ///
    /// E.g. `trim = false`.
    Trim(LitBool),
}

impl Parse for ConfigOptionKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if lookahead.peek(kw::vis) {
            input.parse::<kw::vis>()?;
            Self::Vis
        } else if lookahead.peek(kw::name) {
            input.parse::<kw::name>()?;
            Self::Name
        } else if lookahead.peek(kw::default) {
            input.parse::<kw::default>()?;
            Self::Default
        } else if lookahead.peek(kw::trim) {
            input.parse::<kw::trim>()?;
            Self::Trim
        } else {
            Err(lookahead.error())?
        };
        Ok(ty)
    }
}

/// Make sure there are no duplicate options.
/// Otherwise produces an error with detailed span info.
pub fn ensure_unique_options(opts: &[ConfigOption]) -> syn::Result<()> {
    for (kind, opts) in opts
        .iter()
        .into_group_map_by(|opt| ConfigOptionKind::from(&opt.data))
        .into_iter()
    {
        match &opts[..] {
            [] => unreachable!(), // guaranteed by `into_group_map_by`
            [_unique] => continue,
            [first, rest @ ..] => {
                let initial_error = Error::new(
                    first.span,
                    format!("Option {kind} can only be declaration once"),
                );
                let final_error = rest.iter().fold(initial_error, |mut err, opt| {
                    err.combine(Error::new(opt.span, "Duplicate declaration here"));
                    err
                });
                Err(final_error)?
            }
        }
    }
    Ok(())
}
