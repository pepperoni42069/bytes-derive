use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn from_bytes(value: &[u8]) -> #struct_name {
                assert!(
                    value.len() >= std::mem::size_of::<#struct_name>(),
                    "`value` should be at least the size of T"
                );

                let mut result: #struct_name = unsafe { std::mem::zeroed() };
                let result_pointer: *mut #struct_name = &mut result;

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        value.as_ptr() as *const u8,
                        result_pointer as *mut u8,
                        std::mem::size_of::<#struct_name>()
                    );
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}
