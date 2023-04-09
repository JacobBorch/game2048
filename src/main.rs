use grid::Model;
use wasm_bindgen::prelude::wasm_bindgen;


mod grid;

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Model>::new().render();
}
