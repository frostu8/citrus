use wasm_bindgen::JsCast as _;
use web_sys::{
    WebGlShader, 
    WebGlProgram, 
    WebGlTexture, 
    WebGlRenderingContext as GL,
    WebGlUniformLocation,
};

use std::ops::Deref;

const FIELD_VERT_SHADER: &'static str = include_str!("./field.vert");
const FIELD_FRAG_SHADER: &'static str = include_str!("./field.frag");

/// Shader used to render panels to a canvas.
pub struct FieldProgram {
    gl: GL,
    program: WebGlProgram,
    // uniforms
    view_matrix: WebGlUniformLocation,
    panel_texture: WebGlUniformLocation,
    // attributes
    pos: u32,
    tex_coord: u32,
}

impl FieldProgram {
    /// Makes a new field program.
    pub fn new(gl: &GL) -> Result<FieldProgram, String> {
        let program = link_program(
            gl,
            compile_vert_shader(gl, FIELD_VERT_SHADER)?,
            compile_frag_shader(gl, FIELD_FRAG_SHADER)?,
        )?;

        Ok(FieldProgram {
            pos: gl.get_attrib_location(&program, "aPos") as u32,
            tex_coord: gl.get_attrib_location(&program, "aTexCoord") as u32,

            view_matrix: get_uniform_location(gl, &program, "viewMatrix")?,
            panel_texture: get_uniform_location(gl, &program, "panelTexture")?,

            program,
            gl: gl.clone().unchecked_into(),
        })
    }

    /// Draw a panel at the position specified
    pub fn draw_panel(&self, tex: &WebGlTexture, x: f32, y: f32) {
        // use our program
        self.gl.use_program(Some(&self.program));

        // create vertex buffer
        let vertex_buffer = self.gl.create_buffer().unwrap();
        let vertex_data = js_sys::Float32Array::from(&[
            // top right
            x + 1., y,
            // top left corner is just x, y
            x,      y,
            // bottom right
            x + 1., y + 1.,
            // bottom left
            x,      y + 1.,
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
            0., 0.,
            1., 0.,
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
        self.gl.uniform1i(Some(&self.panel_texture), 0);

        // attach the transformation matrix
        // TODO
        self.gl.uniform_matrix3fv_with_f32_array(
            Some(&self.view_matrix),
            false,
            &[
                128., 0., 0.,
                0., 128., 0.,
                0., 0., 1.,
            ],
        );

        self.gl.draw_arrays(GL::TRIANGLE_STRIP, 0, 4);
    }
}

pub fn test_texture(gl: &GL) -> Result<WebGlTexture, String> {
    let texture = gl.create_texture().unwrap();
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    // create texture with blue pixel
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        1,
        1,
        0,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        Some(&[0, 0, 255, 255]),
    )
        .map(|_| texture)
        .map_err(|_| String::from("failed to init test texture"))
}

fn get_uniform_location(
    gl: &GL, program: &WebGlProgram, name: &str
) -> Result<WebGlUniformLocation, String> {
    gl.get_uniform_location(program, name)
        .map(|u| Ok(u))
        .unwrap_or(Err(format!("cannot find uniform: {}", name)))
}

fn link_program(
    gl: &GL, 
    vert: WebGlShader, 
    frag: WebGlShader,
) -> Result<WebGlProgram, String> {
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

        Err(err)
    }
}

fn compile_vert_shader(gl: &GL, src: &str) -> Result<WebGlShader, String> {
    compile_shader(gl, GL::VERTEX_SHADER, src)
}

fn compile_frag_shader(gl: &GL, src: &str) -> Result<WebGlShader, String> {
    compile_shader(gl, GL::FRAGMENT_SHADER, src)
}

fn compile_shader(gl: &GL, kind: u32, src: &str) -> Result<WebGlShader, String> {
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

        Err(err)
    }
}
