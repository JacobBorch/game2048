use grid::Model;

mod grid;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Model>::new().render();
}
