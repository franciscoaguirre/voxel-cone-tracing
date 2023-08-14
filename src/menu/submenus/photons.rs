use egui_backend::egui;

use super::SubMenu;
use crate::menu::{get_button_text, MenuInternals};

pub struct NodeSearchMenu {
    is_showing: bool,
}
