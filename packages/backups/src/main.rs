use dotenv::dotenv;
use std::env;
use notify::{Error, Event, EventKind, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc, fs};

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let watch_path = env::var("TERRARIA_WORLDS_DIR").expect("TERRARIA_WORLDS_DIR not set");
    let backup_path = env::var("BACKUP_WORLDS_DIR").expect("BACKUP_WORLDS_DIR not set");

    log::info!("Watching path: {}", &watch_path);
    log::info!("Backups directory: {}", &backup_path);

    let backup_path = Path::new(&backup_path);
    if !backup_path.exists() {
        match fs::create_dir(backup_path) {
            Ok(_) => log::info!("Created backup directory: {}", &backup_path.display()),
            Err(error) => {
                log::error!("Created backup directory: {} - {:?}", &backup_path.display(), error);
                return Err(Error::io(error));
            }
        }
    }

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        log::info!("Created event");
                    }
                    _ => ()
                }
            },
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }

    Ok(())
}
