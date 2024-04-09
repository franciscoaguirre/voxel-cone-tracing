use macros::aggregated_kernel;

struct Scene;
struct AssetRegistry;

trait Kernel {
    unsafe fn setup(&mut self, assets: &mut AssetRegistry);
    unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry);
}

struct Algo;
impl Kernel for Algo {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, _scene: &Scene, _assets: &AssetRegistry) {}
}

struct Mas;
impl Kernel for Mas {
    unsafe fn setup(&mut self, _assets: &mut AssetRegistry) {}
    unsafe fn update(&mut self, _scene: &Scene, _assets: &AssetRegistry) {}
}

#[aggregated_kernel]
enum AggregatedKernel {
    Algo,
    Mas,
}
