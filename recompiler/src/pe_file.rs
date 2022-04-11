#[cfg(feature = "mmap")]
use crate::error::Error;
use crate::error::Result;
use itertools::Itertools;
#[cfg(feature = "mmap")]
use memmap2::Mmap;
use object::read::pe::PeFile32;
use object::{LittleEndian, Object, ObjectSymbol, ObjectSymbolTable, SymbolKind as ObjSymbolKind};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use stable_deref_trait::StableDeref;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;
#[cfg(feature = "mmap")]
use std::{fs::File, path::Path};
use yoke::{Yoke, Yokeable};

pub const LE: LittleEndian = LittleEndian {};

pub struct PeFileWrapper<'a>(pub PeFile32<'a>);

unsafe impl<'a> Yokeable<'a> for PeFileWrapper<'static> {
    type Output = PeFileWrapper<'a>;

    fn transform(&'a self) -> &'a Self::Output {
        self
    }

    fn transform_owned(self) -> Self::Output {
        self
    }

    // pizdetc
    unsafe fn make(from: Self::Output) -> Self {
        // SAFETY: спаси и сохрани

        // We're just doing mem::transmute() here, however Rust is
        // not smart enough to realize that Bar<'a> and Bar<'static> are of
        // the same size, so instead we use transmute_copy

        // This assert will be optimized out, but is included for additional
        // peace of mind as we are using transmute_copy
        debug_assert!(std::mem::size_of::<Self::Output>() == std::mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        std::mem::forget(from);
        std::ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        // SAFETY: спаси и сохрани
        unsafe { f(std::mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

impl<'a> Deref for PeFileWrapper<'a> {
    type Target = PeFile32<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum PeFileCart {
    #[cfg(feature = "mmap")]
    Mmap(Mmap),
    Vec(Vec<u8>),
    StaticBuf(&'static [u8]),
}

impl Deref for PeFileCart {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            #[cfg(feature = "mmap")]
            PeFileCart::Mmap(mmap) => &*mmap,
            PeFileCart::Vec(vec) => vec,
            PeFileCart::StaticBuf(buf) => buf,
        }
    }
}

unsafe impl StableDeref for PeFileCart {}

impl From<Vec<u8>> for PeFileCart {
    fn from(vec: Vec<u8>) -> Self {
        PeFileCart::Vec(vec)
    }
}

#[cfg(feature = "mmap")]
impl From<Mmap> for PeFileCart {
    fn from(mmap: Mmap) -> Self {
        PeFileCart::Mmap(mmap)
    }
}

impl From<&'static [u8]> for PeFileCart {
    fn from(buf: &'static [u8]) -> Self {
        PeFileCart::StaticBuf(buf)
    }
}

type PeFileYoke = Yoke<PeFileWrapper<'static>, PeFileCart>;

fn copy_data(yoke: &PeFileYoke) -> ByteBuf {
    ByteBuf::from(yoke.backing_cart().deref().to_vec())
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PeFileYoke")]
struct PeFileYokeDef {
    #[serde(getter = "copy_data")]
    data: ByteBuf,
}

impl From<PeFileYokeDef> for PeFileYoke {
    fn from(def: PeFileYokeDef) -> Self {
        let data = def.data.into_vec();

        Yoke::try_attach_to_cart_badly::<object::Error>(PeFileCart::from(data), |c| {
            Ok(PeFileWrapper(PeFile32::parse(c)?))
        })
        .expect("Could not parse the serialized PE file")
    }
}

#[derive(Serialize, Deserialize)]
pub struct PeFile {
    // kinda ugly, huh?
    // we serialize PeFileYoke as a backing bytes in the PE file and parse it on deserialization
    // this can panic on bad inputs be we don't __precisely__ care about them... =)
    #[serde(with = "PeFileYokeDef")]
    yoke: PeFileYoke,
    name: String,
    externally_supplied_symbols: Option<BTreeMap<u32, PeSymbol>>,
}

impl PeFile {
    fn parse_from<T>(name: String, data: T) -> Result<Self>
    where
        PeFileCart: From<T>,
    {
        let yoke = Yoke::try_attach_to_cart_badly::<object::Error>(PeFileCart::from(data), |c| {
            Ok(PeFileWrapper(PeFile32::parse(c)?))
        })?;
        Ok(PeFile {
            yoke,
            name,
            externally_supplied_symbols: None,
        })
    }

    #[cfg(feature = "mmap")]
    pub fn parse_from_path(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path).map_err(Error::FileOpen)?;
        Self::parse_from_file(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_ascii_lowercase(),
            &file,
        )
    }

    #[cfg(feature = "mmap")]
    pub fn parse_from_file(name: String, file: &File) -> Result<Self> {
        // SAFETY: the file is (hopefully) not modified during the execution
        let mmap = unsafe { Mmap::map(file) }.map_err(Error::FileMmap)?;
        Self::parse_from(name, mmap)
    }

    pub fn parse_from_memory(name: String, data: Vec<u8>) -> Result<Self> {
        Self::parse_from(name, data)
    }

    pub fn parse_from_static_memory(name: String, data: &'static [u8]) -> Result<Self> {
        Self::parse_from(name, data)
    }

    pub fn with_symbols(self, symbols: BTreeMap<u32, PeSymbol>) -> Self {
        Self {
            externally_supplied_symbols: Some(symbols),
            ..self
        }
    }

    pub fn get(&self) -> &PeFile32 {
        self.yoke.get()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_addr(&self) -> u32 {
        self.get().nt_headers().optional_header.image_base.get(LE)
    }

    pub fn entry(&self) -> u32 {
        self.get()
            .nt_headers()
            .optional_header
            .address_of_entry_point
            .get(LE)
    }

    #[allow(unused)]
    pub fn collect_symbols(&self) -> Option<BTreeMap<u32, PeSymbol>> {
        // TODO: should we merge the tables if multiple are found?
        if let Some(symbols) = &self.externally_supplied_symbols {
            return Some(symbols.clone());
        }

        if let Some(symtable) = self.get().symbol_table() {
            return Some(
                symtable
                    .symbols()
                    .filter(|sym| {
                        sym.address() != 0
                            && matches!(sym.kind(), ObjSymbolKind::Data | ObjSymbolKind::Text)
                    })
                    .map(|sym| {
                        (
                            sym.address() as u32 - self.base_addr(),
                            PeSymbol {
                                name: sym.name().unwrap().to_string(),
                                kind: match sym.kind() {
                                    ObjSymbolKind::Text => SymbolKind::Code,
                                    ObjSymbolKind::Data => SymbolKind::Data,
                                    _ => unreachable!(),
                                },
                            },
                        )
                    })
                    .sorted_by_key(|&(addr, _)| addr)
                    .unique()
                    .collect::<BTreeMap<_, _>>(),
            );
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoadedPeInfo {
    pub base_addr: u32,
    pub image_size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Code,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeSymbol {
    pub name: String,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessImageSymbol {
    // serde will dedup these Arc's on deserialization =(
    // maybe writing a custom Deserialize will help :shrug:
    pub module: Arc<str>,
    pub symbol: String,
    pub kind: SymbolKind,
}
