use crate::gl::*;
use crate::gl::shader::Shader;

use na::{Matrix4, Vector3, Orthographic3};

const VERT_SHADER: &'static str = include_str!("./canvas.vert");
const FRAG_SHADER: &'static str = include_str!("./canvas.frag");

/// GL shader that exposes basic "html canvas"-like functions.
pub struct CanvasShader { 
    projection: Orthographic3<f32>,
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

    pub fn begin_draw<'a>(
        &'a mut self, 
        transform: &Matrix4<f32>,
    ) -> DrawCommand<'a> {
        DrawCommand::new(
            &mut self.program, 
            &self.projection,
            transform,
        )
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
    // static data
    unit_square: GLBuffer,
}

impl CanvasShaderProgram {
    pub fn new(gl: GL) -> Result<CanvasShaderProgram, GlError> {
        gl.enable(GL::BLEND);
        gl.blend_func_separate(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA, GL::ONE, GL::ONE_MINUS_SRC_ALPHA);

        let program = gl.link_program(
            gl.compile_vert_shader(VERT_SHADER)?,
            gl.compile_frag_shader(FRAG_SHADER)?,
        )?;

        let unit_square = gl.create_static_buffer(
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
        );

        Ok(CanvasShaderProgram {
            pos: gl.get_attrib_location(&program, "aUnitPos") as u32,

            texture: gl.get_uniform_location(&program, "uTexture")?,
            world_transform: gl.get_uniform_location(&program, "uWorldMatrix")?,

            program,
            gl,

            unit_square,
        })
    }

    pub fn clear(&self) {
        self.gl.clear_color(1., 1., 1., 0.);
        self.gl.clear(GL::COLOR_BUFFER_BIT);
    }
}

/// Draw command for a [`CanvasShader`].
///
/// # Warning!
/// Only one of these should exist at a time!
pub struct DrawCommand<'a> {
    projection: Orthographic3<f32>,
    transform: Matrix4<f32>,
    program: &'a mut CanvasShaderProgram,
}

impl<'a> DrawCommand<'a> {
    fn new(
        program: &'a mut CanvasShaderProgram, 
        projection: &Orthographic3<f32>,
        transform: &Matrix4<f32>,
    ) -> DrawCommand<'a> {
        // use program
        program.gl.use_program(Some(&program.program));

        // bind unit square verts
        program.gl.attribute_buffer(
            &program.unit_square,
            program.pos,
        );

        DrawCommand { 
            projection: *projection,
            transform: *transform,
            program,
        }
    }

    pub fn texture(&mut self, tex: &GLTexture) {
        self.program.gl.uniform_tex(
            tex,
            &self.program.texture,
            0,
        );
    }
    
    pub fn draw_rect(
        &mut self,
        x: f32, y: f32,
        width: f32, height: f32
    ) {
        // bind the world matrix
        self.program.gl.uniform_mat4(
            &(
                self.projection.into_inner() *
                self.transform *
                Matrix4::new_translation(&Vector3::new(x, y, 0.))
                    .prepend_nonuniform_scaling(&Vector3::new(width, height, 1.))
            ),
            &self.program.world_transform,
        );

        // draw
        self.program.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);
    }
}
