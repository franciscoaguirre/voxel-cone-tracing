use egui_glfw_gl::{egui, glfw};

use crate::prelude::{AssetRegistry, Scene};

/// The menu is made up of submenus.
/// Every submenu must implement the `SubMenu` trait.
pub trait SubMenu {
    /// Show the submenu.
    fn show(&self, context: &egui::Context, scene: &Scene, assets: &mut AssetRegistry);

    /// Handle window events while the menu is on.
    /// Most submenus won't need to implement this, that's why
    /// there's an empty default implementation.
    fn handle_event(
        &mut self,
        event: &glfw::WindowEvent,
        context: &egui::Context,
        assets: &mut AssetRegistry,
    ) {
    }
}

pub trait Showable {
    fn should_show(&self) -> bool;
    fn toggle_showing(&mut self);
}
