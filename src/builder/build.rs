use std::{
    fs::read_dir,
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    process::Command,
};

use tokio::task::{self};

use crate::state::{AppState, Executable};

pub async fn build_workspace(state: AppState, path: String) {
    let path_clone = path.clone();

    println!("Inicializando o build");

    let output = task::spawn_blocking(move || {
        Command::new("cargo")
            .arg("build")
            .current_dir(path_clone)
            .output()
            .expect("Falha ao executar o comando cargo build")
    })
    .await
    .expect("Falha na task de build");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Erro no build: {}", stderr);
        return;
    }

    println!("Build realizado com sucesso no diretório: {}", &path);
    list_exes(state, &path).await;
}

async fn list_exes(state: AppState, path: &String) {
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

    {
        state.crates.lock().await.clear();
    }

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && is_executable(&path) {
            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };

            let mut hash = DefaultHasher::new();
            entry.metadata().unwrap().created().unwrap().hash(&mut hash);

            let executable = Executable {
                name: file_name.to_string(),
                path: path.to_string_lossy().to_string(),
                hash: hash.finish(),
            };

            state.crates.lock().await.push(executable);
        }
    }
}

// Função auxiliar para verificar se um arquivo é executável
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
