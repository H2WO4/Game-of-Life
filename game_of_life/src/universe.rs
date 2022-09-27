use super::*;

pub struct Obj {
    pub cells: Grid<bool>,
    rules:     Rules,
    torus:     bool,

    run_for: Option<u32>,

    _interval: Option<Interval>,
    _speed:    u16,
    _producer: Box<dyn Bridge<ToUniverse>>,
}
#[derive(PartialEq)]
pub enum Msg {
    Play,
    Pause,
    Step,

	VirtualStep,
    RunFor(u32),

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
               run_for: None,

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

			VirtualStep => {
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
                false
            },

            RunFor(value) => {
                if value != 0 {
                    ctx.link().send_message(Msg::VirtualStep);
                    ctx.link()
                       .send_message(Msg::RunFor(value - 1));
                }

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
