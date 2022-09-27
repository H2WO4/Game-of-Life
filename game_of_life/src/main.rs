#![feature(mixed_integer_ops)]
#![feature(is_some_with)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod rules;

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


mod universe {
    use super::*;

    pub struct Obj {
        pub cells: Grid<bool>,
        rules:     Rules,
        torus:     bool,

        _interval: Option<Interval>,
        _speed:    u16,
        _producer: Box<dyn Bridge<ToUniverse>>,
    }
    #[derive(PartialEq)]
    pub enum Msg {
        Play,
        Pause,
        Step,

        ChangeSpeed(u16),

        Generate(u8),
        Clear,

        ChangeSize(usize, usize, bool),

        ChangeRules(Rules),

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
            let _interval = None;

            Self { cells,
                   rules,
                   torus: true,
                   _producer,
                   _speed: 200,
                   _interval }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use Msg::*;

            match msg {
                Play => {
                    let callback = ctx.link().callback(|_| Msg::Step);
                    self._interval = Some(Interval::new(self._speed.into(), move || callback.emit(())));

                    ctx.link().send_message(Msg::Step);

                    false
                },

                Pause => {
                    let interval = replace(&mut self._interval, None);
                    interval.unwrap().cancel();

                    false
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

                ChangeSpeed(speed) => {
                    self._speed = speed;

                    if self._interval.is_some() {
                        ctx.link().send_message(Msg::Pause);
                        ctx.link().send_message(Msg::Play);
                    }

                    false
                },

                Generate(prob) => {
                    let (width, height) = self.cells.size();
                    let prob = prob as f32 / 100.0;

                    for x in 0..width {
                        for y in 0..height {
                            let cell = self.cells.get_mut(x, y).unwrap();
                            *cell = rand::random::<f32>() < prob;
                        }
                    }

                    true
                },

                Clear => {
                    let (width, height) = self.cells.size();
                    self.cells = Grid::init(width, height, false);

                    true
                },

                ChangeSize(width, height, torus) => {
                    let (old_width, old_height) = self.cells.size();

                    let mut new_cells = Grid::init(width, height, false);

                    for x in 0..width.min(old_width) {
                        for y in 0..height.min(old_height) {
                            let new_cell = new_cells.get_mut(x, y).unwrap();
                            let cell = self.cells.get(x, y).unwrap();

                            *new_cell = *cell;
                        }
                    }
                    self.cells = new_cells;

                    self.torus = torus;

                    true
                },

                ChangeRules(rules) => {
                    self.rules = rules;
                    false
                },

                Toggle(x, y) => {
                    let cell = self.cells.get_mut(x, y).unwrap();
                    *cell = !*cell;
                    true
                },

                Message(msg) => self.update(ctx, *msg),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (width, height) = self.cells.size();

            html! {
                <div class={ "universe" }>
                    { for (0..height).map(|y| {
                        html! {
                            <div class={ "inner-row" }>
                                { for (0..width).map(|x| {
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

            let (width, height) = self.cells.size();
            let x = x as isize;
            let y = y as isize;

            NEIGHBORS.map(|(i, j)| {
                         let x = x + i;
                         let y = y + j;

                         if self.torus {
                             let x = match x {
                                 x if x < 0 => x + width as isize,
                                 x if x >= width as isize => x - width as isize,
                                 x => x,
                             };
                             let y = match y {
                                 y if y < 0 => y + height as isize,
                                 y if y >= height as isize => y - height as isize,
                                 y => y,
                             };

                             self.cells
                                 .get(x as usize, y as usize)
                                 .and_then(|val| Some(*val))
                         } else {
                             self.cells
                                 .get(x as usize, y as usize)
                                 .and_then(|val| Some(*val))
                         }
                     })
        }
    }
}


mod play_button {
    use super::*;

    pub struct Obj {
        state:     State,
        event_bus: Dispatcher<ToUniverse>,
    }
    #[derive(PartialEq)]
    enum State {
        Playing,
        Paused,
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


mod speed_button {
    use super::*;

    pub struct Obj {
        input_ref: NodeRef,
        event_bus: Dispatcher<ToUniverse>,
    }
    pub enum Msg {
        ChangeSpeed,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self { input_ref: NodeRef::default(),
                   event_bus: ToUniverse::dispatcher(), }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use to_universe::In;
            use Msg::*;

            match msg {
                ChangeSpeed => {
                    let speed = self.input_ref
                                    .cast::<HtmlInputElement>()
                                    .unwrap()
                                    .value()
                                    .parse()
                                    .unwrap();

                    self.event_bus
                        .send(In::ChangeSpeed(speed));

                    false
                },
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let change_speed = ctx.link()
                                  .callback(|_| Msg::ChangeSpeed);

            html! {
                <div class={ "speed-btn" }>
                    <input type={ "range" } min={ 50 } max={ 1000 } step={ 50 } value={ 200 }  ref={ self.input_ref.clone() } />
                    <button onclick={ change_speed }>
                        { "Change Speed" }
                    </button>
                </div>
            }
        }
    }
}


mod gen_button {
    use super::*;

    pub struct Obj {
        input_ref: NodeRef,
        event_bus: Dispatcher<ToUniverse>,
    }
    pub enum Msg {
        Generate,
        Clear,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            let input_ref = NodeRef::default();
            let event_bus = ToUniverse::dispatcher();

            Self { event_bus, input_ref }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use to_universe::In;
            use Msg::*;

            match msg {
                Generate => {
                    let prob = self.input_ref
                                   .cast::<HtmlInputElement>()
                                   .unwrap()
                                   .value()
                                   .parse()
                                   .unwrap();

                    self.event_bus.send(In::Generate(prob));
                    false
                },

                Clear => {
                    self.event_bus.send(In::Clear);
                    false
                },
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let generate = ctx.link().callback(|_| Msg::Generate);
            let clear = ctx.link().callback(|_| Msg::Clear);

            html! {
                <div class={ "gen-btn" }>
                    <input type={ "range" } min={ 0 } max={ 100 } step={ 5 } ref={ self.input_ref.clone() } />
                    <button onclick={ generate }>
                        { "Generate" }
                    </button>
                    <button onclick={ clear }>
                        { "Clear" }
                    </button>
                </div>
            }
        }
    }
}


mod size_button {
    use super::*;

    pub struct Obj {
        width_ref:  NodeRef,
        height_ref: NodeRef,

        torus_ref: NodeRef,

        event_bus: Dispatcher<ToUniverse>,
    }
    pub enum Msg {
        ChangeSize,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self { width_ref:  NodeRef::default(),
                   height_ref: NodeRef::default(),
                   torus_ref:  NodeRef::default(),
                   event_bus:  ToUniverse::dispatcher(), }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use to_universe::In;
            use Msg::*;

            match msg {
                ChangeSize => {
                    let width = self.width_ref
                                    .cast::<HtmlInputElement>()
                                    .unwrap()
                                    .value()
                                    .parse()
                                    .unwrap();

                    let height = self.height_ref
                                     .cast::<HtmlInputElement>()
                                     .unwrap()
                                     .value()
                                     .parse()
                                     .unwrap();

                    let torus = self.torus_ref
                                    .cast::<HtmlInputElement>()
                                    .unwrap()
                                    .checked();

                    self.event_bus
                        .send(In::ChangeSize(width, height, torus));
                    false
                },
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let change_size = ctx.link().callback(|_| Msg::ChangeSize);

            html! {
                <div class={ "size-btn" }>
                   <input type={ "number" } value={ 96 } ref={ self.width_ref.clone() } />
                   <input type={ "number" } value={ 64 } ref={ self.height_ref.clone() } />
                   <div>
                       <input type={ "checkbox" } ref={ self.torus_ref.clone() } checked={ true } />
                       <label>{ "Torus" }</label>
                   </div>
                   <button onclick={ change_size } >
                       { "Change Size" }
                   </button>
                </div>
            }
        }
    }
}


mod rules_button {
    use super::*;

    pub struct Obj {
        input_ref: NodeRef,
        event_bus: Dispatcher<ToUniverse>,
    }
    pub enum Msg {
        ChangeRules,
    }
    impl Component for Obj {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self { input_ref: NodeRef::default(),
                   event_bus: ToUniverse::dispatcher(), }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            use to_universe::In;
            use Msg::*;

            match msg {
                ChangeRules => {
                    let rules = self.input_ref
                                    .cast::<HtmlInputElement>()
                                    .unwrap()
                                    .value();

                    if let Ok(rules) = Rules::from_string(&rules) {
                        self.event_bus
                            .send(In::ChangeRules(rules));
                    }

                    false
                },
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let change_rules = ctx.link()
                                  .callback(|_| Msg::ChangeRules);

            html! {
                <div class={ "rules-btn" }>
                    <input type={ "text" } value={ "B3/S23" } ref={ self.input_ref.clone() } />
                    <button onclick={ change_rules }>
                        { "Change Rules" }
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

        ChangeSpeed(u16),

        Generate(u8),
        Clear,

        ChangeSize(usize, usize, bool),

        ChangeRules(Rules),
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

                In::ChangeSpeed(speed) =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::ChangeSpeed(speed)));
                    },

                In::Generate(prob) =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::Generate(prob)));
                    },

                In::Clear =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::Clear));
                    },

                In::ChangeSize(width, height, torus) =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::ChangeSize(width, height, torus)));
                    },

                In::ChangeRules(rules) =>
                    for sub in self.subscribers.iter() {
                        self.link
                            .respond(*sub, Box::new(Out::ChangeRules(rules.clone())));
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

            <Universe width=96 height=64 />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
