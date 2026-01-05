//! Theme system entrypoint.
//!
//! Responsible for:
//! - Discovering themes from disk
//! - Loading layouts, previews, styles
//! - Managing theme state
//! - GTK CSS + layer-shell setup

mod css;
mod layer_shell;
mod loader;
mod model;
mod state;

pub use css::{setup_css, setup_css_provider};
pub use layer_shell::setup_layer_shell;
pub use loader::{setup_themes, with_themes};
pub use model::Theme;
