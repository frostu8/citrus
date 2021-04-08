use super::*;

use na::Matrix4;

/// A draw command.
pub struct DrawCommand<A>
where A: AttributeLink {
    gl: WebGl,
    attribute: A,
}

impl<A> DrawCommand<A>
where A: AttributeLink {
    /// Attributes an array of Vector2s.
    pub fn attribute_vec2(
        self, 
        data: &GLBuffer, 
        attrib: u32
    ) -> DrawCommand<AttributeVec2<A>> {
        DrawCommand {
            attribute: AttributeVec2 {
                parent: self.attribute,
                data,
                attrib,
            },
            gl: self.gl,
        }
    }

    /// Sets a uniform using a [`Matrix4`].
    pub fn uniform_mat4<'a, 'b>(
        self,
        matrix: &'a Matrix4<f32>,
        uniform: &'b GLUniformLocation,
    ) -> DrawCommand<Matrix4Bind<'a, 'b, A>> {
        DrawCommand {
            attribute: Matrix4Bind {
                parent: self.attribute,
                matrix,
                uniform,
            },
            gl: self.gl,
        }
    }

    /// Binds a texture to a uniform at slot 0.
    pub fn bind_texture0<'a, 'b>(
        self,
        texture: &'a GLTexture,
        uniform: &'b GLUniformLocation,
    ) -> DrawCommand<TextureBind<'a, 'b, A>> {
        self.bind_texture(texture, uniform, 0)
    }

    /// Binds a texture to a uniform at a slot.
    pub fn bind_texture<'a, 'b>(
        self,
        texture: &'a GLTexture,
        uniform: &'b GLUniformLocation,
        slot: usize,
    ) -> DrawCommand<TextureBind<'a, 'b, A>> {
        DrawCommand {
            attribute: TextureBind {
                parent: self.attribute,
                texture,
                uniform,
                slot,
            },
            gl: self.gl,
        }
    }

    /// Executes the command.
    pub fn draw_triangle_strip(&mut self, count: usize) {
        // execute attributes
        self.attribute.attribute(&self.gl);

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, count as i32);
    }

    pub fn new(gl: WebGl) -> DrawCommand<()> {
        DrawCommand {
            gl,
            attribute: (),
        }
    }
}

pub trait AttributeLink {
    fn attribute(&self, gl: &WebGl);
}

impl AttributeLink for () {
    fn attribute(&self, _gl: &WebGl) { }
}

/// Attributes a Vector2 array to an attribute.
pub struct AttributeVec2<'a, P> 
where P: AttributeLink {
    parent: P,
    data: &'a GLBuffer,
    attrib: u32,
}

impl<'a, P> AttributeLink for AttributeVec2<'a, P> 
where P: AttributeLink {
    fn attribute(&self, gl: &WebGl) {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.data));
        gl.vertex_attrib_pointer_with_i32(self.attrib, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.attrib);

        // call next in chain
        self.parent.attribute(gl);
    }
}

/// Binds a texture to a texture slot.
pub struct TextureBind<'a, 'b, P>
where P: AttributeLink {
    parent: P,
    texture: &'a GLTexture,
    uniform: &'b GLUniformLocation,
    slot: usize,
}

impl<'a, 'b, P> AttributeLink for TextureBind<'a, 'b, P> 
where P: AttributeLink {
    fn attribute(&self, gl: &WebGl) {
        debug_assert!(self.slot < 32);

        gl.active_texture(self.slot as u32 + GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(self.texture));
        gl.uniform1i(Some(self.uniform), 0);

        // call next in chain
        self.parent.attribute(gl);
    }
}

/// Binds a [`Matrix4`] to a uniform.
pub struct Matrix4Bind<'a, 'b, P>
where P: AttributeLink {
    parent: P,
    matrix: &'a Matrix4<f32>,
    uniform: &'b GLUniformLocation,
}

impl<'a, 'b, P> AttributeLink for Matrix4Bind<'a, 'b, P>
where P: AttributeLink {
    fn attribute(&self, gl: &WebGl) {
        gl.uniform_matrix4fv_with_f32_array(
            Some(self.uniform),
            false,
            self.matrix.as_slice(),
        );

        // call next in chain
        self.parent.attribute(gl);
    }
}
