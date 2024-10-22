use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn from_bytes(value: &[u8]) -> Self {
                assert!(
                    value.len() >= std::mem::size_of::<Self>(),
                    "`value` should be at least the size of T"
                );

                let mut result: Self = unsafe { std::mem::zeroed() };
                let result_pointer: *mut Self = &mut result;

                unsafe {
                    std::ptr::copy_nonoverlapping(
                        value.as_ptr() as *const u8,
                        result_pointer as *mut u8,
                        std::mem::size_of::<Self>()
                    );
                }

                result
            }
        }
    };

    TokenStream::from(expanded)
}
