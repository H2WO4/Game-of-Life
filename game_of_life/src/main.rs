#![feature(mixed_integer_ops)]
#![feature(is_some_with)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod rules;

use std::collections::HashSet;

use gloo_timers::callback::Interval;
use grid::Grid;
use play_button::Obj as PlayButton;
use to_universe::Obj as ToUniverse;
use universe::Obj as Universe;
use yew::prelude::*;
use yew_agent::{Agent, AgentLink, Bridge, Bridged, Dispatched, Dispatcher, HandlerId};

use crate::rules::Rules;


mod universe {

    use super::*;

    pub struct Obj {
        pub cells: Grid<bool>,
        rules:     Rules,
        state:     State,

        _interval: Interval,
        _producer: Box<dyn Bridge<ToUniverse>>,
    }
    #[derive(PartialEq)]
	enum State {
        Playing,
        Paused,
    }
    pub enum Msg {
        Play,
        Pause,
        Tick,
        Step,
        ChangeSize(usize, usize),
        Toggle(usize, usize),

        Message(Box<Msg>),
    }
    #[derive(PartialEq, Properties)]
    pub struct Props {
        pub width:  usize,
        pub height: usize,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = Props;

        fn create(ctx: &Context<Self>) -> Self {
            let width = ctx.props().width;
            let height = ctx.props().height;

            let cells = Grid::init(width, height, false);
            let rules = Rules::default();

            let _producer = ToUniverse::bridge(ctx.link().callback(Msg::Message));
            let callback = ctx.link().callback(|_| Msg::Tick);
            let _interval = Interval::new(100, move || callback.emit(()));

            Self { cells,
                   rules,
                   state: State::Paused,
                   _producer,
                   _interval }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use Msg::*;

            match msg {
                Play => {
                    self.state = State::Playing;
                    false
                },

                Pause => {
                    self.state = State::Paused;
                    false
                },

                Tick => {
                    if self.state == State::Playing {
                        ctx.link().send_message(Msg::Step);
                    }
                    true
                },

                Step => {
                    let (width, height) = self.cells.size();

                    // TODO: Multithreading
                    let mut new_cells = Grid::init(width, height, false);

                    for x in 0..width {
                        for y in 0..height {
                            let new_cell = new_cells.get_mut(x, y).unwrap();
                            let cell = self.cells.get(x, y).unwrap();
                            let neighbors = self.get_neighbors(x, y);

                            if *cell {
                                *new_cell = self.rules.survive_arr[neighbors.iter()
                                                                            .fold(0u16, |v, c| (v << 1) + u16::from(c.is_some_and(|c| *c)))
                                                                   as usize];
                            } else {
                                *new_cell = self.rules.birth_arr[neighbors.iter()
                                                                          .fold(0u16, |v, c| (v << 1) + u16::from(c.is_some_and(|c| *c)))
                                                                 as usize];
                            }
                        }
                    }

                    self.cells = new_cells;
                    true
                },

                Toggle(x, y) => {
                    let cell = self.cells.get_mut(x, y).unwrap();
                    *cell = !*cell;
                    true
                },

                Message(msg) => self.update(ctx, *msg),
                _ => todo!(),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (width, height) = self.cells.size();

            html! {
                <div class={ "universe" }>
                    { for (0..width).map(|x| {
                        html! {
                            <div class={ "inner-row" }>
                                { for (0..height).map(|y| {
                                    let alive = self.cells[x][y];
                                    let onclick = ctx.link().callback(move |_| Msg::Toggle(x, y));

                                    html! {
                                        <div class={classes!("cell", if alive { "alive" } else { "dead" })} {onclick} />
                                    }
                                }) }
                            </div>
                        }
                     }) }
                </div>
            }
        }
    }
    impl Obj {
        fn get_neighbors(&self, x: usize, y: usize) -> [Option<bool>; 8] {
            const NEIGHBORS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

            NEIGHBORS.map(|(i, j)| {
                         let x = x.checked_add_signed(i);
                         let y = y.checked_add_signed(j);

                         x.and_then(|x| {
                              y.and_then(|y| {
                                   self.cells
                                       .get(x, y)
                                       .and_then(|val| Some(*val))
                               })
                          })
                     })
        }
    }
}


mod play_button {
    use super::*;

    #[derive(PartialEq)]
    enum State {
        Playing,
        Paused,
    }
    pub struct Obj {
        state:     State,
        event_bus: Dispatcher<ToUniverse>,
    }
    pub enum Msg {
        Play,
        Pause,
        Step,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self { event_bus: ToUniverse::dispatcher(),
                   state:     State::Paused, }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use to_universe::In;
            use Msg::*;

            match msg {
                Play => {
                    self.state = State::Playing;
                    self.event_bus.send(In::Play);
                    true
                },

                Pause => {
                    self.state = State::Paused;
                    self.event_bus.send(In::Pause);
                    true
                },

                Step => {
                    self.event_bus.send(In::Step);
                    false
                },
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let play = ctx.link().callback(|_| Msg::Play);
            let pause = ctx.link().callback(|_| Msg::Pause);
            let step = ctx.link().callback(|_| Msg::Step);

            html! {
                <div class={ "play-btn" }>
                    <button onclick={ play } disabled={ self.state == State::Playing }>
                        { "Play" }
                    </button>
                    <button onclick={ pause } disabled={ self.state == State::Paused }>
                        { "Pause" }
                    </button>
                    <button onclick={ step } disabled={ self.state == State::Playing }>
                        { "Step" }
                    </button>
                </div>
            }
        }
    }
}


mod to_universe {
    use super::*;

    pub struct Obj {
        link:        AgentLink<Self>,
        subscribers: HashSet<HandlerId>,
    }
    pub enum In {
        Play,
        Pause,
        Step,
    }
    impl Agent for Obj {
        type Input = In;
        type Message = ();
        type Output = Box<universe::Msg>;
        type Reach = yew_agent::Context<Self>;

        fn create(link: AgentLink<Self>) -> Self {
            Self { link,
                   subscribers: HashSet::new() }
        }

        fn update(&mut self, msg: Self::Message) {}

        fn handle_input(&mut self, msg: Self::Input, id: yew_agent::HandlerId) {
            use universe::Msg as Out;

            match msg {
                In::Play =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::Play));
                    },

                In::Pause =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::Pause));
                    },

                In::Step =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::Step));
                    },
            }
        }

        fn connected(&mut self, id: yew_agent::HandlerId) {
            self.subscribers.insert(id);
        }

        fn disconnected(&mut self, id: yew_agent::HandlerId) {
            self.subscribers.remove(&id);
        }
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class={ "option" }>
                <PlayButton />
            </div>

            <Universe width=20 height=20 />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
