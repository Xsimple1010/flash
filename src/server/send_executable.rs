use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct PathParams {
    pub name: String,
}

pub async fn send_executable<'a>(
    State(state): State<AppState>,
    Path(params): Path<PathParams>,
) -> Result<Response, (StatusCode, &'a str)> {
    let mut exe_path: Option<String> = None;

    {
        let executables = &*state.crates.lock().await;
        for exe in executables {
            if exe.name.eq(&params.name) {
                exe_path = Some(exe.path.clone());
                break;
            }
        }
    }

    let file_path = match exe_path {
        Some(exe_path) => PathBuf::from(exe_path),
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Não foi possivel enviar o excutavel",
            ));
        }
    };

    // Lê o conteúdo do arquivo
    match fs::read(&file_path).await {
        Ok(content) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", params.name),
                )
                .body(content.into())
                .unwrap();

            Ok(response)
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "Não foi possivel enviar o excutavel")),
    }
}
