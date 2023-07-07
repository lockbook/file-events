use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

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
        let mut watcher = Watcher::new(".".into());

        let (sender, receiver) = mpsc::channel();

        let _t = thread::spawn(move || {
            let fsevent = fsevent::FsEvent::new(vec![".".to_string()]);
            fsevent.observe(sender);
        });

        loop {
            let val = receiver.recv();
            println!("{:?}", val.unwrap());
        }
    }
}
