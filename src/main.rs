//todo: add flag for hidden files
//todo: add flag for recursive
//todo: add help/usage
use std::env;
use std::fs;

fn run(dir_path: &str) -> Result<(), String> {
    let paths = fs::read_dir(dir_path).map_err(|e| format!("failed to read directory: {}", e))?;

    for path in paths {
        let entry = path.map_err(|e| format!("failed to get path: {}", e))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("failed to get file type: {}", e))?;

        if file_type.is_file() {
            let abs_path = entry
                .path()
                .canonicalize()
                .map_err(|e| format!("failed to canonicalize path: {}", e))?;
            println!("{}", abs_path.display());
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let dir_path = env::args()
        .nth(1)
        .ok_or_else(|| "no directory path provided".to_string())?;

    run(&dir_path)?;

    Ok(())
}
