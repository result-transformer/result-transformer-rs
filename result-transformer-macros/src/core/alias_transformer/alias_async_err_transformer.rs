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
        input_ty = syn::parse_quote!(InputErr);
        generics.push(format_ident!("InputErr"));
    }
    if matches!(output_ty, Type::Infer(_)) {
        output_ty = syn::parse_quote!(OutputErr);
        generics.push(format_ident!("OutputErr"));
    }

    let trait_path = quote!(
        result_transformer::async_::AsyncErrTransformer<#input_ty, OutputErr = #output_ty>
    );

    TokenStream::from(build_alias(&vis, &name, &generics, trait_path))
}
