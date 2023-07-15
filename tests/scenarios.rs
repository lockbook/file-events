use file_events::{FileEvent, Watcher};
use std::{fs, path::PathBuf, thread, time::Duration};

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
fn move_out_test() {
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

#[test]
fn double_test() {
    let dest = create_dir();
    let mut newfile = dest.clone();
    newfile.push("new_file");
    let mut anotherfile = dest.clone();
    anotherfile.push("another_file");
    let newfile2 = PathBuf::from("./new_file");
    let anotherfile2 = PathBuf::from("./another_file");
    fs::File::create(&newfile).unwrap();
    fs::File::create(&anotherfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::rename(&newfile, &newfile2).unwrap();
    fs::rename(&anotherfile, &anotherfile2).unwrap();
    let evt = x.recv().unwrap();
    let evt2 = x.recv().unwrap();
    assert_eq!(evt, FileEvent::MoveOut(newfile));
    assert_eq!(evt2, FileEvent::MoveOut(anotherfile));
    fs::remove_dir_all(dest).unwrap();
    fs::remove_file(newfile2).unwrap();
    fs::remove_file(anotherfile2).unwrap();
}

#[test]
fn move_within_test() {
    let dest = create_dir();
    let mut newfolder = dest.clone();
    newfolder.push("new_folder/");
    let mut newfolder2 = dest.clone();
    newfolder2.push("new_folder_2/");
    fs::create_dir(&newfolder).unwrap();
    fs::create_dir(&newfolder2).unwrap();
    thread::sleep(Duration::from_millis(100));
    let mut newfile = newfolder.clone();
    newfile.push("new_file");
    let mut newfile2 = newfolder2.clone();
    newfile2.push("new_file");
    fs::File::create(&newfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::rename(&newfile, &newfile2).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::MoveWithin(newfile, newfile2));
    fs::remove_dir_all(dest).unwrap();
}

#[test]
fn move_and_rename_test() {
    let dest = create_dir();
    let mut newfolder = dest.clone();
    newfolder.push("new_folder/");
    let mut newfolder2 = dest.clone();
    newfolder2.push("new_folder_2/");
    fs::create_dir(&newfolder).unwrap();
    fs::create_dir(&newfolder2).unwrap();
    thread::sleep(Duration::from_millis(100));
    let mut newfile = newfolder.clone();
    newfile.push("new_file");
    let mut newfile2 = newfolder2.clone();
    newfile2.push("new_file_2");
    fs::File::create(&newfile).unwrap();
    let watcher = Watcher::new(dest.clone());
    let x = watcher.watch_for_changes();
    fs::rename(&newfile, &newfile2).unwrap();
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::MoveAndRename(newfile, newfile2));
    fs::remove_dir_all(dest).unwrap();
}
