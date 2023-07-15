use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum FileEvent {
    Create(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
    MoveWithin(PathBuf, PathBuf),
    MoveOut(PathBuf),
    Write(PathBuf),
    MoveAndRename(PathBuf, PathBuf),
}

#[cfg(target_os = "macos")]
mod apple;

#[cfg(target_os = "macos")]
pub use apple::Watcher;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::Watcher;
