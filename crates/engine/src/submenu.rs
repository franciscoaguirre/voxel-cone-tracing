use egui_glfw_gl::{egui, glfw};

use crate::{
    prelude::{AssetRegistry, Scene},
    system::SystemInfo,
};

pub struct SubMenuInputs<'a, SystemType> {
    pub scene: &'a Scene,
    pub assets: &'a mut AssetRegistry,
    pub system_info: &'a [SystemInfo],
    pub systems: &'a mut [SystemType],
}

/// The menu is made up of submenus.
/// Every submenu must implement the `SubMenu` trait.
pub trait SubMenu<SystemType> {
    /// Show the submenu.
    fn show(&mut self, context: &egui::Context, inputs: &mut SubMenuInputs<SystemType>);

    /// Handle window events while the menu is on.
    /// Most submenus won't need to implement this, that's why
    /// there's an empty default implementation.
    fn handle_event(
        &mut self,
        event: &glfw::WindowEvent,
        context: &egui::Context,
        inputs: &mut SubMenuInputs<SystemType>,
    ) {
    }
}

pub trait Showable {
    fn should_show(&self) -> bool;
    fn toggle_showing(&mut self);
}
