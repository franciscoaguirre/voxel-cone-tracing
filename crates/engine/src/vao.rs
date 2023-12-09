use gl::types::GLuint;

pub struct Vao(GLuint);

impl Vao {
    /// Create a new, empty VAO
    pub unsafe fn new() -> Self {
        let mut id = 0;
        gl::GenVertexArrays(1, &mut id);
        Self(id)
    }

    /// Draw a certain number of points.
    /// Corresponds to launching a certain number of vertex shader threads.
    /// Can be used with an empty VAO to just use the thread indices.
    pub unsafe fn draw_points(&self, number_of_points: usize) {
        gl::BindVertexArray(self.0);
        gl::DrawArrays(
            gl::POINTS,
            0,
            number_of_points as i32,
        );
    }

    /// Only bind the VAO.
    /// Make sure to unbind later.
    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.0);
    }
}
