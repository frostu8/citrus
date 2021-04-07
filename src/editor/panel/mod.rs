use yew::prelude::*;
use yew::callback::Callback;
use yew::services::timeout::{TimeoutTask, TimeoutService};
use web_sys::HtmlElement;

use citrus_common::PanelKind;

use super::assets::{PanelMap, self};

/// Panel selector component.
pub struct PanelSelector {
    link: ComponentLink<Self>,
    props: Props,

    panel_images: PanelMap<NodeRef>,
    container: NodeRef,

    pos_lerp: f32,
    _lerp_request: Option<TimeoutTask>,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub onselect: Callback<PanelKind>,
    pub selected: PanelKind,
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        if self.selected == other.selected {
            return false;
        }

        match &self.onselect {
            Callback::Callback(ptr1) => {
                if let Callback::Callback(ptr2) = &other.onselect {
                    std::rc::Rc::ptr_eq(ptr1, ptr2)
                } else {
                    false
                }
            },
            Callback::CallbackOnce(ptr1) => {
                if let Callback::CallbackOnce(ptr2) = &other.onselect {
                    std::rc::Rc::ptr_eq(ptr1, ptr2)
                } else {
                    false
                }
            },
        }
    }
}

pub enum Msg {
    Lerp(f32),
    MouseEnter,
    MouseLeave,
    Select(PanelKind),
}

impl Component for PanelSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PanelSelector {
            link,
            props,

            panel_images: PanelMap::new(|_| NodeRef::default()),
            container: NodeRef::default(),

            // start retracted
            pos_lerp: 1f32,
            _lerp_request: None,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        self.render_lerp();
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Lerp(inc) => {
                // update lerp
                self.pos_lerp += inc;

                if self.pos_lerp > 0. && self.pos_lerp < 1. {
                    // continue lerp
                    self.start_lerp(inc);
                } else {
                    // clamp
                    self.pos_lerp = self.pos_lerp.clamp(0., 1.);
                    // forcibly drop lerp request
                    self._lerp_request = None;
                }

                self.render_lerp();
            },
            Msg::MouseEnter => {
                // setup timeout, replacing the old one, if it exists
                // Thanks to Closures for being awesome and existing.
                self.start_lerp(-0.08);
            },
            Msg::MouseLeave => {
                self.start_lerp(0.08);
            },
            Msg::Select(kind) => {
                self.props.selected = kind;
                // bubble
                self.props.onselect.emit(kind);
                // rerender
                return true;
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="panel-selector-area"
                 onmouseenter=self.link.callback(|_| Msg::MouseEnter) 
                 onmouseleave=self.link.callback(|_| Msg::MouseLeave) >
                { self.generate_buttons() }
            </div>
        }
    }
}

impl PanelSelector {
    pub fn render_lerp(&self) {
        if let Some(container) = self.container.cast::<HtmlElement>() {
            let width = container.client_width();

            let _ = container.style().set_property(
                "transform",
                &format!(
                    "translate({}px, 0px)",
                    width as f32 * -self.pos_lerp,
                ),
            );
        }
    }

    fn start_lerp(&mut self, dir: f32) {
        self._lerp_request = Some(
            TimeoutService::spawn(
                std::time::Duration::from_millis(1000 / 60),
                self.link.callback(move |_| Msg::Lerp(dir)),
            )
        )
    }

    fn generate_buttons(&self) -> Html {
        html! {
            <div class="panel-selector"
                 ref=self.container.clone() >
                { for self.panel_images.iter()
                    .filter_map(|(kind, r)| {
                        Some((assets::panel_source(kind)?, kind, r))
                    })
                    .map(|(src, kind, r)| {
                        html! {
                            <a class={ if kind == self.props.selected {
                                   "panel-button selected"
                               } else {
                                   "panel-button"
                               } } 
                               href="#"
                               onclick=self.link.callback(move |_| Msg::Select(kind))>
                                <img ref=r.clone() src=src />
                            </a>
                        }
                    }) }
            </div>
        }
    }
}
