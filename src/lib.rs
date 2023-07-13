

use std::path::{PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use fsevent::{Event, StreamFlags};

#[derive(Debug, PartialEq)]
pub enum FileEvent {
    Create(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
    MoveWithin(PathBuf, PathBuf),
    MoveOut(PathBuf),
    Modify(PathBuf)
}

pub struct RenameState {
    path: PathBuf,
    id: u64,
}

#[derive(Clone)]
pub struct Watcher {
    dest: PathBuf,
    rename_candidate: Arc<Mutex<Option<RenameState>>>,
}

impl Watcher {
    pub fn new(dest: PathBuf) -> Self {
        Self {
            dest,
            rename_candidate: Default::default(),
        }
    }

    pub fn watch_for_changes(&self) -> mpsc::Receiver<FileEvent> {
        let (sender, receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let deststr = self.dest.as_os_str().to_str().unwrap().to_string();

        thread::spawn(move || {
            let fsevent = fsevent::FsEvent::new(vec![deststr]);
            fsevent.observe(sender);
        });

        let event_loop_state = self.clone();
        thread::spawn(move || {
            event_loop_state.event_loop(receiver, tx);
        });

        rx
    }

    fn event_loop(&self, fsevent_rx: Receiver<Event>, tx: Sender<FileEvent>) {
        loop {
            let val = fsevent_rx.recv().unwrap();
            println!("{:?}", &val);
            let location = PathBuf::from(&val.path);
            let flags = &val.flag;
            if flags.contains(StreamFlags::ITEM_CREATED)
                && !flags.contains(StreamFlags::ITEM_REMOVED)
                && !flags.contains(StreamFlags::ITEM_RENAMED)
            {
                tx.send(FileEvent::Create(location)).unwrap();
            } else if flags.contains(StreamFlags::ITEM_REMOVED) {
                tx.send(FileEvent::Remove(location)).unwrap();
            } else if flags.contains(StreamFlags::ITEM_RENAMED) {
                let mut rename_candidate = self.rename_candidate.lock().unwrap();
                if let Some(pending) = rename_candidate.take() {
                    if pending.id == val.event_id - 1 {
                        tx.send(FileEvent::Rename(pending.path, location)).unwrap();
                    } else {
                        tx.send(FileEvent::MoveOut(pending.path)).unwrap();
                        tx.send(FileEvent::MoveOut(location)).unwrap();
                    }
                } else {
                    *rename_candidate = Some(RenameState {
                        path: location,
                        id: val.event_id,
                    });
                    let clone_self = self.clone();
                    let clone_tx = tx.clone();
                    thread::spawn(move || {
                        thread::sleep(Duration::from_millis(100));
                        if let Some(pending) = clone_self.rename_candidate.lock().unwrap().take() {
                            clone_tx.send(FileEvent::MoveOut(pending.path)).unwrap();
                        }
                    });
                }
            } else if flags.contains(StreamFlags::INODE_META_MOD) || flags.contains(StreamFlags::ITEM_MODIFIED){
                tx.send(FileEvent::Modify(location)).unwrap();
            }
        }
    }
}
