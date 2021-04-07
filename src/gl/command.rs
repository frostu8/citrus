use super::*;

use js_sys::Float32Array;

use na::Matrix4;

/// A draw command.
///
/// Because OpenGL is a state-machine abomination, we have to bundle up draw
/// commands so their components are disposed of properly.
pub struct DrawCommand<A>
where A: AttributeLink {
    gl: WebGl,
    attribute: A,
    draw_count: usize,
}

impl<A> DrawCommand<A>
where A: AttributeLink {
    /// Attributes an array of Vector2s. The length will be used to call the
    /// draw function.
    pub fn vertices_vec2(
        self,
        data: &[f32],
        attrib: u32,
    ) -> DrawCommand<AttributeVec2<A>> {
        self.attribute_vec2(data, attrib)
            .draw_count(data.len() / 2)
    }

    /// Attributes an array of Vector2s.
    pub fn attribute_vec2(
        self, 
        data: &[f32], 
        attrib: u32
    ) -> DrawCommand<AttributeVec2<A>> {
        let buffer = self.gl.create_buffer().unwrap();
        let data = Float32Array::from(data);

        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER, 
            &data, 
            GL::STREAM_DRAW,
        );

        DrawCommand {
            attribute: AttributeVec2 {
                parent: self.attribute,
                data: buffer,
                attrib,
            },
            gl: self.gl,
            draw_count: self.draw_count,
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
            draw_count: self.draw_count,
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
            draw_count: self.draw_count,
        }
    }

    /// Sets the draw count, which is how many units must be processed.
    pub fn draw_count(self, draw_count: usize) -> DrawCommand<A> {
        DrawCommand {
            draw_count,
            ..self
        }
    }

    /// Executes the command.
    pub fn draw_triangle_strip(self) {
        // execute attributes
        self.attribute.attribute(&self.gl);

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, self.draw_count as i32);

        // drop values
        self.attribute.destroy(&self.gl);
    }

    pub fn new(gl: WebGl) -> DrawCommand<()> {
        DrawCommand {
            gl,
            attribute: (),
            draw_count: 0,
        }
    }
}

pub trait AttributeLink {
    fn attribute(&self, gl: &WebGl);

    fn destroy(&self, gl: &WebGl);
}

impl AttributeLink for () {
    fn attribute(&self, _gl: &WebGl) { }

    fn destroy(&self, _gl: &WebGl) { }
}

/// Attributes a Vector2 array to an attribute.
pub struct AttributeVec2<P> 
where P: AttributeLink {
    parent: P,
    data: WebGlBuffer,
    attrib: u32,
}

impl<P> AttributeLink for AttributeVec2<P> 
where P: AttributeLink {
    fn attribute(&self, gl: &WebGl) {
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.data));
        gl.vertex_attrib_pointer_with_i32(self.attrib, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.attrib);

        // call next in chain
        self.parent.attribute(gl);
    }

    fn destroy(&self, gl: &WebGl) {
        gl.delete_buffer(Some(&self.data));

        // call next in chain
        self.parent.destroy(gl);
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

    fn destroy(&self, gl: &WebGl) {
        // do not delete these, because they aren't our assets and will be
        // dropped when they are needed.
        // call next in chain
        self.parent.destroy(gl);
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

    fn destroy(&self, gl: &WebGl) {
        // do not delete these, because they aren't our assets and will be
        // dropped when they are needed.
        // call next in chain
        self.parent.destroy(gl);
    }
}
