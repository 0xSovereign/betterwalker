//! Shared helpers and cleanup logic.

use gtk4::{Image, Picture, PolicyType, ScrolledWindow, TextView, prelude::*};

use crate::preview::PreviewWidget;

impl PreviewWidget {
    pub fn clear_preview(&self) {
        while let Some(child) = self.preview_area.first_child() {
            if let Some(scrolled) = child.downcast_ref::<ScrolledWindow>() {
                if let Some(inner) = scrolled.child() {
                    if let Some(tv) = inner.downcast_ref::<TextView>() {
                        tv.buffer().set_text("");
                    }
                    if let Some(pic) = inner.downcast_ref::<Picture>() {
                        pic.set_paintable(gtk4::gdk::Paintable::NONE);
                    }
                    if let Some(img) = inner.downcast_ref::<Image>() {
                        img.clear();
                    }
                }
            }
            self.preview_area.remove(&child);
        }
    }

    pub fn get_or_create_scrolled(&self) -> ScrolledWindow {
        if let Some(child) = self.preview_area.first_child()
            && let Ok(scrolled) = child.downcast::<ScrolledWindow>()
        {
            return scrolled;
        }

        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        self.preview_area.add_child(&scrolled);
        scrolled
    }

    pub async fn load_and_resize_image(
        file_path: &str,
    ) -> Result<gtk4::gdk::Texture, Box<dyn std::error::Error>> {
        let file = gtk4::gio::File::for_path(file_path);
        let (bytes, _) = file.load_bytes_future().await?;
        let stream = gtk4::gio::MemoryInputStream::from_bytes(&bytes);

        let pixbuf = gtk4::gdk_pixbuf::Pixbuf::from_stream(&stream, None)?;

        let resized = pixbuf
            .scale_simple(
                pixbuf.width().min(800),
                pixbuf.height().min(600),
                gtk4::gdk_pixbuf::InterpType::Bilinear,
            )
            .unwrap_or(pixbuf);

        Ok(gtk4::gdk::Texture::for_pixbuf(&resized))
    }
}
