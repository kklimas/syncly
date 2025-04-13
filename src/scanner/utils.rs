use crate::scanner::file_entry::FileEntry;
use blake3::Hasher;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_dir(base_path: &Path) -> HashMap<String, FileEntry> {
    let mut files = HashMap::new();

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let absolute_path = entry.path().to_path_buf();
        let relative_path = absolute_path.strip_prefix(base_path).unwrap().to_path_buf();
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let hash = calculate_file_hash(&absolute_path).unwrap();
        let key = format!("{}{}", &relative_path.to_string_lossy(), hash);

        files.insert(
            key,
            FileEntry {
                file_name,
                absolute_path,
                relative_path,
                hash,
                size,
            },
        );
    }

    files
}

fn calculate_file_hash(file_path: &PathBuf) -> Result<String, Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    hasher.update(&buffer);

    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod scanner_utils_test {
    use super::scan_dir;
    use crate::scanner::FileEntry;
    use std::path::PathBuf;

    #[test]
    fn test_scan_flat_dir() {
        // given
        let base_path = PathBuf::from("tests/fixtures/unit/scan_flat_dir");

        // when
        let files_map = scan_dir(&base_path);
        let mut found_files = files_map
            .iter()
            .map(|(_, file_entry)| file_entry)
            .collect::<Vec<&FileEntry>>();

        found_files.sort_by_key(|file| &file.file_name);

        // then
        assert_eq!(3, found_files.len());

        let file1 = &found_files[0];
        assert_eq!(file1.file_name, "file1.txt".to_string());
        assert_eq!(file1.relative_path, PathBuf::from("file1.txt"));
        assert_eq!(
            file1.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_flat_dir/file1.txt")
        );
        assert_eq!(file1.size, 3402);

        let file2 = &found_files[1];
        assert_eq!(file2.file_name, "file2.txt".to_string());
        assert_eq!(file2.relative_path, PathBuf::from("file2.txt"));
        assert_eq!(
            file2.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_flat_dir/file2.txt")
        );
        assert_eq!(file2.size, 136);

        let file3 = &found_files[2];
        assert_eq!(file3.file_name, "file3.json".to_string());
        assert_eq!(file3.relative_path, PathBuf::from("file3.json"));
        assert_eq!(
            file3.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_flat_dir/file3.json")
        );
        assert_eq!(file3.size, 8271);
    }

    #[test]
    fn test_scan_dir() {
        // given
        let base_path = PathBuf::from("tests/fixtures/unit/scan_dir");

        // when
        let files_map = scan_dir(&base_path);
        let mut found_files = files_map
            .iter()
            .map(|(_, file_entry)| file_entry)
            .collect::<Vec<&FileEntry>>();

        found_files.sort_by_key(|file| &file.file_name);

        // then
        assert_eq!(3, found_files.len());

        let file1 = &found_files[0];
        assert_eq!(file1.file_name, "file1.txt".to_string());
        assert_eq!(file1.relative_path, PathBuf::from("r/file1.txt"));
        assert_eq!(
            file1.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_dir/r/file1.txt")
        );
        assert_eq!(file1.size, 3402);

        let file2 = &found_files[1];
        assert_eq!(file2.file_name, "file2.txt".to_string());
        assert_eq!(file2.relative_path, PathBuf::from("q/file2.txt"));
        assert_eq!(
            file2.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_dir/q/file2.txt")
        );
        assert_eq!(file2.size, 136);

        let file3 = &found_files[2];
        assert_eq!(file3.file_name, "file3.json".to_string());
        assert_eq!(file3.relative_path, PathBuf::from("e/file3.json"));
        assert_eq!(
            file3.absolute_path,
            PathBuf::from("tests/fixtures/unit/scan_dir/e/file3.json")
        );
        assert_eq!(file3.size, 8271);
    }
}
