use yew::{classes, html, Callback, Html, MouseEvent};

#[derive(Clone, Copy)]
pub struct Cell {
    val:   Val,
    state: State,
}
#[derive(Clone, Copy)]
pub enum Val {
    Num(u8),
    Mine,
}
#[derive(Clone, Copy)]
pub enum State {
    Hidden,
    Revealed,
    Flagged,
}

impl Cell {
    pub const fn new(val: Val) -> Self {
        Self { val,
               state: State::Hidden }
    }

    pub fn render(self, onclick: Callback<MouseEvent>, oncontextmenu: Callback<MouseEvent>) -> Html {
        use State::*;
        use Val::*;

        let class = match self {
            Self { val: _, state: Hidden } => "hidden",
            Self { val: _, state: Flagged } => "flagged",
            Self { val: Mine, state: _ } => "mine",
            Self { val: Num(_), state: _ } => "num",
        };

        let inner = match self {
            Self { val: _,
                   state: Hidden | Flagged, }
            | Self { val: Mine, state: _ } => String::new(),
            Self { val: Num(x), state: _ } => x.to_string(),
        };

        html!(<div class={ classes!("cell", class) } {onclick} {oncontextmenu}>{ inner }</div>)
    }

    pub fn l_click(&mut self) {
        if !matches!(self.state, State::Hidden) {
            return
        }

        self.state = State::Revealed;

        if matches!(self.val, Val::Mine) {
            todo!() // Game over
        }
    }

    pub fn r_click(&mut self) {
        use State::*;

        self.state = match self.state {
            Hidden => Flagged,
            Flagged => Hidden,
            Revealed => Revealed,
        }
    }
}
