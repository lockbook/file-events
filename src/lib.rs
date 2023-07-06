use std::path::PathBuf;
use std::sync::mpsc;

pub enum FileEvent {
    Create(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
    MoveWithin(PathBuf, PathBuf),
    MoveOut(PathBuf),
}

pub struct Watcher {
    dest: PathBuf,

    tx: mpsc::Sender<FileEvent>,
    rx: mpsc::Receiver<FileEvent>,
}

impl Watcher {
    pub fn new(dest: PathBuf) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            dest,

            rx,
            tx,
        }
    }

    pub fn watch_for_changes(&self) -> mpsc::Receiver<FileEvent> {
        todo!()
    }
}
