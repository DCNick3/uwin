pub mod error;
mod path;

use crate::error::{CreateError, OpenError, ReadError, RemoveError, SeekError, WriteError};
use anyhow::Context;
use arcstr::ArcStr;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::io;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;
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
    fn lookup_node_name(&self, needle: &ArcStr) -> Option<ArcStr> {
        match self {
            Tree::ArchiveDir(dir) => dir
                .get_key_value(&UniCase::new(needle.clone()))
                .map(|(k, _)| k.clone().into_inner()),
            Tree::FsDir(path) => {
                let readdir = std::fs::read_dir(path.as_str())
                    .with_context(|| format!("Opening directory {:?}", path))
                    .unwrap();

                // ASSUMPTION: no two files have the same name (ignoring case)
                // if they do, we will return the first matching name
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
                    let name = ArcStr::from(name);

                    let name = UniCase::new(name);
                    if name == UniCase::new(needle) {
                        return Some(name.into_inner());
                    }
                }

                None
            }
        }
    }

    /// Tries to lookup an immediate child node with specified name
    ///
    pub fn lookup_node(&self, name: &ArcStr) -> Option<NodeRef> {
        match self {
            Tree::ArchiveDir(dir) => dir.get(&UniCase::new(name.clone())).cloned(),
            Tree::FsDir(path) => {
                let name = self.lookup_node_name(name)?;

                let path = ArcStr::from(
                    Path::new(path.as_str())
                        .join(name.as_str())
                        .to_str()
                        .unwrap(),
                );

                let file_type = std::fs::metadata(path.as_str())
                    .with_context(|| format!("Reading metadata of {:?}", path))
                    .unwrap()
                    .file_type();

                if file_type.is_file() {
                    Some(NodeRef::File(FileRef::FsFile(path)))
                } else if file_type.is_dir() {
                    Some(NodeRef::Tree(Tree::FsDir(path)))
                } else {
                    unimplemented!("Unsupported file_type: {:?}", file_type)
                }
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
    pub fn create_node(&self, name: &str, node_type: NodeType) -> Result<NodeRef, CreateError> {
        match self {
            Tree::ArchiveDir(_) => Err(CreateError::Readonly),
            Tree::FsDir(path) => {
                assert!(!name.contains('/'));
                assert!(!name.contains('\\'));

                // check if another entry with the same name exists (use windows rules, ignore the case)
                // this is actually not really a strict check, as TOC and TOU for this are different
                // we may end up with several files with the same name (according to windows)
                // ignore for now =)
                let unicase_name = UniCase::new(name);
                let exists = self
                    .list_names()
                    .into_iter()
                    .any(|n| UniCase::new(n) == unicase_name);

                if exists {
                    return Err(CreateError::Exists);
                }

                let path = ArcStr::from(Path::new(path.as_str()).join(name).to_str().unwrap());
                let res = match node_type {
                    NodeType::File => std::fs::File::options()
                        .create_new(true)
                        .write(true)
                        .open(path.as_str())
                        .map(|_| NodeRef::File(FileRef::FsFile(path.clone()))),
                    NodeType::Directory => std::fs::create_dir(path.as_str())
                        .map(|_| NodeRef::Tree(Tree::FsDir(path.clone()))),
                }
                .map_err(|e| match e.kind() {
                    ErrorKind::AlreadyExists => CreateError::Exists,
                    _ => {
                        panic!(
                            "{:?}",
                            anyhow::Error::new(e)
                                .context(format!("Creating fs node at {:?}", path))
                        );
                    }
                });

                res
            }
        }
    }
    pub fn create_file(&self, name: &str) -> Result<FileRef, CreateError> {
        Ok(self.create_node(name, NodeType::File)?.to_file().unwrap())
    }
    pub fn create_directory(&self, name: &str) -> Result<Tree, CreateError> {
        Ok(self
            .create_node(name, NodeType::Directory)?
            .to_tree()
            .unwrap())
    }

    pub fn remove_node(&self, name: &str, node_type: NodeType) -> Result<(), RemoveError> {
        match self {
            Tree::ArchiveDir(_) => Err(RemoveError::Readonly),
            Tree::FsDir(path) => {
                let name = ArcStr::from(name);
                // lookup real name (with the correct casing)
                let name = self
                    .lookup_node_name(&name)
                    .ok_or(RemoveError::DoesNotExist)?;

                let path = Path::new(path.as_str()).join(name.as_str());
                let metadata = match std::fs::metadata(&path) {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        if e.kind() == ErrorKind::NotFound {
                            return Err(RemoveError::DoesNotExist);
                        }
                        panic!("Failed to read metadata of {:?}: {:?}", path, e);
                    }
                };

                match node_type {
                    NodeType::File => {
                        // !! TOU/TOC races
                        // it is better to check the return code, but this can't be easily done without io_error_more being stabilized
                        if !metadata.is_file() {
                            return Err(RemoveError::NotAFile);
                        }
                        std::fs::remove_file(&path)
                    }
                    NodeType::Directory => {
                        if !metadata.is_dir() {
                            return Err(RemoveError::NotADirectory);
                        }
                        let mut dir = std::fs::read_dir(&path)
                            .with_context(|| {
                                format!("Can't read directory {:?} to check if it's empty", path)
                            })
                            .unwrap();
                        if !dir.next().is_none() {
                            return Err(RemoveError::DirectoryNotEmpty);
                        }
                        std::fs::remove_dir(&path)
                    }
                }
                .map_err(|e| match e.kind() {
                    // can't use those because they are unstable
                    // ErrorKind::DirectoryNotEmpty => {}
                    // ErrorKind::IsADirectory => {}
                    e => panic!("Can't remove the entry {:?}: {:?}", path, e),
                })
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
    StaticFileHandle { pos: u64, file: &'static StaticFile },
    FsFileHandle(std::fs::File),
}

impl FileHandle {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadError> {
        match self {
            FileHandle::StaticFileHandle { pos, file } => {
                let start_pos: usize = (*pos).try_into().unwrap();
                let size = buf.len();
                let size = std::cmp::min(size, file.contents.len().saturating_sub(start_pos));
                let src_buffer = &file.contents[start_pos..start_pos + size];

                buf[..size].copy_from_slice(src_buffer);

                *pos += (size.try_into() as Result<u64, _>).unwrap();

                Ok(size)
            }
            FileHandle::FsFileHandle(file) => file
                .read(buf)
                .map_err(|_| todo!("Mapping native fs read error to a ReadError")),
        }
    }
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, WriteError> {
        match self {
            FileHandle::StaticFileHandle { .. } => Err(WriteError::NoAccess),
            FileHandle::FsFileHandle(file) => file
                .write(buf)
                .map_err(|_| todo!("Mapping native fs read error to a ReadError")),
        }
    }
    pub fn seek(&mut self, from: SeekFrom) -> Result<u64, SeekError> {
        match self {
            FileHandle::StaticFileHandle { pos, file } => {
                let new_pos = match from {
                    SeekFrom::Start(offset) => offset.try_into().unwrap(),
                    SeekFrom::End(offset) => (file.contents.len() as i64) + offset,
                    SeekFrom::Current(offset) => {
                        ((*pos).try_into() as Result<i64, _>).unwrap() + offset
                    }
                };

                if new_pos < 0 {
                    return Err(SeekError::NegativePosition);
                }

                *pos = new_pos.try_into().unwrap();

                Ok(*pos)
            }
            FileHandle::FsFileHandle(file) => file
                .seek(from)
                .map_err(|_| todo!("Mapping native fs seek error to a SeekError")),
        }
    }
}

impl Read for FileHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
            .map_err(|e| io::Error::new(e.clone().into(), e))
    }
}

impl Write for FileHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
            .map_err(|e| io::Error::new(e.clone().into(), e))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for FileHandle {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.seek(pos)
            .map_err(|e| io::Error::new(e.clone().into(), e))
    }
}

#[derive(Clone, Debug)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub enum NodeRef {
    Tree(Tree),
    File(FileRef),
}

#[derive(Clone, Debug)]
pub struct Attributes {}

impl NodeRef {
    pub fn attrs(&self) -> Attributes {
        todo!()
    }
    pub fn to_tree(self) -> Result<Tree, Self> {
        match self {
            NodeRef::Tree(tree) => Ok(tree),
            NodeRef::File(file) => Err(NodeRef::File(file)),
        }
    }
    pub fn to_file(self) -> Result<FileRef, Self> {
        match self {
            NodeRef::File(file) => Ok(file),
            NodeRef::Tree(tree) => Err(NodeRef::Tree(tree)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Attributes, CreateError, FileRef, NodeRef, NodeType, OpenError, OpenOptions, RemoveError,
        StaticFile, Tree,
    };
    use arcstr::ArcStr;
    use cool_asserts::assert_matches;
    use std::collections::BTreeMap;
    use std::io::{Read, SeekFrom};
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
                NodeRef::Tree(Tree::ArchiveDir(Arc::new(BTreeMap::from([(
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

                let mut file = file.open(OpenOptions::new().read(true)).unwrap();

                // test reading
                {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();

                    assert_eq!(contents, "Hewwo wowld, this is some static file contents");
                }
                // test reading and seeking
                {
                    file.seek(SeekFrom::Start(13)).unwrap();

                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();

                    assert_eq!(contents, "this is some static file contents");
                }
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
            if let NodeRef::Tree(_) = directory {
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

    #[test]
    fn create_stuff() {
        let dir = TempDir::new().unwrap();
        let fs = Tree::FsDir(ArcStr::from(dir.path().to_str().unwrap()));

        let file = fs.create_node("test_file", NodeType::File).unwrap();
        assert_matches!(file, NodeRef::File(FileRef::FsFile(_)));
        // check that creating errors out
        assert_matches!(
            fs.create_node("test_file", NodeType::File),
            Err(CreateError::Exists)
        );
        assert_matches!(
            fs.create_node("test_file", NodeType::Directory),
            Err(CreateError::Exists)
        );
        // check that casing does not matter
        assert_matches!(
            fs.create_node("TesT_FilE", NodeType::File),
            Err(CreateError::Exists)
        );
        assert_matches!(
            fs.create_node("TesT_FilE", NodeType::Directory),
            Err(CreateError::Exists)
        );

        let file = fs
            .create_node("test_Directory", NodeType::Directory)
            .unwrap();
        assert_matches!(file, NodeRef::Tree(Tree::FsDir(_)));
        // check that creating errors out
        assert_matches!(
            fs.create_node("test_Directory", NodeType::File),
            Err(CreateError::Exists)
        );
        assert_matches!(
            fs.create_node("test_Directory", NodeType::Directory),
            Err(CreateError::Exists)
        );
        // check that casing does not matter
        assert_matches!(
            fs.create_node("TesT_Directory", NodeType::File),
            Err(CreateError::Exists)
        );
        assert_matches!(
            fs.create_node("TesT_Directory", NodeType::Directory),
            Err(CreateError::Exists)
        );

        let mut iter = fs.list_names().into_iter();
        assert_eq!(iter.next().unwrap().as_ref(), "test_Directory");
        assert_eq!(iter.next().unwrap().as_ref(), "test_file");
        assert!(iter.next().is_none());
    }

    #[test]
    fn remove_stuff() {
        let dir = TempDir::new().unwrap();
        let fs = Tree::FsDir(ArcStr::from(dir.path().to_str().unwrap()));

        // create file
        fs.create_node("test_file", NodeType::File).unwrap();

        // can't remove it as a directory
        assert_matches!(
            fs.remove_node("test_file", NodeType::Directory),
            Err(RemoveError::NotADirectory)
        );

        // can remove it as a file
        fs.remove_node("test_file", NodeType::File).unwrap();

        // nothing is left
        assert!(fs.list_names().into_iter().next().is_none());

        // =====

        // create a directory
        fs.create_node("Test_Directory", NodeType::Directory)
            .unwrap();

        // can't remove it a sa file
        assert_matches!(
            fs.remove_node("test_directory", NodeType::File),
            Err(RemoveError::NotAFile)
        );

        // can remove it as a directory
        fs.remove_node("test_directory", NodeType::Directory)
            .unwrap();

        // nothing is left behind
        assert!(fs.list_names().into_iter().next().is_none());

        // ====

        // create a directory with a file inside
        let dir = fs.create_directory("test_directory").unwrap();
        dir.create_file("test_file_in_directory").unwrap();

        // can't remove it cuz there is a file in it
        assert_matches!(
            fs.remove_node("test_directory", NodeType::Directory),
            Err(RemoveError::DirectoryNotEmpty)
        );

        // remove the file and the directory itself
        dir.remove_node("test_file_in_directory", NodeType::File)
            .unwrap();
        fs.remove_node("test_directory", NodeType::Directory)
            .unwrap();
    }
}
