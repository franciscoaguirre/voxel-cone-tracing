use std::fmt::Display;

use egui_backend::{
    egui::{self, vec2, Color32, Pos2, Rect},
    glfw::{Action, CursorMode, Key, Window, WindowEvent},
};
use egui_glfw_gl as egui_backend;

use crate::{config::CONFIG, octree::BricksToShow};

pub struct Menu {
    is_showing: bool,
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
    is_showing_node_positions_window: bool,
    is_showing_diagnostics_window: bool,
    is_showing_photons_window: bool,
    is_showing_children_window: bool,
    is_showing_images_window: bool,
}

impl Menu {
    pub fn new(window: &mut Window) -> Self {
        Self::setup_egui(window)
    }

    pub fn toggle_showing(&mut self, window: &mut Window) {
        self.is_showing = !self.is_showing;

        if self.is_showing {
            window.set_cursor_mode(CursorMode::Normal)
        } else {
            window.set_cursor_mode(CursorMode::Disabled)
        };
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn is_showing_node_positions_window(&self) -> bool {
        self.is_showing_node_positions_window
    }

    pub fn is_showing_diagnostics_window(&self) -> bool {
        self.is_showing_diagnostics_window
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
            is_showing_node_positions_window: false,
            is_showing_diagnostics_window: false,
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
            if ui.button("Diagnostics").clicked() {
                self.is_showing_diagnostics_window = !self.is_showing_diagnostics_window;
            }
            if ui.button("Node positions").clicked() {
                self.is_showing_node_positions_window = !self.is_showing_node_positions_window;
            }
            if ui.button("Photons").clicked() {
                self.is_showing_photons_window = !self.is_showing_photons_window;
            }
            if ui.button("Children").clicked() {
                self.is_showing_children_window = !self.is_showing_children_window;
            }
            if ui.button("Images").clicked() {
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

    pub fn create_node_positions_window(
        &self,
        items: &Vec<DebugNode>,
        selected_items: &mut Vec<DebugNode>,
        filter_text: &mut String,
        should_show_neighbors: &mut bool,
        bricks_to_show: &mut BricksToShow,
        selected_items_updated: &mut bool,
        color_direction: &mut u32,
        current_octree_level: &mut u32,
    ) {
        let pinned_items: Vec<DebugNode> = selected_items.clone();

        egui::Window::new("Nodes")
            .resize(|r| r.fixed_size((200., 400.)))
            .show(&self.context, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Neighbors: ");
                        if ui.button("Toggle").clicked() {
                            *should_show_neighbors = !*should_show_neighbors;
                        }
                    });
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
                    ui.add(
                        egui::Slider::new(current_octree_level, 0..=CONFIG.octree_levels - 1)
                            .text("Octree level"),
                    );
                    ui.add(egui::Slider::new(color_direction, 0..=5).text("Direction"));
                    ui.text_edit_singleline(filter_text);
                    egui::ScrollArea::vertical()
                        .max_height(200.)
                        .show(ui, |ui| {
                            for pinned_node in pinned_items.iter() {
                                let button_text = format!("{}", pinned_node);
                                if ui
                                    .button(egui::RichText::new(button_text).color(Color32::RED))
                                    .clicked()
                                {
                                    let pinned_index = selected_items
                                        .iter()
                                        .position(|node| node.index == pinned_node.index)
                                        .expect("Pinned item was selected");
                                    *selected_items_updated = true;
                                    selected_items.remove(pinned_index);
                                }
                            }
                        });
                    ui.separator();
                    for node in items
                        .iter()
                        .filter(|node| {
                            pinned_items
                                .iter()
                                .find(|pinned_node| pinned_node.index == node.index as u32)
                                .is_none()
                                && (node.index.to_string().starts_with(&*filter_text)
                                    || node.text.contains(&*filter_text))
                        })
                        .take(20)
                    {
                        let button_text = format!("{}", node);
                        let button = ui.button(button_text.clone());
                        if button.clicked() {
                            if !self.input_state.input.modifiers.shift {
                                selected_items.clear();
                                selected_items.push(node.clone());
                            } else if selected_items
                                .iter()
                                .find(|selected_node| selected_node.index == node.index as u32)
                                .is_some()
                            {
                                selected_items.remove(node.index as usize);
                            } else {
                                selected_items.push(node.clone());
                            }
                            *selected_items_updated = true;
                        }
                    }
                });
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
