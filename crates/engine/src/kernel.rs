use crate::prelude::{AssetRegistry, Scene};

/// Represents a program that will be run by the `RenderLoop`.
pub trait Kernel {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry);
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry);
}

/// Represents a program that can be paused.
pub trait Pausable {
    fn pause(&mut self);
    fn unpause(&mut self);
    fn is_paused(&self) -> bool;
}
