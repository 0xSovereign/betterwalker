//! Image, PDF, and video previews.

use gtk4::gio::{self, Cancellable};
use gtk4::glib::{self, Bytes};
use gtk4::{
    Box as GtkBox, ContentFit, Image, Orientation, Picture, PolicyType, ScrolledWindow, Video,
    gdk_pixbuf, prelude::*,
};
use poppler::{Document, Page};

use std::rc::Rc;

use crate::preview::PreviewWidget;

impl PreviewWidget {
    pub fn preview_image(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let picture = Picture::new();
        picture.set_content_fit(ContentFit::Contain);

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&picture));
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);

        self.preview_area.add_child(&scrolled);
        self.preview_area.set_visible_child(&scrolled);

        let path = file_path.to_string();
        let pic = picture.clone();

        glib::MainContext::ref_thread_default().spawn_local(async move {
            match Self::load_and_resize_image(&path).await {
                Ok(tex) => pic.set_paintable(Some(&tex)),
                Err(_) => pic.set_file(Some(&gio::File::for_path(&path))),
            }
        });

        Ok(())
    }

    pub fn preview_pdf(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let uri = format!("file://{file_path}");
        let document = Document::from_file(&uri, None)?;

        let container = GtkBox::new(Orientation::Vertical, 0);

        if let Some(page) = document.page(0) {
            container.append(&self.render_pdf_page(&page)?);
        }

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&container));
        scrolled.set_policy(PolicyType::Never, PolicyType::Automatic);

        self.preview_area.add_child(&scrolled);
        self.preview_area.set_visible_child(&scrolled);
        Ok(())
    }

    pub fn render_pdf_page(&self, page: &Page) -> Result<GtkBox, Box<dyn std::error::Error>> {
        let (w, h) = page.size();
        let scale = 800.0 / w;

        let mut surface = cairo::ImageSurface::create(
            cairo::Format::ARgb32,
            (w * scale) as i32,
            (h * scale) as i32,
        )?;

        let ctx = cairo::Context::new(&surface)?;
        ctx.scale(scale, scale);
        page.render(&ctx);
        surface.flush();

        let mut data = surface.data()?.to_vec();
        for c in data.chunks_exact_mut(4) {
            c.swap(0, 2);
        }

        let bytes = Bytes::from_owned(data);
        let tex = gtk4::gdk::MemoryTexture::new(
            surface.width(),
            surface.height(),
            gtk4::gdk::MemoryFormat::R8g8b8a8,
            &bytes,
            surface.width() as usize * 4,
        );

        let pic = Picture::for_paintable(&tex);
        let box_ = GtkBox::new(Orientation::Vertical, 0);
        box_.append(&pic);
        Ok(box_)
    }

    pub fn preview_video(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(c) = self.current_video_cancellable.borrow_mut().take() {
            c.cancel();
        }

        let cancellable = Cancellable::new();
        *self.current_video_cancellable.borrow_mut() = Some(cancellable.clone());

        let scrolled = self.get_or_create_scrolled();
        let path = file_path.to_string();

        glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            if cancellable.is_cancelled() {
                return glib::ControlFlow::Break;
            }

            let file = gio::File::for_path(&path);
            let video = Video::for_file(Some(&file));
            video.set_autoplay(true);

            scrolled.set_child(Some(&video));
            glib::ControlFlow::Break
        });

        Ok(())
    }

    pub fn preview_generic(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let container = GtkBox::new(Orientation::Vertical, 10);
        let icon = Image::from_icon_name("text-x-generic");

        let file = gio::File::for_path(file_path);
        if let Ok(info) = file.query_info("standard::icon", gio::FileQueryInfoFlags::NONE, None)
            && let Some(gicon) = info.icon()
        {
            icon.set_from_gicon(&gicon);
        }

        container.append(&icon);
        self.preview_area.add_child(&container);
        self.preview_area.set_visible_child(&container);
        Ok(())
    }
}
