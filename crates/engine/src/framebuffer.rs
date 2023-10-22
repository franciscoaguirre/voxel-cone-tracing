//! This file breaks separation of concerns because both `GEOMETRY_BUFFERS` and
//! `LIGHT_MAP_BUFFERS` are tightly coupled to `core`'s concerns.
//! TODO: To make it better, we could have only one `new` function that receives
//! all the textures already and just assembles the framebuffer.

use std::mem::MaybeUninit;

use gl::types::*;
use image::{ImageBuffer, Rgba, RgbaImage, GenericImageView, Pixel};

use super::{common, types::*};

pub struct Framebuffer<const N: usize> {
    fbo: GLuint,
    attachments: [ColorAttachment; N],
}

#[derive(Debug)]
pub struct ColorAttachment {
    name: String,
    texture_id: GLuint,
    width: i32,
    height: i32,
    format: GLenum,
}

/// Number of geometry buffers
pub const GEOMETRY_BUFFERS: usize = 5;
/// Number of light map buffers
pub const LIGHT_MAP_BUFFERS: usize = 3;

pub type GeometryFramebuffer = Framebuffer<GEOMETRY_BUFFERS>;
pub type LightFramebuffer = Framebuffer<LIGHT_MAP_BUFFERS>;

/// Implementation of framebuffer with only 1 output buffer.
/// Meant to be used for easily visualizing and saving any rendering artifact.
impl Framebuffer<1> {
    pub unsafe fn new() -> Self {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut texture = 0;
        gl::GenTextures(1, &mut texture);

        let (width, height) = common::get_framebuffer_size();

        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width,
            height,
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
            width as i32,
            height as i32,
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
            texture,
            0,
        );

        gl::DrawBuffers(1, [gl::COLOR_ATTACHMENT0].as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        let attachments = [ColorAttachment {
            name: "color_output".to_string(),
            texture_id: texture,
            width,
            height,
            format: gl::RGBA,
        }];

        Self { fbo, attachments }
    }
}

/// Implementation of framebuffer with [`GEOMETRY_BUFFERS`] output buffers.
/// Used for geometry buffers.
/// The buffers hold the following:
/// - Positions: rgb10_a2ui
/// - Viewing positions: rgba8
/// - Normals: rgb32f
/// - Colors: rgba8
/// - Specular: rgba8
impl Framebuffer<GEOMETRY_BUFFERS> {
    pub unsafe fn new() -> Self {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut textures = [0; GEOMETRY_BUFFERS];
        gl::GenTextures(5, textures.as_mut_ptr());

        let mut attachments = Vec::with_capacity(GEOMETRY_BUFFERS);

        let (width, height) = common::get_framebuffer_size();

        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB32F as i32, // Why isn't this rgb10_a2ui
            width,
            height,
            0,
            gl::RGB,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "positions".to_string(),
            texture_id: textures[0],
            width,
            height,
            format: gl::RGB,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "normalized_positions".to_string(),
            texture_id: textures[1],
            width,
            height,
            format: gl::RGBA,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB32F as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "normals".to_string(),
            texture_id: textures[2],
            width,
            height,
            format: gl::RGB,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[3]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "colors".to_string(),
            texture_id: textures[3],
            width,
            height,
            format: gl::RGBA,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[4]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "specular".to_string(),
            texture_id: textures[4],
            width,
            height,
            format: gl::RGBA,
        });

        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            width as i32,
            height as i32,
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
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT4,
            gl::TEXTURE_2D,
            textures[4],
            0,
        );

        gl::DrawBuffers(
            5,
            [
                gl::COLOR_ATTACHMENT0,
                gl::COLOR_ATTACHMENT1,
                gl::COLOR_ATTACHMENT2,
                gl::COLOR_ATTACHMENT3,
                gl::COLOR_ATTACHMENT4,
            ]
            .as_ptr(),
        );

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::FRAMEBUFFER: Framebuffer is not complete!");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        Self { fbo, attachments: attachments.try_into().expect("Too many attachments") }
    }
}

/// Implementation of framebuffer with three output buffers.
/// Used for light maps.
/// The buffers hold the following:
/// - Positions: rgb10_a2ui
/// - Viewing positions: rgba8
/// - Depth
impl Framebuffer<LIGHT_MAP_BUFFERS> {
    pub unsafe fn new() -> Self {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut textures = [0; LIGHT_MAP_BUFFERS]; // First one is rgb10_a2ui, second rgba8 for viewing, third for depth (shadow mapping)
        gl::GenTextures(3, textures.as_mut_ptr());

        let mut attachments = Vec::with_capacity(LIGHT_MAP_BUFFERS);

        let (width, height) = common::get_framebuffer_size();

        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB10_A2UI as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA_INTEGER,
            gl::UNSIGNED_INT_2_10_10_10_REV,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "positions".to_string(),
            texture_id: textures[0],
            width,
            height,
            format: gl::RGBA_INTEGER,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        attachments.push(ColorAttachment {
            name: "normalized_positions".to_string(),
            texture_id: textures[1],
            width,
            height,
            format: gl::RGBA,
        });

        gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            width as i32,
            height as i32,
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
        // TODO: Deal with this better
        attachments.push(ColorAttachment {
            name: "depth".to_string(),
            texture_id: textures[2],
            width,
            height,
            format: gl::DEPTH_COMPONENT,
        });

        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            textures[2],
            0,
        );

        // let (width, height) = common::get_framebuffer_size();
        // let mut rbo = 0;
        // gl::GenRenderbuffers(1, &mut rbo);
        // gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        // gl::RenderbufferStorage(
        //     gl::RENDERBUFFER,
        //     gl::DEPTH24_STENCIL8,
        //     width as i32,
        //     height as i32,
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
            attachments: attachments.try_into().expect("too many attachments"),
        }
    }
}

impl<const N: usize> Framebuffer<N> {
    pub fn fbo(&self) -> GLuint {
        self.fbo
    }

    pub fn textures(&self) -> [Texture2D; N] {
        let mut result: [Texture2D; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (index, attachment) in self.attachments.iter().enumerate() {
            result[index] = attachment.texture_id;
        }
        result
    }

    /// Gets the image from the attachment
    fn get_image_from_attachment(&self, attachment_index: usize) -> RgbaImage {
        let attachment = self.attachments.get(attachment_index).expect("Invalid attachment index");
        let pixels = unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo());
            gl::ReadBuffer(gl::COLOR_ATTACHMENT0 + attachment_index as u32);
            let size = match attachment.format {
                gl::RGB => attachment.width * attachment.height * 3,
                gl::RGBA => attachment.width * attachment.height * 4,
                _ => panic!("Unsupported format"),
            };
            let mut pixels = vec![0u8; size as usize];
            gl::ReadPixels(
                0, 0,
                attachment.width,
                attachment.height,
                attachment.format,
                gl::UNSIGNED_BYTE,
                pixels.as_mut_ptr() as *mut GLvoid,
            );
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            pixels
        };
        let width = attachment.width as u32;
        let height = attachment.height as u32;
        // Image requires flipping because of OpenGL's coordinate system
        let image: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
            let original_y = height - y - 1;
            let base = (original_y * width + x) as usize * 4; // 4 because of RGBA
            Rgba([pixels[base], pixels[base + 1], pixels[base + 2], pixels[base + 3]])
        });
        image
    }

    /// Saves the color attachment with index `attachment_index` of the current framebuffer
    /// to an image with the name `filename`
    pub fn save_color_attachment_to_file(&self, attachment_index: usize, filepath: &str) {
        let image = self.get_image_from_attachment(attachment_index);
        image.save(filepath).expect("Failed to save the image");
    }

    /// Compares the texture in the `attachment_index` attachment of this framebuffer
    /// to an image on disk with path `file_to_compare`.
    /// Returns whether the images were deemed the same.
    pub fn compare_attachment_to_file(&self, attachment_index: usize, file_to_compare: &str) -> Result<bool, ()> {
        let image = self.get_image_from_attachment(attachment_index);
        let image_to_compare = image::open(file_to_compare)
            .map_err(|_| ())?; // We know this error has something to do with creating the file. We'll handle it outside.

        // Ensure the two images have the same dimensions
        let (width_1, height_1) = image.dimensions();
        let (width_2, height_2) = image_to_compare.dimensions();
        if width_1 != width_2 || height_1 != height_2 {
            return Ok(false);
        }

        let tolerance = 0.01f32; // 1 percent
        let channel_tolerance = (tolerance * 255.0f32).round() as u8;

        for y in 0..height_1 {
            for x in 0..width_1 {
                let pixel_1 = image.get_pixel(x, y).to_rgb();
                let pixel_2 = image_to_compare.get_pixel(x, y).to_rgb();
                if (
                    (pixel_1[0] as i16 - pixel_2[0] as i16).abs() > channel_tolerance as i16
                    || (pixel_1[1] as i16 - pixel_2[1] as i16).abs() > channel_tolerance as i16
                    || (pixel_1[2] as i16 - pixel_2[2] as i16).abs() > channel_tolerance as i16
                ) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}
