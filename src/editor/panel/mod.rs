use yew::prelude::*;
use yew::callback::Callback;

use citrus_common::PanelKind;

use super::assets::{PanelMap, self};

/// Panel selector component.
pub struct PanelSelector {
    link: ComponentLink<Self>,
    props: Props,

    panel_images: PanelMap<NodeRef>,

    pos: f32,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onselect: Callback<PanelKind>,
    pub selected: PanelKind,
}

pub enum Msg {
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

            // start retracted
            pos: -1.,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MouseEnter => {
                self.update_pos(0.)
            },
            Msg::MouseLeave => {
                self.update_pos(-1.)
            },
            Msg::Select(kind) => {
                self.props.selected = kind;
                // bubble
                self.props.onselect.emit(kind);
                // rerender
                true
            }
        }
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
    fn update_pos(&mut self, pos: f32) -> ShouldRender {
        self.pos = pos;
        true
    }

    fn generate_buttons(&self) -> Html {
        html! {
            <div class="panel-selector" style=self.get_style() >
                { 
                    for self.panel_images.iter()
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
                        }) 
                }
            </div>
        }
    }

    fn get_style(&self) -> String {
        format!(
            concat!(
                "transform: translate({}%, 0);",
                "transition: transform 0.3s;",
            ), 
            self.pos * 100.
        )
    }
}
