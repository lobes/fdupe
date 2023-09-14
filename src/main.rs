//todo: only print binary duplicates
//todo: flag for hidden files
//todo: flag for recursive
//todo: help/usage
//todo: update README with usage and examples
//todo: GitHub releases
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hasher;
use std::io::{BufReader, Read};

fn main() -> Result<(), String> {
    let dir_path = env::args()
        .nth(1)
        .ok_or_else(|| "no directory path provided".to_string())?;

    run(&dir_path)?;

    Ok(())
}

struct File {
    path: String,
    size: u64,
    hash: u64,
}

impl File {
    fn hash(&self) -> Result<u64, String> {
        let file = fs::File::open(&self.path).map_err(|e| format!("failed to open file: {}", e))?;
        let mut reader = BufReader::new(file);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .map_err(|e| format!("failed to read file: {}", e))?;

            if bytes_read == 0 {
                break;
            }

            hasher.write(&buffer[..bytes_read]);
        }

        Ok(hasher.finish())
    }
}

fn run(dir_path: &str) -> Result<(), String> {
    let paths = fs::read_dir(dir_path).map_err(|e| format!("failed to read directory: {}", e))?;

    let mut files: Vec<File> = Vec::new();

    for path in paths {
        let entry = path.map_err(|e| format!("failed to get path: {}", e))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("failed to get file type: {}", e))?;

        if file_type.is_file() {
            let metadata = entry
                .metadata()
                .map_err(|e| format!("failed to get metadata: {}", e))?;
            let file_size = metadata.len();

            let abs_path = entry
                .path()
                .canonicalize()
                .map_err(|e| format!("failed to canonicalize path: {}", e))?;

            let file: File = File {
                path: abs_path.display().to_string(),
                size: file_size,
                hash: 0,
            };

            files.push(file);
        }
    }

    let groups = group_by_size(files);

    for group in groups {
        for mut file in group {
            file.hash = file.hash()?;
        }
    }

    Ok(())
}

fn group_by_size(files: Vec<File>) -> Vec<Vec<File>> {
    let mut file_sizes: HashMap<u64, Vec<File>> = HashMap::new();

    for file in files {
        file_sizes.entry(file.size).or_default().push(file);
    }

    let mut groups: Vec<Vec<File>> = Vec::new();

    for (_, files) in file_sizes {
        if files.len() > 1 {
            groups.push(files);
        }
    }

    groups
}
