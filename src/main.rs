//todo: add flag for hidden files
//todo: add flag for recursive
//todo: add help/usage
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let dir_path = env::args()
        .nth(1)
        .ok_or_else(|| "no directory path provided".to_string())?;

    run(&dir_path)?;

    Ok(())
}
fn run(dir_path: &str) -> Result<(), String> {
    let paths = fs::read_dir(dir_path).map_err(|e| format!("failed to read directory: {}", e))?;

    let mut file_sizes: HashMap<u64, Vec<String>> = HashMap::new();

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

            file_sizes
                .entry(file_size)
                .or_default()
                .push(abs_path.display().to_string());
        }
    }

    let mut file_sizes: Vec<(u64, Vec<String>)> = file_sizes.into_iter().collect();
    file_sizes.sort_by_key(|(_, paths)| paths.len());
    file_sizes.reverse();

    for (size, paths) in file_sizes {
        if paths.len() > 1 {
            println!("{} bytes:", size);
            println!("{}", paths.join("\n"));
            println!();
        }
    }

    Ok(())
}
