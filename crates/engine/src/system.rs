use crate::{
    prelude::{AssetRegistry, Scene},
    time::TimeManager,
};

/// Represents a program that will be run by the `RenderLoop`.
pub trait System {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry);
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry, time: &TimeManager);
}

/// Represents a program that can be paused.
pub trait Pausable {
    fn pause(&mut self);
    fn unpause(&mut self);
    fn is_paused(&self) -> bool;
}
