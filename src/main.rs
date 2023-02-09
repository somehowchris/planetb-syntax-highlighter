#![deny(clippy::all)]

mod app;

pub mod utils;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
