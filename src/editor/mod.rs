mod shader;

use wasm_bindgen::{JsValue, JsCast as _};
use web_sys::{console, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::prelude::*;
use yew::services::render::{RenderTask, RenderService};

pub struct FieldEditor {
    link: ComponentLink<Self>,

    // canvas things
    canvas: NodeRef,
    gl: Option<GL>,
    _render_request: Option<RenderTask>,
}

pub enum Msg {
    Render(f64),
}

impl Component for FieldEditor {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FieldEditor { 
            link,
            canvas: NodeRef::default(),
            gl: None,
            _render_request: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();

        // get gl context
        let gl: GL = canvas
            .get_context("webgl").unwrap().unwrap()
            .dyn_into().unwrap();

        self.gl = Some(gl);

        if first_render {
            self.request_animation_frame();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                // render the field editor
                self.render(timestamp);

                // setup another request
                self.request_animation_frame();

                false
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <canvas ref=self.canvas.clone()/>
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
    pub fn render(&self, timestamp: f64) {
        use shader::{BasicShader, Color};

        let gl = self.gl.as_ref().unwrap();

        let field_program = match BasicShader::new(gl) {
            Ok(p) => p,
            Err(err) => {
                // print pretty error to console.
                console::error_1(&JsValue::from_str(&err.to_string()));
                panic!("failed to compile shaders");
            },
        };

        field_program.fill_rect(
            Color::BLUE,
            0., 0.,
            150., 150.,
        );
    }

    fn request_animation_frame(&mut self) {
        let render_frame = self.link.callback(Msg::Render);
        let handle = RenderService::request_animation_frame(render_frame);

        self._render_request = Some(handle);
    }
}
