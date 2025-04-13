use crate::scanner::{FileChangeAction, FileEntryChange, Scanner};
use std::fs::{copy, create_dir_all, read_dir, remove_dir, remove_file};
use std::path::{PathBuf};
use tracing::{error, info};
use walkdir::{WalkDir};

pub struct SyncExecutor<'a> {
    target: &'a PathBuf,
    scanner: Scanner,
}

impl<'a> SyncExecutor<'a> {
    pub fn new(source: &PathBuf, target: &'a PathBuf) -> Self {
        Self {
            target,
            scanner: Scanner::new(source, target),
        }
    }

    pub fn execute(&self) {
        let changes = self.scanner.calculate_diff();

        info!("Found {} actions to execute", changes.len());

        changes
            .iter()
            .filter(|change| change.action == FileChangeAction::Delete)
            .for_each(|change| Self::delete_file(&change.target_path));

        changes
            .iter()
            .filter(|change| change.action == FileChangeAction::Copy)
            .for_each(|change| self.copy_file(change));

        self.clean_up();
    }

    fn copy_file(&self, change: &FileEntryChange) {
        let parent_dir = change.target_path.parent().unwrap();

        if !parent_dir.exists() {
            match create_dir_all(parent_dir) {
                Ok(_) => info!("Created parent dir {:?}", parent_dir),
                Err(e) => error!("Failed to create parent dir {:?}: {}", parent_dir, e),
            };
        }

        match copy(&change.source_path, &change.target_path) {
            Ok(_) => info!(
                "File copied from {:?} to {:?}",
                &change.source_path, &change.target_path
            ),
            Err(msg) => error!("{}", msg),
        }
    }
    fn delete_file(path: &PathBuf) {
        match remove_file(path) {
            Ok(_) => info!("Deleted file {:?}", path),
            Err(msg) => error!("{}", msg),
        }
    }

    fn clean_up(&self) {
        info!("Cleaning up empty directories...");

        let root_dir = self.target;
        for entry in WalkDir::new(root_dir)
            .min_depth(1)
            .max_depth(usize::MAX)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            let path = entry.path();

            if read_dir(path)
                .map(|mut i| i.next().is_none())
                .unwrap_or(false)
            {
                match remove_dir(path) {
                    Ok(_) => info!("Directory {} removed.", path.display()),
                    Err(msg) => error!("{}", msg),
                };
            }
        }
    }
}
