use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum SupportedOs {
    Windows2000,
    WindowsXp,
    WindowsVista,
    Windows7,
    Windows8,
    Windows81,
    Windows10,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef(pub Row);

impl From<Row> for MethodDef {
    fn from(row: Row) -> Self {
        Self(row)
    }
}

impl MethodDef {
    pub fn is_special(&self) -> bool {
        self.0.u32(2) & 0b1000_0000_0000 != 0
    }

    pub fn preserve_sig(&self) -> bool {
        self.0.u32(1) & 0b1000_0000 != 0
    }

    pub fn params(&self) -> impl Iterator<Item = Param> {
        self.0.list(5, TableIndex::Param).map(Param)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }

    pub fn rust_name(&self) -> String {
        let name = self.name();

        if self.is_special() {
            if name.starts_with("get") {
                name[4..].to_string()
            } else if name.starts_with("put") {
                format!("Set{}", &name[4..])
            } else if name.starts_with("add") {
                name[4..].to_string()
            } else if name.starts_with("remove") {
                format!("Remove{}", &name[7..])
            } else {
                // A delegate's 'Invoke' method is "special" but lacks a preamble.
                "Invoke".to_string()
            }
        } else {
            for attribute in self.attributes() {
                if attribute.name() == "OverloadAttribute" {
                    for (_, arg) in attribute.args() {
                        if let ConstantValue::String(name) = arg {
                            return name;
                        }
                    }
                }
            }

            name.to_string()
        }
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0
            .file
            .attributes(HasAttribute::MethodDef(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_deprecated(&self) -> bool {
        self.has_attribute("DeprecatedAttribute")
    }

    pub fn does_not_return(&self) -> bool {
        self.has_attribute("DoesNotReturnAttribute")
    }

    pub fn supported_os(&self) -> Option<SupportedOs> {
        self.attributes()
            .filter_map(|attribute| match attribute.name() {
                "SupportedOSPlatformAttribute" => {
                    let args = attribute.args();
                    match &args[0].1 {
                        ConstantValue::String(value) => Some(match value.as_str() {
                            "windows5.0" => SupportedOs::Windows2000,
                            "windows5.1.2600" => SupportedOs::WindowsXp,
                            "windows6.0.6000" => SupportedOs::WindowsVista,
                            "windows6.1" => SupportedOs::Windows7,
                            "windows8.0" => SupportedOs::Windows8,
                            "windows8.1" => SupportedOs::Windows81,
                            v if v.starts_with("windows10.0") => SupportedOs::Windows10,
                            v => panic!("Unknown SupportedOSPlatformAttribute: {}", v),
                        }),
                        _ => None,
                    }
                }
                _ => None,
            })
            .next()
    }

    pub fn static_lib(&self) -> Option<String> {
        self.attributes()
            .filter_map(|attribute| match attribute.name() {
                "StaticLibraryAttribute" => {
                    let args = attribute.args();
                    match &args[0].1 {
                        ConstantValue::String(value) => Some(value.clone()),
                        _ => None,
                    }
                }
                _ => None,
            })
            .next()
    }

    pub fn dll_import(&self) -> Option<&'static str> {
        Some(self.impl_map()?.scope().name())
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.0
            .file
            .equal_range(
                TableIndex::ImplMap,
                1,
                MemberForwarded::MethodDef(self.clone()).encode(),
            )
            .map(ImplMap)
            .next()
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        let reader = TypeReader::get();
        let params = self.params();

        let mut blob = self.0.blob(4);
        blob.read_unsigned();
        blob.read_unsigned(); // parameter count

        let return_type = reader.type_from_blob(&mut blob, None, generics);
        let mut return_param = None;
        let preserve_sig = self.preserve_sig();

        let params = params
            .filter_map(|def| {
                if def.sequence() == 0 {
                    return_param = Some(def);
                    None
                } else {
                    let ty = reader
                        .type_from_blob(&mut blob, None, generics)
                        .expect("MethodDef");
                    let ty = if !def.flags().output() {
                        ty.to_const()
                    } else {
                        ty
                    };
                    Some(MethodParam { def, ty })
                }
            })
            .collect();

        Signature {
            params,
            return_type,
            return_param,
            preserve_sig,
        }
    }

    pub fn cfg(&self) -> Cfg {
        let mut cfg = Cfg::new();
        self.combine_cfg(&mut cfg);
        cfg.add_attributes(self.attributes());
        cfg
    }

    pub(crate) fn combine_cfg(&self, cfg: &mut Cfg) {
        self.signature(&[]).combine_cfg(cfg);
    }
}
