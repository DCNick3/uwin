mod r#impl;

use proc_macro::TokenStream;

#[proc_macro]
pub fn test_snippets(input: TokenStream) -> TokenStream {
    r#impl::test_snippets_impl(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn test_functions(input: TokenStream) -> TokenStream {
    r#impl::test_functions_impl(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn test_elf_functions(input: TokenStream) -> TokenStream {
    r#impl::test_elf_functions_impl(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
