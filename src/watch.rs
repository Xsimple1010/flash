use crate::config::FlashConfig;
use crate::{build::build_workspace, observers_handle::try_send_to_observer, state::AppState};
use notify::{Error, Event, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::{
    path::Path,
    sync::mpsc,
    time::{Duration, Instant},
};

pub fn watch_workspace(mut state: AppState, path: String) -> Result<(), Error> {
    let config = FlashConfig::new(PathBuf::from(&path)).unwrap();

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;

    let mut last_processed = Instant::now();
    let debounce_duration = Duration::from_secs(2);

    if state.crates.len().eq(&0) {
        build_workspace(&mut state, path.clone());
        let _ = try_send_to_observer(&state, &config.observers);
    }

    while let Ok(res) = rx.recv() {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Modify(_) => {
                    if is_not_allow_dir(event, &config) {
                        continue;
                    }

                    let now = Instant::now();
                    if now.duration_since(last_processed) > debounce_duration {
                        build_workspace(&mut state, path.clone());
                        let _ = try_send_to_observer(&state, &config.observers);
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

fn is_not_allow_dir(event: Event, config: &FlashConfig) -> bool {
    event.paths.iter().any(|p| {
        for dir in &config.dir {
            return p.to_string_lossy().contains(dir);
        }

        return false;
    })
}
