use clap::Parser;
use crate::{arg::Args, state::AppState, watch::watch_workspace};

mod arg;
mod build;
mod observers_handle;
mod send_executable;
mod state;
mod watch;
mod config;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("{:?}", args);

    let state = AppState::default();
    let state_for_thread = state.clone();

    let err = watch_workspace(state_for_thread.clone(), args.path).await;
    println!("{:?}", err);
}
