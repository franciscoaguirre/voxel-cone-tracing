use engine::ui::prelude::*;

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

mod save_preset;
pub use save_preset::{SavePresetMenu, SavePresetMenuInput};

mod camera;
pub use camera::{CameraMenu, CameraMenuOutput};

mod cone_tracing;
pub use cone_tracing::{ConeTracingMenu, ConeTracingMenuOutput};

mod debug_cone_tracing;
pub use debug_cone_tracing::{DebugConeMenu, DebugConeMenuOutput};

mod helpers;
use helpers::cone_parameters_inputs;

use serde::{Deserialize, Serialize};

pub trait SubMenu: std::fmt::Debug + Default + for<'a> Deserialize<'a> + Serialize + Clone {
    type InputData<'a>: 'a;
    type OutputData: std::fmt::Debug + Default + for<'a> Deserialize<'a> + Serialize + Clone;

    fn is_showing(&self) -> bool;
    fn toggle_showing(&mut self);
    fn get_data(&self) -> &Self::OutputData;
    fn render<'a>(&mut self, context: &egui::Context, input: &Self::InputData<'a>);
}
