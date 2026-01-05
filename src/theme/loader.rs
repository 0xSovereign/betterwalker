use super::Theme;
use crate::config::get_config;
use crate::providers::PROVIDERS;
use crate::state::add_theme;
use crate::theme::state::THEMES;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Discover and load themes from disk.
///
/// Behavior:
/// - Supports single-theme mode (non-service)
/// - Supports multi-theme discovery (service mode)
/// - Merges provider layouts conditionally (`elephant`)
///
/// TODO:
/// - Detect duplicate theme names across directories
/// - Validate XML before insertion
pub fn setup_themes(elephant: bool, theme: String, is_service: bool) {
    let mut themes: HashMap<String, Theme> = HashMap::new();

    let dirs = xdg::BaseDirectories::with_prefix("walker").find_config_files("themes");

    let mut config_paths: Vec<PathBuf> = dirs.collect();

    if let Some(extra) = &get_config().additional_theme_location
        && let Ok(home) = env::var("HOME")
    {
        config_paths.push(PathBuf::from(extra.replace("~", &home)));
    }

    let base_files = vec![
        "layout.xml",
        "keybind.xml",
        "style.scss",
        "style.css",
        "preview.xml",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    let files = if elephant {
        let mut combined = base_files;
        combined.extend(
            PROVIDERS
                .get()
                .unwrap()
                .iter()
                .map(|v| format!("item_{}.xml", v.0)),
        );
        combined
    } else {
        base_files
    };

    for config_path in config_paths {
        if !is_service {
            let mut path = config_path.clone();
            path.push(&theme);

            if let Some(t) = setup_theme_from_path(path, &files) {
                themes.insert(theme.clone(), t);
                add_theme(theme.clone());
            }

            continue;
        }

        let Ok(entries) = fs::read_dir(config_path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let Some(name) = path.file_name() else {
                continue;
            };
            let theme_name = name.to_string_lossy().to_string();

            if let Some(t) = setup_theme_from_path(path, &files) {
                themes.insert(theme_name.clone(), t);
                add_theme(theme_name);
            }
        }
    }

    if !themes.contains_key("default") {
        themes.insert("default".to_string(), Theme::default());
        add_theme("default".to_string());
    }

    THEMES.with(|cell| {
        cell.set(themes).expect("failed initializing themes");
    });
}

/// Load a single theme directory.
///
/// Returns `None` if the directory does not exist.
fn setup_theme_from_path(path: PathBuf, files: &Vec<String>) -> Option<Theme> {
    if !path.exists() {
        return None;
    }

    let mut theme = Theme::default();
    let mut base = path.clone();

    let mut read_file = |filename: &str| -> Option<String> {
        base.push(filename);
        let result = fs::read_to_string(&base).ok();
        base.pop();
        result
    };

    for file in files {
        match file.as_str() {
            "style.scss" => {
                base.push(file);
                theme.scss = Some(base.clone());
                base.pop();
            }
            "style.css" => {
                base.push(file);
                theme.css = Some(gtk4::gio::File::for_path(&base));
                base.pop();
            }
            "layout.xml" => theme.layout = read_file(file)?,
            "keybind.xml" => theme.keybind = read_file(file)?,
            "preview.xml" => theme.preview = read_file(file)?,

            name if name.starts_with("item_") && name.ends_with("_grid.xml") => {
                let key = name.strip_prefix("item_")?.strip_suffix("_grid.xml")?;
                theme.grid_items.insert(key.to_string(), read_file(file)?);
            }

            name if name.starts_with("item_") && name.ends_with(".xml") => {
                let key = name.strip_prefix("item_")?.strip_suffix(".xml")?;
                theme.items.insert(key.to_string(), read_file(file)?);
            }

            _ => {}
        }
    }

    Some(theme)
}
