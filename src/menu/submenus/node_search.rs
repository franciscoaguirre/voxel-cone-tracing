use egui_glfw_gl::egui;

use super::SubMenu;
use crate::menu::{get_button_text, DebugNode, MenuInternals};

#[derive(Default)]
pub struct NodeSearchMenu {
    is_showing: bool,
    output: NodeSearchMenuOutput,
}

pub struct NodeSearchMenuInput {
    items: Vec<DebugNode>,
}

impl NodeSearchMenuInput {
    pub fn new(items: Vec<DebugNode>) -> Self {
        Self { items }
    }
}

#[derive(Default)]
pub struct NodeSearchMenuOutput {
    pub selected_items: Vec<DebugNode>,
    pub filter_text: String,
    pub should_show_neighbors: bool,
    pub selected_items_updated: bool,
}

impl SubMenu for NodeSearchMenu {
    type InputData = NodeSearchMenuInput;
    type OutputData = NodeSearchMenuOutput;

    fn is_showing(&self) -> bool {
        self.is_showing
    }

    fn toggle_showing(&mut self) {
        self.is_showing = !self.is_showing;
    }

    fn get_data(&self) -> &Self::OutputData {
        &self.output
    }

    fn render(&mut self, internals: &MenuInternals, input: &Self::InputData) {
        if !self.is_showing() {
            return;
        }

        // Variables for handling modifications to `selected_items`
        let mut should_clear = false;
        let mut index_to_push = None;
        let mut index_to_remove = None;

        egui::Window::new("Node search")
            .resize(|r| r.fixed_size((200., 400.)))
            .show(&internals.context, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neighbors: ");
                        if ui
                            .button(get_button_text("Toggle", self.output.should_show_neighbors))
                            .clicked()
                        {
                            self.output.should_show_neighbors = !self.output.should_show_neighbors;
                        }
                    });
                    ui.text_edit_singleline(&mut self.output.filter_text);
                    egui::ScrollArea::vertical()
                        .max_height(200.)
                        .show(ui, |ui| {
                            for selected_index in 0..self.output.selected_items.len() {
                                let button_text =
                                    format!("{}", self.output.selected_items[selected_index]);
                                if ui.button(get_button_text(&button_text, true)).clicked() {
                                    let selected_item = &self.output.selected_items[selected_index];
                                    index_to_remove = Some(
                                        self.output
                                            .selected_items
                                            .iter()
                                            .position(|item| item.index == selected_item.index)
                                            .expect("Selected item was clicked"),
                                    );
                                    self.output.selected_items_updated = true;
                                }
                            }
                        });
                    ui.separator();
                    for item_index in (0..input.items.len())
                        .filter(|&item_index| {
                            (0..self.output.selected_items.len())
                                .find(|&selected_index| {
                                    self.output.selected_items[selected_index].index
                                        == item_index as u32
                                })
                                .is_none()
                                && (item_index.to_string().starts_with(&self.output.filter_text)
                                    || input.items[item_index]
                                        .text
                                        .contains(&self.output.filter_text))
                        })
                        .take(20)
                    {
                        let button_text = format!("{}", &input.items[item_index]);
                        let button = ui.button(button_text.clone());
                        let clicking_selected_item = self
                            .output
                            .selected_items
                            .iter()
                            .find(|selected_item| selected_item.index == item_index as u32)
                            .is_some();
                        if button.clicked() {
                            if clicking_selected_item {
                                index_to_remove = Some(item_index);
                            } else if !internals.input_state.input.modifiers.shift {
                                should_clear = true;
                                index_to_push = Some(item_index);
                            } else {
                                index_to_push = Some(item_index);
                            }
                            self.output.selected_items_updated = true;
                        }
                    }

                    if should_clear {
                        self.output.selected_items.clear();
                        should_clear = false;
                    }

                    if let Some(index) = index_to_push {
                        self.output.selected_items.push(input.items[index].clone());
                        index_to_push = None;
                    }

                    if let Some(index) = index_to_remove {
                        self.output.selected_items.remove(index);
                        index_to_remove = None;
                    }
                });
            });
    }
}
