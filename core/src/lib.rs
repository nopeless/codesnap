mod code;
mod color;
mod components;
mod config;
mod copy;
mod copy_ascii;
mod edges;
mod highlight;
mod path;
mod save;
mod snapshot;
mod text;

pub use config::TakeSnapshotParams;
pub use copy::copy_into_clipboard;
pub use copy_ascii::copy_ascii;
pub use save::save_snapshot;
