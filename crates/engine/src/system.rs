use std::ptr::addr_of_mut;

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

pub trait PausableSystem: System + Pausable {}

/// Represents a program that will be run by the `RenderLoop`.
pub trait System {
    /// Runs once at the start of the RenderLoop.
    /// Meant to be used for registering textures and uniforms.
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}

    /// Runs every frame.
    /// Meant to render, perform calculations, anything really.
    unsafe fn update(&mut self, _inputs: SystemInputs) {}

    /// Runs every frame after `update`.
    /// Meant to mutate the asset registry with results from the
    /// `update` stage.
    unsafe fn post_update(&mut self, _assets: &mut AssetRegistry) {}

    /// Returns some information about the system.
    /// Displayed in UIs.
    fn get_info(&self) -> SystemInfo;

    /// Returns subsystems that make up this system.
    /// If empty, this system is not a group.
    fn subsystems(&mut self) -> &mut [Box<dyn PausableSystem>] {
        &mut []
    }
}

impl System for () {
    fn get_info(&self) -> SystemInfo {
        SystemInfo { name: "" }
    }
}

impl Pausable for () {
    fn pause(&mut self) {}
    fn unpause(&mut self) {}
    fn is_paused(&self) -> bool {
        false
    }
    fn is_paused_mut(&mut self) -> &mut bool {
        // Did some fancy stuff to make it compile.
        static mut VALUE: bool = false;
        unsafe { &mut *addr_of_mut!(VALUE) }
    }
    fn pause_next_frame(&self) -> bool {
        false
    }
    fn set_pause_next_frame(&mut self, _: bool) {}
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
    /// Only made to support one-shot systems easily.
    /// These systems will be unpaused one frame and then
    /// set to be paused in the next one, that way they run only once.
    fn pause_next_frame(&self) -> bool;
    /// Setter for pausing on the next frame.
    fn set_pause_next_frame(&mut self, value: bool);
}
