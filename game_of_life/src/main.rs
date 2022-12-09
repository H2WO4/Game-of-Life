#![allow(dead_code)]
#![allow(unused_variables)]


mod gen_button;
mod play_button;
mod rules;
mod rules_button;
mod size_button;
mod speed_button;
mod to_universe;
mod universe;


use std::collections::HashSet;
use std::mem::replace;

use gen_button::Obj as GenButton;
use gloo_timers::callback::Interval;
use grid::Grid;
use play_button::Obj as PlayButton;
use rules::Rules;
use rules_button::Obj as RulesButton;
use size_button::Obj as SizeButton;
use speed_button::Obj as SpeedButton;
use to_universe::Obj as ToUniverse;
use universe::Obj as Universe;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Agent, AgentLink, Bridge, Bridged, Dispatched, Dispatcher, HandlerId};


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class={ "option" }>
                <div class={ "config" }>
                    <PlayButton />
                    <SpeedButton />
                    <GenButton />
                    <SizeButton />
                    <RulesButton />
                </div>
                <div class={ "help" } >
                    <div>
                        { "Some rules:" }
                    </div>
                    <div>
                        { "Standard: B3/S23" }
                    </div>
                    <div>
                        { "Day & Night: B3678/S34678" }
                    </div>
                    <div>
                        { "AntiLife: B0123478/S01234678" }
                    </div>
                    <div>
                        { "Replicator: B1357/S1357" }
                    </div>
                    <div>
                        { "Life w/ Death: B3/S012345678" }
                    </div>
                </div>
            </div>

            <Universe width=96 height=63 />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
