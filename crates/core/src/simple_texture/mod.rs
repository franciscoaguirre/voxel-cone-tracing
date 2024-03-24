mod voxelizer;
pub use voxelizer::{Voxelizer, VoxelizerRunInputs};

mod visualizer;
pub use visualizer::{Visualizer, VisualizerRunInputs};

mod cone_tracer;
pub use cone_tracer::{ConeTracer, ConeTracerRunInputs};

pub trait GpuKernel {
    type InitInputs<'a>: 'a;
    type RunInputs<'b>: 'b;

    unsafe fn init<'a>(inputs: Self::InitInputs<'a>) -> Self;
    unsafe fn run<'b>(&self, inputs: Self::RunInputs<'b>);
}
