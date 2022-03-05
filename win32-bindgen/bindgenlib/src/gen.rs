use super::*;

#[derive(Default)]
pub struct Gen<'a> {
    pub enabled_namespaces: &'a [&'a str],
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

    pub(crate) fn cfg(&self, cfg: &Cfg) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let arches = cfg.arches();
            let features = cfg.features(self.namespace);

            let arch_enabled = arches.is_empty() || arches.contains("x86");
            let features_enabled =
                features.is_empty() || features.iter().all(|f| self.enabled_namespaces.contains(f));

            let enabled = if arch_enabled && features_enabled {
                quote! {}
            } else {
                quote! {
                    // not for x86
                    #[cfg(dummy_option_that_does_not_exist)]
                }
            };

            quote! { #enabled }
        }
    }
}
