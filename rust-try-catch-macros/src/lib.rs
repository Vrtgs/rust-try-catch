use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{parse_macro_input, parse_quote, ExprClosure, ItemFn};
use syn::spanned::Spanned;
use crate::parse::FnOrClosure;

mod parse;


#[proc_macro_attribute]
pub fn throw_guard(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    if !attr.is_empty() {
        let message = format!("Unexpected attributes \"{attr}\"");
        return syn::Error::new(TokenStream::from(attr).span(), message)
            .into_compile_error()
            .into()
    }
    
    let tokens = match parse_macro_input!(item as FnOrClosure) {
        FnOrClosure::Function(func) => wrap_func(func),
        FnOrClosure::Closure(closure) => wrap_closure(closure),
    };
    
    tokens.into()
}


#[proc_macro]
pub fn closure_throw_guard(item: TokenStream1) -> TokenStream1 {
    wrap_closure(parse_macro_input!(item as ExprClosure)).into()
}

fn throw_guard_body(async_ness: Option<&syn::Token![async]>, fn_body: TokenStream) -> TokenStream {
    let span = fn_body.span();
    match &async_ness {
        Some(_) => quote_spanned! { span=>
            {
                let mut fut = ::core::pin::pin!(async { #fn_body });
                ::core::future::poll_fn(move |cx| {
                    ::rust_try_catch::__throw_driver(|| {
                        ::core::future::Future::poll(::core::pin::Pin::as_mut(&mut fut), cx)
                    })
                }).await
            }
        },
        None => quote_spanned! { span=> ::rust_try_catch::__throw_driver(|| #fn_body) },
    }
}

fn wrap_closure(closure: ExprClosure) -> TokenStream {
    if let Some(token) = closure.constness {
        return syn::Error::new(token.span, "can't drive try catch logic in const")
            .into_compile_error()
    }
    let body_tokens = throw_guard_body(closure.asyncness.as_ref(), closure.body.into_token_stream());
    
    let closure = ExprClosure {
        body: parse_quote! { #body_tokens },
        ..closure
    };

    closure.into_token_stream()
}

fn wrap_func(input: ItemFn) -> TokenStream {
    if let Some(token) = input.sig.constness {
        return syn::Error::new(token.span, "can't drive try catch logic in const")
            .into_compile_error()
    }
    
    if let Some(variadic) = input.sig.variadic { 
        return syn::Error::new(variadic.span(), "using variadic arguments would cause UB!!!")
            .into_compile_error()
    }
    
    let outer_fn_body = throw_guard_body(input.sig.asyncness.as_ref(), input.block.into_token_stream());
    
    let new_fn = ItemFn {
        attrs: input.attrs,
        vis: input.vis,
        sig: input.sig,
        block: parse_quote!({ #outer_fn_body }),
    };
    
    new_fn.into_token_stream()
}