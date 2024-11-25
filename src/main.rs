use std::env;
use std::fs;
use std::path::Path;

fn replace_umlauts(content: &str) -> String {
    content
        .replace("ä", "&auml;")
        .replace("ö", "&ouml;")
        .replace("ü", "&uuml;")
        .replace("Ä", "&Auml;")
        .replace("Ö", "&Ouml;")
        .replace("Ü", "&Uuml;")
        .replace("ß", "&szlig;")
}

fn process_file(file_path: &Path) {
    if let Ok(content) = fs::read_to_string(file_path) {
        let new_content = replace_umlauts(&content);
        fs::write(file_path, new_content).expect("Unable to write file");
    }
}

fn process_directory(dir_path: &Path) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    process_directory(&path);
                } else if path.extension().and_then(|s| s.to_str()) == Some("html") {
                    process_file(&path);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_or_directory>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    if path.is_dir() {
        process_directory(path);
    } else if path.is_file() {
        process_file(path);
    } else {
        eprintln!("Invalid path: {}", path.display());
    }
}
