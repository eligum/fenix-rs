//! This module provides a simple interface to load an image to the GPU.

use image::error::ImageError;
use std::{cmp::PartialEq, ffi::c_void, ops::Drop};

#[allow(dead_code)]
pub struct Texture2D {
    path: String, // This field is for debugging purposes
    id: u32,
    width: u32,
    height: u32,
    format: u32,
    internal_format: u32,
}

impl Texture2D {
    /// Loads an image from memory to the GPU as an OpenGL texture.
    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        let image = image::open(path)?;

        let format;
        let internal_format;

        match image {
            image::DynamicImage::ImageRgb8(_) => {
                format = gl::RGB;
                internal_format = gl::RGB8;
            }
            image::DynamicImage::ImageRgba8(_) => {
                format = gl::RGBA;
                internal_format = gl::RGBA8;
            }
            _ => panic!("Image {} has an unsupported color format!", path),
        }

        let width = image.width();
        let height = image.height();

        let mut id = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, internal_format, width as i32, height as i32);

            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TextureSubImage2D(
                id,
                0,
                0,
                0,
                width as i32,
                height as i32,
                format,
                gl::UNSIGNED_BYTE,
                image.flipv().as_bytes().as_ptr() as *const c_void,
            );
        }

        Ok(Self {
            path: path.to_string(),
            id,
            width,
            height,
            format,
            internal_format,
        })
    }

    /// Allocates memory in the GPU to store a texture of the specified dimensions.
    pub fn with_size(width: u32, height: u32) -> Self {
        let format = gl::RGBA;
        let internal_format = gl::RGBA8;

        let mut id = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, internal_format, width as i32, height as i32);

            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }

        Self {
            path: "no path".to_string(),
            id,
            width,
            height,
            format,
            internal_format,
        }
    }

    /// Fills the memory region of the texture with the given data overwriting any
    /// previously stored information.
    pub unsafe fn overwrite(&mut self, data: &[u8]) {
        // Bytes per pixel
        let bpp = if self.format == gl::RGBA { 4 } else { 3 };
        assert_eq!(
            data.len(),
            (self.width * self.height * bpp) as usize,
            "Data does not have the same size as the texture!"
        );

        gl::TextureSubImage2D(
            self.id,
            0,
            0,
            0,
            self.width as i32,
            self.height as i32,
            self.format,
            gl::UNSIGNED_BYTE,
            data.as_ptr().cast(),
        );
    }

    /// Binds the texture to the specified texture unit.
    pub fn bind(&self, slot: u32) {
        unsafe { gl::BindTextureUnit(slot, self.id) };
    }

    /// Retruns the width of the texture.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the texture.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Returns the internal id of the texture used by OpenGL.
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id) };
    }
}

impl PartialEq for Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
