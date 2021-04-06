mod color;
mod error;

use wasm_bindgen::JsCast as _;
use web_sys::{
    WebGlShader, 
    WebGlProgram, 
    WebGlTexture, 
    WebGlRenderingContext as GL,
    WebGlUniformLocation,
};
use na::{Vector2, Matrix3};

pub use error::*;
pub use color::*;

const BASIC_VERT_SHADER: &'static str = include_str!("./basic.vert");
const BASIC_FRAG_SHADER: &'static str = include_str!("./basic.frag");

/// GL shader that exposes basic functions to draw to a canvas.
pub struct BasicShader { 
    transform: Matrix3<f32>,
    program: BasicGlProgram,
}

impl BasicShader {
    /// Makes a new field program.
    pub fn new(gl: &GL) -> Result<BasicShader, GlError> {
        BasicGlProgram::new(gl)
            .map(|program| BasicShader {
                transform: Matrix3::identity(),
                program,
            })
    }

    /// Fills a rectangle with a color.
    pub fn fill_rect(
        &self,
        color: Color,
        x: f32, y: f32,
        width: f32, height: f32,
    ) {
        let color_tex = self.program.solid_color_texture(color);

        self.program.draw_rect(
            &self.transform,
            &color_tex,
            x, y, width, height,
        );

        self.program.delete_texture(color_tex);
    }
}

struct BasicGlProgram {
    gl: GL,
    program: WebGlProgram,
    projection: Matrix3<f32>,
    // uniforms
    view_matrix: WebGlUniformLocation,
    projection_matrix: WebGlUniformLocation,
    texture: WebGlUniformLocation,
    // attributes
    pos: u32,
    tex_coord: u32,
}

impl BasicGlProgram {
    pub fn new(gl: &GL) -> Result<BasicGlProgram, GlError> {
        let program = link_program(
            gl,
            compile_vert_shader(gl, BASIC_VERT_SHADER)?,
            compile_frag_shader(gl, BASIC_FRAG_SHADER)?,
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

            view_matrix: get_uniform_location(gl, &program, "viewMatrix")?,
            projection_matrix: get_uniform_location(gl, &program, "projectionMatrix")?,
            texture: get_uniform_location(gl, &program, "uTexture")?,

            program,
            gl: gl.clone().unchecked_into(),
        })
    }

    pub fn draw_rect(
        &self, 
        matrix: &na::Matrix3<f32>,
        tex: &WebGlTexture,
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
        uniform_matrix3(
            &self.gl, 
            &self.projection_matrix,
            &self.projection,
        );

        // attach the view matrix
        uniform_matrix3(&self.gl, &self.view_matrix, matrix);

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);

        // delete buffers
        self.gl.delete_buffer(Some(&tex_coord_buffer));
        self.gl.delete_buffer(Some(&vertex_buffer));
    }

    pub fn solid_color_texture(&self, color: Color) -> WebGlTexture {
        let texture = self.gl.create_texture().unwrap();
        self.gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        // create texture with blue pixel
        // this should always be valid
        self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            1,
            1,
            0,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            Some(&[color.red(), color.green(), color.blue(), color.alpha()]),
        ).unwrap();

        texture
    }

    pub fn delete_texture(&self, tex: WebGlTexture) {
        self.gl.delete_texture(Some(&tex));
    }
}

fn uniform_matrix3(
    gl: &GL,
    uniform: &WebGlUniformLocation,
    mat: &na::Matrix3<f32>,
) {
    gl.uniform_matrix3fv_with_f32_array(
        Some(&uniform),
        false,
        mat.as_slice(),
    );
}

fn get_uniform_location(
    gl: &GL, program: &WebGlProgram, name: &str
) -> Result<WebGlUniformLocation, UniformNotFoundError> {
    gl.get_uniform_location(program, name)
        .map(|u| Ok(u))
        .unwrap_or(Err(UniformNotFoundError::new(name.to_string())))
}

fn link_program(
    gl: &GL, 
    vert: WebGlShader, 
    frag: WebGlShader,
) -> Result<WebGlProgram, ProgramLinkError> {
    // create and link program
    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vert);
    gl.attach_shader(&program, &frag);
    gl.link_program(&program);

    // check for link errors
    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap() {
        Ok(program)
    } else {
        let err = gl.get_program_info_log(&program).unwrap();
        gl.delete_program(Some(&program));

        Err(ProgramLinkError::new(err))
    }
}

fn compile_vert_shader(gl: &GL, src: &str) -> Result<WebGlShader, ShaderCompileError> {
    compile_shader(gl, GL::VERTEX_SHADER, src)
}

fn compile_frag_shader(gl: &GL, src: &str) -> Result<WebGlShader, ShaderCompileError> {
    compile_shader(gl, GL::FRAGMENT_SHADER, src)
}

fn compile_shader(
    gl: &GL, 
    kind: u32, 
    src: &str,
) -> Result<WebGlShader, ShaderCompileError> {
    // create and compile shader
    let shader = gl.create_shader(kind).unwrap();
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);

    // check shader compilation issues
    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap() {
        Ok(shader)
    } else {
        let err = gl.get_shader_info_log(&shader).unwrap();
        gl.delete_shader(Some(&shader));

        Err(ShaderCompileError::new(err))
    }
}
