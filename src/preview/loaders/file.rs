//! File and command previews.

use std::path::Path;
use std::process::Command;

use gtk4::{PolicyType, ScrolledWindow, TextView, WrapMode, prelude::*};

use crate::preview::PreviewWidget;
use crate::renderers::create_drag_source;

impl PreviewWidget {
    pub fn preview_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.current_content = format!("file{}", file_path);
        self.clear_preview();

        if Path::new(file_path).is_absolute() {
            self.box_widget
                .add_controller(create_drag_source(file_path));
        }

        if !Path::new(file_path).exists() {
            return Err(format!("File does not exist: {}", file_path).into());
        }

        let Some(guess) = new_mime_guess::from_path(file_path).first() else {
            return self.preview_generic(file_path);
        };

        match (guess.type_(), guess.subtype()) {
            (mime::IMAGE, _) => self.preview_image(file_path),
            (mime::APPLICATION, mime::PDF) => self.preview_pdf(file_path),
            (mime::TEXT, _) => self.preview_text_file(file_path),
            (mime::VIDEO, _) => self.preview_video(file_path),
            _ => self.preview_generic(file_path),
        }
    }

    pub fn preview_command(&mut self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.current_content = format!("command{}", command);
        self.clear_preview();

        let output = Command::new("sh").arg("-c").arg(command).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let combined = if stderr.is_empty() {
            stdout.to_string()
        } else {
            format!("{}\n\nSTDERR:\n{}", stdout, stderr)
        };

        let view = TextView::new();
        view.set_editable(false);
        view.set_monospace(true);
        view.set_wrap_mode(WrapMode::Word);
        view.buffer().set_text(&combined);

        let scrolled = ScrolledWindow::new();
        scrolled.set_child(Some(&view));
        scrolled.set_policy(PolicyType::Automatic, PolicyType::Automatic);

        self.preview_area.add_child(&scrolled);
        self.preview_area.set_visible_child(&scrolled);
        Ok(())
    }
}
