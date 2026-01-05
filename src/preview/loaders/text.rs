//! Text and text-file previews.

use gtk4::{Label, PolicyType, ScrolledWindow, TextView, WrapMode, prelude::*};

use std::io::{BufRead, BufReader};

use crate::preview::PreviewWidget;

impl PreviewWidget {
    /// Preview inline text or pango markup.
    pub fn preview_text(&mut self, text: &str, pt: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.current_content = format!("text{}", text);
        self.clear_preview();

        let label = Label::new(None);
        label.set_selectable(true);
        label.set_wrap(true);
        label.set_wrap_mode(gtk4::pango::WrapMode::Word);
        label.set_xalign(0.0);

        let display = if text.len() > 10_000 {
            format!("{}\n\n[Text truncated...]", &text[..10_000])
        } else {
            text.to_string()
        };

        if pt == "pango" {
            label.set_use_markup(true);
            label.set_markup(&display);
        } else {
            label.set_text(&display);
        }

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&label));
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);

        self.preview_area.add_child(&scrolled);
        self.preview_area.set_visible_child(&scrolled);
        Ok(())
    }

    /// Preview a text file with a hard size cap.
    pub fn preview_text_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::open(file_path)?;
        let mut reader = BufReader::new(file);

        let max_size = 512 * 1024;
        let mut content = String::new();
        let mut total = 0;
        let mut buf = String::new();

        while total < max_size {
            buf.clear();
            let read = reader.read_line(&mut buf)?;
            if read == 0 {
                break;
            }
            if total + read > max_size {
                content.push_str(&buf[..max_size - total]);
                content.push_str("\n\n[File truncated...]");
                break;
            }
            content.push_str(&buf);
            total += read;
        }

        let view = TextView::new();
        view.set_editable(false);
        view.set_monospace(true);
        view.set_wrap_mode(WrapMode::Word);
        view.buffer().set_text(&content);

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&view));
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);

        self.preview_area.add_child(&scrolled);
        self.preview_area.set_visible_child(&scrolled);
        Ok(())
    }
}
