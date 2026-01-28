use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn info(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &function.sig.ident;

    quote! {
        #[::abi_stable::sabi_extern_fn]
        fn ffi_internal_info() -> ::ffi_plugin::ffi_interface::PluginInfo {
            #function

            #fn_name()
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &function.sig.ident;

    quote! {
        #[::abi_stable::sabi_extern_fn]
        fn ffi_internal_version() -> ::abi_stable::std_types::RString {
            #function

            #fn_name()
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &function.sig.ident;

    quote! {
        #[::abi_stable::export_root_module]
        fn ffi_internal_init_root_module() -> ::ffi_plugin::ffi_interface::PluginRef {
            use ::abi_stable::prefix_type::PrefixTypeTrait;

            ::ffi_plugin::ffi_interface::Plugin {
                init: ffi_internal_init,
                info: ffi_internal_info,
                version: ffi_internal_version,
            }
            .leak_into_prefix()
        }

        #[::abi_stable::sabi_extern_fn]
        fn ffi_internal_init() {
            #function

            #fn_name()
        }
    }
    .into()
}
