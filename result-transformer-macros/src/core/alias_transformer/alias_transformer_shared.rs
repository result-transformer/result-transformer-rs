use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Ident, Token, Type, Visibility,
    parse::{Parse, ParseStream},
};

#[allow(unused)]
pub struct ResultAliasInput {
    pub vis: Visibility,
    pub trait_token: Token![trait],
    pub name: Ident,
    pub colon_token: Token![:],
    pub content: ResultAliasTypes,
}

#[allow(unused)]
pub struct ResultAliasTypes {
    pub bracket: syn::token::Bracket,
    pub in_ok: Type,
    pub comma1: Token![,],
    pub in_err: Type,
    pub arrow_token: Token![=>],
    pub out_ok: Type,
    pub comma2: Token![,],
    pub out_err: Type,
    pub trailing: Option<Token![,]>,
}

impl Parse for ResultAliasInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let trait_token: Token![trait] = input.parse()?;
        let name: Ident = input.parse()?;
        let colon_token: Token![:] = input.parse()?;
        let content;
        let bracket = syn::bracketed!(content in input);
        let in_ok: Type = content.parse()?;
        let comma1: Token![,] = content.parse()?;
        let in_err: Type = content.parse()?;
        let arrow_token: Token![=>] = content.parse()?;
        let out_ok: Type = content.parse()?;
        let comma2: Token![,] = content.parse()?;
        let out_err: Type = content.parse()?;
        let trailing: Option<Token![,]> = if content.peek(Token![,]) {
            Some(content.parse()?)
        } else {
            None
        };
        Ok(ResultAliasInput {
            vis,
            trait_token,
            name,
            colon_token,
            content: ResultAliasTypes {
                bracket,
                in_ok,
                comma1,
                in_err,
                arrow_token,
                out_ok,
                comma2,
                out_err,
                trailing,
            },
        })
    }
}

#[allow(unused)]
pub struct TwoAliasInput {
    pub vis: Visibility,
    pub trait_token: Token![trait],
    pub name: Ident,
    pub colon_token: Token![:],
    pub content: TwoAliasTypes,
}

#[allow(unused)]
pub struct TwoAliasTypes {
    pub bracket: syn::token::Bracket,
    pub input: Type,
    pub arrow_token: Token![=>],
    pub output: Type,
    pub trailing: Option<Token![,]>,
}

impl Parse for TwoAliasInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let trait_token: Token![trait] = input.parse()?;
        let name: Ident = input.parse()?;
        let colon_token: Token![:] = input.parse()?;
        let content;
        let bracket = syn::bracketed!(content in input);
        let input_ty: Type = content.parse()?;
        let arrow_token: Token![=>] = content.parse()?;
        let output_ty: Type = content.parse()?;
        let trailing: Option<Token![,]> = if content.peek(Token![,]) {
            Some(content.parse()?)
        } else {
            None
        };
        Ok(TwoAliasInput {
            vis,
            trait_token,
            name,
            colon_token,
            content: TwoAliasTypes {
                bracket,
                input: input_ty,
                arrow_token,
                output: output_ty,
                trailing,
            },
        })
    }
}

pub fn build_alias(
    vis: &Visibility,
    name: &Ident,
    generics: &[Ident],
    trait_path: TokenStream2,
) -> TokenStream2 {
    let trait_generics = if generics.is_empty() {
        quote!()
    } else {
        quote!(<#(#generics),*>)
    };
    let impl_generics = if generics.is_empty() {
        quote!(<T>)
    } else {
        quote!(<T, #(#generics),*>)
    };
    quote! {
        #vis trait #name #trait_generics: #trait_path { }
        impl #impl_generics #name #trait_generics for T
        where
            T: #trait_path
        { }
    }
}
