use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(attr);
    let function = parse_macro_input!(item as ItemFn);

    let name = &function.sig.ident;

    quote! {
        #[poise::command(#args)]
        #function

        ::poise_collect::inventory::submit! {
            crate::RegisteredCommand {
                constructor: #name,
            }
        }
    }
    .into()
}
