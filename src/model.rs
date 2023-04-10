use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use wasm_bindgen::prelude::Closure;
use yew::events::KeyboardEvent;
use yew::prelude::*;
use web_sys::{TouchEvent, TouchList};

use crate::grid::{Grid, Move};

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

pub enum Msg {
    KeyDown(KeyboardEvent),
    TouchStart(TouchEvent),
    TouchMove(TouchEvent),
    TouchEnd(TouchEvent)
}

pub struct Model {
    grid: Grid,
    score: u64,
    grid_node: NodeRef,
    touch_start_x: Option<i32>,
    touch_start_y: Option<i32>
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
        let cell_text = match cell {
            0 => "".to_string(),
            _ => cell.to_string()
        };
        html! {
            <div class="square" style={style}>
                <span class="square-number">{ cell_text }</span>
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
            touch_start_x: None,
            touch_start_y: None,
            
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
            <div class="grid disable-scroll" tabindex="0" ref={self.grid_node.clone()} 
            onkeydown={ctx.link().callback(|event| Msg::KeyDown(event))}
            ontouchstart={ctx.link().callback(|event| Msg::TouchStart(event))}
            ontouchmove={ctx.link().callback(|event| Msg::TouchMove(event))}
            ontouchend={ctx.link().callback(|event| Msg::TouchEnd(event))}
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
            Msg::TouchStart(event) => {
                event.prevent_default();
                let touches: TouchList = event.target_touches();
                if touches.length() > 0 {
                    let touch = touches.item(0).expect("No touch found");
                    self.touch_start_x = Some(touch.client_x());
                    self.touch_start_y = Some(touch.client_y());
                }
                true
            }
            Msg::TouchMove(event) => {
                event.prevent_default();
                true
            }
            Msg::TouchEnd(event) => {
                event.prevent_default();
                if let Some(touch_start_x) = self.touch_start_x {
                    if let Some(touch_start_y) = self.touch_start_y {
                        let changed_touches: TouchList = event.changed_touches();
                        if changed_touches.length() > 0 {
                            let touch = changed_touches.item(0).expect("No touch found");
                            let dx = touch.client_x() - touch_start_x;
                            let dy = touch.client_y() - touch_start_y;
                            let move_threshold = 30; // You can adjust this value as needed
            
                            let arrow = if dx.abs() > dy.abs() {
                                if dx > move_threshold {
                                    Some(Move::Right)
                                } else if dx < -move_threshold {
                                    Some(Move::Left)
                                } else {
                                    None
                                }
                            } else {
                                if dy > move_threshold {
                                    Some(Move::Down)
                                } else if dy < -move_threshold {
                                    Some(Move::Up)
                                } else {
                                    None
                                }
                            };
            
                            if let Some(a) = arrow {
                                self.grid.attempt(a);
                            }
            
                            self.touch_start_x = None;
                            self.touch_start_y = None;
                        }
                    }
                }
                true
            }
        }
    }
}