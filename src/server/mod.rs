use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::state::AppState;
use tower_http::cors::CorsLayer;
mod get_executables;
mod send_executable;

use get_executables::get_executables;
use send_executable::send_executable;

pub async fn init_server(state: AppState) {
    let app = Router::new()
        .route("/executables", get(get_executables))
        .route("/executable/{name}", get(send_executable))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:4090";

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Flash listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
