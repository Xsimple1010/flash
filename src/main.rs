use clap::Parser;
use tokio::task;

use crate::{arg::Args, builder::watch::watch_workspace, server::init_server, state::AppState};

mod arg;
mod builder;
mod server;
mod state;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let state = AppState::default();
    let state_for_thread = state.clone();

    task::spawn(init_server(state.clone()));

    let err = watch_workspace(state_for_thread.clone(), args.path).await;
    println!("{:?}", err);
}
