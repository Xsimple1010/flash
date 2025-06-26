use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::state::AppState;

mod get_executables;
mod send_executable;

use get_executables::get_executables;

pub async fn init_server(state: AppState) {
    let app = Router::new()
        .route("/executables", get(get_executables))
        .with_state(state);

    let addr = "0.0.0.0:4090";

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Flash listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
