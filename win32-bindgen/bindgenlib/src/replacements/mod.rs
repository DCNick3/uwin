use super::*;
mod bool32;
mod handle;

pub fn gen(def: &TypeDef) -> Option<TokenStream> {
    match def.type_name() {
        TypeName::BOOL => Some(bool32::gen()),
        TypeName::HANDLE => Some(handle::gen()),
        _ => None,
    }
}
