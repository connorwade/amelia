use std::env;
use std::fs::{read_dir, remove_dir_all, DirEntry};
use std::io;
use std::path::Path;
use std::process;
use std::time::SystemTime;

fn main() {
    let path = Path::new("/home/connor/local-repo");
    if let Err(e) = visit_dirs(path, &callback) {
        println!("Application error: {e}");
        process::exit(1);
    };
}

fn callback(dir: &DirEntry) {
    let node_path = format!("{}{}", dir.path().to_str().unwrap(), "/node_modules");
    let flutter_path = format!("{}{}", dir.path().to_str().unwrap(), "/build");

    let mut binding = process::Command::new("flutter");
    let flutter_clean = binding.arg("clean");

    if let Ok(metadata) = dir.metadata() {
        if let Ok(last_access) = metadata.accessed() {
            match SystemTime::now().duration_since(last_access) {
                Ok(since) => {
                    let hours_since = since.as_secs() / 60 / 60;

                    if hours_since >= 2400 {
                        if Path::new(&node_path).is_dir() {
                            println!("Cleaning project {:?}...", dir.path());
                            remove_dir_all(node_path).expect("Failed to clean node project");
                        } else if Path::new(&flutter_path).is_dir() {
                            println!("Cleaning project {:?}...", dir.path());
                            flutter_clean.status().expect("Flutter Clean failed to run");
                        }
                    }
                }
                Err(_) => panic!("You did something wrong you ape"),
            }
        }
    }
}

// lib code

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // if path.is_dir() {
            if false {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)
            }
        }
    }
    Ok(())
}
