use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());

    let method = def.invoke_method();
    let signature = method.signature(&[]);

    let return_type = if let Some(return_type) = &signature.return_type {
        gen_default_type(return_type, gen)
    } else {
        quote! { () }
    };

    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    let params = signature
        .params
        .iter()
        .map(|p| {
            let tokens = gen_default_type(&p.ty, gen);
            quote! { #tokens }
        })
        .collect::<Vec<_>>();

    // TODO: implement callback wrapper
    // (how does one do it without variadic generics? Maybe use tuples?)

    // pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_type>;

    let param_tuple = if !params.is_empty() {
        quote! { (#(#params),*,) }
    } else {
        quote! { () }
    };

    quote! {
        #doc
        #features
        pub type #name = StdCallFnPtr<#param_tuple, #return_type>;
    }
}
