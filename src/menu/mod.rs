use std::fmt::Display;

use egui_backend::{
    egui::{self, vec2, Color32, Pos2, Rect},
    glfw::{Action, CursorMode, Key, Window, WindowEvent},
};
use egui_glfw_gl as egui_backend;

use crate::{
    config::CONFIG,
    octree::{BrickAttribute, BricksToShow},
};

pub struct Menu {
    is_showing: bool,
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
    is_showing_all_nodes_window: bool,
    is_showing_node_search_window: bool,
    is_showing_bricks_window: bool,
    is_showing_photons_window: bool,
    is_showing_children_window: bool,
    is_showing_images_window: bool,
}

impl Menu {
    pub fn new(window: &mut Window) -> Self {
        Self::setup_egui(window)
    }

    pub fn toggle_showing(&mut self, window: &mut Window, last_x: &mut f32, last_y: &mut f32) {
        self.is_showing = !self.is_showing;

        if self.is_showing {
            window.set_cursor_mode(CursorMode::Normal);
        } else {
            window.set_cursor_mode(CursorMode::Disabled);

            // So that we don't take into account mouse movements while using the menu
            let cursor_position = window.get_cursor_pos();
            *last_x = cursor_position.0 as f32;
            *last_y = cursor_position.1 as f32;
        };
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn is_showing_all_nodes_window(&self) -> bool {
        self.is_showing_all_nodes_window
    }

    pub fn is_showing_node_search_window(&self) -> bool {
        self.is_showing_node_search_window
    }

    pub fn is_showing_bricks_window(&self) -> bool {
        self.is_showing_bricks_window
    }

    pub fn is_showing_photons_window(&self) -> bool {
        self.is_showing_photons_window
    }

    pub fn is_showing_children_window(&self) -> bool {
        self.is_showing_children_window
    }

    pub fn is_showing_images_window(&self) -> bool {
        self.is_showing_images_window
    }

    pub fn handle_event(&mut self, event: WindowEvent) {
        if !self.is_showing {
            return;
        }

        if let WindowEvent::Key(Key::LeftShift, _, Action::Press, _) = event {
            self.modifier_keys.shift = true;
        } else if let WindowEvent::Key(Key::LeftShift, _, Action::Release, _) = event {
            self.modifier_keys.shift = false;
        }

        egui_backend::handle_event(event, &mut self.input_state);
    }

    pub fn begin_frame(&mut self, current_frame: f64) {
        if !self.is_showing {
            return;
        }

        self.input_state.input.time = Some(current_frame);
        self.input_state.input.modifiers = self.modifier_keys;
        self.context.begin_frame(self.input_state.input.take());
        self.input_state.input.pixels_per_point = Some(self.native_pixels_per_point);
    }

    pub fn end_frame(&mut self) {
        if !self.is_showing {
            return;
        }

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = self.context.end_frame();
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.input_state, platform_output.copied_text);
        }
        let clipped_shapes = self.context.tessellate(shapes);
        self.painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }

    fn setup_egui(window: &mut Window) -> Menu {
        let painter = egui_backend::Painter::new(window);
        let context = egui::Context::default();

        let native_pixels_per_point = window.get_content_scale().0;

        let input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0_f32, 0_f32),
                vec2(CONFIG.viewport_width as f32, CONFIG.viewport_height as f32)
                    / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });

        let modifier_keys = egui::Modifiers::default();

        Self {
            is_showing: false,
            is_showing_all_nodes_window: false,
            is_showing_node_search_window: false,
            is_showing_bricks_window: false,
            is_showing_photons_window: false,
            is_showing_children_window: false,
            is_showing_images_window: false,
            painter,
            context,
            input_state,
            modifier_keys,
            native_pixels_per_point,
        }
    }

    pub fn show_main_window(&mut self) {
        egui::Window::new("Menu").show(&self.context, |ui| {
            if ui
                .button(Self::get_button_text(
                    "All nodes",
                    self.is_showing_all_nodes_window,
                ))
                .clicked()
            {
                self.is_showing_all_nodes_window = !self.is_showing_all_nodes_window;
            }
            if ui
                .button(Self::get_button_text(
                    "Node search",
                    self.is_showing_node_search_window,
                ))
                .clicked()
            {
                self.is_showing_node_search_window = !self.is_showing_node_search_window;
            }
            if ui
                .button(Self::get_button_text(
                    "Bricks",
                    self.is_showing_bricks_window,
                ))
                .clicked()
            {
                self.is_showing_bricks_window = !self.is_showing_bricks_window;
            }
            if ui
                .button(Self::get_button_text(
                    "Photons",
                    self.is_showing_photons_window,
                ))
                .clicked()
            {
                self.is_showing_photons_window = !self.is_showing_photons_window;
            }
            if ui
                .button(Self::get_button_text(
                    "Children",
                    self.is_showing_children_window,
                ))
                .clicked()
            {
                self.is_showing_children_window = !self.is_showing_children_window;
            }
            if ui
                .button(Self::get_button_text(
                    "Images",
                    self.is_showing_images_window,
                ))
                .clicked()
            {
                self.is_showing_images_window = !self.is_showing_images_window;
            }
        });
    }

    pub fn create_diagnostics_window(&self, fps: f64) {
        egui::Window::new("Diagnostics").show(&self.context, |ui| {
            let fps_text = format!("FPS: {fps:.2}");
            ui.label(fps_text);
        });
    }

    pub fn create_photons_window(&self, photons: &[u32]) {
        egui::Window::new("Photons").show(&self.context, |ui| {
            if photons.is_empty() {
                ui.label("No photon data. Pick a node!");
                return;
            }

            ui.vertical(|ui| {
                for (index, photon) in photons.iter().enumerate() {
                    let x = index % 3;
                    let y = (index / 3) % 3;
                    let z = index / (3 * 3);
                    let label_text = format!("({x}, {y}, {z}): {photon}");
                    ui.label(label_text);
                }
            });
        });
    }

    pub fn create_children_window(&self, children: &[u32]) {
        egui::Window::new("Children").show(&self.context, |ui| {
            if children.is_empty() {
                ui.label("No children data. Pick a node!");
                return;
            }

            ui.vertical(|ui| {
                for child in children.iter() {
                    ui.label(child.to_string());
                }
            });
        });
    }

    pub fn create_images_window(
        &self,
        should_show_color: &mut bool,
        should_show_direct: &mut bool,
        should_show_indirect: &mut bool,
        should_show_indirect_specular: &mut bool,
        should_show_ambient_occlusion: &mut bool,
    ) {
        egui::Window::new("Images").show(&self.context, |ui| {
            if ui
                .button(Self::get_button_text("Color", *should_show_color))
                .clicked()
            {
                *should_show_color = !*should_show_color;
            }
            if ui
                .button(Self::get_button_text("Direct light", *should_show_direct))
                .clicked()
            {
                *should_show_direct = !*should_show_direct;
            }
            if ui
                .button(Self::get_button_text(
                    "Indirect diffuse",
                    *should_show_indirect,
                ))
                .clicked()
            {
                *should_show_indirect = !*should_show_indirect;
            }
            if ui
                .button(Self::get_button_text(
                    "Indirect specular",
                    *should_show_indirect_specular,
                ))
                .clicked()
            {
                *should_show_indirect_specular = !*should_show_indirect_specular;
            }
            if ui
                .button(Self::get_button_text(
                    "Ambient occlusion",
                    *should_show_ambient_occlusion,
                ))
                .clicked()
            {
                *should_show_ambient_occlusion = !*should_show_ambient_occlusion;
            }
        });
    }

    fn get_button_text(text: &str, clicked: bool) -> egui::RichText {
        let mut button_text = egui::RichText::new(text);
        if clicked {
            button_text = button_text.color(Color32::RED);
        }
        button_text
    }

    pub fn create_all_nodes_window(
        &self,
        should_render_octree: &mut bool,
        current_octree_level: &mut u32,
    ) {
        egui::Window::new("All nodes").show(&self.context, |ui| {
            if ui
                .button(Self::get_button_text("Show octree", *should_render_octree))
                .clicked()
            {
                *should_render_octree = !*should_render_octree;
            }
            ui.add(
                egui::Slider::new(current_octree_level, 0..=CONFIG.octree_levels - 1)
                    .text("Octree level"),
            );
        });
    }

    pub fn create_node_search_window(
        &self,
        items: &Vec<DebugNode>,
        selected_items: &mut Vec<DebugNode>,
        filter_text: &mut String,
        should_show_neighbors: &mut bool,
        selected_items_updated: &mut bool,
    ) {
        // Variables for handling modifications to `selected_items`
        let mut should_clear = false;
        let mut index_to_push = None;
        let mut index_to_remove = None;

        egui::Window::new("Node search")
            .resize(|r| r.fixed_size((200., 400.)))
            .show(&self.context, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neighbors: ");
                        if ui
                            .button(Self::get_button_text("Toggle", *should_show_neighbors))
                            .clicked()
                        {
                            *should_show_neighbors = !*should_show_neighbors;
                        }
                    });
                    ui.text_edit_singleline(filter_text);
                    egui::ScrollArea::vertical()
                        .max_height(200.)
                        .show(ui, |ui| {
                            for selected_index in 0..selected_items.len() {
                                let button_text = format!("{}", selected_items[selected_index]);
                                if ui
                                    .button(Self::get_button_text(&button_text, true))
                                    .clicked()
                                {
                                    let selected_item = &selected_items[selected_index];
                                    index_to_remove = Some(
                                        selected_items
                                            .iter()
                                            .position(|item| item.index == selected_item.index)
                                            .expect("Selected item was clicked"),
                                    );
                                    *selected_items_updated = true;
                                }
                            }
                        });
                    ui.separator();
                    for item_index in (0..items.len())
                        .filter(|&item_index| {
                            (0..selected_items.len())
                                .find(|&selected_index| {
                                    selected_items[selected_index].index == item_index as u32
                                })
                                .is_none()
                                && (item_index.to_string().starts_with(&*filter_text)
                                    || items[item_index].text.contains(&*filter_text))
                        })
                        .take(20)
                    {
                        let button_text = format!("{}", &items[item_index]);
                        let button = ui.button(button_text.clone());
                        let clicking_selected_item = selected_items
                            .iter()
                            .find(|selected_item| selected_item.index == item_index as u32)
                            .is_some();
                        if button.clicked() {
                            if clicking_selected_item {
                                index_to_remove = Some(item_index);
                            } else if !self.input_state.input.modifiers.shift {
                                should_clear = true;
                                index_to_push = Some(item_index);
                            } else {
                                index_to_push = Some(item_index);
                            }
                            *selected_items_updated = true;
                        }
                    }

                    if should_clear {
                        selected_items.clear();
                        should_clear = false;
                    }

                    if let Some(index) = index_to_push {
                        selected_items.push(items[index].clone());
                        index_to_push = None;
                    }

                    if let Some(index) = index_to_remove {
                        selected_items.remove(index);
                        index_to_remove = None;
                    }
                });
            });
    }

    pub fn create_bricks_window(
        &self,
        bricks_to_show: &mut BricksToShow,
        brick_attribute: &mut BrickAttribute,
        should_show_brick_normals: &mut bool,
        color_direction: &mut u32,
        brick_padding: &mut f32,
    ) {
        egui::Window::new("Bricks").show(&self.context, |ui| {
            ui.horizontal(|ui| {
                ui.label("Bricks: ");
                if ui
                    .button(Self::get_button_text("Z0", bricks_to_show.z0()))
                    .clicked()
                {
                    bricks_to_show.toggle_z0();
                }
                if ui
                    .button(Self::get_button_text("Z1", bricks_to_show.z1()))
                    .clicked()
                {
                    bricks_to_show.toggle_z1();
                }
                if ui
                    .button(Self::get_button_text("Z2", bricks_to_show.z2()))
                    .clicked()
                {
                    bricks_to_show.toggle_z2();
                }
            });
            ui.horizontal(|ui| {
                ui.label("Brick attribute: ");
                let button_text = match *brick_attribute {
                    BrickAttribute::None => "None",
                    BrickAttribute::Color => "Color",
                    BrickAttribute::Photons => "Photons",
                };
                if ui.button(button_text).clicked() {
                    *brick_attribute = brick_attribute.next();
                }
            });
            if ui
                .button(Self::get_button_text(
                    "Show normals",
                    *should_show_brick_normals,
                ))
                .clicked()
            {
                *should_show_brick_normals = !*should_show_brick_normals;
            }
            ui.add(egui::Slider::new(color_direction, 0..=5).text("Color direction"));
            ui.add(egui::Slider::new(brick_padding, 0.0..=1.0).text("Brick padding"));
        });
    }
}

#[derive(Clone, Debug)]
pub struct DebugNode {
    index: u32,
    text: String,
}

impl DebugNode {
    pub fn new(index: u32, text: String) -> Self {
        Self { index, text }
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl Display for DebugNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.index, self.text)
    }
}
