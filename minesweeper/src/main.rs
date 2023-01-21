#![allow(dead_code)]
#![allow(clippy::enum_glob_use)]


mod cell;
mod minefield;


use minefield::Obj as Minefield;
use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class={ "option" }>
                <div class={ "config" }>
                </div>
            </div>
            <Minefield height=10 width=10/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
