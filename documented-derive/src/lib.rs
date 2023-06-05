use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error, Expr, ExprLit, Lit, Meta};

/// Derive proc-macro for `Documented` trait.
#[proc_macro_derive(Documented)]
pub fn documented(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let doc_comments = {
        let maybe_str_literals = input
            .attrs
            .into_iter()
            .filter_map(|attr| match attr.meta {
                Meta::NameValue(name_value) if name_value.path.is_ident("doc") => {
                    Some(name_value.value)
                }
                _ => None,
            })
            .map(|expr| match expr {
                Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
                e => Err(e),
            })
            .collect::<Result<Vec<_>, _>>();

        let literals = match maybe_str_literals {
            Ok(lits) => lits,
            Err(expr) => {
                return Error::new(expr.span(), "Doc comment is not a string literal")
                    .into_compile_error()
                    .into()
            }
        };

        if literals.len() == 0 {
            return Error::new(ident.span(), "No doc comment found on this type")
                .into_compile_error()
                .into();
        }

        let trimmed: Vec<_> = literals
            .iter()
            .flat_map(|lit| lit.split("\n").collect::<Vec<_>>())
            .map(|line| line.trim().to_string())
            .collect();

        trimmed.join("\n")
    };

    quote! {
        impl documented::Documented for #ident {
            const DOCS: &'static str = #doc_comments;
        }
    }
    .into()
}
