use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use wasm_bindgen::prelude::Closure;
use yew::events::KeyboardEvent;
use yew::prelude::*;

use crate::grid::{Grid, Move};

pub enum Msg {
    KeyDown(KeyboardEvent)
}

fn get_color_for_cell(value: u64) -> &'static str {
    match value {
        0 => "rgba(238, 228, 218, 0.35)",
        2 => "#eee4da",
        4 => "#ede0c8",
        8 => "#f2b179",
        16 => "#f59563",
        32 => "#f67c5f",
        64 => "#f65e3b",
        128 => "#edcf72",
        256 => "#edcc61",
        512 => "#edc850",
        1024 => "#edc53f",
        2048 => "#edc22e",
        _ => "#3c3a32",
    }
}

pub struct Model {
    grid: Grid,
    score: u64,
    grid_node: NodeRef,
}

impl Model {
    fn view_row(&self, (y, row): (usize, &[u64; 4])) -> Html {
        html! {
            <div class="square-row">
                { for row.iter().enumerate().map(|(x, cell)| self.view_cell(*cell, x, y)) }
            </div>
        }
    }
    

    fn view_cell(&self, cell: u64, x: usize, y: usize) -> Html {
        let background_color = format!("background-color:{};", get_color_for_cell(cell));
        let position_top = format!("top:{}px;", y * 100); // Adjust this value based on your grid cell size
        let position_left = format!("left:{}px;", x * (100 + 7)); // Adjust this value based on your grid cell size
        let style = format!("{}{}{}", background_color, position_top, position_left);
        html! {
            <div class="square" style={style}>
                <span class="square-number">{ cell }</span>
            </div>
        }
    } 
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let model = Model {
            grid: Grid::default(),
            score: 0,
            grid_node: NodeRef::default(),
        };

        let grid_node = model.grid_node.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(grid) = grid_node.cast::<HtmlDivElement>() {
                grid.focus().unwrap();
            }
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                10,
            )
            .unwrap();
        closure.forget();

        model
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="grid" tabindex="0" ref={self.grid_node.clone()} 
            onkeydown={ctx.link().callback(|event| Msg::KeyDown(event))}
            >
            <section class="section">
                <div class="container">
                    <div class="vcenter">
                        <div class="board">
                            <div class="square-grid">
                                { for self.grid.cells.iter().enumerate().map(|(y, row)| self.view_row((y, row))) }
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
        </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(event) => {
                let key_code = event.key_code();
                self.score = key_code as u64;
                let arrow = match key_code {
                    37 => Some(Move::Left),
                    38 => Some(Move::Up),
                    39 => Some(Move::Right),
                    40 => Some(Move::Down),
                    _ => None,
                };
                if let Some(a) = arrow {
                    self.grid.attempt(a);
                }

                true
            }
        }
    }
}