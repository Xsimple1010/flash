use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutablePublic {
    pub name: String,
    pub hash: u64,
}

pub async fn get_executables(State(state): State<AppState>) -> Json<Vec<ExecutablePublic>> {
    let executables = &*state.crates.lock().await;

    let mut out = Vec::new();

    for executable in executables {
        out.push(ExecutablePublic {
            name: executable.name.clone(),
            hash: executable.hash,
        });
    }

    Json(out)
}
