pub mod assets;
pub mod panel;
pub mod view;

use wasm_bindgen::JsValue;
use web_sys::{console, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;
use yew::services::render::{RenderTask, RenderService};
use yew::services::resize::{ResizeTask, ResizeService};
use crate::services::image::{ImageService, ImageTask};

use assets::PanelMap;
use view::EditorView;

use citrus_common::{PanelKind, Panel};
use na::Vector2;
use crate::gl::{GLTexture, GL, GlError};
use crate::gl::shader::canvas::CanvasShader;
use crate::util::{MouseEvent, WheelEvent};

pub struct FieldEditor {
    link: ComponentLink<Self>,
    view: EditorView,

    // event things
    mouse_last: MouseEvent,

    // canvas things
    canvas: NodeRef,
    canvas_size: Vector2<f32>,
    gl: Option<GL>,
    basic_shader: Option<CanvasShader>,
    panel_textures: PanelMap<Texture>,

    // callback things
    _render_request: Option<RenderTask>,
    _resize_request: Option<ResizeTask>,
}

pub enum Msg {
    Render(f64),
    MouseMove(web_sys::MouseEvent),
    MouseUp(web_sys::MouseEvent),
    MouseWheel(web_sys::WheelEvent),
    ContextMenu(web_sys::MouseEvent),
    TextureLoad((HtmlImageElement, PanelKind)),
    TextureError((HtmlImageElement, PanelKind)),
    PanelKindSelect(PanelKind),
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
            view: EditorView::new(default_panel()),
            mouse_last: MouseEvent::default(),
            canvas: NodeRef::default(),
            canvas_size: na::zero(),
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
            self.build_basic_shader();

            if first_render {
                // request for textures
                self.request_panel_images();
            }
        }

        self.update_size();

        if first_render {
            self.view.center(&self.canvas_size);

            self.setup_callbacks();
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
            },
            Msg::MouseMove(ev) => {
                let ev: MouseEvent = (&ev).into();

                // handle mouse move if mouse is down
                if ev.buttons().right() {
                    self.view.pan(ev.pos() - self.mouse_last.pos());
                }

                // set as last mouse event
                self.mouse_last = ev;
            },
            Msg::MouseUp(ev) => {
                let ev: MouseEvent = (&ev).into();

                // handle panel placement if mouse is down
                if ev.button().left() {
                    if ev.modifiers().shift() {
                        // delete tile
                        *self.view.flex_mut(&ev.pos()) = Panel::EMPTY;
                        self.view.collapse();
                    } else {
                        // place current tile
                        self.view.flex_mut(&ev.pos()).kind = self.view.selected;
                        self.view.collapse();
                    }
                }
            },
            Msg::MouseWheel(ev) => {
                let ev: WheelEvent = (&ev).into();

                let delta = ev.delta_y() * -0.01;

                // handle scroll? ez
                let scale = self.view.get_scale();
                // cap scroll
                if delta > 0. {
                    if scale.x.max(scale.y) < EditorView::MAX_ZOOM {
                        self.view.scale(1. + delta, ev.pos());
                    }
                } else {
                    if scale.x.max(scale.y) > EditorView::MIN_ZOOM {
                        self.view.scale(1. + delta, ev.pos());
                    }
                }
            }
            Msg::ContextMenu(ev) => {
                ev.prevent_default();
            },
            Msg::TextureLoad((image, panel_kind)) => {
                // NOTE: this call is completely sane, since the textures are
                // only requested after the GL creation.
                let gl = self.gl.as_ref().unwrap();

                self.panel_textures[panel_kind] = Texture::Ready(
                    gl.create_texture(&image),
                );
            },
            Msg::TextureError((_image, panel_kind)) => {
                console::log_1(&JsValue::from(format!("image for {:?} failed to load.", panel_kind)));

                self.panel_textures[panel_kind] = Texture::Error;
            },
            Msg::PanelKindSelect(kind) => {
                self.view.selected = kind;
            },
            Msg::Resize => {
                // rerender
                return true;
            },
        };

        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="editor-container">
                <panel::PanelSelector onselect=self.link.callback(Msg::PanelKindSelect) 
                                      selected=default_panel() />
                <canvas class="editor-canvas"
                        oncontextmenu=self.link.callback(Msg::ContextMenu)
                        onmousemove=self.link.callback(Msg::MouseMove)
                        onmouseup=self.link.callback(Msg::MouseUp)
                        onwheel=self.link.callback(Msg::MouseWheel)
                        ref=self.canvas.clone()>
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
    pub fn render(&mut self, _timestamp: f64) {
        let basic = match self.basic_shader.as_mut() {
            Some(basic) => basic,
            None => return,
        };

        // clear
        basic.clear();
        
        // setup view matrix
        let mut draw = basic.begin_draw(&self.view.view);

        for (x, y) in self.view.field.iter() {
            let panel = self.view.field.get(x, y);
            let (x, y) = (x as f32, y as f32);

            let panel_tex = match &self.panel_textures[panel.kind] {
                Texture::Ready(tex) => tex,
                _ => continue,
            };

            draw.texture(panel_tex);

            draw.draw_rect(
                x, y,
                1., 1.,
            );
        }
    }

    fn canvas(&self) -> HtmlCanvasElement {
        self.canvas.cast::<HtmlCanvasElement>().unwrap()
    }

    fn gl_invalidated(&self) -> bool {
        self.gl.as_ref().map(|gl| gl.is_context_lost()).unwrap_or(true)
    }

    fn build_gl(&mut self) {
        // get gl context
        match GL::new(self.canvas()) {
            Some(gl) => {
                self.gl = Some(gl);
            },
            None => {
                self.canvas().set_inner_text(
                    "OpenGL is not supported on your browser."
                );
            }
        }
    }

    fn build_basic_shader(&mut self) {
        let gl = match self.gl.as_ref() {
            Some(gl) => gl,
            None => return,
        };

        let basic_shader = match gl.shader() {
            Ok(p) => p,
            Err(err) => {
                // print pretty error to console.
                match err {
                    GlError::ShaderCompile(error) => {
                        console::error_1(&JsValue::from_str("shader compile errors:"));
                        for error in error.errors() {
                            console::error_1(&JsValue::from_str(error));
                        }
                    },
                    err => {
                        console::error_1(&JsValue::from_str(&err.to_string()));
                    }
                }
                panic!("failed to compile shaders");
            },
        };

        self.basic_shader = Some(basic_shader);
    }

    fn update_size(&mut self) {
        let canvas = self.canvas();

        self.canvas_size = Vector2::new(
            canvas.client_width() as f32,
            canvas.client_height() as f32,
        );

        if let Some(gl) = self.gl.as_ref() {
            canvas.set_width(self.canvas_size.x as u32);
            canvas.set_height(self.canvas_size.y as u32);

            gl.viewport(0, 0, self.canvas_size.x as i32, self.canvas_size.y as i32);

            if let Some(basic) = self.basic_shader.as_mut() {
                basic.rebuild_projection(&self.canvas_size);
            }
        }
    }

    fn request_panel_images(&mut self) {
        for (kind, image) in self.panel_textures.iter_mut() {
            let src = assets::panel_source(kind);

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

    fn setup_callbacks(&mut self) {
        self.request_resize_event();
        self.request_animation_frame();
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

pub fn default_panel() -> PanelKind {
    PanelKind::Neutral
}
