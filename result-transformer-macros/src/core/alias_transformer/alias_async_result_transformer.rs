use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{Ident, Type, parse_macro_input};

use super::alias_transformer_shared::{ResultAliasInput, build_alias};

pub fn expand(item: TokenStream) -> TokenStream {
    let ResultAliasInput {
        vis, name, content, ..
    } = parse_macro_input!(item as ResultAliasInput);
    let mut generics: Vec<Ident> = Vec::new();
    let mut in_ok_ty = content.in_ok;
    let mut in_err_ty = content.in_err;
    let mut out_ok_ty = content.out_ok;
    let mut out_err_ty = content.out_err;

    if matches!(in_ok_ty, Type::Infer(_)) {
        in_ok_ty = syn::parse_quote!(InputOk);
        generics.push(format_ident!("InputOk"));
    }
    if matches!(in_err_ty, Type::Infer(_)) {
        in_err_ty = syn::parse_quote!(InputErr);
        generics.push(format_ident!("InputErr"));
    }
    if matches!(out_ok_ty, Type::Infer(_)) {
        out_ok_ty = syn::parse_quote!(OutputOk);
        generics.push(format_ident!("OutputOk"));
    }
    if matches!(out_err_ty, Type::Infer(_)) {
        out_err_ty = syn::parse_quote!(OutputErr);
        generics.push(format_ident!("OutputErr"));
    }

    let trait_path = quote!(
        result_transformer::async_::AsyncResultTransformer<#in_ok_ty, #in_err_ty,
            OutputOk = #out_ok_ty, OutputErr = #out_err_ty>
    );

    TokenStream::from(build_alias(&vis, &name, &generics, trait_path))
}
