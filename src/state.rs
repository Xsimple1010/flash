use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub crates: Vec<Executable>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Executable {
    pub name: String,
    pub path: String,
    pub hash: u64,
    pub need_update: bool,
}
