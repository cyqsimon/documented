use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse_quote, spanned::Spanned, Attribute, Error, Expr, ExprLit, ExprMacro, Item, Lit, Macro,
    Meta, Path, Visibility,
};

pub fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

pub fn get_vis_name_attrs(item: &Item) -> syn::Result<(Visibility, String, &[Attribute])> {
    match item {
        Item::Const(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Enum(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::ExternCrate(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Fn(item) => Ok((item.vis.clone(), item.sig.ident.to_string(), &item.attrs)),
        Item::Mod(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Static(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Struct(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Trait(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::TraitAlias(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Type(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Union(item) => Ok((item.vis.clone(), item.ident.to_string(), &item.attrs)),
        Item::Macro(item) => {
            let Some(ref ident) = item.ident else {
                Err(Error::new(
                    item.span(),
                    "Doc comments are not supported on macro invocations",
                ))?
            };
            Ok((Visibility::Inherited, ident.to_string(), &item.attrs))
        }
        Item::ForeignMod(_) | Item::Impl(_) | Item::Use(_) => Err(Error::new(
            item.span(),
            "Doc comments are not supported on this item",
        )),
        Item::Verbatim(_) => Err(Error::new(
            item.span(),
            "Doc comments are not supported on items unknown to syn",
        )),
        _ => Err(Error::new(
            item.span(),
            "This item is unknown to documented\n\
            If this item supports doc comments, consider submitting an issue or PR",
        )),
    }
}

/// The processed value(s) of `#[doc = VAL]` attribute(s).
#[derive(Clone, Debug)]
enum DocValue<'a> {
    /// At least one consecutive `/// foo` or `#{doc = "foo"]`.
    ///
    /// - Each literal value is trimmed if requested.
    /// - Consecutive literal values are folded into one.
    Lit(String),
    /// `#[doc = include_str!("path")]`.
    ///
    /// No processing on this form because we don't have the expansion.
    Macro(&'a Macro),
}
impl ToTokens for DocValue<'_> {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let tokens = match self {
            Self::Lit(lit) => quote! { #lit },
            Self::Macro(mac) => quote! { #mac },
        };
        ts.append_all(tokens);
    }
}

/// The processed and aggregated values of `#[doc = VAL]` attribute(s).
#[derive(Clone, Debug)]
pub struct DocContent<'a>(Vec<DocValue<'a>>);
impl ToTokens for DocContent<'_> {
    fn to_tokens(&self, ts: &mut TokenStream) {
        let tokens = match self.0.as_slice() {
            [] => unreachable!("0-length DocContent should not be produced"),
            [single] => quote! { #single },
            [head, tail @ ..] => quote! { concat!(#head, #("\n", #tail),*) },
        };
        ts.append_all([tokens]);
    }
}

pub fn get_docs(attrs: &[Attribute], trim: bool) -> syn::Result<Option<DocContent<'_>>> {
    let content = attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            Meta::NameValue(ref name_value) if name_value.path.is_ident("doc") => {
                Some(&name_value.value)
            }
            _ => None,
        })
        .try_fold(vec![], |mut docs, expr| -> syn::Result<_> {
            let val = match expr {
                Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => {
                    let maybe_trimmed = if trim {
                        lit.value()
                            .split('\n')
                            .map(|line| line.trim().to_string())
                            .collect::<Vec<_>>()
                            .join("\n")
                    } else {
                        lit.value()
                    };
                    DocValue::Lit(maybe_trimmed)
                }
                Expr::Macro(ExprMacro { mac, .. }) => DocValue::Macro(mac),
                other => Err(Error::new(
                    other.span(),
                    "Doc comment is neither a string literal nor a macro invocation",
                ))?,
            };

            match docs.as_mut_slice() {
                // always push first element
                [] => {
                    docs.push(val);
                }
                // try to fold subsequent elements
                [.., tail] => match (tail, &val) {
                    // fold consecutive literals
                    (DocValue::Lit(tail), DocValue::Lit(lit)) => {
                        tail.push('\n');
                        tail.push_str(lit);
                    }
                    // simple push otherwise
                    (_, _) => {
                        docs.push(val);
                    }
                },
            }

            Ok(docs)
        })?;

    if content.is_empty() {
        Ok(None)
    } else {
        Ok(Some(DocContent(content)))
    }
}
