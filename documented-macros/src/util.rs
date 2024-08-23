use syn::{parse_quote, spanned::Spanned, Attribute, Error, Expr, ExprLit, Lit, Meta, Path};

pub fn crate_module_path() -> Path {
    parse_quote!(::documented)
}

pub fn get_docs(attrs: &[Attribute], trim: bool) -> syn::Result<Option<String>> {
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

    let docs = if trim {
        string_literals
            .iter()
            .flat_map(|lit| lit.split('\n').collect::<Vec<_>>())
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        string_literals.join("\n")
    };

    Ok(Some(docs))
}
