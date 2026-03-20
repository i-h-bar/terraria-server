use dotenv::dotenv;
use notify_debouncer_full::{
    DebounceEventResult, new_debouncer,
    notify::{Error, EventKind, RecursiveMode, Result},
};
use std::collections::HashSet;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, path::Path, sync::mpsc};

fn backup_file(src: &Path, backup_dir: &Path, max_saves: usize) {
    if !src.is_file() {
        return;
    }

    match src.extension() {
        Some(ext) => {
            if ext != "wld" {
                return;
            }
        }
        None => return,
    }

    let file_name = match src.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return,
    };

    log::info!("Backing up {}", src.display());
    let suffix = format!("_{file_name}");
    let mut existing: Vec<(u64, std::path::PathBuf)> = match fs::read_dir(backup_dir) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                let ts = name.strip_suffix(&suffix)?.parse::<u64>().ok()?;
                Some((ts, e.path()))
            })
            .collect(),
        Err(e) => {
            log::error!("Failed to read backup dir: {e:?}");
            return;
        }
    };

    existing.sort_unstable_by_key(|(ts, _)| *ts);

    while existing.len() >= max_saves {
        let (_, oldest) = existing.remove(0);
        match fs::remove_file(&oldest) {
            Ok(()) => log::info!("Removed old backup {}", oldest.display()),
            Err(e) => log::error!("Failed to remove old backup {}: {:?}", oldest.display(), e),
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let dest = backup_dir.join(format!("{timestamp}_{file_name}"));
    match fs::copy(src, &dest) {
        Ok(_) => log::info!("Backed up {} -> {}", src.display(), dest.display()),
        Err(e) => log::error!("Failed to backup {}: {:?}", src.display(), e),
    }
}

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let watch_path = env::var("TERRARIA_WORLDS_DIR").expect("TERRARIA_WORLDS_DIR not set");
    let backup_path = env::var("BACKUP_WORLDS_DIR").expect("BACKUP_WORLDS_DIR not set");
    let max_saves = env::var("MAX_SAVES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(10);

    log::info!("Watching path: {}", &watch_path);
    log::info!("Backups directory: {}", &backup_path);
    log::info!("Max saves per file: {max_saves}");

    let backup_path = Path::new(&backup_path);
    if !backup_path.exists() {
        match fs::create_dir(backup_path) {
            Ok(()) => log::info!("Created backup directory: {}", &backup_path.display()),
            Err(error) => {
                log::error!(
                    "Created backup directory: {} - {:?}",
                    &backup_path.display(),
                    error
                );
                return Err(Error::io(error));
            }
        }
    }

    let (tx, rx) = mpsc::channel::<DebounceEventResult>();
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;
    debouncer.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(events) => {
                let mut paths_to_backup = HashSet::new();
                for event in &events {
                    match event.event.kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            for path in &event.event.paths {
                                paths_to_backup.insert(path.clone());
                            }
                        }
                        _ => (),
                    }
                }
                for path in paths_to_backup {
                    backup_file(&path, backup_path, max_saves);
                }
            }
            Err(errors) => {
                for e in errors {
                    log::error!("watch error: {e:?}");
                }
            }
        }
    }

    Ok(())
}
