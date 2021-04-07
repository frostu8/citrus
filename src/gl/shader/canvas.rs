use crate::gl::*;
use crate::gl::shader::Shader;

use na::{Matrix4, Vector3, Orthographic3};

const VERT_SHADER: &'static str = include_str!("./canvas.vert");
const FRAG_SHADER: &'static str = include_str!("./canvas.frag");

/// GL shader that exposes basic "html canvas"-like functions.
pub struct CanvasShader { 
    projection: Orthographic3<f32>,
    pub transform: Matrix4<f32>,
    program: CanvasShaderProgram,
}

impl CanvasShader {
    /// Clears the canvas.
    pub fn clear(&self) {
        self.program.clear();
    }

    /// Rebuilds the projection matrix.
    ///
    /// Should be called whenever the canvas is resized.
    pub fn rebuild_projection(&mut self, proj: &na::Vector2<f32>) {
        self.projection = Orthographic3::new(
            0., proj.x, 
            proj.y, 0., 
            -1., 1.
        );
    }

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
            &tex,
            &(
                self.projection.into_inner() *
                self.transform *
                Matrix4::new_translation(&Vector3::new(x, y, 0.))
                    .prepend_nonuniform_scaling(&Vector3::new(width, height, 1.))
            ),
        );
    }
}

impl Shader for CanvasShader {
    fn link(gl: GL) -> Result<CanvasShader, GlError> {
        CanvasShaderProgram::new(gl)
            .map(|program| CanvasShader {
                projection: Orthographic3::new(
                    0., 1., 
                    1., 0., 
                    -1., 1.
                ),
                transform: Matrix4::identity(),
                program,
            })
    }
}

struct CanvasShaderProgram {
    gl: GL,
    program: GLProgram,
    // uniforms
    texture: GLUniformLocation,
    world_transform: GLUniformLocation,
    // attributes
    pos: u32,
}

impl CanvasShaderProgram {
    pub fn new(gl: GL) -> Result<CanvasShaderProgram, GlError> {
        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        let program = gl.link_program(
            gl.compile_vert_shader(VERT_SHADER)?,
            gl.compile_frag_shader(FRAG_SHADER)?,
        )?;

        Ok(CanvasShaderProgram {
            pos: gl.get_attrib_location(&program, "aUnitPos") as u32,

            texture: gl.get_uniform_location(&program, "uTexture")?,
            world_transform: gl.get_uniform_location(&program, "uWorldMatrix")?,

            program,
            gl,
        })
    }

    pub fn clear(&self) {
        self.gl.clear_color(1., 1., 1., 1.);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
    }

    pub fn draw_rect(
        &self, 
        tex: &GLTexture,
        mat: &Matrix4<f32>,
    ) {
        // use our program
        self.gl.use_program(Some(&self.program));

        // create vertex buffer
        self.gl.start_draw()
            .vertices_vec2(
                &[
                    // top right
                    1., 0.,
                    // top left corner is just x, y
                    0., 0.,
                    // bottom right
                    1., 1.,
                    // bottom left
                    0., 1.,
                ],
                self.pos,
            )
            // bind our texture
            .bind_texture0(
                tex,
                &self.texture,
            )
            // bind the world matrix
            .uniform_mat4(
                mat,
                &self.world_transform,
            )
            .draw_triangle_strip();
    }
}

