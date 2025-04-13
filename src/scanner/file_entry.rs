use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    pub file_name: String,
    pub absolute_path: PathBuf,
    pub relative_path: PathBuf,
    pub hash: String,
    pub size: u64,
}

impl Display for FileEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nFile: {}\nRelative path: {}\nAbsolute path: {}\nHash: {}\nSize: {} bytes",
            self.file_name,
            self.relative_path.display(),
            self.absolute_path.display(),
            self.hash,
            self.size
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum FileChangeAction {
    Copy, Delete
}

#[derive(Debug)]
pub struct FileEntryChange {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub action: FileChangeAction
}
