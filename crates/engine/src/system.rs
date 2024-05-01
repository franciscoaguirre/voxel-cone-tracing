use crate::{
    prelude::{AssetRegistry, Scene},
    time::TimeManager,
};

#[derive(Debug)]
pub struct SystemInfo {
    pub name: &'static str,
}

#[derive(Clone, Copy)]
pub struct SystemInputs<'a> {
    pub scene: &'a Scene,
    pub assets: &'a AssetRegistry,
    pub time: &'a TimeManager,
}

/// Represents a program that will be run by the `RenderLoop`.
pub trait System {
    /// Runs once at the start of the RenderLoop.
    /// Meant to be used for registering textures and uniforms.
    unsafe fn setup(&mut self, assets: &mut AssetRegistry);

    /// Runs every frame.
    /// Meant to render, perform calculations, anything really.
    unsafe fn update(&mut self, inputs: SystemInputs);

    /// Runs every frame after `update`.
    /// Meant to mutate the asset registry with results from the
    /// `update` stage.
    unsafe fn post_update(&mut self, _assets: &mut AssetRegistry) {}

    /// Returns some information about the system.
    /// Displayed in UIs.
    fn get_info(&self) -> SystemInfo;
}

/// Represents a system that can be paused.
pub trait Pausable {
    /// Pause it.
    fn pause(&mut self);
    /// Unpause it.
    fn unpause(&mut self);
    /// Is it paused right now?
    fn is_paused(&self) -> bool;
    /// Return a mutable reference to the paused value, useful for
    /// UI manipulating it.
    fn is_paused_mut(&mut self) -> &mut bool;
}
