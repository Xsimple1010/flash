use std::{fs::read_dir, path::Path, process::Command};

use crate::state::{AppState, Executable};

pub fn build_workspace(state: &mut AppState, path: String) {
    let path_clone = path.clone();

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(path_clone)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Build error: {}", stderr);
                return;
            }

            println!("Build completed successfully in directory: {}", &path);
            list_exes(state, &path);
        }
        Err(e) => {
            eprintln!("Error executing 'cargo build': {}", e);
            return;
        }
    }
}

fn list_exes(state: &mut AppState, path: &String) {
    let target_dir = Path::new(&path).join("target").join("debug");

    let entries = match read_dir(&target_dir) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!(
                "Error reading target/debug directory: {}",
                target_dir.display()
            );
            return;
        }
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && is_executable(&path) {
            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let time = entry.metadata().unwrap().modified().unwrap();

            let mut nee_push = true;

            for exe in &mut state.crates {
                if exe.name == file_name {
                    exe.need_update = time > exe.time;
                    exe.time = time;
                    exe.path = path.to_string_lossy().to_string();
                    nee_push = false;
                    break;
                }
            }

            if nee_push {
                state.crates.push(Executable {
                    name: file_name.to_string(),
                    time,
                    need_update: true,
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }
}

// Helper function to check whether a file is executable.
fn is_executable(path: &Path) -> bool {
    // On Windows, check for the .exe extension
    #[cfg(target_os = "windows")]
    {
        path.extension().map(|ext| ext == "exe").unwrap_or(false)
    }
    // On Linux/macOS, check whether the file has execute permissions.
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|meta| meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}
