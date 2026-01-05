//! Item UI construction.
//!
//! Responsible for:
//! - Building list/grid items from theme XML
//! - Applying provider-specific transformations
//! - Handling drag-and-drop for file-backed items

mod builder;
mod drag;

pub use builder::create_item;
pub use drag::create_drag_source;
