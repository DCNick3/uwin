use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum Thunk<'a> {
    PlainFunction(&'a str),
    ComMethod {
        object: &'a str,
        interface: &'a str,
        method: &'a str,
    },
}

impl<'a> Thunk<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Thunk::PlainFunction(name) => format!("dll_{}", name),
            Thunk::ComMethod {
                object,
                interface,
                method,
            } => format!("com_{}_as_{}_{}", object, interface, method),
        }
    }
}

/// Provides an abstraction to allocate thunk ids over different parts of the stub generators
pub struct ThunkIdAllocator {
    next_thunk_id: u32,
    thunk_id_names: BTreeMap<u32, String>,
    thunk_names_to_id: BTreeMap<String, u32>,
}

impl ThunkIdAllocator {
    pub fn new() -> Self {
        ThunkIdAllocator {
            next_thunk_id: 1,
            thunk_id_names: BTreeMap::new(),
            thunk_names_to_id: BTreeMap::new(),
        }
    }

    pub fn get_plain_function(&mut self, name: &str) -> u32 {
        // TODO: this can be optimized, but copying is easier for now=)
        let thunk = Thunk::PlainFunction(name);
        let thunk_name = thunk.to_string();

        if let Some(&id) = self.thunk_names_to_id.get(&thunk_name) {
            id
        } else {
            self.next(thunk)
        }
    }

    pub fn next(&mut self, thunk: Thunk) -> u32 {
        let thunk_name = thunk.to_string();

        let res = self.next_thunk_id;
        self.next_thunk_id += 1;

        assert!(self
            .thunk_id_names
            .insert(res, thunk_name.clone())
            .is_none());
        assert!(self.thunk_names_to_id.insert(thunk_name, res).is_none());

        res
    }

    pub fn to_thunk_functions(self) -> BTreeMap<u32, String> {
        self.thunk_id_names
    }
}
