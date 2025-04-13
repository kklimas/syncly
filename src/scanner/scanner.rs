use crate::scanner::utils::scan_dir;
use crate::scanner::{FileChangeAction, FileEntry, FileEntryChange};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Scanner {
    target: PathBuf,
    source_files: HashMap<String, FileEntry>,
    target_files: HashMap<String, FileEntry>,
}

impl Scanner {
    pub fn new(source: &PathBuf, target: &PathBuf) -> Self {
        let path_buf = target.clone();
        let source_files = scan_dir(source);
        let target_files = scan_dir(target);
        Self {
            target: path_buf,
            source_files,
            target_files,
        }
    }
}

impl Scanner {
    pub fn calculate_diff(&self) -> Vec<FileEntryChange> {
        let mut diff: Vec<FileEntryChange> = Vec::new();

        let source_files = &self.source_files;
        let target_files = &self.target_files;

        source_files
            .iter()
            .for_each(|(key, source_file)| match target_files.get(key) {
                Some(_) => {}
                None => {
                    let source_path = source_file.absolute_path.clone();
                    let target_path = self.target.join(&source_file.relative_path);

                    diff.push(FileEntryChange {
                        source_path,
                        target_path,
                        action: FileChangeAction::Copy,
                    });
                }
            });

        target_files
            .iter()
            .for_each(|(key, target_file)| match source_files.get(key) {
                Some(_) => {}
                None => diff.push(FileEntryChange {
                    source_path: target_file.absolute_path.clone(),
                    target_path: target_file.absolute_path.clone(),
                    action: FileChangeAction::Delete,
                }),
            });

        diff
    }
}

#[cfg(test)]
mod scanner_test {

    /// Suit nr 1
    /// Testing ... 0/1 source file - 0/1 target file
    mod scanner_calc_diff_suit_1 {

        /// Case 1: No source file - No target file
        #[test]
        fn test_scanner_calc_diff_suit_1_case_1() {}

        /// Case 2: One source file - No target file
        #[test]
        fn test_scanner_calc_diff_suit_1_case_2() {}

        /// Case 3: No source file - One target file
        #[test]
        fn test_scanner_calc_diff_suit_1_case_3() {}

        /// Case 4: One source file - One matching target file
        #[test]
        fn test_scanner_calc_diff_suit_1_case_4() {}

        /// Case 5: One source file - One non-matching target file (absolute path)
        #[test]
        fn test_scanner_calc_diff_suit_1_case_5() {}
    }
}
