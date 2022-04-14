use super::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct Gen<'a> {
    pub enabled_namespaces: &'a [&'a str],
    pub excluded_items: HashSet<&'a str>,
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

    pub(crate) fn is_cfg_enabled(&self, cfg: &Cfg) -> bool {
        let arches = cfg.arches();
        let features = cfg.features(self.namespace);

        let arch_enabled = arches.is_empty() || arches.contains("x86");
        let features_enabled =
            features.is_empty() || features.iter().all(|f| self.enabled_namespaces.contains(f));

        arch_enabled && features_enabled
    }

    pub(crate) fn cfg(&self, cfg: &Cfg) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let features = cfg.features(self.namespace);

            let features: Vec<String> = features.into_iter().map(|f| format!("'{}'", f)).collect();
            let tokens = features.join(", ");

            let required_namespaces: TokenStream =
                format!(r#"#[doc = "*Required namespaces: {}*"]"#, tokens).into();

            let enabled = if self.is_cfg_enabled(cfg) {
                quote! {}
            } else {
                quote! {
                    #required_namespaces
                    #[cfg(dummy_option_that_does_not_exist)]
                }
            };

            quote! { #enabled }
        }
    }
}
