mod api;
mod app;
mod components;
mod route;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<app::App>::new().mount_to_body();
}
