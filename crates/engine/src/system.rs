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
    unsafe fn setup(&mut self, assets: &mut AssetRegistry);
    unsafe fn update(&mut self, inputs: SystemInputs);
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
