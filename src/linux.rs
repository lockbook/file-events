use std::{path::PathBuf, sync::mpsc};

use crate::FileEvent;

pub struct Watcher {}

impl Watcher {
    pub fn new(_: PathBuf) -> Self {
        todo!()
    }

    pub fn watch_for_changes(&self) -> mpsc::Receiver<FileEvent> {
        todo!()
    }
}
