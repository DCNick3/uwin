use case_insensitive_hashmap::CaseInsensitiveHashMap;
use core_handletable::{Handle, HandleTable};
use std::borrow::Cow;
use unicase::UniCase;

struct AtomStorage {
    value: String,
    reference_count: u32,
}

#[derive(Copy, Clone)]
struct Atom(u16);

impl From<u16> for Atom {
    fn from(atom: u16) -> Self {
        Atom(atom)
    }
}

impl From<Atom> for u16 {
    fn from(atom: Atom) -> Self {
        atom.0
    }
}

const MAXINTATOM: u16 = win32::Win32::System::WindowsProgramming::MAXINTATOM as u16;

impl Handle for Atom {
    fn to_index(self, _: &()) -> usize {
        assert!(self.0 >= MAXINTATOM);
        (self.0 - MAXINTATOM) as usize
    }

    fn from_index(index: usize, _: &()) -> Self {
        let index: u16 = index.try_into().unwrap();
        index.checked_add(MAXINTATOM).unwrap().into()
    }
}

pub struct AtomTable {
    table: HandleTable<(), Atom, AtomStorage, false>,
    value_to_atom: CaseInsensitiveHashMap<Atom>,
}

impl AtomTable {
    pub fn new() -> Self {
        Self {
            table: HandleTable::new((), 0x10000 - MAXINTATOM as usize),
            value_to_atom: CaseInsensitiveHashMap::new(),
        }
    }

    pub fn add(&mut self, value: &str) -> u16 {
        if value.starts_with('#') {
            todo!("Integer atoms")
        }

        match self.value_to_atom.get(value) {
            None => {
                let atom = self.table.put(AtomStorage {
                    value: value.to_string(),
                    reference_count: 1,
                });
                assert!(self
                    .value_to_atom
                    //                          \/ we duplicate this string... Not optimal, but kinda meh
                    .insert(UniCase::unicode(value.to_string()), atom)
                    .is_none());
                atom.into()
            }
            Some(&value) => {
                let entry = self.table.find_mut(value).unwrap();
                entry.reference_count += 1;
                value.into()
            }
        }
    }

    pub fn find_value(&self, atom: u16) -> Option<Cow<str>> /* Cow for the case of integer atoms */
    {
        if atom < MAXINTATOM {
            todo!("Integer atoms")
        }

        self.table
            .find(atom.into())
            .map(|s| s.value.as_str().into())
    }

    pub fn remove(&mut self, atom: u16) -> bool {
        if atom < MAXINTATOM {
            todo!("Integer atoms")
        }

        let entry = match self.table.find_mut(atom.into()) {
            Some(entry) => entry,
            None => return false,
        };

        entry.reference_count -= 1;

        if entry.reference_count == 0 {
            let storage = self.table.remove(atom.into()).unwrap();
            self.value_to_atom.remove(storage.value).unwrap();
        }

        true
    }
}

impl Default for AtomTable {
    fn default() -> Self {
        AtomTable::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{AtomTable, MAXINTATOM};

    #[test]
    fn basic() {
        let mut atom_table = AtomTable::new();

        let atom1 = atom_table.add("value");
        let atom2 = atom_table.add("VaLuE");
        assert_eq!(atom1, atom2);
        assert!(atom1 >= MAXINTATOM);

        assert_eq!(atom_table.find_value(atom1).unwrap(), "value");

        assert!(atom_table.remove(atom1));
        assert!(atom_table.remove(atom1));
        assert!(!atom_table.remove(atom1));
    }
}
