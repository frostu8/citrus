pub mod assets;
pub mod shader;

use wasm_bindgen::{JsValue, JsCast as _};
use web_sys::{console, HtmlCanvasElement, HtmlImageElement, WebGlRenderingContext as WebGl};
use yew::prelude::*;
use yew::services::render::{RenderTask, RenderService};
use yew::services::resize::{ResizeTask, ResizeService};
use crate::services::image::{ImageService, ImageTask};

use assets::PanelMap;
use shader::BasicShader;

use citrus_common::PanelKind;
use crate::gl::{Color, GLTexture, GL};


pub struct FieldEditor {
    link: ComponentLink<Self>,

    // canvas things
    canvas: NodeRef,
    gl: Option<GL>,
    basic_shader: Option<BasicShader>,
    panel_textures: PanelMap<Texture>,

    // callback things
    _render_request: Option<RenderTask>,
    _resize_request: Option<ResizeTask>,
}

pub enum Msg {
    Render(f64),
    TextureLoad((HtmlImageElement, PanelKind)),
    TextureError((HtmlImageElement, PanelKind)),
    Resize,
}

pub enum Texture {
    Ready(GLTexture),
    Pending(ImageTask),
    Error,
    Null,
}

impl Component for FieldEditor {
    type Message = Msg;
    type Properties = (); 

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FieldEditor { 
            link,
            canvas: NodeRef::default(),
            gl: None,
            basic_shader: None,
            panel_textures: PanelMap::new(|_| Texture::Null),
            _render_request: None,
            _resize_request: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // rebuild gl if gl is invalidated
        if self.gl_invalidated() {
            self.build_gl();
        }

        self.build_basic_shader();

        // request for textures
        self.request_panel_images();

        if first_render {
            self.request_animation_frame();
            self.request_resize_event();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                // render the field editor
                if self.textures_loaded() {
                    self.render(timestamp);
                }

                // setup another request
                self.request_animation_frame();

                false
            },
            Msg::TextureLoad((image, panel_kind)) => {
                // NOTE: this call is completely sane, since the textures are
                // only requested after the GL creation.
                let gl = self.gl.as_ref().unwrap();

                self.panel_textures[panel_kind] = Texture::Ready(
                    gl.create_texture(&image),
                );

                false
            },
            Msg::TextureError((_image, panel_kind)) => {
                console::log_1(&JsValue::from(format!("image for {:?} failed to load.", panel_kind)));

                self.panel_textures[panel_kind] = Texture::Error;

                false
            }
            Msg::Resize => {
                // rerender
                true
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="editor-container">
                <canvas class="editor-canvas" ref=self.canvas.clone()>
                </canvas>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // do not render changes
        false
    }
}

impl FieldEditor {
    /// Renders the field editor to the attached canvas.
    pub fn render(&mut self, timestamp: f64) {
        let basic = match self.basic_shader.as_mut() {
            Some(basic) => basic,
            None => return,
        };

        let encounter = match &self.panel_textures[PanelKind::Encounter] {
            Texture::Ready(tex) => tex,
            _ => panic!(),
        };

        basic.tex_rect(
            encounter,
            0., 0.,
            150., 150.,
        );
    }

    fn gl_invalidated(&self) -> bool {
        self.gl.as_ref().map(|gl| gl.is_context_lost()).unwrap_or(true)
    }

    fn build_gl(&mut self) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

        // get gl context
        match canvas.get_context("webgl").ok().flatten() {
            Some(gl) => {
                self.gl = Some(gl.dyn_into::<WebGl>().unwrap().into());
            },
            None => {
                canvas.set_inner_text(
                    "OpenGL is not supported on your browser."
                );
            }
        }
    }

    fn build_basic_shader(&mut self) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

        let gl = match self.gl.as_ref() {
            Some(gl) => gl,
            None => return,
        };

        Self::update_size(&gl, &canvas);

        let basic_shader = match BasicShader::new(gl.clone()) {
            Ok(p) => p,
            Err(err) => {
                // print pretty error to console.
                console::error_1(&JsValue::from_str(&err.to_string()));
                panic!("failed to compile shaders");
            },
        };

        self.basic_shader = Some(basic_shader);
    }

    fn update_size(gl: &GL, canvas: &HtmlCanvasElement) {
        let width = canvas.client_width();
        let height = canvas.client_height();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        gl.viewport(0, 0, width, height);
    }

    fn request_panel_images(&mut self) {
        for (kind, image) in self.panel_textures.iter_mut() {
            let src = match kind {
                PanelKind::Encounter => Some("./img/encounter.png"),
                _ => None,
            };

            // make request
            if let Some(src) = src {
                *image = Texture::Pending(ImageService::new(
                    src,
                    self.link.callback(Msg::TextureLoad),
                    self.link.callback(Msg::TextureError),
                    kind,
                ))
            }
        }
    }

    fn textures_loaded(&self) -> bool {
        self.panel_textures.iter()
            .all(|(_, status)| {
                match status {
                    Texture::Ready(_) => true,
                    Texture::Null => true,
                    _ => false,
                }
            })
    }

    fn request_resize_event(&mut self) {
        let resize = self.link.callback(|_| Msg::Resize);
        let handle = ResizeService::new().register(resize);

        self._resize_request = Some(handle);
    }

    fn request_animation_frame(&mut self) {
        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        self._render_request = Some(handle);
    }
}
