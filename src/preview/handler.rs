//! Preview dispatch and caching logic.

use crate::config::get_config;
use crate::protos::generated_proto::query::query_response::Item;
use crate::ui::window::get_selected_item;

use gtk4::{Box as GtkBox, Builder};

use std::cell::RefCell;

use super::PreviewWidget;

/// Thread-local preview handler.
///
/// Keeps exactly one cached PreviewWidget per thread
/// to avoid repeated GTK allocations.
#[derive(Debug)]
pub struct UnifiedPreviewHandler {
    cached_preview: RefCell<Option<PreviewWidget>>,
}

impl UnifiedPreviewHandler {
    pub fn new() -> Self {
        Self {
            cached_preview: RefCell::new(None),
        }
    }

    /// Drop cached widget and aggressively clear GTK children.
    pub fn clear_cache(&self) {
        let mut cached = self.cached_preview.borrow_mut();
        if let Some(widget) = cached.as_mut() {
            widget.clear_preview();
        }
        *cached = None;
    }

    /// Main preview dispatcher.
    ///
    /// Guards:
    /// - provider preview disabled
    /// - item not currently selected
    /// - cached content mismatch
    pub fn handle(&self, item: &Item, preview: &GtkBox, builder: &Builder) {
        let config = get_config();
        if config.providers.ignore_preview.contains(&item.provider) {
            return;
        }

        let Some(current) = get_selected_item() else {
            return;
        };
        if current != *item {
            return;
        }

        let mut cached = self.cached_preview.borrow_mut();

        if let Some(existing) = cached.as_ref()
            && existing.current_content
                != format!("{}{}{}", item.preview_type, item.preview, item.text)
        {
            *cached = None;
        }

        if cached.is_none()
            && let Ok(widget) =
                PreviewWidget::new_with_builder(builder).or_else(|_| PreviewWidget::new())
        {
            *cached = Some(widget);
        } else if cached.is_none() {
            return;
        }

        let widget = cached.as_mut().unwrap();

        let result = match item.preview_type.as_str() {
            "text" | "pango" => widget.preview_text(&item.preview, &item.preview_type),
            "file" => {
                if item.preview.is_empty() {
                    widget.preview_file(&item.text)
                } else {
                    widget.preview_file(&item.preview)
                }
            }
            "command" => widget.preview_command(&item.preview),
            _ => return,
        };

        if result.is_err() {
            return;
        }

        while let Some(child) = preview.first_child() {
            child.unparent();
        }

        preview.append(&widget.box_widget);
        preview.set_visible(get_selected_item().is_some_and(|c| c == *item));
    }
}

/// UI entrypoint.
pub fn handle_preview(item: &Item, preview: &GtkBox, builder: &Builder) {
    thread_local! {
        static PREVIEW_HANDLER: RefCell<UnifiedPreviewHandler> =
            RefCell::new(UnifiedPreviewHandler::new());
    }

    preview.add_css_class("preview-content");
    preview.add_css_class(&item.provider);

    PREVIEW_HANDLER.with(|h| h.borrow().handle(item, preview, builder));
}

/// Clears all thread-local preview caches.
pub fn clear_all_caches() {
    thread_local! {
        static PREVIEW_HANDLER: RefCell<UnifiedPreviewHandler> =
            RefCell::new(UnifiedPreviewHandler::new());
    }

    PREVIEW_HANDLER.with(|h| h.borrow().clear_cache());
}
