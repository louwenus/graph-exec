use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Stmt};

#[proc_macro_attribute]
pub fn tomasulo(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as syn::ItemFn);

    let fn_name = f.sig.ident.clone();
    let args = f.sig.inputs.iter().map(|arg| match arg {
        syn::FnArg::Receiver(receiver) => match receiver.mutability {
            None => quote! { #receiver },
            Some(_) => panic!("Mutable reference aren't allowed"),
        },
        syn::FnArg::Typed(pat_type) => {
            if let syn::Type::Reference(type_reference) = &*pat_type.ty {
                if type_reference.mutability.is_some() {
                    panic!("Mutable reference aren't allowed")
                }
            }
            let arg = pat_type.pat.clone();
            quote! { #arg }
        }
    });

    let mut custom_fn = f.clone();
    let call = quote! {
        #fn_name(#(#args),*)
    }
    .into();
    custom_fn.block.stmts = vec![Stmt::Expr(parse_macro_input!(call as syn::Expr), None)];

    f.sig.ident = format_ident!("{}_user", fn_name);
    quote::quote! {
        #custom_fn
        #f
    }
    .into()
}
