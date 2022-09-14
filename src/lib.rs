//! For structs with named fields generates an impl that would return a doc comment for a field, if
//! one is available:
//!
//!
//! ```rust
//! # use comgrabe::Comgrabe;
//! #[derive(Debug, Comgrabe)]
//! /// outer
//! #[allow(dead_code)]
//! struct Foo<'a, T> {
//!    /// Bar
//!    /// Bar2
//!    x: bool,
//!    /// B
//!    b: &'a str,
//!    /// t
//!    ttt: T,
//! }
//!
//!
//! assert_eq!(Some("Bar\nBar2"), Foo::<()>::comgrabe("x"));
//! assert_eq!(Some("t"), Foo::<()>::comgrabe("ttt"));
//! assert_eq!(None, Foo::<()>::comgrabe("nosuch"));
//! ```
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    token, AngleBracketedGenericArguments, Attribute, LitStr, Visibility,
};

#[proc_macro_derive(Comgrabe)]
pub fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as Top).to_token_stream().into()
}

struct Doc(pub String);
impl Parse for Doc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<token::Eq>()?;
        let s = input.parse::<LitStr>()?.value();
        Ok(Doc(s.trim_start().to_string()))
    }
}

struct Top {
    abga: Option<AngleBracketedGenericArguments>,
    ident: Ident,
    fields: Vec<(LitStr, LitStr)>,
}
impl Parse for Top {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _attrs = input.call(Attribute::parse_outer)?;
        let _vis = input.parse::<Visibility>()?;

        if !input.peek(token::Struct) {
            return Err(input.error("Only structs are supported"));
        }
        input.parse::<token::Struct>()?;
        let ident = input.parse::<Ident>()?;

        let abga = if input.peek(token::Lt) {
            Some(input.parse::<AngleBracketedGenericArguments>()?)
        } else {
            None
        };

        if !input.peek(token::Brace) {
            return Err(input.error("Only structs with named fields are supported"));
        }

        let raw_fields = input.parse::<syn::FieldsNamed>()?;
        let fields = raw_fields
            .named
            .iter()
            .filter_map(|f| {
                let mut msg = String::new();
                for attr in &f.attrs {
                    if attr.path.is_ident("doc") {
                        msg.push_str(&syn::parse2::<Doc>(attr.tokens.clone()).ok()?.0);
                        msg.push('\n');
                    }
                }

                if msg.is_empty() {
                    return None;
                }
                msg.pop();
                let ident = f.ident.as_ref().unwrap();
                let name = LitStr::new(&ident.to_string(), ident.span());

                let comment = LitStr::new(&msg, f.span());

                Some((name, comment))
            })
            .collect::<Vec<_>>();

        Ok(Self {
            abga,
            ident,
            fields,
        })
    }
}

impl ToTokens for Top {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote;
        let Top {
            abga,
            ident,
            fields,
        } = self;

        let field_matches = fields.iter().map(|(name, val)| quote!(#name => #val,));

        quote! {
            impl #abga #ident #abga {
                pub fn comgrabe(name: &str) -> Option<&'static str> {
                    Some(match name {
                        #(#field_matches)*
                        _ => return None
                    })
                }
            }
        }
        .to_tokens(tokens);
    }
}
