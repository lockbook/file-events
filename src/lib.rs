use std::path::{PathBuf, Path};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, PartialEq)]
pub enum FileEvent {
    Create(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
    MoveWithin(PathBuf, PathBuf),
    MoveOut(PathBuf),
}

pub struct Watcher {
    dest: PathBuf,
}

impl Watcher {
    pub fn new(dest: PathBuf) -> Self {
        Self {
            dest,
        }
    }

    pub fn watch_for_changes(&self) -> mpsc::Receiver<FileEvent> {
        //let mut watcher = Watcher::new(".".into());

        let (sender, receiver) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let deststr = self.dest.as_os_str().to_str().unwrap().to_string();

        let _t = thread::spawn(move || {
            let fsevent = fsevent::FsEvent::new(vec![deststr]);
            fsevent.observe(sender);
        });

        let _t2 = thread::spawn(move || {
            loop {
                println!("Here");
                let val = receiver.recv();
                println!("{:?}", val.as_ref().unwrap());
                let location = &val.as_ref().unwrap().clone().path;
                if val.as_ref().unwrap().flag.contains(fsevent::StreamFlags::ITEM_CREATED){
                    tx.send(FileEvent::Create((location.to_string()).into())).unwrap();
                }
                else if val.as_ref().unwrap().flag.contains(fsevent::StreamFlags::ITEM_REMOVED){
                    tx.send(FileEvent::Remove((location.to_string()).into())).unwrap();
                }
            }
        });

        rx
    }
}
