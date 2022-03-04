use super::*;

#[derive(Default)]
pub struct Gen<'a> {
    pub namespace: &'a str,
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
    pub doc: bool,
    pub min_enum: bool,
    pub min_inherit: bool,
    pub min_xaml: bool,
}

impl Gen<'_> {
    pub(crate) fn namespace(&self, namespace: &str) -> TokenStream {
        if self.flatten || namespace == self.namespace {
            quote! {}
        } else {
            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::new();

            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }

    pub(crate) fn doc(&self, _cfg: &Cfg) -> TokenStream {
        // Required Features was generated here. In uwin we don't care
        quote! {}
    }

    pub(crate) fn cfg(&self, _cfg: &Cfg) -> TokenStream {
        quote! {}
    }
}

fn to_feature(name: &str) -> String {
    let mut feature = String::new();

    for name in name.split('.').skip(1) {
        feature.push_str(name);
        feature.push('_');
    }

    if feature.is_empty() {
        feature = name.to_string();
    } else {
        feature.truncate(feature.len() - 1);
    }

    feature
}
