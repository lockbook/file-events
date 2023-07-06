use std::path::PathBuf;

use file_events::Watcher;

#[test]
fn create_test() {
    let watcher = Watcher::new(PathBuf::default());
}
