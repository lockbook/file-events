use std::{path::PathBuf, fs};

use file_events::Watcher;

#[test]
fn create_test() {
    let watcher = Watcher::new(PathBuf::default());
    let x = watcher.watch_for_changes();
    let mut dest = "./new_file";
    fs::File::create(&dest).unwrap();
    println!("{:?}", x);
    assert_eq!(dest, "diff");
}

