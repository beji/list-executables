use std::{
    env, fs,
    io::{BufWriter, Write},
    os::unix::prelude::PermissionsExt,
    path::Path,
    process::exit,
};

fn visit_path_entry(path_entry: &str) -> Vec<String> {
    let path = Path::new(path_entry);

    let mut result: Vec<String> = Vec::new();
    eprintln!("Checking {}", path_entry);
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for dir_entry in entries {
                    match dir_entry {
                        Ok(entry) => {
                            if entry.path().is_file() {
                                match fs::metadata(entry.path()) {
                                    Ok(metadata) => {
                                        if metadata.permissions().mode() & 0o111 != 0 {
                                            match entry.path().to_str() {
                                                Some(x) => result.push(x.to_string()),
                                                None => (),
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("{}", e),
                                }
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
            Err(_e) => eprintln!("Failed to read from {}", path_entry),
        }
    } else {
        eprintln!("{} is not a directory, skipping...", path_entry);
    }
    result
}

fn main() {
    eprintln!("Reading $PATH");
    match env::var("PATH") {
        Ok(path) => {
            eprintln!("got {}", path);

            let split = path.split(":");
            let collected = split.map(|path_entry| visit_path_entry(path_entry));

            let mut executables: Vec<String> = Vec::new();

            for collection in collected {
                for executable in collection {
                    executables.push(executable);
                }
            }

            let mut binaries: Vec<&str> = executables
                .iter()
                .filter_map(|executable| executable.split("/").last())
                .collect();
            binaries.sort();

            let stdout = std::io::stdout();
            let lock = stdout.lock();
            let mut buf = BufWriter::new(lock);

            for binary in binaries {
                writeln!(buf, "{}", binary).unwrap();
            }
        }
        Err(_e) => {
            eprintln!("Failed to read $PATH");
            exit(1);
        }
    }
}
