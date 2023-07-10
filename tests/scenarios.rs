use std::{path::PathBuf, fs, time::Duration, thread};

use file_events::{Watcher, FileEvent};

#[test]
fn create_test() {
    let watcher = Watcher::new(PathBuf::from("."));
    let x = watcher.watch_for_changes();
    let dest = "./new_file";
    fs::File::create(&dest).unwrap();
    println!("{:?}", x);
    //thread::sleep(Duration::from_millis(1500));
    let evt = x.recv().unwrap();
    assert_eq!(evt, FileEvent::Create("./new_file".into()));
    
}

