use crate::theme::with_themes;
use crate::ui::window::{set_css_provider, with_css_provider};
use gtk4::{CssProvider, gdk::Display};
use std::fs;

/// Apply CSS for the given theme.
///
/// Priority:
/// 1. SCSS (compiled at runtime)
/// 2. CSS file
/// 3. Built-in default CSS
///
/// TODO:
/// - Cache compiled SCSS output
/// - Add file watcher for live reload
pub fn setup_css(theme: String) {
    with_themes(|themes| {
        if let Some(t) = themes.get(&theme) {
            with_css_provider(|provider| {
                if let Some(path) = &t.scss {
                    if let Ok(scss) = fs::read_to_string(path) {
                        let opts = match path.parent() {
                            Some(dir) => grass::Options::default().load_path(dir),
                            None => grass::Options::default(),
                        };

                        if let Ok(css) = grass::from_string(scss, &opts) {
                            provider.load_from_string(&css);
                            return;
                        }
                    }
                }

                if let Some(css) = &t.css {
                    provider.load_from_file(css);
                } else {
                    provider
                        .load_from_string(include_str!("../../resources/themes/default/style.css"));
                }
            });
        }
    });
}

/// Initialize global GTK CSS provider.
pub fn setup_css_provider() {
    let display = Display::default().unwrap();
    let provider = CssProvider::new();

    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );

    set_css_provider(provider);
}
