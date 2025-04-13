use std::env;
use std::path::PathBuf;

pub struct Env {
    pub source: PathBuf,
    pub target: PathBuf,
    pub verbose: bool,
}

impl Env {
    pub fn new() -> Self {
        let source = PathBuf::from(extract("SOURCE_DIR".to_string()));
        let target = PathBuf::from(extract("TARGET_DIR".to_string()));
        let verbose = env::var("VERBOSE")
            .map(|s| s.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);

        Env {
            source,
            target,
            verbose,
        }
    }
}

fn extract(key: String) -> String {
    match env::var(&key) {
        Ok(val) => val,
        Err(_) => panic!("Missing environment variable {}", &key),
    }
}
