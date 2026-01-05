use crate::config::get_config;
use gtk4::Window;
use gtk4::prelude::GtkWindowExt;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

/// Configure layer-shell behavior for the main window.
///
/// Falls back gracefully when layer-shell is unsupported.
pub fn setup_layer_shell(win: &Window) {
    if !gtk4_layer_shell::is_supported() {
        let titlebar = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        win.set_titlebar(Some(&titlebar));
        return;
    }

    let cfg = get_config();

    win.init_layer_shell();
    win.set_namespace(Some("walker"));
    win.set_exclusive_zone(-1);
    win.set_layer(Layer::Overlay);
    win.set_keyboard_mode(if cfg.force_keyboard_focus {
        KeyboardMode::Exclusive
    } else {
        KeyboardMode::OnDemand
    });

    win.set_anchor(Edge::Left, cfg.shell.anchor_left);
    win.set_anchor(Edge::Right, cfg.shell.anchor_right);
    win.set_anchor(Edge::Top, cfg.shell.anchor_top);
    win.set_anchor(Edge::Bottom, cfg.shell.anchor_bottom);
}
