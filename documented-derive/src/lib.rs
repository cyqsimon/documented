use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    DataUnion, DeriveInput, Error, Expr, ExprLit, Lit, Meta, Path,
};

fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

/// Derive proc-macro for `Documented` trait.
#[proc_macro_derive(Documented)]
pub fn documented(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let doc_comments = match get_comment(&input.attrs) {
        Ok(Some(r)) => r,
        // Should we use an Option instead?
        Ok(None) => "".to_string(),
        Err(e) => return e.into_compile_error().into(),
    };

    let fields_doc_comments = {
        let fields_attrs: Vec<(syn::Ident, Vec<Attribute>)> = match input.data.clone() {
            Data::Enum(DataEnum { variants, .. }) => {
                variants.into_iter().map(|v| (v.ident, v.attrs)).collect()
            }
            Data::Struct(DataStruct { fields, .. }) => fields
                .into_iter()
                .filter(|f| f.ident.is_some())
                .map(|f| (f.ident.unwrap(), f.attrs))
                .collect(),
            Data::Union(DataUnion { fields, .. }) => fields
                .named
                .into_iter()
                .map(|f| (f.ident.unwrap(), f.attrs))
                .collect(),
        };

        match fields_attrs
            .into_iter()
            .map(|(i, attrs)| get_comment(&attrs).map(|c| (i, c.unwrap_or_default())))
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(t) => t,
            Err(e) => return e.into_compile_error().into(),
        }
    };

    let ident = input.ident;

    let (field_idents, field_comments): (Vec<_>, Vec<_>) = fields_doc_comments.into_iter().unzip();

    let phf_match_arms: Vec<_> = field_idents
        .into_iter()
        .map(|ident| ident.to_string())
        .enumerate()
        .map(|(i, ident)| quote! { #ident => #i, })
        .collect();

    let documented_module_path = crate_module_path();

    quote! {
        #[automatically_derived]
        impl documented::Documented for #ident {
            const DOCS: &'static str = #doc_comments;

            const FIELD_DOCS: &'static [&'static str] = &[#(#field_comments),*];

            fn get_index_by_name<T: AsRef<str>>(field_name: T) -> Option<usize> {
                use #documented_module_path::_private_phf_reexport_for_macro as phf;

                static PHF: phf::Map<&'static str, usize> = phf::phf_map! {
                    #(#phf_match_arms)*
                };
                PHF.get(field_name.as_ref()).copied()
            }
        }
    }
    .into()
}

fn get_comment(attrs: &[Attribute]) -> syn::Result<Option<String>> {
    let string_literals = attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            Meta::NameValue(ref name_value) if name_value.path.is_ident("doc") => {
                Some(&name_value.value)
            }
            _ => None,
        })
        .map(|expr| match expr {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
            other => Err(Error::new(
                other.span(),
                "Doc comment is not a string literal",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;

    if string_literals.is_empty() {
        return Ok(None);
    }

    let trimmed: Vec<_> = string_literals
        .iter()
        .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
        .map(|line| line.trim().to_string())
        .collect();

    Ok(Some(trimmed.join("\n")))
}
