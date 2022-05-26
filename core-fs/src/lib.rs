pub mod error;

use crate::error::{CreateError, OpenError};
use arcstr::ArcStr;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use unicase::UniCase;

pub struct StaticFile {
    attributes: FsAttributes,
    contents: &'static [u8],
}

impl Debug for StaticFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticFile")
            .field("attributes", &self.attributes)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub enum FsTree {
    ArchiveDir(Arc<BTreeMap<UniCase<ArcStr>, FsNodeRef>>),
    FsDir(ArcStr),
}

impl FsTree {
    /// Tries to lookup an immediate child node with specified name
    ///
    pub fn lookup_node(&self, name: &ArcStr) -> Option<FsNodeRef> {
        match self {
            FsTree::ArchiveDir(dir) => dir.get(&UniCase::new(name.clone())).cloned(),
            FsTree::FsDir(_dir) => {
                todo!()
            }
        }
    }
    pub fn list_names(&self) -> impl Iterator<Item = impl AsRef<str> + '_> + '_ {
        match self {
            FsTree::ArchiveDir(entries) => entries.keys(),
            FsTree::FsDir(_) => {
                todo!()
            }
        }
    }
    pub fn create_node(&self, _name: &str) -> Result<FsNodeRef, CreateError> {
        match self {
            FsTree::ArchiveDir(_) => Err(CreateError::Readonly),
            FsTree::FsDir(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum FsFileRef {
    StaticFile(&'static StaticFile),
    FsFile(
        ArcStr, /* use different representation? Maybe split the prefix & filename to opt the memory usage? */
    ),
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
        }
    }

    pub fn read(mut self, read: bool) -> Self {
        self.read = read;
        self
    }
    pub fn write(mut self, write: bool) -> Self {
        self.write = write;
        self
    }
    pub fn append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl From<OpenOptions> for std::fs::OpenOptions {
    fn from(opt: OpenOptions) -> Self {
        let mut res = std::fs::OpenOptions::new();
        res.read(opt.read).write(opt.write).append(opt.truncate);
        res
    }
}

impl FsFileRef {
    pub fn open(&self, options: OpenOptions) -> Result<FileHandle, OpenError> {
        match self {
            FsFileRef::StaticFile(file) => {
                if options.write || options.append || options.truncate {
                    return Err(OpenError::Readonly);
                }
                if !options.read {
                    return Err(OpenError::InvalidOptions);
                }
                Ok(FileHandle::StaticFileHandle { pos: 0, file: file })
            }
            FsFileRef::FsFile(path) => std::fs::OpenOptions::from(options)
                .open(path.as_str())
                .map(FileHandle::FsFileHandle)
                .map_err(|_e| todo!("Map real fs errors to OpenError")),
        }
    }
}

#[derive(Debug)]
pub enum FileHandle {
    StaticFileHandle {
        pos: usize,
        file: &'static StaticFile,
    },
    FsFileHandle(std::fs::File),
}

#[derive(Clone, Debug)]
pub enum FsNodeRef {
    FsTree(FsTree),
    File(FsFileRef),
}

#[derive(Clone, Debug)]
pub struct FsAttributes {}

impl FsNodeRef {
    pub fn attrs(&self) -> FsAttributes {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        CreateError, FsAttributes, FsFileRef, FsNodeRef, FsTree, OpenError, OpenOptions, StaticFile,
    };
    use cool_asserts::assert_matches;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use unicase::UniCase;

    fn get_empty_archive() -> FsTree {
        FsTree::ArchiveDir(Arc::new(BTreeMap::from([])))
    }

    fn get_simple_archive() -> FsTree {
        FsTree::ArchiveDir(Arc::new(BTreeMap::from([
            (
                UniCase::new(arcstr::literal!("hello.txt")),
                FsNodeRef::File(FsFileRef::StaticFile(&StaticFile {
                    attributes: FsAttributes {},
                    contents: b"Hello world, this is some static file contents",
                })),
            ),
            (
                UniCase::new(arcstr::literal!("ENA.PNG")),
                FsNodeRef::File(FsFileRef::StaticFile(&StaticFile {
                    attributes: FsAttributes {},
                    contents: include_bytes!("ena.png"),
                })),
            ),
            (
                UniCase::new(arcstr::literal!("directory")),
                FsNodeRef::FsTree(FsTree::ArchiveDir(Arc::new(BTreeMap::from([(
                    UniCase::new(arcstr::literal!("subhello.txt")),
                    FsNodeRef::File(FsFileRef::StaticFile(&StaticFile {
                        attributes: FsAttributes {},
                        contents: b"Hello from subdirectory!",
                    })),
                )])))),
            ),
        ])))
    }

    #[test]
    fn empty_archive() {
        let empty = get_empty_archive();

        assert_matches!(empty.create_node("some_file"), Err(CreateError::Readonly));
        {
            let mut iter = empty.list_names();
            assert!(iter.next().is_none())
        }
        assert_matches!(empty.lookup_node(&arcstr::literal!("hello.txt")), None);
    }

    #[test]
    fn simple_archive() {
        let arc = get_simple_archive();

        assert_matches!(arc.create_node("some_file"), Err(CreateError::Readonly));
        {
            let mut iter = arc.list_names();
            assert_eq!(iter.next().unwrap().as_ref(), "directory");
            assert_eq!(iter.next().unwrap().as_ref(), "ENA.PNG");
            assert_eq!(iter.next().unwrap().as_ref(), "hello.txt");
            assert!(iter.next().is_none())
        }
        assert_matches!(arc.lookup_node(&arcstr::literal!("nonexistent")), None);
        {
            let hello_txt = arc.lookup_node(&arcstr::literal!("HELlO.txt")).unwrap();
            if let FsNodeRef::File(file) = hello_txt {
                assert_matches!(
                    file.open(OpenOptions::new().write(true)),
                    Err(OpenError::Readonly)
                );

                let _file = file.open(OpenOptions::new().read(true)).unwrap();
                // TODO: test reading from this file
            } else {
                panic!("Expected to get a file");
            }
        }
        {
            let ena_png = arc.lookup_node(&arcstr::literal!("Ena.Png")).unwrap();
            if let FsNodeRef::File(_) = ena_png {
            } else {
                panic!("Expected to get a file");
            }
        }
        {
            let directory = arc.lookup_node(&arcstr::literal!("directory")).unwrap();
            if let FsNodeRef::FsTree(_) = directory {
            } else {
                panic!("Expected to get a directory");
            }
        }
    }
}
