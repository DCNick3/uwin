use super::*;

pub fn gen_classes(gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    let type_reader = TypeReader::get();

    for class in gen.com_classes {
        tokens.combine(&gen_class(class, gen, type_reader))
    }

    tokens
}

pub fn gen_class_thunks(gen: &Gen) -> TokenStream {
    todo!()
}

fn gen_class(class: &ComClass, _gen: &Gen, type_reader: &TypeReader) -> TokenStream {
    // Classes would be really different in uwin (if they would be there at all)
    // don't expect to get them working for now

    quote! {}
}
