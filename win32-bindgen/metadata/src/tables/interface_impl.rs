use super::*;

#[derive(Clone)]
pub struct InterfaceImpl(pub Row);

impl InterfaceImpl {
    pub fn interface(&self) -> TypeDefOrRef {
        self.0.decode(1)
    }

    fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.file.attributes(HasAttribute::InterfaceImpl(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_default(&self) -> bool {
        self.has_attribute("DefaultAttribute")
    }

    pub fn is_overridable(&self) -> bool {
        self.has_attribute("OverridableAttribute")
    }

    // TODO: what if default interface is IInspectable?
    pub fn generic_interface(&self, generics: &[Type]) -> Type {
        TypeReader::get().type_from_code(&self.interface(), None, generics)
    }
}
