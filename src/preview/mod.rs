//! Owns preview dispatch, caching, widget construction,
//! and all preview rendering logic.

mod handler;
mod loaders;
mod widget;

pub use handler::{clear_all_caches, handle_preview};
pub use widget::PreviewWidget;
