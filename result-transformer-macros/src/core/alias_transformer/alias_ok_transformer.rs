use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{Ident, Type, parse_macro_input};

use super::alias_transformer_shared::{TwoAliasInput, build_alias};

pub fn expand(item: TokenStream) -> TokenStream {
    let TwoAliasInput {
        vis, name, content, ..
    } = parse_macro_input!(item as TwoAliasInput);
    let mut generics: Vec<Ident> = Vec::new();
    let mut input_ty = content.input;
    let mut output_ty = content.output;

    if matches!(input_ty, Type::Infer(_)) {
        input_ty = syn::parse_quote!(InputOk);
        generics.push(format_ident!("InputOk"));
    }
    if matches!(output_ty, Type::Infer(_)) {
        output_ty = syn::parse_quote!(OutputOk);
        generics.push(format_ident!("OutputOk"));
    }

    let trait_path = quote!(
        result_transformer::sync::OkTransformer<#input_ty, OutputOk = #output_ty>
    );

    TokenStream::from(build_alias(&vis, &name, &generics, trait_path))
}
