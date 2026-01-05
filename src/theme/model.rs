use crate::providers::PROVIDERS;
use gtk4::gio;
use std::collections::HashMap;
use std::path::PathBuf;

/// In-memory representation of a theme.
///
/// A theme is composed of:
/// - XML layouts (window, items, grid items, preview)
/// - Optional SCSS / CSS styling
/// - Provider-specific item layouts
///
/// NOTE:
/// - `scss` and `css` are mutually exclusive at runtime
/// - Provider layouts are populated eagerly
#[derive(Debug)]
pub struct Theme {
    pub layout: String,
    pub keybind: String,
    pub preview: String,

    /// Path to SCSS source file (compiled at runtime)
    pub scss: Option<PathBuf>,

    /// GTK-compatible CSS file
    pub css: Option<gio::File>,

    /// Per-provider list item layouts
    pub items: HashMap<String, String>,

    /// Per-provider grid item layouts
    pub grid_items: HashMap<String, String>,
}

impl Theme {
    /// Built-in fallback theme.
    ///
    /// This theme is always available and cannot fail to load.
    /// Provider layouts are injected dynamically.
    pub fn default() -> Self {
        let mut theme = Self {
            layout: include_str!("../../resources/themes/default/layout.xml").to_string(),
            keybind: include_str!("../../resources/themes/default/keybind.xml").to_string(),
            preview: include_str!("../../resources/themes/default/preview.xml").to_string(),
            scss: None,
            css: None,
            items: HashMap::new(),
            grid_items: HashMap::new(),
        };

        // Populate provider item layouts
        for (k, v) in PROVIDERS.get().unwrap() {
            theme.items.insert(k.clone(), v.get_item_layout());
            theme.grid_items.insert(k.clone(), v.get_item_grid_layout());
        }

        theme
    }
}
