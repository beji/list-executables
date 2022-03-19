use rayon::prelude::*;
use std::{
    env, fs,
    io::{BufWriter, Write},
    os::unix::prelude::PermissionsExt,
    path::Path,
    process::exit,
};

fn visit_path_entry(path_entry: &str) -> Vec<String> {
    let path = Path::new(path_entry);

    let empty: Vec<String> = Vec::new();
    eprintln!("Checking {}", path_entry);
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                let collected: Vec<_> = entries.collect();

                let result: Vec<String> = collected
                    .par_iter()
                    .filter_map(|dir_entry| match dir_entry {
                        Ok(entry) => {
                            if entry.path().is_file() {
                                match fs::metadata(entry.path()) {
                                    Ok(metadata) => {
                                        if metadata.permissions().mode() & 0o111 != 0 {
                                            match entry.path().to_str() {
                                                Some(s) => Some(s.to_string()),
                                                None => None,
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        None
                                    }
                                }
                            } else {
                                None
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            None
                        }
                    })
                    .map(|entry| entry.to_string())
                    .collect();
                result
            }
            Err(_e) => {
                eprintln!("Failed to read from {}", path_entry);
                empty
            }
        }
    } else {
        eprintln!("{} is not a directory, skipping...", path_entry);
        empty
    }
}

fn main() {
    eprintln!("Reading $PATH");
    match env::var("PATH") {
        Ok(path) => {
            eprintln!("got {}", path);

            let split: Vec<&str> = path.split(":").collect();
            let collected: Vec<_> = split
                .par_iter()
                .map(|path_entry| visit_path_entry(path_entry))
                .collect();

            let mut executables: Vec<String> = Vec::new();

            for collection in collected {
                for executable in collection {
                    executables.push(executable);
                }
            }

            let mut binaries: Vec<&str> = executables
                .par_iter()
                .filter_map(|executable| executable.split("/").last())
                .collect();
            binaries.par_sort();

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
