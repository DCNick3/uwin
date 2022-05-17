pub struct ComStubParams {
    pub classes: Vec<ComStubClass>,
}

pub struct ComStubClass {
    pub name: String,
    pub vtables: Vec<ComStubVtable>,
}

pub struct ComStubVtable {
    pub thunk_names: Vec<String>,
}
