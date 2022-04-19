use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());

    let method = def.invoke_method();
    let signature = method.signature(&[]);
    let _return_type = gen_return_sig(&signature, gen);
    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    let _params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.def);
        let tokens = gen_default_type(&p.ty, gen);
        quote! { #name: #tokens }
    });

    // TODO: implement callback wrapper
    // (how does one do it without variadic generics? Maybe use tuples?)

    // pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_type>;

    quote! {
        #doc
        #features
        pub type #name = ::core::option::Option<()/* TODO: STUB STUB STUB */>;
    }
}
