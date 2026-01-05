use crate::config::get_config;
use crate::protos::generated_proto::query::query_response::Item;
use crate::providers::PROVIDERS;
use crate::state::{get_dmenu_current, is_grid, is_hide_qa, set_error};
use crate::theme::{Theme, with_themes};
use crate::ui::item::drag::create_drag_source;

use gtk4::prelude::{ListItemExt, WidgetExt};
use gtk4::{Box, Builder, Label, ListItem};

use std::path::Path;

/// Build a single list or grid item widget.
///
/// Flow:
/// - Select layout XML (list vs grid)
/// - Build GTK widgets via `Builder`
/// - Apply provider-specific transformations
/// - Attach drag source if item points to a file
///
/// Error handling:
/// - If a theme is broken (missing 'ItemBox'), it will fall back to default theme
///
/// TODO:
/// - Validate XML structure earlier (theme load time)
/// - Avoid rebuilding Builder on fallback
pub fn create_item(list_item: &ListItem, item: &Item, theme: &Theme) {
    let mut builder = Builder::new();

    // Load the correct XML layout
    let _ = if !is_grid() {
        builder.add_from_string(
            theme
                .items
                .get(&item.provider)
                .unwrap_or_else(|| panic!("failed to get item layout: {}", &item.provider)),
        )
    } else {
        builder.add_from_string(
            theme
                .grid_items
                .get(&item.provider)
                .unwrap_or_else(|| panic!("failed to get item grid layout: {}", &item.provider)),
        )
    };

    // Resolve ItemBox (required root widget)
    let itembox: Box = match builder.object("ItemBox") {
        Some(w) => w,
        None => {
            // Broken theme: report error and fall back
            set_error("Theme: missing 'ItemBox' object".to_string());

            builder = Builder::new();

            with_themes(|themes| {
                let default = themes.get("default").unwrap();
                let _ = builder.add_from_string(
                    default
                        .items
                        .get(&item.provider)
                        .expect("failed to get item layout"),
                );
            });

            builder.object("ItemBox").unwrap()
        }
    };

    // Provider-specific CSS class
    itembox.add_css_class(&item.provider.replace("menus:", "menus-"));

    // State-based CSS classes
    item.state
        .iter()
        .filter(|s| !s.is_empty())
        .for_each(|s| itembox.add_css_class(s));

    // Highlight current item (dmenu-style navigation)
    if get_dmenu_current() != 0 && get_dmenu_current() as u32 == list_item.position() + 1 {
        itembox.add_css_class("current");
    }

    list_item.set_child(Some(&itembox));

    // Enable drag-and-drop for absolute paths
    if Path::new(&item.text).is_absolute() {
        itembox.add_controller(create_drag_source(&item.text));
    }

    let provider = PROVIDERS.get().unwrap().get(&item.provider).unwrap();

    // Apply provider text transformations
    if let Some(label) = builder.object::<Label>("ItemText") {
        provider.text_transformer(item, &label);
    }

    if let Some(label) = builder.object::<Label>("ItemSubtext") {
        provider.subtext_transformer(item, &label);
    }

    // Provider image/icon handling
    provider.image_transformer(&builder, list_item, item);

    // Quick-activation label handling
    if let Some(label) = builder.object::<Label>("QuickActivation") {
        if is_hide_qa() || get_config().hide_quick_activation {
            label.set_visible(false);
            return;
        }

        if let Some(bindings) = &get_config().keybinds.quick_activate {
            let index = list_item.position() as usize;
            if let Some(value) = bindings.get(index) {
                label.set_label(value);
            } else {
                label.set_visible(false);
            }
        }
    }
}
