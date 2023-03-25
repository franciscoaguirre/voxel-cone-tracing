use cgmath::Matrix4;

/// Has a gizmo that can be rendered on the screen
pub trait RenderGizmo {
    /// Draw the gizmo this frame
    unsafe fn draw_gizmo(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    );
}
