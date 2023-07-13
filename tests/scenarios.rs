use std::{fs, path::PathBuf, thread, time::Duration};

use file_events::{FileEvent, Watcher};

fn create_dir() -> PathBuf {
    let path = uuid::Uuid::new_v4();
    let path = PathBuf::from(format!("./{path}"));
    fs::create_dir(&path).unwrap();
    thread::sleep(Duration::from_millis(100));
    path.canonicalize().unwrap()
}

#[test]
fn create_test() {
    let dest = create_dir();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    let mut newfile = dest.clone();
    newfile.push("new_file");
    fs::File::create(&newfile).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::Create(newfile));
    fs::remove_dir_all(dest).unwrap();
}

#[test]
fn remove_test() {
    let dest = create_dir();
    let mut newfile = dest.clone();
    newfile.push("new_file");
    fs::File::create(&newfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::remove_file(&newfile).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::Remove(newfile));
    fs::remove_dir_all(dest).unwrap();
}

#[test]
fn rename_test() {
    let dest = create_dir();
    let mut newfile = dest.clone();
    newfile.push("new_file");
    let mut newfile2 = dest.clone();
    newfile2.push("new_file_2");
    fs::File::create(&newfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::rename(&newfile, &newfile2).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::Rename(newfile, newfile2));
    fs::remove_dir_all(dest).unwrap();
}

#[test]
fn move_out_test(){
    let dest = create_dir();
    let mut newfile = dest.clone();
    newfile.push("new_file");
    let newfile2 = PathBuf::from("./new_file");
    fs::File::create(&newfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::rename(&newfile, &newfile2).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::MoveOut(newfile));
    fs::remove_dir_all(dest).unwrap();
    fs::remove_file(newfile2).unwrap();
}
