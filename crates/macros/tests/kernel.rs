use macros::aggregated_kernel;

struct Scene;

trait Kernel {
    unsafe fn setup(&mut self);
    unsafe fn update(&mut self, scene: &Scene);
}

struct Algo;
struct Mas;

#[aggregated_kernel]
enum AggregatedKernel {
    Algo,
    Mas,
}
