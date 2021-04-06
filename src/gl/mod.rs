//! GL wrapper types.
//!
//! The main purpose of these GL wrapper types is to ensure that GL types are
//! deleted when they are created. Nothing that something like `glium` couldn't
//! do, but there you go.

pub mod color;
pub mod error;

pub use error::*;
pub use color::*;

use wasm_bindgen::JsCast as _;
use web_sys::{
    HtmlImageElement,
    WebGlShader, 
    WebGlProgram, 
    WebGlTexture, 
    WebGlRenderingContext as WebGl,
    WebGlUniformLocation,
};
use na::{Vector2, Matrix3};

use std::ops::Deref;

/// Main GL context.
///
/// Many references can be made to this object thanks to interior mutability,
/// *not that we have a choice in that matter*.
pub struct GL(WebGl);

impl GL {
    /// Creates a new GL context.
    ///
    /// This is explicit; use `GL`'s into implementation.
    pub fn new(context: WebGl) -> GL {
        GL(context)
    }

    /// Gets a copied reference to the inner [`WebGlRenderingContext`] object.
    pub fn clone_ref(&self) -> WebGl {
        // SAFETY: the inner type is `WebGl`, so this will always be `WebGl`.
        self.0.clone().unchecked_into()
    }

    /// Create a texture from an image element.
    ///
    /// # Panics
    /// Panics if `image` is not a complete image.
    pub fn create_texture(&self, image: &HtmlImageElement) -> GLTexture {
        let texture = self.0.create_texture().unwrap();
        let texture = GLTexture(self.clone_ref(), texture);
        self.0.bind_texture(WebGl::TEXTURE_2D, Some(&texture));

        // create texture
        if image.complete() {
            self.0.tex_image_2d_with_u32_and_u32_and_image(
                WebGl::TEXTURE_2D,
                0,
                WebGl::RGBA as i32,
                WebGl::RGBA,
                WebGl::UNSIGNED_BYTE,
                image,
            ).unwrap();

            texture
        } else {
            panic!("image is not complete!");
        }
    }

    /// Create a texture whose contents are a single pixel defined by a
    /// [`Color`].
    pub fn solid_color_texture(&self, color: Color) -> GLTexture {
        let texture = self.0.create_texture().unwrap();
        let texture = GLTexture(self.clone_ref(), texture);
        self.0.bind_texture(WebGl::TEXTURE_2D, Some(&texture));

        // this should always be valid
        self.0.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl::TEXTURE_2D,
            0,
            WebGl::RGBA as i32,
            1,
            1,
            0,
            WebGl::RGBA,
            WebGl::UNSIGNED_BYTE,
            Some(&[color.red(), color.green(), color.blue(), color.alpha()]),
        ).unwrap();

        texture
    }

    /// Sets a uniform at the specified location to some [`Matrix3`].
    pub fn uniform_matrix3(
        &self,
        uniform: &GLUniformLocation,
        mat: &na::Matrix3<f32>,
    ) {
        self.0.uniform_matrix3fv_with_f32_array(
            Some(uniform),
            false,
            mat.as_slice(),
        );
    }

    /// Gets the location of a uniform specified by name.
    pub fn get_uniform_location(
        &self,
        program: &WebGlProgram, 
        name: &str,
    ) -> Result<GLUniformLocation, UniformNotFoundError> {
        self.0.get_uniform_location(program, name)
            .map(|u| Ok(GLUniformLocation(self.clone_ref(), u)))
            .unwrap_or(Err(UniformNotFoundError::new(name.to_string())))
    }

    /// Creates and compiles a vertex shader.
    pub fn compile_vert_shader(&self, src: &str) -> Result<GLShader, ShaderCompileError> {
        self.compile_shader(WebGl::VERTEX_SHADER, src)
    }

    /// Creates and compiles a fragment shader.
    pub fn compile_frag_shader(&self, src: &str) -> Result<GLShader, ShaderCompileError> {
        self.compile_shader(WebGl::FRAGMENT_SHADER, src)
    }

    /// Creates and compiles a shader.
    pub fn compile_shader(
        &self, 
        kind: u32, 
        src: &str,
    ) -> Result<GLShader, ShaderCompileError> {
        // create shader
        let shader = self.0.create_shader(kind).unwrap();
        let shader = GLShader(self.clone_ref(), shader);

        // compile shader
        self.0.shader_source(&shader, src);
        self.0.compile_shader(&shader);

        // check shader compilation issues
        if self.0.get_shader_parameter(&shader, WebGl::COMPILE_STATUS).as_bool().unwrap() {
            Ok(shader)
        } else {
            let err = self.0.get_shader_info_log(&shader).unwrap();
            Err(ShaderCompileError::new(err))
        }
    }

    /// Links a program together.
    pub fn link_program(
        &self,
        vert: GLShader, 
        frag: GLShader,
    ) -> Result<GLProgram, ProgramLinkError> {
        // create and link program
        let program = self.0.create_program().unwrap();
        let program = GLProgram(self.clone_ref(), program);
        self.0.attach_shader(&program, &vert);
        self.0.attach_shader(&program, &frag);
        self.0.link_program(&program);

        // check for link errors
        if self.0.get_program_parameter(&program, WebGl::LINK_STATUS).as_bool().unwrap() {
            Ok(program)
        } else {
            let err = self.0.get_program_info_log(&program).unwrap();
            Err(ProgramLinkError::new(err))
        }
    }
}

impl From<WebGl> for GL {
    fn from(inner: WebGl) -> GL {
        GL(inner)
    }
}

impl From<&WebGl> for GL {
    fn from(inner_ref: &WebGl) -> GL {
        GL(inner_ref.clone().unchecked_into())
    }
}

impl Deref for GL {
    type Target = WebGl;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Shader handle.
pub struct GLShader(WebGl, WebGlShader);

impl Drop for GLShader {
    fn drop(&mut self) {
        self.0.delete_shader(Some(&self.1));
    }
}

impl Deref for GLShader  {
    type Target = WebGlShader;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

/// Texture handle.
pub struct GLTexture(WebGl, WebGlTexture);

impl Drop for GLTexture {
    fn drop(&mut self) {
        self.0.delete_texture(Some(&self.1));
    }
}

impl Deref for GLTexture {
    type Target = WebGlTexture;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

/// Program handle.
pub struct GLProgram(WebGl, WebGlProgram);

impl Drop for GLProgram {
    fn drop(&mut self) {
        self.0.delete_program(Some(&self.1));
    }
}

impl Deref for GLProgram {
    type Target = WebGlProgram;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

/// Uniform location.
///
/// This doesn't actually require any special cleanup, but for consistency's
/// sake, the type is here.
pub struct GLUniformLocation(WebGl, WebGlUniformLocation);

impl Deref for GLUniformLocation {
    type Target = WebGlUniformLocation;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

// Here is some constant implementations
macro_rules! reexport {
    ($const:ident) => {
        pub const $const: u32 = WebGl::$const;
    }
}

impl GL {
    reexport!(TEXTURE0);
    reexport!(TEXTURE_2D);
    reexport!(ARRAY_BUFFER);
    reexport!(FLOAT);
    reexport!(STATIC_DRAW);
    reexport!(TRIANGLE_STRIP);
}
