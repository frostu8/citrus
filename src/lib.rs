#![recursion_limit="256"]

extern crate nalgebra as na;

pub mod editor;
pub mod enum_map;
pub mod gl;
pub mod services;
pub mod util;

#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::*;

use yew::App;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    App::<editor::FieldEditor>::new().mount_to_body();

    Ok(())
}
