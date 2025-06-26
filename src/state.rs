use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub crates: Arc<Mutex<Vec<Executable>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Executable {
    pub name: String,
    pub path: String,
    pub hash: u64,
}
