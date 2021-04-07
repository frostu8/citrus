//! Shader types.

pub mod canvas;

use super::*;

pub use canvas::CanvasShader;

/// A linkable shader.
///
/// Better associates shaders with contexts.
pub trait Shader
where Self: Sized {
    /// Links a program together.
    fn link(gl: GL, width: u32, height: u32) -> Result<Self, GlError>;
}

impl GL {
    /// Links a shader together.
    pub fn shader<T>(&self) -> Result<T, GlError> 
    where T: Shader {
        T::link(self.clone(), self.1.client_width() as u32, self.1.client_height() as u32)
    }
}
