use super::*;

#[allow(unused)]
pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = quote! {};

    for (position, param) in params.iter().enumerate() {
        let name = gen_param_name(&param.def);
        // let kind = if param.is_convertible() {
        //     format!("Param{}", position).into()
        // } else {
        let kind = gen_default_type(&param.ty, gen);
        // };
        tokens.combine(&quote! { #name: #kind, });
    }

    tokens
}

#[allow(unused)]
pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.def);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::core::mem::transmute(#name) }
    }
}
