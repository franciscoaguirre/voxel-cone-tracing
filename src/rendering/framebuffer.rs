use gl::types::GLuint;

use crate::config::CONFIG;

use super::geometry_buffers::GeometryBuffers;

pub struct Framebuffer {
    fbo: GLuint,
    textures: [GLuint; 4], // TODO: Make generic?
}

impl Framebuffer {
    pub unsafe fn new() -> Self {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut textures = [0; 4]; // First one is rgb10_a2ui, second rgba8 for viewing, third is normals, fourth for colors
        gl::GenTextures(4, textures.as_mut_ptr());

        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB32F as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGB,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB32F as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGB,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[3]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            textures[0],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            textures[1],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT2,
            gl::TEXTURE_2D,
            textures[2],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT3,
            gl::TEXTURE_2D,
            textures[3],
            0,
        );

        gl::DrawBuffers(
            4,
            [
                gl::COLOR_ATTACHMENT0,
                gl::COLOR_ATTACHMENT1,
                gl::COLOR_ATTACHMENT2,
                gl::COLOR_ATTACHMENT3,
            ]
            .as_ptr(),
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        Self { fbo, textures }
    }

    pub unsafe fn new_light() -> Self {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut textures = [0; 3]; // First one is rgb10_a2ui, second rgba8 for viewing, third for depth (shadow mapping)
        gl::GenTextures(3, textures.as_mut_ptr());

        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB10_A2UI as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA_INTEGER,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            CONFIG.viewport_width as i32,
            CONFIG.viewport_height as i32,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_BORDER as i32,
        );
        let border_color = [1.0f32; 4];
        gl::TexParameterfv(
            gl::TEXTURE_2D,
            gl::TEXTURE_BORDER_COLOR,
            border_color.as_ptr(),
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            textures[2],
            0,
        );

        // let mut rbo = 0;
        // gl::GenRenderbuffers(1, &mut rbo);
        // gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        // gl::RenderbufferStorage(
        //     gl::RENDERBUFFER,
        //     gl::DEPTH24_STENCIL8,
        //     CONFIG.viewport_width as i32,
        //     CONFIG.viewport_height as i32,
        // );
        // gl::FramebufferRenderbuffer(
        //     gl::FRAMEBUFFER,
        //     gl::DEPTH_STENCIL_ATTACHMENT,
        //     gl::RENDERBUFFER,
        //     rbo,
        // );
        // gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            textures[0],
            0,
        );
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            textures[1],
            0,
        );

        gl::DrawBuffers(2, [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1].as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        Self {
            fbo,
            textures: [textures[0], textures[1], textures[2], 0],
        }
    }

    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    pub fn textures(&self) -> GeometryBuffers {
        GeometryBuffers::new(
            self.textures[0],
            self.textures[1],
            self.textures[2],
            self.textures[3],
        )
    }
}
