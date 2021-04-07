use crate::gl::*;
use crate::gl::shader::Shader;

use na::{Vector2, Matrix3};

const VERT_SHADER: &'static str = include_str!("./canvas.vert");
const FRAG_SHADER: &'static str = include_str!("./canvas.frag");

/// GL shader that exposes basic "html canvas"-like functions.
pub struct CanvasShader { 
    pub transform: Matrix3<f32>,
    program: CanvasShaderProgram,
}

impl CanvasShader {
    /// Fills a rectangle with a color.
    pub fn fill_rect(
        &self,
        color: Color,
        x: f32, y: f32,
        width: f32, height: f32,
    ) {
        let color_tex = self.program.gl.solid_color_texture(color);

        self.draw_image(&color_tex, x, y, width, height);
    }

    /// Draws a texture to the screen as a rectangle.
    pub fn draw_image(
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

    /// Needs to be called every resize.
    pub fn rebuild_projection(&mut self) {
        self.program.rebuild_projection();
    }
}

impl Shader for CanvasShader {
    fn link(gl: GL) -> Result<CanvasShader, GlError> {
        CanvasShaderProgram::new(gl)
            .map(|program| CanvasShader {
                transform: Matrix3::identity(),
                program,
            })
    }
}

struct CanvasShaderProgram {
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

impl CanvasShaderProgram {
    pub fn new(gl: GL) -> Result<CanvasShaderProgram, GlError> {
        let program = gl.link_program(
            gl.compile_vert_shader(VERT_SHADER)?,
            gl.compile_frag_shader(FRAG_SHADER)?,
        )?;

        Ok(CanvasShaderProgram {
            pos: gl.get_attrib_location(&program, "aPos") as u32,
            tex_coord: gl.get_attrib_location(&program, "aTexCoord") as u32,
            projection: Self::build_projection(&gl),

            view_matrix: gl.get_uniform_location(&program, "viewMatrix")?,
            projection_matrix: gl.get_uniform_location(&program, "projectionMatrix")?,
            texture: gl.get_uniform_location(&program, "uTexture")?,

            program,
            gl,
        })
    }

    pub fn rebuild_projection(&mut self) {
        self.projection = Self::build_projection(&self.gl);
    }

    pub fn build_projection(gl: &GL) -> Matrix3<f32> {
        let scaling = Vector2::new(
            1. / (gl.drawing_buffer_width() as f32 / 2.),
            // flip height so the origin is the top left
            -(1. / (gl.drawing_buffer_height() as f32 / 2.)),
        ) / 2.;
        
        Matrix3::new_nonuniform_scaling(&scaling)
            .append_translation(&na::Vector2::new(-1., 1.))
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
        self.gl.start_draw()
            .vertices_vec2(
                &[
                    // top right
                    x + width, y,
                    // top left corner is just x, y
                    x,         y,
                    // bottom right
                    x + width, y + height,
                    // bottom left
                    x,         y + height,
                ],
                self.pos,
            )
            .attribute_vec2(
                &[
                    1., 0.,
                    0., 0.,
                    1., 1.,
                    0., 1.,
                ],
                self.tex_coord,
            )
            // bind our texture
            .bind_texture0(
                tex,
                &self.texture,
            )
            // attach the projection matrix, used to convert the -1.0 and 1.0 
            // to pixel coordinates.
            .uniform_mat3(
                &self.projection,
                &self.projection_matrix,
            )
            // attach the view matrix
            .uniform_mat3(
                matrix,
                &self.view_matrix,
            )
            .draw_triangle_strip();
    }
}

