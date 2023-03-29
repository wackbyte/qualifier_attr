use proc_macro as pm;
use syn::{spanned::Spanned, parse::Parse};
use quote::ToTokens;

enum FnQualifier {
    Visibility(syn::Visibility),
    Constness(syn::token::Const),
    Asyncness(syn::token::Async),
    Unsafety(syn::token::Unsafe),
    Abi(syn::Abi),
}

struct FnQualifiers {
    visibility: Option<syn::Visibility>,
    constness: Option<syn::token::Const>,
    asyncness: Option<syn::token::Async>,
    unsafety: Option<syn::token::Unsafe>,
    abi: Option<syn::Abi>,
}

enum FnQualifiersMeta {
    Single(FnQualifier),
    FnQualifiers(FnQualifiers),
}

impl Parse for FnQualifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Pub) {
            Ok(FnQualifier::Visibility(input.parse()?))
        } else if input.peek(syn::token::Const) {
            Ok(FnQualifier::Constness(input.parse()?))
        } else if input.peek(syn::token::Async) {
            Ok(FnQualifier::Asyncness(input.parse()?))
        } else if input.peek(syn::token::Unsafe) {
            Ok(FnQualifier::Unsafety(input.parse()?))
        } else if input.peek(syn::token::Extern) {
            Ok(FnQualifier::Abi(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "Expected a qualifier"))
        }
    }
}

impl Parse for FnQualifiers {
    // implement for parsing a list of qualifiers enclosed in square brackets
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);
        let mut visibility = None;
        let mut constness = None;
        let mut asyncness = None;
        let mut unsafety = None;
        let mut abi = None;
        while !content.is_empty() {
            let qualifier = content.parse::<FnQualifier>()?;
            match qualifier {
                FnQualifier::Visibility(visibility_) => {
                    if visibility.is_some() {
                        return Err(syn::Error::new(visibility_.span(), "Duplicate visibility qualifier"));
                    }
                    visibility = Some(visibility_);
                }
                FnQualifier::Constness(constness_) => {
                    if constness.is_some() {
                        return Err(syn::Error::new(constness_.span(), "Duplicate const qualifier"));
                    }
                    constness = Some(constness_);
                }
                FnQualifier::Asyncness(asyncness_) => {
                    if asyncness.is_some() {
                        return Err(syn::Error::new(asyncness_.span(), "Duplicate async qualifier"));
                    }
                    asyncness = Some(asyncness_);
                }
                FnQualifier::Unsafety(unsafety_) => {
                    if unsafety.is_some() {
                        return Err(syn::Error::new(unsafety_.span(), "Duplicate unsafe qualifier"));
                    }
                    unsafety = Some(unsafety_);
                }
                FnQualifier::Abi(abi_) => {
                    if abi.is_some() {
                        return Err(syn::Error::new(abi_.span(), "Duplicate extern qualifier"));
                    }
                    abi = Some(abi_);
                }
            }
            if !content.is_empty() {
                content.parse::<syn::token::Comma>()?;
            }
        }
        Ok(FnQualifiers {
            visibility,
            constness,
            asyncness,
            unsafety,
            abi,
        })
    }
}

impl Parse for FnQualifiersMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            Ok(FnQualifiersMeta::FnQualifiers(input.parse()?))
        } else {
            Ok(FnQualifiersMeta::Single(input.parse()?))
        }
    }
}

#[proc_macro_attribute]
pub fn fn_qualifiers(meta: pm::TokenStream, func: pm::TokenStream) -> pm::TokenStream {
    let meta = syn::parse_macro_input!(meta as FnQualifiersMeta);
    let mut func = syn::parse_macro_input!(func as syn::ItemFn);
    match meta {
        FnQualifiersMeta::Single(qualifier) => {
            match qualifier {
                FnQualifier::Visibility(visibility) => {
                    func.vis = visibility;
                }
                FnQualifier::Constness(constness) => {
                    func.sig.constness = Some(constness);
                }
                FnQualifier::Asyncness(asyncness) => {
                    func.sig.asyncness = Some(asyncness);
                }
                FnQualifier::Unsafety(unsafety) => {
                    func.sig.unsafety = Some(unsafety);
                }
                FnQualifier::Abi(abi) => {
                    func.sig.abi = Some(abi);
                }
            }
        },
        FnQualifiersMeta::FnQualifiers(fn_qualifiers) => {
            if let Some(visibility) = fn_qualifiers.visibility {
                func.vis = visibility;
            }
            func.sig.constness = fn_qualifiers.constness;
            func.sig.asyncness = fn_qualifiers.asyncness;
            func.sig.unsafety = fn_qualifiers.unsafety;
            func.sig.abi = fn_qualifiers.abi;
        }
    }

    func.into_token_stream().into()
}