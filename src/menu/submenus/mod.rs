use super::MenuInternals;

mod all_nodes;
pub use all_nodes::AllNodesMenu;

mod bricks;
pub use bricks::BricksMenu;

mod children;
pub use children::{ChildrenMenu, ChildrenMenuInput};

mod diagnostics;
pub use diagnostics::{DiagnosticsMenu, DiagnosticsMenuInput};

mod images;
pub use images::ImagesMenu;

mod node_search;
pub use node_search::{NodeSearchMenu, NodeSearchMenuInput};

mod photons;
pub use photons::{PhotonsMenu, PhotonsMenuInput};

pub trait SubMenu {
    type InputData;
    type OutputData;

    fn is_showing(&self) -> bool;
    fn toggle_showing(&mut self);
    fn get_data(&self) -> &Self::OutputData;
    fn render(&mut self, internals: &MenuInternals, input: &Self::InputData);
}
