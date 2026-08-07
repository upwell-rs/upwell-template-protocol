extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::ItemStruct;
use upwell_macros_core::attr::ComponentArgs;
use upwell_macros_core::paths::Paths;

/// Declares a component using this protocol's public crate roots.
#[proc_macro_attribute]
pub fn protocol_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = match syn::parse2::<ComponentArgs>(TokenStream2::from(attr)) {
        Ok(args) => {
            let paths = args.paths(Paths::new(
                syn::parse_quote!(::upwell),
                syn::parse_quote!(::{{ crate_name }}),
            ));

            upwell_macros_core::run::<ItemStruct, _>(item.into(), |item| {
                upwell_macros_core::expand_component(args, item, &paths)
            })
        }
        Err(error) => error.into_compile_error(),
    };

    output.into()
}
