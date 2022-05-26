pub mod error;

use crate::error::{CreateError, OpenError, RemoveError};
use anyhow::Context;
use arcstr::ArcStr;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::io;
use std::sync::Arc;
use unicase::UniCase;

pub struct StaticFile {
    attributes: Attributes,
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
pub enum Tree {
    ArchiveDir(Arc<BTreeMap<UniCase<ArcStr>, NodeRef>>),
    FsDir(ArcStr),
}

impl Tree {
    /// Tries to lookup an immediate child node with specified name
    ///
    pub fn lookup_node(&self, needle: &ArcStr) -> Option<NodeRef> {
        match self {
            Tree::ArchiveDir(dir) => dir.get(&UniCase::new(needle.clone())).cloned(),
            Tree::FsDir(path) => {
                let readdir = std::fs::read_dir(path.as_str())
                    .with_context(|| format!("Opening directory {:?}", path))
                    .unwrap();

                // ASSUMPTION: no two files have the same name (ignoring case)
                for entry in readdir {
                    let entry = entry
                        .with_context(|| format!("Opening directory {:?}", path))
                        .unwrap();

                    let name = entry.file_name();
                    let name = name
                        .to_str()
                        .with_context(|| {
                            format!("File name {:?} is not a valid UTF-8 string", name)
                        })
                        .unwrap();

                    let name = UniCase::new(name);
                    if name == UniCase::new(needle.as_str()) {
                        let file_type = entry
                            .file_type()
                            .with_context(|| format!("Getting file type of {:?}", entry))
                            .unwrap();

                        let path = ArcStr::from(entry.path().to_str().unwrap());
                        return if file_type.is_file() {
                            Some(NodeRef::File(FileRef::FsFile(path)))
                        } else if file_type.is_dir() {
                            Some(NodeRef::FsTree(Tree::FsDir(path)))
                        } else {
                            unimplemented!("Unsupported file_type: {:?}", file_type)
                        };
                    }
                }

                None
            }
        }
    }
    pub fn list_names(&self) -> impl IntoIterator<Item = impl AsRef<str> + '_> + '_ {
        match self {
            Tree::ArchiveDir(entries) => entries.keys().cloned().collect::<Vec<_>>(),
            Tree::FsDir(path) => {
                let readdir = std::fs::read_dir(path.as_str())
                    .with_context(|| format!("Opening directory {:?}", path))
                    .unwrap();

                let mut list = readdir
                    .map(|entry| -> io::Result<_> {
                        let name = entry?.file_name();
                        let name = name
                            .to_str()
                            .with_context(|| {
                                format!("File name {:?} is not a valid UTF-8 string", name)
                            })
                            .unwrap();
                        Ok(UniCase::new(ArcStr::from(name)))
                    })
                    .collect::<io::Result<Vec<_>>>()
                    .with_context(|| format!("Reading directory {:?}", path))
                    .unwrap();

                list.sort();

                list
            }
        }
    }
    // this is (on one hand) creates a much cleaner API, closer to that of VFS
    // but, on the other hand, races between file creation and access are possible
    // we can try to lock the file in some way but probably will ignore races for now (because windows games don't care)
    pub fn create_node(&self, _name: &str, node_type: NodeType) -> Result<NodeRef, CreateError> {
        match self {
            Tree::ArchiveDir(_) => Err(CreateError::Readonly),
            Tree::FsDir(_) => {
                todo!()
            }
        }
    }

    pub fn remove_node(&self, _name: &str) -> Result<NodeRef, RemoveError> {
        match self {
            Tree::ArchiveDir(_) => Err(RemoveError::Readonly),
            Tree::FsDir(_) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileRef {
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

impl FileRef {
    pub fn open(&self, options: OpenOptions) -> Result<FileHandle, OpenError> {
        match self {
            FileRef::StaticFile(file) => {
                if options.write || options.append || options.truncate {
                    return Err(OpenError::Readonly);
                }
                if !options.read {
                    return Err(OpenError::InvalidOptions);
                }
                Ok(FileHandle::StaticFileHandle { pos: 0, file: file })
            }
            FileRef::FsFile(path) => std::fs::OpenOptions::from(options)
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
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub enum NodeRef {
    FsTree(Tree),
    File(FileRef),
}

#[derive(Clone, Debug)]
pub struct Attributes {}

impl NodeRef {
    pub fn attrs(&self) -> Attributes {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Attributes, CreateError, FileRef, NodeRef, NodeType, OpenError, OpenOptions, StaticFile,
        Tree,
    };
    use cool_asserts::assert_matches;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use tempfile::TempDir;
    use unicase::UniCase;

    fn get_empty_archive() -> Tree {
        Tree::ArchiveDir(Arc::new(BTreeMap::from([])))
    }

    fn get_simple_archive() -> Tree {
        Tree::ArchiveDir(Arc::new(BTreeMap::from([
            (
                UniCase::new(arcstr::literal!("hello.txt")),
                NodeRef::File(FileRef::StaticFile(&StaticFile {
                    attributes: Attributes {},
                    contents: b"Hewwo wowld, this is some static file contents",
                })),
            ),
            (
                UniCase::new(arcstr::literal!("ENA.PNG")),
                NodeRef::File(FileRef::StaticFile(&StaticFile {
                    attributes: Attributes {},
                    contents: include_bytes!("ena.png"),
                })),
            ),
            (
                UniCase::new(arcstr::literal!("directory")),
                NodeRef::FsTree(Tree::ArchiveDir(Arc::new(BTreeMap::from([(
                    UniCase::new(arcstr::literal!("subhello.txt")),
                    NodeRef::File(FileRef::StaticFile(&StaticFile {
                        attributes: Attributes {},
                        contents: b"Hello from subdirectory!",
                    })),
                )])))),
            ),
        ])))
    }

    fn get_simple_fs() -> (Tree, TempDir) {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        std::fs::write(
            path.join("hello.txt"),
            b"Hewwo wowld, this is some static file contents",
        )
        .unwrap();
        std::fs::write(path.join("ENA.PNG"), include_bytes!("ena.png")).unwrap();
        std::fs::create_dir(path.join("directory")).unwrap();
        std::fs::write(
            path.join("directory/subhello.txt"),
            b"Hello from subdirectory!",
        )
        .unwrap();

        (Tree::FsDir(path.to_str().unwrap().into()), dir)
    }

    #[test]
    fn empty_archive() {
        let fs = get_empty_archive();

        assert_matches!(
            fs.create_node("some_file", NodeType::File),
            Err(CreateError::Readonly)
        );
        {
            let iter = fs.list_names();
            let mut iter = iter.into_iter();
            assert!(iter.next().is_none())
        }
        assert_matches!(fs.lookup_node(&arcstr::literal!("hello.txt")), None);
    }

    fn test_simple_fs(fs: &Tree, ro: bool) {
        {
            let iter = fs.list_names();
            let mut iter = iter.into_iter();
            assert_eq!(iter.next().unwrap().as_ref(), "directory");
            assert_eq!(iter.next().unwrap().as_ref(), "ENA.PNG");
            assert_eq!(iter.next().unwrap().as_ref(), "hello.txt");
            assert!(iter.next().is_none())
        }
        assert_matches!(fs.lookup_node(&arcstr::literal!("some_file")), None);

        if ro {
            assert_matches!(
                fs.create_node("some_file", NodeType::File),
                Err(CreateError::Readonly)
            );
        }
        {
            let hello_txt = fs.lookup_node(&arcstr::literal!("HELlO.txt")).unwrap();
            if let NodeRef::File(file) = hello_txt {
                if ro {
                    assert_matches!(
                        file.open(OpenOptions::new().write(true)),
                        Err(OpenError::Readonly)
                    );
                }

                let _file = file.open(OpenOptions::new().read(true)).unwrap();
                // TODO: test reading from this file
            } else {
                panic!("Expected to get a file");
            }
        }
        {
            let ena_png = fs.lookup_node(&arcstr::literal!("Ena.Png")).unwrap();
            if let NodeRef::File(_) = ena_png {
            } else {
                panic!("Expected to get a file");
            }
        }
        {
            let directory = fs.lookup_node(&arcstr::literal!("directory")).unwrap();
            if let NodeRef::FsTree(_) = directory {
            } else {
                panic!("Expected to get a directory");
            }
        }
    }

    #[test]
    fn simple_archive() {
        let arc = get_simple_archive();
        test_simple_fs(&arc, true);
    }

    #[test]
    fn simple_real_fs() {
        let (fs, _holder) = get_simple_fs();
        test_simple_fs(&fs, false);
    }
}
