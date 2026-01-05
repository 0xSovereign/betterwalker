//! Preview widget container and constructors.

use gtk4::gio::Cancellable;
use gtk4::{Box as GtkBox, Builder, Orientation, Stack};

use std::cell::RefCell;
use std::rc::Rc;

/// GTK container holding preview content.
#[derive(Debug)]
pub struct PreviewWidget {
    pub box_widget: GtkBox,
    pub(crate) preview_area: Stack,
    pub(crate) current_content: String,
    pub current_video_cancellable: Rc<RefCell<Option<Cancellable>>>,
}

impl PreviewWidget {
    pub fn new_with_builder(builder: &Builder) -> Result<Self, Box<dyn std::error::Error>> {
        let box_widget = builder
            .object::<GtkBox>("PreviewBox")
            .ok_or("PreviewBox not found in builder")?;

        let preview_area = builder
            .object::<Stack>("PreviewStack")
            .ok_or("PreviewStack not found in builder")?;

        Ok(Self {
            box_widget,
            preview_area,
            current_content: String::new(),
            current_video_cancellable: Rc::new(RefCell::new(None)),
        })
    }

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let box_widget = GtkBox::new(Orientation::Vertical, 0);
        let preview_area = Stack::new();
        box_widget.append(&preview_area);

        Ok(Self {
            box_widget,
            preview_area,
            current_content: String::new(),
            current_video_cancellable: Rc::new(RefCell::new(None)),
        })
    }
}
