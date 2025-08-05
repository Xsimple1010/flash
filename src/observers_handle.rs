use std::{ops::Not};

use crate::{
    send_executable::{ExeType, send_executable},
    state::{AppState, Executable},
};
use crate::config::Observer;

fn get_executable(exes: &Vec<Executable>, name: &str) -> Option<Executable> {
    for exe in exes {
        if exe.name == name {
            return Some(exe.clone());
        }
    }

    None
}

pub async fn try_send_to_observer(
    state: &AppState,
    observers: &Vec<Observer>,
) -> Result<(), Box<dyn std::error::Error>> {

    for observer in observers {
        let exe = get_executable(&state.crates, &observer.executable);
        let mut deps = Vec::new();

        let mut need_top = false;

        for dep in &observer.deps {
            if let Some(dep_exe) = get_executable(&state.crates, &dep) {
                if dep_exe.need_update {
                    println!("Dependency {} needs update", dep_exe.name);
                    deps.push(dep_exe);
                }
            } else {
                eprintln!("Dependency {} not found in state", dep);
                need_top = true;
                break;
            }
        }

        if need_top {
            eprintln!(
                "Not sending observer {} due to missing dependencies",
                observer.name
            );
            continue;
        }

        match exe {
            Some(exe) => {
                println!("Sending observer: {}", observer.name);
                for dep in &deps {
                    if let Err(e) = send_executable(observer.url.clone(), ExeType::Dep, &dep).await
                    {
                        eprintln!("Failed to send dependency {}: {}", dep.name, e);
                    }
                }

                if deps.is_empty().not() || exe.need_update {
                    if let Err(e) = send_executable(observer.url.clone(), ExeType::Main, &exe).await
                    {
                        eprintln!("Failed to send observer {}: {}", observer.name, e);
                    } else {
                        println!("Observer {} sent successfully", observer.name);
                    }
                } else {
                    println!("Observer {} does not need update", observer.name);
                }
            }
            None => {
                eprintln!("Executable {} not found in state", observer.executable);
            }
        }
    }

    Ok(())
}
