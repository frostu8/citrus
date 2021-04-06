use crate::gl::*;

use na::{Vector2, Matrix3};

use std::ops::Deref;

const BASIC_VERT_SHADER: &'static str = include_str!("./basic.vert");
const BASIC_FRAG_SHADER: &'static str = include_str!("./basic.frag");

/// GL shader that exposes basic functions to draw to a canvas.
pub struct BasicShader { 
    pub transform: Matrix3<f32>,
    program: BasicGlProgram,
}

impl BasicShader {
    /// Makes a new field program, cloning the reference to the GL type.
    pub fn new(gl: impl Into<GL>) -> Result<BasicShader, GlError> {
        BasicGlProgram::new(gl.into())
            .map(|program| BasicShader {
                transform: Matrix3::identity(),
                program,
            })
    }

    /// Gets a reference to the related GL context.
    pub fn gl(&self) -> &GL {
        &self.program.gl
    }

    /// Fills a rectangle with a color.
    pub fn fill_rect(
        &self,
        color: Color,
        x: f32, y: f32,
        width: f32, height: f32,
    ) {
        let color_tex = self.program.solid_color_texture(color);

        self.tex_rect(&color_tex, x, y, width, height);
    }

    /// Draws a texture to the screen as a rectangle.
    pub fn tex_rect(
        &self,
        tex: &GLTexture,
        x: f32, y: f32,
        width: f32, height: f32,
    ) {
        self.program.draw_rect(
            &self.transform,
            &tex,
            x, y, width, height,
        );
    }
}

struct BasicGlProgram {
    gl: GL,
    program: GLProgram,
    projection: Matrix3<f32>,
    // uniforms
    view_matrix: GLUniformLocation,
    projection_matrix: GLUniformLocation,
    texture: GLUniformLocation,
    // attributes
    pos: u32,
    tex_coord: u32,
}

impl BasicGlProgram {
    pub fn new(gl: GL) -> Result<BasicGlProgram, GlError> {
        let program = gl.link_program(
            gl.compile_vert_shader(BASIC_VERT_SHADER)?,
            gl.compile_frag_shader(BASIC_FRAG_SHADER)?,
        )?;

        Ok(BasicGlProgram {
            pos: gl.get_attrib_location(&program, "aPos") as u32,
            tex_coord: gl.get_attrib_location(&program, "aTexCoord") as u32,
            projection: {
                let scaling = Vector2::new(
                    1. / (gl.drawing_buffer_width() as f32 / 2.),
                    // flip height so the origin is the top left
                    -(1. / (gl.drawing_buffer_height() as f32 / 2.)),
                ) / 2.;
                
                Matrix3::new_nonuniform_scaling(&scaling)
                    .append_translation(&na::Vector2::new(-1., 1.))
            },

            view_matrix: gl.get_uniform_location(&program, "viewMatrix")?,
            projection_matrix: gl.get_uniform_location(&program, "projectionMatrix")?,
            texture: gl.get_uniform_location(&program, "uTexture")?,

            program,
            gl,
        })
    }

    pub fn draw_rect(
        &self, 
        matrix: &na::Matrix3<f32>,
        tex: &GLTexture,
        x: f32, y: f32,
        width: f32, height: f32,
    ) {
        // use our program
        self.gl.use_program(Some(&self.program));

        // create vertex buffer
        let vertex_buffer = self.gl.create_buffer().unwrap();
        let vertex_data = js_sys::Float32Array::from(&[
            // top right
            x + width, y,
            // top left corner is just x, y
            x,         y,
            // bottom right
            x + width, y + height,
            // bottom left
            x,         y + height,
        ][..]);

        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER, 
            &vertex_data, 
            GL::STATIC_DRAW
        );

        // attach our position vector to our attribute
        self.gl.vertex_attrib_pointer_with_i32(self.pos, 2, GL::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(self.pos);

        // create texture pos buffer
        let tex_coord_buffer = self.gl.create_buffer().unwrap();
        let tex_coord_data = js_sys::Float32Array::from(&[
            1., 0.,
            0., 0.,
            1., 1.,
            0., 1.,
        ][..]);

        self.gl.bind_buffer(GL::ARRAY_BUFFER, Some(&tex_coord_buffer));
        self.gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER, 
            &tex_coord_data, 
            GL::STATIC_DRAW
        );

        // attach our tex coord
        self.gl.vertex_attrib_pointer_with_i32(self.tex_coord, 2, GL::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(self.tex_coord);

        // attach the texture
        self.gl.active_texture(GL::TEXTURE0);
        self.gl.bind_texture(GL::TEXTURE_2D, Some(tex));
        self.gl.uniform1i(Some(&self.texture), 0);

        // attach the projection matrix, used to convert the -1.0 and 1.0 to
        self.gl.uniform_matrix3(
            &self.projection_matrix,
            &self.projection,
        );

        // attach the view matrix
        self.gl.uniform_matrix3(&self.view_matrix, matrix);

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);

        // delete buffers
        self.gl.delete_buffer(Some(&tex_coord_buffer));
        self.gl.delete_buffer(Some(&vertex_buffer));
    }
}

impl Deref for BasicGlProgram {
    type Target = GL;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}
