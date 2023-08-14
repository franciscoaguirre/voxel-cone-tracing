use super::MenuInternals;

mod all_nodes;
mod bricks;
mod diagnostics;
mod images;
mod node_search;

pub trait SubMenu {
    type InputData;
    type OutputData;

    fn is_showing(&self) -> bool;
    fn toggle_showing(&mut self);
    fn get_data(&self) -> &Self::OutputData;
    fn render(&self, internals: MenuInternals, input: &Self::InputData);
}
