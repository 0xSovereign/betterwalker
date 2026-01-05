use super::Theme;
use std::cell::OnceCell;
use std::collections::HashMap;

/// Global theme registry.
///
/// Initialized once during startup via `setup_themes`.
thread_local! {
    pub static THEMES: OnceCell<HashMap<String, Theme>> = OnceCell::new();
}

/// Access the loaded themes.
///
/// # Panics
/// - If called before `setup_themes`
pub fn with_themes<F, R>(f: F) -> R
where
    F: FnOnce(&HashMap<String, Theme>) -> R,
{
    THEMES.with(|state| {
        let data = state.get().expect("Themes not initialized");
        f(data)
    })
}
