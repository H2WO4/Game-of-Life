use grid::Grid;
use yew::{html, Component, Context, Html, MouseEvent, Properties};

use crate::cell::{Cell, Val};


pub struct Obj {
    field: Grid<Cell>,
}
pub enum Msg {
    LClick(usize, usize),
    RClick(usize, usize),
}
#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub height: usize,
    pub width:  usize,
}
impl Component for Obj {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let height = ctx.props().height;
        let width = ctx.props().width;

        let field = generate(height, width);

        Self { field }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            LClick(x, y) => {
                self.field[x][y].l_click();
                true
            },
            RClick(x, y) => {
                self.field[x][y].r_click();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (width, height) = self.field.size();

        html! {
            <>
                <div class={ "minefield" }> {
                    for (0..height).map(|y| {
                        html! {
                            <div class={ "inner-row" }> {
                                for (0..width).map(|x| {
                                    let onclick = ctx.link().callback(move |e: MouseEvent| { e.prevent_default(); Msg::LClick(x, y) });
                                    let oncontextmenu = ctx.link().callback(move |e: MouseEvent| { e.prevent_default(); Msg::RClick(x, y) });

                                    self.field[x][y].render(onclick, oncontextmenu)}
                                )
                            } </div>
                        }
                    })
                } </div>
            </>
        }
    }
}

fn generate(height: usize, width: usize) -> Grid<Cell> {
    Grid::init(height, width, Cell::new(Val::Num(0)))
}
