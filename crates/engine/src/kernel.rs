use crate::prelude::Scene;

pub trait Kernel {
    unsafe fn setup(&mut self);
    unsafe fn update(&mut self, scene: &Scene);
}
