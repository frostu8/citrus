#![recursion_limit="256"]

extern crate nalgebra as na;

pub mod editor;
pub mod enum_map;
pub mod format;
pub mod gl;
pub mod services;
pub mod util;

#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::storage::{StorageService, Area};

use format::Ron;
use editor::{FieldEditor, EditorView};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Citrus runtime.
pub struct Runtime {
    link: ComponentLink<Self>,
    
    view: EditorView,
}

pub enum Msg {
    Update(EditorView),
}

impl Component for Runtime {
    type Message = Msg;
    type Properties = ();
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Runtime {
            link,
            // load views from storage
            view: Runtime::load_storage()
                .unwrap_or(EditorView::new_example()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(view) => {
                Runtime::save_storage(&view);
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <FieldEditor view=self.view.clone() 
                         onupdate=self.link.callback(Msg::Update) />
        }
    }
}

impl Runtime {
    fn load_storage() -> Option<EditorView> {
        // TODO: add more storage entries
        // for now, one or zero will do.
        let storage = StorageService::new(Area::Local).unwrap();

        // restore our value
        storage.restore::<Ron<Result<EditorView, anyhow::Error>>>("cached_field").0
            .ok()
    }

    fn save_storage(save: &EditorView) {
        let mut storage = StorageService::new(Area::Local).unwrap();

        storage.store::<Ron<&EditorView>>("cached_field", Ron(save));
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    yew::start_app::<Runtime>();

    Ok(())
}
