use cgmath::Point3;
use egui_backend::{
    egui::{self, vec2, Color32, Pos2, Rect},
    glfw::{Action, CursorMode, Key, Window, WindowEvent},
};
use egui_glfw_gl as egui_backend;

pub struct Menu {
    is_showing: bool,
    painter: egui_backend::Painter,
    context: egui::Context,
    input_state: egui_backend::EguiInputState,
    modifier_keys: egui::Modifiers,
    native_pixels_per_point: f32,
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

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0_f32, 0_f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            pixels_per_point: Some(native_pixels_per_point),
            ..Default::default()
        });

        let modifier_keys = egui::Modifiers::default();

        Self {
            is_showing: false,
            painter,
            context,
            input_state,
            modifier_keys,
            native_pixels_per_point,
        }
    }

    pub fn show_points_menu(&self, current_point_raw: &mut String, points: &mut Vec<Point3<f32>>) {
        egui::Window::new("Points").show(&self.context, |ui| {
            ui.text_edit_singleline(current_point_raw);
            if ui.button("Draw point!").clicked() {
                let current_point: Point3<f32> = match current_point_raw
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    [x, y, z] => Point3 {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                        z: z.parse().unwrap(),
                    },
                    _ => panic!(),
                };
                points.push(current_point);
            }
        });
    }

    pub fn create_clickable_list(
        &self,
        items: &Vec<String>,
        selected_items: &mut Vec<(u32, String)>,
        window_title: &str,
        filter_text: &mut String,
    ) {
        let pinned_items: Vec<(u32, String)> = items
            .iter()
            .enumerate()
            .filter(|(item_index, _)| {
                selected_items
                    .iter()
                    .find(|(index, _)| *index == *item_index as u32)
                    .is_some()
            })
            .map(|(index, item)| (index as u32, item.clone()))
            .collect();

        egui::Window::new(window_title)
            .resize(|r| r.fixed_size((200., 400.)))
            .show(&self.context, |ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(filter_text);
                    egui::ScrollArea::vertical()
                        .max_height(200.)
                        .show(ui, |ui| {
                            for (voxel_fragment_index, item) in pinned_items.iter() {
                                let button_text = format!("{}: {}", voxel_fragment_index, item);
                                if ui
                                    .button(egui::RichText::new(button_text).color(Color32::RED))
                                    .clicked()
                                {
                                    let pinned_index = selected_items
                                        .iter()
                                        .position(|(index, _)| *index == *voxel_fragment_index)
                                        .expect("Pinned item was selected");
                                    selected_items.remove(pinned_index);
                                }
                            }
                        });
                    ui.separator();
                    for (item_index, item) in items
                        .iter()
                        .enumerate()
                        .filter(|(index, item)| {
                            pinned_items
                                .iter()
                                .find(|(pinned_index, _)| *pinned_index == *index as u32)
                                .is_none()
                                && (index.to_string().starts_with(&*filter_text)
                                    || item.contains(&*filter_text))
                        })
                        .take(20)
                    {
                        let button_text = format!("{}: {}", item_index, item);
                        let button = ui.button(button_text.clone());
                        if button.clicked() {
                            if !self.input_state.input.modifiers.shift {
                                selected_items.clear();
                                selected_items.push((item_index as u32, item.clone()));
                            } else if selected_items
                                .iter()
                                .find(|(index, _)| *index == item_index as u32)
                                .is_some()
                            {
                                let index = selected_items
                                    .iter()
                                    .position(|(index, _)| *index == item_index as u32)
                                    .expect("voxel_indices should contain index");
                                selected_items.remove(index as usize);
                            } else {
                                selected_items.push((item_index as u32, item.clone()));
                            }
                        }
                    }
                });
            });
    }
}
