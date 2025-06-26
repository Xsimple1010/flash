use std::{path::Path, time::Duration};

use notify::{Error, Event, RecursiveMode, Watcher};
use tokio::{
    sync::mpsc,
    task::{self, JoinHandle},
    time::Instant,
};

use crate::{builder::build::build_workspace, state::AppState};

pub async fn watch_workspace(state: AppState, path: String) -> Result<(), Error> {
    let (tx, mut rx) = mpsc::channel::<notify::Result<Event>>(100);

    let mut watcher = notify::recommended_watcher(move |res| {
        let tx = tx.clone();
        let _ = tx.blocking_send(res); // Envia eventos de forma síncrona para o canal Tokio
    })?;

    watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;

    let mut last_processed = Instant::now();
    let debounce_duration = Duration::from_secs(2);
    let mut _join: Option<JoinHandle<()>> = None;

    if state.crates.lock().await.len().eq(&0) {
        // Executa o build em uma task separada
        let state_clone = state.clone();
        let path_clone = path.clone();

        let _ = task::spawn(async move {
            build_workspace(state_clone, path_clone).await;
        })
        .await;
    }

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Modify(_) => {
                    if event
                        .paths
                        .iter()
                        .any(|p| p.to_string_lossy().contains("/target/"))
                    {
                        continue;
                    }

                    let now = Instant::now();
                    if now.duration_since(last_processed) > debounce_duration {
                        // Executa o build em uma task separada
                        let state_clone = state.clone();
                        let path_clone = path.clone();

                        _join = Some(task::spawn(async move {
                            build_workspace(state_clone, path_clone).await;
                        }));

                        last_processed = now;
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
