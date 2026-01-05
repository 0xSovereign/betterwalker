use crate::ui::window::{quit, with_window};

use gtk4::gdk::ContentProvider;
use gtk4::gio::File;
use gtk4::gio::prelude::FileExt;
use gtk4::{DragSource, glib};

/// Create a drag source for file-backed items.
///
/// Drag payload:
/// - `text/uri-list`
///
/// Behavior:
/// - Hides window on drag start
/// - Quits application after drop
///
/// TODO:
/// - Support dragging non-file items (text, URLs)
/// - Make quit-on-drop configurable
pub fn create_drag_source(text: &str) -> DragSource {
    let drag_source = DragSource::new();
    let path = text.to_string();

    drag_source.connect_prepare(move |_, _, _| {
        let file = File::for_path(&path);
        let uri_string = format!("{}\n", file.uri());
        let bytes = glib::Bytes::from(uri_string.as_bytes());

        Some(ContentProvider::for_bytes("text/uri-list", &bytes))
    });

    drag_source.connect_drag_begin(|_, _| with_window(|w| w.window.set_visible(false)));

    drag_source.connect_drag_end(|_, _, _| with_window(|w| quit(&w.app, false)));

    drag_source
}
