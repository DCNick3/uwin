use arcstr::ArcStr;
use core_fs::path::{AbsolutePath, DirectoryPath, Drive, Root, WindowsPath};
use core_fs::{Access, NodeRef, NodeType, OpenOptions, Tree};
use core_handletable::{Handle, HandleTable};
use glob::MatchOptions;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::warn;
use win32::Win32::Foundation::{FILETIME, HANDLE};
use win32::Win32::Storage::FileSystem::FindFileHandle;
use win32_kobj::{KernelHandleTable, KernelObject};

pub struct WindowsFsManager {
    current_directory: Mutex<AbsolutePath>,
    handle_table: Arc<Mutex<KernelHandleTable>>,
    roots: HashMap<Root, Tree>,
    search_handle_table: Mutex<HandleTable<(), FindFileHandle_, Search, true>>,
}

#[derive(Copy, Clone, Debug)]
struct FindFileHandle_(u32);

impl Handle for FindFileHandle_ {
    fn to_index(self, _: &()) -> usize {
        let handle: usize = self.0.try_into().unwrap();
        (handle >> 2) - 10
    }

    fn from_index(index: usize, _: &()) -> Self {
        let index = (index + 10) << 2;
        Self(index.try_into().unwrap())
    }
}

impl Into<FindFileHandle> for FindFileHandle_ {
    fn into(self) -> FindFileHandle {
        FindFileHandle(self.0 as _)
    }
}

impl Into<FindFileHandle_> for FindFileHandle {
    fn into(self) -> FindFileHandle_ {
        FindFileHandle_(self.0 as _)
    }
}

struct Search {
    directory_tree: Tree,
    names: std::vec::IntoIter<ArcStr>,
}

pub struct FindData {
    pub kind: NodeType,
    pub name: ArcStr,
    pub size: u64,
    pub create_time: FILETIME,
    pub access_time: FILETIME,
    pub write_time: FILETIME,
}

// TODO: ignored for now, probably can implement later (is it needed in a single process context though?)
// pub enum ShareMode {}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CreationDisposition {
    CreateNew,    // only new
    OpenExisting, // only existing
    OpenAlways,   // whatever
}

impl WindowsFsManager {
    pub fn new(handle_table: Arc<Mutex<KernelHandleTable>>, roots: HashMap<Root, Tree>) -> Self {
        Self {
            handle_table,
            current_directory: Mutex::new(AbsolutePath {
                root: Root::Drive(Drive::C),
                path: DirectoryPath::parse("", false).unwrap(),
            }),
            roots,
            search_handle_table: Mutex::new(HandleTable::new((), 4)),
        }
    }

    /// Returns a bool indicating whether a new file was created and a HANDLE to the file created/opened
    pub fn create_file(
        &self,
        path: &str,
        access: Access,
        creation_disposition: CreationDisposition,
        truncate: bool,
    ) -> (bool, HANDLE) {
        use crate::CreationDisposition::*;

        let AbsolutePath { root, path } = {
            let current_directory = self.current_directory.lock().unwrap();

            let path = WindowsPath::parse(path).expect("Invalid path passed to create_file");

            path.resolve(&*current_directory)
        };

        let root = self.roots.get(&root).expect("Root tree not found").clone();

        let mut current_tree = root;

        // TODO: use Tree::traverse_to_node to replace part of this logic
        let mut iter = path.iter().peekable();
        let (was_file_created, handle) = loop {
            let part = iter.next().unwrap(); // we always break on last iteration
            let is_filename_part = iter.peek().is_none();
            if is_filename_part {
                let node = current_tree.lookup_node(part);
                let file_found = node.is_none();

                let file = if let Some(node) = node {
                    if creation_disposition == CreateNew {
                        panic!("creation_disposition was CreateNew, but file already existed")
                    }
                    node.to_file()
                        .ok()
                        .expect("was trying to open a file, but path pointed to directory instead")
                } else {
                    if creation_disposition == OpenExisting {
                        panic!("creation_disposition was OpenExisting, but file did not exist")
                    }
                    current_tree
                        .create_file(part)
                        .expect("File creation failed")
                };

                break (
                    !file_found,
                    file.open(OpenOptions::new(access).truncate(truncate))
                        .expect("Opening file failed"),
                );
            } else {
                current_tree = current_tree
                    .lookup_node(part)
                    .expect("Can't find part of a path")
                    .to_tree()
                    .ok()
                    .expect("Encountered a file, but expected directory while traversing a path");
            }
        };

        let mut handle_table = self.handle_table.lock().unwrap();

        (
            was_file_created,
            handle_table.put(Arc::new(KernelObject::File(Mutex::new(handle)))),
        )
    }

    pub fn get_current_directory(&self) -> String {
        let current_directory = self.current_directory.lock().unwrap();

        current_directory.to_string()
    }

    pub fn create_search(&self, search_mask: &str) -> FindFileHandle {
        assert!(
            !search_mask.is_empty(),
            "Can't use an empty string as search mask"
        );
        assert_ne!(
            search_mask.chars().last().unwrap(),
            '/',
            "Can't end search mask with a slash"
        );

        // we need to split the mask into directory part and filename part
        // the directory part doesn't have mask applied to it and is used to get a list of files
        // filename part is used as a mask

        let (directory, mask) = {
            let current_directory = self.current_directory.lock().unwrap();

            if let Some((directory, filename)) = search_mask.rsplit_once('/') {
                let directory =
                    WindowsPath::parse(directory).expect("Invalid path passed to seach");

                let directory = directory.resolve(&current_directory);

                (directory, filename)
            } else {
                // we don't have slashes, so it's just a bare filename
                // lookup in a current directory then!.clone();

                (current_directory.clone(), search_mask)
            }
        };

        // to simplify implementation we just do all the search beforehand (filtering the filenames)
        // the attributes would still be received in search_next

        let tree = self
            .roots
            .get(&directory.root)
            .expect("Unknown root passed to search");
        let directory_tree = tree
            .traverse_to_node(directory.path.parts.iter())
            .expect("Could not find directory specified in search")
            .to_tree()
            .ok()
            .expect("Found a file while expected a directory to perform search on");

        assert!(!mask.contains(&['[', ']']) && !mask.contains("**"), "Mask contains unsupported characters (TODO: make a patched glob library that ignores unix-specific things)");

        let glob = glob::Pattern::new(mask).expect("Can't parse the mask as a glob");
        let match_options = MatchOptions {
            case_sensitive: false,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        };

        let mut names = directory_tree.list_names();
        names.retain(|name| glob.matches_with(name, match_options));

        let names = names.into_iter();

        let mut search_handle_table = self.search_handle_table.lock().unwrap();

        let handle = search_handle_table.put(Search {
            directory_tree,
            names,
        });

        handle.into()
    }

    pub fn search_next(&self, handle: FindFileHandle) -> Option<FindData> {
        let mut search_handle_table = self.search_handle_table.lock().unwrap();

        let search = search_handle_table
            .find_mut(handle.into())
            .expect("search_next called on an unknown handle");

        loop {
            let name = search.names.next()?;
            let node = search.directory_tree.lookup_node(&name);

            let node = if let Some(node) = node {
                node
            } else {
                warn!("A file {} disappeared while we were doing the search! Please don't touch the fs while uwin is running, it's fully of TOC-TOU races. Will ignore this one", name);
                continue;
            };

            let kind = node.node_type();

            // stub time (because not implemented yet, lol)
            let file_time = FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            };

            let size = match node {
                NodeRef::Tree(_) => 0,
                NodeRef::File(file) => file.get_size(),
            };

            break Some(FindData {
                kind,
                name,
                size,
                create_time: file_time,
                access_time: file_time,
                write_time: file_time,
            });
        }
    }

    pub fn search_close(&self, handle: FindFileHandle) {
        let mut search_handle_table = self.search_handle_table.lock().unwrap();
        assert!(
            search_handle_table.remove(handle.into()).is_some(),
            "search_closed called on an unknown handle"
        );
    }
}
