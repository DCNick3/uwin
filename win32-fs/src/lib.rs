use core_fs::path::{AbsolutePath, DirectoryPath, Drive, Root, WindowsPath};
use core_fs::{Access, OpenOptions, Tree};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use win32::Win32::Foundation::HANDLE;
use win32_kobj::{KernelHandleTable, KernelObject};

pub struct WindowsFsManager {
    current_directory: Mutex<AbsolutePath>,
    handle_table: Arc<Mutex<KernelHandleTable>>,
    roots: HashMap<Root, Tree>,
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

        // TODO: maybe shift this tree traversal logic to core_fs?
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
}
