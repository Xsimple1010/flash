use notify::{Error, Event, RecursiveMode, Watcher};
use std::{
    path::Path,
    sync::mpsc,
    time::{Duration, Instant},
};

use crate::{build::build_workspace, observers_handle::try_send_to_observer, state::AppState};

pub async fn watch_workspace(mut state: AppState, path: String) -> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;

    let mut last_processed = Instant::now();
    let debounce_duration = Duration::from_secs(2);

    if state.crates.len().eq(&0) {
        build_workspace(&mut state, path.clone()).await;
        let _ = try_send_to_observer(&state, path.clone()).await;
    }

    while let Ok(res) = rx.recv() {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Modify(_) => {
                    if is_target_dir(event) {
                        continue;
                    }

                    let now = Instant::now();
                    if now.duration_since(last_processed) > debounce_duration {
                        build_workspace(&mut state, path.clone()).await;
                        let _ = try_send_to_observer(&state, path.clone()).await;
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

fn is_target_dir(event: Event) -> bool {
    event
        .paths
        .iter()
        .any(|p| p.to_string_lossy().contains("/target/"))
}
