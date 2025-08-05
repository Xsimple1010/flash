use std::{
    fs::read_dir,
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    process::Command,
};

use crate::state::{AppState, Executable};

pub async fn build_workspace(state: &mut AppState, path: String) {
    let path_clone = path.clone();

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(path_clone)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Erro no build: {}", stderr);
                return;
            }

            println!("Build realizado com sucesso no diretório: {}", &path);
            list_exes(state, &path).await;
        }
        Err(e) => {
            eprintln!("Erro ao executar o comando cargo build: {}", e);
            return;
        }
    }
}

async fn list_exes(state: &mut AppState, path: &String) {
    let target_dir = Path::new(&path).join("target").join("debug");

    let entries = match read_dir(&target_dir) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!(
                "Erro ao ler o diretório target/debug: {}",
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

            let mut hash = DefaultHasher::new();
            entry
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
                .hash(&mut hash);

            let hash = hash.finish();

            let mut nee_push = true;

            for exe in &mut state.crates {
                if exe.name == file_name {
                    exe.hash = hash;
                    exe.need_update = exe.hash != hash;
                    exe.path = path.to_string_lossy().to_string();
                    nee_push = false;
                    break;
                }
            }

            if nee_push {
                state.crates.push(Executable {
                    name: file_name.to_string(),
                    hash,
                    need_update: true,
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }
}

// Função auxiliar para verificar se um arquivo é executável.
fn is_executable(path: &Path) -> bool {
    // No Windows, verificamos a extensão .exe
    #[cfg(target_os = "windows")]
    {
        path.extension().map(|ext| ext == "exe").unwrap_or(false)
    }
    // No Linux/macOS, verificamos se o arquivo tem permissões de execução
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|meta| meta.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
}
