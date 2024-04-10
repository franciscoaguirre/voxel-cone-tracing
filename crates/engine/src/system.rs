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

/// Represents a program that can be paused.
pub trait Pausable {
    fn pause(&mut self);
    fn unpause(&mut self);
    fn is_paused(&self) -> bool;
}
