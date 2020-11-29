#![recursion_limit="2000"]

mod app;
mod woocsv;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::Gui>();

    Ok(())
}
